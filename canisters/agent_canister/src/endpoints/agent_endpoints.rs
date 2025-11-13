// ============================================================================
// Agent Management Endpoints - Agent Canister
// ============================================================================
// Handles agent balance queries and settlement management
// ============================================================================

use ic_cdk_macros::{update, query};
use shared_types::audit;
use crate::config;
use crate::services::data_client;

use candid::{CandidType, Deserialize};

// ============================================================================
// Response Types
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AgentBalanceResponse {
    pub agent_id: String,
    pub currency: String,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub commission_earned: u64,
    pub commission_paid: u64,
    pub commission_pending: u64,
    pub last_settlement_date: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SettlementResponse {
    pub month: String,
    pub agent_id: String,
    pub currency: String,
    pub total_commission: u64,
    pub paid: bool,
    pub paid_at: Option<u64>,
}

// ============================================================================
// Agent Balance Queries
// ============================================================================

/// Get agent balance for specific currency (update - makes inter-canister calls)
#[update]
async fn get_agent_balance(agent_id: String, currency: String) -> Result<AgentBalanceResponse, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    let balance = data_client::get_agent_balance(&agent_id, &currency).await?
        .ok_or_else(|| format!("No balance found for agent {} in currency {}", agent_id, currency))?;
    
    let commission_pending = balance.commission_earned.saturating_sub(balance.commission_paid);
    
    Ok(AgentBalanceResponse {
        agent_id: balance.agent_id,
        currency: balance.currency,
        total_deposits: balance.total_deposits,
        total_withdrawals: balance.total_withdrawals,
        commission_earned: balance.commission_earned,
        commission_paid: balance.commission_paid,
        commission_pending,
        last_settlement_date: balance.last_settlement_date,
    })
}

/// Get all balances for an agent (across all currencies)
#[query]
pub async fn get_agent_all_balances(agent_id: String) -> Result<Vec<AgentBalanceResponse>, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    let all_balances = data_client::get_all_agent_balances().await?;
    
    let agent_balances: Vec<AgentBalanceResponse> = all_balances
        .into_iter()
        .filter(|b| b.agent_id == agent_id)
        .map(|balance| {
            let commission_pending = balance.commission_earned.saturating_sub(balance.commission_paid);
            AgentBalanceResponse {
                agent_id: balance.agent_id,
                currency: balance.currency,
                total_deposits: balance.total_deposits,
                total_withdrawals: balance.total_withdrawals,
                commission_earned: balance.commission_earned,
                commission_paid: balance.commission_paid,
                commission_pending,
                last_settlement_date: balance.last_settlement_date,
            }
        })
        .collect();
    
    Ok(agent_balances)
}

/// Get all agent balances (admin only)
#[query]
pub async fn get_all_agent_balances() -> Result<Vec<AgentBalanceResponse>, String> {
    // Only controller can access this
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    let all_balances = data_client::get_all_agent_balances().await?;
    
    let responses: Vec<AgentBalanceResponse> = all_balances
        .into_iter()
        .map(|balance| {
            let commission_pending = balance.commission_earned.saturating_sub(balance.commission_paid);
            AgentBalanceResponse {
                agent_id: balance.agent_id,
                currency: balance.currency,
                total_deposits: balance.total_deposits,
                total_withdrawals: balance.total_withdrawals,
                commission_earned: balance.commission_earned,
                commission_paid: balance.commission_paid,
                commission_pending,
                last_settlement_date: balance.last_settlement_date,
            }
        })
        .collect();
    
    Ok(responses)
}

// ============================================================================
// Settlement Management
// ============================================================================

