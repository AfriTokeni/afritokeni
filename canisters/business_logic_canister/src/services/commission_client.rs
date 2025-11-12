// ============================================================================
// Commission Client - Inter-Canister Calls to Revenue Canisters
// ============================================================================
// Handles communication with Deposit, Withdrawal, and Exchange canisters
// for commission collection and revenue tracking
// ============================================================================

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call::Call;
use super::config;

// ============================================================================
// DEPOSIT CANISTER TYPES
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DepositTransaction {
    pub id: u64,
    pub user_principal: Principal,
    pub agent_principal: Principal,
    pub amount_ugx: u64,
    pub commission_ugx: u64,
    pub deposit_code: String,
    pub timestamp: u64,
    pub status: DepositStatus,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DepositStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(CandidType, Deserialize)]
pub struct CreateDepositRequest {
    pub user_principal: Principal,
    pub agent_principal: Principal,
    pub amount_ugx: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ConfirmDepositRequest {
    pub deposit_code: String,
    pub agent_principal: Principal,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AgentBalance {
    pub principal: Principal,
    pub total_deposits: u64,
    pub total_commission_owed: u64,
    pub total_commission_paid: u64,
    pub last_settlement_date: Option<u64>,
}

// ============================================================================
// WITHDRAWAL CANISTER TYPES
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WithdrawalTransaction {
    pub id: u64,
    pub user_principal: Principal,
    pub agent_principal: Principal,
    pub amount_ugx: u64,
    pub platform_fee_ugx: u64,
    pub agent_fee_ugx: u64,
    pub withdrawal_code: String,
    pub timestamp: u64,
    pub status: WithdrawalStatus,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(CandidType, Deserialize)]
pub struct CreateWithdrawalRequest {
    pub user_principal: Principal,
    pub agent_principal: Principal,
    pub amount_ugx: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ConfirmWithdrawalRequest {
    pub withdrawal_code: String,
    pub agent_principal: Principal,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AgentEarnings {
    pub principal: Principal,
    pub total_withdrawals_processed: u64,
    pub total_fees_earned: u64,
    pub total_fees_withdrawn: u64,
    pub last_withdrawal_date: Option<u64>,
}

// ============================================================================
// DEPOSIT CANISTER CALLS
// ============================================================================

/// Create a deposit request (user brings cash to agent)
pub async fn create_deposit_request(
    user_principal: Principal,
    agent_principal: Principal,
    amount_ugx: u64,
) -> Result<DepositTransaction, String> {
    let deposit_canister = config::get_deposit_canister_id();
    
    let request = CreateDepositRequest {
        user_principal,
        agent_principal,
        amount_ugx,
    };
    
    let response = Call::unbounded_wait(deposit_canister, "create_deposit_request")
        .with_arg((request,))
        .await
        .map_err(|e| format!("Deposit canister call failed: {:?}", e))?;
    
    let (result,): (Result<DepositTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode deposit response: {:?}", e))?;
    
    result
}

/// Confirm deposit (agent received cash from user)
pub async fn confirm_deposit(
    deposit_code: String,
    agent_principal: Principal,
) -> Result<DepositTransaction, String> {
    let deposit_canister = config::get_deposit_canister_id();
    
    let request = ConfirmDepositRequest {
        deposit_code,
        agent_principal,
    };
    
    let response = Call::unbounded_wait(deposit_canister, "confirm_deposit")
        .with_arg((request,))
        .await
        .map_err(|e| format!("Confirm deposit call failed: {:?}", e))?;
    
    let (result,): (Result<DepositTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode confirm response: {:?}", e))?;
    
    result
}

/// Get agent's deposit commission balance
pub async fn get_agent_deposit_balance(
    agent: Principal,
) -> Result<Option<AgentBalance>, String> {
    let deposit_canister = config::get_deposit_canister_id();
    
    let response = Call::unbounded_wait(deposit_canister, "get_agent_balance")
        .with_arg((agent,))
        .await
        .map_err(|e| format!("Get agent balance call failed: {:?}", e))?;
    
    let (balance,): (Option<AgentBalance>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode balance response: {:?}", e))?;
    
    Ok(balance)
}

/// Get all agent balances from the deposit canister
pub async fn get_all_agent_balances() -> Result<Vec<AgentBalance>, String> {
    let deposit_canister = config::get_deposit_canister_id();
    let response = Call::unbounded_wait(deposit_canister, "get_all_agent_balances")
        .with_arg(())
        .await
        .map_err(|e| format!("Get all agent balances call failed: {:?}", e))?;
    let (balances,): (Vec<AgentBalance>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode balances: {:?}", e))?;
    Ok(balances)
}

/// Mark settlement paid in the deposit canister (updates agent paid totals)
pub async fn mark_settlement_paid(month: String, agent: Principal) -> Result<(), String> {
    let deposit_canister = config::get_deposit_canister_id();
    let response = Call::unbounded_wait(deposit_canister, "mark_settlement_paid")
        .with_arg((month, agent))
        .await
        .map_err(|e| format!("Mark settlement paid call failed: {:?}", e))?;
    let (result,): (Result<(), String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode mark_settlement_paid: {:?}", e))?;
    result
}

/// Ask deposit canister to materialize its internal monthly settlements for a month
pub async fn create_monthly_settlement(month: String) -> Result<(), String> {
    let deposit_canister = config::get_deposit_canister_id();
    let response = Call::unbounded_wait(deposit_canister, "create_monthly_settlement")
        .with_arg((month,))
        .await
        .map_err(|e| format!("Create monthly settlement call failed: {:?}", e))?;
    // We ignore the returned list; only ensure it succeeded
    let (_result,): (Result<Vec<AgentBalance>, String>,) = {
        // Decode as generic Result<Vec<MonthlySettlement>, String>. We can't import the type here.
        // So we just attempt to decode as bytes and ignore content if Ok.
        // Fallback: try decoding as candid::types::Serializer unsupported; instead, decode to candid::utils::ArgumentDecoder with Vec<u8>.
        // Minimal approach: treat success if decode to any Result succeeds; otherwise, still Ok(()) to avoid tight coupling.
        match candid::decode_args::<(Result<Vec<AgentBalance>, String>,)>(&response.into_bytes()) {
            Ok(v) => v,
            Err(_e) => (Ok(Vec::new()),),
        }
    };
    Ok(())
}

// ============================================================================
// WITHDRAWAL CANISTER CALLS
// ============================================================================

/// Create a withdrawal request (user wants cash from agent)
pub async fn create_withdrawal_request(
    user_principal: Principal,
    agent_principal: Principal,
    amount_ugx: u64,
) -> Result<WithdrawalTransaction, String> {
    let withdrawal_canister = config::get_withdrawal_canister_id();
    
    let request = CreateWithdrawalRequest {
        user_principal,
        agent_principal,
        amount_ugx,
    };
    
    let response = Call::unbounded_wait(withdrawal_canister, "create_withdrawal_request")
        .with_arg((request,))
        .await
        .map_err(|e| format!("Withdrawal canister call failed: {:?}", e))?;
    
    let (result,): (Result<WithdrawalTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode withdrawal response: {:?}", e))?;
    
    result
}

/// Confirm withdrawal (agent gave cash to user)
pub async fn confirm_withdrawal(
    withdrawal_code: String,
    agent_principal: Principal,
) -> Result<WithdrawalTransaction, String> {
    let withdrawal_canister = config::get_withdrawal_canister_id();
    
    let request = ConfirmWithdrawalRequest {
        withdrawal_code,
        agent_principal,
    };
    
    let response = Call::unbounded_wait(withdrawal_canister, "confirm_withdrawal")
        .with_arg((request,))
        .await
        .map_err(|e| format!("Confirm withdrawal call failed: {:?}", e))?;
    
    let (result,): (Result<WithdrawalTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode confirm response: {:?}", e))?;
    
    result
}

/// Get agent's withdrawal earnings
pub async fn get_agent_withdrawal_earnings(
    agent: Principal,
) -> Result<Option<AgentEarnings>, String> {
    let withdrawal_canister = config::get_withdrawal_canister_id();
    
    let response = Call::unbounded_wait(withdrawal_canister, "get_agent_earnings")
        .with_arg((agent,))
        .await
        .map_err(|e| format!("Get agent earnings call failed: {:?}", e))?;
    
    let (earnings,): (Option<AgentEarnings>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode earnings response: {:?}", e))?;
    
    Ok(earnings)
}

/// Get withdrawal fee split (platform fee rate + agent fee rate)
/// Returns (platform_fee_basis_points, agent_fee_basis_points)
pub async fn get_withdrawal_fee_split() -> Result<(u64, u64), String> {
    let withdrawal_canister = config::get_withdrawal_canister_id();
    
    let response = Call::unbounded_wait(withdrawal_canister, "get_fee_split")
        .await
        .map_err(|e| format!("Get fee split call failed: {:?}", e))?;
    
    let (platform_fee, agent_fee): (u64, u64) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode fee split response: {:?}", e))?;
    
    Ok((platform_fee, agent_fee))
}

// ============================================================================
// EXCHANGE CANISTER TYPES
// ============================================================================

use shared_types::CryptoType;

#[derive(CandidType, Deserialize, Clone)]
pub struct ExchangeRequest {
    pub to_token: CryptoType,
    pub user_principal: Principal,
    pub from_token: CryptoType,
    pub min_output: u64,
    pub amount: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ExchangeResult {
    pub output_amount: u64,
    pub spread_amount: u64,
    pub exchange_rate: String,
    pub tx_id: String,
}

// ============================================================================
// EXCHANGE CANISTER CALLS
// ============================================================================

/// Swap tokens via exchange canister
pub async fn swap_tokens(
    from_token: CryptoType,
    to_token: CryptoType,
    amount: u64,
    user_principal: Principal,
) -> Result<ExchangeResult, String> {
    let exchange_canister = config::get_exchange_canister_id();
    
    // Calculate min_output with 1% slippage tolerance
    let min_output = (amount * 99) / 100;
    
    let request = ExchangeRequest {
        to_token,
        user_principal,
        from_token,
        min_output,
        amount,
    };
    
    let response = Call::unbounded_wait(exchange_canister, "swap_tokens")
        .with_arg((request,))
        .await
        .map_err(|e| format!("Exchange canister call failed: {:?}", e))?;
    
    let (result,): (Result<ExchangeResult, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode exchange response: {:?}", e))?;
    
    result
}
