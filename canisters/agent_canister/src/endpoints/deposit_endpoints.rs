// ============================================================================
// Deposit Endpoints - Agent Canister
// ============================================================================
// Handles deposit request creation and confirmation
// ============================================================================

use ic_cdk_macros::{update, query};
use shared_types::audit;
use crate::config;
use crate::logic::{deposit_logic, fraud_detection};
use crate::services::{data_client, user_client, wallet_client};

// ============================================================================
// Request/Response Types
// ============================================================================

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateDepositRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub pin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateDepositResponse {
    pub deposit_code: String,
    pub amount: u64,
    pub currency: String,
    pub agent_commission: u64,
    pub net_to_user: u64,
    pub expires_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ConfirmDepositRequest {
    pub deposit_code: String,
    pub agent_id: String,
    pub agent_pin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ConfirmDepositResponse {
    pub deposit_code: String,
    pub user_id: String,
    pub amount: u64,
    pub currency: String,
    pub agent_commission: u64,
    pub confirmed_at: u64,
}

// ============================================================================
// Deposit Endpoints
// ============================================================================

/// Create deposit request (user brings cash to agent)
#[update]
pub async fn create_deposit_request(request: CreateDepositRequest) -> Result<CreateDepositResponse, String> {
    // Validate authorization
    if !config::is_authorized() {
        return Err("Unauthorized: Only authorized canisters can call this endpoint".to_string());
    }
    
    audit::log_success("create_deposit_request", Some(request.user_id.clone()), "Starting deposit request".to_string());
    
    // Validate currency
    deposit_logic::validate_currency(&request.currency)?;
    
    // Validate amount
    deposit_logic::validate_deposit_amount(request.amount, &request.currency)?;
    
    // Verify user exists
    let user_exists = user_client::user_exists(&request.user_id).await?;
    if !user_exists {
        audit::log_failure("create_deposit_request", Some(request.user_id.clone()), "User not found".to_string());
        return Err("User not found".to_string());
    }
    
    // Verify agent exists
    let agent_exists = user_client::user_exists(&request.agent_id).await?;
    if !agent_exists {
        audit::log_failure("create_deposit_request", Some(request.user_id.clone()), "Agent not found".to_string());
        return Err("Agent not found".to_string());
    }
    
    // Verify user PIN
    let pin_valid = user_client::verify_pin(&request.user_id, &request.pin).await?;
    if !pin_valid {
        audit::log_failure("create_deposit_request", Some(request.user_id.clone()), "Invalid PIN".to_string());
        return Err("Invalid PIN".to_string());
    }
    
    // FRAUD DETECTION: Load agent activity from data_canister
    let now = ic_cdk::api::time();
    let mut agent_activity = match data_client::get_agent_activity(&request.agent_id, &request.currency).await? {
        Some(shared_activity) => {
            // Convert from storage format to fraud detection format
            fraud_detection::AgentActivity::from_shared(shared_activity)
        }
        None => {
            // First transaction for this agent/currency - create new activity tracker
            fraud_detection::AgentActivity::new(request.agent_id.clone(), now)
        }
    };

    // Run fraud detection checks BEFORE recording operation
    let fraud_result = fraud_detection::check_deposit_fraud(&agent_activity, &request.user_id, request.amount);
    if fraud_result.should_block {
        audit::log_failure(
            "create_deposit_request",
            Some(request.user_id.clone()),
            format!("FRAUD BLOCKED - Deposit blocked by fraud detection: {:?}", fraud_result.warnings)
        );
        return Err(format!("Transaction blocked: {}", fraud_result.warnings.join(", ")));
    }

    // Log warnings if any (non-blocking)
    if !fraud_result.warnings.is_empty() {
        audit::log_failure(
            "create_deposit_request",
            Some(request.user_id.clone()),
            format!("FRAUD WARNING - Suspicious patterns detected: {:?}", fraud_result.warnings)
        );
    }

    // Record this operation in activity tracker
    agent_activity.record_operation(&request.user_id, true, request.amount, now);

    // Store updated activity back to data_canister
    let shared_activity = agent_activity.to_shared(request.currency.clone());
    data_client::store_agent_activity(shared_activity).await?;
    
    // Calculate fees
    let fees = deposit_logic::calculate_deposit_fees(request.amount)?;
    
    // Generate deposit code
    let now = ic_cdk::api::time();
    let deposit_id = now / 1_000_000; // Use timestamp as ID
    let agent_prefix = &request.agent_id[..std::cmp::min(6, request.agent_id.len())];
    let deposit_code = deposit_logic::generate_deposit_code(deposit_id, agent_prefix, now);
    
    // Calculate expiration (24 hours from now)
    let cfg = config::get_config();
    let expires_at = ic_cdk::api::time() + cfg.codes.code_expiration_ns;
    
    // Create deposit transaction
    let deposit_tx = shared_types::DepositTransaction {
        id: format!("deposit_{}", deposit_id),
        user_id: request.user_id.clone(),
        agent_id: request.agent_id.clone(),
        amount: request.amount,
        currency: request.currency.clone(),
        agent_commission: fees.agent_commission,
        agent_keeps: fees.agent_keeps,
        platform_revenue: fees.total_platform_revenue,
        deposit_code: deposit_code.clone(),
        status: shared_types::AgentTransactionStatus::Pending,
        timestamp: ic_cdk::api::time(),
        confirmed_at: None,
    };
    
    // Store in data canister
    data_client::store_deposit_transaction(deposit_tx).await?;
    
    audit::log_success(
        "create_deposit_request",
        Some(request.user_id.clone()),
        format!("Deposit request created: code={}, amount={} {}", deposit_code, request.amount, request.currency)
    );
    
    Ok(CreateDepositResponse {
        deposit_code,
        amount: request.amount,
        currency: request.currency,
        agent_commission: fees.agent_commission,
        net_to_user: fees.net_to_user_balance,
        expires_at,
    })
}

/// Confirm deposit (agent confirms they received cash from user)
#[update]
pub async fn confirm_deposit(request: ConfirmDepositRequest) -> Result<ConfirmDepositResponse, String> {
    // Validate authorization
    if !config::is_authorized() {
        return Err("Unauthorized: Only authorized canisters can call this endpoint".to_string());
    }
    
    audit::log_success("confirm_deposit", None, format!("Confirming deposit: {}", request.deposit_code));
    
    // Validate deposit code format
    deposit_logic::validate_deposit_code_format(&request.deposit_code)?;
    
    // Get deposit from data canister
    let deposit = data_client::get_deposit_by_code(&request.deposit_code).await?
        .ok_or_else(|| "Deposit not found".to_string())?;
    
    // Verify deposit is pending
    if deposit.status != shared_types::AgentTransactionStatus::Pending {
        return Err(format!("Deposit is not pending (status: {:?})", deposit.status));
    }
    
    // Verify agent matches
    if deposit.agent_id != request.agent_id {
        audit::log_failure("confirm_deposit", Some(deposit.user_id.clone()), "Agent mismatch".to_string());
        return Err("Agent ID mismatch".to_string());
    }
    
    // Verify agent PIN
    let pin_valid = user_client::verify_pin(&request.agent_id, &request.agent_pin).await?;
    if !pin_valid {
        audit::log_failure("confirm_deposit", Some(deposit.user_id.clone()), "Invalid agent PIN".to_string());
        return Err("Invalid agent PIN".to_string());
    }
    
    // Check if expired
    let cfg = config::get_config();
    let now = ic_cdk::api::time();
    if now > deposit.timestamp + cfg.codes.code_expiration_ns {
        // Mark as expired
        data_client::update_deposit_status(&request.deposit_code, shared_types::AgentTransactionStatus::Expired).await?;
        return Err("Deposit code has expired".to_string());
    }
    
    // Update deposit status to confirmed
    let confirmed_deposit = data_client::update_deposit_status(
        &request.deposit_code,
        shared_types::AgentTransactionStatus::Confirmed
    ).await?;
    
    // Add balance to user (net amount after commission)
    let net_amount = deposit.amount - deposit.agent_commission;
    wallet_client::add_fiat_balance(&deposit.user_id, net_amount, &deposit.currency).await?;
    
    // Update agent balance (CREDIT SYSTEM)
    let _agent_balance_key = format!("{}:{}", deposit.agent_id, deposit.currency);
    let mut agent_balance = data_client::get_agent_balance(&deposit.agent_id, &deposit.currency).await?
        .unwrap_or_else(|| shared_types::AgentBalance {
            agent_id: deposit.agent_id.clone(),
            currency: deposit.currency.clone(),
            total_deposits: 0,
            total_withdrawals: 0,
            commission_earned: 0,
            commission_paid: 0,
            outstanding_balance: 0,  // NEW: starts at 0
            credit_limit: shared_types::AgentTier::New.default_credit_limit(),  // NEW: default to New tier (1M)
            last_settlement_date: None,
            last_updated: now,
        });
    
    agent_balance.total_deposits += 1;  // Increment COUNT of deposits (not amount)
    agent_balance.commission_earned += deposit.agent_keeps;
    
    // NEW: Agent owes platform (outstanding_balance decreases)
    // Agent credited user's fiat balance but hasn't sent cash to platform yet
    agent_balance.outstanding_balance -= net_amount as i64;
    
    agent_balance.last_updated = now;
    
    // NEW: Check if agent exceeded credit limit
    if agent_balance.outstanding_balance.abs() as u64 > agent_balance.credit_limit {
        return Err(format!(
            "Agent credit limit exceeded. Outstanding: {}, Limit: {}. Agent must settle before processing more deposits.",
            agent_balance.outstanding_balance.abs(),
            agent_balance.credit_limit
        ));
    }
    
    data_client::update_agent_balance(agent_balance).await?;
    
    audit::log_success(
        "confirm_deposit",
        Some(deposit.user_id.clone()),
        format!("Deposit confirmed: code={}, amount={} {}, user_balance_added={}", 
            request.deposit_code, deposit.amount, deposit.currency, net_amount)
    );
    
    Ok(ConfirmDepositResponse {
        deposit_code: request.deposit_code,
        user_id: deposit.user_id,
        amount: deposit.amount,
        currency: deposit.currency,
        agent_commission: deposit.agent_commission,
        confirmed_at: confirmed_deposit.confirmed_at.unwrap_or(now),
    })
}

/// Get deposit by code (query)
#[query]
pub async fn get_deposit_status(deposit_code: String) -> Result<shared_types::DepositTransaction, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    data_client::get_deposit_by_code(&deposit_code).await?
        .ok_or_else(|| "Deposit not found".to_string())
}

/// Get agent's deposits (query)
#[query]
pub async fn get_agent_deposits(agent_id: String) -> Result<Vec<shared_types::DepositTransaction>, String> {
    if !config::is_authorized() {
        return Err("Unauthorized".to_string());
    }
    
    data_client::get_agent_deposits(&agent_id).await
}