/// Generate monthly settlements for all agents (admin only)
#[update]
pub async fn generate_monthly_settlements(month: String) -> Result<Vec<SettlementResponse>, String> {
    // Only controller can generate settlements
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    audit::log_success("generate_monthly_settlements", None, format!("Generating settlements for month: {}", month));
    
    // Validate month format (YYYY-MM)
    if !month.contains('-') || month.len() != 7 {
        return Err("Invalid month format. Expected: YYYY-MM (e.g., 2024-11)".to_string());
    }
    
    let cfg = config::get_config();
    let min_settlement = cfg.settlement.min_settlement_amount;
    
    // Get all agent balances
    let all_balances = data_client::get_all_agent_balances().await?;
    
    let mut settlements = Vec::new();
    let _now = ic_cdk::api::time();
    
    for balance in all_balances {
        let pending_commission = balance.commission_earned.saturating_sub(balance.commission_paid);
        
        // Only create settlement if pending commission exceeds minimum
        if pending_commission >= min_settlement {
            let settlement = shared_types::MonthlySettlement {
                month: month.clone(),
                agent_principal: balance.agent_id.clone(),
                total_commission: pending_commission,
                paid: false,
                paid_date: None,
            };
            
            // Store settlement
            data_client::store_settlement(settlement.clone()).await?;
            
            settlements.push(SettlementResponse {
                month: month.clone(),
                agent_id: balance.agent_id,
                currency: balance.currency,
                total_commission: pending_commission,
                paid: false,
                paid_at: None,
            });
        }
    }
    
    audit::log_success(
        "generate_monthly_settlements",
        None,
        format!("Generated {} settlements for month {}", settlements.len(), month)
    );
    
    Ok(settlements)
}

/// Mark settlement as paid (admin only)
#[update]
pub async fn mark_settlement_paid(settlement_id: String) -> Result<(), String> {
    // Only controller can mark settlements as paid
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    audit::log_success("mark_settlement_paid", None, format!("Marking settlement as paid: {}", settlement_id));
    
    // Mark settlement as paid in data canister
    let settlement = data_client::mark_settlement_paid(&settlement_id).await?;
    
    // Update agent balance to reflect payment
    let mut agent_balance = data_client::get_agent_balance(&settlement.agent_principal, "UGX").await? // TODO: Get currency from settlement
        .ok_or_else(|| "Agent balance not found".to_string())?;
    
    agent_balance.commission_paid += settlement.total_commission;
    agent_balance.last_settlement_date = Some(ic_cdk::api::time());
    agent_balance.last_updated = ic_cdk::api::time();
    
    data_client::update_agent_balance(agent_balance).await?;
    
    audit::log_success(
        "mark_settlement_paid",
        Some(settlement.agent_principal.clone()),
        format!("Settlement marked as paid: {}, amount: {}", settlement_id, settlement.total_commission)
    );
    
    Ok(())
}

/// Get settlements for a specific month
#[query]
pub async fn get_settlements_for_month(month: String) -> Result<Vec<shared_types::MonthlySettlement>, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    data_client::get_settlements_for_month(&month).await
}

/// Get settlements for a specific agent
#[query]
pub async fn get_agent_settlements(agent_id: String) -> Result<Vec<shared_types::MonthlySettlement>, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    // Get all settlements and filter by agent
    // Note: This is a simplified version. In production, you'd want a dedicated data_canister method
    let all_settlements = data_client::get_settlements_for_month("").await?; // Get all
    
    let agent_settlements: Vec<shared_types::MonthlySettlement> = all_settlements
        .into_iter()
        .filter(|s| s.agent_principal == agent_id)
        .collect();
    
    Ok(agent_settlements)
}

// ============================================================================
// Statistics (Admin Only)
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct PlatformStatistics {
    pub total_agents: usize,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub total_commission_earned: u64,
    pub total_commission_paid: u64,
    pub total_commission_pending: u64,
}

/// Get platform-wide statistics (admin only)
#[query]
pub async fn get_platform_statistics() -> Result<PlatformStatistics, String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    let all_balances = data_client::get_all_agent_balances().await?;
    
    let mut stats = PlatformStatistics {
        total_agents: all_balances.len(),
        total_deposits: 0,
        total_withdrawals: 0,
        total_commission_earned: 0,
        total_commission_paid: 0,
        total_commission_pending: 0,
    };
    
    for balance in all_balances {
        stats.total_deposits += balance.total_deposits;
        stats.total_withdrawals += balance.total_withdrawals;
        stats.total_commission_earned += balance.commission_earned;
        stats.total_commission_paid += balance.commission_paid;
    }
    
    stats.total_commission_pending = stats.total_commission_earned.saturating_sub(stats.total_commission_paid);
    
    Ok(stats)
}
