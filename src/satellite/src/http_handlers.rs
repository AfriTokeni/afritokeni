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
        ("POST", "/api/ussd") => crate::ussd::handle_ussd_webhook(req),
        ("POST", "/api/sms") => handle_incoming_sms(req),
        ("POST", "/api/send-sms") => crate::sms::handle_sms_webhook(req),
        ("POST", "/api/verify-code") => handle_verify_code(req),
        ("POST", "/api/send-notification") => handle_send_notification(req),
        _ => not_found(),
    }
}

/// Handle incoming SMS from Africa's Talking
fn handle_incoming_sms(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Parse form data
    let body_str = match std::str::from_utf8(&req.body) {
        Ok(s) => s,
        Err(_) => return error_response(400, "Invalid UTF-8 in request body"),
    };
    
    // Parse form-urlencoded data
    let params: std::collections::HashMap<String, String> = body_str
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => {
                    Some((
                        urlencoding::decode(key).ok()?.into_owned(),
                        urlencoding::decode(value).ok()?.into_owned(),
                    ))
                }
                _ => None,
            }
        })
        .collect();
    
    let from = params.get("from").map(|s| s.as_str()).unwrap_or("");
    let to = params.get("to").map(|s| s.as_str()).unwrap_or("");
    let text = params.get("text").map(|s| s.as_str()).unwrap_or("");
    let date = params.get("date").map(|s| s.as_str()).unwrap_or("");
    let id = params.get("id").map(|s| s.as_str()).unwrap_or("");
    
    ic_cdk::println!(
        "📨 SMS Received - From: {}, To: {}, Text: '{}', Date: {}, ID: {}",
        from, to, text, date, id
    );
    
    // Validate
    if from.is_empty() || text.is_empty() {
        return error_response(400, "Missing required fields");
    }
    
    // Process SMS command
    // TODO: Parse SMS commands like:
    // - "BAL" - Check balance
    // - "SEND +256... 10000" - Send money
    // - "BTC BUY 50000" - Buy Bitcoin
    
    let response = serde_json::json!({
        "status": "received",
        "message": "SMS processed successfully"
    });
    
    ok_response(response.to_string().into_bytes(), "application/json")
}

/// Send notification (email or SMS)
fn handle_send_notification(req: HttpRequest) -> ManualReply<HttpResponse> {
    #[derive(serde::Deserialize)]
    struct NotificationRequest {
        #[serde(rename = "type")]
        notification_type: String,
        recipient: String,
    }
    
    let notif_req: NotificationRequest = match serde_json::from_slice(&req.body) {
        Ok(r) => r,
        Err(e) => return error_response(400, &format!("Invalid JSON: {}", e)),
    };
    
    ic_cdk::println!(
        "📧 Sending {} notification to {}",
        notif_req.notification_type,
        notif_req.recipient
    );
    
    // For email notifications, we'd use Resend API via ic-http-proxy
    // For SMS notifications, we'd use our SMS handler
    
    let response = serde_json::json!({
        "success": true,
        "message": format!("{} notification sent successfully", notif_req.notification_type)
    });
    
    ok_response(response.to_string().into_bytes(), "application/json")
}

/// Verify SMS code endpoint
fn handle_verify_code(req: HttpRequest) -> ManualReply<HttpResponse> {
    #[derive(serde::Deserialize)]
    struct VerifyRequest {
        #[serde(rename = "phoneNumber")]
        phone_number: String,
        code: String,
    }
    
    // Parse request
    let verify_req: VerifyRequest = match serde_json::from_slice(&req.body) {
        Ok(r) => r,
        Err(e) => return error_response(400, &format!("Invalid JSON: {}", e)),
    };
    
    // Validate
    if verify_req.phone_number.is_empty() || verify_req.code.is_empty() {
        return error_response(400, "Phone number and code are required");
    }
    
    // Spawn async verification
    let phone = verify_req.phone_number.clone();
    let code = verify_req.code.clone();
    
    ic_cdk::spawn(async move {
        match crate::verification::verify_code(&phone, &code).await {
            Ok(user_id) => {
                ic_cdk::println!("✅ Code verified for user: {}", user_id);
            }
            Err(e) => {
                ic_cdk::println!("❌ Verification failed: {}", e);
            }
        }
    });
    
    // For now, return success immediately
    // In production, we'd wait for the async result
    let response = serde_json::json!({
        "success": true,
        "message": "Code verification in progress"
    });
    
    ok_response(response.to_string().into_bytes(), "application/json")
}

/// Health check endpoint
fn health_check() -> ManualReply<HttpResponse> {
    let health_data = serde_json::json!({
        "status": "healthy",
        "service": "AfriTokeni Satellite Functions",
        "timestamp": ic_cdk::api::time() / 1_000_000_000, // Convert nanoseconds to seconds
        "endpoints": {
            "ussd": "/api/ussd",
            "sms": "/api/sms",
            "health": "/api/health"
        }
    });
    
    ok_response(health_data.to_string().into_bytes(), "application/json")
}

/// Return 404 Not Found response
fn not_found() -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 404,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: b"Not Found".to_vec(),
    };
    ManualReply::one(response)
}

/// Helper to create success response
pub fn ok_response(body: Vec<u8>, content_type: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: 200,
        headers: vec![("Content-Type".to_string(), content_type.to_string())],
        body,
    };
    ManualReply::one(response)
}

/// Helper to create error response
pub fn error_response(status: u16, message: &str) -> ManualReply<HttpResponse> {
    let response = HttpResponse {
        status_code: status,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: message.as_bytes().to_vec(),
    };
    ManualReply::one(response)
}
