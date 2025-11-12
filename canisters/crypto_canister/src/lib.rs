use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::{time, caller};

mod logic;
mod services;
mod config;

use shared_types::{
    FiatCurrency, CryptoType, Transaction, TransactionType, TransactionStatus,
    CurrencyType, Escrow, EscrowStatus, audit,
};

/// Buy crypto request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BuyCryptoRequest {
    pub user_identifier: String,
    pub fiat_amount: u64,
    pub currency: String,
    pub crypto_type: String,
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

/// Buy crypto response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BuyCryptoResponse {
    pub transaction_id: String,
    pub crypto_amount: u64,
    pub fiat_amount: u64,
    pub crypto_type: String,
    pub exchange_rate: f64,
    pub timestamp: u64,
}

/// Sell crypto request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SellCryptoRequest {
    pub user_identifier: String,
    pub crypto_amount: u64,
    pub currency: String,
    pub crypto_type: String,
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

/// Send crypto request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SendCryptoRequest {
    pub user_identifier: String,
    pub to_address: String,
    pub amount: u64,
    pub crypto_type: String,
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

/// Swap crypto request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SwapCryptoRequest {
    pub user_identifier: String,
    pub from_crypto: String,
    pub to_crypto: String,
    pub amount: u64,
    pub pin: String,
}

/// Swap crypto response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SwapCryptoResponse {
    pub transaction_id: String,
    pub from_amount: u64,
    pub to_amount: u64,
    pub spread_amount: u64,
    pub exchange_rate: f64,
    pub timestamp: u64,
}

/// Create escrow request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowRequest {
    pub user_identifier: String,
    pub agent_id: String,
    pub amount: u64,
    pub crypto_type: String,
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

/// Create escrow response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowResponse {
    pub code: String,
    pub amount: u64,
    pub crypto_type: String,
    pub expires_at: u64,
}

/// Verify escrow request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct VerifyEscrowRequest {
    pub code: String,
    pub agent_id: String,
    pub pin: String,
}

/// Cleanup result
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CleanupResult {
    pub escrows_processed: u32,
    pub escrows_refunded: u32,
    pub total_refunded_btc: u64,
    pub total_refunded_usdc: u64,
}

// ============================================================================
// CANISTER INITIALIZATION & TIMERS
// ============================================================================

#[init]
fn init() {
    config::init_config();
    
    // Set up periodic escrow cleanup timer (runs every hour)
    ic_cdk_timers::set_timer_interval(
        std::time::Duration::from_secs(3600), // 1 hour
        || {
            ic_cdk::spawn(async {
                match cleanup_expired_escrows().await {
                    Ok(result) => {
                        ic_cdk::println!("⏰ Periodic cleanup: processed={}, refunded={}",
                            result.escrows_processed, result.escrows_refunded);
                    }
                    Err(e) => {
                        ic_cdk::println!("❌ Periodic cleanup failed: {}", e);
                    }
                }
            });
        }
    );
    
    ic_cdk::println!("✅ Crypto Canister initialized with periodic cleanup timer");
}

// ============================================================================
// ADMIN ENDPOINTS
// ============================================================================

/// Set data canister ID (admin only)
#[update]
fn set_data_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) {
        return Err("Only controller can set data canister ID".to_string());
    }
    config::set_data_canister_id(principal);
    Ok(())
}

/// Set user canister ID (admin only)
#[update]
fn set_user_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) {
        return Err("Only controller can set user canister ID".to_string());
    }
    config::set_user_canister_id(principal);
    Ok(())
}

/// Set wallet canister ID (admin only)
#[update]
fn set_wallet_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) {
        return Err("Only controller can set wallet canister ID".to_string());
    }
    config::set_wallet_canister_id(principal);
    Ok(())
}

/// Add authorized canister (admin only)
#[update]
fn add_authorized_canister(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) {
        return Err("Only controller can add authorized canisters".to_string());
    }
    config::add_authorized_canister(principal);
    Ok(())
}

/// Enable test mode (admin only)
#[update]
fn enable_test_mode() -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) {
        return Err("Only controller can enable test mode".to_string());
    }
    config::enable_test_mode();
    Ok(())
}

/// Disable test mode (admin only)
#[update]
fn disable_test_mode() -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) {
        return Err("Only controller can disable test mode".to_string());
    }
    config::disable_test_mode();
    Ok(())
}

// ============================================================================
// CRYPTO PURCHASE/SALE ENDPOINTS
// ============================================================================

