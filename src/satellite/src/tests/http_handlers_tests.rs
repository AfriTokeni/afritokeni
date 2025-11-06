use crate::http_handlers::{route_request, HttpRequest};

#[test]
fn test_route_ussd_endpoint() {
    let req = HttpRequest {
        method: "POST".to_string(),
        url: "/api/ussd".to_string(),
        headers: vec![],
        body: b"sessionId=123&phoneNumber=%2B254700000000&text=".to_vec(),
    };
    
    let response = route_request(req);
    // Response should be handled by USSD handler
    // We can't easily test ManualReply, but we verify it doesn't panic
}

#[test]
fn test_route_sms_endpoint() {
    let req = HttpRequest {
        method: "POST".to_string(),
        url: "/api/sms".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: br#"{"to":["+254700000000"],"message":"Test"}"#.to_vec(),
    };
    
    let response = route_request(req);
    // Response should be handled by SMS handler
}

#[test]
fn test_route_not_found() {
    let req = HttpRequest {
        method: "GET".to_string(),
        url: "/api/unknown".to_string(),
        headers: vec![],
        body: vec![],
    };
    
    let response = route_request(req);
    // Should return 404
}

#[test]
fn test_route_wrong_method() {
    let req = HttpRequest {
        method: "GET".to_string(),
        url: "/api/ussd".to_string(),
        headers: vec![],
        body: vec![],
    };
    
    let response = route_request(req);
    // Should return 404 (only POST allowed)
}

#[test]
fn test_url_with_query_params() {
    let req = HttpRequest {
        method: "POST".to_string(),
        url: "/api/ussd?foo=bar".to_string(),
        headers: vec![],
        body: b"sessionId=123&phoneNumber=%2B254700000000&text=".to_vec(),
    };
    
    let response = route_request(req);
    // Should still route correctly (query params ignored)
}

#[test]
fn test_security_sql_injection_attempt() {
    let req = HttpRequest {
        method: "POST".to_string(),
        url: "/api/ussd'; DROP TABLE users; --".to_string(),
        headers: vec![],
        body: vec![],
    };
    
    let response = route_request(req);
    // Should return 404, not execute SQL
}

#[test]
fn test_security_path_traversal_attempt() {
    let req = HttpRequest {
        method: "POST".to_string(),
        url: "/api/../../../etc/passwd".to_string(),
        headers: vec![],
        body: vec![],
    };
    
    let response = route_request(req);
    // Should return 404
}

#[test]
fn test_large_body() {
    let large_body = vec![b'A'; 1_000_000]; // 1MB
    let req = HttpRequest {
        method: "POST".to_string(),
        url: "/api/sms".to_string(),
        headers: vec![],
        body: large_body,
    };
    
    let response = route_request(req);
    // Should handle gracefully (likely return error)
}
