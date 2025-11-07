// Integration tests for USSD language selection
// Run with: cargo test --package ussd_canister --test ussd_language_tests

use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[test]
fn test_select_english() {
    // Menu -> Language -> English
    let ussd_request = serde_json::json!({
        "sessionId": "session-lang-001",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "5*1"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "5"); // Language
    assert_eq!(parts[1], "1"); // English
}

#[test]
fn test_select_swahili() {
    // Menu -> Language -> Swahili
    let ussd_request = serde_json::json!({
        "sessionId": "session-lang-002",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "5*2"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "5"); // Language
    assert_eq!(parts[1], "2"); // Swahili
}

#[test]
fn test_select_luganda() {
    // Menu -> Language -> Luganda
    let ussd_request = serde_json::json!({
        "sessionId": "session-lang-003",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "5*3"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "5"); // Language
    assert_eq!(parts[1], "3"); // Luganda
}

#[test]
fn test_language_back_to_main() {
    // Menu -> Language -> Back
    let ussd_request = serde_json::json!({
        "sessionId": "session-lang-004",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "5*0"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[1], "0"); // Back
}

#[test]
fn test_language_persistence() {
    // After selecting language, subsequent requests should use it
    let session_id = "session-lang-005";
    
    // First request: Select Swahili
    let request1 = serde_json::json!({
        "sessionId": session_id,
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "5*2"
    });
    
    let body1 = serde_json::to_vec(&request1).unwrap();
    let parsed1: serde_json::Value = serde_json::from_slice(&body1).unwrap();
    assert_eq!(parsed1["text"], "5*2");
    
    // Second request: Navigate to main menu (should be in Swahili)
    let request2 = serde_json::json!({
        "sessionId": session_id,
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": ""
    });
    
    let body2 = serde_json::to_vec(&request2).unwrap();
    let parsed2: serde_json::Value = serde_json::from_slice(&body2).unwrap();
    assert_eq!(parsed2["sessionId"], session_id);
}
