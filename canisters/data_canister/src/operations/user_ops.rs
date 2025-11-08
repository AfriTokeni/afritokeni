use crate::models::*;
use crate::DataCanisterState;
use ic_cdk::api::time;

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
        phone_number: user_data.phone_number,
        principal_id: user_data.principal_id,
        first_name: user_data.first_name,
        last_name: user_data.last_name,
        email: user_data.email,
        user_type: user_data.user_type,
        preferred_currency: user_data.preferred_currency,
        kyc_status: KYCStatus::NotStarted,
        is_verified: false,
        created_at: now,
        last_active: now,
    };
    
    // Log audit entry
    let audit_entry = AuditEntry {
        timestamp: now,
        action: "user_created".to_string(),
        user_id: Some(user_id.clone()),
        details: format!("Created user: {} {}", user.first_name, user.last_name),
    };
    state.log_audit(audit_entry);
    
    // Store user
    state.users.insert(user_id, user.clone());
    
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
    
    let now = time() / 1_000_000_000;
    let audit_entry = AuditEntry {
        timestamp: now,
        action: "phone_linked".to_string(),
        user_id: Some(user_id.to_string()),
        details: format!("Linked phone: {}", phone_number),
    };
    state.log_audit(audit_entry);
    
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
    
    let now = time() / 1_000_000_000;
    let audit_entry = AuditEntry {
        timestamp: now,
        action: "principal_linked".to_string(),
        user_id: Some(user_id.to_string()),
        details: format!("Linked principal: {}", principal_id),
    };
    state.log_audit(audit_entry);
    
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
    
    let now = time() / 1_000_000_000;
    let audit_entry = AuditEntry {
        timestamp: now,
        action: "kyc_updated".to_string(),
        user_id: Some(user_id.to_string()),
        details: format!("KYC status updated to: {:?}", status),
    };
    state.log_audit(audit_entry);
    
    Ok(())
}