/// Buy cryptocurrency with fiat
#[update]
async fn buy_crypto(request: BuyCryptoRequest) -> Result<BuyCryptoResponse, String> {
    
    // Validate inputs
    logic::crypto_logic::validate_fiat_amount_for_crypto(request.fiat_amount)?;
    
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    let fiat_currency = FiatCurrency::from_code(&request.currency)
        .ok_or_else(|| format!("Invalid currency code: {}", request.currency))?;
    
    // 1. Verify user exists
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Check PIN attempts allowed (exponential backoff)
    logic::fraud_detection::check_pin_attempts_allowed(&request.user_identifier)?;
    
    // 3. Verify PIN
    let verified = services::user_client::verify_pin(&request.user_identifier, &request.pin).await?;
    if !verified {
        logic::fraud_detection::record_failed_pin_attempt(&request.user_identifier)?;
        audit::log_failure(
            "failed_pin_buy_crypto",
            Some(request.user_identifier.clone()),
            format!("Invalid PIN | Amount: {} {} | Device: {:?} | Location: {:?}",
                request.fiat_amount, request.currency, request.device_fingerprint, request.geo_location)
        );
        return Err("Invalid PIN".to_string());
    }
    logic::fraud_detection::reset_pin_attempts(&request.user_identifier);
    
    // 4. Check operation rate limit
    if !logic::fraud_detection::check_operation_rate_limit(&request.user_identifier, "buy_crypto")? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(request.user_identifier.clone()),
            format!("Operation: buy_crypto | Amount: {} {}", request.fiat_amount, request.currency)
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }
    
    // 5. Comprehensive fraud check
    let fraud_check = logic::fraud_detection::check_transaction(
        &request.user_identifier,
        request.fiat_amount,
        &request.currency,
        "buy_crypto",
        request.device_fingerprint.as_deref(),
        request.geo_location.as_deref(),
    )?;
    
    if fraud_check.should_block {
        audit::log_failure(
            "transaction_blocked",
            Some(request.user_identifier.clone()),
            format!("Operation: buy_crypto | Amount: {} {} | Risk Score: {} | Warnings: {:?} | Device: {:?} | Location: {:?}",
                request.fiat_amount, request.currency, fraud_check.risk_score, fraud_check.warnings,
                request.device_fingerprint, request.geo_location)
        );
        return Err(format!("Transaction blocked due to security concerns: {:?}", fraud_check.warnings));
    }
    
    if fraud_check.requires_manual_review {
        audit::log_success(
            "manual_review_required",
            Some(request.user_identifier.clone()),
            format!("Operation: buy_crypto | Amount: {} {} | Risk Score: {} | Warnings: {:?}",
                request.fiat_amount, request.currency, fraud_check.risk_score, fraud_check.warnings)
        );
    }
    
    // 6. Record device and location
    if let Some(fingerprint) = &request.device_fingerprint {
        logic::fraud_detection::record_device_fingerprint(&request.user_identifier, fingerprint)?;
        audit::log_success(
            "device_recorded",
            Some(request.user_identifier.clone()),
            format!("Device: {}", fingerprint)
        );
    }
    if let Some(location) = &request.geo_location {
        logic::fraud_detection::record_geo_location(&request.user_identifier, location)?;
        audit::log_success(
            "location_recorded",
            Some(request.user_identifier.clone()),
            format!("Location: {}", location)
        );
    }
    
    // 3. Check fiat balance
    let fiat_balance = services::wallet_client::get_fiat_balance(&request.user_identifier, fiat_currency).await?;
    if fiat_balance < request.fiat_amount {
        return Err(format!("Insufficient fiat balance. Have: {}, Need: {}", fiat_balance, request.fiat_amount));
    }
    
    // 4. Calculate crypto amount using real exchange rates
    let crypto_type_str = format!("{:?}", crypto_type);
    let crypto_amount = services::exchange_rate::calculate_crypto_from_fiat(
        request.fiat_amount,
        &request.currency,
        &crypto_type_str
    ).await?;
    
    // 5. Calculate exchange rate for display
    let exchange_rate = if request.fiat_amount > 0 {
        crypto_amount as f64 / request.fiat_amount as f64
    } else {
        0.0
    };
    
    // 6. Deduct fiat from wallet
    let new_fiat_balance = fiat_balance.checked_sub(request.fiat_amount)
        .ok_or("Fiat balance calculation would underflow")?;
    services::wallet_client::set_fiat_balance(&request.user_identifier, fiat_currency, new_fiat_balance).await?;
    
    // 7. Add crypto to user's balance
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (crypto_amount as i64, 0),
        CryptoType::CkUSDC => (0, crypto_amount as i64),
    };
    services::data_client::update_crypto_balance(&request.user_identifier, ckbtc_delta, ckusdc_delta).await?;
    
    // 8. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("buy-crypto-{}-{}", request.user_identifier, timestamp),
        transaction_type: TransactionType::BuyCrypto,
        from_user: Some(request.user_identifier.clone()),
        to_user: Some(request.user_identifier.clone()),
        amount: crypto_amount,
        currency_type: CurrencyType::Crypto(crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Bought {} {} for {} {}", 
            crypto_amount, crypto_type_str, request.fiat_amount, request.currency)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 9. Record transaction for velocity tracking
    logic::fraud_detection::record_transaction(
        &request.user_identifier,
        request.fiat_amount,
        &request.currency,
        "buy_crypto",
    )?;
    
    // 10. Audit successful transaction
    audit::log_success(
        "buy_crypto_completed",
        Some(request.user_identifier.clone()),
        format!("Bought {} {} for {} {} | Exchange Rate: {} | TX: {}",
            crypto_amount, crypto_type_str, request.fiat_amount, request.currency,
            exchange_rate, transaction.id)
    );
    
    Ok(BuyCryptoResponse {
        transaction_id: transaction.id,
        crypto_amount,
        fiat_amount: request.fiat_amount,
        crypto_type: crypto_type_str,
        exchange_rate,
        timestamp,
    })
}

