use ic_cdk::call::Call;
use shared_types::FiatCurrency;

use crate::config;

/// Get fiat balance via wallet canister
pub async fn get_fiat_balance(user_id: &str, currency: FiatCurrency) -> Result<u64, String> {
    let canister_id = config::get_wallet_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_fiat_balance")
        .with_args(&(user_id.to_string(), currency))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Set fiat balance via wallet canister  
pub async fn set_fiat_balance(
    user_id: &str,
    currency: FiatCurrency,
    amount: u64,
) -> Result<(), String> {
    let canister_id = config::get_wallet_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "set_fiat_balance")
        .with_args(&(user_id.to_string(), currency, amount))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
