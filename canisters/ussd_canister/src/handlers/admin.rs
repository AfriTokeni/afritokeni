/// Admin endpoints for testing
/// These should be protected in production!

use crate::utils::data_canister_client::{self, FiatCurrency, UserType, CreateUserData};

/// Set user balance for testing
pub async fn set_user_balance(phone_number: &str, kes: f64, ckbtc: f64, ckusdc: f64) -> Result<(), String> {
    let client = data_canister_client::create_client()?;
    
    // Get or create user
    let user = match client.get_user_by_phone(phone_number).await? {
        Some(u) => u,
        None => {
            // Create user if doesn't exist
            let user_data = CreateUserData {
                phone_number: Some(phone_number.to_string()),
                principal_id: None,
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                email: format!("{}@test.com", phone_number),
                user_type: UserType::User,
                preferred_currency: FiatCurrency::UGX,
            };
            client.create_user(user_data).await?
        }
    };
    
    // Deposit UGX (convert from float to smallest unit - cents)
    if kes > 0.0 {
        let amount_cents = (kes * 100.0) as u64;
        client.deposit_fiat(&user.id, amount_cents, FiatCurrency::UGX, Some("Test deposit".to_string())).await?;
    }
    
    // Update crypto balances
    if ckbtc > 0.0 || ckusdc > 0.0 {
        let ckbtc_sats = (ckbtc * 100_000_000.0) as i64;  // Convert BTC to satoshis
        let ckusdc_micro = (ckusdc * 1_000_000.0) as i64;  // Convert USDC to micro-USDC
        
        client.update_crypto_balance(&user.id, ckbtc_sats, ckusdc_micro).await?;
    }
    
    Ok(())
}

/// Set user PIN for testing
pub async fn set_user_pin(phone_number: &str, pin: &str) -> Result<(), String> {
    let client = data_canister_client::create_client()?;
    
    // Get user
    let user = client.get_user_by_phone(phone_number).await?
        .ok_or("User not found")?;
    
    // Setup PIN
    client.setup_user_pin(&user.id, pin).await
}

/// Get user balance for verification
pub async fn get_user_balance(phone_number: &str) -> Result<(f64, f64, f64), String> {
    let client = data_canister_client::create_client()?;
    
    // Get user
    let user = client.get_user_by_phone(phone_number).await?
        .ok_or("User not found")?;
    
    // Get UGX balance (convert from cents to float)
    let kes_cents = client.get_fiat_balance(&user.id, FiatCurrency::UGX).await?;
    let kes = (kes_cents as f64) / 100.0;
    
    // Get crypto balances
    let (ckbtc_sats, ckusdc_micro) = client.get_crypto_balance(&user.id).await?;
    let ckbtc = (ckbtc_sats as f64) / 100_000_000.0;  // Convert satoshis to BTC
    let ckusdc = (ckusdc_micro as f64) / 1_000_000.0;  // Convert micro-USDC to USDC
    
    Ok((kes, ckbtc, ckusdc))
}

/// Reset PIN attempts for a user
pub async fn reset_pin_attempts(_phone_number: &str) -> Result<(), String> {
    // PIN attempts are now managed by data canister
    // This is handled automatically by the data canister's PIN verification
    ic_cdk::println!("ℹ️  PIN attempts are now managed by data canister");
    Ok(())
}

/// Setup test user in one call (PIN + balances)
pub async fn setup_test_user(phone_number: &str, pin: &str, kes: f64, _ckbtc: f64, _ckusdc: f64) -> Result<(), String> {
    let client = data_canister_client::create_client()?;
    
    // Create user if doesn't exist
    let user = match client.get_user_by_phone(phone_number).await? {
        Some(u) => u,
        None => {
            let user_data = CreateUserData {
                phone_number: Some(phone_number.to_string()),
                principal_id: None,
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                email: format!("{}@test.com", phone_number),
                user_type: UserType::User,
                preferred_currency: FiatCurrency::UGX,
            };
            client.create_user(user_data).await?
        }
    };
    
    // Set PIN
    client.setup_user_pin(&user.id, pin).await?;
    
    // Set balance
    if kes > 0.0 {
        let amount_cents = (kes * 100.0) as u64;
        client.deposit_fiat(&user.id, amount_cents, FiatCurrency::UGX, Some("Test deposit".to_string())).await?;
    }
    
    ic_cdk::println!("✅ Test user {} setup complete", phone_number);
    Ok(())
}
