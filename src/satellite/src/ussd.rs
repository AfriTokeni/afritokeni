use crate::http_handlers::{error_response, ok_response, HttpRequest, HttpResponse};
use ic_cdk::api::call::ManualReply;
use std::str;

/// Handle USSD webhook from Africa's Talking
/// 
/// Supports both:
/// - Form-urlencoded (Africa's Talking production)
/// - JSON (playground/testing)
/// 
/// Expected fields:
/// - sessionId: string
/// - serviceCode: string (optional)
/// - phoneNumber: string
/// - text: string
pub fn handle_ussd_webhook(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Check content type
    let content_type = req
        .headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == "content-type")
        .map(|(_, v)| v.as_str())
        .unwrap_or("");
    
    let is_json = content_type.contains("application/json");
    
    // Parse request body
    let body_str = match str::from_utf8(&req.body) {
        Ok(s) => s,
        Err(_) => return error_response(400, "Invalid UTF-8 in request body"),
    };
    
    let (session_id, phone_number, text) = if is_json {
        // Parse JSON request (playground)
        match parse_json_request(body_str) {
            Ok(data) => data,
            Err(e) => return error_response(400, &e),
        }
    } else {
        // Parse form-urlencoded data (Africa's Talking)
        let params = parse_form_data(body_str);
        let session_id = params.get("sessionId").map(|s| s.as_str()).unwrap_or("");
        let phone_number = params.get("phoneNumber").map(|s| s.as_str()).unwrap_or("");
        let text = params.get("text").map(|s| s.as_str()).unwrap_or("");
        (session_id.to_string(), phone_number.to_string(), text.to_string())
    };
    
    // Validate required fields
    if session_id.is_empty() || phone_number.is_empty() {
        return error_response(400, "Missing required fields: sessionId and phoneNumber");
    }
    
    // Log the request
    ic_cdk::println!(
        "📱 USSD Request - Session: {}, Phone: {}, Text: '{}', JSON: {}",
        session_id,
        phone_number,
        text,
        is_json
    );
    
    // Get or create session, process menu, and save session - all async
    let session_id_clone = session_id.clone();
    let phone_clone = phone_number.clone();
    let text_clone = text.clone();
    
    ic_cdk::spawn(async move {
        // Get or create session
        match crate::session::get_or_create_session(&session_id_clone, &phone_clone).await {
            Ok(mut session) => {
                // Process menu with session context
                let (response_text, continue_session) = process_ussd_menu_with_session(&text_clone, &mut session);
                
                if continue_session {
                    // Save session for next interaction
                    if let Err(e) = crate::session::save_session(&session).await {
                        ic_cdk::println!("❌ Failed to save session: {}", e);
                    }
                } else {
                    // Session ended, delete it
                    if let Err(e) = crate::session::delete_session(&session_id_clone).await {
                        ic_cdk::println!("❌ Failed to delete session: {}", e);
                    }
                }
                
                ic_cdk::println!("✅ USSD processed: continue={}, response_len={}", continue_session, response_text.len());
            }
            Err(e) => {
                ic_cdk::println!("❌ Session error: {}", e);
            }
        }
    });
    
    // Return immediate response (actual processing happens async)
    // For now, use simple menu logic synchronously
    let (response_text, continue_session) = process_ussd_menu(&text, &phone_number);
    
    // Return response based on request type
    if is_json {
        // JSON response for playground
        let json_response = serde_json::json!({
            "continueSession": continue_session,
            "response": response_text
        });
        ok_response(json_response.to_string().into_bytes(), "application/json")
    } else {
        // Plain text response for Africa's Talking
        // CON = Continue session, END = End session
        let prefix = if continue_session { "CON" } else { "END" };
        let ussd_response = format!("{} {}", prefix, response_text);
        ok_response(ussd_response.into_bytes(), "text/plain")
    }
}

/// Parse JSON USSD request
fn parse_json_request(body: &str) -> Result<(String, String, String), String> {
    #[derive(serde::Deserialize)]
    struct JsonRequest {
        #[serde(rename = "sessionId")]
        session_id: String,
        #[serde(rename = "phoneNumber")]
        phone_number: String,
        #[serde(default)]
        text: String,
    }
    
    let req: JsonRequest = serde_json::from_str(body)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    Ok((req.session_id, req.phone_number, req.text))
}

