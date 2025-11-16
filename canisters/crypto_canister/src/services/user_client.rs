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

/// Resolve user identifier (phone/principal/user_id) to user_id
/// This ensures we always use user_id for data canister operations
pub async fn resolve_user_id(user_identifier: &str) -> Result<String, String> {
    use candid::{CandidType, Deserialize};

    #[derive(CandidType, Deserialize)]
    struct UserProfile {
        pub id: String,
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

    let canister_id = config::get_user_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "get_user_profile_update")
        .with_args(&(user_identifier.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;

    let (result,): (Result<UserProfile, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;

    result.map(|profile| profile.id)
}
