use ic_cdk_macros::{query, update};

// Organized module structure
mod config_loader;
mod handlers;
mod models;
mod utils;

/// HTTP request handler for GET requests (IC HTTP gateway)
#[query(manual_reply = true)]
fn http_request(req: handlers::http_handlers::HttpRequest) {
    handlers::http_handlers::route_request(req)
}

/// HTTP request handler for POST requests (USSD webhook)
/// 
/// Routes:
/// - POST /api/ussd - USSD webhook handler
#[update(manual_reply = true)]
fn http_request_update(req: handlers::http_handlers::HttpRequest) {
    handlers::http_handlers::route_request(req)
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

/// Admin endpoint: Get user balance for verification
#[query]
async fn admin_get_balance(phone_number: String) -> Result<(f64, f64, f64), String> {
    handlers::admin::get_user_balance(&phone_number).await
}

/// Admin endpoint: Clear all test data
#[update]
async fn admin_clear_data() -> Result<String, String> {
    handlers::admin::clear_test_data().await?;
    Ok("All test data cleared".to_string())
}

// Export Candid interface
ic_cdk::export_candid!();
