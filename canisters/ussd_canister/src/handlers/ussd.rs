use crate::handlers::http_handlers::{HttpRequest, HttpResponse};
use std::str;

/// Helper to create error response
fn error_response(status_code: u16, message: &str) {
    let response = HttpResponse {
        status_code,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: message.as_bytes().to_vec(),
    };
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::call::reply_raw(&bytes);
}

/// Helper to create OK response
fn ok_response(body: &str) {
    let response = HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), "text/plain; charset=utf-8".to_string())],
        body: body.as_bytes().to_vec(),
    };
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::call::reply_raw(&bytes);
}

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
pub async fn handle_ussd_webhook(req: HttpRequest) {
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
    
    // Check rate limit
    if !crate::utils::rate_limit::check_rate_limit(&phone_number) {
        // Rate limit error - use English as fallback since we don't have session yet
        let lang = crate::utils::translations::Language::English;
        let message = crate::utils::translations::TranslationService::translate("rate_limit_exceeded", lang);
        return error_response(429, message);
    }
    
    // Periodically clean up old rate limit entries (every ~10th request)
    if ic_cdk::api::time() % 10 == 0 {
        crate::utils::rate_limit::cleanup_old_entries();
    }
    
    // Log the request
    ic_cdk::println!(
        "📱 USSD Request - Session: {}, Phone: {}, Text: '{}', JSON: {}",
        session_id,
        phone_number,
        text,
        is_json
    );
    
    // Process USSD request and get response
    let (response_text, continue_session) = {
        // Get or create session
        match crate::models::session::get_or_create_session(&session_id, &phone_number).await {
            Ok(mut session) => {
                // Process menu with async handlers
                let (response_text, continue_session) = if session.current_menu.is_empty() {
                    // Main menu
                    crate::handlers::ussd_handlers::handle_main_menu(&text, &mut session).await
                } else {
                    // Route to submenu
                    match session.current_menu.as_str() {
                        "register" => {
                            // Extract PIN from input
                            let parts: Vec<&str> = text.split('*').collect();
                            let pin = parts.last().unwrap_or(&"");
                            crate::handlers::ussd_handlers::handle_registration(&mut session, pin).await
                        },
                        "local_currency" => crate::handlers::ussd_handlers::handle_local_currency_menu(&text, &mut session).await,
                        "bitcoin" => crate::handlers::ussd_handlers::handle_bitcoin_menu(&text, &mut session).await,
                        "usdc" => crate::handlers::ussd_handlers::handle_usdc_menu(&text, &mut session).await,
                        "dao" => crate::handlers::ussd_handlers::handle_dao_menu(&text, &mut session).await,
                        "language" => crate::handlers::ussd_handlers::handle_language_menu(&text, &mut session).await,
                        _ => crate::handlers::ussd_handlers::handle_main_menu(&text, &mut session).await,
                    }
                };
                
                if continue_session {
                    // Save session for next interaction
                    if let Err(e) = crate::models::session::save_session(&session).await {
                        ic_cdk::println!("❌ Failed to save session: {}", e);
                    }
                } else {
                    // Session ended, delete it
                    if let Err(e) = crate::models::session::delete_session(&session_id).await {
                        ic_cdk::println!("❌ Failed to delete session: {}", e);
                    }
                }
                
                ic_cdk::println!("✅ USSD processed: continue={}, response_len={}", continue_session, response_text.len());
                (response_text, continue_session)
            }
            Err(e) => {
                ic_cdk::println!("❌ Session error: {}", e);
                (format!("Error: {}", e), false)
            }
        }
    };
    
    // Return response based on request type
    if is_json {
        // JSON response for playground
        let json_response = serde_json::json!({
            "continueSession": continue_session,
            "response": response_text
        });
        ok_response(&json_response.to_string())
    } else {
        // Plain text response for Africa's Talking
        // CON = Continue session, END = End session
        let prefix = if continue_session { "CON" } else { "END" };
        let ussd_response = format!("{} {}", prefix, response_text);
        ok_response(&ussd_response)
    }
}

/// Parse JSON USSD request
fn parse_json_request(body: &str) -> Result<(String, String, String), String> {
    let json: serde_json::Value = serde_json::from_str(body)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    let session_id = json["sessionId"].as_str().unwrap_or("").to_string();
    let phone_number = json["phoneNumber"].as_str().unwrap_or("").to_string();
    let text = json["text"].as_str().unwrap_or("").to_string();
    
    Ok((session_id, phone_number, text))
}

/// Parse form-urlencoded data
fn parse_form_data(body: &str) -> std::collections::HashMap<String, String> {
    body.split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((
                urlencoding::decode(key).ok()?.to_string(),
                urlencoding::decode(value).ok()?.to_string(),
            ))
        })
        .collect()
}

/// Process USSD menu - simplified version (DEPRECATED - use ussd_handlers instead)
#[allow(dead_code)]
fn process_ussd_menu(_text: &str, _phone_number: &str) -> (String, bool) {
    // This function is deprecated and not used
    // All menu logic is now in ussd_handlers.rs with proper translations
    (String::new(), false)
}
