use crate::http_handlers::{error_response, ok_response, HttpRequest, HttpResponse};
use ic_cdk::api::call::ManualReply;
use std::str;

/// Handle USSD webhook from Africa's Talking
/// 
/// Expected POST body (form-urlencoded):
/// - sessionId: string
/// - serviceCode: string
/// - phoneNumber: string
/// - text: string
pub fn handle_ussd_webhook(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Parse form data from request body
    let body_str = match str::from_utf8(&req.body) {
        Ok(s) => s,
        Err(_) => return error_response(400, "Invalid UTF-8 in request body"),
    };
    
    // Parse form-urlencoded data
    let params = parse_form_data(body_str);
    
    let session_id = params.get("sessionId").map(|s| s.as_str()).unwrap_or("");
    let phone_number = params.get("phoneNumber").map(|s| s.as_str()).unwrap_or("");
    let text = params.get("text").map(|s| s.as_str()).unwrap_or("");
    
    // Log the request
    ic_cdk::println!(
        "USSD Request - Session: {}, Phone: {}, Text: '{}'",
        session_id,
        phone_number,
        text
    );
    
    // Process USSD menu
    let response_text = process_ussd_menu(text, phone_number);
    
    // Return USSD response (text/plain with CON or END prefix)
    ok_response(response_text.into_bytes(), "text/plain")
}

/// Process USSD menu navigation
fn process_ussd_menu(input: &str, _phone_number: &str) -> String {
    let parts: Vec<&str> = input.split('*').collect();
    
    // New session (empty input)
    if input.is_empty() {
        return format!(
            "CON Welcome to AfriTokeni\n\
            1. Check Balance\n\
            2. Send Money\n\
            3. Buy ckBTC\n\
            4. Buy ckUSDC\n\
            5. Withdraw\n\
            0. Exit"
        );
    }
    
    // Handle menu selection
    match parts.get(0) {
        Some(&"1") => handle_check_balance(parts),
        Some(&"2") => handle_send_money(parts),
        Some(&"3") => handle_buy_ckbtc(parts),
        Some(&"4") => handle_buy_ckusdc(parts),
        Some(&"5") => handle_withdraw(parts),
        Some(&"0") => "END Thank you for using AfriTokeni!".to_string(),
        _ => "END Invalid option. Please try again.".to_string(),
    }
}

fn handle_check_balance(_parts: Vec<&str>) -> String {
    // TODO: Fetch balance from Juno datastore
    "END Your Balance:\nKES: 0\nckBTC: 0\nckUSDC: 0".to_string()
}

fn handle_send_money(parts: Vec<&str>) -> String {
    match parts.len() {
        1 => "CON Enter recipient phone number:".to_string(),
        2 => "CON Enter amount (KES):".to_string(),
        3 => format!(
            "END Sending {} KES to {}\nTransaction pending...",
            parts[2], parts[1]
        ),
        _ => "END Invalid input.".to_string(),
    }
}

fn handle_buy_ckbtc(parts: Vec<&str>) -> String {
    match parts.len() {
        1 => "CON Enter amount in KES to convert to ckBTC:".to_string(),
        2 => format!(
            "END Buying ckBTC with {} KES\nTransaction pending...",
            parts[1]
        ),
        _ => "END Invalid input.".to_string(),
    }
}

fn handle_buy_ckusdc(parts: Vec<&str>) -> String {
    match parts.len() {
        1 => "CON Enter amount in KES to convert to ckUSDC:".to_string(),
        2 => format!(
            "END Buying ckUSDC with {} KES\nTransaction pending...",
            parts[1]
        ),
        _ => "END Invalid input.".to_string(),
    }
}

fn handle_withdraw(parts: Vec<&str>) -> String {
    match parts.len() {
        1 => "CON Enter amount to withdraw (KES):".to_string(),
        2 => format!(
            "END Withdrawing {} KES\nPlease visit your nearest agent...",
            parts[1]
        ),
        _ => "END Invalid input.".to_string(),
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
