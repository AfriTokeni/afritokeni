// Transaction functions (fiat)
use super::{get_business_logic_canister_id, TransactionResult, Transaction};
use ic_cdk::call::Call;

/// Send money (fiat transfer)
pub async fn send_money(
    from_phone: &str,
    to_phone: &str,
    amount_cents: u64,
    currency: &str,
    pin: &str,
) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling send_money: from={}, to={}, amount={} cents", from_phone, to_phone, amount_cents);
    
    let response = Call::unbounded_wait(canister_id, "send_money")
        .with_args(&(from_phone, to_phone, amount_cents, currency, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Withdraw fiat
pub async fn withdraw_fiat(phone_number: &str, amount_cents: u64, currency: &str, pin: &str) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling withdraw_fiat: phone={}, amount={} cents", phone_number, amount_cents);
    
    let response = Call::unbounded_wait(canister_id, "withdraw_fiat")
        .with_args(&(phone_number, amount_cents, currency, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get transaction history
pub async fn get_transaction_history(phone_number: &str, limit: u32) -> Result<Vec<Transaction>, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_transaction_history: phone={}, limit={}", phone_number, limit);
    
    let response = Call::unbounded_wait(canister_id, "get_transaction_history")
        .with_args(&(phone_number, limit))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<Transaction>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
