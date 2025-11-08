use crate::models::*;
use super::data_client;

// ============================================================================
// Balance Queries Service - Business Logic
// ============================================================================

/// Get all balances for a user
pub async fn get_balances(user_identifier: String) -> Result<UserBalances, String> {
    // Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // Get crypto balances
    let (ckbtc, ckusdc) = data_client::get_crypto_balance(&user.id).await?;
    
    // Get fiat balance for preferred currency
    let fiat_balance = data_client::get_fiat_balance(&user.id, &user.preferred_currency).await?;
    
    let fiat_balances = vec![FiatBalance {
        currency: user.preferred_currency,
        balance: fiat_balance,
    }];
    
    Ok(UserBalances {
        fiat_balances,
        ckbtc_balance: ckbtc,
        ckusdc_balance: ckusdc,
    })
}

/// Check fiat balance for specific currency
pub async fn check_fiat_balance(
    user_identifier: String,
    currency: String,
) -> Result<u64, String> {
    let user = get_user_by_identifier(&user_identifier).await?;
    data_client::get_fiat_balance(&user.id, &currency).await
}

/// Check crypto balance
pub async fn check_crypto_balance(
    user_identifier: String,
    crypto_type: CryptoType,
) -> Result<u64, String> {
    let user = get_user_by_identifier(&user_identifier).await?;
    let (ckbtc, ckusdc) = data_client::get_crypto_balance(&user.id).await?;
    
    match crypto_type {
        CryptoType::CkBTC => Ok(ckbtc),
        CryptoType::CkUSDC => Ok(ckusdc),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

async fn get_user_by_identifier(identifier: &str) -> Result<data_client::User, String> {
    if let Some(user) = data_client::get_user_by_phone(identifier).await? {
        return Ok(user);
    }
    if let Some(user) = data_client::get_user(identifier).await? {
        return Ok(user);
    }
    Err(format!("User not found: {}", identifier))
}
