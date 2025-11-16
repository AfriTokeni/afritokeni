/// Configuration management for crypto canister
/// All values loaded from crypto_config.toml

use candid::Principal;
use serde::Deserialize;
use std::cell::RefCell;

const CONFIG_TOML: &str = include_str!("../crypto_config.toml");

#[derive(Deserialize, Clone, Debug)]
pub struct CryptoConfig {
    pub fees: FeesConfig,
    pub exchange: ExchangeConfig,
    pub reserve: ReserveConfig,
    pub tokens: TokensConfig,
    pub escrow: EscrowConfig,
    pub company_wallet: CompanyWalletConfig,
    pub limits: LimitsConfig,
    pub external_apis: ExternalApisConfig,
    pub security: SecurityConfig,
    pub fraud_detection: FraudDetectionConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FeesConfig {
    pub purchase_fee_basis_points: u64,
    pub sale_fee_basis_points: u64,
    pub btc_network_fee_satoshis: u64,
    pub usdc_network_fee_cents: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExchangeConfig {
    pub spread_basis_points: u64,
    pub dex_provider: String,
    pub sonic: SonicConfig,
    /// Alternative DEX configuration - currently using Sonic, IcpSwap available as fallback
    #[allow(dead_code)]
    pub icpswap: IcpSwapConfig,
    pub slippage: SlippageConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SonicConfig {
    pub swap_canister: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IcpSwapConfig {
    /// IcpSwap router canister ID - unused while Sonic is primary DEX
    #[allow(dead_code)]
    pub router_canister: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SlippageConfig {
    pub default_tolerance: u64,
    pub max_tolerance: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ReserveConfig {
    pub target_ckbtc_percent: f64,
    pub target_ckusdc_percent: f64,
    pub rebalance_threshold_percent: f64,
    pub rebalance_slippage_tolerance_bp: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TokensConfig {
    pub ckbtc: TokenConfig,
    pub ckusdc: TokenConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TokenConfig {
    pub ledger: String,
    /// Token decimals - may be needed for UI conversions or precise calculations
    #[allow(dead_code)]
    pub decimals: u8,
    pub min_transfer_amount: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EscrowConfig {
    pub expiration_time_ns: u64,
    pub min_btc_escrow_satoshis: u64,
    pub min_usdc_escrow_cents: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CompanyWalletConfig {
    pub principal: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LimitsConfig {
    pub default: DefaultLimitsConfig,
    pub btc: BtcLimitsConfig,
    pub usdc: UsdcLimitsConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DefaultLimitsConfig {
    pub max_single_purchase_cents: u64,
    pub max_daily_purchase_cents: u64,
    pub max_single_sale_cents: u64,
    pub max_daily_sale_cents: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BtcLimitsConfig {
    pub max_single_transfer_satoshis: u64,
    pub max_daily_transfer_satoshis: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UsdcLimitsConfig {
    pub max_single_transfer_cents: u64,
    pub max_daily_transfer_cents: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExternalApisConfig {
    pub coingecko_api_url: String,
    pub exchangerate_api_url: String,
    pub max_api_calls_per_minute: u64,
    pub api_timeout_seconds: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SecurityConfig {
    pub require_pin_for_transfers: bool,
    pub require_pin_for_purchases: bool,
    pub require_pin_for_sales: bool,
    pub require_pin_for_escrow: bool,
    pub validate_btc_addresses: bool,
    pub validate_eth_addresses: bool,
    pub log_all_transactions: bool,
    pub log_suspicious_activity: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FraudDetectionConfig {
    pub max_buys_per_hour: usize,
    pub max_sells_per_hour: usize,
    pub max_transfers_per_hour: usize,
    pub max_escrows_per_hour: usize,
    pub max_transactions_per_minute: usize,
    pub max_transactions_per_hour: usize,
    pub max_24h_volume_cents: u64,
    pub max_1h_volume_cents: u64,
    pub suspicious_amount_cents: u64,
    pub high_risk_amount_cents: u64,
    pub max_pin_attempts: u32,
    pub initial_backoff_ns: u64,
    pub max_backoff_ns: u64,
}

// Runtime state
thread_local! {
    static CONFIG: RefCell<Option<CryptoConfig>> = RefCell::new(None);
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static USER_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static WALLET_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static CKBTC_LEDGER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static CKUSDC_LEDGER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static TEST_MODE: RefCell<bool> = RefCell::new(false);
}

/// Initialize configuration from TOML file
pub fn init_config() {
    let config: CryptoConfig = toml::from_str(CONFIG_TOML)
        .expect("Failed to parse crypto_config.toml");
    
    CONFIG.with(|c| *c.borrow_mut() = Some(config));
}

/// Get the full configuration
pub fn get_config() -> CryptoConfig {
    CONFIG.with(|c| {
        c.borrow()
            .clone()
            .expect("Config not initialized. Call init_config() first.")
    })
}

// ============================================================================
// Canister ID Management
// ============================================================================

pub fn set_data_canister_id(principal: Principal) {
    DATA_CANISTER_ID.with(|id| *id.borrow_mut() = Some(principal));
}

pub fn get_data_canister_id() -> Result<Principal, String> {
    DATA_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Data canister ID not set".to_string())
    })
}

pub fn set_user_canister_id(principal: Principal) {
    USER_CANISTER_ID.with(|id| *id.borrow_mut() = Some(principal));
}

pub fn get_user_canister_id() -> Result<Principal, String> {
    USER_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "User canister ID not set".to_string())
    })
}

pub fn set_wallet_canister_id(principal: Principal) {
    WALLET_CANISTER_ID.with(|id| *id.borrow_mut() = Some(principal));
}

pub fn get_wallet_canister_id() -> Result<Principal, String> {
    WALLET_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Wallet canister ID not set".to_string())
    })
}

pub fn set_ckbtc_ledger_id(principal: Principal) {
    CKBTC_LEDGER_ID.with(|id| *id.borrow_mut() = Some(principal));
}

pub fn get_ckbtc_ledger_id() -> Option<Principal> {
    CKBTC_LEDGER_ID.with(|id| *id.borrow())
}

pub fn set_ckusdc_ledger_id(principal: Principal) {
    CKUSDC_LEDGER_ID.with(|id| *id.borrow_mut() = Some(principal));
}

pub fn get_ckusdc_ledger_id() -> Option<Principal> {
    CKUSDC_LEDGER_ID.with(|id| *id.borrow())
}

// ============================================================================
// Authorization Management
// ============================================================================

pub fn add_authorized_canister(principal: Principal) {
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut canisters = canisters.borrow_mut();
        if !canisters.contains(&principal) {
            canisters.push(principal);
        }
    });
}

pub fn is_authorized(principal: &Principal) -> bool {
    // In test mode, allow all
    if is_test_mode() {
        return true;
    }
    
    // Check if caller is controller
    if ic_cdk::api::is_controller(principal) {
        return true;
    }
    
    // Check if caller is in authorized list
    AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow().contains(principal)
    })
}

// ============================================================================
// Test Mode Management
// ============================================================================

pub fn enable_test_mode() {
    TEST_MODE.with(|mode| *mode.borrow_mut() = true);
}

pub fn disable_test_mode() {
    TEST_MODE.with(|mode| *mode.borrow_mut() = false);
}

pub fn is_test_mode() -> bool {
    TEST_MODE.with(|mode| *mode.borrow())
}

// ============================================================================
// Security Configuration Helpers
// ============================================================================

/// Check if PIN is required for the given operation
pub fn require_pin_for_operation(operation: &str) -> bool {
    let cfg = get_config();
    match operation {
        "transfer" => cfg.security.require_pin_for_transfers,
        "purchase" => cfg.security.require_pin_for_purchases,
        "sale" => cfg.security.require_pin_for_sales,
        "escrow" => cfg.security.require_pin_for_escrow,
        _ => false,
    }
}

/// Check if address validation is required for the given crypto type
pub fn should_validate_address(crypto_type: &str) -> bool {
    let cfg = get_config();
    match crypto_type {
        "BTC" | "CkBTC" => cfg.security.validate_btc_addresses,
        "ETH" | "USDC" | "CkUSD" => cfg.security.validate_eth_addresses,
        _ => false,
    }
}

/// Check if all transactions should be logged
pub fn should_log_transaction() -> bool {
    get_config().security.log_all_transactions
}

/// Check if suspicious activity should be logged
pub fn should_log_suspicious_activity() -> bool {
    get_config().security.log_suspicious_activity
}

// ============================================================================
// Configuration Getters (convenience methods)
// ============================================================================

/// Used by external canisters (USSD, web) via inter-canister calls
#[allow(dead_code)]
pub fn get_purchase_fee_basis_points() -> u64 {
    get_config().fees.purchase_fee_basis_points
}

/// Used by external canisters (USSD, web) via inter-canister calls
#[allow(dead_code)]
pub fn get_sale_fee_basis_points() -> u64 {
    get_config().fees.sale_fee_basis_points
}

pub fn get_spread_basis_points() -> u64 {
    get_config().exchange.spread_basis_points
}

pub fn get_escrow_expiration_ns() -> u64 {
    get_config().escrow.expiration_time_ns
}

/// Used by external canisters (USSD, web) to retrieve platform wallet for fee collection
#[allow(dead_code)]
pub fn get_company_wallet_principal() -> Result<Principal, String> {
    let config = get_config();
    Principal::from_text(&config.company_wallet.principal)
        .map_err(|e| format!("Invalid company wallet principal: {}", e))
}

pub fn get_dex_provider() -> String {
    get_config().exchange.dex_provider
}

pub fn get_sonic_swap_canister() -> Result<Principal, String> {
    let config = get_config();
    Principal::from_text(&config.exchange.sonic.swap_canister)
        .map_err(|e| format!("Invalid Sonic swap canister: {}", e))
}

pub fn get_ckbtc_ledger() -> Result<Principal, String> {
    let config = get_config();
    Principal::from_text(&config.tokens.ckbtc.ledger)
        .map_err(|e| format!("Invalid ckBTC ledger: {}", e))
}

pub fn get_ckusdc_ledger() -> Result<Principal, String> {
    let config = get_config();
    Principal::from_text(&config.tokens.ckusdc.ledger)
        .map_err(|e| format!("Invalid ckUSDC ledger: {}", e))
}

pub fn get_coingecko_api_url() -> String {
    get_config().external_apis.coingecko_api_url
}

pub fn get_exchangerate_api_url() -> String {
    get_config().external_apis.exchangerate_api_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads() {
        init_config();
        let config = get_config();
        
        assert_eq!(config.fees.purchase_fee_basis_points, 50);
        assert_eq!(config.fees.sale_fee_basis_points, 50);
        assert_eq!(config.exchange.spread_basis_points, 50);
        assert_eq!(config.escrow.expiration_time_ns, 86400000000000);
    }

    #[test]
    fn test_convenience_getters() {
        init_config();
        
        assert_eq!(get_purchase_fee_basis_points(), 50);
        assert_eq!(get_sale_fee_basis_points(), 50);
        assert_eq!(get_spread_basis_points(), 50);
        assert_eq!(get_escrow_expiration_ns(), 86400000000000);
        assert_eq!(get_dex_provider(), "sonic");
    }

    #[test]
    fn test_test_mode() {
        assert!(!is_test_mode());
        enable_test_mode();
        assert!(is_test_mode());
        disable_test_mode();
        assert!(!is_test_mode());
    }
}
