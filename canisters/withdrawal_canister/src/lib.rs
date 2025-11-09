use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use serde::{Serialize, Deserialize as SerdeDeserialize};
use std::cell::RefCell;
use std::collections::HashMap;

pub mod logic;

const CONFIG_TOML: &str = include_str!("../../revenue_config.toml");

#[derive(SerdeDeserialize, Clone)]
struct RevenueConfig {
    company_wallet: CompanyWalletConfig,
    withdrawal: WithdrawalConfig,
}

#[derive(SerdeDeserialize, Clone)]
struct CompanyWalletConfig {
    principal: String,
}

#[derive(SerdeDeserialize, Clone)]
struct WithdrawalConfig {
    agent_commission_basis_points: u64,
    platform_fee_basis_points: u64,
    #[allow(dead_code)]
    min_withdrawal_ugx: u64,
    #[allow(dead_code)]
    max_withdrawal_ugx: u64,
}

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

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
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

thread_local! {
    static CONFIG: RefCell<Option<RevenueConfig>> = RefCell::new(None);
    static WITHDRAWALS: RefCell<HashMap<u64, WithdrawalTransaction>> = RefCell::new(HashMap::new());
    static AGENT_EARNINGS: RefCell<HashMap<Principal, AgentEarnings>> = RefCell::new(HashMap::new());
    static NEXT_WITHDRAWAL_ID: RefCell<u64> = RefCell::new(1);
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
// WITHDRAWAL FLOW
// ============================================================================

#[update]
fn create_withdrawal_request(request: CreateWithdrawalRequest) -> Result<WithdrawalTransaction, String> {
    let caller = ic_cdk::api::msg_caller();
    
    logic::validate_caller_is_user(caller, request.user_principal)?;
    logic::validate_amount_positive(request.amount_ugx)?;
    
    let withdrawal_id = NEXT_WITHDRAWAL_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() = current + 1;
        current
    });
    
    let withdrawal_code = logic::generate_withdrawal_code(withdrawal_id);
    let config = get_config();
    
    let platform_fee = logic::calculate_platform_fee(
        request.amount_ugx,
        config.withdrawal.platform_fee_basis_points
    )?;
    
    let agent_fee = logic::calculate_agent_fee(
        request.amount_ugx,
        config.withdrawal.agent_commission_basis_points
    )?;
    
    let transaction = logic::create_withdrawal_transaction(
        withdrawal_id,
        request.user_principal,
        request.agent_principal,
        request.amount_ugx,
        platform_fee,
        agent_fee,
        withdrawal_code,
        ic_cdk::api::time(),
    );
    
    WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow_mut().insert(withdrawal_id, transaction.clone());
    });
    
    Ok(transaction)
}

#[update]
fn confirm_withdrawal(request: ConfirmWithdrawalRequest) -> Result<WithdrawalTransaction, String> {
    let caller = ic_cdk::api::msg_caller();
    
    logic::validate_caller_is_agent(caller, request.agent_principal)?;
    
    let withdrawal_id = WITHDRAWALS.with(|withdrawals| {
        withdrawals.borrow()
            .iter()
            .find(|(_, w)| w.withdrawal_code == request.withdrawal_code)
            .map(|(id, _)| *id)
    }).ok_or("Withdrawal code not found".to_string())?;
    
    let transaction = WITHDRAWALS.with(|withdrawals| {
        let mut wds = withdrawals.borrow_mut();
        let withdrawal = wds.get_mut(&withdrawal_id)
            .ok_or("Withdrawal not found".to_string())?;
        
        logic::validate_status_is_pending(&withdrawal.status)?;
        logic::validate_agent_matches(withdrawal.agent_principal, request.agent_principal)?;
        
        *withdrawal = logic::confirm_transaction_status(withdrawal.clone());
        Ok::<WithdrawalTransaction, String>(withdrawal.clone())
    })?;
    
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
    let config = get_config();
    (config.withdrawal.platform_fee_basis_points, config.withdrawal.agent_commission_basis_points)
}

#[query]
fn get_company_wallet_principal() -> Result<Principal, String> {
    get_company_wallet()
}

ic_cdk::export_candid!();
