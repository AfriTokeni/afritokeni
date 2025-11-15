use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::time;

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
    pub fee_charged: u64,
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
            ic_cdk::futures::spawn_017_compat(async {
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
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can set data canister ID".to_string());
    }
    config::set_data_canister_id(principal);
    Ok(())
}

/// Set user canister ID (admin only)
#[update]
fn set_user_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can set user canister ID".to_string());
    }
    config::set_user_canister_id(principal);
    Ok(())
}

/// Set wallet canister ID (admin only)
#[update]
fn set_wallet_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can set wallet canister ID".to_string());
    }
    config::set_wallet_canister_id(principal);
    Ok(())
}

/// Add authorized canister (admin only)
#[update]
fn add_authorized_canister(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can add authorized canisters".to_string());
    }
    config::add_authorized_canister(principal);
    Ok(())
}

/// Set ckBTC ledger ID (admin only, for testing)
#[update]
fn set_ckbtc_ledger_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can set ckBTC ledger ID".to_string());
    }
    config::set_ckbtc_ledger_id(principal);
    Ok(())
}

/// Set ckUSDC ledger ID (admin only, for testing)
#[update]
fn set_ckusdc_ledger_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can set ckUSDC ledger ID".to_string());
    }
    config::set_ckusdc_ledger_id(principal);
    Ok(())
}

/// Enable test mode (admin only)
#[update]
fn enable_test_mode() -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can enable test mode".to_string());
    }
    config::enable_test_mode();
    Ok(())
}

/// Disable test mode (admin only)
#[update]
fn disable_test_mode() -> Result<(), String> {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        return Err("Only controller can disable test mode".to_string());
    }
    config::disable_test_mode();
    Ok(())
}

/// Test-only helper: set crypto balances directly for a user
/// This is ONLY allowed in test mode and is used by PocketIC integration tests
#[update]
async fn set_crypto_balance_for_testing(
    user_identifier: String,
    ckbtc: u64,
    ckusdc: u64,
) -> Result<(), String> {
    // Guard: only in test mode
    if !config::is_test_mode() {
        return Err("set_crypto_balance_for_testing only allowed in test mode".to_string());
    }

    // Read current balances
    let (current_btc, current_usdc) = services::data_client::get_crypto_balance(&user_identifier).await?;

    // Compute deltas as signed i64
    let delta_btc: i64 = ckbtc as i64 - current_btc as i64;
    let delta_usdc: i64 = ckusdc as i64 - current_usdc as i64;

    // Apply deltas via normal balance update mechanism
    services::data_client::update_crypto_balance(&user_identifier, delta_btc, delta_usdc).await
}

// ============================================================================
// CRYPTO PURCHASE/SALE ENDPOINTS
// ============================================================================

/// Helper to check caller authorization
fn check_caller_authorized() -> Result<(), String> {
    if config::is_test_mode() {
        return Ok(());
    }
    let caller = ic_cdk::api::caller();
    if !config::is_authorized(&caller) {
        return Err("Unauthorized caller canister".to_string());
    }
    Ok(())
}

/// Buy cryptocurrency with fiat
#[update]
async fn buy_crypto(request: BuyCryptoRequest) -> Result<BuyCryptoResponse, String> {
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("buy_crypto");

    // Parse and validate inputs
    logic::crypto_logic::validate_fiat_amount_for_crypto(request.fiat_amount)?;
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    let fiat_currency = FiatCurrency::from_code(&request.currency)
        .ok_or_else(|| format!("Invalid currency code: {}", request.currency))?;

    // Calculate platform fee (0.5% = 50 basis points)
    timer.check_timeout()?;
    let fee_bp = config::get_purchase_fee_basis_points();
    let (fee_amount, total_to_deduct) = logic::transaction_helpers::calculate_purchase_fee(
        request.fiat_amount,
        fee_bp
    )?;

    // Step 1: Perform all security checks
    timer.check_timeout()?;
    perform_buy_crypto_security_checks(&request).await?;

    // Step 2: Execute the crypto purchase (includes fee deduction)
    timer.check_timeout()?;
    let (crypto_amount, exchange_rate, block_index) = execute_crypto_purchase(
        &request.user_identifier,
        request.fiat_amount,
        total_to_deduct,
        fee_amount,
        &request.currency,
        crypto_type,
        fiat_currency,
    ).await?;

    // Step 3: Record and finalize transaction
    timer.check_timeout()?;
    let timestamp = finalize_buy_crypto_transaction(
        &request.user_identifier,
        request.fiat_amount,
        &request.currency,
        crypto_amount,
        crypto_type,
        exchange_rate,
        block_index,
        fee_amount,
    ).await?;

    Ok(BuyCryptoResponse {
        transaction_id: format!("buy-crypto-{}-{}", request.user_identifier, timestamp),
        crypto_amount,
        fiat_amount: request.fiat_amount,
        crypto_type: format!("{:?}", crypto_type),
        exchange_rate,
        timestamp,
        fee_charged: fee_amount,
    })
}

