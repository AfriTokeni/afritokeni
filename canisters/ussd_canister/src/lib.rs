use ic_cdk_macros::{query, update};

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

// Export Candid interface
ic_cdk::export_candid!();
