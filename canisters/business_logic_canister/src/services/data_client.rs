use ic_cdk::call::Call;
use super::config;

// ============================================================================
// Data Canister Client - Pure CRUD Operations
// ============================================================================

/// Get user by ID
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

/// Get user by phone number
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

/// Create user - uses request type with proper non-deprecated API
pub async fn create_user(user_data: shared_types::CreateUserData) -> Result<User, String> {
    let canister_id = config::get_data_canister_id()?;
    
    ic_cdk::println!("📤 Calling create_user with request type");
    
    // Create request with string types
    let request = shared_types::CreateUserRequest {
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

/// Get fiat balance
pub async fn get_fiat_balance(user_id: &str, currency: &str) -> Result<u64, String> {
    let canister_id = config::get_data_canister_id()?;
    
    // Convert currency string to enum
    let currency_enum = shared_types::FiatCurrency::from_string(currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let response = Call::unbounded_wait(canister_id, "get_fiat_balance")
        .with_args(&(user_id.to_string(), currency_enum))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Withdraw fiat (reduces balance)
pub async fn withdraw_fiat(user_id: &str, amount: u64, currency: &str, description: Option<String>) -> Result<Transaction, String> {
    let canister_id = config::get_data_canister_id()?;
    
    // Convert currency string to enum
    let currency_enum = shared_types::FiatCurrency::from_string(currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let response = Call::unbounded_wait(canister_id, "withdraw_fiat")
        .with_args(&(user_id.to_string(), amount, currency_enum, description))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Transaction, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Set fiat balance (pure CRUD - no validation)
pub async fn set_fiat_balance(user_id: &str, currency: &str, amount: u64) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    // set_fiat_balance in data_canister takes strings, not enums
    let response = Call::unbounded_wait(canister_id, "set_fiat_balance")
        .with_args(&(user_id.to_string(), currency.to_string(), amount))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Verify PIN
pub async fn verify_pin(user_id: &str, pin: &str) -> Result<bool, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "verify_user_pin")
        .with_args(&(user_id.to_string(), pin.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Store transaction record
pub async fn store_transaction(tx: &Transaction) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_transaction")
        .with_args(&(tx.clone(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get crypto balance
pub async fn get_crypto_balance(user_id: &str) -> Result<(u64, u64), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_crypto_balance")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<CryptoBalance, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    let balance = result?;
    Ok((balance.ckbtc, balance.ckusdc))
}

/// Update crypto balance
pub async fn update_crypto_balance(user_id: &str, ckbtc_delta: i64, ckusdc_delta: i64) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_crypto_balance")
        .with_args(&(user_id.to_string(), ckbtc_delta, ckusdc_delta))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user transactions with pagination
pub async fn get_user_transactions(user_id: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Transaction>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user_transactions")
        .with_args(&(user_id.to_string(), limit, offset))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<Transaction>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Setup PIN for user  
pub async fn setup_pin(user_id: &str, pin: &str, salt: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    ic_cdk::println!("📤 Calling setup_user_pin with request type");
    
    let request = shared_types::SetupPinRequest {
        user_id: user_id.to_string(),
        pin: pin.to_string(),
        salt: salt.to_string(),
    };
    
    let response = Call::unbounded_wait(canister_id, "setup_user_pin")
        .with_args(&(request,))
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

/// Check for account takeover
pub async fn check_account_takeover(user_id: &str) -> Result<bool, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "check_account_takeover")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update last active timestamp
pub async fn update_last_active(user_id: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_last_active")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Change PIN (requires old PIN verification)
pub async fn change_pin(user_id: &str, old_pin: &str, new_pin: &str, new_salt: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "change_pin")
        .with_args(&(user_id.to_string(), old_pin.to_string(), new_pin.to_string(), new_salt.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update user phone number
pub async fn update_user_phone(user_id: &str, phone_number: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_user_phone")
        .with_args(&(user_id.to_string(), phone_number.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

// ============================================================================
// ALL DATA TYPES NOW IN SHARED_TYPES - SINGLE SOURCE OF TRUTH
// ============================================================================

pub use shared_types::{
    User,
    CryptoBalance,
    Transaction,
    TransactionRecord,
};
