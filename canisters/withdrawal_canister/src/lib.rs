use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawalTransaction {
    pub id: u64,
    pub user_principal: Principal,
    pub agent_principal: Principal,
    pub amount_ugx: u64,
    pub platform_fee_ugx: u64,      // 0.5% of amount + 10% of agent fee
    pub agent_fee_ugx: u64,          // Dynamic 2-12% (agent keeps 90%)
    pub withdrawal_code: String,
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
pub struct AgentEarnings {
    pub principal: Principal,
    pub total_withdrawals_processed: u64,
    pub total_fees_earned: u64,
    pub total_fees_withdrawn: u64,
    pub last_withdrawal_date: Option<u64>,
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

// ============================================================================
// STATE
// ============================================================================

// ============================================================================
// CONFIGURABLE CONSTANTS - CHANGE THESE TO UPDATE FEE STRUCTURE
// ============================================================================

/// Platform fee in basis points (0.5% = 50 bps)
/// This is charged on the withdrawal amount
const DEFAULT_PLATFORM_FEE_BPS: u64 = 50;

/// Agent fee in basis points (3% = 300 bps)
/// This is the default agent fee. Can be dynamic 2-12% based on location
const DEFAULT_AGENT_FEE_BPS: u64 = 300;

/// Platform's cut of agent fee as percentage (10%)
/// Platform takes 10% of the agent's earnings, agent keeps 90%
const PLATFORM_CUT_OF_AGENT_FEE_PERCENT: u64 = 10;

thread_local! {
    static WITHDRAWALS: RefCell<HashMap<u64, WithdrawalTransaction>> = RefCell::new(HashMap::new());
    static AGENT_EARNINGS: RefCell<HashMap<Principal, AgentEarnings>> = RefCell::new(HashMap::new());
    static NEXT_WITHDRAWAL_ID: RefCell<u64> = RefCell::new(1);
    static PLATFORM_FEE_BPS: RefCell<u64> = RefCell::new(DEFAULT_PLATFORM_FEE_BPS);
    static AGENT_FEE_BPS: RefCell<u64> = RefCell::new(DEFAULT_AGENT_FEE_BPS);
    static PLATFORM_CUT_OF_AGENT_FEE: RefCell<u64> = RefCell::new(PLATFORM_CUT_OF_AGENT_FEE_PERCENT);
    static COMPANY_WALLET: RefCell<Option<Principal>> = RefCell::new(None);
}

// ============================================================================
// INITIALIZATION
// ============================================================================

#[init]
fn init(company_wallet: Principal) {
    COMPANY_WALLET.with(|w| *w.borrow_mut() = Some(company_wallet));
}

// ============================================================================
// WITHDRAWAL FLOW
// ============================================================================

#[update]
fn create_withdrawal_request(request: CreateWithdrawalRequest) -> Result<WithdrawalTransaction, String> {
    let caller = ic_cdk::api::msg_caller();
    
    // Verify caller is the user
    if caller != request.user_principal {
        return Err("Caller must be the user".to_string());
    }
    
    if request.amount_ugx == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    
    // Generate unique withdrawal code
    let withdrawal_id = NEXT_WITHDRAWAL_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() = current + 1;
        current
    });
    
    let withdrawal_code = generate_withdrawal_code(withdrawal_id);
    
    // Calculate fees according to revenue model:
    // 1. Platform fee: 0.5% of withdrawal amount
    // 2. Agent fee: Dynamic 2-12% based on location (default 3%)
    // 3. Platform takes 10% of agent's fee
    
    let platform_fee_bps = PLATFORM_FEE_BPS.with(|p| *p.borrow());
    let agent_fee_bps = AGENT_FEE_BPS.with(|a| *a.borrow());
    let platform_cut = PLATFORM_CUT_OF_AGENT_FEE.with(|c| *c.borrow());
    
    // Platform fee: 0.5% of amount (50 bps)
    let platform_base_fee = (request.amount_ugx * platform_fee_bps) / 10000;
    
    // Agent fee: 3% of amount (300 bps) - TODO: Make dynamic based on location
    let agent_total_fee = (request.amount_ugx * agent_fee_bps) / 10000;
    
    // Platform gets 10% of agent's fee
    let platform_cut_of_agent = (agent_total_fee * platform_cut) / 100;
    
    // Total platform revenue
    let platform_fee = platform_base_fee + platform_cut_of_agent;
    
    // Agent keeps 90% of their fee
    let agent_fee = agent_total_fee - platform_cut_of_agent;
    
    let transaction = WithdrawalTransaction {
        id: withdrawal_id,
        user_principal: request.user_principal,
        agent_principal: request.agent_principal,
        amount_ugx: request.amount_ugx,
        platform_fee_ugx: platform_fee,
        agent_fee_ugx: agent_fee,
        withdrawal_code: withdrawal_code.clone(),
        timestamp: ic_cdk::api::time(),
        status: TransactionStatus::Pending,
    };
    
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow_mut().insert(withdrawal_id, transaction.clone());
    });
    
    Ok(transaction)
}

