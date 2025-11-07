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
        ("POST", "/api/ussd") => handle_ussd_webhook(req),
        _ => not_found(),
    }
}

/// Health check endpoint
fn health_check() -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: b"{\"status\":\"ok\"}".to_vec(),
    };
    ManualReply::one(response)
}

/// 404 Not Found response
fn not_found() -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 404,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: b"Not Found".to_vec(),
    };
    ManualReply::one(response)
}

/// Error response helper
fn error_response(status_code: u16, message: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: message.as_bytes().to_vec(),
    };
    ManualReply::one(response)
}

/// Handle USSD webhook from Africa's Talking
fn handle_ussd_webhook(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Delegate to ussd module
    crate::handlers::ussd::handle_ussd_webhook(req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_structure() {
        let req = HttpRequest {
            method: "POST".to_string(),
            url: "/api/ussd".to_string(),
            headers: vec![],
            body: vec![],
        };
        assert_eq!(req.method, "POST");
        assert_eq!(req.url, "/api/ussd");
    }

    #[test]
    fn test_http_response_structure() {
        let resp = HttpResponse {
            status_code: 200,
            headers: vec![],
            body: b"OK".to_vec(),
        };
        assert_eq!(resp.status_code, 200);
    }
}
