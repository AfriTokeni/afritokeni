// Business Logic Canister API client - modular organization
pub mod types;
pub mod user;
pub mod transactions;
pub mod crypto;
pub mod dao;
pub mod config;

use candid::Principal;
use std::cell::RefCell;

thread_local! {
    static BUSINESS_LOGIC_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

pub fn set_business_logic_canister_id(canister_id: Principal) {
    BUSINESS_LOGIC_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(canister_id);
    });
}

pub fn get_business_logic_canister_id() -> Result<Principal, String> {
    BUSINESS_LOGIC_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Business Logic Canister ID not set".to_string())
    })
}

// Re-export commonly used types
pub use types::*;
pub use user::*;
pub use transactions::*;
pub use crypto::*;
pub use dao::*;
pub use config::*;
