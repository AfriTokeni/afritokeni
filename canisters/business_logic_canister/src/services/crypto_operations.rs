use crate::models::*;
use crate::logic::{crypto_logic, transfer_logic};
use super::{data_client, fraud_detection, exchange_rate, ledger_client};
use candid::Principal;

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
    // Validate inputs
    transfer_logic::validate_identifier_not_empty(&user_identifier, "User identifier")?;
    crypto_logic::validate_fiat_amount_for_crypto(fiat_amount)?;
    transfer_logic::validate_currency_code(&fiat_currency)?;
    
    // 1. Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // 2. Verify PIN
    let verified = data_client::verify_pin(&user.id, &pin).await?;
    if !verified {
        return Err("Invalid PIN".to_string());
    }
    
    // 3. Check fiat balance
    let fiat_balance = data_client::get_fiat_balance(&user.id, &fiat_currency).await?;
    transfer_logic::validate_sufficient_balance(fiat_balance, fiat_amount)?;
    
    // 4. Rate limiting (prevent abuse)
    if !fraud_detection::check_rate_limit(&user.id)? {
        return Err("Too many transactions. Please wait before trying again.".to_string());
    }
    
    // 5. Fraud check
    let fraud_check = fraud_detection::check_transaction(&user.id, fiat_amount, &fiat_currency)?;
    
    // Log suspicious transactions even if not blocked (for monitoring)
    if fraud_check.is_suspicious {
        ic_cdk::println!("⚠️ SUSPICIOUS CRYPTO PURCHASE: user={}, amount={}, currency={}, crypto={:?}, risk_score={}, requires_review={}, warnings={:?}", 
            user.id, fiat_amount, fiat_currency, crypto_type, fraud_check.risk_score, fraud_check.requires_manual_review, fraud_check.warnings);
    }
    
    // Log high-risk transactions that require manual review
    if fraud_check.requires_manual_review {
        ic_cdk::println!("🚨 HIGH-RISK CRYPTO PURCHASE REQUIRES MANUAL REVIEW: user={}, amount={}, risk_score={}", 
            user.id, fiat_amount, fraud_check.risk_score);
    }
    
    if fraud_check.should_block {
        return Err(format!("Transaction blocked: {:?}", fraud_check.warnings));
    }
    
    // 5. Calculate crypto amount using real exchange rates
    let crypto_type_str = format!("{:?}", crypto_type);
    crypto_logic::validate_crypto_calculation_inputs(fiat_amount, &crypto_type_str)?;
    let crypto_amount = exchange_rate::calculate_crypto_from_fiat(
        fiat_amount,
        &fiat_currency,
        &crypto_type_str
    ).await?;
    
    // 6. Get user principal for ledger transfer (validate BEFORE deducting money)
    let user_principal = Principal::from_text(&user.principal_id.ok_or("User has no principal ID")?)
        .map_err(|e| format!("Invalid user principal: {}", e))?;
    
    let ledger_token = match crypto_type {
        CryptoType::CkBTC => ledger_client::CryptoToken::CkBTC,
        CryptoType::CkUSDC => ledger_client::CryptoToken::CkUSDC,
    };
    
    // 7. Transfer crypto from this canister to user via ICRC-1 FIRST
    // This ensures we don't deduct fiat if ledger transfer fails
    let block_index = ledger_client::transfer_crypto_to_user(
        ledger_token,
        user_principal,
        crypto_amount
    ).await?;
    
    ic_cdk::println!("✅ Crypto transferred to user. Block index: {}", block_index);
    
    // 8. Only AFTER successful ledger transfer, deduct fiat
    let new_fiat_balance = transfer_logic::calculate_new_balance(fiat_balance, fiat_amount)?;
    data_client::set_fiat_balance(&user.id, &fiat_currency, new_fiat_balance).await?;
    
    // 9. Add crypto to user's balance
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (crypto_amount as i64, 0),
        CryptoType::CkUSDC => (0, crypto_amount as i64),
    };
    data_client::update_crypto_balance(&user.id, ckbtc_delta, ckusdc_delta).await?;
    
    // 10. Record transaction
    let timestamp = ic_cdk::api::time();
    let tx_id = transfer_logic::generate_transaction_id(timestamp);
    let currency_enum = shared_types::FiatCurrency::from_string(&fiat_currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let tx = shared_types::Transaction {
        id: tx_id.clone(),
        transaction_type: shared_types::TransactionType::BuyCrypto,
        from_user: Some(user.id.clone()),
        to_user: None,
        amount: fiat_amount,
        currency_type: shared_types::CurrencyType::Fiat(currency_enum),
        description: None,
        created_at: ic_cdk::api::time(),
        completed_at: Some(ic_cdk::api::time()),
        status: shared_types::TransactionStatus::Completed,
    };
    data_client::store_transaction(&tx).await?;
    
    // 11. Update last active (for security monitoring)
    let _ = data_client::update_last_active(&user.id).await;
    
    ic_cdk::println!("✅ Crypto transferred to user. Block index: {}", block_index);
    
    Ok(TransactionResult {
        transaction_id: tx_id,
        from_user: user.id.clone(),
        to_user: user.id,
        amount: crypto_amount,
        currency: format!("{:?}", crypto_type),
        new_balance: new_fiat_balance,
        timestamp: ic_cdk::api::time(),
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
    // Validate inputs
    transfer_logic::validate_identifier_not_empty(&user_identifier, "User identifier")?;
    crypto_logic::validate_crypto_amount_positive(amount)?;
    let crypto_type_str = format!("{:?}", crypto_type);
    crypto_logic::validate_crypto_address(&to_address, &crypto_type_str)?;
    
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
    
    crypto_logic::validate_sufficient_crypto_balance(current_balance, amount)?;
    
    // 4. Validate recipient address and prepare ledger transfer FIRST
    let recipient_principal = Principal::from_text(&to_address)
        .map_err(|e| format!("Invalid recipient address: {}", e))?;
    
    let ledger_token = match crypto_type {
        CryptoType::CkBTC => ledger_client::CryptoToken::CkBTC,
        CryptoType::CkUSDC => ledger_client::CryptoToken::CkUSDC,
    };
    
    // 5. Transfer crypto via ICRC-1 FIRST (before deducting balance)
    let block_index = ledger_client::transfer_crypto_to_user(
        ledger_token,
        recipient_principal,
        amount
    ).await?;
    
    ic_cdk::println!("✅ Crypto sent. Block index: {}", block_index);
    
    // 6. Only AFTER successful ledger transfer, deduct crypto from sender
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (-(amount as i64), 0),
        CryptoType::CkUSDC => (0, -(amount as i64)),
    };
    data_client::update_crypto_balance(&user.id, ckbtc_delta, ckusdc_delta).await?;
    
    // 7. Record transaction
    let timestamp = ic_cdk::api::time();
    let tx_id = transfer_logic::generate_transaction_id(timestamp);
    let tx = shared_types::Transaction {
        id: tx_id.clone(),
        transaction_type: shared_types::TransactionType::TransferCrypto,
        from_user: Some(user.id.clone()),
        to_user: Some(to_address.clone()),
        amount,
        currency_type: shared_types::CurrencyType::Crypto(crypto_type),
        description: None,
        created_at: ic_cdk::api::time(),
        completed_at: Some(ic_cdk::api::time()),
        status: shared_types::TransactionStatus::Completed,
    };
    data_client::store_transaction(&tx).await?;
    
    ic_cdk::println!("✅ Crypto sent to {}. Block index: {}", to_address, block_index);
    
    Ok(TransactionResult {
        transaction_id: tx_id,
        from_user: user.id,
        to_user: to_address,
        amount,
        currency: format!("{:?}", crypto_type),
        new_balance: current_balance - amount,
        timestamp: ic_cdk::api::time(),
    })
}

/// Sell cryptocurrency for fiat via agent (creates escrow)
pub async fn sell_crypto_to_agent(
    user_identifier: String,
    crypto_amount: u64,
    crypto_type: CryptoType,
    agent_id: String,
    pin: String,
) -> Result<TransactionResult, String> {
    // Validate inputs
    transfer_logic::validate_identifier_not_empty(&user_identifier, "User identifier")?;
    crypto_logic::validate_crypto_amount_positive(crypto_amount)?;
    transfer_logic::validate_identifier_not_empty(&agent_id, "Agent ID")?;
    
    // 1. Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // 1.5. Validate agent exists
    let _agent = data_client::get_user(&agent_id).await?
        .ok_or_else(|| format!("Agent not found: {}", agent_id))?;
    
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
    
    crypto_logic::validate_sufficient_crypto_balance(current_balance, crypto_amount)?;
    
    // 4. Rate limiting
    if !fraud_detection::check_rate_limit(&user.id)? {
        return Err("Too many transactions. Please wait before trying again.".to_string());
    }
    
    // 5. Generate escrow code (6-digit)
    let timestamp = ic_cdk::api::time();
    let code_prefix = match crypto_type {
        CryptoType::CkBTC => "BTC",
        CryptoType::CkUSDC => "USD",
    };
    let escrow_code = format!("{}-{:06}", code_prefix, (timestamp % 1_000_000) as u32);
    
    // 6. Create escrow record
    let escrow = shared_types::Escrow {
        code: escrow_code.clone(),
        user_id: user.id.clone(),
        agent_id: agent_id.clone(),
        amount: crypto_amount,
        crypto_type,
        status: shared_types::EscrowStatus::Active,
        created_at: timestamp,
        expires_at: timestamp + (24 * 60 * 60 * 1_000_000_000), // 24 hours in nanoseconds
        claimed_at: None,
    };
    
    // 7. Store escrow FIRST (before deducting crypto for atomicity)
    data_client::store_escrow(escrow).await?;
    
    // 8. Put crypto in escrow (deduct from user balance)
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (-(crypto_amount as i64), 0),
        CryptoType::CkUSDC => (0, -(crypto_amount as i64)),
    };
    data_client::update_crypto_balance(&user.id, ckbtc_delta, ckusdc_delta).await?;
    
    // 9. Create escrow transaction record
    let tx_id = transfer_logic::generate_transaction_id(timestamp);
    let tx = shared_types::Transaction {
        id: tx_id.clone(),
        transaction_type: shared_types::TransactionType::SellCrypto,
        from_user: Some(user.id.clone()),
        to_user: Some(agent_id.clone()),
        amount: crypto_amount,
        currency_type: shared_types::CurrencyType::Crypto(crypto_type),
        description: Some("Escrow pending agent confirmation".to_string()),
        created_at: ic_cdk::api::time(),
        completed_at: None,
        status: shared_types::TransactionStatus::Pending, // Pending until agent confirms
    };
    data_client::store_transaction(&tx).await?;
    
    // 8. Update last active
    let _ = data_client::update_last_active(&user.id).await;
    
    ic_cdk::println!("✅ Escrow created: {} {} → Agent {}, Code: {}", 
        crypto_amount, format!("{:?}", crypto_type), agent_id, escrow_code);
    
    Ok(TransactionResult {
        transaction_id: escrow_code, // Return escrow code as transaction ID
        from_user: user.id.clone(),
        to_user: agent_id,
        amount: crypto_amount,
        currency: format!("{:?}", crypto_type),
        new_balance: current_balance - crypto_amount,
        timestamp: ic_cdk::api::time(),
    })
}

/// Get estimated fiat value for crypto amount (for display purposes)
pub async fn get_crypto_value_estimate(
    crypto_amount: u64,
    crypto_type: CryptoType,
    fiat_currency: String,
) -> Result<u64, String> {
    let crypto_type_str = format!("{:?}", crypto_type);
    exchange_rate::calculate_fiat_from_crypto(
        crypto_amount,
        &crypto_type_str,
        &fiat_currency
    ).await
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

