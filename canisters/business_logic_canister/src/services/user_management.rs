use super::data_client;
use crate::logic::user_logic;
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
    // Validate inputs using pure logic functions
    user_logic::validate_identifier_required(&phone_number, &principal_id)?;
    user_logic::validate_pin_format(&pin)?;
    user_logic::validate_name(&first_name, "First name")?;
    user_logic::validate_name(&last_name, "Last name")?;
    user_logic::validate_email_format(&email)?;
    
    // Validate phone number format if provided
    if let Some(ref phone) = phone_number {
        user_logic::validate_phone_number_format(phone)?;
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
    
    // Generate salt using pure logic function
    let time = ic_cdk::api::time();
    let salt = user_logic::generate_salt_from_time(time);
    
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
    // Validate inputs using pure logic functions
    user_logic::validate_phone_number_format(&phone_number)?;
    user_logic::validate_pin_format(&pin)?;
    
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
    
    // Update user phone number in data canister
    data_client::update_user_phone(&user.id, &phone_number).await?;
    
    ic_cdk::println!("✅ Phone {} linked to user {}", phone_number, user.id);
    
    Ok(())
}