/// Sell cryptocurrency for fiat
#[update]
async fn sell_crypto(request: SellCryptoRequest) -> Result<BuyCryptoResponse, String> {
    
    // Validate inputs
    logic::crypto_logic::validate_crypto_amount_positive(request.crypto_amount)?;
    
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    let fiat_currency = FiatCurrency::from_code(&request.currency)
        .ok_or_else(|| format!("Invalid currency code: {}", request.currency))?;
    
    // 1. Verify user exists
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Check PIN attempts allowed (exponential backoff)
    logic::fraud_detection::check_pin_attempts_allowed(&request.user_identifier)?;
    
    // 3. Verify PIN
    let verified = services::user_client::verify_pin(&request.user_identifier, &request.pin).await?;
    if !verified {
        logic::fraud_detection::record_failed_pin_attempt(&request.user_identifier)?;
        audit::log_failure(
            "failed_pin_sell_crypto",
            Some(request.user_identifier.clone()),
            format!("Invalid PIN | Amount: {} {} | Device: {:?} | Location: {:?}",
                request.crypto_amount, request.crypto_type, request.device_fingerprint, request.geo_location)
        );
        return Err("Invalid PIN".to_string());
    }
    logic::fraud_detection::reset_pin_attempts(&request.user_identifier);
    
    // 4. Check operation rate limit
    if !logic::fraud_detection::check_operation_rate_limit(&request.user_identifier, "sell_crypto")? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(request.user_identifier.clone()),
            format!("Operation: sell_crypto | Amount: {} {}", request.crypto_amount, request.crypto_type)
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }
    
    // 5. Check crypto balance
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&request.user_identifier).await?;
    let crypto_balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    
    logic::crypto_logic::validate_sufficient_crypto_balance(crypto_balance, request.crypto_amount)?;
    
    // 4. Calculate fiat amount using real exchange rates
    let crypto_type_str = format!("{:?}", crypto_type);
    let fiat_amount = services::exchange_rate::calculate_fiat_from_crypto(
        request.crypto_amount,
        &crypto_type_str,
        &request.currency
    ).await?;
    
    // 5. Calculate exchange rate for display
    let exchange_rate = if fiat_amount > 0 {
        request.crypto_amount as f64 / fiat_amount as f64
    } else {
        0.0
    };
    
    // 6. Comprehensive fraud check
    let fraud_check = logic::fraud_detection::check_transaction(
        &request.user_identifier,
        fiat_amount,
        &request.currency,
        "sell_crypto",
        request.device_fingerprint.as_deref(),
        request.geo_location.as_deref(),
    )?;
    
    if fraud_check.should_block {
        audit::log_failure(
            "transaction_blocked",
            Some(request.user_identifier.clone()),
            format!("Operation: sell_crypto | Amount: {} {} | Risk Score: {} | Warnings: {:?} | Device: {:?} | Location: {:?}",
                fiat_amount, request.currency, fraud_check.risk_score, fraud_check.warnings,
                request.device_fingerprint, request.geo_location)
        );
        return Err(format!("Transaction blocked due to security concerns: {:?}", fraud_check.warnings));
    }
    
    if fraud_check.requires_manual_review {
        audit::log_success(
            "manual_review_required",
            Some(request.user_identifier.clone()),
            format!("Operation: sell_crypto | Amount: {} {} | Risk Score: {} | Warnings: {:?}",
                fiat_amount, request.currency, fraud_check.risk_score, fraud_check.warnings)
        );
    }
    
    // 7. Record device and location
    if let Some(fingerprint) = &request.device_fingerprint {
        logic::fraud_detection::record_device_fingerprint(&request.user_identifier, fingerprint)?;
        audit::log_success(
            "device_recorded",
            Some(request.user_identifier.clone()),
            format!("Device: {}", fingerprint)
        );
    }
    if let Some(location) = &request.geo_location {
        logic::fraud_detection::record_geo_location(&request.user_identifier, location)?;
        audit::log_success(
            "location_recorded",
            Some(request.user_identifier.clone()),
            format!("Location: {}", location)
        );
    }
    
    // 8. Deduct crypto from user's balance
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (-(request.crypto_amount as i64), 0),
        CryptoType::CkUSDC => (0, -(request.crypto_amount as i64)),
    };
    services::data_client::update_crypto_balance(&request.user_identifier, ckbtc_delta, ckusdc_delta).await?;
    
    // 7. Add fiat to wallet
    let current_fiat_balance = services::wallet_client::get_fiat_balance(&request.user_identifier, fiat_currency).await?;
    let new_fiat_balance = current_fiat_balance.checked_add(fiat_amount)
        .ok_or("Fiat balance calculation would overflow")?;
    services::wallet_client::set_fiat_balance(&request.user_identifier, fiat_currency, new_fiat_balance).await?;
    
    // 8. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("sell-crypto-{}-{}", request.user_identifier, timestamp),
        transaction_type: TransactionType::SellCrypto,
        from_user: Some(request.user_identifier.clone()),
        to_user: Some(request.user_identifier.clone()),
        amount: request.crypto_amount,
        currency_type: CurrencyType::Crypto(crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Sold {} {} for {} {}", 
            request.crypto_amount, crypto_type_str, fiat_amount, request.currency)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 9. Record transaction for velocity tracking
    logic::fraud_detection::record_transaction(
        &request.user_identifier,
        fiat_amount,
        &request.currency,
        "sell_crypto",
    )?;
    
    // 10. Audit successful transaction
    audit::log_success(
        "sell_crypto_completed",
        Some(request.user_identifier.clone()),
        format!("Sold {} {} for {} {} | Exchange Rate: {} | TX: {}",
            request.crypto_amount, crypto_type_str, fiat_amount, request.currency,
            exchange_rate, transaction.id)
    );
    
    Ok(BuyCryptoResponse {
        transaction_id: transaction.id,
        crypto_amount: request.crypto_amount,
        fiat_amount,
        crypto_type: crypto_type_str,
        exchange_rate,
        timestamp,
    })
}

