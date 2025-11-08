use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

// Organized module structure
mod config_loader;
mod handlers;
mod models;
mod utils;

/// HTTP request handler for GET requests (IC HTTP gateway)
#[query(manual_reply = true)]
fn http_request(req: handlers::http_handlers::HttpRequest) {
    ic_cdk::println!("📥 http_request (query): {} {}", req.method, req.url);
    let response = handlers::http_handlers::route_request(req);
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::call::reply_raw(&bytes);
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
    ic_cdk::api::call::reply_raw(&bytes);
}

/// Admin endpoint: Set user balance for testing
#[update]
async fn admin_set_balance(phone_number: String, kes: f64, ckbtc: f64, ckusdc: f64) -> Result<String, String> {
    handlers::admin::set_user_balance(&phone_number, kes, ckbtc, ckusdc).await?;
    Ok(format!("Balance set for {}: {} KES, {} ckBTC, {} ckUSDC", phone_number, kes, ckbtc, ckusdc))
}

/// Admin endpoint: Set user PIN for testing
#[update]
async fn admin_set_pin(phone_number: String, pin: String) -> Result<String, String> {
    handlers::admin::set_user_pin(&phone_number, &pin).await?;
    Ok(format!("PIN set for {}", phone_number))
}

/// Test endpoint: Call USSD handler and return response (for integration tests)
#[update]
async fn test_ussd(session_id: String, phone_number: String, text: String) -> String {
    let request_body = format!(r#"{{"sessionId":"{}","phoneNumber":"{}","text":"{}"}}"#, session_id, phone_number, text);
    let req = handlers::http_handlers::HttpRequest {
        method: "POST".to_string(),
        url: "/api/ussd".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: request_body.as_bytes().to_vec(),
    };
    
    let response = handlers::http_handlers::route_request_async(req).await;
    String::from_utf8(response.body).unwrap_or_else(|_| "Invalid UTF-8".to_string())
}

/// Admin endpoint: Get user balance for verification
#[query]
async fn admin_get_balance(phone_number: String) -> Result<(f64, f64, f64), String> {
    handlers::admin::get_user_balance(&phone_number).await
}

/// Admin endpoint: Reset PIN attempts for a user
#[update]
async fn admin_reset_pin_attempts(phone_number: String) -> Result<String, String> {
    handlers::admin::reset_pin_attempts(&phone_number).await?;
    Ok(format!("PIN attempts reset for {}", phone_number))
}

/// Admin endpoint: Setup test user (all-in-one for faster tests)
#[update]
async fn admin_setup_test_user(phone_number: String, pin: String, kes: f64, ckbtc: f64, ckusdc: f64) -> Result<String, String> {
    handlers::admin::setup_test_user(&phone_number, &pin, kes, ckbtc, ckusdc).await?;
    Ok(format!("Test user {} setup complete", phone_number))
}

// Export Candid interface
ic_cdk::export_candid!();
