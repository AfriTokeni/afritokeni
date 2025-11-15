use ic_cdk::call::Call;
use shared_types::{User, CreateUserData, CreateUserRequest, UpdateUserPhoneRequest, UserType};

use crate::config;

/// Get user by ID from data canister
pub async fn get_user(user_id: &str) -> Result<Option<User>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Option<User>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user by phone number from data canister
pub async fn get_user_by_phone(phone: &str) -> Result<Option<User>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user_by_phone")
        .with_args(&(phone.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Option<User>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user by principal from data canister
pub async fn get_user_by_principal(principal: &str) -> Result<Option<User>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user_by_principal")
        .with_args(&(principal.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Option<User>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Create user in data canister
pub async fn create_user(user_data: CreateUserData) -> Result<User, String> {
    let canister_id = config::get_data_canister_id()?;
    
    ic_cdk::println!("📤 Calling create_user with request type");
    
    let request = CreateUserRequest {
        user_type_str: format!("{:?}", user_data.user_type),
        preferred_currency_str: user_data.preferred_currency.to_string(),
        email: user_data.email,
        first_name: user_data.first_name,
        last_name: user_data.last_name,
        principal_id: user_data.principal_id,
        phone_number: user_data.phone_number,
    };
    
    let response = Call::unbounded_wait(canister_id, "create_user")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<User, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Store PIN hash for user in data canister (Argon2 hash includes salt)
pub async fn store_pin_hash(user_id: &str, pin_hash: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    ic_cdk::println!("📤 Calling store_pin_hash");
    
    let response = Call::unbounded_wait(canister_id, "store_pin_hash")
        .with_args(&(user_id.to_string(), pin_hash.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get PIN hash from data canister
pub async fn get_pin_hash(user_id: &str) -> Result<String, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_pin_hash")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<String, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Increment failed PIN attempts
pub async fn increment_failed_attempts(user_id: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "increment_failed_attempts")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update user phone number in data canister
pub async fn update_user_phone(user_id: &str, phone_number: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let request = UpdateUserPhoneRequest {
        user_id: user_id.to_string(),
        phone_number: phone_number.to_string(),
    };
    
    let response = Call::unbounded_wait(canister_id, "update_user_phone")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update user type in data canister
pub async fn update_user_type(user_id: &str, user_type: UserType) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "update_user_type")
        .with_args(&(user_id.to_string(), user_type))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;

    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;

    result
}

/// Check if PIN is locked
pub async fn is_pin_locked(user_id: &str) -> Result<bool, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "is_pin_locked")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get failed PIN attempts
pub async fn get_failed_attempts(user_id: &str) -> Result<u32, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_failed_attempts")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<u32, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get remaining lockout time in seconds
pub async fn get_remaining_lockout_time(user_id: &str) -> Result<u64, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_remaining_lockout_time")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Reset PIN attempts (after successful verification)
pub async fn reset_pin_attempts(user_id: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "reset_pin_attempts")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
