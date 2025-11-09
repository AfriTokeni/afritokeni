use std::cell::RefCell;
use candid::Principal;

// Config is now loaded in lib.rs
thread_local! {
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

pub fn set_data_canister_id(canister_id: String) {
    let principal = Principal::from_text(&canister_id)
        .expect("Invalid data canister principal");
    
    DATA_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
}

pub fn get_data_canister_id() -> Result<Principal, String> {
    DATA_CANISTER_ID.with(|id| {
        id.borrow().ok_or_else(|| "Data canister ID not set".to_string())
    })
}

// Fraud detection configuration
pub fn get_max_transaction_amount() -> u64 {
    crate::get_config().fraud_detection.max_transaction_amount
}

pub fn get_suspicious_amount_threshold() -> u64 {
    crate::get_config().fraud_detection.suspicious_amount_threshold
}

pub fn get_rate_limit_window_seconds() -> u64 {
    crate::get_config().fraud_detection.rate_limit_window_seconds
}

pub fn get_max_transactions_per_window() -> usize {
    crate::get_config().fraud_detection.max_transactions_per_window
}

// Exchange rates configuration
pub fn get_coingecko_api_url() -> String {
    crate::get_config().exchange_rates.coingecko_api_url
}

pub fn get_exchangerate_api_url() -> String {
    crate::get_config().exchange_rates.exchangerate_api_url
}

// Ledger configuration
pub fn get_ckbtc_ledger_id() -> Principal {
    Principal::from_text(&crate::get_config().ledger.ckbtc_ledger_id)
        .expect("Invalid ckBTC ledger ID in config")
}

pub fn get_ckusdc_ledger_id() -> Principal {
    Principal::from_text(&crate::get_config().ledger.ckusdc_ledger_id)
        .expect("Invalid ckUSDC ledger ID in config")
}
