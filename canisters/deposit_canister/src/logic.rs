use candid::Principal;
use crate::{DepositTransaction, TransactionStatus};

pub fn validate_caller_is_user(caller: Principal, user: Principal) -> Result<(), String> {
    if caller != user {
        return Err("Caller must be the user".to_string());
    }
    Ok(())
}

pub fn validate_amount_positive(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    Ok(())
}

pub fn validate_caller_is_agent(caller: Principal, agent: Principal) -> Result<(), String> {
    if caller != agent {
        return Err("Only the assigned agent can confirm".to_string());
    }
    Ok(())
}

pub fn validate_status_is_pending(status: &TransactionStatus) -> Result<(), String> {
    if *status != TransactionStatus::Pending {
        return Err("Deposit already processed".to_string());
    }
    Ok(())
}

pub fn validate_agent_matches(deposit_agent: Principal, request_agent: Principal) -> Result<(), String> {
    if deposit_agent != request_agent {
        return Err("Wrong agent".to_string());
    }
    Ok(())
}

pub fn calculate_commission(amount: u64, basis_points: u64) -> Result<u64, String> {
    amount.checked_mul(basis_points)
        .and_then(|v| v.checked_div(10000))
        .ok_or_else(|| "Commission calculation overflow".to_string())
}

pub fn generate_deposit_code(id: u64) -> String {
    format!("DEP{:06}", id)
}

pub fn validate_deposit_code_format(code: &str) -> bool {
    code.starts_with("DEP") && code.len() == 9
}

pub fn create_deposit_transaction(
    id: u64,
    user_principal: Principal,
    agent_principal: Principal,
    amount_ugx: u64,
    commission_ugx: u64,
    deposit_code: String,
    timestamp: u64,
) -> DepositTransaction {
    DepositTransaction {
        id,
        user_principal,
        agent_principal,
        amount_ugx,
        commission_ugx,
        deposit_code,
        timestamp,
        status: TransactionStatus::Pending,
    }
}

pub fn confirm_transaction_status(mut transaction: DepositTransaction) -> DepositTransaction {
    transaction.status = TransactionStatus::Confirmed;
    transaction
}

pub fn calculate_agent_balance_update(
    current_total_deposits: u64,
    current_commission_owed: u64,
    deposit_amount: u64,
    commission: u64,
) -> Result<(u64, u64), String> {
    let new_total_deposits = current_total_deposits
        .checked_add(deposit_amount)
        .ok_or_else(|| "Deposit total overflow".to_string())?;
    
    let new_commission_owed = current_commission_owed
        .checked_add(commission)
        .ok_or_else(|| "Commission total overflow".to_string())?;
    
    Ok((new_total_deposits, new_commission_owed))
}

pub fn calculate_commission_due(total_owed: u64, total_paid: u64) -> u64 {
    total_owed.saturating_sub(total_paid)
}