/// Performs all security checks for buy_crypto operation
async fn perform_buy_crypto_security_checks(request: &BuyCryptoRequest) -> Result<(), String> {
    // 1. Verify user exists
    logic::transaction_helpers::verify_user_exists(&request.user_identifier).await?;

    // 2. Verify PIN with exponential backoff
    let audit_context = format!(
        "Invalid PIN | Amount: {} {} | Device: {:?} | Location: {:?}",
        request.fiat_amount, request.currency, request.device_fingerprint, request.geo_location
    );
    logic::transaction_helpers::verify_pin_with_backoff(
        &request.user_identifier,
        &request.pin,
        "buy_crypto",
        &audit_context,
    ).await?;

    // 3. Check operation rate limit
    let rate_limit_context = format!(
        "Operation: buy_crypto | Amount: {} {}",
        request.fiat_amount, request.currency
    );
    logic::transaction_helpers::check_operation_rate_limit(
        &request.user_identifier,
        "buy_crypto",
        &rate_limit_context,
    )?;

    // 4. Comprehensive fraud check
    let fraud_context = logic::transaction_helpers::FraudCheckContext {
        user_identifier: &request.user_identifier,
        amount: request.fiat_amount,
        currency: &request.currency,
        operation: "buy_crypto",
        device_fingerprint: request.device_fingerprint.as_deref(),
        geo_location: request.geo_location.as_deref(),
    };
    logic::transaction_helpers::perform_fraud_check(&fraud_context)?;

    // 5. Record device and location
    logic::transaction_helpers::record_device_and_location(
        &request.user_identifier,
        request.device_fingerprint.as_deref(),
        request.geo_location.as_deref(),
    )?;

    Ok(())
}

