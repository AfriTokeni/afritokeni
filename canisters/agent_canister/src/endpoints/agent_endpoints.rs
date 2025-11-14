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
    pub outstanding_balance: i64,
    pub credit_limit: u64,
    pub last_settlement_date: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SetAgentTierRequest {
    pub agent_id: String,
    pub currency: String,
    pub tier: shared_types::AgentTier,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AgentCreditStatus {
    pub agent_id: String,
    pub currency: String,
    pub tier: shared_types::AgentTier,
    pub credit_limit: u64,
    pub outstanding_balance: i64,
    pub available_credit: u64,
    pub credit_utilization_percent: f64,
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
        outstanding_balance: balance.outstanding_balance,
        credit_limit: balance.credit_limit,
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
                outstanding_balance: balance.outstanding_balance,
                credit_limit: balance.credit_limit,
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
                outstanding_balance: balance.outstanding_balance,
                credit_limit: balance.credit_limit,
                last_settlement_date: balance.last_settlement_date,
            }
        })
        .collect();
    
    Ok(responses)
}

// ============================================================================
// Agent Tier & Credit Management
// ============================================================================

/// Set agent tier and update credit limit (admin only)
#[update]
pub async fn set_agent_tier(request: SetAgentTierRequest) -> Result<AgentCreditStatus, String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    let new_credit_limit = request.tier.default_credit_limit();
    
    // Get or create agent balance
    let mut balance = data_client::get_agent_balance(&request.agent_id, &request.currency).await?
        .unwrap_or_else(|| shared_types::AgentBalance {
            agent_id: request.agent_id.clone(),
            currency: request.currency.clone(),
            total_deposits: 0,
            total_withdrawals: 0,
            commission_earned: 0,
            commission_paid: 0,
            outstanding_balance: 0,
            credit_limit: new_credit_limit,
            last_settlement_date: None,
            last_updated: ic_cdk::api::time(),
        });
    
    balance.credit_limit = new_credit_limit;
    balance.last_updated = ic_cdk::api::time();
    
    data_client::update_agent_balance(balance.clone()).await?;
    
    audit::log_success(
        "set_agent_tier",
        Some(request.agent_id.clone()),
        format!("Tier set to {:?}, credit limit: {} {}", request.tier, new_credit_limit, request.currency)
    );
    
    let available_credit = if balance.outstanding_balance < 0 {
        new_credit_limit.saturating_sub(balance.outstanding_balance.unsigned_abs())
    } else {
        new_credit_limit
    };
    
    let utilization = if new_credit_limit > 0 {
        (balance.outstanding_balance.unsigned_abs() as f64 / new_credit_limit as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(AgentCreditStatus {
        agent_id: request.agent_id,
        currency: request.currency,
        tier: request.tier,
        credit_limit: new_credit_limit,
        outstanding_balance: balance.outstanding_balance,
        available_credit,
        credit_utilization_percent: utilization,
    })
}

/// Get agent credit status
#[update]
pub async fn get_agent_credit_status(agent_id: String, currency: String) -> Result<AgentCreditStatus, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    let balance = data_client::get_agent_balance(&agent_id, &currency).await?
        .ok_or_else(|| format!("No balance found for agent {} in currency {}", agent_id, currency))?;
    
    // Determine tier based on credit limit
    let tier = if balance.credit_limit >= shared_types::AgentTier::Premium.default_credit_limit() {
        shared_types::AgentTier::Premium
    } else if balance.credit_limit >= shared_types::AgentTier::Trusted.default_credit_limit() {
        shared_types::AgentTier::Trusted
    } else {
        shared_types::AgentTier::New
    };
    
    let available_credit = if balance.outstanding_balance < 0 {
        balance.credit_limit.saturating_sub(balance.outstanding_balance.unsigned_abs())
    } else {
        balance.credit_limit
    };
    
    let utilization = if balance.credit_limit > 0 {
        (balance.outstanding_balance.unsigned_abs() as f64 / balance.credit_limit as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(AgentCreditStatus {
        agent_id: balance.agent_id,
        currency: balance.currency,
        tier,
        credit_limit: balance.credit_limit,
        outstanding_balance: balance.outstanding_balance,
        available_credit,
        credit_utilization_percent: utilization,
    })
}

/// Check if agent can process a deposit (has available credit)
#[update]
pub async fn check_agent_credit_available(
    agent_id: String,
    currency: String,
    amount: u64,
) -> Result<bool, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    let balance = data_client::get_agent_balance(&agent_id, &currency).await?
        .ok_or_else(|| format!("No balance found for agent {} in currency {}", agent_id, currency))?;
    
    // Calculate what outstanding balance would be after this deposit
    let new_outstanding = balance.outstanding_balance - (amount as i64);
    
    // Check if new outstanding balance would exceed credit limit
    let would_exceed = new_outstanding.unsigned_abs() > balance.credit_limit;
    
    Ok(!would_exceed)
}

// ============================================================================
// Settlement Management
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WeeklySettlement {
    pub week: String,  // Format: "2025-W46"
    pub agent_id: String,
    pub currency: String,
    pub outstanding_balance: i64,  // Negative = agent owes platform, Positive = platform owes agent
    pub settlement_amount: u64,    // Absolute amount to settle
    pub settlement_direction: String,  // "agent_to_platform" or "platform_to_agent"
    pub paid: bool,
    pub paid_at: Option<u64>,
}

/// Generate weekly settlements for all agents (admin only)
#[update]
pub async fn generate_weekly_settlements(week: String) -> Result<Vec<WeeklySettlement>, String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    // Validate week format (YYYY-Www)
    if !week.contains("-W") || week.len() != 8 {
        return Err("Invalid week format. Expected: YYYY-Www (e.g., 2025-W46)".to_string());
    }
    
    audit::log_success("generate_weekly_settlements", None, format!("Generating settlements for week: {}", week));
    
    let all_balances = data_client::get_all_agent_balances().await?;
    let mut settlements = Vec::new();
    
    for balance in all_balances {
        // Only create settlement if there's an outstanding balance
        if balance.outstanding_balance != 0 {
            let (settlement_amount, settlement_direction) = if balance.outstanding_balance < 0 {
                (balance.outstanding_balance.unsigned_abs(), "agent_to_platform".to_string())
            } else {
                (balance.outstanding_balance as u64, "platform_to_agent".to_string())
            };
            
            let settlement = WeeklySettlement {
                week: week.clone(),
                agent_id: balance.agent_id.clone(),
                currency: balance.currency.clone(),
                outstanding_balance: balance.outstanding_balance,
                settlement_amount,
                settlement_direction: settlement_direction.clone(),
                paid: false,
                paid_at: None,
            };
            
            settlements.push(settlement);
            
            audit::log_success(
                "weekly_settlement_created",
                Some(balance.agent_id),
                format!("Settlement: {} {} {}, amount: {}", 
                    week, balance.currency, settlement_direction, settlement_amount)
            );
        }
    }
    
    audit::log_success(
        "generate_weekly_settlements",
        None,
        format!("Generated {} settlements for week {}", settlements.len(), week)
    );
    
    Ok(settlements)
}

/// Process weekly settlement payment (admin only)
#[update]
pub async fn process_weekly_settlement(
    agent_id: String,
    currency: String,
    week: String,
) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    let mut balance = data_client::get_agent_balance(&agent_id, &currency).await?
        .ok_or_else(|| format!("Agent balance not found: {} {}", agent_id, currency))?;
    
    let settlement_amount = balance.outstanding_balance;
    
    // Reset outstanding balance to 0
    balance.outstanding_balance = 0;
    balance.last_settlement_date = Some(ic_cdk::api::time());
    balance.last_updated = ic_cdk::api::time();
    
    data_client::update_agent_balance(balance).await?;
    
    audit::log_success(
        "process_weekly_settlement",
        Some(agent_id.clone()),
        format!("Settlement processed: week={}, currency={}, amount={}", week, currency, settlement_amount)
    );
    
    Ok(())
}

/// Generate monthly settlements for all agents (admin only)
/// DEPRECATED: Use generate_weekly_settlements instead
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
