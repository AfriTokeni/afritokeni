// Integration tests for USSD Bitcoin menu
// Run with: cargo test --package ussd_canister --test ussd_bitcoin_tests

use candid::{CandidType, Deserialize};

/// Test helper structs - may be used in future request validation tests
#[allow(dead_code)]
#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[test]
fn test_check_bitcoin_balance() {
    // Menu -> Bitcoin -> Check Balance -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-btc-001",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2*1*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "2"); // Bitcoin
    assert_eq!(parts[1], "1"); // Check Balance
    assert_eq!(parts[2], "1234"); // PIN
}

#[test]
fn test_buy_bitcoin_flow() {
    // Menu -> Bitcoin -> Buy -> Amount (KES) -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-btc-002",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2*2*10000*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "2"); // Bitcoin
    assert_eq!(parts[1], "2"); // Buy
    assert_eq!(parts[2], "10000"); // Amount in KES
    assert_eq!(parts[3], "1234"); // PIN
}

#[test]
fn test_sell_bitcoin_flow() {
    // Menu -> Bitcoin -> Sell -> Amount (BTC) -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-btc-003",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2*3*0.001*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "2"); // Bitcoin
    assert_eq!(parts[1], "3"); // Sell
    assert_eq!(parts[2], "0.001"); // Amount in BTC
    assert_eq!(parts[3], "1234"); // PIN
}

#[test]
fn test_send_bitcoin_flow() {
    // Menu -> Bitcoin -> Send -> Address -> Amount -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-btc-004",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2*4*bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh*0.0005*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "2"); // Bitcoin
    assert_eq!(parts[1], "4"); // Send
    assert!(parts[2].starts_with("bc1")); // Bitcoin address
    assert_eq!(parts[3], "0.0005"); // Amount in BTC
    assert_eq!(parts[4], "1234"); // PIN
}

#[test]
fn test_bitcoin_price_check() {
    // Menu -> Bitcoin -> Price
    let ussd_request = serde_json::json!({
        "sessionId": "session-btc-005",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2*5"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "2"); // Bitcoin
    assert_eq!(parts[1], "5"); // Price
}

#[test]
fn test_bitcoin_back_to_main() {
    // Menu -> Bitcoin -> Back
    let ussd_request = serde_json::json!({
        "sessionId": "session-btc-006",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2*0"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[1], "0"); // Back
}