/// Executes the crypto purchase including fiat deduction and crypto transfer
/// Returns (crypto_amount, exchange_rate, block_index)
async fn execute_crypto_purchase(
    user_identifier: &str,
    fiat_amount: u64,
    total_to_deduct: u64,
    fee_amount: u64,
    currency: &str,
    crypto_type: CryptoType,
    fiat_currency: FiatCurrency,
) -> Result<(u64, f64, u64), String> {
    // 1. Check fiat balance (must cover fiat_amount + fee)
    let fiat_balance = services::wallet_client::get_fiat_balance(user_identifier, fiat_currency).await?;
    if fiat_balance < total_to_deduct {
        return Err(format!(
            "Insufficient fiat balance. Have: {}, Need: {} (amount: {} + fee: {})",
            fiat_balance, total_to_deduct, fiat_amount, fee_amount
        ));
    }

    // 2. Calculate crypto amount using real exchange rates (on base amount, not including fee)
    let crypto_type_str = format!("{:?}", crypto_type);
    let crypto_amount = services::exchange_rate::calculate_crypto_from_fiat(
        fiat_amount,
        currency,
        &crypto_type_str
    ).await?;

    // 3. Calculate exchange rate for display
    let exchange_rate = logic::transaction_helpers::calculate_exchange_rate(crypto_amount, fiat_amount);

    // 4. Deduct fiat from wallet (base amount + fee)
    let new_fiat_balance = fiat_balance.checked_sub(total_to_deduct)
        .ok_or("Fiat balance calculation would underflow")?;
    services::wallet_client::set_fiat_balance(user_identifier, fiat_currency, new_fiat_balance).await?;

    // 5. Transfer fee to company wallet
    let company_principal = config::get_company_wallet_principal()?;
    let company_balance = services::wallet_client::get_fiat_balance(
        &company_principal.to_string(),
        fiat_currency
    ).await.unwrap_or(0);
    let new_company_balance = company_balance.checked_add(fee_amount)
        .ok_or("Company wallet balance would overflow")?;
    services::wallet_client::set_fiat_balance(
        &company_principal.to_string(),
        fiat_currency,
        new_company_balance
    ).await?;

    audit::log_success(
        "platform_fee_collected",
        Some(user_identifier.to_string()),
        format!("Purchase fee collected: {} {} ({}%)", fee_amount, currency,
            (fee_amount as f64 / fiat_amount as f64) * 100.0)
    );

    // 6. Get user's Principal ID for non-custodial transfer
    let user_principal = services::ledger_client::get_user_principal(user_identifier).await?;

    // 7. Transfer crypto from platform reserve to user's Principal (NON-CUSTODIAL)
    let block_index = match crypto_type {
        CryptoType::CkBTC => {
            services::ledger_client::transfer_ckbtc_to_user(user_principal, crypto_amount).await?
        },
        CryptoType::CkUSDC => {
            services::ledger_client::transfer_ckusdc_to_user(user_principal, crypto_amount).await?
        },
    };

    audit::log_success(
        "crypto_transferred_to_user",
        Some(user_identifier.to_string()),
        format!("Transferred {} {} to Principal {} | Block: {}",
            crypto_amount, crypto_type_str, user_principal, block_index)
    );

    Ok((crypto_amount, exchange_rate, block_index))
}

/// Finalizes buy_crypto transaction by updating balances and recording transaction
/// Returns timestamp of transaction
async fn finalize_buy_crypto_transaction(
    user_identifier: &str,
    fiat_amount: u64,
    currency: &str,
    crypto_amount: u64,
    crypto_type: CryptoType,
    exchange_rate: f64,
    block_index: u64,
    fee_amount: u64,
) -> Result<u64, String> {
    let crypto_type_str = format!("{:?}", crypto_type);

    // 1. Update crypto balance in data canister (for tracking purposes)
    let (delta_btc, delta_usdc) = logic::transaction_helpers::calculate_crypto_delta(
        crypto_amount,
        crypto_type,
        true, // is_credit
    );
    services::data_client::update_crypto_balance(user_identifier, delta_btc, delta_usdc).await?;

    // 2. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("buy-crypto-{}-{}", user_identifier, timestamp),
        transaction_type: TransactionType::BuyCrypto,
        from_user: Some(user_identifier.to_string()),
        to_user: Some(user_identifier.to_string()),
        amount: crypto_amount,
        currency_type: CurrencyType::Crypto(crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Bought {} {} for {} {} (fee: {} {}) | Block: {}",
            crypto_amount, crypto_type_str, fiat_amount, currency, fee_amount, currency, block_index)),
    };

    services::data_client::store_transaction(&transaction).await?;

    // 3. Record transaction for velocity tracking
    logic::transaction_helpers::record_transaction_for_velocity(
        user_identifier,
        fiat_amount,
        currency,
        "buy_crypto",
    )?;

    // 4. Audit successful transaction
    audit::log_success(
        "buy_crypto_completed",
        Some(user_identifier.to_string()),
        format!("Bought {} {} for {} {} | Fee: {} {} | Exchange Rate: {} | Block: {} | TX: {}",
            crypto_amount, crypto_type_str, fiat_amount, currency, fee_amount, currency,
            exchange_rate, block_index, transaction.id)
    );

    Ok(timestamp)
}

