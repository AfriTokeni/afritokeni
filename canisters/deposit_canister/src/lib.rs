use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use serde::{Serialize, Deserialize as SerdeDeserialize};
use std::cell::RefCell;
use std::collections::HashMap;

// Configuration loaded from shared TOML
const CONFIG_TOML: &str = include_str!("../../revenue_config.toml");

#[derive(SerdeDeserialize, Clone)]
struct RevenueConfig {
    company_wallet: CompanyWalletConfig,
    deposit: DepositConfig,
}

#[derive(SerdeDeserialize, Clone)]
struct CompanyWalletConfig {
    principal: String,
}

#[derive(SerdeDeserialize, Clone)]
struct DepositConfig {
    agent_commission_basis_points: u64,
    platform_fee_basis_points: u64,
    min_deposit_ugx: u64,
    max_deposit_ugx: u64,
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DepositTransaction {
    pub id: u64,
    pub user_principal: Principal,
    pub agent_principal: Principal,
    pub amount_ugx: u64,
    pub commission_ugx: u64,
    pub deposit_code: String,
    pub timestamp: u64,
    pub status: TransactionStatus,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AgentBalance {
    pub principal: Principal,
    pub total_deposits: u64,
    pub total_commission_owed: u64,
    pub total_commission_paid: u64,
    pub last_settlement_date: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct MonthlySettlement {
    pub month: String, // "2024-11"
    pub agent_principal: Principal,
    pub total_commission: u64,
    pub paid: bool,
    pub paid_date: Option<u64>,
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

// ============================================================================
// STATE
// ============================================================================

thread_local! {
    static CONFIG: RefCell<Option<RevenueConfig>> = RefCell::new(None);
    static DEPOSITS: RefCell<HashMap<u64, DepositTransaction>> = RefCell::new(HashMap::new());
    static AGENT_BALANCES: RefCell<HashMap<Principal, AgentBalance>> = RefCell::new(HashMap::new());
    static SETTLEMENTS: RefCell<Vec<MonthlySettlement>> = RefCell::new(Vec::new());
    static NEXT_DEPOSIT_ID: RefCell<u64> = RefCell::new(1);
}

// ============================================================================
// INITIALIZATION
// ============================================================================

#[init]
fn init() {
    // Load configuration from shared TOML
    let config: RevenueConfig = toml::from_str(CONFIG_TOML)
        .expect("Failed to parse revenue_config.toml");
    
    CONFIG.with(|c| *c.borrow_mut() = Some(config));
}

fn get_config() -> RevenueConfig {
    CONFIG.with(|c| {
        c.borrow()
            .clone()
            .expect("Config not initialized. Call init() first.")
    })
}

fn get_company_wallet() -> Result<Principal, String> {
    let config = get_config();
    Principal::from_text(&config.company_wallet.principal)
        .map_err(|e| format!("Invalid company wallet principal: {}", e))
}

// ============================================================================
// DEPOSIT FLOW
// ============================================================================

#[update]
fn create_deposit_request(request: CreateDepositRequest) -> Result<DepositTransaction, String> {
    let caller = ic_cdk::api::msg_caller();
    
    // Verify caller is the user
    if caller != request.user_principal {
        return Err("Caller must be the user".to_string());
    }
    
    if request.amount_ugx == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    
    // Generate unique deposit code
    let deposit_id = NEXT_DEPOSIT_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() = current + 1;
        current
    });
    
    let deposit_code = generate_deposit_code(deposit_id);
    
    // Calculate commission from config
    let config = get_config();
    let commission = (request.amount_ugx * config.deposit.platform_fee_basis_points) / 10000;
    
    let transaction = DepositTransaction {
        id: deposit_id,
        user_principal: request.user_principal,
        agent_principal: request.agent_principal,
        amount_ugx: request.amount_ugx,
        commission_ugx: commission,
        deposit_code: deposit_code.clone(),
        timestamp: ic_cdk::api::time(),
        status: TransactionStatus::Pending,
    };
    
    DEPOSITS.with(|deposits| {
        deposits.borrow_mut().insert(deposit_id, transaction.clone());
    });
    
    Ok(transaction)
}

#[update]
fn confirm_deposit(request: ConfirmDepositRequest) -> Result<DepositTransaction, String> {
    let caller = ic_cdk::api::msg_caller();
    
    // Verify caller is the agent
    if caller != request.agent_principal {
        return Err("Only the assigned agent can confirm".to_string());
    }
    
    // Find deposit by code
    let deposit_id = DEPOSITS.with(|deposits| {
        deposits.borrow()
            .iter()
            .find(|(_, d)| d.deposit_code == request.deposit_code)
            .map(|(id, _)| *id)
    }).ok_or("Deposit code not found".to_string())?;
    
    // Update deposit status
    let transaction = DEPOSITS.with(|deposits| {
        let mut deps = deposits.borrow_mut();
        let deposit = deps.get_mut(&deposit_id)
            .ok_or("Deposit not found".to_string())?;
        
        if deposit.status != TransactionStatus::Pending {
            return Err("Deposit already processed".to_string());
        }
        
        if deposit.agent_principal != request.agent_principal {
            return Err("Wrong agent".to_string());
        }
        
        deposit.status = TransactionStatus::Confirmed;
        Ok(deposit.clone())
    })?;
    
    // Update agent balance
    update_agent_balance(
        request.agent_principal,
        transaction.amount_ugx,
        transaction.commission_ugx,
    );
    
    Ok(transaction)
}

// ============================================================================
// AGENT BALANCE MANAGEMENT
// ============================================================================

fn update_agent_balance(agent: Principal, deposit_amount: u64, commission: u64) {
    AGENT_BALANCES.with(|balances| {
        let mut bals = balances.borrow_mut();
        let balance = bals.entry(agent).or_insert(AgentBalance {
            principal: agent,
            total_deposits: 0,
            total_commission_owed: 0,
            total_commission_paid: 0,
            last_settlement_date: None,
        });
        
        balance.total_deposits += deposit_amount;
        balance.total_commission_owed += commission;
    });
}

#[query]
fn get_agent_balance(agent: Principal) -> Option<AgentBalance> {
    AGENT_BALANCES.with(|balances| {
        balances.borrow().get(&agent).cloned()
    })
}

#[query]
fn get_all_agent_balances() -> Vec<AgentBalance> {
    AGENT_BALANCES.with(|balances| {
        balances.borrow().values().cloned().collect()
    })
}

// ============================================================================
// SETTLEMENT MANAGEMENT
// ============================================================================

#[update]
fn create_monthly_settlement(month: String) -> Result<Vec<MonthlySettlement>, String> {
    // Only company wallet can create settlements
    let caller = ic_cdk::api::msg_caller();
    let company = get_company_wallet()?;
    
    if caller != company {
        return Err("Only company wallet can create settlements".to_string());
    }
    
    let mut new_settlements = Vec::new();
    
    AGENT_BALANCES.with(|balances| {
        for (agent, balance) in balances.borrow().iter() {
            if balance.total_commission_owed > balance.total_commission_paid {
                let outstanding = balance.total_commission_owed - balance.total_commission_paid;
                
                let settlement = MonthlySettlement {
                    month: month.clone(),
                    agent_principal: *agent,
                    total_commission: outstanding,
                    paid: false,
                    paid_date: None,
                };
                
                new_settlements.push(settlement);
            }
        }
    });
    
    SETTLEMENTS.with(|settlements| {
        settlements.borrow_mut().extend(new_settlements.clone());
    });
    
    Ok(new_settlements)
}

#[update]
fn mark_settlement_paid(month: String, agent: Principal) -> Result<(), String> {
    // Only company wallet can mark as paid
    let caller = ic_cdk::api::msg_caller();
    let company = get_company_wallet()?;
    
    if caller != company {
        return Err("Only company wallet can mark settlements paid".to_string());
    }
    
    SETTLEMENTS.with(|settlements| {
        let mut setts = settlements.borrow_mut();
        let settlement = setts.iter_mut()
            .find(|s| s.month == month && s.agent_principal == agent)
            .ok_or("Settlement not found".to_string())?;
        
        if settlement.paid {
            return Err("Settlement already paid".to_string());
        }
        
        settlement.paid = true;
        settlement.paid_date = Some(ic_cdk::api::time());
        
        // Update agent balance
        AGENT_BALANCES.with(|balances| {
            let mut bals = balances.borrow_mut();
            if let Some(balance) = bals.get_mut(&agent) {
                balance.total_commission_paid += settlement.total_commission;
                balance.last_settlement_date = Some(ic_cdk::api::time());
            }
        });
        
        Ok(())
    })
}

#[query]
fn get_settlements_for_month(month: String) -> Vec<MonthlySettlement> {
    SETTLEMENTS.with(|settlements| {
        settlements.borrow()
            .iter()
            .filter(|s| s.month == month)
            .cloned()
            .collect()
    })
}

#[query]
fn get_agent_settlements(agent: Principal) -> Vec<MonthlySettlement> {
    SETTLEMENTS.with(|settlements| {
        settlements.borrow()
            .iter()
            .filter(|s| s.agent_principal == agent)
            .cloned()
            .collect()
    })
}

// ============================================================================
// QUERY FUNCTIONS
// ============================================================================

#[query]
fn get_deposit(id: u64) -> Option<DepositTransaction> {
    DEPOSITS.with(|deposits| {
        deposits.borrow().get(&id).cloned()
    })
}

#[query]
fn get_user_deposits(user: Principal) -> Vec<DepositTransaction> {
    DEPOSITS.with(|deposits| {
        deposits.borrow()
            .values()
            .filter(|d| d.user_principal == user)
            .cloned()
            .collect()
    })
}

#[query]
fn get_agent_deposits(agent: Principal) -> Vec<DepositTransaction> {
    DEPOSITS.with(|deposits| {
        deposits.borrow()
            .values()
            .filter(|d| d.agent_principal == agent)
            .cloned()
            .collect()
    })
}

#[query]
fn get_pending_deposits(agent: Principal) -> Vec<DepositTransaction> {
    DEPOSITS.with(|deposits| {
        deposits.borrow()
            .values()
            .filter(|d| d.agent_principal == agent && d.status == TransactionStatus::Pending)
            .cloned()
            .collect()
    })
}

#[query]
fn get_total_revenue() -> u64 {
    AGENT_BALANCES.with(|balances| {
        balances.borrow()
            .values()
            .map(|b| b.total_commission_owed)
            .sum()
    })
}

#[query]
fn get_commission_rate() -> u64 {
    let config = get_config();
    config.deposit.platform_fee_basis_points
}

#[query]
fn get_company_wallet_principal() -> Result<Principal, String> {
    get_company_wallet()
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================
// Helper: Generate deposit code
fn generate_deposit_code(id: u64) -> String {
    format!("DEP-{:08}", id)
}

// Tests module
#[cfg(test)]
mod tests;

// Export Candid interface
ic_cdk::export_candid!();
