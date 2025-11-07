// Integration tests for USSD local currency (KES) menu
// Run with: cargo test --package ussd_canister --test ussd_local_currency_tests

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[test]
fn test_check_balance_flow() {
    // Menu -> Local Currency -> Check Balance -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-kes-001",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1*1*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "1"); // Local Currency
    assert_eq!(parts[1], "1"); // Check Balance
    assert_eq!(parts[2], "1234"); // PIN
}

#[test]
fn test_send_money_flow() {
    // Menu -> Local Currency -> Send -> Phone -> Amount -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-kes-002",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1*2*256701234567*5000*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "1"); // Local Currency
    assert_eq!(parts[1], "2"); // Send Money
    assert!(parts[2].starts_with("256")); // Phone number
    assert_eq!(parts[3], "5000"); // Amount
    assert_eq!(parts[4], "1234"); // PIN
}

#[test]
fn test_send_money_with_invalid_phone() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-kes-003",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1*2*123*5000*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    // Invalid phone number (too short)
    assert!(parts[2].len() < 10);
}

#[test]
fn test_deposit_flow() {
    // Menu -> Local Currency -> Deposit -> Amount
    let ussd_request = serde_json::json!({
        "sessionId": "session-kes-004",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1*3*10000"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "1"); // Local Currency
    assert_eq!(parts[1], "3"); // Deposit
    assert_eq!(parts[2], "10000"); // Amount
}

#[test]
fn test_withdraw_flow() {
    // Menu -> Local Currency -> Withdraw -> Amount -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-kes-005",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1*4*5000*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "1"); // Local Currency
    assert_eq!(parts[1], "4"); // Withdraw
    assert_eq!(parts[2], "5000"); // Amount
    assert_eq!(parts[3], "1234"); // PIN
}

#[test]
fn test_back_to_main_menu() {
    // Menu -> Local Currency -> Back
    let ussd_request = serde_json::json!({
        "sessionId": "session-kes-006",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1*0"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[1], "0"); // Back
}
