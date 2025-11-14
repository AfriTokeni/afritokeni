use ic_cdk_macros::{init, query, update};
use candid::Principal;

// Organized module structure
mod config_loader;
mod api;
pub mod core;
pub mod flows;
pub mod logic;
pub mod services;
pub mod utils;

/// Initialize USSD canister
#[init]
fn init() {
    ic_cdk::println!("🔧 USSD canister initialized - use configure_domain_canisters to set canister IDs");
}

// ============================================================================
// DOMAIN CANISTER CONFIGURATION
// ============================================================================

/// Set User Canister ID
#[update]
fn set_user_canister_id(principal: Principal) -> Result<(), String> {
    services::user_client::set_user_canister_id(principal);
    ic_cdk::println!("✅ User Canister ID set to: {}", principal);
    Ok(())
}

/// Set Wallet Canister ID
#[update]
fn set_wallet_canister_id(principal: Principal) -> Result<(), String> {
    services::wallet_client::set_wallet_canister_id(principal);
    ic_cdk::println!("✅ Wallet Canister ID set to: {}", principal);
    Ok(())
}

/// Set Crypto Canister ID
#[update]
fn set_crypto_canister_id(principal: Principal) -> Result<(), String> {
    services::crypto_client::set_crypto_canister_id(principal);
    ic_cdk::println!("✅ Crypto Canister ID set to: {}", principal);
    Ok(())
}

/// Set Agent Canister ID
#[update]
fn set_agent_canister_id(principal: Principal) -> Result<(), String> {
    services::agent_client::set_agent_canister_id(principal);
    ic_cdk::println!("✅ Agent Canister ID set to: {}", principal);
    Ok(())
}

/// Configure all domain canisters at once (convenience function)
#[update]
fn configure_domain_canisters(
    user_canister_id: Principal,
    wallet_canister_id: Principal,
    crypto_canister_id: Principal,
    agent_canister_id: Principal,
) -> Result<(), String> {
    set_user_canister_id(user_canister_id)?;
    set_wallet_canister_id(wallet_canister_id)?;
    set_crypto_canister_id(crypto_canister_id)?;
    set_agent_canister_id(agent_canister_id)?;
    
    ic_cdk::println!("✅ All domain canisters configured successfully");
    Ok(())
}

/// USSD endpoint for integration tests
/// Returns (response_text, continue_session)
#[update]
async fn ussd(session_id: String, phone_number: String, text: String) -> (String, bool) {
    // Call the USSD handler directly
    let req = api::http::HttpRequest {
        method: "POST".to_string(),
        url: "/api/ussd".to_string(),
        headers: vec![("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string())],
        body: format!("sessionId={}&phoneNumber={}&text={}", session_id, phone_number, text).into_bytes(),
    };
    
    let response = api::http::route_request_async(req).await;
    let body_text = String::from_utf8_lossy(&response.body).to_string();
    
    // Check if session continues (status 200 with CON prefix)
    let continues = response.status_code == 200 && body_text.starts_with("CON");
    
    // Remove CON/END prefix
    let clean_text = body_text.replace("CON ", "").replace("END ", "");
    
    (clean_text, continues)
}

/// HTTP request handler for GET requests (IC HTTP gateway)
#[query(manual_reply = true)]
fn http_request(req: api::http::HttpRequest) {
    ic_cdk::println!("📥 http_request (query): {} {}", req.method, req.url);
    let response = api::http::route_request(req);

    match candid::encode_one(&response) {
        Ok(bytes) => ic_cdk::api::msg_reply(&bytes),
        Err(e) => {
            ic_cdk::println!("❌ Failed to encode response: {}", e);
            // Return HTTP 500 error response
            let error_response = api::http::HttpResponse {
                status_code: 500,
                headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
                body: b"Internal Server Error: Failed to encode response".to_vec(),
            };
            if let Ok(error_bytes) = candid::encode_one(&error_response) {
                ic_cdk::api::msg_reply(&error_bytes);
            } else {
                // Last resort - reply with empty bytes
                ic_cdk::api::msg_reply(&[]);
            }
        }
    }
}

/// HTTP request handler for POST requests (IC HTTP gateway)
/// Handles update calls that modify state
/// Routes:
/// - POST /api/ussd - USSD webhook handler
#[update(manual_reply = true)]
async fn http_request_update(req: api::http::HttpRequest) {
    ic_cdk::println!("📥 http_request_update (update): {} {}", req.method, req.url);
    let response = api::http::route_request_async(req).await;

    match candid::encode_one(&response) {
        Ok(bytes) => ic_cdk::api::msg_reply(&bytes),
        Err(e) => {
            ic_cdk::println!("❌ Failed to encode response: {}", e);
            // Return HTTP 500 error response
            let error_response = api::http::HttpResponse {
                status_code: 500,
                headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
                body: b"Internal Server Error: Failed to encode response".to_vec(),
            };
            if let Ok(error_bytes) = candid::encode_one(&error_response) {
                ic_cdk::api::msg_reply(&error_bytes);
            } else {
                // Last resort - reply with empty bytes
                ic_cdk::api::msg_reply(&[]);
            }
        }
    }
}

// Export Candid interface
ic_cdk::export_candid!();
