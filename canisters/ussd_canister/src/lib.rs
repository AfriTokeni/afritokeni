use ic_cdk_macros::{query, update};

// Organized module structure
mod config_loader;
mod handlers;
mod models;
mod utils;

/// Test helper for integration tests (bypasses HTTP layer)
/// Returns (response_text, continue_session)
#[update]
async fn test_ussd_direct(session_id: String, phone_number: String, text: String) -> (String, bool) {
    // Call the USSD handler directly
    let req = handlers::http_handlers::HttpRequest {
        method: "POST".to_string(),
        url: "/api/ussd".to_string(),
        headers: vec![("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string())],
        body: format!("sessionId={}&phoneNumber={}&text={}", session_id, phone_number, text).into_bytes(),
    };
    
    let response = handlers::http_handlers::route_request_async(req).await;
    let body_text = String::from_utf8_lossy(&response.body).to_string();
    
    // Check if session continues (status 200 with CON prefix)
    let continues = response.status_code == 200 && body_text.starts_with("CON");
    
    // Remove CON/END prefix
    let clean_text = body_text.replace("CON ", "").replace("END ", "");
    
    (clean_text, continues)
}

/// HTTP request handler for GET requests (IC HTTP gateway)
#[query(manual_reply = true)]
fn http_request(req: handlers::http_handlers::HttpRequest) {
    ic_cdk::println!("📥 http_request (query): {} {}", req.method, req.url);
    let response = handlers::http_handlers::route_request(req);
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::msg_reply(&bytes);
}

/// HTTP request handler for POST requests (IC HTTP gateway)
/// Handles update calls that modify state
/// Routes:
/// - POST /api/ussd - USSD webhook handler
#[update(manual_reply = true)]
async fn http_request_update(req: handlers::http_handlers::HttpRequest) {
    ic_cdk::println!("📥 http_request_update (update): {} {}", req.method, req.url);
    let response = handlers::http_handlers::route_request_async(req).await;
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::msg_reply(&bytes);
}

// Export Candid interface
ic_cdk::export_candid!();