/// Sell cryptocurrency for fiat
#[update]
async fn sell_crypto(request: SellCryptoRequest) -> Result<BuyCryptoResponse, String> {
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("sell_crypto");

    // Parse and validate inputs
    logic::crypto_logic::validate_crypto_amount_positive(request.crypto_amount)?;
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    let fiat_currency = FiatCurrency::from_code(&request.currency)
        .ok_or_else(|| format!("Invalid currency code: {}", request.currency))?;

    // Step 1: Perform initial security checks (PIN, rate limits)
    timer.check_timeout()?;
    perform_sell_crypto_initial_checks(&request).await?;

    // Step 2: Execute the crypto sale
    timer.check_timeout()?;
    let (gross_fiat_amount, exchange_rate, transfer_block) = execute_crypto_sale(
        &request.user_identifier,
        request.crypto_amount,
        &request.currency,
        crypto_type,
        fiat_currency,
        request.device_fingerprint.as_deref(),
        request.geo_location.as_deref(),
    ).await?;

    // Step 3: Calculate and deduct platform fee (0.5% = 50 basis points)
    timer.check_timeout()?;
    let fee_bp = config::get_sale_fee_basis_points();
    let (fee_amount, net_proceeds) = logic::transaction_helpers::calculate_sale_fee(
        gross_fiat_amount,
        fee_bp
    )?;

    // Step 4: Transfer fee to company wallet
    timer.check_timeout()?;
    transfer_sale_fee_to_company(
        &request.user_identifier,
        fee_amount,
        fiat_currency,
        &request.currency
    ).await?;

    // Step 5: Record and finalize transaction
    timer.check_timeout()?;
    let timestamp = finalize_sell_crypto_transaction(
        &request.user_identifier,
        request.crypto_amount,
        gross_fiat_amount,
        net_proceeds,
        &request.currency,
        crypto_type,
        exchange_rate,
        transfer_block,
        fee_amount,
    ).await?;

    Ok(BuyCryptoResponse {
        transaction_id: format!("sell-crypto-{}-{}", request.user_identifier, timestamp),
        crypto_amount: request.crypto_amount,
        fiat_amount: net_proceeds,
        crypto_type: format!("{:?}", crypto_type),
        exchange_rate,
        timestamp,
        fee_charged: fee_amount,
    })
}

/// Transfers sale fee to company wallet
async fn transfer_sale_fee_to_company(
    user_identifier: &str,
    fee_amount: u64,
    fiat_currency: FiatCurrency,
    currency_code: &str,
) -> Result<(), String> {
    let company_principal = config::get_company_wallet_principal()?;
    let company_balance = services::wallet_client::get_fiat_balance(
        &company_principal.to_string(),
        fiat_currency
    ).await.unwrap_or(0);

    let new_company_balance = company_balance.checked_add(fee_amount)
        .ok_or("Company wallet balance would overflow")?;

    services::wallet_client::set_fiat_balance(
        &company_principal.to_string(),
        fiat_currency,
        new_company_balance
    ).await?;

    audit::log_success(
        "platform_fee_collected",
        Some(user_identifier.to_string()),
        format!("Sale fee collected: {} {} (0.5%)", fee_amount, currency_code)
    );

    Ok(())
}

/// Performs initial security checks for sell_crypto (PIN, rate limits, user verification)
async fn perform_sell_crypto_initial_checks(request: &SellCryptoRequest) -> Result<(), String> {
    // 1. Verify user exists
    logic::transaction_helpers::verify_user_exists(&request.user_identifier).await?;

    // 2. Verify PIN with exponential backoff
    let audit_context = format!(
        "Invalid PIN | Amount: {} {} | Device: {:?} | Location: {:?}",
        request.crypto_amount, request.crypto_type, request.device_fingerprint, request.geo_location
    );
    logic::transaction_helpers::verify_pin_with_backoff(
        &request.user_identifier,
        &request.pin,
        "sell_crypto",
        &audit_context,
    ).await?;

    // 3. Check operation rate limit
    let rate_limit_context = format!(
        "Operation: sell_crypto | Amount: {} {}",
        request.crypto_amount, request.crypto_type
    );
    logic::transaction_helpers::check_operation_rate_limit(
        &request.user_identifier,
        "sell_crypto",
        &rate_limit_context,
    )?;

    Ok(())
}

