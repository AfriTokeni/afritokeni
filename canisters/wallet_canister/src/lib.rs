use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::{time, msg_caller};

mod logic;
mod services;
mod config;

use shared_types::{
    FiatCurrency, CryptoType, Transaction, TransactionType, TransactionStatus,
    CurrencyType, Escrow, EscrowStatus, audit,
};

/// Transfer request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: u64,
    pub currency: String,
    pub pin: String,
    pub description: Option<String>,
}

/// Transfer response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferResponse {
    pub transaction_id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: u64,
    pub fee: u64,
    pub currency: String,
    pub sender_new_balance: u64,
    pub recipient_new_balance: u64,
    pub timestamp: u64,
}

/// Escrow creation request
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub crypto_type: String,
    pub pin: String,
}

/// Escrow creation response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowResponse {
    pub code: String,
    pub amount: u64,
    pub crypto_type: String,
    pub expires_at: u64,
}

// ============================================================================
// CANISTER INITIALIZATION
// ============================================================================

#[init]
fn init() {
    config::init_config();
    ic_cdk::println!("✅ Wallet Canister initialized");
}

// ============================================================================
// ADMIN ENDPOINTS
// ============================================================================

/// Set data canister ID (admin only)
#[update]
fn set_data_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&msg_caller()) {
        return Err("Only controller can set data canister ID".to_string());
    }
    config::set_data_canister_id(principal);
    Ok(())
}

/// Set user canister ID (admin only)
#[update]
fn set_user_canister_id(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&msg_caller()) {
        return Err("Only controller can set user canister ID".to_string());
    }
    config::set_user_canister_id(principal);
    Ok(())
}

/// Add authorized canister (admin only)
#[update]
fn add_authorized_canister(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&msg_caller()) {
        return Err("Only controller can add authorized canisters".to_string());
    }
    config::add_authorized_canister(principal);
    Ok(())
}

/// Remove authorized canister (admin only)
#[update]
fn remove_authorized_canister(principal: Principal) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&msg_caller()) {
        return Err("Only controller can remove authorized canisters".to_string());
    }
    config::remove_authorized_canister(principal);
    Ok(())
}

// ============================================================================
// FIAT TRANSFER ENDPOINTS
// ============================================================================

