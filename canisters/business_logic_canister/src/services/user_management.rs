use super::data_client;
use shared_types::{CreateUserData, FiatCurrency, UserType};

// ============================================================================
// User Management Service - Business Logic
// ============================================================================

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
    
    // Convert currency string to enum
    let currency_enum = FiatCurrency::from_string(&preferred_currency)?;
    
    // Create user in data canister (order MUST match Data Canister's CreateUserData)
    let user_data = CreateUserData {
        user_type: UserType::User,
        preferred_currency: currency_enum,
        email,
        first_name,
        last_name,
        principal_id: principal_id.clone(),
        phone_number: phone_number.clone(),
    };
    
    // Create user
    let user = data_client::create_user(user_data).await?;
    
    // Generate random 32-byte salt as hex string
    let time = ic_cdk::api::time();
    let salt_bytes: Vec<u8> = (0..32).map(|i| ((time >> (i % 8)) ^ i) as u8).collect();
    let salt = hex::encode(salt_bytes);
    
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