// ============================================================================
// CRYPTO TRANSFER ENDPOINTS
// ============================================================================

/// Send cryptocurrency to external address
#[update]
async fn send_crypto(request: SendCryptoRequest) -> Result<String, String> {
    
    // Validate inputs
    logic::crypto_logic::validate_crypto_amount_positive(request.amount)?;
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    let crypto_type_str = format!("{:?}", crypto_type);
    logic::crypto_logic::validate_crypto_address(&request.to_address, &crypto_type_str)?;
    
    // 1. Verify user exists
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Check PIN attempts allowed (exponential backoff)
    logic::fraud_detection::check_pin_attempts_allowed(&request.user_identifier)?;
    
    // 3. Verify PIN
    let verified = services::user_client::verify_pin(&request.user_identifier, &request.pin).await?;
    if !verified {
        logic::fraud_detection::record_failed_pin_attempt(&request.user_identifier)?;
        audit::log_failure(
            "failed_pin_send_crypto",
            Some(request.user_identifier.clone()),
            format!("Invalid PIN | Amount: {} {} | To: {} | Device: {:?} | Location: {:?}",
                request.amount, request.crypto_type, request.to_address, request.device_fingerprint, request.geo_location)
        );
        return Err("Invalid PIN".to_string());
    }
    logic::fraud_detection::reset_pin_attempts(&request.user_identifier);
    
    // 4. Check operation rate limit
    if !logic::fraud_detection::check_operation_rate_limit(&request.user_identifier, "send_crypto")? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(request.user_identifier.clone()),
            format!("Operation: send_crypto | Amount: {} {} | To: {}", request.amount, request.crypto_type, request.to_address)
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }
    
    // 5. Comprehensive fraud check
    let fraud_check = logic::fraud_detection::check_transaction(
        &request.user_identifier,
        request.amount,
        "USD",
        "send_crypto",
        request.device_fingerprint.as_deref(),
        request.geo_location.as_deref(),
    )?;
    
    if fraud_check.should_block {
        audit::log_failure(
            "transaction_blocked",
            Some(request.user_identifier.clone()),
            format!("Operation: send_crypto | Amount: {} {} | To: {} | Risk Score: {} | Warnings: {:?} | Device: {:?} | Location: {:?}",
                request.amount, request.crypto_type, request.to_address, fraud_check.risk_score, fraud_check.warnings,
                request.device_fingerprint, request.geo_location)
        );
        return Err(format!("Transaction blocked due to security concerns: {:?}", fraud_check.warnings));
    }
    
    if fraud_check.requires_manual_review {
        audit::log_success(
            "manual_review_required",
            Some(request.user_identifier.clone()),
            format!("Operation: send_crypto | Amount: {} {} | To: {} | Risk Score: {} | Warnings: {:?}",
                request.amount, request.crypto_type, request.to_address, fraud_check.risk_score, fraud_check.warnings)
        );
    }
    
    // 6. Record device and location
    if let Some(fingerprint) = &request.device_fingerprint {
        logic::fraud_detection::record_device_fingerprint(&request.user_identifier, fingerprint)?;
        audit::log_success(
            "device_recorded",
            Some(request.user_identifier.clone()),
            format!("Device: {}", fingerprint)
        );
    }
    if let Some(location) = &request.geo_location {
        logic::fraud_detection::record_geo_location(&request.user_identifier, location)?;
        audit::log_success(
            "location_recorded",
            Some(request.user_identifier.clone()),
            format!("Location: {}", location)
        );
    }
    
    // 7. Check crypto balance
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&request.user_identifier).await?;
    let crypto_balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    
    logic::crypto_logic::validate_sufficient_crypto_balance(crypto_balance, request.amount)?;
    
    // 4. Deduct crypto from user's balance
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (-(request.amount as i64), 0),
        CryptoType::CkUSDC => (0, -(request.amount as i64)),
    };
    services::data_client::update_crypto_balance(&request.user_identifier, ckbtc_delta, ckusdc_delta).await?;
    
    // 5. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("send-crypto-{}-{}", request.user_identifier, timestamp),
        transaction_type: TransactionType::TransferCrypto,
        from_user: Some(request.user_identifier.clone()),
        to_user: Some(request.to_address.clone()),
        amount: request.amount,
        currency_type: CurrencyType::Crypto(crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Sent {} {} to {}", 
            request.amount, crypto_type_str, request.to_address)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 6. Record transaction for velocity tracking
    logic::fraud_detection::record_transaction(
        &request.user_identifier,
        request.amount,
        "USD",
        "send_crypto",
    )?;
    
    // 7. Audit successful transaction
    audit::log_success(
        "send_crypto_completed",
        Some(request.user_identifier.clone()),
        format!("Sent {} {} to {} | TX: {}",
            request.amount, crypto_type_str, request.to_address, transaction.id)
    );
    
    Ok(transaction.id)
}

