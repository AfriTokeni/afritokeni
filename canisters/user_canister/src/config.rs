use candid::Principal;
use std::cell::RefCell;

// ============================================================================
// Configuration State
// ============================================================================

thread_local! {
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static TEST_MODE: RefCell<bool> = RefCell::new(false);
}

// ============================================================================
// Configuration Functions
// ============================================================================

/// Set data canister ID (admin only)
#[ic_cdk_macros::update]
pub fn set_data_canister_id(principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can set canister IDs".to_string());
    }
    
    DATA_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
    
    ic_cdk::println!("✅ Data canister ID set: {}", principal);
    
    Ok(())
}

/// Add authorized canister (admin only)
#[ic_cdk_macros::update]
pub fn add_authorized_canister(principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can authorize canisters".to_string());
    }
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut canisters = canisters.borrow_mut();
        if !canisters.contains(&principal) {
            canisters.push(principal);
        }
    });
    
    ic_cdk::println!("✅ Authorized canister added: {}", principal);
    
    Ok(())
}

/// Enable test mode (admin only)
#[ic_cdk_macros::update]
pub fn enable_test_mode() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can enable test mode".to_string());
    }
    
    TEST_MODE.with(|mode| {
        *mode.borrow_mut() = true;
    });
    
    ic_cdk::println!("✅ Test mode enabled");
    
    Ok(())
}

/// Get data canister ID
pub fn get_data_canister_id() -> Result<Principal, String> {
    DATA_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Data canister ID not configured".to_string())
    })
}

/// Verify caller is authorized
pub fn verify_authorized_caller() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    
    // Controllers always authorized
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    }
    
    // Check if test mode is enabled
    let test_mode = TEST_MODE.with(|mode| *mode.borrow());
    if test_mode {
        return Ok(());
    }
    
    // Check if caller is in authorized list
    let has_authorized = AUTHORIZED_CANISTERS.with(|c| !c.borrow().is_empty());
    
    if !has_authorized {
        // No authorized canisters configured yet, allow all
        return Ok(());
    }
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        if canisters.borrow().contains(&caller) {
            Ok(())
        } else {
            Err(format!("Unauthorized caller: {}", caller))
        }
    })
}
