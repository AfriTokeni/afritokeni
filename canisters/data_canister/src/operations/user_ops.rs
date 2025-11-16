use crate::models::*;
use crate::DataCanisterState;
use ic_cdk::api::time;
use shared_types::audit;

/// Create a new user
pub fn create_user(
    state: &mut DataCanisterState,
    user_data: CreateUserData,
) -> Result<User, String> {
    // Generate UUID for user (simple timestamp-based for now)
    let user_id = format!("user_{}", time());

    // Validate: phone or principal must be provided
    if user_data.phone_number.is_none() && user_data.principal_id.is_none() {
        return Err("Either phone_number or principal_id must be provided".to_string());
    }

    // Validate phone number format if provided
    if let Some(ref phone) = user_data.phone_number {
        if phone.is_empty() {
            return Err("Phone number cannot be empty".to_string());
        }
        if phone.len() > 20 {
            return Err("Phone number too long (max 20 characters)".to_string());
        }
        if !phone.chars().all(|c| c.is_ascii_digit() || c == '+') {
            return Err("Phone number must contain only digits and optional '+' prefix".to_string());
        }
    }

    // Validate principal ID format if provided
    if let Some(ref principal) = user_data.principal_id {
        if principal.is_empty() {
            return Err("Principal ID cannot be empty".to_string());
        }
        if principal.len() > 100 {
            return Err("Principal ID too long (max 100 characters)".to_string());
        }
    }

    // Validate first name
    if user_data.first_name.is_empty() {
        return Err("First name cannot be empty".to_string());
    }
    if user_data.first_name.len() > 100 {
        return Err("First name too long (max 100 characters)".to_string());
    }

    // Validate last name
    if user_data.last_name.is_empty() {
        return Err("Last name cannot be empty".to_string());
    }
    if user_data.last_name.len() > 100 {
        return Err("Last name too long (max 100 characters)".to_string());
    }

    // Validate email
    if !user_data.email.is_empty() {
        if user_data.email.len() > 255 {
            return Err("Email too long (max 255 characters)".to_string());
        }
        if !user_data.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
    }

    // Check if phone number already exists
    if let Some(ref phone) = user_data.phone_number {
        if state.users.values().any(|u| u.phone_number.as_ref() == Some(phone)) {
            return Err("Phone number already registered".to_string());
        }
    }

    // Check if principal already exists
    if let Some(ref principal) = user_data.principal_id {
        if state.users.values().any(|u| u.principal_id.as_ref() == Some(principal)) {
            return Err("Principal already registered".to_string());
        }
    }

    let now = time() / 1_000_000_000;

    let user = User {
        id: user_id.clone(),
        phone_number: user_data.phone_number.clone(),
        principal_id: user_data.principal_id.clone(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        email: user_data.email.clone(),
        user_type: user_data.user_type,
        preferred_currency: user_data.preferred_currency,
        kyc_status: KYCStatus::NotStarted,
        is_verified: false,
        created_at: now,
        last_active: now,
    };

    // Store user
    state.users.insert(user_id.clone(), user.clone());

    // Log audit entry using shared library
    audit::log_success(
        "user_created",
        Some(user_id),
        format!("Created user: {} {}", user.first_name, user.last_name)
    );

    Ok(user)
}

/// Update user's last active timestamp
pub fn update_last_active(
    state: &mut DataCanisterState,
    user_id: &str,
) -> Result<(), String> {
    let user = state.users.get_mut(user_id)
        .ok_or("User not found")?;

    user.last_active = time() / 1_000_000_000;

    Ok(())
}

// ============================================================================
// KYC Status Management
// ============================================================================

/// Update user KYC status (canister only - for compliance)
/// Called by authorized canisters after KYC verification process
pub fn update_kyc_status(
    state: &mut DataCanisterState,
    user_id: &str,
    status: KYCStatus,
) -> Result<(), String> {
    // Input validation
    if user_id.is_empty() {
        return Err("User ID cannot be empty".to_string());
    }

    // Get user
    let user = state.users.get_mut(user_id)
        .ok_or_else(|| format!("User not found: {}", user_id))?;

    // Store old status for audit logging
    let old_status = user.kyc_status;

    // Update KYC status
    user.kyc_status = status;

    // If KYC approved, mark user as verified
    if status == KYCStatus::Approved {
        user.is_verified = true;
    }

    // Log audit using shared library
    audit::log_success(
        "kyc_updated",
        Some(user_id.to_string()),
        format!("KYC status updated: {:?} -> {:?}", old_status, status)
    );

    Ok(())
}

// ============================================================================
// REMOVED FUNCTIONS - Not exposed in API, account linking handled by user_canister
// ============================================================================
// The following functions were defined but never used:
// - link_phone_to_user: Phone linking is handled by user_canister.link_phone_to_account()
// - link_principal_to_user: Principal is set during user creation via CreateUserData
//
// If account reconciliation features are needed in the future, they should be:
// 1. Implemented in user_canister (business logic layer)
// 2. Use data_canister.update_user_phone() for storage
// 3. Follow the established pattern of separation of concerns
//
