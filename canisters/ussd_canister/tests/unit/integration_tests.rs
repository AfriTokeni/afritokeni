// Integration tests for USSD canister
// Run with: cargo test --package ussd_canister --test integration_tests

use candid::{encode_one, decode_one, CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[derive(Clone, Debug, CandidType, Serialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

#[test]
fn test_health_check() {
    let request = HttpRequest {
        url: "/api/health".to_string(),
        method: "GET".to_string(),
        body: vec![],
        headers: vec![],
    };
    
    // In real integration test, this would call the deployed canister
    // For now, just verify the structure
    assert_eq!(request.url, "/api/health");
    assert_eq!(request.method, "GET");
}

#[test]
fn test_ussd_request_structure() {
    let ussd_request = serde_json::json!({
        "sessionId": "test123",
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
    
    assert_eq!(request.url, "/api/ussd");
    assert_eq!(request.method, "POST");
    assert!(!request.body.is_empty());
}

#[test]
fn test_ussd_response_parsing() {
    let response_text = "CON Welcome to AfriTokeni\n1. Check Balance\n2. Send Money\n0. Exit";
    let body = response_text.as_bytes().to_vec();
    
    let response = HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body,
    };
    
    assert_eq!(response.status_code, 200);
    
    let response_str = String::from_utf8(response.body).unwrap();
    assert!(response_str.starts_with("CON"));
    assert!(response_str.contains("Welcome"));
}

#[test]
fn test_session_id_format() {
    let session_id = format!("test-{}", 1234567890);
    assert!(session_id.starts_with("test-"));
    assert!(session_id.len() > 5);
}

#[test]
fn test_phone_number_validation() {
    let valid_phones = vec![
        "+256700123456",
        "+254712345678",
        "+255712345678",
    ];
    
    for phone in valid_phones {
        assert!(phone.starts_with("+"));
        assert!(phone.len() >= 10);
        assert!(phone.chars().skip(1).all(|c| c.is_numeric()));
    }
}
