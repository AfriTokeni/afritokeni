use candid::Principal;
use std::cell::RefCell;

// ============================================================================
// Configuration State
// ============================================================================

thread_local! {
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = const { RefCell::new(None) };
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = const { RefCell::new(Vec::new()) };
    static TEST_MODE: RefCell<bool> = const { RefCell::new(false) };
}

// ============================================================================
// Configuration Functions
// ============================================================================

/// Set data canister ID (controller only)
///
/// # Arguments
/// * `principal` - Principal ID of the data_canister
///
/// # Security
/// Only callable by canister controller
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

/// Add authorized canister to whitelist (controller only)
///
/// # Arguments
/// * `principal` - Principal ID of canister to authorize (e.g., ussd_canister, web_canister)
///
/// # Security
/// Only callable by canister controller. Authorized canisters can call user management endpoints.
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

/// Enable test mode (controller only)
///
/// # Security
/// Only callable by canister controller. Test mode allows all callers (bypasses authorization).
/// WARNING: Do not enable in production.
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

/// Get configured data canister ID
///
/// # Returns
/// * `Ok(Principal)` if data canister is configured
/// * `Err` if not yet configured
pub fn get_data_canister_id() -> Result<Principal, String> {
    DATA_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Data canister ID not configured".to_string())
    })
}

/// Get authorized canisters list (for upgrade persistence)
pub fn get_authorized_canisters() -> Vec<Principal> {
    AUTHORIZED_CANISTERS.with(|c| c.borrow().clone())
}

/// Get test mode status (for upgrade persistence)
pub fn get_test_mode() -> bool {
    TEST_MODE.with(|mode| *mode.borrow())
}

/// Restore data canister ID from stable memory (used during post_upgrade)
pub fn restore_data_canister_id(principal: Principal) {
    DATA_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
}

/// Restore authorized canister to list (used during post_upgrade)
pub fn restore_authorized_canister(principal: Principal) {
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut canisters = canisters.borrow_mut();
        if !canisters.contains(&principal) {
            canisters.push(principal);
        }
    });
}

/// Restore test mode status (used during post_upgrade)
pub fn restore_test_mode(enabled: bool) {
    TEST_MODE.with(|mode| {
        *mode.borrow_mut() = enabled;
    });
}

/// Verify caller is authorized to call user management functions
///
/// # Authorization Levels
/// 1. Controller - Always authorized (canister owner)
/// 2. Test mode - All callers authorized (development only)
/// 3. Authorized canisters - Whitelisted canisters (USSD, web, etc.)
///
/// # Returns
/// * `Ok(())` if caller is authorized
/// * `Err` if caller is unauthorized
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
    AUTHORIZED_CANISTERS.with(|canisters| {
        if canisters.borrow().contains(&caller) {
            Ok(())
        } else {
            Err(format!("Unauthorized caller: {}", caller))
        }
    })
}

