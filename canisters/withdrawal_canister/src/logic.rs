use candid::Principal;
use crate::{WithdrawalTransaction, TransactionStatus};

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
        return Err("Withdrawal already processed".to_string());
    }
    Ok(())
}

pub fn validate_agent_matches(withdrawal_agent: Principal, request_agent: Principal) -> Result<(), String> {
    if withdrawal_agent != request_agent {
        return Err("Wrong agent".to_string());
    }
    Ok(())
}


pub fn calculate_platform_fee(amount: u64, basis_points: u64) -> Result<u64, String> {
    amount.checked_mul(basis_points)
        .and_then(|v| v.checked_div(10000))
        .ok_or_else(|| "Fee calculation overflow".to_string())
}

pub fn calculate_agent_fee(amount: u64, basis_points: u64) -> Result<u64, String> {
    amount.checked_mul(basis_points)
        .and_then(|v| v.checked_div(10000))
        .ok_or_else(|| "Fee calculation overflow".to_string())
}


pub fn generate_withdrawal_code(id: u64) -> String {
    format!("WTH{:06}", id)
}

pub fn validate_withdrawal_code_format(code: &str) -> bool {
    code.starts_with("WTH") && code.len() == 9
}


pub fn create_withdrawal_transaction(
    id: u64,
    user_principal: Principal,
    agent_principal: Principal,
    amount_ugx: u64,
    platform_fee_ugx: u64,
    agent_fee_ugx: u64,
    withdrawal_code: String,
    timestamp: u64,
) -> WithdrawalTransaction {
    WithdrawalTransaction {
        id,
        user_principal,
        agent_principal,
        amount_ugx,
        platform_fee_ugx,
        agent_fee_ugx,
        withdrawal_code,
        timestamp,
        status: TransactionStatus::Pending,
    }
}

pub fn confirm_transaction_status(mut transaction: WithdrawalTransaction) -> WithdrawalTransaction {
    transaction.status = TransactionStatus::Confirmed;
    transaction
}


pub fn calculate_agent_earnings_update(
    current_total_withdrawals: u64,
    current_total_fees: u64,
    withdrawal_amount: u64,
    agent_fee: u64,
) -> Result<(u64, u64), String> {
    let new_total_withdrawals = current_total_withdrawals
        .checked_add(withdrawal_amount)
        .ok_or_else(|| "Withdrawal total overflow".to_string())?;
    
    let new_total_fees = current_total_fees
        .checked_add(agent_fee)
        .ok_or_else(|| "Fee total overflow".to_string())?;
    
    Ok((new_total_withdrawals, new_total_fees))
}

pub fn calculate_available_earnings(total_earned: u64, total_withdrawn: u64) -> u64 {
    total_earned.saturating_sub(total_withdrawn)
}
