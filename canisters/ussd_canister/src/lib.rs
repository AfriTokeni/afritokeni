use ic_cdk::api::call::ManualReply;
use ic_cdk_macros::update;

// Organized module structure
mod handlers;
mod models;
mod utils;

/// HTTP request handler for USSD webhook
/// 
/// Routes:
/// - POST /api/ussd - USSD webhook handler
#[update(manual_reply = true)]
fn http_request_update(req: handlers::http_handlers::HttpRequest) -> ManualReply<handlers::http_handlers::HttpResponse> {
    handlers::http_handlers::route_request(req)
}
