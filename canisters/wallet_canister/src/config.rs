use candid::Principal;
use ic_cdk::api::msg_caller;
use serde::Deserialize as SerdeDeserialize;
use std::cell::RefCell;
use std::collections::HashMap;

const CONFIG_TOML: &str = include_str!("../wallet_config.toml");

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct WalletConfig {
    pub fees: FeesConfig,
    pub escrow: EscrowConfig,
    pub fraud_limits: FraudLimitsConfig,
    pub external_apis: ExternalApisConfig,
    pub canisters: CanistersConfig,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct FeesConfig {
    pub transfer_fee_basis_points: u64,
    pub exchange_fee_basis_points: u64,
    pub withdrawal_fee_basis_points: u64,
    pub agent_commission_percentage: u64,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct EscrowConfig {
    pub expiration_time_ns: u64,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct FraudLimitsConfig {
    pub default: CurrencyFraudLimits,
    #[serde(flatten)]
    pub currencies: HashMap<String, CurrencyFraudLimits>,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct CurrencyFraudLimits {
    pub max_transaction_amount: u64,
    pub suspicious_threshold: u64,
    pub max_daily_transactions: Option<usize>,
    pub max_daily_amount: Option<u64>,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct ExternalApisConfig {
    pub coingecko_api_url: String,
    pub exchangerate_api_url: String,
}

#[derive(SerdeDeserialize, Clone, Debug)]
pub struct CanistersConfig {
    pub data_canister_id: String,
    pub user_canister_id: String,
}

thread_local! {
    static CONFIG: RefCell<Option<WalletConfig>> = RefCell::new(None);
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static USER_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static TEST_MODE: RefCell<bool> = RefCell::new(false);
}

/// Initialize configuration from TOML
pub fn init_config() {
    CONFIG.with(|c| {
        if c.borrow().is_none() {
            let config: WalletConfig = toml::from_str(CONFIG_TOML)
                .expect("Failed to parse wallet_config.toml");
            *c.borrow_mut() = Some(config);
        }
    });
}

/// Get configuration
pub fn get_config() -> WalletConfig {
    CONFIG.with(|c| {
        if c.borrow().is_none() {
            init_config();
        }
        c.borrow().clone().expect("Config not initialized")
    })
}

/// Get fraud limits for a specific currency
pub fn get_fraud_limits(currency_code: &str) -> CurrencyFraudLimits {
    let config = get_config();
    config.fraud_limits.currencies
        .get(currency_code)
        .cloned()
        .unwrap_or_else(|| config.fraud_limits.default.clone())
}

/// Get transfer fee in basis points
pub fn get_transfer_fee_bps() -> u64 {
    get_config().fees.transfer_fee_basis_points
}

/// Get exchange fee in basis points
#[allow(dead_code)] // TODO: Use when implementing crypto exchange
pub fn get_exchange_fee_bps() -> u64 {
    get_config().fees.exchange_fee_basis_points
}

/// Get withdrawal fee in basis points
#[allow(dead_code)] // TODO: Use when implementing withdrawals
pub fn get_withdrawal_fee_bps() -> u64 {
    get_config().fees.withdrawal_fee_basis_points
}

/// Get agent commission percentage
#[allow(dead_code)] // TODO: Use when implementing agent withdrawals
pub fn get_agent_commission_percentage() -> u64 {
    get_config().fees.agent_commission_percentage
}

/// Get escrow expiration time in nanoseconds
pub fn get_escrow_expiration_time_ns() -> u64 {
    get_config().escrow.expiration_time_ns
}

/// Get CoinGecko API URL
#[allow(dead_code)] // TODO: Use when implementing crypto exchange
pub fn get_coingecko_api_url() -> String {
    get_config().external_apis.coingecko_api_url
}

/// Get ExchangeRate-API URL
#[allow(dead_code)] // TODO: Use when implementing fiat exchange
pub fn get_exchangerate_api_url() -> String {
    get_config().external_apis.exchangerate_api_url
}

/// Set data canister ID
pub fn set_data_canister_id(principal: Principal) {
    DATA_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
}

/// Get data canister ID
pub fn get_data_canister_id() -> Result<Principal, String> {
    DATA_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Data canister ID not set".to_string())
    })
}

/// Set user canister ID
pub fn set_user_canister_id(principal: Principal) {
    USER_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
}

/// Get user canister ID
pub fn get_user_canister_id() -> Result<Principal, String> {
    USER_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "User canister ID not set".to_string())
    })
}

