// ============================================================================
// Data Canister Client - Inter-Canister Calls
// ============================================================================
// Handles all communication with data_canister for agent operations
// All data storage goes through data_canister (no local state)
// ============================================================================

use ic_cdk::call::Call;
use crate::config::get_data_canister_id;

// Import types from shared_types (single source of truth)
pub use shared_types::{
    DepositTransaction,
    WithdrawalTransaction,
    AgentBalance,
    AgentTransactionStatus,
    MonthlySettlement,
    AgentActivity,
};

// ============================================================================
// Deposit Operations
// ============================================================================

pub async fn store_deposit_transaction(
    deposit: DepositTransaction,
) -> Result<DepositTransaction, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_deposit_transaction")
        .with_arg(deposit)
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<DepositTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn get_deposit_by_code(code: &str) -> Result<Option<DepositTransaction>, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_deposit_by_code")
        .with_arg(code.to_string())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<Option<DepositTransaction>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn update_deposit_status(
    code: &str,
    status: AgentTransactionStatus,
) -> Result<DepositTransaction, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_deposit_status")
        .with_args(&(code.to_string(), status))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<DepositTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn get_agent_deposits(agent_id: &str) -> Result<Vec<DepositTransaction>, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_agent_deposits")
        .with_arg(agent_id.to_string())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<Vec<DepositTransaction>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

// ============================================================================
// Withdrawal Operations
// ============================================================================

pub async fn store_withdrawal_transaction(
    withdrawal: WithdrawalTransaction,
) -> Result<WithdrawalTransaction, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_withdrawal_transaction")
        .with_arg(withdrawal)
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<WithdrawalTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn get_withdrawal_by_code(code: &str) -> Result<Option<WithdrawalTransaction>, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_withdrawal_by_code")
        .with_arg(code.to_string())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<Option<WithdrawalTransaction>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn update_withdrawal_status(
    code: &str,
    status: AgentTransactionStatus,
) -> Result<WithdrawalTransaction, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_withdrawal_status")
        .with_args(&(code.to_string(), status))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<WithdrawalTransaction, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn get_agent_withdrawals(agent_id: &str) -> Result<Vec<WithdrawalTransaction>, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_agent_withdrawals")
        .with_arg(agent_id.to_string())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<Vec<WithdrawalTransaction>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

// ============================================================================
// Agent Balance Operations
// ============================================================================

pub async fn get_agent_balance(agent_id: &str, currency: &str) -> Result<Option<AgentBalance>, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_agent_balance")
        .with_args(&(agent_id.to_string(), currency.to_string()))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<Option<AgentBalance>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn update_agent_balance(balance: AgentBalance) -> Result<AgentBalance, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "update_agent_balance")
        .with_arg(balance)
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<AgentBalance, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn get_all_agent_balances() -> Result<Vec<AgentBalance>, String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_all_agent_balances")
        .with_arg(())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<Vec<AgentBalance>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

// ============================================================================
// Settlement Operations
// ============================================================================

pub async fn store_settlement(settlement: MonthlySettlement) -> Result<(), String> {
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "store_settlements")
        .with_args(&(settlement.month.clone(), vec![settlement]))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result
}

pub async fn mark_settlement_paid(settlement_id: &str) -> Result<MonthlySettlement, String> {
    // Parse settlement_id format: "month:agent_id"
    let parts: Vec<&str> = settlement_id.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid settlement ID format. Expected: month:agent_id".to_string());
    }
    
    let month = parts[0].to_string();
    let agent_principal = parts[1].to_string();
    
    let canister_id = get_data_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "mark_settlement_paid_record")
        .with_args(&(month.clone(), agent_principal.clone()))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;
    
    result?;
    
    // Return the settlement (fetch it)
    let settlements = get_settlements_for_month(&month).await?;
    settlements.into_iter()
        .find(|s| s.agent_principal == agent_principal)
        .ok_or_else(|| "Settlement not found after marking as paid".to_string())
}

pub async fn get_settlements_for_month(month: &str) -> Result<Vec<MonthlySettlement>, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "get_settlements_for_month")
        .with_arg(month.to_string())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<Vec<MonthlySettlement>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}

// ============================================================================
// Agent Activity Operations (Fraud Detection)
// ============================================================================

/// Get agent activity for fraud detection
/// Returns None if no activity record exists yet
pub async fn get_agent_activity(agent_id: &str, currency: &str) -> Result<Option<AgentActivity>, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "get_agent_activity")
        .with_args(&(agent_id.to_string(), currency.to_string()))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<Option<AgentActivity>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}

/// Store or update agent activity for fraud detection
pub async fn store_agent_activity(activity: AgentActivity) -> Result<AgentActivity, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "store_agent_activity")
        .with_arg(activity)
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<AgentActivity, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}

// ============================================================================
// Agent Profile Operations
// ============================================================================

/// Create agent profile
pub async fn create_agent_profile(
    request: shared_types::CreateAgentProfileRequest,
) -> Result<shared_types::AgentProfile, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "create_agent_profile")
        .with_arg(request)
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<shared_types::AgentProfile, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}

/// Get agent profile by user_id
pub async fn get_agent_profile(user_id: &str) -> Result<Option<shared_types::AgentProfile>, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "get_agent_profile")
        .with_arg(user_id.to_string())
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<Option<shared_types::AgentProfile>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}

/// Update agent profile
pub async fn update_agent_profile(
    request: shared_types::UpdateAgentProfileRequest,
) -> Result<shared_types::AgentProfile, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "update_agent_profile")
        .with_arg(request)
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<shared_types::AgentProfile, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}

/// Get nearby agent profiles
pub async fn get_nearby_agent_profiles(
    lat: f64,
    lng: f64,
    radius: f64,
    limit: usize,
) -> Result<Vec<shared_types::AgentProfile>, String> {
    let canister_id = get_data_canister_id()?;

    let response = Call::unbounded_wait(canister_id, "get_nearby_agent_profiles")
        .with_args(&(lat, lng, radius, limit))
        .await
        .map_err(|e| format!("Failed to call data_canister: {:?}", e))?;

    let (result,): (Result<Vec<shared_types::AgentProfile>, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    result
}
