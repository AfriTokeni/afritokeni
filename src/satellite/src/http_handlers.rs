use candid::{CandidType, Deserialize};
use ic_cdk::api::call::ManualReply;
use serde::Serialize;

/// HTTP Request structure (matches IC HTTP Gateway Protocol)
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// HTTP Response structure (matches IC HTTP Gateway Protocol)
#[derive(Clone, Debug, CandidType, Serialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Route incoming HTTP requests to appropriate handlers
pub fn route_request(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Parse URL path
    let path = req.url.split('?').next().unwrap_or(&req.url);
    
    match (req.method.as_str(), path) {
        ("GET", "/api/health") => health_check(),
        ("POST", "/api/ussd") => crate::ussd::handle_ussd_webhook(req),
        ("POST", "/api/sms") => handle_incoming_sms(req),
        ("POST", "/api/send-sms") => crate::sms::handle_sms_webhook(req),
        ("POST", "/api/verify-code") => handle_verify_code(req),
        ("POST", "/api/send-notification") => handle_send_notification(req),
        _ => not_found(),
    }
}

/// Handle incoming SMS from Africa's Talking
fn handle_incoming_sms(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Parse form data
    let body_str = match std::str::from_utf8(&req.body) {
        Ok(s) => s,
        Err(_) => return error_response(400, "Invalid UTF-8 in request body"),
    };
    
    // Parse form-urlencoded data
    let params: std::collections::HashMap<String, String> = body_str
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => {
                    Some((
                        urlencoding::decode(key).ok()?.into_owned(),
                        urlencoding::decode(value).ok()?.into_owned(),
                    ))
                }
                _ => None,
            }
        })
        .collect();
    
    let from = params.get("from").map(|s| s.as_str()).unwrap_or("");
    let to = params.get("to").map(|s| s.as_str()).unwrap_or("");
    let text = params.get("text").map(|s| s.as_str()).unwrap_or("");
    let date = params.get("date").map(|s| s.as_str()).unwrap_or("");
    let id = params.get("id").map(|s| s.as_str()).unwrap_or("");
    
    ic_cdk::println!(
        "📨 SMS Received - From: {}, To: {}, Text: '{}', Date: {}, ID: {}",
        from, to, text, date, id
    );
    
    // Validate
    if from.is_empty() || text.is_empty() {
        return error_response(400, "Missing required fields");
    }
    
    // Process SMS command
    let command = text.trim().to_uppercase();
    let parts: Vec<&str> = text.trim().split_whitespace().collect();
    
    // Default to English for SMS commands (could fetch user's language preference from DB)
    let lang = crate::translations::Language::English;
    
    let response_message = match parts.get(0).map(|s| s.to_uppercase()).as_deref() {
        Some("BAL") | Some("BALANCE") => {
            // Check balance command - spawn async task to fetch and send SMS
            let from_clone = from.to_string();
            ic_cdk::spawn(async move {
                let lang = crate::translations::Language::English; // TODO: fetch user's language
                match junobuild_satellite::get_doc_store(
                    ic_cdk::caller(),
                    "balances".to_string(),
                    from_clone.clone(),
                ) {
                    Some(doc) => {
                        // Decode balance data
                        #[derive(serde::Deserialize)]
                        struct Balance {
                            kes: f64,
                            ckbtc: f64,
                            ckusdc: f64,
                        }
                        
                        match junobuild_utils::decode_doc_data::<Balance>(&doc.data) {
                            Ok(balance) => {
                                let sms_message = format!(
                                    "{} {}:\nKES: {:.2}\nckBTC: {:.8}\nckUSDC: {:.2}",
                                    crate::translations::TranslationService::translate("welcome_afritokeni", lang),
                                    crate::translations::TranslationService::translate("balance", lang),
                                    balance.kes, balance.ckbtc, balance.ckusdc
                                );
                                // Send SMS with balance
                                let _ = crate::sms::send_sms_via_api(vec![from_clone], sms_message).await;
                            }
                            Err(e) => {
                                ic_cdk::println!("❌ Failed to decode balance: {}", e);
                            }
                        }
                    }
                    None => {
                        // No account found - send SMS to register
                        let sms_message = format!("{}. {} *229#.", 
                            crate::translations::TranslationService::translate("user_not_found", lang),
                            crate::translations::TranslationService::translate("help_ussd", lang).replace("Dial", "").trim());
                        let _ = crate::sms::send_sms_via_api(vec![from_clone], sms_message).await;
                    }
                }
            });
            format!("{}. {}", 
                crate::translations::TranslationService::translate("balance", lang),
                crate::translations::TranslationService::translate("sms_sent", lang))
        }
        Some("SEND") => {
            // Send money command: SEND +256... 10000
            if parts.len() < 3 {
                format!("{}: SEND +phone amount", 
                    crate::translations::TranslationService::translate("invalid_phone_format", lang))
            } else {
                let recipient = parts[1];
                let amount = parts[2];
                
                let from_clone = from.to_string();
                let recipient_clone = recipient.to_string();
                let amount_clone = amount.to_string();
                
                ic_cdk::spawn(async move {
                    // Create transaction in Juno datastore
                    #[derive(serde::Serialize)]
                    struct Transaction {
                        from: String,
                        to: String,
                        amount: String,
                        currency: String,
                        timestamp: u64,
                        status: String,
                    }
                    
                    let tx = Transaction {
                        from: from_clone.clone(),
                        to: recipient_clone.clone(),
                        amount: amount_clone.clone(),
                        currency: "KES".to_string(),
                        timestamp: ic_cdk::api::time(),
                        status: "pending".to_string(),
                    };
                    
                    let encoded = junobuild_utils::encode_doc_data(&tx).unwrap();
                    let doc = junobuild_satellite::SetDoc {
                        data: encoded,
                        description: Some("SMS send money transaction".to_string()),
                        version: None,
                    };
                    
                    let tx_id = format!("tx_{}", ic_cdk::api::time());
                    match junobuild_satellite::set_doc_store(
                        ic_cdk::caller(),
                        "transactions".to_string(),
                        tx_id,
                        doc,
                    ) {
                        Ok(_) => {
                            let lang = crate::translations::Language::English;
                            let sms = format!("{} {} KES {} {}. {}.", 
                                crate::translations::TranslationService::translate("send_money", lang),
                                amount_clone, 
                                crate::translations::TranslationService::translate("to", lang),
                                recipient_clone,
                                crate::translations::TranslationService::translate("transaction_successful", lang));
                            let _ = crate::sms::send_sms_via_api(vec![from_clone], sms).await;
                        }
                        Err(e) => {
                            ic_cdk::println!("❌ Failed to create transaction: {}", e);
                        }
                    }
                });
                
                format!("{} {} KES {} {}. {}.", 
                    crate::translations::TranslationService::translate("send_money", lang),
                    amount,
                    crate::translations::TranslationService::translate("to", lang),
                    recipient,
                    crate::translations::TranslationService::translate("sms_confirmations_sent", lang))
            }
        }
        Some(cmd) if cmd.contains("BTC") || (parts.len() > 1 && parts[1].to_uppercase() == "BTC") => {
            // Buy Bitcoin command: BUY BTC 50000 or BTC BUY 50000
            let amount_idx = if cmd == "BUY" { 2 } else { 2 };
            if parts.len() <= amount_idx {
                format!("{}: BUY BTC amount", 
                    crate::translations::TranslationService::translate("invalid_amount", lang))
            } else {
                let amount = parts[amount_idx];
                let from_clone = from.to_string();
                let amount_clone = amount.to_string();
                
                ic_cdk::spawn(async move {
                    // Create BTC purchase transaction
                    #[derive(serde::Serialize)]
                    struct BtcPurchase {
                        user: String,
                        amount_kes: String,
                        asset: String,
                        timestamp: u64,
                        status: String,
                    }
                    
                    let purchase = BtcPurchase {
                        user: from_clone.clone(),
                        amount_kes: amount_clone.clone(),
                        asset: "ckBTC".to_string(),
                        timestamp: ic_cdk::api::time(),
                        status: "pending".to_string(),
                    };
                    
                    let encoded = junobuild_utils::encode_doc_data(&purchase).unwrap();
                    let doc = junobuild_satellite::SetDoc {
                        data: encoded,
                        description: Some("SMS BTC purchase".to_string()),
                        version: None,
                    };
                    
                    let tx_id = format!("btc_{}", ic_cdk::api::time());
                    match junobuild_satellite::set_doc_store(
                        ic_cdk::caller(),
                        "transactions".to_string(),
                        tx_id,
                        doc,
                    ) {
                        Ok(_) => {
                            let lang = crate::translations::Language::English;
                            let sms = format!("{} ckBTC {} {} KES. {}.", 
                                crate::translations::TranslationService::translate("buy_bitcoin", lang),
                                crate::translations::TranslationService::translate("with", lang),
                                amount_clone,
                                crate::translations::TranslationService::translate("transaction_successful", lang));
                            let _ = crate::sms::send_sms_via_api(vec![from_clone], sms).await;
                        }
                        Err(e) => {
                            ic_cdk::println!("❌ Failed to create BTC purchase: {}", e);
                        }
                    }
                });
                
                format!("{} ckBTC {} {} KES. {}.", 
                    crate::translations::TranslationService::translate("buy_bitcoin", lang),
                    crate::translations::TranslationService::translate("with", lang),
                    amount,
                    crate::translations::TranslationService::translate("sms_confirmations_sent", lang))
            }
        }
        Some(cmd) if cmd.contains("USDC") || (parts.len() > 1 && parts[1].to_uppercase() == "USDC") => {
            // Buy USDC command: BUY USDC 50000 or USDC BUY 50000
            let amount_idx = if cmd == "BUY" { 2 } else { 2 };
            if parts.len() <= amount_idx {
                format!("{}: BUY USDC amount", 
                    crate::translations::TranslationService::translate("invalid_amount", lang))
            } else {
                let amount = parts[amount_idx];
                let from_clone = from.to_string();
                let amount_clone = amount.to_string();
                
                ic_cdk::spawn(async move {
                    // Create USDC purchase transaction
                    #[derive(serde::Serialize)]
                    struct UsdcPurchase {
                        user: String,
                        amount_kes: String,
                        asset: String,
                        timestamp: u64,
                        status: String,
                    }
                    
                    let purchase = UsdcPurchase {
                        user: from_clone.clone(),
                        amount_kes: amount_clone.clone(),
                        asset: "ckUSDC".to_string(),
                        timestamp: ic_cdk::api::time(),
                        status: "pending".to_string(),
                    };
                    
                    let encoded = junobuild_utils::encode_doc_data(&purchase).unwrap();
                    let doc = junobuild_satellite::SetDoc {
                        data: encoded,
                        description: Some("SMS USDC purchase".to_string()),
                        version: None,
                    };
                    
                    let tx_id = format!("usdc_{}", ic_cdk::api::time());
                    match junobuild_satellite::set_doc_store(
                        ic_cdk::caller(),
                        "transactions".to_string(),
                        tx_id,
                        doc,
                    ) {
                        Ok(_) => {
                            let lang = crate::translations::Language::English;
                            let sms = format!("{} ckUSDC {} {} KES. {}.", 
                                crate::translations::TranslationService::translate("buy_usdc", lang),
                                crate::translations::TranslationService::translate("with", lang),
                                amount_clone,
                                crate::translations::TranslationService::translate("transaction_successful", lang));
                            let _ = crate::sms::send_sms_via_api(vec![from_clone], sms).await;
                        }
                        Err(e) => {
                            ic_cdk::println!("❌ Failed to create USDC purchase: {}", e);
                        }
                    }
                });
                
                format!("{} ckUSDC {} {} KES. {}.", 
                    crate::translations::TranslationService::translate("buy_usdc", lang),
                    crate::translations::TranslationService::translate("with", lang),
                    amount,
                    crate::translations::TranslationService::translate("sms_confirmations_sent", lang))
            }
        }
        _ => {
            format!("{}:\n{}", 
                crate::translations::TranslationService::translate("invalid_selection", lang),
                crate::translations::TranslationService::translate("help_commands", lang))
        }
    };
    
    let response = serde_json::json!({
        "status": "received",
        "message": response_message
    });
    
    ok_response(response.to_string().into_bytes(), "application/json")
}

