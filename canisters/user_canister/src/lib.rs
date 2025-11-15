use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

mod logic;
mod services;
mod config;
mod security;

use shared_types::{RegisterUserRequest, User, UserType, FiatCurrency, CreateUserData, AuditEntry, audit};

/// User profile response (simplified for API)
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,
    pub kyc_status: String,
    pub created_at: u64,
    pub last_active: u64,
}

/// Profile update request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProfileUpdates {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub preferred_currency: Option<String>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        UserProfile {
            id: user.id,
            phone_number: user.phone_number,
            principal_id: user.principal_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            preferred_currency: user.preferred_currency.to_string(),
            kyc_status: format!("{:?}", user.kyc_status),
            created_at: user.created_at,
            last_active: user.last_active,
        }
    }
}

// ============================================================================
// USER MANAGEMENT ENDPOINTS
// ============================================================================

/// Register new user
#[update]
async fn register_user(request: RegisterUserRequest) -> Result<String, String> {
    config::verify_authorized_caller()?;
    
    ic_cdk::println!("📥 User Canister received register_user:");
    ic_cdk::println!("  phone_number: {:?}", request.phone_number);
    ic_cdk::println!("  principal_id: {:?}", request.principal_id);
    ic_cdk::println!("  first_name: {}", request.first_name);
    ic_cdk::println!("  last_name: {}", request.last_name);
    ic_cdk::println!("  email: {}", request.email);
    ic_cdk::println!("  preferred_currency: {}", request.preferred_currency);
    
    // Validate at least one identifier
    if let Err(e) = logic::user_logic::validate_identifier_required(&request.phone_number, &request.principal_id) {
        audit::log_failure(
            "user_registration_failed",
            None,
            format!("Validation error: {}", e),
        );
        return Err(e);
    }
    
    // Validate PIN format
    logic::user_logic::validate_pin_format(&request.pin)?;
    
    // Validate phone if provided
    if let Some(ref phone) = request.phone_number {
        logic::user_logic::validate_phone_number_format(phone)?;
    }
    
    // Validate email
    logic::user_logic::validate_email_format(&request.email)?;
    
    // Validate names
    logic::user_logic::validate_name(&request.first_name, "First name")?;
    logic::user_logic::validate_name(&request.last_name, "Last name")?;
    
    // Parse currency
    let currency = FiatCurrency::from_string(&request.preferred_currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    // Check if user already exists (use generic error to prevent enumeration)
    if let Some(ref phone) = request.phone_number {
        if services::data_client::get_user_by_phone(phone).await?.is_some() {
            audit::log_failure(
                "user_registration_failed",
                None,
                format!("Duplicate phone number attempt: {}", phone),
            );
            return Err("Registration failed. This identifier may already be in use.".to_string());
        }
    }

    if let Some(ref principal) = request.principal_id {
        if services::data_client::get_user_by_principal(principal).await?.is_some() {
            audit::log_failure(
                "user_registration_failed",
                None,
                format!("Duplicate principal attempt: {}", principal),
            );
            return Err("Registration failed. This identifier may already be in use.".to_string());
        }
    }
    
    // Create user data
    let user_data = CreateUserData {
        user_type: UserType::User,
        preferred_currency: currency,
        email: request.email.clone(),
        first_name: request.first_name.clone(),
        last_name: request.last_name.clone(),
        principal_id: request.principal_id.clone(),
        phone_number: request.phone_number.clone(),
    };
    
    // Create user in data canister
    let user = services::data_client::create_user(user_data).await?;
    
    // Hash PIN using Argon2 (includes salt automatically)
    let pin_hash = security::hash_pin(&request.pin).await
        .map_err(|e| format!("Failed to hash PIN: {}", e))?;
    
    // Store the hash in data canister (no salt needed - it's in the hash)
    services::data_client::store_pin_hash(&user.id, &pin_hash).await?;
    
    ic_cdk::println!("✅ User registered successfully: {}", user.id);

    // Log successful registration
    audit::log_success(
        "user_registered",
        Some(user.id.clone()),
        format!(
            "User: {} {} | Phone: {:?} | Principal: {:?} | Currency: {}",
            request.first_name,
            request.last_name,
            request.phone_number,
            request.principal_id,
            request.preferred_currency
        ),
    );

    Ok(user.id)
}

/// Check if user exists by phone number, principal ID, or user ID
///
/// # Arguments
/// * `user_identifier` - Phone number (+256...), principal ID, or user ID
///
/// # Returns
/// * `Ok(true)` if user exists
/// * `Ok(false)` if user does not exist
/// * `Err` if there was an error checking
#[update]
async fn user_exists(user_identifier: String) -> Result<bool, String> {
    config::verify_authorized_caller()?;

    // Check by phone or principal
    let exists = services::data_client::get_user_by_phone(&user_identifier).await?.is_some()
        || services::data_client::get_user(&user_identifier).await?.is_some();

    Ok(exists)
}

/// Verify user PIN with automatic lockout after 3 failed attempts
///
/// # Arguments
/// * `user_identifier` - Phone number, principal ID, or user ID
/// * `pin` - 4-digit PIN to verify
///
/// # Returns
/// * `Ok(true)` if PIN is correct
/// * `Ok(false)` if PIN is incorrect
/// * `Err` if user not found or account is locked
///
/// # Security
/// - Account locked for 30 minutes after 3 failed attempts
/// - All attempts logged in audit trail
/// - Uses Argon2id for secure verification
#[update]
async fn verify_pin(user_identifier: String, pin: String) -> Result<bool, String> {
    config::verify_authorized_caller()?;

    // Get user by phone, principal, or user ID
    // Use generic error message to prevent user enumeration
    let user = if let Some(u) = services::data_client::get_user_by_phone(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user_by_principal(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user(&user_identifier).await? {
        u
    } else {
        audit::log_failure(
            "pin_verification_failed",
            None,
            format!("Invalid credentials for identifier: {}", user_identifier),
        );
        return Err("Invalid credentials".to_string());
    };
    
    // Check if PIN is locked
    if services::data_client::is_pin_locked(&user.id).await? {
        let remaining_seconds = services::data_client::get_remaining_lockout_time(&user.id).await?;
        let attempts = services::data_client::get_failed_attempts(&user.id).await?;
        let remaining_minutes = remaining_seconds.div_ceil(60);
        
        return Err(format!(
            "PIN locked due to {} failed attempts. Try again in {} minutes",
            attempts, remaining_minutes
        ));
    }
    
    // Get stored PIN hash from data canister
    let pin_hash = services::data_client::get_pin_hash(&user.id).await?;
    
    // Verify PIN using Argon2
    let verified = security::verify_pin(&pin, &pin_hash)
        .map_err(|e| format!("PIN verification error: {}", e))?;

    if verified {
        // Reset failed attempts on success
        services::data_client::reset_pin_attempts(&user.id).await?;

        audit::log_success(
            "pin_verified",
            Some(user.id.clone()),
            format!("PIN verified successfully for identifier: {}", user_identifier),
        );

        Ok(true)
    } else {
        // Increment failed attempts
        services::data_client::increment_failed_attempts(&user.id).await?;

        audit::log_failure(
            "pin_verification_failed",
            Some(user.id.clone()),
            format!("Incorrect PIN for identifier: {}", user_identifier),
        );

        Ok(false)
    }
}

/// Change user PIN (requires verification of old PIN)
///
/// # Arguments
/// * `user_identifier` - Phone number, principal ID, or user ID
/// * `old_pin` - Current PIN for verification
/// * `new_pin` - New 4-digit PIN (must be numeric)
///
/// # Returns
/// * `Ok(())` if PIN changed successfully
/// * `Err` if old PIN is incorrect or new PIN is invalid
///
/// # Security
/// - Requires old PIN verification (prevents unauthorized changes)
/// - New PIN is validated and hashed with fresh salt
/// - Change is logged in audit trail
#[update]
async fn change_pin(user_identifier: String, old_pin: String, new_pin: String) -> Result<(), String> {
    config::verify_authorized_caller()?;

    // Get user by phone, principal, or user ID
    // Use generic error message to prevent user enumeration
    let user = if let Some(u) = services::data_client::get_user_by_phone(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user_by_principal(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user(&user_identifier).await? {
        u
    } else {
        audit::log_failure(
            "pin_change_failed",
            None,
            format!("Invalid credentials for identifier: {}", user_identifier),
        );
        return Err("Invalid credentials".to_string());
    };
    
    // Validate new PIN format
    logic::user_logic::validate_pin_format(&new_pin)?;
    
    // Get stored PIN hash
    let stored_hash = services::data_client::get_pin_hash(&user.id).await?;
    
    // Verify old PIN
    let verified = security::verify_pin(&old_pin, &stored_hash)
        .map_err(|e| format!("PIN verification error: {}", e))?;

    if !verified {
        audit::log_failure(
            "pin_change_failed",
            Some(user.id.clone()),
            format!("Incorrect old PIN for user: {}", user_identifier),
        );
        return Err("Invalid credentials".to_string());
    }
    
    // Hash new PIN
    let new_hash = security::hash_pin(&new_pin).await
        .map_err(|e| format!("Failed to hash new PIN: {}", e))?;
    
    // Store new hash
    services::data_client::store_pin_hash(&user.id, &new_hash).await?;

    audit::log_success(
        "pin_changed",
        Some(user.id.clone()),
        format!("PIN changed for user: {}", user_identifier),
    );

    Ok(())
}

/// Link phone number to existing principal-based account
///
/// # Arguments
/// * `principal_id` - Principal ID of existing user
/// * `phone_number` - Phone number to link (format: +256...)
///
/// # Returns
/// * `Ok(())` if phone linked successfully
/// * `Err` if principal not found, phone already taken, or invalid format
///
/// # Use Case
/// Enables USSD access for web-registered users by adding phone number
#[update]
async fn link_phone_to_account(principal_id: String, phone_number: String) -> Result<(), String> {
    config::verify_authorized_caller()?;
    
    // Validate phone format
    logic::user_logic::validate_phone_number_format(&phone_number)?;
    
    // Get user by principal (use generic error to prevent enumeration)
    let user = services::data_client::get_user_by_principal(&principal_id).await?
        .ok_or_else(|| "Unable to link phone number. Please verify your account.".to_string())?;

    // Check if phone is already taken (use generic error to prevent enumeration)
    if services::data_client::get_user_by_phone(&phone_number).await?.is_some() {
        audit::log_failure(
            "phone_link_failed",
            Some(user.id.clone()),
            format!("Phone number already registered: {}", phone_number),
        );
        return Err("Unable to link phone number. This phone may already be in use.".to_string());
    }
    
    // Update user phone
    services::data_client::update_user_phone(&user.id, &phone_number).await?;

    audit::log_success(
        "phone_linked",
        Some(user.id.clone()),
        format!("Linked phone {} to principal {}", phone_number, principal_id),
    );

    Ok(())
}

/// Get user profile
#[query]
fn get_user_profile(_user_identifier: String) -> Result<UserProfile, String> {
    Err("Query calls cannot make inter-canister calls. Use update call instead.".to_string())
}

/// Get user profile (update version for inter-canister calls)
///
/// # Arguments
/// * `user_identifier` - Phone number, principal ID, or user ID
///
/// # Returns
/// Simplified user profile with public information (no PIN or sensitive data)
#[update]
async fn get_user_profile_update(user_identifier: String) -> Result<UserProfile, String> {
    config::verify_authorized_caller()?;
    
    // Get user by phone, principal, or user ID
    let user = if let Some(u) = services::data_client::get_user_by_phone(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user_by_principal(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user(&user_identifier).await? {
        u
    } else {
        return Err("User not found".to_string());
    };
    
    Ok(UserProfile::from(user))
}

/// Update user profile information
///
/// # Arguments
/// * `user_identifier` - Phone number or user ID
/// * `updates` - Fields to update (optional first_name, last_name, email, preferred_currency)
///
/// # Returns
/// * `Ok(())` if profile updated successfully
/// * `Err` if user not found or validation fails
///
/// # Note
/// Currently not fully implemented in data_canister
#[update]
async fn update_user_profile(user_identifier: String, updates: ProfileUpdates) -> Result<(), String> {
    config::verify_authorized_caller()?;
    
    // Get user (use generic error to prevent enumeration)
    let _user = if let Some(u) = services::data_client::get_user_by_phone(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user(&user_identifier).await? {
        u
    } else {
        return Err("Unable to update profile. Please verify your account.".to_string());
    };
    
    // Validate updates
    if let Some(ref first_name) = updates.first_name {
        logic::user_logic::validate_name(first_name, "First name")?;
    }
    
    if let Some(ref last_name) = updates.last_name {
        logic::user_logic::validate_name(last_name, "Last name")?;
    }
    
    if let Some(ref email) = updates.email {
        logic::user_logic::validate_email_format(email)?;
    }
    
    if let Some(ref currency) = updates.preferred_currency {
        FiatCurrency::from_string(currency)
            .map_err(|e| format!("Invalid currency: {}", e))?;
    }
    
    // TODO: Call data canister to update profile
    // For now, return not implemented
    Err("Profile updates not yet implemented in data canister".to_string())
}

/// Get user by phone
#[query]
fn get_user_by_phone(_phone: String) -> Result<UserProfile, String> {
    Err("Query calls cannot make inter-canister calls. Use update call instead.".to_string())
}

/// Get user by phone number (update version for inter-canister calls)
///
/// # Arguments
/// * `phone` - Phone number in E.164 format (+256...)
///
/// # Returns
/// User profile if found, error if not found
#[update]
async fn get_user_by_phone_update(phone: String) -> Result<UserProfile, String> {
    config::verify_authorized_caller()?;
    
    let user = services::data_client::get_user_by_phone(&phone).await?
        .ok_or_else(|| "User not found".to_string())?;
    
    Ok(UserProfile::from(user))
}

/// Get user by principal
#[query]
fn get_user_by_principal(_principal: String) -> Result<UserProfile, String> {
    Err("Query calls cannot make inter-canister calls. Use update call instead.".to_string())
}

/// Get user by principal ID (update version for inter-canister calls)
///
/// # Arguments
/// * `principal` - Principal ID string
///
/// # Returns
/// User profile if found, error if not found
#[update]
async fn get_user_by_principal_update(principal: String) -> Result<UserProfile, String> {
    config::verify_authorized_caller()?;

    let user = services::data_client::get_user_by_principal(&principal).await?
        .ok_or_else(|| "User not found".to_string())?;

    Ok(UserProfile::from(user))
}

/// Get user's principal ID by user_id (canister only - for ICRC-1 ledger operations)
///
/// # Arguments
/// * `user_id` - Internal user ID
///
/// # Returns
/// * `Ok(Some(principal))` if user has principal ID
/// * `Ok(None)` if user exists but has no principal (phone-only account)
/// * `Err` if canister call fails
///
/// # Use Case
/// Required for ICRC-1 token transfers which need Principal type for account identification
#[update]
async fn get_user_principal(user_id: String) -> Result<Option<String>, String> {
    config::verify_authorized_caller()?;

    let user = services::data_client::get_user(&user_id).await?;

    Ok(user.and_then(|u| u.principal_id))
}

// ============================================================================
// AUDIT & TRACING ENDPOINTS
// ============================================================================

/// Get recent audit log entries
///
/// # Security
/// Only authorized canisters and controllers can access audit logs
#[query]
fn get_audit_log(limit: Option<u64>) -> Result<Vec<AuditEntry>, String> {
    config::verify_authorized_caller()?;
    Ok(audit::get_audit_log(limit.map(|l| l as usize)))
}

/// Get audit entries for a specific user
///
/// # Security
/// Only authorized canisters and controllers can access audit logs
#[query]
fn get_user_audit_log(user_id: String, limit: Option<u64>) -> Result<Vec<AuditEntry>, String> {
    config::verify_authorized_caller()?;
    Ok(audit::get_user_audit_log(&user_id, limit.map(|l| l as usize)))
}

/// Get audit entries by action type
///
/// # Security
/// Only authorized canisters and controllers can access audit logs
#[query]
fn get_audit_by_action(action: String, limit: Option<u64>) -> Result<Vec<AuditEntry>, String> {
    config::verify_authorized_caller()?;
    Ok(audit::get_audit_by_action(&action, limit.map(|l| l as usize)))
}

/// Get failed operations (for debugging)
///
/// # Security
/// Only authorized canisters and controllers can access audit logs
#[query]
fn get_failed_operations(limit: Option<u64>) -> Result<Vec<AuditEntry>, String> {
    config::verify_authorized_caller()?;
    Ok(audit::get_failed_operations(limit.map(|l| l as usize)))
}

/// Get audit log statistics
///
/// # Security
/// Only authorized canisters and controllers can access audit logs
#[query]
fn get_audit_stats() -> Result<shared_types::audit::AuditStats, String> {
    config::verify_authorized_caller()?;
    Ok(audit::get_audit_stats())
}

// ============================================================================
// CANISTER LIFECYCLE
// ============================================================================

/// State to be preserved across upgrades
#[derive(CandidType, Deserialize)]
struct CanisterState {
    audit_entries: Vec<AuditEntry>,
    authorized_canisters: Vec<Principal>,
    data_canister_id: Option<Principal>,
    test_mode: bool,
}

#[init]
fn init() {
    ic_cdk::println!("🚀 User Canister initialized");
    audit::log_success(
        "canister_initialized",
        None,
        "User canister initialized successfully".to_string(),
    );
}

/// Save state before canister upgrade
#[pre_upgrade]
fn pre_upgrade() {
    // Collect current state
    let audit_entries = audit::get_all_entries();
    let authorized_canisters = config::get_authorized_canisters();
    let data_canister_id = config::get_data_canister_id().ok();
    let test_mode = config::get_test_mode();

    let state = CanisterState {
        audit_entries,
        authorized_canisters,
        data_canister_id,
        test_mode,
    };

    // Save to stable memory
    ic_cdk::storage::stable_save((state,))
        .expect("Failed to save state to stable memory");

    ic_cdk::println!("✅ Pre-upgrade: State saved to stable memory");
}

/// Restore state after canister upgrade
#[post_upgrade]
fn post_upgrade() {
    // Restore from stable memory
    let (state,): (CanisterState,) = ic_cdk::storage::stable_restore()
        .expect("Failed to restore state from stable memory");

    // Restore configuration
    if let Some(data_id) = state.data_canister_id {
        config::restore_data_canister_id(data_id);
    }

    for canister in state.authorized_canisters {
        config::restore_authorized_canister(canister);
    }

    config::restore_test_mode(state.test_mode);

    // Restore audit log
    audit::restore_entries(state.audit_entries);

    ic_cdk::println!("✅ Post-upgrade: State restored from stable memory");

    audit::log_success(
        "canister_upgraded",
        None,
        "User canister upgraded successfully".to_string(),
    );
}

// Export Candid interface
ic_cdk::export_candid!();
