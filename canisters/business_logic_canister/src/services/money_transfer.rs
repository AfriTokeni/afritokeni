use crate::models::*;
use super::{data_client, fraud_detection};

// ============================================================================
// Money Transfer Service - Business Logic
// ============================================================================

/// Transfer money between users (works with phone or principal)
pub async fn transfer_money(
    from_identifier: String,
    to_identifier: String,
    amount: u64,
    currency: String,
    pin: String,
) -> Result<TransactionResult, String> {
    // 1. Get users from data canister
    let from_user = get_user_by_identifier(&from_identifier).await?;
    let to_user = get_user_by_identifier(&to_identifier).await?;
    
    // 2. Verify PIN (business logic: must verify before proceeding)
    let pin_verified = data_client::verify_pin(&from_user.id, &pin).await?;
    if !pin_verified {
        return Err("Invalid PIN".to_string());
    }
    
    // 3. Check balance (business logic: must have sufficient funds)
    let from_balance = data_client::get_fiat_balance(&from_user.id, &currency).await?;
    if from_balance < amount {
        return Err(format!("Insufficient balance. Have: {}, Need: {}", from_balance, amount));
    }
    
    // 4. Fraud detection (business logic: check for suspicious activity)
    let fraud_check = fraud_detection::check_transaction(&from_user.id, amount, &currency)?;
    if fraud_check.should_block {
        return Err(format!("Transaction blocked: {:?}", fraud_check.warnings));
    }
    
    // 5. Execute transfer (update both balances)
    let new_from_balance = from_balance - amount;
    data_client::set_fiat_balance(&from_user.id, &currency, new_from_balance).await?;
    
    let to_balance = data_client::get_fiat_balance(&to_user.id, &currency).await?;
    let new_to_balance = to_balance + amount;
    data_client::set_fiat_balance(&to_user.id, &currency, new_to_balance).await?;
    
    // 6. Record transaction
    let tx_id = generate_transaction_id();
    let tx_record = data_client::TransactionRecord {
        id: tx_id.clone(),
        transaction_type: "transfer_fiat".to_string(),
        from_user: Some(from_user.id.clone()),
        to_user: Some(to_user.id.clone()),
        amount,
        currency: currency.clone(),
        timestamp: ic_cdk::api::time() / 1_000_000_000,
        status: "completed".to_string(),
    };
    
    data_client::store_transaction(&tx_record).await?;
    
    // 7. Return result
    Ok(TransactionResult {
        transaction_id: tx_id,
        from_user: from_user.id,
        to_user: to_user.id,
        amount,
        currency,
        new_balance: new_from_balance,
        timestamp: tx_record.timestamp,
    })
}

/// Send money to phone number (convenience for USSD)
pub async fn send_money_to_phone(
    from_phone: String,
    to_phone: String,
    amount: u64,
    currency: String,
    pin: String,
) -> Result<TransactionResult, String> {
    transfer_money(from_phone, to_phone, amount, currency, pin).await
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get user by phone or principal
async fn get_user_by_identifier(identifier: &str) -> Result<data_client::User, String> {
    // Try phone first (most common for USSD)
    if let Some(user) = data_client::get_user_by_phone(identifier).await? {
        return Ok(user);
    }
    
    // Try as user ID
    if let Some(user) = data_client::get_user(identifier).await? {
        return Ok(user);
    }
    
    Err(format!("User not found: {}", identifier))
}

/// Generate unique transaction ID
fn generate_transaction_id() -> String {
    format!("tx_{}", ic_cdk::api::time())
}
