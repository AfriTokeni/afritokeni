use crate::http_handlers::{error_response, ok_response, HttpRequest, HttpResponse};
use candid::{CandidType, Principal};
use ic_cdk::api::call::ManualReply;
use ic_cdk::call;
use serde::{Deserialize, Serialize};
use std::str;

// Omnia Network's ic-http-proxy canister (mainnet)
// This proxy handles non-replicated HTTPS outcalls to avoid duplicate requests
const HTTP_PROXY_CANISTER: &str = "7hcrm-4iaaa-aaaak-akuka-cai";

/// SMS request from frontend
#[derive(Deserialize)]
struct SmsRequest {
    to: Vec<String>,
    message: String,
    #[serde(rename = "verificationCode")]
    verification_code: Option<String>,
    #[serde(rename = "userId")]
    user_id: Option<String>,
}

/// Africa's Talking SMS response
#[derive(Deserialize)]
struct AtSmsResponse {
    #[serde(rename = "SMSMessageData")]
    sms_message_data: SmsMessageData,
}

#[derive(Deserialize)]
struct SmsMessageData {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Recipients")]
    recipients: Vec<Recipient>,
}

#[derive(Deserialize)]
struct Recipient {
    #[serde(rename = "statusCode")]
    status_code: i32,
    #[serde(rename = "number")]
    number: String,
    #[serde(rename = "messageId")]
    message_id: String,
}

/// SMS response to frontend
#[derive(Serialize)]
struct SmsResponse {
    success: bool,
    message_id: Option<String>,
    error: Option<String>,
}

/// ic-http-proxy request structure
#[derive(CandidType, Serialize)]
struct ProxyHttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

/// ic-http-proxy response structure
#[derive(CandidType, Deserialize)]
struct ProxyHttpResponse {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

/// Handle SMS webhook/request
pub fn handle_sms_webhook(req: HttpRequest) -> ManualReply<HttpResponse> {
    // Parse JSON body
    let sms_req: SmsRequest = match serde_json::from_slice(&req.body) {
        Ok(r) => r,
        Err(e) => return error_response(400, &format!("Invalid JSON: {}", e)),
    };
    
    // Validate request
    if sms_req.to.is_empty() {
        return error_response(400, "No recipients provided");
    }
    
    if sms_req.message.is_empty() {
        return error_response(400, "Message cannot be empty");
    }
    
    ic_cdk::println!(
        "📱 SMS Request - To: {:?}, Message: '{}', HasVerificationCode: {}",
        sms_req.to,
        sms_req.message,
        sms_req.verification_code.is_some()
    );
    
    // If verification code provided, store it
    if let Some(code) = &sms_req.verification_code {
        let phone = sms_req.to[0].clone(); // First recipient
        let code_clone = code.clone();
        let user_id = sms_req.user_id.clone().unwrap_or_else(|| "anonymous".to_string());
        
        // Spawn async task to store verification code
        ic_cdk::futures::spawn(async move {
            if let Err(e) = crate::verification::store_verification_code(&phone, &code_clone, &user_id).await {
                ic_cdk::println!("❌ Failed to store verification code: {}", e);
            }
        });
    }
    
    // Spawn async task to send SMS
    let to = sms_req.to.clone();
    let message = sms_req.message.clone();
    
    ic_cdk::spawn(async move {
        match send_sms_via_api(to, message).await {
            Ok(message_id) => {
                ic_cdk::println!("✅ SMS sent successfully: {}", message_id);
            }
            Err(e) => {
                ic_cdk::println!("❌ Failed to send SMS: {}", e);
            }
        }
    });
    
    // Return immediate success (actual sending happens async)
    let response = SmsResponse {
        success: true,
        message_id: Some(format!("msg_{}", ic_cdk::api::time())),
        error: None,
    };
    
    let response_json = serde_json::to_vec(&response).unwrap();
    ok_response(response_json, "application/json")
}

/// Send SMS via Africa's Talking API using ic-http-proxy
/// 
/// Uses Omnia Network's ic-http-proxy for non-replicated HTTPS outcalls
/// This avoids duplicate SMS sends and reduces costs by 100x
/// Cost: ~20_000_000 cycles per request (vs 2_000_000_000 for replicated)
pub async fn send_sms_via_api(to: Vec<String>, message: String) -> Result<String, String> {
    // Get Africa's Talking credentials from Juno datastore
    // Store config in "config" collection with key "afritalking":
    // {
    //   "at_username": "your_username",
    //   "at_api_key": "your_api_key",
    //   "playground_mode": false  // set to true for testing
    // }
    let (username, api_key, api_url, is_sandbox) = crate::config::get_at_credentials_async().await?;
    
    if is_sandbox {
        ic_cdk::println!("🎮 SANDBOX MODE: SMS will not be delivered to real phones");
    }
    
    // Prepare form data
    let form_data = format!(
        "username={}&to={}&message={}",
        urlencoding::encode(&username),
        urlencoding::encode(&to.join(",")),
        urlencoding::encode(&message)
    );
    
    // Prepare HTTP request for proxy
    let proxy_request = ProxyHttpRequest {
        url: api_url,
        method: "POST".to_string(),
        headers: vec![
            ("apiKey".to_string(), api_key.to_string()),
            ("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string()),
            ("Accept".to_string(), "application/json".to_string()),
        ],
        body: Some(form_data.as_bytes().to_vec()),
    };
    
    // Call ic-http-proxy canister
    let proxy_principal = Principal::from_text(HTTP_PROXY_CANISTER)
        .map_err(|e| format!("Invalid proxy principal: {}", e))?;
    
    let (response,): (ProxyHttpResponse,) = call(proxy_principal, "http_request", (proxy_request,))
        .await
        .map_err(|(code, msg)| format!("Proxy call failed: {:?} - {}", code, msg))?;
    
    // Check response status
    if response.status != 200 && response.status != 201 {
        return Err(format!("API returned status: {}", response.status));
    }
    
    // Parse response
    let body_str = String::from_utf8(response.body)
        .map_err(|e| format!("Invalid UTF-8 response: {}", e))?;
    
    let at_response: AtSmsResponse = serde_json::from_str(&body_str)
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    // Get first message ID
    let message_id = at_response
        .sms_message_data
        .recipients
        .first()
        .map(|r| r.message_id.clone())
        .unwrap_or_else(|| "unknown".to_string());
    
    Ok(message_id)
}