/// Executes the crypto sale including balance checks, fraud detection, and transfers
/// Returns (gross_fiat_amount, exchange_rate, transfer_block)
async fn execute_crypto_sale(
    user_identifier: &str,
    crypto_amount: u64,
    currency: &str,
    crypto_type: CryptoType,
    _fiat_currency: FiatCurrency,
    device_fingerprint: Option<&str>,
    geo_location: Option<&str>,
) -> Result<(u64, f64, u64), String> {
    let crypto_type_str = format!("{:?}", crypto_type);

    // 1. Get user's Principal ID and check crypto balance on ledger (NON-CUSTODIAL)
    let user_principal = services::ledger_client::get_user_principal(user_identifier).await?;
    let crypto_balance = match crypto_type {
        CryptoType::CkBTC => services::ledger_client::get_user_ckbtc_balance(user_principal).await?,
        CryptoType::CkUSDC => services::ledger_client::get_user_ckusdc_balance(user_principal).await?,
    };
    logic::crypto_logic::validate_sufficient_crypto_balance(crypto_balance, crypto_amount)?;

    // 2. Calculate gross fiat amount using real exchange rates (BEFORE fee deduction)
    let gross_fiat_amount = services::exchange_rate::calculate_fiat_from_crypto(
        crypto_amount,
        &crypto_type_str,
        currency
    ).await?;

    // 3. Calculate exchange rate for display
    let exchange_rate = logic::transaction_helpers::calculate_exchange_rate(crypto_amount, gross_fiat_amount);

    // 4. Comprehensive fraud check (using gross amount)
    let fraud_context = logic::transaction_helpers::FraudCheckContext {
        user_identifier,
        amount: gross_fiat_amount,
        currency,
        operation: "sell_crypto",
        device_fingerprint,
        geo_location,
    };
    logic::transaction_helpers::perform_fraud_check(&fraud_context)?;

    // 5. Record device and location
    logic::transaction_helpers::record_device_and_location(
        user_identifier,
        device_fingerprint,
        geo_location,
    )?;

    // 6. Transfer crypto from user to platform reserve (ICRC-2 transfer_from)
    // NOTE: User must have pre-approved spending via web UI or test setup
    let transfer_block = match crypto_type {
        CryptoType::CkBTC => {
            services::ledger_client::transfer_from_ckbtc(user_principal, crypto_amount).await?
        },
        CryptoType::CkUSDC => {
            services::ledger_client::transfer_from_ckusdc(user_principal, crypto_amount).await?
        },
    };

    audit::log_success(
        "crypto_transferred_from_user",
        Some(user_identifier.to_string()),
        format!("Transferred {} {} from Principal {} to reserve | Block: {}",
            crypto_amount, crypto_type_str, user_principal, transfer_block)
    );

    Ok((gross_fiat_amount, exchange_rate, transfer_block))
}

