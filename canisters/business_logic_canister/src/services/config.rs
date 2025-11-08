use std::cell::RefCell;
use candid::Principal;

// Default fraud detection limits (can be overridden)
const DEFAULT_MAX_TRANSACTION_AMOUNT: u64 = 10_000_000; // 10M in smallest unit
const DEFAULT_SUSPICIOUS_AMOUNT_THRESHOLD: u64 = 5_000_000; // 5M in smallest unit

thread_local! {
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static MAX_TRANSACTION_AMOUNT: RefCell<u64> = RefCell::new(DEFAULT_MAX_TRANSACTION_AMOUNT);
    static SUSPICIOUS_AMOUNT_THRESHOLD: RefCell<u64> = RefCell::new(DEFAULT_SUSPICIOUS_AMOUNT_THRESHOLD);
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
        id.borrow().ok_or("Data canister ID not set".to_string())
    })
}

// Fraud detection configuration
pub fn set_max_transaction_amount(amount: u64) {
    MAX_TRANSACTION_AMOUNT.with(|max| {
        *max.borrow_mut() = amount;
    });
}

pub fn get_max_transaction_amount() -> u64 {
    MAX_TRANSACTION_AMOUNT.with(|max| *max.borrow())
}

pub fn set_suspicious_amount_threshold(amount: u64) {
    SUSPICIOUS_AMOUNT_THRESHOLD.with(|threshold| {
        *threshold.borrow_mut() = amount;
    });
}

pub fn get_suspicious_amount_threshold() -> u64 {
    SUSPICIOUS_AMOUNT_THRESHOLD.with(|threshold| *threshold.borrow())
}
