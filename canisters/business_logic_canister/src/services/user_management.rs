use super::data_client;

// ============================================================================
// User Management Service - Business Logic
// ============================================================================

/// Register new user
pub async fn register_user(
    phone_number: Option<String>,
    principal_id: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
    preferred_currency: String,
    pin: String,
) -> Result<String, String> {
    // Validation: must have phone or principal
    if phone_number.is_none() && principal_id.is_none() {
        return Err("Must provide phone number or principal ID".to_string());
    }
    
    // Check if user already exists
    if let Some(ref phone) = phone_number {
        if data_client::get_user_by_phone(phone).await?.is_some() {
            return Err("Phone number already registered".to_string());
        }
    }
    
    // TODO: Create user in data canister
    // TODO: Setup PIN in data canister
    
    // For now, return placeholder
    Ok("user_id_placeholder".to_string())
}

/// Link phone number to existing account
pub async fn link_phone_to_account(
    principal_id: String,
    phone_number: String,
    pin: String,
) -> Result<(), String> {
    // Get user by principal
    let user = data_client::get_user(&principal_id).await?
        .ok_or("User not found")?;
    
    // Verify PIN
    let verified = data_client::verify_pin(&user.id, &pin).await?;
    if !verified {
        return Err("Invalid PIN".to_string());
    }
    
    // Check if phone already in use
    if data_client::get_user_by_phone(&phone_number).await?.is_some() {
        return Err("Phone number already in use".to_string());
    }
    
    // TODO: Update user in data canister to add phone number
    
    Ok(())
}
