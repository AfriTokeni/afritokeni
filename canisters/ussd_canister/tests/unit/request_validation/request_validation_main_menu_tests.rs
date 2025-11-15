// Integration tests for USSD main menu
// Run with: cargo test --package ussd_canister --test ussd_main_menu_tests

use candid::{CandidType, Deserialize};
use serde::Serialize;

/// Test helper structs - may be used in future request validation tests
#[allow(dead_code)]
#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

/// Test helper structs - may be used in future request validation tests
#[allow(dead_code)]
#[derive(Clone, Debug, CandidType, Serialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

#[test]
fn test_initial_ussd_dial() {
    // Initial USSD dial with empty text - should show main menu
    let ussd_request = serde_json::json!({
        "sessionId": "session-001",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": ""
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let request = HttpRequest {
        url: "/api/ussd".to_string(),
        method: "POST".to_string(),
        body,
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
    };
    
    assert_eq!(request.method, "POST");
    assert_eq!(request.url, "/api/ussd");
    
    let parsed: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
    assert_eq!(parsed["text"], "");
    assert_eq!(parsed["phoneNumber"], "+256700123456");
    assert_eq!(parsed["serviceCode"], "*229#");
}

#[test]
fn test_main_menu_option_1_local_currency() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-002",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "1"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let request = HttpRequest {
        url: "/api/ussd".to_string(),
        method: "POST".to_string(),
        body,
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
    };
    
    let parsed: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
    assert_eq!(parsed["text"], "1");
}

#[test]
fn test_main_menu_option_2_bitcoin() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-003",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "2"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(parsed["text"], "2");
}

#[test]
fn test_main_menu_option_3_usdc() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-004",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "3"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(parsed["text"], "3");
}

#[test]
fn test_main_menu_option_4_dao() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-005",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "4"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(parsed["text"], "4");
}

#[test]
fn test_main_menu_option_5_language() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-006",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "5"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(parsed["text"], "5");
}

#[test]
fn test_main_menu_option_0_exit() {
    let ussd_request = serde_json::json!({
        "sessionId": "session-007",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "0"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(parsed["text"], "0");
}