/// Finalizes sell_crypto transaction by updating balances and recording transaction
/// Returns timestamp of transaction
async fn finalize_sell_crypto_transaction(
    user_identifier: &str,
    crypto_amount: u64,
    gross_fiat_amount: u64,
    net_proceeds: u64,
    currency: &str,
    crypto_type: CryptoType,
    exchange_rate: f64,
    transfer_block: u64,
    fee_amount: u64,
) -> Result<u64, String> {
    let crypto_type_str = format!("{:?}", crypto_type);

    // 1. Update crypto balance in data canister (deduct sold crypto)
    let (delta_btc, delta_usdc) = logic::transaction_helpers::calculate_crypto_delta(
        crypto_amount,
        crypto_type,
        false, // is_credit = false (debit)
    );
    services::data_client::update_crypto_balance(user_identifier, delta_btc, delta_usdc).await?;

    // 2. Add NET proceeds to wallet (IOU system) - after fee deduction
    let fiat_currency = FiatCurrency::from_code(currency)
        .ok_or_else(|| format!("Invalid currency code: {}", currency))?;
    let current_fiat_balance = services::wallet_client::get_fiat_balance(user_identifier, fiat_currency).await?;
    let new_fiat_balance = current_fiat_balance.checked_add(net_proceeds)
        .ok_or("Fiat balance calculation would overflow")?;
    services::wallet_client::set_fiat_balance(user_identifier, fiat_currency, new_fiat_balance).await?;

    // 3. Record transaction
    let timestamp = time();
    let transaction = Transaction {
        id: format!("sell-crypto-{}-{}", user_identifier, timestamp),
        transaction_type: TransactionType::SellCrypto,
        from_user: Some(user_identifier.to_string()),
        to_user: Some(user_identifier.to_string()),
        amount: crypto_amount,
        currency_type: CurrencyType::Crypto(crypto_type),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Sold {} {} for {} {} (gross: {}, fee: {}) | Transfer Block: {}",
            crypto_amount, crypto_type_str, net_proceeds, currency, gross_fiat_amount, fee_amount, transfer_block)),
    };

    services::data_client::store_transaction(&transaction).await?;

    // 4. Record transaction for velocity tracking (use gross amount)
    logic::transaction_helpers::record_transaction_for_velocity(
        user_identifier,
        gross_fiat_amount,
        currency,
        "sell_crypto",
    )?;

    // 5. Audit successful transaction
    audit::log_success(
        "sell_crypto_completed",
        Some(user_identifier.to_string()),
        format!("Sold {} {} for {} {} | Gross: {} | Fee: {} | Exchange Rate: {} | Transfer Block: {} | TX: {}",
            crypto_amount, crypto_type_str, net_proceeds, currency, gross_fiat_amount, fee_amount,
            exchange_rate, transfer_block, transaction.id)
    );

    Ok(timestamp)
}

// ============================================================================
// CRYPTO TRANSFER ENDPOINTS
// ============================================================================

