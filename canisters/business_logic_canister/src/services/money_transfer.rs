use crate::models::*;
use crate::logic::transfer_logic;
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
    // Validate inputs using pure logic functions
    transfer_logic::validate_identifier_not_empty(&from_identifier, "Sender identifier")?;
    transfer_logic::validate_identifier_not_empty(&to_identifier, "Recipient identifier")?;
    transfer_logic::validate_amount_positive(amount)?;
    transfer_logic::validate_currency_code(&currency)?;
    transfer_logic::validate_not_self_transfer(&from_identifier, &to_identifier)?;
    
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
    transfer_logic::validate_sufficient_balance(from_balance, amount)?;
    
    // 4. Rate limiting (prevent abuse)
    if !fraud_detection::check_rate_limit(&from_user.id)? {
        return Err("Too many transactions. Please wait before trying again.".to_string());
    }
    
    // 5. Fraud detection (business logic: check for suspicious activity)
    let fraud_check = fraud_detection::check_transaction(&from_user.id, amount, &currency)?;
    
    // Log suspicious transactions even if not blocked (for monitoring)
    if fraud_check.is_suspicious {
        ic_cdk::println!("⚠️ SUSPICIOUS TRANSACTION: user={}, amount={}, currency={}, risk_score={}, requires_review={}, warnings={:?}", 
            from_user.id, amount, currency, fraud_check.risk_score, fraud_check.requires_manual_review, fraud_check.warnings);
    }
    
    // Log high-risk transactions that require manual review
    if fraud_check.requires_manual_review {
        ic_cdk::println!("🚨 HIGH-RISK TRANSACTION REQUIRES MANUAL REVIEW: user={}, amount={}, risk_score={}", 
            from_user.id, amount, fraud_check.risk_score);
    }
    
    if fraud_check.should_block {
        return Err(format!("Transaction blocked: {:?}", fraud_check.warnings));
    }
    
    // 5. Execute transfer (update both balances)
    let new_from_balance = transfer_logic::calculate_new_balance(from_balance, amount)?;
    data_client::set_fiat_balance(&from_user.id, &currency, new_from_balance).await?;
    
    let to_balance = data_client::get_fiat_balance(&to_user.id, &currency).await?;
    let new_to_balance = transfer_logic::calculate_balance_addition(to_balance, amount)?;
    data_client::set_fiat_balance(&to_user.id, &currency, new_to_balance).await?;
    
    // 6. Record transaction
    let timestamp = ic_cdk::api::time();
    let tx_id = transfer_logic::generate_transaction_id(timestamp);
    
    // Convert to proper Transaction type with enums
    let currency_enum = shared_types::FiatCurrency::from_string(&currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let tx = shared_types::Transaction {
        id: tx_id.clone(),
        transaction_type: shared_types::TransactionType::TransferFiat,
        from_user: Some(from_user.id.clone()),
        to_user: Some(to_user.id.clone()),
        amount,
        currency_type: shared_types::CurrencyType::Fiat(currency_enum),
        description: None,
        created_at: ic_cdk::api::time(),
        completed_at: Some(ic_cdk::api::time()),
        status: shared_types::TransactionStatus::Completed,
    };
    
    data_client::store_transaction(&tx).await?;
    
    // 7. Update last active for both users (for security monitoring)
    let _ = data_client::update_last_active(&from_user.id).await;
    let _ = data_client::update_last_active(&to_user.id).await;
    
    // 8. Return result
    Ok(TransactionResult {
        transaction_id: tx_id,
        from_user: from_user.id,
        to_user: to_user.id,
        amount,
        currency,
        new_balance: new_from_balance,
        timestamp: ic_cdk::api::time(),
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

