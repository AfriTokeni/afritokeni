/// Admin endpoints for testing
/// These should be protected in production!

use crate::utils::datastore;

/// Set user balance for testing
pub async fn set_user_balance(phone_number: &str, kes: f64, ckbtc: f64, ckusdc: f64) -> Result<(), String> {
    // Store balances in datastore
    datastore::set_user_data(phone_number, "kes_balance", &kes.to_string()).await?;
    datastore::set_user_data(phone_number, "ckbtc_balance", &ckbtc.to_string()).await?;
    datastore::set_user_data(phone_number, "ckusdc_balance", &ckusdc.to_string()).await?;
    Ok(())
}

/// Set user PIN for testing
pub async fn set_user_pin(phone_number: &str, pin: &str) -> Result<(), String> {
    // Hash and store PIN
    crate::utils::pin::setup_pin(phone_number, pin).await
}

/// Get user balance for verification
pub async fn get_user_balance(phone_number: &str) -> Result<(f64, f64, f64), String> {
    let kes = datastore::get_user_data(phone_number, "kes_balance")
        .await?
        .unwrap_or("0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    
    let ckbtc = datastore::get_user_data(phone_number, "ckbtc_balance")
        .await?
        .unwrap_or("0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    
    let ckusdc = datastore::get_user_data(phone_number, "ckusdc_balance")
        .await?
        .unwrap_or("0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    
    Ok((kes, ckbtc, ckusdc))
}

/// Reset PIN attempts for a user
pub async fn reset_pin_attempts(phone_number: &str) -> Result<(), String> {
    crate::utils::juno_client::set_pin_attempts(phone_number, 0).await?;
    Ok(())
}

/// Setup test user in one call (PIN + reset attempts + reset language)
pub async fn setup_test_user(phone_number: &str, pin: &str, kes: f64, ckbtc: f64, ckusdc: f64) -> Result<(), String> {
    // Set PIN
    crate::utils::juno_client::set_user_pin(phone_number, pin).await?;
    
    // Reset PIN attempts
    crate::utils::juno_client::set_pin_attempts(phone_number, 0).await?;
    
    // Delete language preference (will default to English)
    let _ = crate::utils::juno_client::delete_user_language(phone_number).await;
    
    // Set balances
    if kes > 0.0 {
        datastore::set_user_data(phone_number, "kes_balance", &kes.to_string()).await?;
    }
    if ckbtc > 0.0 {
        datastore::set_user_data(phone_number, "ckbtc_balance", &ckbtc.to_string()).await?;
    }
    if ckusdc > 0.0 {
        datastore::set_user_data(phone_number, "ckusdc_balance", &ckusdc.to_string()).await?;
    }
    
    Ok(())
}
