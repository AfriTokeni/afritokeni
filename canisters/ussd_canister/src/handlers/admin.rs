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

/// Clear all test data
pub async fn clear_test_data() -> Result<(), String> {
    // Clear all sessions and user data
    datastore::clear_all_data().await
}