/// Process USSD menu with session state
/// This version uses the session to track multi-step flows
fn process_ussd_menu_with_session(input: &str, session: &mut crate::session::UssdSession) -> (String, bool) {
    let parts: Vec<&str> = input.split('*').collect();
    
    // Check if we're in a multi-step flow
    match session.current_menu.as_str() {
        "send_money" => {
            // Multi-step send money flow
            match session.step {
                0 => {
                    // Ask for recipient
                    session.step = 1;
                    let lang = crate::translations::Language::from_code(&session.language);
                    (crate::translations::TranslationService::translate("enter_recipient_phone", lang).to_string(), true)
                }
                1 => {
                    // Save recipient, ask for amount
                    if parts.len() > 0 {
                        session.set_data("recipient", parts[parts.len() - 1]);
                    }
                    session.step = 2;
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} (KES):", crate::translations::TranslationService::translate("enter_amount", lang)), true)
                }
                2 => {
                    // Save amount, confirm
                    if parts.len() > 0 {
                        session.set_data("amount", parts[parts.len() - 1]);
                    }
                    let recipient = session.get_data("recipient").cloned().unwrap_or_default();
                    let amount = session.get_data("amount").cloned().unwrap_or_default();
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} {} KES {} {}\n{}...", 
                        crate::translations::TranslationService::translate("send_money", lang),
                        amount,
                        crate::translations::TranslationService::translate("to", lang),
                        recipient,
                        crate::translations::TranslationService::translate("transaction_successful", lang)), false)
                }
                _ => ("Invalid state".to_string(), false)
            }
        }
        "buy_ckbtc" => {
            match session.step {
                0 => {
                    session.step = 1;
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} (KES) {} ckBTC:", 
                        crate::translations::TranslationService::translate("enter_amount", lang),
                        crate::translations::TranslationService::translate("to", lang)), true)
                }
                1 => {
                    if parts.len() > 0 {
                        session.set_data("amount", parts[parts.len() - 1]);
                    }
                    let amount = session.get_data("amount").cloned().unwrap_or_default();
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} ckBTC {} {} KES\n{}...", 
                        crate::translations::TranslationService::translate("buy_bitcoin", lang),
                        crate::translations::TranslationService::translate("with", lang).unwrap_or("with"),
                        amount,
                        crate::translations::TranslationService::translate("transaction_successful", lang)), false)
                }
                _ => ("Invalid state".to_string(), false)
            }
        }
        "buy_ckusdc" => {
            match session.step {
                0 => {
                    session.step = 1;
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} (KES) {} ckUSDC:", 
                        crate::translations::TranslationService::translate("enter_amount", lang),
                        crate::translations::TranslationService::translate("to", lang)), true)
                }
                1 => {
                    if parts.len() > 0 {
                        session.set_data("amount", parts[parts.len() - 1]);
                    }
                    let amount = session.get_data("amount").cloned().unwrap_or_default();
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} ckUSDC {} {} KES\n{}...", 
                        crate::translations::TranslationService::translate("buy_usdc", lang),
                        crate::translations::TranslationService::translate("with", lang).unwrap_or("with"),
                        amount,
                        crate::translations::TranslationService::translate("transaction_successful", lang)), false)
                }
                _ => ("Invalid state".to_string(), false)
            }
        }
        "withdraw" => {
            match session.step {
                0 => {
                    session.step = 1;
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} (KES):", crate::translations::TranslationService::translate("enter_amount", lang)), true)
                }
                1 => {
                    if parts.len() > 0 {
                        session.set_data("amount", parts[parts.len() - 1]);
                    }
                    let amount = session.get_data("amount").cloned().unwrap_or_default();
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{} {} KES\n{}", 
                        crate::translations::TranslationService::translate("withdraw", lang),
                        amount,
                        crate::translations::TranslationService::translate("receive_cash", lang)), false)
                }
                _ => ("Invalid state".to_string(), false)
            }
        }
        _ => {
            // Main menu or first interaction
            if input.is_empty() {
                let lang = crate::translations::Language::from_code(&session.language);
                return (
                    crate::translations::TranslationService::get_main_menu(lang),
                    true,
                );
            }
            
            // Handle menu selection
            match parts.get(0) {
                Some(&"1") => {
                    let lang = crate::translations::Language::from_code(&session.language);
                    (format!("{}:\nKES: 0\nckBTC: 0\nckUSDC: 0", 
                        crate::translations::TranslationService::translate("your_balance", lang)), false)
                }
                Some(&"2") => {
                    session.current_menu = "send_money".to_string();
                    session.step = 0;
                    process_ussd_menu_with_session(input, session)
                }
                Some(&"3") => {
                    session.current_menu = "buy_ckbtc".to_string();
                    session.step = 0;
                    process_ussd_menu_with_session(input, session)
                }
                Some(&"4") => {
                    session.current_menu = "buy_ckusdc".to_string();
                    session.step = 0;
                    process_ussd_menu_with_session(input, session)
                }
                Some(&"5") => {
                    session.current_menu = "withdraw".to_string();
                    session.step = 0;
                    process_ussd_menu_with_session(input, session)
                }
                Some(&"0") => {
                    let lang = crate::translations::Language::from_code(&session.language);
                    (crate::translations::TranslationService::translate("thank_you", lang).to_string(), false)
                }
                _ => {
                    let lang = crate::translations::Language::from_code(&session.language);
                    (crate::translations::TranslationService::translate("invalid_option", lang).to_string(), false)
                }
            }
        }
    }
}

