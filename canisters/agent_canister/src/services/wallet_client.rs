// ============================================================================
// Wallet Canister Client - Inter-Canister Calls
// ============================================================================
// Handles all communication with wallet_canister for balance operations
// ============================================================================

use ic_cdk::call::Call;
use crate::config::get_wallet_canister_id;
use shared_types::FiatCurrency;

// ============================================================================
// Balance Operations
// ============================================================================

/// Get user's fiat balance for a specific currency
pub async fn get_fiat_balance(user_id: &str, currency: &str) -> Result<u64, String> {
    let canister_id = get_wallet_canister_id()?;
    
    let currency_enum = FiatCurrency::from_code(currency)
        .ok_or_else(|| format!("Invalid currency code: {}", currency))?;
    
    let response = Call::unbounded_wait(canister_id, "get_fiat_balance")
        .with_args(&(user_id.to_string(), currency_enum))
        .await
        .map_err(|e| format!("Failed to call wallet_canister: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

/// Add to user's fiat balance (for deposits)
pub async fn add_fiat_balance(
    user_id: &str,
    amount: u64,
    currency: &str,
) -> Result<u64, String> {
    let canister_id = get_wallet_canister_id()?;
    
    let currency_enum = FiatCurrency::from_code(currency)
        .ok_or_else(|| format!("Invalid currency code: {}", currency))?;
    
    let response = Call::unbounded_wait(canister_id, "add_fiat_balance")
        .with_args(&(user_id.to_string(), amount, currency_enum))
        .await
        .map_err(|e| format!("Failed to call wallet_canister: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

/// Deduct from user's fiat balance (for withdrawals)
pub async fn deduct_fiat_balance(
    user_id: &str,
    amount: u64,
    currency: &str,
) -> Result<u64, String> {
    let canister_id = get_wallet_canister_id()?;
    
    let currency_enum = FiatCurrency::from_code(currency)
        .ok_or_else(|| format!("Invalid currency code: {}", currency))?;
    
    let response = Call::unbounded_wait(canister_id, "deduct_fiat_balance")
        .with_args(&(user_id.to_string(), amount, currency_enum))
        .await
        .map_err(|e| format!("Failed to call wallet_canister: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}
