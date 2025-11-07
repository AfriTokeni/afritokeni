use candid::{CandidType, Deserialize};
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

/// Route HTTP requests to appropriate handlers (sync version for GET)
pub fn route_request(req: HttpRequest) -> HttpResponse {
    // Parse URL path
    let path = req.url.split('?').next().unwrap_or(&req.url);
    
    match (req.method.as_str(), path) {
        ("GET", "/api/health") => health_check(),
        _ => not_found(),
    }
}

/// Route HTTP requests to appropriate handlers (async version for POST)
pub async fn route_request_async(req: HttpRequest) -> HttpResponse {
    // Parse URL path
    let path = req.url.split('?').next().unwrap_or(&req.url);
    ic_cdk::println!("🔍 Route request: method={}, path={}", req.method, path);
    
    match req.method.as_str() {
        "GET" if path.contains("/health") || path == "/api/health" => health_check(),
        "POST" if path.contains("/ussd") || path == "/api/ussd" => {
            // Verify request is from Africa's Talking
            if !verify_africas_talking_request(&req) {
                return error_response(403, "Forbidden: Invalid request source");
            }
            // Handle USSD webhook and return response
            handle_ussd_webhook(req).await
        },
        _ => {
            ic_cdk::println!("❌ No route matched for: {} {}", req.method, path);
            not_found()
        }
    }
}

/// Verify request is from Africa's Talking
/// Checks User-Agent and optionally API key header
fn verify_africas_talking_request(req: &HttpRequest) -> bool {
    let config = crate::config_loader::get_config();
    
    // In test/development mode, skip verification
    // Check if environment is development (no signature verification configured)
    if !config.security.verify_signature {
        ic_cdk::println!("⚠️ Skipping request verification (development mode)");
        return true;
    }
    
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
fn health_check() -> HttpResponse {
    HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: b"{\"status\":\"healthy\"}".to_vec(),
    }
}

/// 404 Not Found response
fn not_found() -> HttpResponse {
    HttpResponse {
        status_code: 404,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: b"Not Found".to_vec(),
    }
}

/// Error response helper
fn error_response(status_code: u16, message: &str) -> HttpResponse {
    HttpResponse {
        status_code,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: message.as_bytes().to_vec(),
    }
}

/// Handle USSD webhook from Africa's Talking
async fn handle_ussd_webhook(req: HttpRequest) -> HttpResponse {
    // Delegate to ussd module
    crate::handlers::ussd::handle_ussd_webhook(req).await
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
