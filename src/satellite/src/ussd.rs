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
    
    // Process USSD menu
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

/// Process USSD menu navigation
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

fn handle_check_balance(_parts: Vec<&str>) -> (String, bool) {
    // TODO: Fetch balance from Juno datastore
    ("Your Balance:\nKES: 0\nckBTC: 0\nckUSDC: 0".to_string(), false)
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