#[update]
fn confirm_withdrawal(request: ConfirmWithdrawalRequest) -> Result<WithdrawalTransaction, String> {
    let caller = ic_cdk::api::msg_caller();
    
    // Verify caller is the agent
    if caller != request.agent_principal {
        return Err("Only the assigned agent can confirm".to_string());
    }
    
    // Find withdrawal by code
    let withdrawal_id = WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow()
            .iter()
            .find(|(_, w)| w.withdrawal_code == request.withdrawal_code)
            .map(|(id, _)| *id)
    }).ok_or("Withdrawal code not found".to_string())?;
    
    // Update withdrawal status
    let transaction = WITHDRAWALS.with(|withdrawals| {
        let mut wds = withdrawals.borrow_mut();
        let withdrawal = wds.get_mut(&withdrawal_id)
            .ok_or("Withdrawal not found".to_string())?;
        
        if withdrawal.status != TransactionStatus::Pending {
            return Err("Withdrawal already processed".to_string());
        }
        
        if withdrawal.agent_principal != request.agent_principal {
            return Err("Wrong agent".to_string());
        }
        
        withdrawal.status = TransactionStatus::Confirmed;
        Ok(withdrawal.clone())
    })?;
    
    // Update agent earnings
    update_agent_earnings(
        request.agent_principal,
        transaction.amount_ugx,
        transaction.agent_fee_ugx,
    );
    
    Ok(transaction)
}

// ============================================================================
// AGENT EARNINGS MANAGEMENT
// ============================================================================

fn update_agent_earnings(agent: Principal, withdrawal_amount: u64, agent_fee: u64) {
    AGENT_EARNINGS.with(|earnings| {
        let mut earns = earnings.borrow_mut();
        let earning = earns.entry(agent).or_insert(AgentEarnings {
            principal: agent,
            total_withdrawals_processed: 0,
            total_fees_earned: 0,
            total_fees_withdrawn: 0,
            last_withdrawal_date: None,
        });
        
        earning.total_withdrawals_processed += withdrawal_amount;
        earning.total_fees_earned += agent_fee;
        earning.last_withdrawal_date = Some(ic_cdk::api::time());
    });
}

#[query]
fn get_agent_earnings(agent: Principal) -> Option<AgentEarnings> {
    AGENT_EARNINGS.with(|earnings| {
        earnings.borrow().get(&agent).cloned()
    })
}

#[query]
fn get_all_agent_earnings() -> Vec<AgentEarnings> {
    AGENT_EARNINGS.with(|earnings| {
        earnings.borrow().values().cloned().collect()
    })
}

// ============================================================================
// QUERY FUNCTIONS
// ============================================================================

#[query]
fn get_withdrawal(id: u64) -> Option<WithdrawalTransaction> {
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow().get(&id).cloned()
    })
}

#[query]
fn get_user_withdrawals(user: Principal) -> Vec<WithdrawalTransaction> {
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow()
            .values()
            .filter(|w| w.user_principal == user)
            .cloned()
            .collect()
    })
}

#[query]
fn get_agent_withdrawals(agent: Principal) -> Vec<WithdrawalTransaction> {
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow()
            .values()
            .filter(|w| w.agent_principal == agent)
            .cloned()
            .collect()
    })
}

#[query]
fn get_pending_withdrawals(agent: Principal) -> Vec<WithdrawalTransaction> {
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow()
            .values()
            .filter(|w| w.agent_principal == agent && w.status == TransactionStatus::Pending)
            .cloned()
            .collect()
    })
}

#[query]
fn get_total_platform_revenue() -> u64 {
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow()
            .values()
            .filter(|w| w.status == TransactionStatus::Confirmed)
            .map(|w| w.platform_fee_ugx)
            .sum()
    })
}

#[query]
fn get_total_agent_earnings() -> u64 {
    AGENT_EARNINGS.with(|earnings| {
        earnings.borrow()
            .values()
            .map(|e| e.total_fees_earned)
            .sum()
    })
}

#[query]
fn get_fee_split() -> (u64, u64) {
    let platform_bps = PLATFORM_FEE_BPS.with(|p| *p.borrow());
    let agent_bps = AGENT_FEE_BPS.with(|a| *a.borrow());
    (platform_bps, agent_bps)
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn generate_withdrawal_code(id: u64) -> String {
    format!("WTH-{:08}", id)
}

// Tests module
#[cfg(test)]
mod tests;

// Export Candid interface
ic_cdk::export_candid!();
