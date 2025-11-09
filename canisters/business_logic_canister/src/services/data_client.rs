use candid::{CandidType, Deserialize};
use ic_cdk::call::Call;
use super::config;

// ============================================================================
// Data Canister Client - Pure CRUD Operations
// ============================================================================

/// Get user by ID
pub async fn get_user(user_id: &str) -> Result<Option<User>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user")
        .with_arg((user_id.to_string(),))
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
        .with_arg((phone.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Option<User>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Create user
pub async fn create_user(user_data: shared_types::CreateUserData) -> Result<User, String> {
    let canister_id = config::get_data_canister_id()?;
    
    ic_cdk::println!("📤 Calling create_user with CreateUserData:");
    ic_cdk::println!("  user_type: {:?}", user_data.user_type);
    ic_cdk::println!("  preferred_currency: {:?}", user_data.preferred_currency);
    ic_cdk::println!("  email: {:?}", user_data.email);
    ic_cdk::println!("  first_name: {:?}", user_data.first_name);
    ic_cdk::println!("  last_name: {:?}", user_data.last_name);
    ic_cdk::println!("  principal_id: {:?}", user_data.principal_id);
    ic_cdk::println!("  phone_number: {:?}", user_data.phone_number);
    
    let response = Call::unbounded_wait(canister_id, "create_user")
        .with_arg((user_data,))
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
    
    let response = Call::unbounded_wait(canister_id, "get_fiat_balance")
        .with_arg((user_id.to_string(), currency.to_string()))
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
    
    let response = Call::unbounded_wait(canister_id, "withdraw_fiat")
        .with_arg((user_id.to_string(), amount, currency.to_string(), description))
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
    
    let response = Call::unbounded_wait(canister_id, "set_fiat_balance")
        .with_arg((user_id.to_string(), currency.to_string(), amount))
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
        .with_arg((user_id.to_string(), pin.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Store transaction record
pub async fn store_transaction(tx: &TransactionRecord) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_transaction")
        .with_arg((tx.clone(),))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id, ckbtc_delta, ckusdc_delta))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user transactions with pagination
pub async fn get_user_transactions(user_id: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<crate::models::TransactionRecord>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_user_transactions")
        .with_arg((user_id, limit, offset))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<crate::models::TransactionRecord>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Setup PIN for user  
pub async fn setup_pin(user_id: &str, pin: &str, salt: &str) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    ic_cdk::println!("📤 Calling setup_user_pin with: user_id={}, pin={}, salt={}", user_id, pin, salt);
    
    // Use with_args() for multiple CandidType values (requires reference to tuple)
    let response = Call::unbounded_wait(canister_id, "setup_user_pin")
        .with_args(&(user_id, pin, salt))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id.to_string(),))
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
        .with_arg((user_id.to_string(), old_pin.to_string(), new_pin.to_string(), new_salt.to_string()))
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
        .with_arg((user_id.to_string(), phone_number.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

// ============================================================================
// Data Types
// ============================================================================

pub use shared_types::User;

#[derive(CandidType, Deserialize, Clone)]
pub struct CryptoBalance {
    pub ckbtc: u64,
    pub ckusdc: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransactionRecord {
    pub id: String,
    pub transaction_type: String,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub amount: u64,
    pub currency: String,
    pub timestamp: u64,
    pub status: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: String,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub amount: u64,
    pub currency: String,
    pub timestamp: u64,
    pub status: String,
    pub description: Option<String>,
}