/// Transfer fiat currency between users
#[update]
async fn transfer_fiat(request: TransferRequest) -> Result<TransferResponse, String> {
    config::verify_authorized_caller()?;
    
    let current_time = time();
    
    // 1. Validate inputs
    logic::transfer_logic::validate_identifier_not_empty(&request.from_user_id, "From user ID")?;
    logic::transfer_logic::validate_identifier_not_empty(&request.to_user_id, "To user ID")?;
    logic::transfer_logic::validate_amount_positive(request.amount)?;
    logic::transfer_logic::validate_not_self_transfer(&request.from_user_id, &request.to_user_id)?;
    
    // 2. Parse currency
    let currency = FiatCurrency::from_string(&request.currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    // 3. Verify PIN
    let pin_valid = services::user_client::verify_pin(&request.from_user_id, &request.pin).await?;
    if !pin_valid {
        audit::log_failure(
            "transfer_fiat_failed",
            Some(request.from_user_id.clone()),
            "Invalid PIN".to_string(),
        );
        return Err("Invalid PIN".to_string());
    }
    
    // 4. Calculate fee
    let fee_bps = config::get_transfer_fee_bps();
    let fee = logic::transfer_logic::calculate_fee(request.amount, fee_bps)?;
    
    // 5. Get fraud limits for currency
    let fraud_limits = config::get_fraud_limits(currency.code());
    
    // 6. Check fraud detection
    let fraud_check = logic::fraud_logic::check_transaction_amount(
        request.amount,
        fraud_limits.max_transaction_amount,
        fraud_limits.suspicious_threshold,
    );
    
    if fraud_check.should_block {
        audit::log_failure(
            "transfer_fiat_blocked",
            Some(request.from_user_id.clone()),
            format!("Fraud check failed: {:?}", fraud_check.warnings),
        );
        return Err(format!("Transaction blocked: {}", fraud_check.warnings.join(", ")));
    }
    
    // 7. Get sender balance
    let sender_balance = services::data_client::get_fiat_balance(&request.from_user_id, currency).await?;
    
    // 8. Validate sufficient balance
    logic::transfer_logic::validate_sufficient_balance(sender_balance, request.amount, fee)?;
    
    // 9. Get recipient balance
    let recipient_balance = services::data_client::get_fiat_balance(&request.to_user_id, currency).await?;
    
    // 10. Calculate new balances
    let sender_new_balance = logic::transfer_logic::calculate_new_balance(sender_balance, request.amount + fee)?;
    let recipient_new_balance = logic::transfer_logic::calculate_balance_addition(recipient_balance, request.amount)?;
    
    // 11. Update balances
    services::data_client::set_fiat_balance(&request.from_user_id, currency, sender_new_balance).await?;
    services::data_client::set_fiat_balance(&request.to_user_id, currency, recipient_new_balance).await?;
    
    // 12. Generate transaction ID
    let tx_id = logic::transfer_logic::generate_transaction_id(current_time);
    
    // 13. Store transaction
    let transaction = Transaction {
        id: tx_id.clone(),
        transaction_type: TransactionType::TransferFiat,
        from_user: Some(request.from_user_id.clone()),
        to_user: Some(request.to_user_id.clone()),
        amount: request.amount,
        currency_type: CurrencyType::Fiat(currency),
        status: TransactionStatus::Completed,
        created_at: current_time,
        completed_at: Some(current_time),
        description: request.description.or(Some(format!("Transfer {} {}", request.amount, currency.code()))),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 14. Log success
    audit::log_success(
        "transfer_fiat",
        Some(request.from_user_id.clone()),
        format!("Transferred {} {} to {}", request.amount, currency.code(), request.to_user_id),
    );
    
    Ok(TransferResponse {
        transaction_id: tx_id,
        from_user_id: request.from_user_id,
        to_user_id: request.to_user_id,
        amount: request.amount,
        fee,
        currency: currency.code().to_string(),
        sender_new_balance,
        recipient_new_balance,
        timestamp: current_time,
    })
}

// ============================================================================
// ESCROW ENDPOINTS
// ============================================================================

/// Create escrow for crypto sale to agent
#[update]
async fn create_escrow(request: CreateEscrowRequest) -> Result<CreateEscrowResponse, String> {
    config::verify_authorized_caller()?;
    
    let current_time = time();
    
    // 1. Validate inputs
    logic::transfer_logic::validate_identifier_not_empty(&request.user_id, "User ID")?;
    logic::transfer_logic::validate_identifier_not_empty(&request.agent_id, "Agent ID")?;
    logic::escrow_logic::validate_escrow_amount(request.amount)?;
    
    // 2. Parse crypto type
    let crypto_type = match request.crypto_type.as_str() {
        "CkBTC" => CryptoType::CkBTC,
        "CkUSDC" => CryptoType::CkUSDC,
        _ => return Err(format!("Invalid crypto type: {}", request.crypto_type)),
    };
    
    // 3. Verify PIN
    let pin_valid = services::user_client::verify_pin(&request.user_id, &request.pin).await?;
    if !pin_valid {
        audit::log_failure(
            "create_escrow_failed",
            Some(request.user_id.clone()),
            "Invalid PIN".to_string(),
        );
        return Err("Invalid PIN".to_string());
    }
    
    // 4. Generate escrow code
    let code = logic::escrow_logic::generate_escrow_code(current_time, &request.user_id);
    
    // 5. Calculate expiration time
    let expiration_duration = config::get_escrow_expiration_time_ns();
    let expires_at = logic::escrow_logic::calculate_expiration_time(current_time, expiration_duration)?;
    
    // 6. Deduct crypto from user balance
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_creation_delta(request.amount, crypto_type);
    services::data_client::update_crypto_balance(&request.user_id, ckbtc_delta, ckusdc_delta).await?;
    
    // 7. Create and store escrow in data canister
    let escrow = Escrow {
        code: code.clone(),
        user_id: request.user_id.clone(),
        agent_id: request.agent_id.clone(),
        amount: request.amount,
        crypto_type,
        status: EscrowStatus::Active,
        created_at: current_time,
        expires_at,
        claimed_at: None,
    };
    services::data_client::store_escrow(escrow).await?;
    
    // 8. Store transaction
    let tx_id = logic::transfer_logic::generate_transaction_id(current_time);
    let transaction = Transaction {
        id: tx_id,
        transaction_type: TransactionType::SellCrypto,
        from_user: Some(request.user_id.clone()),
        to_user: Some(request.agent_id.clone()),
        amount: request.amount,
        currency_type: CurrencyType::Crypto(crypto_type),
        status: TransactionStatus::Pending,
        created_at: current_time,
        completed_at: None,
        description: Some(format!("Escrow created: {}", code)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 9. Log success
    audit::log_success(
        "create_escrow",
        Some(request.user_id.clone()),
        format!("Created escrow {} for {} {:?}", code, request.amount, crypto_type),
    );
    
    Ok(CreateEscrowResponse {
        code,
        amount: request.amount,
        crypto_type: format!("{:?}", crypto_type),
        expires_at,
    })
}

/// Claim escrow (agent verifies code and receives crypto)
#[update]
async fn claim_escrow(code: String, agent_id: String) -> Result<(), String> {
    config::verify_authorized_caller()?;
    
    let current_time = time();
    
    // 1. Get escrow
    let escrow = services::data_client::get_escrow(&code).await?
        .ok_or_else(|| format!("Escrow not found: {}", code))?;
    
    // 2. Validate escrow status
    logic::escrow_logic::validate_escrow_active(escrow.status)?;
    
    // 3. Validate not expired
    logic::escrow_logic::validate_escrow_not_expired(current_time, escrow.expires_at)?;
    
    // 4. Validate agent
    logic::escrow_logic::validate_agent_authorized(&escrow.agent_id, &agent_id)?;
    
    // 5. Transfer crypto to agent
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_claim_delta(escrow.amount, escrow.crypto_type);
    services::data_client::update_crypto_balance(&agent_id, ckbtc_delta, ckusdc_delta).await?;
    
    // 6. Update escrow status
    services::data_client::update_escrow_status(&code, EscrowStatus::Claimed).await?;
    
    // 7. Store transaction
    let tx_id = logic::transfer_logic::generate_transaction_id(current_time);
    let transaction = Transaction {
        id: tx_id,
        transaction_type: TransactionType::SellCrypto,
        from_user: Some(escrow.user_id.clone()),
        to_user: Some(agent_id.clone()),
        amount: escrow.amount,
        currency_type: CurrencyType::Crypto(escrow.crypto_type),
        status: TransactionStatus::Completed,
        created_at: current_time,
        completed_at: Some(current_time),
        description: Some(format!("Escrow claimed: {}", code)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 8. Log success
    audit::log_success(
        "claim_escrow",
        Some(agent_id),
        format!("Agent claimed escrow: {}", code),
    );
    
    Ok(())
}

/// Cancel escrow and refund crypto to user
#[update]
async fn cancel_escrow(code: String, user_id: String, pin: String) -> Result<(), String> {
    config::verify_authorized_caller()?;
    
    let current_time = time();
    
    // 1. Get escrow
    let escrow = services::data_client::get_escrow(&code).await?
        .ok_or_else(|| format!("Escrow not found: {}", code))?;
    
    // 2. Validate user owns escrow
    logic::escrow_logic::validate_user_owns_escrow(&escrow.user_id, &user_id)?;
    
    // 3. Validate escrow status
    logic::escrow_logic::validate_escrow_active(escrow.status)?;
    
    // 4. Verify PIN
    let pin_valid = services::user_client::verify_pin(&user_id, &pin).await?;
    if !pin_valid {
        audit::log_failure(
            "cancel_escrow_failed",
            Some(user_id.clone()),
            "Invalid PIN".to_string(),
        );
        return Err("Invalid PIN".to_string());
    }
    
    // 5. Refund crypto to user
    let (ckbtc_delta, ckusdc_delta) = logic::escrow_logic::calculate_escrow_refund_delta(escrow.amount, escrow.crypto_type);
    services::data_client::update_crypto_balance(&user_id, ckbtc_delta, ckusdc_delta).await?;
    
    // 6. Update escrow status
    services::data_client::update_escrow_status(&code, EscrowStatus::Cancelled).await?;
    
    // 7. Store transaction
    let tx_id = logic::transfer_logic::generate_transaction_id(current_time);
    let transaction = Transaction {
        id: tx_id,
        transaction_type: TransactionType::SellCrypto,
        from_user: Some(user_id.clone()),
        to_user: Some(escrow.agent_id.clone()),
        amount: escrow.amount,
        currency_type: CurrencyType::Crypto(escrow.crypto_type),
        status: TransactionStatus::Cancelled,
        created_at: current_time,
        completed_at: Some(current_time),
        description: Some(format!("Escrow cancelled: {}", code)),
    };
    
    services::data_client::store_transaction(&transaction).await?;
    
    // 8. Log success
    audit::log_success(
        "cancel_escrow",
        Some(user_id),
        format!("User cancelled escrow: {}", code),
    );
    
    Ok(())
}

/// Get escrow status
#[update]
async fn get_escrow(code: String) -> Result<Escrow, String> {
    config::verify_authorized_caller()?;
    
    services::data_client::get_escrow(&code).await?
        .ok_or_else(|| format!("Escrow not found: {}", code))
}

// ============================================================================
// Balance Operations
// ============================================================================

/// Get fiat balance (proxy to data_canister)
#[update]
async fn get_fiat_balance(user_id: String, currency: FiatCurrency) -> Result<u64, String> {
    config::verify_authorized_caller()?;
    services::data_client::get_fiat_balance(&user_id, currency).await
}

/// Set fiat balance (proxy to data_canister, for testing/admin)
#[update]
async fn set_fiat_balance(user_id: String, currency: FiatCurrency, amount: u64) -> Result<(), String> {
    config::verify_authorized_caller()?;
    services::data_client::set_fiat_balance(&user_id, currency, amount).await
}

/// Add to fiat balance (for deposits)
#[update]
async fn add_fiat_balance(user_id: String, amount: u64, currency: FiatCurrency) -> Result<u64, String> {
    config::verify_authorized_caller()?;
    
    let current = services::data_client::get_fiat_balance(&user_id, currency).await?;
    let new_balance = current.saturating_add(amount);
    services::data_client::set_fiat_balance(&user_id, currency, new_balance).await?;
    
    Ok(new_balance)
}

/// Deduct from fiat balance (for withdrawals)
#[update]
async fn deduct_fiat_balance(user_id: String, amount: u64, currency: FiatCurrency) -> Result<u64, String> {
    config::verify_authorized_caller()?;
    
    let current = services::data_client::get_fiat_balance(&user_id, currency).await?;
    if current < amount {
        return Err("Insufficient balance".to_string());
    }
    let new_balance = current - amount;
    services::data_client::set_fiat_balance(&user_id, currency, new_balance).await?;
    
    Ok(new_balance)
}

// ============================================================================
// TRANSACTION HISTORY ENDPOINTS
// ============================================================================

/// Get user transaction history (paginated)
#[update]
async fn get_transaction_history(
    user_id: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<Transaction>, String> {
    config::verify_authorized_caller()?;
    
    services::data_client::get_user_transactions(&user_id, limit, offset).await
}

// ============================================================================
// QUERY ENDPOINTS
// ============================================================================

/// Get wallet configuration (for debugging)
#[query]
fn get_config_info() -> String {
    let config = config::get_config();
    format!(
        "Transfer fee: {} bps, Exchange fee: {} bps, Escrow expiration: {} ns",
        config.fees.transfer_fee_basis_points,
        config.fees.exchange_fee_basis_points,
        config.escrow.expiration_time_ns
    )
}
