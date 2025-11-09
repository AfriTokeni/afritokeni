// User management functions
use super::{get_business_logic_canister_id, UserBalances};
use ic_cdk::call::Call;

/// Register a new user (USSD - phone number based)
/// Uses shared RegisterUserRequest type - GUARANTEED to match Business Logic!
pub async fn register_user(
    phone_number: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
    pin: &str,
    currency: &str,
) -> Result<String, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling register_user: phone={}", phone_number);
    
    // Use shared type - NO MORE SIGNATURE MISMATCHES!
    let request = shared_types::RegisterUserRequest {
        phone_number: Some(phone_number.to_string()),
        principal_id: None,
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.to_string(),
        preferred_currency: currency.to_string(),
        pin: pin.to_string(),
    };
    
    let response = Call::unbounded_wait(canister_id, "register_user")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<String, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user balances (fiat and crypto)
pub async fn get_balances(phone_number: &str) -> Result<UserBalances, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_balances")
        .with_args(&(phone_number.to_string(),))  // Single String argument
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<UserBalances, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update user language preference
pub async fn update_user_language(phone_number: &str, language: &str) -> Result<(), String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling update_user_language: phone={}, lang={}", phone_number, language);
    
    let response = Call::unbounded_wait(canister_id, "update_user_language")
        .with_args(&(phone_number, language))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get nearby agents
pub async fn get_nearby_agents(phone_number: &str, currency: &str) -> Result<Vec<super::Agent>, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_nearby_agents: phone={}, currency={}", phone_number, currency);
    
    let response = Call::unbounded_wait(canister_id, "get_nearby_agents")
        .with_args(&(phone_number, currency))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<super::Agent>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Check if user exists
pub async fn user_exists(phone_number: &str) -> Result<bool, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "user_exists")
        .with_args(&(phone_number.to_string(),))  // Single String argument
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
