use ic_cdk_macros::{init, query, update};
use candid::Principal;

// Organized module structure
mod config_loader;
mod api;
pub mod core;
mod flows;
mod services;
pub mod utils;

/// Initialize USSD canister with Business Logic Canister ID
#[init]
fn init(business_logic_canister_id: Option<String>) {
    if let Some(canister_id) = business_logic_canister_id {
        match Principal::from_text(&canister_id) {
            Ok(principal) => {
                services::business_logic::set_business_logic_canister_id(principal);
                ic_cdk::println!("🔧 USSD canister initialized with Business Logic Canister: {}", canister_id);
            }
            Err(e) => {
                ic_cdk::println!("❌ Invalid Business Logic Canister ID: {:?}", e);
            }
        }
    } else {
        ic_cdk::println!("🔧 USSD canister initialized - use set_business_logic_canister_id to configure");
    }
}

/// Set Business Logic Canister ID (for manual configuration)
#[update]
fn set_business_logic_canister_id(canister_id: String) -> Result<(), String> {
    let principal = Principal::from_text(&canister_id)
        .map_err(|e| format!("Invalid principal: {:?}", e))?;
    
    services::business_logic::set_business_logic_canister_id(principal);
    
    ic_cdk::println!("✅ Business Logic Canister ID set to: {}", canister_id);
    Ok(())
}

/// Get Business Logic Canister ID (delegates to business_logic module)
pub fn get_business_logic_canister_id() -> Result<Principal, String> {
    services::business_logic::get_business_logic_canister_id()
}

/// Set Exchange Canister ID (for manual configuration)
#[update]
fn set_exchange_canister_id(canister_id: Principal) {
    services::exchange::set_exchange_canister_id(canister_id);
    ic_cdk::println!("✅ Exchange Canister ID set");
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
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::msg_reply(&bytes);
}

/// HTTP request handler for POST requests (IC HTTP gateway)
/// Handles update calls that modify state
/// Routes:
/// - POST /api/ussd - USSD webhook handler
#[update(manual_reply = true)]
async fn http_request_update(req: api::http::HttpRequest) {
    ic_cdk::println!("📥 http_request_update (update): {} {}", req.method, req.url);
    let response = api::http::route_request_async(req).await;
    let bytes = candid::encode_one(&response).expect("Failed to encode response");
    ic_cdk::api::msg_reply(&bytes);
}

// Export Candid interface
ic_cdk::export_candid!();