/// Check crypto balance
#[update]
async fn check_crypto_balance(user_identifier: String, crypto_type: String) -> Result<u64, String> {
    let crypto_type = parse_crypto_type(&crypto_type)?;
    
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&user_identifier).await?;
    
    let balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    
    Ok(balance)
}

// ============================================================================
// CRYPTO SWAP ENDPOINTS
// ============================================================================

/// Swap between cryptocurrencies
#[update]
async fn swap_crypto(request: SwapCryptoRequest) -> Result<SwapCryptoResponse, String> {
    
    // Validate inputs
    logic::crypto_logic::validate_crypto_amount_positive(request.amount)?;
    
    let from_crypto = parse_crypto_type(&request.from_crypto)?;
    let to_crypto = parse_crypto_type(&request.to_crypto)?;
    let from_crypto_str = format!("{:?}", from_crypto);
    let to_crypto_str = format!("{:?}", to_crypto);
    
    if from_crypto == to_crypto {
        return Err("Cannot swap same cryptocurrency".to_string());
    }
    
    // 1. Verify user exists
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Verify PIN
    let verified = services::user_client::verify_pin(&request.user_identifier, &request.pin).await?;
    if !verified {
        audit::log_failure(
            "failed_pin_swap_crypto",
            Some(request.user_identifier.clone()),
            format!("Invalid PIN | Swap: {} {} to {} {}", request.amount, request.from_crypto, request.amount, request.to_crypto)
        );
        return Err("Invalid PIN".to_string());
    }
    
    // 3. Check balance
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&request.user_identifier).await?;
    let from_balance = match from_crypto {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    
    logic::crypto_logic::validate_sufficient_crypto_balance(from_balance, request.amount)?;
    
    // 4. Calculate spread
    let spread_bp = config::get_spread_basis_points();
    let spread_amount = (request.amount * spread_bp) / 10000;
    let swap_amount = request.amount - spread_amount;
    
    // 5. Perform swap via DEX
    let to_amount = services::dex_client::swap_tokens(from_crypto, to_crypto, swap_amount, 0).await?;
    
    // 6. Calculate exchange rate
    let exchange_rate = if request.amount > 0 {
        to_amount as f64 / request.amount as f64
    } else {
        0.0
    };
    
    // 7. Update balances
    let (from_delta_btc, from_delta_usdc) = match from_crypto {
        CryptoType::CkBTC => (-(request.amount as i64), 0),
        CryptoType::CkUSDC => (0, -(request.amount as i64)),
    };
    
    let (to_delta_btc, to_delta_usdc) = match to_crypto {
        CryptoType::CkBTC => (to_amount as i64, 0),
        CryptoType::CkUSDC => (0, to_amount as i64),
    };
    
    let total_btc_delta = from_delta_btc + to_delta_btc;
    let total_usdc_delta = from_delta_usdc + to_delta_usdc;
    
    services::data_client::update_crypto_balance(&request.user_identifier, total_btc_delta, total_usdc_delta).await?;
    
    // 8. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("swap-crypto-{}-{}", request.user_identifier, timestamp),
        transaction_type: TransactionType::SwapCrypto,
        from_user: Some(request.user_identifier.clone()),
        to_user: Some(request.user_identifier.clone()),
        amount: request.amount,
        currency_type: CurrencyType::Crypto(from_crypto),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Swapped {} {} to {} {} (spread: {})", 
            request.amount, request.from_crypto, to_amount, request.to_crypto, spread_amount)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 9. Audit successful swap
    audit::log_success(
        "swap_crypto_completed",
        Some(request.user_identifier.clone()),
        format!("Swapped {} {} to {} {} | Exchange Rate: {} | Spread: {} | TX: {}",
            request.amount, from_crypto_str, to_amount, to_crypto_str, exchange_rate, spread_amount, transaction.id)
    );
    
    Ok(SwapCryptoResponse {
        transaction_id: transaction.id,
        from_amount: request.amount,
        to_amount,
        spread_amount,
        exchange_rate,
        timestamp,
    })
}

