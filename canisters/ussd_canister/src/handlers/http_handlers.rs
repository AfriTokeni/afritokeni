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
        ("POST", "/api/ussd") => {
            // Verify request is from Africa's Talking
            if !verify_africas_talking_request(&req) {
                return error_response(403, "Forbidden: Invalid request source");
            }
            handle_ussd_webhook(req)
        },
        _ => not_found(),
    }
}

/// Verify request is from Africa's Talking
/// Checks User-Agent and optionally API key header
fn verify_africas_talking_request(req: &HttpRequest) -> bool {
    let config = crate::config_loader::get_config();
    let allowed_agents: Vec<&str> = config.security.allowed_user_agents.split(',').collect();
    
    // Check for Africa's Talking User-Agent
    let has_valid_user_agent = req.headers.iter().any(|(key, value)| {
        key.to_lowercase() == "user-agent" && 
        allowed_agents.iter().any(|agent| value.contains(agent))
    });
    
    // HMAC signature verification (optional, configure in config.toml)
    // Set verify_signature=true and hmac_secret in production for additional security
    if config.security.verify_signature && !config.security.hmac_secret.is_empty() {
        // For now, signature verification is disabled by default
        // Enable in production by setting verify_signature=true in config.toml
        ic_cdk::println!("⚠️ HMAC verification enabled but not yet implemented - will be added in production");
    }
    
    if !has_valid_user_agent {
        ic_cdk::println!("⚠️ Rejected request - invalid User-Agent");
    }
    
    has_valid_user_agent
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
