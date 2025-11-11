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

// ============================================================================
// DEPOSIT & WITHDRAWAL COMMISSION (NEW)
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct DepositRequestResult {
    pub deposit_code: String,
    pub amount_ugx: u64,
    pub commission_ugx: u64,
    pub net_amount: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct WithdrawalFeesResult {
    pub amount: u64,
    pub platform_fee: u64,
    pub agent_fee: u64,
    pub total_fees: u64,
    pub net_amount: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct WithdrawalRequestResult {
    pub withdrawal_code: String,
    pub amount_ugx: u64,
    pub platform_fee_ugx: u64,
    pub agent_fee_ugx: u64,
    pub net_amount: u64,
}

/// Create deposit request (user shows code to agent)
pub async fn create_deposit_request(
    user_phone: &str,
    agent_id: &str,
    amount: u64,
) -> Result<DepositRequestResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling create_deposit_request: phone={}, agent={}, amount={}", user_phone, agent_id, amount);
    
    let response = Call::unbounded_wait(canister_id, "create_deposit_request")
        .with_args(&(user_phone, agent_id, amount))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<DepositRequestResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get withdrawal fees estimate (before creating withdrawal)
pub async fn get_withdrawal_fees(amount: u64) -> Result<WithdrawalFeesResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_withdrawal_fees: amount={}", amount);
    
    let response = Call::unbounded_wait(canister_id, "get_withdrawal_fees")
        .with_args(&(amount,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<WithdrawalFeesResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Create withdrawal request (with fees already shown to user)
pub async fn create_withdrawal_request(
    user_phone: &str,
    agent_id: &str,
    amount: u64,
    pin: &str,
) -> Result<WithdrawalRequestResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling create_withdrawal_request: phone={}, agent={}, amount={}", user_phone, agent_id, amount);
    
    let response = Call::unbounded_wait(canister_id, "create_withdrawal_request")
        .with_args(&(user_phone, agent_id, amount, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<WithdrawalRequestResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
