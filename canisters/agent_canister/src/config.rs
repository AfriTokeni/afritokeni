// ============================================================================
// Configuration Management - Agent Canister
// ============================================================================
// Loads configuration from agent_config.toml and provides type-safe access
// No hardcoded values - everything configurable!
// ============================================================================

use candid::Principal;
use serde::Deserialize as SerdeDeserialize;
use std::cell::RefCell;
use std::collections::HashMap;

const CONFIG_TOML: &str = include_str!("../agent_config.toml");

// ============================================================================
// Configuration Structures
// ============================================================================

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct AgentConfig {
    pub company_wallet: CompanyWalletConfig,
    pub fees: FeesConfig,
    pub limits: LimitsConfig,
    pub settlement: SettlementConfig,
    pub fraud: FraudConfig,
    pub codes: CodesConfig,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct CompanyWalletConfig {
    pub principal: String,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct FeesConfig {
    pub deposit: DepositFeesConfig,
    pub withdrawal: WithdrawalFeesConfig,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct DepositFeesConfig {
    pub agent_commission_basis_points: u64,
    pub platform_operation_fee_basis_points: u64,
    pub platform_commission_cut_percentage: u64,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct WithdrawalFeesConfig {
    pub agent_commission_basis_points: u64,
    pub platform_operation_fee_basis_points: u64,
    pub platform_commission_cut_percentage: u64,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct LimitsConfig {
    #[serde(rename = "KES")]
    pub kes: Option<CurrencyLimits>,
    #[serde(rename = "UGX")]
    pub ugx: Option<CurrencyLimits>,
    #[serde(rename = "TZS")]
    pub tzs: Option<CurrencyLimits>,
    #[serde(rename = "NGN")]
    pub ngn: Option<CurrencyLimits>,
    #[serde(rename = "ZAR")]
    pub zar: Option<CurrencyLimits>,
    #[serde(rename = "GHS")]
    pub ghs: Option<CurrencyLimits>,
    pub default: CurrencyLimits,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct CurrencyLimits {
    pub max_deposit: u64,
    pub max_withdrawal: u64,
    pub min_deposit: u64,
    pub min_withdrawal: u64,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct SettlementConfig {
    pub settlement_day_of_month: u8,
    pub min_settlement_amount: u64,
    pub auto_settlement_enabled: bool,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct FraudConfig {
    pub max_deposits_per_agent_per_day: u64,
    pub max_withdrawals_per_agent_per_day: u64,
    pub max_deposit_volume_per_day: u64,
    pub max_withdrawal_volume_per_day: u64,
    pub velocity_check_window_1h: u64,
    pub velocity_check_window_24h: u64,
    pub max_operations_per_hour: u64,
    pub max_operations_per_day: u64,
    pub suspicious_same_user_agent_threshold: u64,
    pub suspicious_rapid_transactions_threshold: u64,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct CodesConfig {
    pub deposit_code_prefix: String,
    pub withdrawal_code_prefix: String,
    pub code_expiration_ns: u64,
}

// ============================================================================
// State Management
// ============================================================================

thread_local! {
    static CONFIG: RefCell<Option<AgentConfig>> = RefCell::new(None);
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static USER_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static WALLET_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static TEST_MODE: RefCell<bool> = RefCell::new(false);
}

// ============================================================================
// Initialization
// ============================================================================

pub fn init_config() {
    let config: AgentConfig = toml::from_str(CONFIG_TOML)
        .expect("Failed to parse agent_config.toml");
    
    CONFIG.with(|c| *c.borrow_mut() = Some(config));
}

pub fn get_config() -> AgentConfig {
    CONFIG.with(|c| {
        c.borrow()
            .clone()
            .expect("Config not initialized. Call init_config() first.")
    })
}

// ============================================================================
// Canister ID Management
// ============================================================================

pub fn set_data_canister_id(canister_id: Principal) {
    DATA_CANISTER_ID.with(|id| *id.borrow_mut() = Some(canister_id));
}

pub fn get_data_canister_id() -> Result<Principal, String> {
    DATA_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Data canister ID not set".to_string())
    })
}

pub fn set_user_canister_id(canister_id: Principal) {
    USER_CANISTER_ID.with(|id| *id.borrow_mut() = Some(canister_id));
}

pub fn get_user_canister_id() -> Result<Principal, String> {
    USER_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "User canister ID not set".to_string())
    })
}

pub fn set_wallet_canister_id(canister_id: Principal) {
    WALLET_CANISTER_ID.with(|id| *id.borrow_mut() = Some(canister_id));
}

pub fn get_wallet_canister_id() -> Result<Principal, String> {
    WALLET_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Wallet canister ID not set".to_string())
    })
}

// ============================================================================
// Authorization Management
// ============================================================================

pub fn add_authorized_canister(canister_id: Principal) {
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut c = canisters.borrow_mut();
        if !c.contains(&canister_id) {
            c.push(canister_id);
        }
    });
}

pub fn is_authorized() -> bool {
    let caller = ic_cdk::api::msg_caller();
    
    // Controller is always authorized
    if ic_cdk::api::is_controller(&caller) {
        return true;
    }
    
    // Check if test mode is enabled
    if TEST_MODE.with(|tm| *tm.borrow()) {
        return true;
    }
    
    // Check if caller is in authorized list
    AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow().contains(&caller)
    })
}

pub fn enable_test_mode() {
    TEST_MODE.with(|tm| *tm.borrow_mut() = true);
}

pub fn disable_test_mode() {
    TEST_MODE.with(|tm| *tm.borrow_mut() = false);
}

pub fn is_test_mode() -> bool {
    TEST_MODE.with(|tm| *tm.borrow())
}

// ============================================================================
// Helper Functions
// ============================================================================

pub fn get_company_wallet() -> Result<Principal, String> {
    let config = get_config();
    Principal::from_text(&config.company_wallet.principal)
        .map_err(|e| format!("Invalid company wallet principal: {}", e))
}

pub fn get_limits_for_currency(currency: &str) -> CurrencyLimits {
    let config = get_config();
    
    match currency {
        "KES" => config.limits.kes.unwrap_or(config.limits.default.clone()),
        "UGX" => config.limits.ugx.unwrap_or(config.limits.default.clone()),
        "TZS" => config.limits.tzs.unwrap_or(config.limits.default.clone()),
        "NGN" => config.limits.ngn.unwrap_or(config.limits.default.clone()),
        "ZAR" => config.limits.zar.unwrap_or(config.limits.default.clone()),
        "GHS" => config.limits.ghs.unwrap_or(config.limits.default.clone()),
        _ => config.limits.default.clone(),
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads() {
        init_config();
        let config = get_config();
        
        assert_eq!(config.fees.deposit.agent_commission_basis_points, 1000);
        assert_eq!(config.fees.deposit.platform_operation_fee_basis_points, 50);
        assert_eq!(config.fees.deposit.platform_commission_cut_percentage, 10);
    }

    #[test]
    fn test_get_limits_for_currency() {
        init_config();
        
        let kes_limits = get_limits_for_currency("KES");
        assert_eq!(kes_limits.max_deposit, 1000000);
        
        let ugx_limits = get_limits_for_currency("UGX");
        assert_eq!(ugx_limits.max_deposit, 10000000);
        
        let unknown_limits = get_limits_for_currency("XXX");
        assert_eq!(unknown_limits.max_deposit, 1000000); // default
    }

    #[test]
    fn test_company_wallet_valid() {
        init_config();
        let wallet = get_company_wallet();
        assert!(wallet.is_ok());
    }
}
