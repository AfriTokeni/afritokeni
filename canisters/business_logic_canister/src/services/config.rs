use std::cell::RefCell;
use candid::Principal;
use serde::{Serialize, Deserialize as SerdeDeserialize};

const CONFIG_TOML: &str = include_str!("../../business_logic_config.toml");

#[derive(SerdeDeserialize, Serialize, Clone)]
struct Config {
    fraud_detection: FraudDetectionConfig,
    exchange_rates: ExchangeRatesConfig,
    ledger: LedgerConfig,
}

#[derive(SerdeDeserialize, Serialize, Clone)]
struct FraudDetectionConfig {
    max_transaction_amount: u64,
    suspicious_amount_threshold: u64,
    rate_limit_window_seconds: u64,
    max_transactions_per_window: usize,
}

#[derive(SerdeDeserialize, Serialize, Clone)]
struct ExchangeRatesConfig {
    coingecko_api_url: String,
    exchangerate_api_url: String,
}

#[derive(SerdeDeserialize, Serialize, Clone)]
struct LedgerConfig {
    ckbtc_ledger_id: String,
    ckusdc_ledger_id: String,
}

thread_local! {
    static CONFIG: RefCell<Option<Config>> = RefCell::new(None);
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

fn get_config() -> Config {
    CONFIG.with(|c| {
        if c.borrow().is_none() {
            let config: Config = toml::from_str(CONFIG_TOML)
                .expect("Failed to parse business_logic_config.toml");
            *c.borrow_mut() = Some(config.clone());
            config
        } else {
            c.borrow().clone().unwrap()
        }
    })
}

pub fn set_data_canister_id(canister_id: String) {
    let principal = Principal::from_text(&canister_id)
        .expect("Invalid data canister principal");
    
    DATA_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
}

pub fn get_data_canister_id() -> Principal {
    DATA_CANISTER_ID.with(|id| {
        id.borrow().expect("Data canister ID not set")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_config_toml() {
        let config: Config = toml::from_str(CONFIG_TOML)
            .expect("Failed to parse TOML");
        
        assert_eq!(config.fraud_detection.max_transaction_amount, 10000000);
        assert_eq!(config.fraud_detection.suspicious_amount_threshold, 5000000);
    }
}

// Fraud detection configuration
pub fn get_max_transaction_amount() -> u64 {
    get_config().fraud_detection.max_transaction_amount
}

pub fn get_suspicious_amount_threshold() -> u64 {
    get_config().fraud_detection.suspicious_amount_threshold
}

pub fn get_rate_limit_window_seconds() -> u64 {
    get_config().fraud_detection.rate_limit_window_seconds
}

pub fn get_max_transactions_per_window() -> usize {
    get_config().fraud_detection.max_transactions_per_window
}

// Exchange rates configuration
pub fn get_coingecko_api_url() -> String {
    get_config().exchange_rates.coingecko_api_url
}

pub fn get_exchangerate_api_url() -> String {
    get_config().exchange_rates.exchangerate_api_url
}

// Ledger configuration
pub fn get_ckbtc_ledger_id() -> String {
    get_config().ledger.ckbtc_ledger_id
}

pub fn get_ckusdc_ledger_id() -> String {
    get_config().ledger.ckusdc_ledger_id
}
