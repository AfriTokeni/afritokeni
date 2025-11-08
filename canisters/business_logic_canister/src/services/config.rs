use std::cell::RefCell;
use candid::Principal;

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
        id.borrow().ok_or("Data canister ID not set".to_string())
    })
}
