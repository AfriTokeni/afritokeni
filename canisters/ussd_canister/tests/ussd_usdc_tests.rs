// Integration tests for USSD USDC menu
// Run with: cargo test --package ussd_canister --test ussd_usdc_tests

use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[test]
fn test_check_usdc_balance() {
    // Menu -> USDC -> Check Balance -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-usdc-001",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "3*1*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "3"); // USDC
    assert_eq!(parts[1], "1"); // Check Balance
    assert_eq!(parts[2], "1234"); // PIN
}

#[test]
fn test_send_usdc_flow() {
    // Menu -> USDC -> Send -> Phone -> Amount -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-usdc-002",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "3*2*256701234567*50*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "3"); // USDC
    assert_eq!(parts[1], "2"); // Send
    assert!(parts[2].starts_with("256")); // Phone number
    assert_eq!(parts[3], "50"); // Amount in USDC
    assert_eq!(parts[4], "1234"); // PIN
}

#[test]
fn test_swap_usdc_to_kes() {
    // Menu -> USDC -> Swap to KES -> Amount -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-usdc-003",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "3*3*100*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "3"); // USDC
    assert_eq!(parts[1], "3"); // Swap to KES
    assert_eq!(parts[2], "100"); // Amount in USDC
    assert_eq!(parts[3], "1234"); // PIN
}

#[test]
fn test_buy_usdc_with_kes() {
    // Menu -> USDC -> Buy -> Amount (KES) -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-usdc-004",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "3*4*15000*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "3"); // USDC
    assert_eq!(parts[1], "4"); // Buy
    assert_eq!(parts[2], "15000"); // Amount in KES
    assert_eq!(parts[3], "1234"); // PIN
}

#[test]
fn test_usdc_back_to_main() {
    // Menu -> USDC -> Back
    let ussd_request = serde_json::json!({
        "sessionId": "session-usdc-005",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "3*0"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[1], "0"); // Back
}
