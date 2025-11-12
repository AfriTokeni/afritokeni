use ic_cdk::call::Call;

use crate::config;

/// Verify PIN via user canister
/// user_canister's verify_pin takes (user_identifier: String, pin: String)
pub async fn verify_pin(user_id: &str, pin: &str) -> Result<bool, String> {
    let canister_id = config::get_user_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "verify_pin")
        .with_args(&(user_id.to_string(), pin.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Check if user exists via user canister
pub async fn user_exists(user_identifier: &str) -> Result<bool, String> {
    let canister_id = config::get_user_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "user_exists")
        .with_args(&(user_identifier.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