// ============================================================================
// ESCROW ENDPOINTS
// ============================================================================

/// Create escrow for crypto-to-cash transaction
#[update]
async fn create_escrow(request: CreateEscrowRequest) -> Result<CreateEscrowResponse, String> {
    
    // Validate inputs
    logic::escrow_logic::validate_escrow_amount(request.amount)?;
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    
    // 1. Verify user exists
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Check PIN attempts allowed (exponential backoff)
    logic::fraud_detection::check_pin_attempts_allowed(&request.user_identifier)?;
    
    // 3. Verify PIN
    let verified = services::user_client::verify_pin(&request.user_identifier, &request.pin).await?;
    if !verified {
        logic::fraud_detection::record_failed_pin_attempt(&request.user_identifier)?;
        audit::log_failure(
            "failed_pin_create_escrow",
            Some(request.user_identifier.clone()),
            format!("Invalid PIN | Amount: {} {} | Agent: {} | Device: {:?} | Location: {:?}",
                request.amount, request.crypto_type, request.agent_id, request.device_fingerprint, request.geo_location)
        );
        return Err("Invalid PIN".to_string());
    }
    logic::fraud_detection::reset_pin_attempts(&request.user_identifier);
    
    // 4. Check operation rate limit
    if !logic::fraud_detection::check_operation_rate_limit(&request.user_identifier, "create_escrow")? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(request.user_identifier.clone()),
            format!("Operation: create_escrow | Amount: {} {} | Agent: {}", request.amount, request.crypto_type, request.agent_id)
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }
    
    // 5. Comprehensive fraud check
    let fraud_check = logic::fraud_detection::check_transaction(
        &request.user_identifier,
        request.amount,
        "USD",
        "create_escrow",
        request.device_fingerprint.as_deref(),
        request.geo_location.as_deref(),
    )?;
    
    if fraud_check.should_block {
        audit::log_failure(
            "transaction_blocked",
            Some(request.user_identifier.clone()),
            format!("Operation: create_escrow | Amount: {} {} | Agent: {} | Risk Score: {} | Warnings: {:?} | Device: {:?} | Location: {:?}",
                request.amount, request.crypto_type, request.agent_id, fraud_check.risk_score, fraud_check.warnings,
                request.device_fingerprint, request.geo_location)
        );
        return Err(format!("Transaction blocked due to security concerns: {:?}", fraud_check.warnings));
    }
    
    if fraud_check.requires_manual_review {
        audit::log_success(
            "manual_review_required",
            Some(request.user_identifier.clone()),
            format!("Operation: create_escrow | Amount: {} {} | Agent: {} | Risk Score: {} | Warnings: {:?}",
                request.amount, request.crypto_type, request.agent_id, fraud_check.risk_score, fraud_check.warnings)
        );
    }
    
    // 6. Record device and location
    if let Some(fingerprint) = &request.device_fingerprint {
        logic::fraud_detection::record_device_fingerprint(&request.user_identifier, fingerprint)?;
        audit::log_success(
            "device_recorded",
            Some(request.user_identifier.clone()),
            format!("Device: {}", fingerprint)
        );
    }
    if let Some(location) = &request.geo_location {
        logic::fraud_detection::record_geo_location(&request.user_identifier, location)?;
        audit::log_success(
            "location_recorded",
            Some(request.user_identifier.clone()),
            format!("Location: {}", location)
        );
    }
    
    // 7. Check crypto balance
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&request.user_identifier).await?;
    let crypto_balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    
    logic::crypto_logic::validate_sufficient_crypto_balance(crypto_balance, request.amount)?;
    
    // 4. Generate escrow code
    let timestamp = time();
    let code = logic::escrow_logic::generate_escrow_code(timestamp, &request.user_identifier);
    
    // 5. Calculate expiration
    let expiration_ns = config::get_escrow_expiration_ns();
    let expires_at = logic::escrow_logic::calculate_expiration_time(timestamp, expiration_ns)?;
    
    // 6. Deduct crypto from user's balance
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_creation_delta(request.amount, crypto_type);
    services::data_client::update_crypto_balance(&request.user_identifier, ckbtc_delta, ckusdc_delta).await?;
    
    // 7. Create escrow in data canister
    let escrow = Escrow {
        code: code.clone(),
        user_id: request.user_identifier.clone(),
        agent_id: request.agent_id.clone(),
        amount: request.amount,
        crypto_type,
        status: EscrowStatus::Active,
        created_at: timestamp,
        expires_at,
        claimed_at: None,
    };
    
    services::data_client::create_escrow(&escrow).await?;
    
    // 8. Record transaction for velocity tracking
    logic::fraud_detection::record_transaction(
        &request.user_identifier,
        request.amount,
        "USD",
        "create_escrow",
    )?;
    
    // 9. Audit successful escrow creation
    audit::log_success(
        "escrow_created",
        Some(request.user_identifier.clone()),
        format!("Created escrow {} | Amount: {} {} | Agent: {} | Expires: {}",
            code, request.amount, request.crypto_type, request.agent_id, expires_at)
    );
    
    Ok(CreateEscrowResponse {
        code,
        amount: request.amount,
        crypto_type: request.crypto_type,
        expires_at,
    })
}

