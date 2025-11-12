use ic_cdk::call::Call;
use shared_types::{
    FiatCurrency, Transaction, Escrow, EscrowStatus, CryptoType,
};

use crate::config;

/// Get fiat balance from data canister
/// data_canister expects (String, FiatCurrency) - NOT a request struct
pub async fn get_fiat_balance(user_id: &str, currency: FiatCurrency) -> Result<u64, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_fiat_balance")
        .with_args(&(user_id.to_string(), currency))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Set fiat balance in data canister
/// data_canister expects (String, String, u64)
pub async fn set_fiat_balance(
    user_id: &str,
    currency: FiatCurrency,
    amount: u64,
) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "set_fiat_balance")
        .with_args(&(user_id.to_string(), currency.code().to_string(), amount))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update crypto balance in data canister
/// data_canister expects (String, i64, i64)
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
/// data_canister has store_escrow(Escrow), not create_escrow
pub async fn store_escrow(escrow: Escrow) -> Result<(), String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_escrow")
        .with_args(&(escrow,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get escrow by code from data canister
pub async fn get_escrow(code: &str) -> Result<Option<Escrow>, String> {
    let canister_id = config::get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_escrow")
        .with_args(&(code.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Option<Escrow>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
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
