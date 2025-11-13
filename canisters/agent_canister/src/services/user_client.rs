// ============================================================================
// User Canister Client - Inter-Canister Calls
// ============================================================================
// Handles all communication with user_canister for authentication
// ============================================================================

use candid::Principal;
use ic_cdk::call::Call;
use crate::config::get_user_canister_id;

// ============================================================================
// User Operations
// ============================================================================

/// Verify user's PIN
pub async fn verify_pin(user_id: &str, pin: &str) -> Result<bool, String> {
    let canister_id = get_user_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "verify_pin")
        .with_args(&(user_id.to_string(), pin.to_string()))
        .await
        .map_err(|e| format!("Failed to call user_canister: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

/// Check if user exists
pub async fn user_exists(user_identifier: &str) -> Result<bool, String> {
    let canister_id = get_user_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "user_exists")
        .with_arg(user_identifier.to_string())
        .await
        .map_err(|e| format!("Failed to call user_canister: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

/// Get user by phone number
pub async fn get_user_by_phone(phone: &str) -> Result<Option<UserInfo>, String> {
    let canister_id = get_user_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user_by_phone_update")
        .with_arg((phone.to_string(),))
        .await
        .map_err(|e| format!("Failed to call user_canister: {:?}", e))?;
    
    let (result,): (Result<UserProfile, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    match result {
        Ok(profile) => Ok(Some(UserInfo {
            user_id: profile.phone_number.clone().unwrap_or_default(),
            phone_number: profile.phone_number,
            principal_id: profile.principal_id,
        })),
        Err(_) => Ok(None),
    }
}

/// Get user by principal
pub async fn get_user_by_principal(principal: &str) -> Result<Option<UserInfo>, String> {
    let canister_id = get_user_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user_by_principal_update")
        .with_arg((principal.to_string(),))
        .await
        .map_err(|e| format!("Failed to call user_canister: {:?}", e))?;
    
    let (result,): (Result<UserProfile, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    match result {
        Ok(profile) => Ok(Some(UserInfo {
            user_id: profile.principal_id.clone().unwrap_or_default(),
            phone_number: profile.phone_number,
            principal_id: profile.principal_id,
        })),
        Err(_) => Ok(None),
    }
}

// ============================================================================
// Helper Types
// ============================================================================

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,
    pub kyc_status: String,
    pub created_at: u64,
    pub last_active: u64,
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub user_id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
}
