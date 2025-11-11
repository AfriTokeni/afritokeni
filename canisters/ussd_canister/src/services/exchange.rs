// Exchange Canister API client - for querying configuration
use candid::Principal;
use ic_cdk::call::Call;
use std::cell::RefCell;

thread_local! {
    static EXCHANGE_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

pub fn set_exchange_canister_id(canister_id: Principal) {
    EXCHANGE_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(canister_id);
    });
}

pub fn get_exchange_canister_id() -> Result<Principal, String> {
    EXCHANGE_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Exchange Canister ID not set".to_string())
    })
}

/// Get spread basis points from exchange canister (e.g., 50 = 0.5%)
pub async fn get_spread_basis_points() -> Result<u64, String> {
    let canister_id = get_exchange_canister_id()?;
    
    ic_cdk::println!("📤 Calling exchange canister: get_spread_basis_points");
    
    let response = Call::unbounded_wait(canister_id, "get_spread_basis_points")
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (spread,): (u64,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    Ok(spread)
}
