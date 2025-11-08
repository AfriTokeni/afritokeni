use super::data_client;
use candid::{CandidType, Deserialize};

// ============================================================================
// User Management Service - Business Logic
// ============================================================================

#[derive(CandidType, Deserialize)]
pub struct CreateUserData {
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,
}

/// Register a new user (USSD or Web)
pub async fn register_user(
    phone_number: Option<String>,
    principal_id: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
    preferred_currency: String,
    pin: String,
) -> Result<String, String> {
    // Validate inputs
    if phone_number.is_none() && principal_id.is_none() {
        return Err("Either phone number or principal ID is required".to_string());
    }
    
    if pin.len() != 4 {
        return Err("PIN must be exactly 4 digits".to_string());
    }
    
    // Create user in data canister
    let user_data = CreateUserData {
        phone_number: phone_number.clone(),
        principal_id: principal_id.clone(),
        first_name,
        last_name,
        email,
        preferred_currency,
    };
    
    // Create user
    let user = data_client::create_user(user_data).await?;
    
    // Generate salt and setup PIN
    let salt = format!("salt_{}", ic_cdk::api::time());
    data_client::setup_pin(&user.id, &pin, &salt).await?;
    
    // Return user ID
    Ok(user.id)
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
