// ============================================================================
// Withdrawal Endpoints - Agent Canister
// ============================================================================
// Handles withdrawal request creation and confirmation
// ============================================================================

use ic_cdk_macros::{update, query};
use shared_types::audit;
use crate::config;
use crate::logic::{withdrawal_logic, fraud_detection};
use crate::services::{data_client, user_client, wallet_client};

// ============================================================================
// Request/Response Types
// ============================================================================

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateWithdrawalRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub pin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateWithdrawalResponse {
    pub withdrawal_code: String,
    pub amount: u64,
    pub currency: String,
    pub total_fees: u64,
    pub net_to_user: u64,
    pub expires_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ConfirmWithdrawalRequest {
    pub withdrawal_code: String,
    pub agent_id: String,
    pub agent_pin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ConfirmWithdrawalResponse {
    pub withdrawal_code: String,
    pub user_id: String,
    pub amount: u64,
    pub currency: String,
    pub total_fees: u64,
    pub confirmed_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WithdrawalFeesResponse {
    pub amount: u64,
    pub agent_fee: u64,
    pub platform_fee: u64,
    pub total_fees: u64,
    pub net_to_user: u64,
}

// ============================================================================
// Withdrawal Endpoints
// ============================================================================

/// Get withdrawal fees estimate
#[query]
pub fn get_withdrawal_fees(amount: u64) -> Result<WithdrawalFeesResponse, String> {
    let fees = withdrawal_logic::calculate_withdrawal_fees(amount)?;
    
    Ok(WithdrawalFeesResponse {
        amount,
        agent_fee: fees.agent_fee,
        platform_fee: fees.platform_operation_fee,
        total_fees: fees.total_fees,
        net_to_user: fees.net_to_agent,
    })
}

/// Create withdrawal request (user wants cash from agent)
#[update]
pub async fn create_withdrawal_request(request: CreateWithdrawalRequest) -> Result<CreateWithdrawalResponse, String> {
    // Validate authorization
    if !config::is_authorized() {
        return Err("Unauthorized: Only authorized canisters can call this endpoint".to_string());
    }
    
    audit::log_success("create_withdrawal_request", Some(request.user_id.clone()), "Starting withdrawal request".to_string());
    
    // Validate currency
    withdrawal_logic::validate_currency(&request.currency)?;
    
    // Validate amount
    withdrawal_logic::validate_withdrawal_amount(request.amount, &request.currency)?;
    
    // Verify user exists
    let user_exists = user_client::user_exists(&request.user_id).await?;
    if !user_exists {
        audit::log_failure("create_withdrawal_request", Some(request.user_id.clone()), "User not found".to_string());
        return Err("User not found".to_string());
    }
    
    // Verify agent exists
    let agent_exists = user_client::user_exists(&request.agent_id).await?;
    if !agent_exists {
        audit::log_failure("create_withdrawal_request", Some(request.user_id.clone()), "Agent not found".to_string());
        return Err("Agent not found".to_string());
    }
    
    // Verify user PIN
    let pin_valid = user_client::verify_pin(&request.user_id, &request.pin).await?;
    if !pin_valid {
        audit::log_failure("create_withdrawal_request", Some(request.user_id.clone()), "Invalid PIN".to_string());
        return Err("Invalid PIN".to_string());
    }
    
    // Calculate fees
    let fees = withdrawal_logic::calculate_withdrawal_fees(request.amount)?;
    
    // Check user balance (must have amount + fees)
    let user_balance = wallet_client::get_fiat_balance(&request.user_id, &request.currency).await?;
    withdrawal_logic::validate_sufficient_balance(user_balance, request.amount, fees.total_fees)?;
    
    // Get or create agent activity for fraud detection
    let now = ic_cdk::api::time();
    let agent_activity = fraud_detection::AgentActivity::new(request.agent_id.clone(), now);
    
    // Fraud check
    let fraud_result = fraud_detection::check_withdrawal_fraud(&agent_activity, &request.user_id, request.amount);
    if fraud_result.should_block {
        audit::log_failure(
            "create_withdrawal_request",
            Some(request.user_id.clone()),
            format!("Blocked by fraud detection: {:?}", fraud_result.warnings)
        );
        return Err(format!("Transaction blocked: {}", fraud_result.warnings.join(", ")));
    }
    
    // Log warnings if any
    if !fraud_result.warnings.is_empty() {
        audit::log_failure(
            "create_withdrawal_request",
            Some(request.user_id.clone()),
            format!("Fraud warnings: {:?}", fraud_result.warnings)
        );
    }
    
    // Generate withdrawal code
    let now = ic_cdk::api::time();
    let withdrawal_id = now / 1_000_000; // Use timestamp as ID
    let agent_prefix = &request.agent_id[..std::cmp::min(6, request.agent_id.len())];
    let withdrawal_code = withdrawal_logic::generate_withdrawal_code(withdrawal_id, agent_prefix, now);
    
    // Calculate expiration (24 hours from now)
    let cfg = config::get_config();
    let expires_at = ic_cdk::api::time() + cfg.codes.code_expiration_ns;
    
    // Create withdrawal transaction
    let withdrawal_tx = shared_types::WithdrawalTransaction {
        id: format!("withdrawal_{}", withdrawal_id),
        user_id: request.user_id.clone(),
        agent_id: request.agent_id.clone(),
        amount: request.amount,
        currency: request.currency.clone(),
        agent_fee: fees.agent_fee,
        agent_keeps: fees.agent_keeps,
        platform_revenue: fees.total_platform_revenue,
        withdrawal_code: withdrawal_code.clone(),
        status: shared_types::AgentTransactionStatus::Pending,
        timestamp: ic_cdk::api::time(),
        confirmed_at: None,
    };
    
    // Store in data canister
    data_client::store_withdrawal_transaction(withdrawal_tx).await?;
    
    // Deduct balance from user immediately (reserve for withdrawal)
    let total_deduction = request.amount + fees.total_fees;
    wallet_client::deduct_fiat_balance(&request.user_id, total_deduction, &request.currency).await?;
    
    audit::log_success(
        "create_withdrawal_request",
        Some(request.user_id.clone()),
        format!("Withdrawal request created: code={}, amount={} {}, fees={}", 
            withdrawal_code, request.amount, request.currency, fees.total_fees)
    );
    
    Ok(CreateWithdrawalResponse {
        withdrawal_code,
        amount: request.amount,
        currency: request.currency,
        total_fees: fees.total_fees,
        net_to_user: fees.net_to_agent,
        expires_at,
    })
}

/// Confirm withdrawal (agent confirms they gave cash to user)
#[update]
pub async fn confirm_withdrawal(request: ConfirmWithdrawalRequest) -> Result<ConfirmWithdrawalResponse, String> {
    // Validate authorization
    if !config::is_authorized() {
        return Err("Unauthorized: Only authorized canisters can call this endpoint".to_string());
    }
    
    audit::log_success("confirm_withdrawal", None, format!("Confirming withdrawal: {}", request.withdrawal_code));
    
    // Validate withdrawal code format
    withdrawal_logic::validate_withdrawal_code_format(&request.withdrawal_code)?;
    
    // Get withdrawal from data canister
    let withdrawal = data_client::get_withdrawal_by_code(&request.withdrawal_code).await?
        .ok_or_else(|| "Withdrawal not found".to_string())?;
    
    // Verify withdrawal is pending
    if withdrawal.status != shared_types::AgentTransactionStatus::Pending {
        return Err(format!("Withdrawal is not pending (status: {:?})", withdrawal.status));
    }
    
    // Verify agent matches
    if withdrawal.agent_id != request.agent_id {
        audit::log_failure("confirm_withdrawal", Some(withdrawal.user_id.clone()), "Agent mismatch".to_string());
        return Err("Agent ID mismatch".to_string());
    }
    
    // Verify agent PIN
    let pin_valid = user_client::verify_pin(&request.agent_id, &request.agent_pin).await?;
    if !pin_valid {
        audit::log_failure("confirm_withdrawal", Some(withdrawal.user_id.clone()), "Invalid agent PIN".to_string());
        return Err("Invalid agent PIN".to_string());
    }
    
    // Check if expired
    let cfg = config::get_config();
    let now = ic_cdk::api::time();
    if now > withdrawal.timestamp + cfg.codes.code_expiration_ns {
        // Mark as expired and refund user
        data_client::update_withdrawal_status(&request.withdrawal_code, shared_types::AgentTransactionStatus::Expired).await?;
        
        // Refund user (amount + fees)
        let total_refund = withdrawal.amount + withdrawal.agent_fee + withdrawal.platform_revenue;
        wallet_client::add_fiat_balance(&withdrawal.user_id, total_refund, &withdrawal.currency).await?;
        
        return Err("Withdrawal code has expired (balance refunded)".to_string());
    }
    
    // Update withdrawal status to confirmed
    let confirmed_withdrawal = data_client::update_withdrawal_status(
        &request.withdrawal_code,
        shared_types::AgentTransactionStatus::Confirmed
    ).await?;
    
    // Update agent balance (CREDIT SYSTEM)
    let mut agent_balance = data_client::get_agent_balance(&withdrawal.agent_id, &withdrawal.currency).await?
        .unwrap_or_else(|| shared_types::AgentBalance {
            agent_id: withdrawal.agent_id.clone(),
            currency: withdrawal.currency.clone(),
            total_deposits: 0,
            total_withdrawals: 0,
            commission_earned: 0,
            commission_paid: 0,
            outstanding_balance: 0,  // NEW: starts at 0
            credit_limit: shared_types::AgentTier::New.default_credit_limit(),  // NEW: default to New tier (1M)
            last_settlement_date: None,
            last_updated: now,
        });
    
    agent_balance.total_withdrawals += withdrawal.amount;  // Track total withdrawal amount
    agent_balance.commission_earned += withdrawal.agent_keeps;
    
    // NEW: Platform owes agent (outstanding_balance increases)
    // Agent gave cash to user from own funds, platform owes agent this amount
    agent_balance.outstanding_balance += withdrawal.amount as i64;
    
    agent_balance.last_updated = now;
    
    data_client::update_agent_balance(agent_balance).await?;
    
    let total_fees = withdrawal.agent_fee + withdrawal.platform_revenue;
    
    audit::log_success(
        "confirm_withdrawal",
        Some(withdrawal.user_id.clone()),
        format!("Withdrawal confirmed: code={}, amount={} {}, fees={}", 
            request.withdrawal_code, withdrawal.amount, withdrawal.currency, total_fees)
    );
    
    Ok(ConfirmWithdrawalResponse {
        withdrawal_code: request.withdrawal_code,
        user_id: withdrawal.user_id,
        amount: withdrawal.amount,
        currency: withdrawal.currency,
        total_fees,
        confirmed_at: confirmed_withdrawal.confirmed_at.unwrap_or(now),
    })
}

/// Cancel withdrawal (user cancels before agent confirms)
#[update]
pub async fn cancel_withdrawal(withdrawal_code: String, user_id: String, pin: String) -> Result<(), String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    audit::log_success("cancel_withdrawal", Some(user_id.clone()), format!("Cancelling withdrawal: {}", withdrawal_code));
    
    // Get withdrawal
    let withdrawal = data_client::get_withdrawal_by_code(&withdrawal_code).await?
        .ok_or_else(|| "Withdrawal not found".to_string())?;
    
    // Verify user matches
    if withdrawal.user_id != user_id {
        return Err("User ID mismatch".to_string());
    }
    
    // Verify PIN
    let pin_valid = user_client::verify_pin(&user_id, &pin).await?;
    if !pin_valid {
        return Err("Invalid PIN".to_string());
    }
    
    // Verify withdrawal is pending
    if withdrawal.status != shared_types::AgentTransactionStatus::Pending {
        return Err(format!("Cannot cancel withdrawal with status: {:?}", withdrawal.status));
    }
    
    // Update status to cancelled
    data_client::update_withdrawal_status(&withdrawal_code, shared_types::AgentTransactionStatus::Cancelled).await?;
    
    // Refund user (amount + fees)
    let total_refund = withdrawal.amount + withdrawal.agent_fee + withdrawal.platform_revenue;
    wallet_client::add_fiat_balance(&user_id, total_refund, &withdrawal.currency).await?;
    
    audit::log_success(
        "cancel_withdrawal",
        Some(user_id),
        format!("Withdrawal cancelled and refunded: code={}, refund={} {}", 
            withdrawal_code, total_refund, withdrawal.currency)
    );
    
    Ok(())
}

/// Get withdrawal by code (query)
#[query]
pub async fn get_withdrawal_status(withdrawal_code: String) -> Result<shared_types::WithdrawalTransaction, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    data_client::get_withdrawal_by_code(&withdrawal_code).await?
        .ok_or_else(|| "Withdrawal not found".to_string())
}

/// Get agent's withdrawals (query)
#[query]
pub async fn get_agent_withdrawals(agent_id: String) -> Result<Vec<shared_types::WithdrawalTransaction>, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    data_client::get_agent_withdrawals(&agent_id).await
}
