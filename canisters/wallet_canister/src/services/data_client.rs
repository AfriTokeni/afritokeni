use ic_cdk::call::Call;
use shared_types::{
    FiatCurrency, Transaction, Escrow, EscrowStatus,
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

/// Get daily transaction statistics for fraud detection
/// Returns (transaction_count, total_amount) for the current day
pub async fn get_daily_transaction_stats(
    user_id: &str,
    currency: FiatCurrency,
) -> Result<(usize, u64), String> {
    let canister_id = config::get_data_canister_id()?;

    // Get transactions for today by filtering the user's transaction history
    // We use a large limit to get all transactions, then filter client-side
    let response = Call::unbounded_wait(canister_id, "get_user_transactions")
        .with_args(&(user_id.to_string(), Some(1000), Some(0)))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;

    let (result,): (Result<Vec<Transaction>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;

    let transactions = result?;

    // Get current time and calculate start of day (last 24 hours)
    let current_time = ic_cdk::api::time();
    let one_day_ns: u64 = 86_400_000_000_000; // 24 hours in nanoseconds
    let start_of_day = current_time.saturating_sub(one_day_ns);

    // Filter transactions from the last 24 hours for the same currency
    let (count, total) = transactions.iter()
        .filter(|tx| {
            // Only count completed transfers in the last 24 hours
            tx.created_at >= start_of_day &&
            tx.status == shared_types::TransactionStatus::Completed &&
            matches!(tx.transaction_type, shared_types::TransactionType::TransferFiat) &&
            matches!(&tx.currency_type, shared_types::CurrencyType::Fiat(c) if *c == currency)
        })
        .fold((0usize, 0u64), |(count, total), tx| {
            (count + 1, total.saturating_add(tx.amount))
        });

    Ok((count, total))
}

/// Get hourly transaction count for velocity checking
/// Returns the count of transactions in the last hour
pub async fn get_hourly_transaction_count(
    user_id: &str,
    currency: FiatCurrency,
) -> Result<usize, String> {
    let canister_id = config::get_data_canister_id()?;

    // Get recent transactions
    let response = Call::unbounded_wait(canister_id, "get_user_transactions")
        .with_args(&(user_id.to_string(), Some(100), Some(0)))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;

    let (result,): (Result<Vec<Transaction>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;

    let transactions = result?;

    // Get current time and calculate start of hour
    let current_time = ic_cdk::api::time();
    let one_hour_ns: u64 = 3_600_000_000_000; // 1 hour in nanoseconds
    let start_of_hour = current_time.saturating_sub(one_hour_ns);

    // Count transactions from the last hour for the same currency
    let count = transactions.iter()
        .filter(|tx| {
            tx.created_at >= start_of_hour &&
            tx.status == shared_types::TransactionStatus::Completed &&
            matches!(tx.transaction_type, shared_types::TransactionType::TransferFiat) &&
            matches!(&tx.currency_type, shared_types::CurrencyType::Fiat(c) if *c == currency)
        })
        .count();

    Ok(count)
}
