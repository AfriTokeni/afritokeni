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

/// Link phone number to existing user (for account reconciliation)
pub fn link_phone_to_user(
    state: &mut DataCanisterState,
    user_id: &str,
    phone_number: String,
) -> Result<(), String> {
    // Check if phone already exists
    if state.users.values().any(|u| u.phone_number.as_ref() == Some(&phone_number)) {
        return Err("Phone number already registered to another user".to_string());
    }
    
    let user = state.users.get_mut(user_id)
        .ok_or("User not found")?;
    
    user.phone_number = Some(phone_number.clone());
    
    // Log audit entry using shared library
    audit::log_success(
        "phone_linked",
        Some(user_id.to_string()),
        format!("Linked phone: {}", phone_number)
    );
    
    Ok(())
}

/// Link principal to existing user (for account reconciliation)
pub fn link_principal_to_user(
    state: &mut DataCanisterState,
    user_id: &str,
    principal_id: String,
) -> Result<(), String> {
    // Check if principal already exists
    if state.users.values().any(|u| u.principal_id.as_ref() == Some(&principal_id)) {
        return Err("Principal already registered to another user".to_string());
    }
    
    let user = state.users.get_mut(user_id)
        .ok_or("User not found")?;
    
    user.principal_id = Some(principal_id.clone());
    
    // Log audit entry using shared library
    audit::log_success(
        "principal_linked",
        Some(user_id.to_string()),
        format!("Linked principal: {}", principal_id)
    );
    
    Ok(())
}

/// Update KYC status
pub fn update_kyc_status(
    state: &mut DataCanisterState,
    user_id: &str,
    status: KYCStatus,
) -> Result<(), String> {
    let user = state.users.get_mut(user_id)
        .ok_or("User not found")?;
    
    user.kyc_status = status;
    
    if matches!(status, KYCStatus::Approved) {
        user.is_verified = true;
    }
    
    // Log audit entry using shared library
    audit::log_success(
        "kyc_updated",
        Some(user_id.to_string()),
        format!("KYC status updated to: {:?}", status)
    );
    
    Ok(())
}
