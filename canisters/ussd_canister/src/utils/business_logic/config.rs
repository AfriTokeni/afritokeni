// Configuration functions
use super::{get_business_logic_canister_id, FeeConfig};
use ic_cdk::call::Call;

/// Get transfer fee configuration
pub async fn get_transfer_fee(currency: &str) -> Result<FeeConfig, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_transfer_fee: currency={}", currency);
    
    let response = Call::unbounded_wait(canister_id, "get_transfer_fee")
        .with_args(&(currency,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<FeeConfig, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