/// Process USSD menu navigation (simple version without session)
/// Returns (response_text, continue_session)
pub fn process_ussd_menu(input: &str, _phone_number: &str) -> (String, bool) {
    let parts: Vec<&str> = input.split('*').collect();
    
    // New session (empty input)
    if input.is_empty() {
        return (
            "Welcome to AfriTokeni\n\
            1. Check Balance\n\
            2. Send Money\n\
            3. Buy ckBTC\n\
            4. Buy ckUSDC\n\
            5. Withdraw\n\
            0. Exit".to_string(),
            true, // Continue session
        );
    }
    
    // Handle menu selection
    match parts.get(0) {
        Some(&"1") => handle_check_balance(parts),
        Some(&"2") => handle_send_money(parts),
        Some(&"3") => handle_buy_ckbtc(parts),
        Some(&"4") => handle_buy_ckusdc(parts),
        Some(&"5") => handle_withdraw(parts),
        Some(&"0") => ("Thank you for using AfriTokeni!".to_string(), false),
        _ => ("Invalid option. Please try again.".to_string(), false),
    }
}

fn handle_check_balance(parts: Vec<&str>) -> (String, bool) {
    // Fetch balance from Juno datastore asynchronously
    // For now, return placeholder - actual balance fetching happens in async context
    // The real implementation would use junobuild_satellite::get_doc_store()
    // to fetch from the "balances" collection
    
    let phone_number = parts.get(1).unwrap_or(&"");
    
    // Spawn async task to fetch balance
    if !phone_number.is_empty() {
        let phone = phone_number.to_string();
        ic_cdk::spawn(async move {
            match junobuild_satellite::get_doc_store(
                ic_cdk::caller(),
                "balances".to_string(),
                phone.clone(),
            ) {
                Some(doc) => {
                    ic_cdk::println!("✅ Balance fetched for {}", phone);
                    // Balance data would be decoded here
                }
                None => {
                    ic_cdk::println!("⚠️ No balance found for {}", phone);
                }
            }
        });
    }
    
    ("Your Balance:\nKES: 0\nckBTC: 0\nckUSDC: 0\nNote: Real-time balance coming soon".to_string(), false)
}

fn handle_send_money(parts: Vec<&str>) -> (String, bool) {
    match parts.len() {
        1 => ("Enter recipient phone number:".to_string(), true),
        2 => ("Enter amount (KES):".to_string(), true),
        3 => (
            format!("Sending {} KES to {}\nTransaction pending...", parts[2], parts[1]),
            false,
        ),
        _ => ("Invalid input.".to_string(), false),
    }
}

fn handle_buy_ckbtc(parts: Vec<&str>) -> (String, bool) {
    match parts.len() {
        1 => ("Enter amount in KES to convert to ckBTC:".to_string(), true),
        2 => (
            format!("Buying ckBTC with {} KES\nTransaction pending...", parts[1]),
            false,
        ),
        _ => ("Invalid input.".to_string(), false),
    }
}

fn handle_buy_ckusdc(parts: Vec<&str>) -> (String, bool) {
    match parts.len() {
        1 => ("Enter amount in KES to convert to ckUSDC:".to_string(), true),
        2 => (
            format!("Buying ckUSDC with {} KES\nTransaction pending...", parts[1]),
            false,
        ),
        _ => ("Invalid input.".to_string(), false),
    }
}

fn handle_withdraw(parts: Vec<&str>) -> (String, bool) {
    match parts.len() {
        1 => ("Enter amount to withdraw (KES):".to_string(), true),
        2 => (
            format!("Withdrawing {} KES\nPlease visit your nearest agent...", parts[1]),
            false,
        ),
        _ => ("Invalid input.".to_string(), false),
    }
}

/// Parse form-urlencoded data into a HashMap
fn parse_form_data(data: &str) -> std::collections::HashMap<String, String> {
    data.split('&')
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
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_form_data() {
        let data = "sessionId=123&phoneNumber=%2B254700000000&text=1";
        let params = parse_form_data(data);
        
        assert_eq!(params.get("sessionId"), Some(&"123".to_string()));
        assert_eq!(params.get("phoneNumber"), Some(&"+254700000000".to_string()));
        assert_eq!(params.get("text"), Some(&"1".to_string()));
    }

    #[test]
    fn test_process_ussd_menu_new_session() {
        let response = process_ussd_menu("", "+254700000000");
        assert!(response.starts_with("CON"));
        assert!(response.contains("Welcome to AfriTokeni"));
    }

    #[test]
    fn test_process_ussd_menu_check_balance() {
        let response = process_ussd_menu("1", "+254700000000");
        assert!(response.starts_with("END"));
        assert!(response.contains("Balance"));
    }
}