/// Send notification (email or SMS)
fn handle_send_notification(req: HttpRequest) -> ManualReply<HttpResponse> {
    #[derive(serde::Deserialize)]
    struct NotificationRequest {
        #[serde(rename = "type")]
        notification_type: String,
        recipient: String,
    }
    
    let notif_req: NotificationRequest = match serde_json::from_slice(&req.body) {
        Ok(r) => r,
        Err(e) => return error_response(400, &format!("Invalid JSON: {}", e)),
    };
    
    ic_cdk::println!(
        "📧 Sending {} notification to {}",
        notif_req.notification_type,
        notif_req.recipient
    );
    
    // For email notifications, we'd use Resend API via ic-http-proxy
    // For SMS notifications, we'd use our SMS handler
    
    let lang = crate::translations::Language::English;
    let response = serde_json::json!({
        "success": true,
        "message": format!("{} {}", 
            notif_req.notification_type,
            crate::translations::TranslationService::translate("sms_sent", lang))
    });
    
    ok_response(response.to_string().into_bytes(), "application/json")
}

/// Verify SMS code endpoint
fn handle_verify_code(req: HttpRequest) -> ManualReply<HttpResponse> {
    #[derive(serde::Deserialize)]
    struct VerifyRequest {
        #[serde(rename = "phoneNumber")]
        phone_number: String,
        code: String,
    }
    
    // Parse request
    let verify_req: VerifyRequest = match serde_json::from_slice(&req.body) {
        Ok(r) => r,
        Err(e) => return error_response(400, &format!("Invalid JSON: {}", e)),
    };
    
    // Validate
    if verify_req.phone_number.is_empty() || verify_req.code.is_empty() {
        let lang = crate::translations::Language::English;
        return error_response(400, crate::translations::TranslationService::translate("invalid_phone", lang));
    }
    
    // Spawn async verification
    let phone = verify_req.phone_number.clone();
    let code = verify_req.code.clone();
    
    ic_cdk::spawn(async move {
        match crate::verification::verify_code(&phone, &code).await {
            Ok(user_id) => {
                ic_cdk::println!("✅ Code verified for user: {}", user_id);
            }
            Err(e) => {
                ic_cdk::println!("❌ Verification failed: {}", e);
            }
        }
    });
    
    // For now, return success immediately
    // In production, we'd wait for the async result
    let lang = crate::translations::Language::English;
    let response = serde_json::json!({
        "success": true,
        "message": crate::translations::TranslationService::translate("verification_required", lang)
    });
    
    ok_response(response.to_string().into_bytes(), "application/json")
}

/// Health check endpoint
fn health_check() -> ManualReply<HttpResponse> {
    let health_data = serde_json::json!({
        "status": "healthy",
        "service": "AfriTokeni Satellite Functions",
        "timestamp": ic_cdk::api::time() / 1_000_000_000, // Convert nanoseconds to seconds
        "endpoints": {
            "ussd": "/api/ussd",
            "sms": "/api/sms",
            "health": "/api/health"
        }
    });
    
    ok_response(health_data.to_string().into_bytes(), "application/json")
}

/// Return 404 Not Found response
fn not_found() -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 404,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: b"Not Found".to_vec(),
    };
    ManualReply::one(response)
}

/// Helper to create success response
pub fn ok_response(body: Vec<u8>, content_type: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), content_type.to_string())],
        body,
    };
    ManualReply::one(response)
}

/// Helper to create error response
pub fn error_response(status: u16, message: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: status,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: message.as_bytes().to_vec(),
    };
    ManualReply::one(response)
}