/// Verify and claim escrow
#[update]
async fn verify_escrow(request: VerifyEscrowRequest) -> Result<String, String> {
    
    // 1. Get escrow
    let escrow = services::data_client::get_escrow(&request.code).await?;
    
    // 2. Validate escrow
    logic::escrow_logic::validate_escrow_active(escrow.status)?;
    logic::escrow_logic::validate_escrow_not_expired(time(), escrow.expires_at)?;
    logic::escrow_logic::validate_agent_authorized(&escrow.agent_id, &request.agent_id)?;
    
    // 3. Verify agent PIN
    let verified = services::user_client::verify_pin(&request.agent_id, &request.pin).await?;
    if !verified {
        audit::log_failure(
            "failed_pin_verify_escrow",
            Some(request.agent_id.clone()),
            format!("Invalid PIN | Escrow: {} | User: {}", request.code, escrow.user_id)
        );
        return Err("Invalid PIN".to_string());
    }
    
    // 4. Transfer crypto to agent
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_claim_delta(escrow.amount, escrow.crypto_type);
    services::data_client::update_crypto_balance(&request.agent_id, ckbtc_delta, ckusdc_delta).await?;
    
    // 5. Update escrow status
    services::data_client::update_escrow_status(&request.code, EscrowStatus::Claimed).await?;
    
    // 6. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("escrow-claim-{}-{}", request.code, timestamp),
        transaction_type: TransactionType::EscrowClaim,
        from_user: Some(escrow.user_id.clone()),
        to_user: Some(request.agent_id.clone()),
        amount: escrow.amount,
        currency_type: CurrencyType::Crypto(escrow.crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Escrow {} claimed by agent", request.code)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 7. Audit successful escrow claim
    audit::log_success(
        "escrow_claimed",
        Some(request.agent_id.clone()),
        format!("Claimed escrow {} | Amount: {} {:?} | From User: {} | TX: {}",
            request.code, escrow.amount, escrow.crypto_type, escrow.user_id, transaction.id)
    );
    
    Ok(transaction.id)
}

/// Cancel escrow (user only)
#[update]
async fn cancel_escrow(code: String, user_id: String, pin: String) -> Result<(), String> {
    
    // 1. Get escrow
    let escrow = services::data_client::get_escrow(&code).await?;
    
    // 2. Validate ownership
    logic::escrow_logic::validate_user_owns_escrow(&escrow.user_id, &user_id)?;
    logic::escrow_logic::validate_escrow_active(escrow.status)?;
    
    // 3. Verify PIN
    let verified = services::user_client::verify_pin(&user_id, &pin).await?;
    if !verified {
        audit::log_failure(
            "failed_pin_cancel_escrow",
            Some(user_id.clone()),
            format!("Invalid PIN | Escrow: {} | Amount: {} {:?}", code, escrow.amount, escrow.crypto_type)
        );
        return Err("Invalid PIN".to_string());
    }
    
    // 4. Refund crypto to user
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_refund_delta(escrow.amount, escrow.crypto_type);
    services::data_client::update_crypto_balance(&user_id, ckbtc_delta, ckusdc_delta).await?;
    
    // 5. Update escrow status
    services::data_client::update_escrow_status(&code, EscrowStatus::Cancelled).await?;
    
    // 6. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("escrow-cancel-{}-{}", code, timestamp),
        transaction_type: TransactionType::EscrowCancel,
        from_user: Some(escrow.user_id.clone()),
        to_user: Some(user_id.clone()),
        amount: escrow.amount,
        currency_type: CurrencyType::Crypto(escrow.crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Escrow {} cancelled by user", code)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 7. Audit successful escrow cancellation
    audit::log_success(
        "escrow_cancelled",
        Some(user_id.clone()),
        format!("Cancelled escrow {} | Amount: {} {:?} | TX: {}",
            code, escrow.amount, escrow.crypto_type, transaction.id)
    );
    
    Ok(())
}

/// Get escrow status
#[update]
async fn get_escrow_status(code: String) -> Result<Escrow, String> {
    services::data_client::get_escrow(&code).await
}

/// Cleanup expired escrows (periodic job)
#[update]
async fn cleanup_expired_escrows() -> Result<CleanupResult, String> {
    let now = time();
    
    // Audit cleanup start
    audit::log_success(
        "cleanup_started",
        None,
        format!("Starting expired escrow cleanup at {}", now)
    );
    
    // Get all active escrows from data_canister
    let active_escrows = services::data_client::get_active_escrows().await?;
    
    let mut escrows_processed = 0u32;
    let mut escrows_refunded = 0u32;
    let mut total_refunded_btc = 0u64;
    let mut total_refunded_usdc = 0u64;
    
    for escrow in active_escrows {
        escrows_processed += 1;
        
        // Check if expired
        if logic::escrow_logic::is_escrow_expired(now, escrow.expires_at) {
            // Calculate refund delta
            let (btc_delta, usdc_delta) = logic::escrow_logic::calculate_escrow_refund_delta(
                escrow.amount,
                escrow.crypto_type,
            );
            
            // Refund crypto to user
            services::data_client::update_crypto_balance(
                &escrow.user_id,
                btc_delta,
                usdc_delta,
            ).await?;
            
            // Update escrow status to Expired
            services::data_client::update_escrow_status(&escrow.code, EscrowStatus::Expired).await?;
            
            // Record transaction
            let transaction = Transaction {
                id: format!("refund-{}-{}", escrow.code, now),
                transaction_type: TransactionType::EscrowCancel,
                from_user: None,
                to_user: Some(escrow.user_id.clone()),
                amount: escrow.amount,
                currency_type: CurrencyType::Crypto(escrow.crypto_type),
                status: TransactionStatus::Completed,
                created_at: now,
                completed_at: Some(now),
                description: Some(format!("Automatic refund of expired escrow {}", escrow.code)),
            };
            
            services::data_client::store_transaction(&transaction).await?;
            
            // Audit each escrow refund
            audit::log_success(
                "escrow_refunded",
                Some(escrow.user_id.clone()),
                format!("Auto-refunded expired escrow {} | Amount: {} {:?} | TX: {}",
                    escrow.code, escrow.amount, escrow.crypto_type, transaction.id)
            );
            
            escrows_refunded += 1;
            match escrow.crypto_type {
                CryptoType::CkBTC => total_refunded_btc += escrow.amount,
                CryptoType::CkUSDC => total_refunded_usdc += escrow.amount,
            }
            
            ic_cdk::println!("🔄 Refunded expired escrow: code={}, user={}, amount={} {:?}",
                escrow.code, escrow.user_id, escrow.amount, escrow.crypto_type);
        }
    }
    
    ic_cdk::println!("✅ Cleanup complete: processed={}, refunded={}, BTC={}, USDC={}",
        escrows_processed, escrows_refunded, total_refunded_btc, total_refunded_usdc);
    
    // Audit cleanup completion
    audit::log_success(
        "cleanup_completed",
        None,
        format!("Cleanup finished | Processed: {} | Refunded: {} | BTC: {} | USDC: {}",
            escrows_processed, escrows_refunded, total_refunded_btc, total_refunded_usdc)
    );
    
    Ok(CleanupResult {
        escrows_processed,
        escrows_refunded,
        total_refunded_btc,
        total_refunded_usdc,
    })
}

// ============================================================================
// QUERY ENDPOINTS
// ============================================================================

/// Get spread basis points
#[query]
fn get_spread_basis_points() -> u64 {
    config::get_spread_basis_points()
}

/// Get DEX provider
#[query]
fn get_dex_provider() -> String {
    config::get_dex_provider()
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn parse_crypto_type(crypto_type_str: &str) -> Result<CryptoType, String> {
    match crypto_type_str {
        "CkBTC" | "BTC" => Ok(CryptoType::CkBTC),
        "CkUSDC" | "USDC" => Ok(CryptoType::CkUSDC),
        _ => Err(format!("Invalid crypto type: {}", crypto_type_str)),
    }
}