/// Send cryptocurrency to external address
#[update]
async fn send_crypto(request: SendCryptoRequest) -> Result<String, String> {
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("send_crypto");

    // Validate inputs
    logic::crypto_logic::validate_crypto_amount_positive(request.amount)?;
    let crypto_type = parse_crypto_type(&request.crypto_type)?;
    let crypto_type_str = format!("{:?}", crypto_type);
    logic::crypto_logic::validate_crypto_address(&request.to_address, &crypto_type_str)?;

    // 1. Verify user exists
    timer.check_timeout()?;
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Check PIN attempts allowed (exponential backoff)
    timer.check_timeout()?;
    logic::fraud_detection::check_pin_attempts_allowed(&request.user_identifier)?;

    // 3. Verify PIN
    timer.check_timeout()?;
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
    timer.check_timeout()?;
    if !logic::fraud_detection::check_operation_rate_limit(&request.user_identifier, "send_crypto")? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(request.user_identifier.clone()),
            format!("Operation: send_crypto | Amount: {} {} | To: {}", request.amount, request.crypto_type, request.to_address)
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }

    // 5. Comprehensive fraud check
    timer.check_timeout()?;
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
    timer.check_timeout()?;
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&request.user_identifier).await?;
    let crypto_balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };

    logic::crypto_logic::validate_sufficient_crypto_balance(crypto_balance, request.amount)?;

    // 8. Deduct crypto from user's balance
    timer.check_timeout()?;
    let (ckbtc_delta, ckusdc_delta) = match crypto_type {
        CryptoType::CkBTC => (-(request.amount as i64), 0),
        CryptoType::CkUSDC => (0, -(request.amount as i64)),
    };
    services::data_client::update_crypto_balance(&request.user_identifier, ckbtc_delta, ckusdc_delta).await?;

    // 9. Record transaction
    timer.check_timeout()?;
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

    // 10. Record transaction for velocity tracking
    logic::fraud_detection::record_transaction(
        &request.user_identifier,
        request.amount,
        "USD",
        "send_crypto",
    )?;

    // 11. Audit successful transaction
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
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("swap_crypto");

    // Validate inputs
    logic::crypto_logic::validate_crypto_amount_positive(request.amount)?;

    let from_crypto = parse_crypto_type(&request.from_crypto)?;
    let to_crypto = parse_crypto_type(&request.to_crypto)?;
    let from_crypto_str = format!("{:?}", from_crypto);
    let to_crypto_str = format!("{:?}", to_crypto);

    if from_crypto == to_crypto {
        return Err("Cannot swap same cryptocurrency".to_string());
    }

    // 1. Resolve user_identifier to user_id (handles phone/principal/user_id)
    timer.check_timeout()?;
    let user_id = services::user_client::resolve_user_id(&request.user_identifier).await?;

    // 2. Verify PIN
    timer.check_timeout()?;
    let verified = services::user_client::verify_pin(&request.user_identifier, &request.pin).await?;
    if !verified {
        audit::log_failure(
            "failed_pin_swap_crypto",
            Some(user_id.clone()),
            format!("Invalid PIN | Swap: {} {} to {} {}", request.amount, request.from_crypto, request.amount, request.to_crypto)
        );
        return Err("Invalid PIN".to_string());
    }

    // 3. Check balance (use user_id for data canister operations)
    timer.check_timeout()?;
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&user_id).await?;
    let from_balance = match from_crypto {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };

    logic::crypto_logic::validate_sufficient_crypto_balance(from_balance, request.amount)?;

    // 4. Calculate spread
    let spread_bp = config::get_spread_basis_points();
    let spread_amount = (request.amount * spread_bp) / 10000;
    let swap_amount = request.amount - spread_amount;

    // 5. Calculate expected output with slippage protection
    timer.check_timeout()?;
    // Estimate expected output (1:1 for simplicity, real implementation would use oracle)
    let expected_output = swap_amount;
    let slippage_bp = 100; // 1% slippage tolerance
    let min_output = services::dex_client::calculate_min_output_with_slippage(expected_output, slippage_bp)?;

    // 6. Perform swap via DEX with slippage protection
    timer.check_timeout()?;
    let to_amount = services::dex_client::swap_tokens(from_crypto, to_crypto, swap_amount, min_output).await?;

    // 7. Validate actual slippage after swap
    services::dex_client::validate_slippage(expected_output, to_amount, slippage_bp)?;

    // 8. Calculate exchange rate
    let exchange_rate = if request.amount > 0 {
        to_amount as f64 / request.amount as f64
    } else {
        0.0
    };

    // 9. Update balances (use user_id for data canister operations)
    timer.check_timeout()?;
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

    services::data_client::update_crypto_balance(&user_id, total_btc_delta, total_usdc_delta).await?;

    // 10. Record transaction (use user_id for data canister operations)
    timer.check_timeout()?;
    let timestamp = time();
    let transaction = Transaction {
        id: format!("swap-crypto-{}-{}", user_id, timestamp),
        transaction_type: TransactionType::SwapCrypto,
        from_user: Some(user_id.clone()),
        to_user: Some(user_id.clone()),
        amount: request.amount,
        currency_type: CurrencyType::Crypto(from_crypto),
        status: TransactionStatus::Completed,
        created_at: timestamp,
        completed_at: Some(timestamp),
        description: Some(format!("Swapped {} {} to {} {} (spread: {})",
            request.amount, request.from_crypto, to_amount, request.to_crypto, spread_amount)),
    };

    services::data_client::store_transaction(&transaction).await?;

    // 11. Audit successful swap
    audit::log_success(
        "swap_crypto_completed",
        Some(user_id.clone()),
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
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("create_escrow");

    // Validate inputs
    logic::escrow_logic::validate_escrow_amount(request.amount)?;
    let crypto_type = parse_crypto_type(&request.crypto_type)?;

    // 1. Verify user exists
    timer.check_timeout()?;
    let user_exists = services::user_client::user_exists(&request.user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    
    // 2. Check PIN attempts allowed (exponential backoff)
    timer.check_timeout()?;
    logic::fraud_detection::check_pin_attempts_allowed(&request.user_identifier)?;

    // 3. Verify PIN
    timer.check_timeout()?;
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
    timer.check_timeout()?;
    if !logic::fraud_detection::check_operation_rate_limit(&request.user_identifier, "create_escrow")? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(request.user_identifier.clone()),
            format!("Operation: create_escrow | Amount: {} {} | Agent: {}", request.amount, request.crypto_type, request.agent_id)
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }

    // 5. Comprehensive fraud check
    timer.check_timeout()?;
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
    timer.check_timeout()?;
    let (ckbtc_balance, ckusdc_balance) = services::data_client::get_crypto_balance(&request.user_identifier).await?;
    let crypto_balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };

    logic::crypto_logic::validate_sufficient_crypto_balance(crypto_balance, request.amount)?;

    // 8. Generate escrow code
    let timestamp = time();
    let code = logic::escrow_logic::generate_escrow_code(timestamp, &request.user_identifier);

    // 9. Calculate expiration
    let expiration_ns = config::get_escrow_expiration_ns();
    let expires_at = logic::escrow_logic::calculate_expiration_time(timestamp, expiration_ns)?;

    // 10. Deduct crypto from user's balance
    timer.check_timeout()?;
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_creation_delta(request.amount, crypto_type);
    services::data_client::update_crypto_balance(&request.user_identifier, ckbtc_delta, ckusdc_delta).await?;

    // 11. Create escrow in data canister
    timer.check_timeout()?;
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
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("verify_escrow");

    // 1. Get escrow
    timer.check_timeout()?;
    let escrow = services::data_client::get_escrow(&request.code).await?;

    // 2. Validate escrow
    logic::escrow_logic::validate_escrow_active(escrow.status)?;
    logic::escrow_logic::validate_escrow_not_expired(time(), escrow.expires_at)?;
    logic::escrow_logic::validate_agent_authorized(&escrow.agent_id, &request.agent_id)?;

    // 3. Verify agent PIN
    timer.check_timeout()?;
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
    timer.check_timeout()?;
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_claim_delta(escrow.amount, escrow.crypto_type);
    services::data_client::update_crypto_balance(&request.agent_id, ckbtc_delta, ckusdc_delta).await?;

    // 5. Update escrow status
    timer.check_timeout()?;
    services::data_client::update_escrow_status(&request.code, EscrowStatus::Claimed).await?;

    // 6. Record transaction
    timer.check_timeout()?;
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
    // CRITICAL: Check authorization first
    check_caller_authorized()?;

    // Initialize timeout protection
    let timer = logic::timeout::TransactionTimer::new("cancel_escrow");

    // 1. Get escrow
    timer.check_timeout()?;
    let escrow = services::data_client::get_escrow(&code).await?;

    // 2. Validate ownership
    logic::escrow_logic::validate_user_owns_escrow(&escrow.user_id, &user_id)?;
    logic::escrow_logic::validate_escrow_active(escrow.status)?;

    // 3. Verify PIN
    timer.check_timeout()?;
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
    timer.check_timeout()?;
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_refund_delta(escrow.amount, escrow.crypto_type);
    services::data_client::update_crypto_balance(&user_id, ckbtc_delta, ckusdc_delta).await?;

    // 5. Update escrow status
    timer.check_timeout()?;
    services::data_client::update_escrow_status(&code, EscrowStatus::Cancelled).await?;

    // 6. Record transaction
    timer.check_timeout()?;
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
// PLATFORM RESERVE MANAGEMENT
// ============================================================================

/// Get platform reserve balances (admin only)
#[update]
async fn get_reserve_balance(btc_price_usd: f64) -> Result<services::reserve_manager::ReserveBalance, String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    if btc_price_usd <= 0.0 {
        return Err("Invalid BTC price. Must be greater than 0".to_string());
    }
    
    services::reserve_manager::get_reserve_balance_with_price(btc_price_usd).await
}

/// Rebalance platform reserve to maintain 50/50 BTC/USDC allocation (admin only)
#[update]
async fn rebalance_reserve(btc_price_usd: f64) -> Result<services::reserve_manager::RebalanceResult, String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Unauthorized: Controller access required".to_string());
    }
    
    if btc_price_usd <= 0.0 {
        return Err("Invalid BTC price. Must be greater than 0".to_string());
    }
    
    services::reserve_manager::rebalance_reserve(btc_price_usd).await
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

// Export Candid interface
ic_cdk::export_candid!();
