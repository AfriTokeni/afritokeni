use candid::{CandidType, Deserialize};
use ic_cdk::api::call::ManualReply;
use serde::Serialize;

/// HTTP Request structure (matches IC HTTP Gateway Protocol)
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// HTTP Response structure (matches IC HTTP Gateway Protocol)
#[derive(Clone, Debug, CandidType, Serialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Route incoming HTTP requests to appropriate handlers
pub fn route_request(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Parse URL path
    let path = req.url.split('?').next().unwrap_or(&req.url);
    
    match (req.method.as_str(), path) {
        ("GET", "/api/health") => health_check(),
        ("POST", "/api/ussd") => crate::ussd::handle_ussd_webhook(req),
        ("POST", "/api/sms") => crate::sms::handle_sms_webhook(req),
        _ => not_found(),
    }
}

/// Health check endpoint
fn health_check() -> ManualReply<HttpResponse> {
    let health_data = serde_json::json!({
        "status": "healthy",
        "service": "AfriTokeni Satellite Functions",
        "timestamp": ic_cdk::api::time() / 1_000_000_000, // Convert nanoseconds to seconds
        "endpoints": {
            "ussd": "/api/ussd",
            "sms": "/api/sms",
            "health": "/api/health"
        }
    });
    
    ok_response(health_data.to_string().into_bytes(), "application/json")
}

/// Return 404 Not Found response
fn not_found() -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 404,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: b"Not Found".to_vec(),
    };
    ManualReply::one(response)
}

/// Helper to create success response
pub fn ok_response(body: Vec<u8>, content_type: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), content_type.to_string())],
        body,
    };
    ManualReply::one(response)
}

/// Helper to create error response
pub fn error_response(status: u16, message: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: status,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: message.as_bytes().to_vec(),
    };
    ManualReply::one(response)
}
