use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult;
use super::config;

// ============================================================================
// Data Canister Client - Pure CRUD Operations
// ============================================================================

/// Get user by ID
pub async fn get_user(user_id: &str) -> Result<Option<User>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<Option<User>, String>,)> = ic_cdk::call(
        canister_id,
        "get_user",
        (user_id.to_string(),),
    ).await;
    
    match result {
        Ok((Ok(user),)) => Ok(user),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Get user by phone number
pub async fn get_user_by_phone(phone: &str) -> Result<Option<User>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<Option<User>, String>,)> = ic_cdk::call(
        canister_id,
        "get_user_by_phone",
        (phone.to_string(),),
    ).await;
    
    match result {
        Ok((Ok(user),)) => Ok(user),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Get fiat balance
pub async fn get_fiat_balance(user_id: &str, currency: &str) -> Result<u64, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<u64, String>,)> = ic_cdk::call(
        canister_id,
        "get_fiat_balance",
        (user_id.to_string(), currency.to_string()),
    ).await;
    
    match result {
        Ok((Ok(balance),)) => Ok(balance),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Set fiat balance (pure CRUD - no validation)
pub async fn set_fiat_balance(user_id: &str, currency: &str, amount: u64) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<(), String>,)> = ic_cdk::call(
        canister_id,
        "set_fiat_balance",
        (user_id.to_string(), currency.to_string(), amount),
    ).await;
    
    match result {
        Ok((Ok(()),)) => Ok(()),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Verify PIN
pub async fn verify_pin(user_id: &str, pin: &str) -> Result<bool, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<bool, String>,)> = ic_cdk::call(
        canister_id,
        "verify_user_pin",
        (user_id.to_string(), pin.to_string()),
    ).await;
    
    match result {
        Ok((Ok(verified),)) => Ok(verified),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Store transaction record
pub async fn store_transaction(tx: &TransactionRecord) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<(), String>,)> = ic_cdk::call(
        canister_id,
        "store_transaction",
        (tx.clone(),),
    ).await;
    
    match result {
        Ok((Ok(()),)) => Ok(()),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Get crypto balance
pub async fn get_crypto_balance(user_id: &str) -> Result<(u64, u64), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<CryptoBalance, String>,)> = ic_cdk::call(
        canister_id,
        "get_crypto_balance",
        (user_id.to_string(),),
    ).await;
    
    match result {
        Ok((Ok(balance),)) => Ok((balance.ckbtc, balance.ckusdc)),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

/// Update crypto balance
pub async fn update_crypto_balance(user_id: &str, ckbtc_delta: i64, ckusdc_delta: i64) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let result: CallResult<(Result<(), String>,)> = ic_cdk::call(
        canister_id,
        "update_crypto_balance",
        (user_id.to_string(), ckbtc_delta, ckusdc_delta),
    ).await;
    
    match result {
        Ok((Ok(()),)) => Ok(()),
        Ok((Err(e),)) => Err(e),
        Err((code, msg)) => Err(format!("Call failed: {:?} - {}", code, msg)),
    }
}

// ============================================================================
// Types (matching data canister)
// ============================================================================

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,
}

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
