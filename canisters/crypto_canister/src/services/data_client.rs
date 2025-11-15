use ic_cdk::call::Call;
use shared_types::{
    Transaction, Escrow, EscrowStatus, CryptoBalance,
};

use crate::config;

/// Get crypto balance from data canister
pub async fn get_crypto_balance(user_id: &str) -> Result<(u64, u64), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_crypto_balance")
        .with_args(&(user_id.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<CryptoBalance, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result.map(|balance| (balance.ckbtc, balance.ckusdc))
}

/// Update crypto balance in data canister
pub async fn update_crypto_balance(
    user_id: &str,
    ckbtc_delta: i64,
    ckusdc_delta: i64,
) -> Result<(), String> {
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

/// Store transaction in data canister
pub async fn store_transaction(transaction: &Transaction) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_transaction")
        .with_args(&(transaction.clone(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user transactions from data canister (paginated)
#[allow(dead_code)]
pub async fn get_user_transactions(
    user_id: &str,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<Transaction>, String> {
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

/// Store escrow in data canister
pub async fn create_escrow(escrow: &Escrow) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_escrow")
        .with_args(&(escrow.clone(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get escrow from data canister
pub async fn get_escrow(code: &str) -> Result<Escrow, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_escrow")
        .with_args(&(code.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Option<Escrow>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result?.ok_or_else(|| format!("Escrow not found: {}", code))
}

/// Update escrow status in data canister
pub async fn update_escrow_status(code: &str, status: EscrowStatus) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_escrow_status")
        .with_args(&(code.to_string(), status))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get all active escrows from data canister (for cleanup job)
pub async fn get_active_escrows() -> Result<Vec<Escrow>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_active_escrows")
        .with_args(&())
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<Escrow>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
