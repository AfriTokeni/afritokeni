use crate::models::*;
use super::{data_client, fraud_detection};

// ============================================================================
// Crypto Operations Service - Business Logic
// ============================================================================

/// Buy cryptocurrency with fiat
pub async fn buy_crypto(
    user_identifier: String,
    fiat_amount: u64,
    fiat_currency: String,
    crypto_type: CryptoType,
    pin: String,
) -> Result<TransactionResult, String> {
    // 1. Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // 2. Verify PIN
    let verified = data_client::verify_pin(&user.id, &pin).await?;
    if !verified {
        return Err("Invalid PIN".to_string());
    }
    
    // 3. Check fiat balance
    let fiat_balance = data_client::get_fiat_balance(&user.id, &fiat_currency).await?;
    if fiat_balance < fiat_amount {
        return Err(format!("Insufficient fiat balance. Have: {}, Need: {}", fiat_balance, fiat_amount));
    }
    
    // 4. Fraud check
    let fraud_check = fraud_detection::check_transaction(&user.id, fiat_amount, &fiat_currency)?;
    
    // Log suspicious transactions even if not blocked (for monitoring)
    if fraud_check.is_suspicious {
        ic_cdk::println!("⚠️ SUSPICIOUS CRYPTO PURCHASE: user={}, amount={}, currency={}, crypto={:?}, warnings={:?}", 
            user.id, fiat_amount, fiat_currency, crypto_type, fraud_check.warnings);
    }
    
    if fraud_check.should_block {
        return Err(format!("Transaction blocked: {:?}", fraud_check.warnings));
    }
    
    // 5. Get exchange rate (TODO: call exchange rate service)
    // For now, using placeholder - will integrate with exchange rate service
    let crypto_amount = calculate_crypto_amount(fiat_amount, &fiat_currency, crypto_type)?;
    
    // 6. Deduct fiat
    let new_fiat_balance = fiat_balance - fiat_amount;
    data_client::set_fiat_balance(&user.id, &fiat_currency, new_fiat_balance).await?;
    
    // 7. Add crypto
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (crypto_amount as i64, 0),
        CryptoType::CkUSDC => (0, crypto_amount as i64),
    };
    data_client::update_crypto_balance(&user.id, ckbtc_delta, ckusdc_delta).await?;
    
    // 8. Record transaction
    let tx_id = generate_transaction_id();
    let tx_record = data_client::TransactionRecord {
        id: tx_id.clone(),
        transaction_type: "buy_crypto".to_string(),
        from_user: Some(user.id.clone()),
        to_user: None,
        amount: fiat_amount,
        currency: fiat_currency.clone(),
        timestamp: ic_cdk::api::time() / 1_000_000_000,
        status: "completed".to_string(),
    };
    data_client::store_transaction(&tx_record).await?;
    
    // 9. Update last active (for security monitoring)
    let _ = data_client::update_last_active(&user.id).await;
    
    // 10. TODO: Call actual ckBTC/ckUSDC ledger to mint/transfer
    
    Ok(TransactionResult {
        transaction_id: tx_id,
        from_user: user.id.clone(),
        to_user: user.id,
        amount: crypto_amount,
        currency: format!("{:?}", crypto_type),
        new_balance: new_fiat_balance,
        timestamp: tx_record.timestamp,
    })
}

/// Send cryptocurrency to address
pub async fn send_crypto(
    user_identifier: String,
    to_address: String,
    amount: u64,
    crypto_type: CryptoType,
    pin: String,
) -> Result<TransactionResult, String> {
    // 1. Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // 2. Verify PIN
    let verified = data_client::verify_pin(&user.id, &pin).await?;
    if !verified {
        return Err("Invalid PIN".to_string());
    }
    
    // 3. Check crypto balance
    let (ckbtc_balance, ckusdc_balance) = data_client::get_crypto_balance(&user.id).await?;
    let current_balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    
    if current_balance < amount {
        return Err(format!("Insufficient crypto balance. Have: {}, Need: {}", current_balance, amount));
    }
    
    // 4. Deduct crypto
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (-(amount as i64), 0),
        CryptoType::CkUSDC => (0, -(amount as i64)),
    };
    data_client::update_crypto_balance(&user.id, ckbtc_delta, ckusdc_delta).await?;
    
    // 5. Record transaction
    let tx_id = generate_transaction_id();
    let tx_record = data_client::TransactionRecord {
        id: tx_id.clone(),
        transaction_type: "send_crypto".to_string(),
        from_user: Some(user.id.clone()),
        to_user: Some(to_address.clone()),
        amount,
        currency: format!("{:?}", crypto_type),
        timestamp: ic_cdk::api::time() / 1_000_000_000,
        status: "completed".to_string(),
    };
    data_client::store_transaction(&tx_record).await?;
    
    // 6. TODO: Call actual ckBTC/ckUSDC ledger to transfer
    
    Ok(TransactionResult {
        transaction_id: tx_id,
        from_user: user.id,
        to_user: to_address,
        amount,
        currency: format!("{:?}", crypto_type),
        new_balance: current_balance - amount,
        timestamp: tx_record.timestamp,
    })
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

fn generate_transaction_id() -> String {
    format!("tx_{}", ic_cdk::api::time())
}

fn calculate_crypto_amount(fiat_amount: u64, _currency: &str, crypto_type: CryptoType) -> Result<u64, String> {
    // TODO: Integrate with exchange rate service
    // Placeholder calculation
    match crypto_type {
        CryptoType::CkBTC => {
            // Assume 1 BTC = 100M fiat units
            Ok(fiat_amount / 100_000_000)
        }
        CryptoType::CkUSDC => {
            // Assume 1 USDC = 3800 fiat units
            Ok(fiat_amount / 3800)
        }
    }
}
