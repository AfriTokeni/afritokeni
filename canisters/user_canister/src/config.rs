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

/// Access levels for authorization
#[derive(Debug, PartialEq, Eq)]
pub enum AccessLevel {
    Controller,
    AuthorizedCanister,
    UserSelf(String), // Contains the user_id or identifier
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

/// Verify caller is authorized to access user data
/// Supports 3-tier access control: Controller, AuthorizedCanister, UserSelf
///
/// # Arguments
/// * `user_identifier` - Optional user identifier (phone, principal, or user_id)
///
/// # Returns
/// * `Ok(AccessLevel)` - The access level granted
/// * `Err` - If caller is not authorized
///
/// # Authorization Levels
/// 1. Controller - Full access to all user data
/// 2. AuthorizedCanister - Access to all user data (USSD, web canisters)
/// 3. UserSelf - Users can only access their own data
pub async fn verify_user_access(user_identifier: Option<&str>) -> Result<AccessLevel, String> {
    let caller = ic_cdk::api::msg_caller();

    // Level 1: Controllers have full access
    if ic_cdk::api::is_controller(&caller) {
        return Ok(AccessLevel::Controller);
    }

    // Check if test mode is enabled
    let test_mode = TEST_MODE.with(|mode| *mode.borrow());
    if test_mode {
        return Ok(AccessLevel::Controller);
    }

    // Level 2: Authorized canisters have full access
    let is_authorized = AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow().contains(&caller)
    });

    if is_authorized {
        return Ok(AccessLevel::AuthorizedCanister);
    }

    // Level 3: Check if user is accessing their own data
    if let Some(identifier) = user_identifier {
        let caller_str = caller.to_string();

        // Check if identifier matches caller principal
        if identifier == caller_str {
            return Ok(AccessLevel::UserSelf(identifier.to_string()));
        }

        // Check if user with this phone has this principal
        // This requires calling data_canister, so we'll use the data_client
        use crate::services::data_client;

        if let Ok(Some(user)) = data_client::get_user_by_phone(identifier).await {
            if let Some(ref principal) = user.principal_id {
                if principal == &caller_str {
                    return Ok(AccessLevel::UserSelf(user.id.clone()));
                }
            }
        }

        // Check by principal
        if let Ok(Some(user)) = data_client::get_user_by_principal(&caller_str).await {
            if identifier == user.id
                || Some(identifier.to_string()) == user.phone_number
                || Some(&caller_str) == user.principal_id.as_ref() {
                return Ok(AccessLevel::UserSelf(user.id.clone()));
            }
        }
    }

    Err("Unauthorized access".to_string())
}