/// Add authorized canister
pub fn add_authorized_canister(principal: Principal) {
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut c = canisters.borrow_mut();
        if !c.contains(&principal) {
            c.push(principal);
        }
    });
}

/// Remove authorized canister
pub fn remove_authorized_canister(principal: Principal) {
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut c = canisters.borrow_mut();
        c.retain(|p| *p != principal);
    });
}

/// Verify caller is authorized
pub fn verify_authorized_caller() -> Result<(), String> {
    let caller_principal = msg_caller();
    
    // Allow controller
    if ic_cdk::api::is_controller(&caller_principal) {
        return Ok(());
    }
    
    // Check if test mode is enabled
    let test_mode = TEST_MODE.with(|mode| *mode.borrow());
    if test_mode {
        return Ok(());
    }
    
    // For testing: Allow anonymous if no authorized canisters are set
    let has_authorized = AUTHORIZED_CANISTERS.with(|canisters| {
        !canisters.borrow().is_empty()
    });
    
    if !has_authorized && caller_principal == Principal::anonymous() {
        return Ok(());
    }
    
    // Check if caller is in authorized list
    AUTHORIZED_CANISTERS.with(|canisters| {
        if canisters.borrow().contains(&caller_principal) {
            Ok(())
        } else {
            Err(format!(
                "Unauthorized caller: {}. Only authorized canisters can call this endpoint.",
                caller_principal
            ))
        }
    })
}

/// Enable test mode
pub fn enable_test_mode() {
    TEST_MODE.with(|mode| {
        *mode.borrow_mut() = true;
    });
}

/// Disable test mode
pub fn disable_test_mode() {
    TEST_MODE.with(|mode| {
        *mode.borrow_mut() = false;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        init_config();
        let config = get_config();
        
        assert_eq!(config.fees.transfer_fee_basis_points, 50);
        assert_eq!(config.fees.exchange_fee_basis_points, 50);
        assert_eq!(config.fees.withdrawal_fee_basis_points, 50);
        assert_eq!(config.fees.agent_commission_percentage, 10);
    }

    #[test]
    fn test_fraud_limits_default() {
        init_config();
        let limits = get_fraud_limits("UNKNOWN");
        
        assert_eq!(limits.max_transaction_amount, 10000000);
        assert_eq!(limits.suspicious_threshold, 5000000);
    }

    #[test]
    fn test_fraud_limits_kes() {
        init_config();
        let limits = get_fraud_limits("KES");
        
        assert_eq!(limits.max_transaction_amount, 15000000);
        assert_eq!(limits.suspicious_threshold, 7500000);
    }

    #[test]
    fn test_fraud_limits_ugx() {
        init_config();
        let limits = get_fraud_limits("UGX");
        
        assert_eq!(limits.max_transaction_amount, 370000000);
        assert_eq!(limits.suspicious_threshold, 185000000);
    }

    #[test]
    fn test_fraud_limits_ngn() {
        init_config();
        let limits = get_fraud_limits("NGN");
        
        assert_eq!(limits.max_transaction_amount, 150000000);
        assert_eq!(limits.suspicious_threshold, 75000000);
    }

    #[test]
    fn test_fee_getters() {
        init_config();
        
        assert_eq!(get_transfer_fee_bps(), 50);
        assert_eq!(get_exchange_fee_bps(), 50);
        assert_eq!(get_withdrawal_fee_bps(), 50);
        assert_eq!(get_agent_commission_percentage(), 10);
    }

    #[test]
    fn test_escrow_expiration() {
        init_config();
        let expiration = get_escrow_expiration_time_ns();
        
        // 24 hours in nanoseconds
        assert_eq!(expiration, 86400000000000);
    }

    #[test]
    fn test_api_urls() {
        init_config();
        
        let coingecko = get_coingecko_api_url();
        assert!(coingecko.contains("coingecko"));
        
        let exchange_rate = get_exchangerate_api_url();
        assert!(exchange_rate.contains("exchangerate-api"));
    }
}
