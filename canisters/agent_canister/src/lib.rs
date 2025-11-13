// ============================================================================
// Agent Canister - Main Entry Point
// ============================================================================
// Handles all agent operations: deposits, withdrawals, commissions, settlements
// Production-grade: Security, auditability, and maintainability at MAX
// ============================================================================

use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use candid::Principal;
use shared_types::audit;

// Module declarations
mod config;
mod logic;
mod services;
mod endpoints;

// Re-export endpoints for Candid interface
pub use endpoints::*;

// ============================================================================
// Canister Lifecycle
// ============================================================================

#[init]
fn init() {
    config::init_config();
    
    audit::log_success(
        "init",
        None,
        "Agent canister initialized successfully".to_string()
    );
}

#[pre_upgrade]
fn pre_upgrade() {
    audit::log_success(
        "pre_upgrade",
        None,
        "Starting canister upgrade".to_string()
    );
}

#[post_upgrade]
fn post_upgrade() {
    config::init_config();
    
    audit::log_success(
        "post_upgrade",
        None,
        "Canister upgrade completed successfully".to_string()
    );
}

// ============================================================================
// Configuration Management (Controller Only)
// ============================================================================

/// Set data canister ID (controller only)
#[update]
fn set_data_canister_id(principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can set data canister ID".to_string());
    }
    
    config::set_data_canister_id(principal);
    
    audit::log_success(
        "set_data_canister_id",
        None,
        format!("Data canister ID set to: {}", principal)
    );
    
    Ok(())
}

/// Set user canister ID (controller only)
#[update]
fn set_user_canister_id(principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can set user canister ID".to_string());
    }
    
    config::set_user_canister_id(principal);
    
    audit::log_success(
        "set_user_canister_id",
        None,
        format!("User canister ID set to: {}", principal)
    );
    
    Ok(())
}

/// Set wallet canister ID (controller only)
#[update]
fn set_wallet_canister_id(principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can set wallet canister ID".to_string());
    }
    
    config::set_wallet_canister_id(principal);
    
    audit::log_success(
        "set_wallet_canister_id",
        None,
        format!("Wallet canister ID set to: {}", principal)
    );
    
    Ok(())
}

/// Add authorized canister (controller only)
#[update]
fn add_authorized_canister(principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can add authorized canisters".to_string());
    }
    
    config::add_authorized_canister(principal);
    
    audit::log_success(
        "add_authorized_canister",
        None,
        format!("Authorized canister added: {}", principal)
    );
    
    Ok(())
}

/// Enable test mode (controller only - for PocketIC tests)
#[update]
fn enable_test_mode() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can enable test mode".to_string());
    }
    
    config::enable_test_mode();
    
    audit::log_failure(
        "enable_test_mode",
        None,
        "Test mode enabled - authorization checks relaxed".to_string()
    );
    
    Ok(())
}

/// Disable test mode (controller only)
#[update]
fn disable_test_mode() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can disable test mode".to_string());
    }
    
    config::disable_test_mode();
    
    audit::log_success(
        "disable_test_mode",
        None,
        "Test mode disabled - full authorization checks active".to_string()
    );
    
    Ok(())
}

// ============================================================================
// Health Check & Status
// ============================================================================

#[query]
fn health_check() -> String {
    "Agent canister is healthy".to_string()
}

#[query]
fn get_canister_status() -> CanisterStatus {
    let data_canister = config::get_data_canister_id().ok();
    let user_canister = config::get_user_canister_id().ok();
    let wallet_canister = config::get_wallet_canister_id().ok();
    
    CanisterStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
        data_canister_configured: data_canister.is_some(),
        user_canister_configured: user_canister.is_some(),
        wallet_canister_configured: wallet_canister.is_some(),
        test_mode: config::is_test_mode(),
    }
}

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CanisterStatus {
    pub version: String,
    pub data_canister_configured: bool,
    pub user_canister_configured: bool,
    pub wallet_canister_configured: bool,
    pub test_mode: bool,
}

// ============================================================================
// Configuration Queries
// ============================================================================

#[query]
fn get_deposit_limits(currency: String) -> Result<CurrencyLimitsResponse, String> {
    let limits = config::get_limits_for_currency(&currency);
    
    Ok(CurrencyLimitsResponse {
        currency,
        max_deposit: limits.max_deposit,
        min_deposit: limits.min_deposit,
        max_withdrawal: limits.max_withdrawal,
        min_withdrawal: limits.min_withdrawal,
    })
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CurrencyLimitsResponse {
    pub currency: String,
    pub max_deposit: u64,
    pub min_deposit: u64,
    pub max_withdrawal: u64,
    pub min_withdrawal: u64,
}

#[query]
fn get_fee_structure() -> FeeStructureResponse {
    let cfg = config::get_config();
    
    FeeStructureResponse {
        deposit_agent_commission_bp: cfg.fees.deposit.agent_commission_basis_points,
        deposit_platform_operation_fee_bp: cfg.fees.deposit.platform_operation_fee_basis_points,
        deposit_platform_commission_cut_pct: cfg.fees.deposit.platform_commission_cut_percentage,
        withdrawal_agent_commission_bp: cfg.fees.withdrawal.agent_commission_basis_points,
        withdrawal_platform_operation_fee_bp: cfg.fees.withdrawal.platform_operation_fee_basis_points,
        withdrawal_platform_commission_cut_pct: cfg.fees.withdrawal.platform_commission_cut_percentage,
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FeeStructureResponse {
    pub deposit_agent_commission_bp: u64,
    pub deposit_platform_operation_fee_bp: u64,
    pub deposit_platform_commission_cut_pct: u64,
    pub withdrawal_agent_commission_bp: u64,
    pub withdrawal_platform_operation_fee_bp: u64,
    pub withdrawal_platform_commission_cut_pct: u64,
}

// ============================================================================
// Candid Interface Export
// ============================================================================

ic_cdk::export_candid!();
