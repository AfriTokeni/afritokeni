use crate::models::*;
use crate::DataCanisterState;
use ic_cdk::api::time;

// Fraud detection thresholds
const MAX_TRANSACTIONS_PER_HOUR: usize = 50;
const MAX_AMOUNT_PER_TRANSACTION: u64 = 10_000_000; // 10M in smallest unit
const SUSPICIOUS_VELOCITY_THRESHOLD: usize = 10; // 10 txns in 5 minutes

/// Check if transaction looks suspicious
pub fn check_transaction_fraud(
    state: &DataCanisterState,
    user_id: &str,
    amount: u64,
    _transaction_type: TransactionType,
) -> Result<FraudCheckResult, String> {
    let now = time() / 1_000_000_000;
    let one_hour_ago = now.saturating_sub(3600);
    let five_minutes_ago = now.saturating_sub(300);
    
    // Get user's recent transactions
    let recent_transactions: Vec<&Transaction> = state.transactions.values()
        .filter(|tx| {
            (tx.from_user.as_ref() == Some(&user_id.to_string()) || 
             tx.to_user.as_ref() == Some(&user_id.to_string())) &&
            tx.created_at >= one_hour_ago
        })
        .collect();
    
    let mut warnings = Vec::new();
    let mut risk_score = 0u32;
    
    // Check 1: Amount too large
    if amount > MAX_AMOUNT_PER_TRANSACTION {
        warnings.push("Transaction amount exceeds maximum limit".to_string());
        risk_score += 50;
    }
    
    // Check 2: Too many transactions in last hour
    if recent_transactions.len() >= MAX_TRANSACTIONS_PER_HOUR {
        warnings.push(format!("High transaction frequency: {} in last hour", recent_transactions.len()));
        risk_score += 30;
    }
    
    // Check 3: Velocity check - too many in 5 minutes
    let very_recent: Vec<&Transaction> = recent_transactions.iter()
        .filter(|tx| tx.created_at >= five_minutes_ago)
        .copied()
        .collect();
    
    if very_recent.len() >= SUSPICIOUS_VELOCITY_THRESHOLD {
        warnings.push(format!("Suspicious velocity: {} transactions in 5 minutes", very_recent.len()));
        risk_score += 40;
    }
    
    // Check 4: First transaction is large
    let user = state.users.get(user_id);
    if let Some(u) = user {
        if u.created_at >= one_hour_ago && amount > 1_000_000 {
            warnings.push("Large first transaction from new user".to_string());
            risk_score += 25;
        }
    }
    
    // Check 5: Pattern detection - same amount multiple times
    let same_amount_count = recent_transactions.iter()
        .filter(|tx| tx.amount == amount)
        .count();
    
    if same_amount_count >= 5 {
        warnings.push(format!("Repeated amount pattern: {} times", same_amount_count));
        risk_score += 20;
    }
    
    // Determine risk level
    let risk_level = if risk_score >= 80 {
        RiskLevel::High
    } else if risk_score >= 50 {
        RiskLevel::Medium
    } else if risk_score >= 20 {
        RiskLevel::Low
    } else {
        RiskLevel::None
    };
    
    Ok(FraudCheckResult {
        is_suspicious: risk_score >= 50,
        risk_level,
        risk_score,
        warnings,
        requires_manual_review: risk_score >= 80,
    })
}

/// Check for account takeover patterns
pub fn check_account_takeover(
    state: &DataCanisterState,
    user_id: &str,
) -> Result<bool, String> {
    let now = time() / 1_000_000_000;
    let one_hour_ago = now.saturating_sub(3600);
    
    // Check PIN failures
    if let Some(pin_data) = state.user_pins.get(user_id) {
        // Multiple failed attempts recently
        if pin_data.failed_attempts >= 3 {
            return Ok(true);
        }
    }
    
    // Check for sudden change in transaction patterns
    let recent_transactions: Vec<&Transaction> = state.transactions.values()
        .filter(|tx| {
            (tx.from_user.as_ref() == Some(&user_id.to_string()) || 
             tx.to_user.as_ref() == Some(&user_id.to_string())) &&
            tx.created_at >= one_hour_ago
        })
        .collect();
    
    // If user suddenly has many outgoing transactions
    let outgoing_count = recent_transactions.iter()
        .filter(|tx| tx.from_user.as_ref() == Some(&user_id.to_string()))
        .count();
    
    if outgoing_count >= 20 {
        return Ok(true);
    }
    
    Ok(false)
}

/// Rate limiting check
pub fn check_rate_limit(
    state: &DataCanisterState,
    user_id: &str,
    max_per_minute: usize,
) -> Result<bool, String> {
    let now = time() / 1_000_000_000;
    let one_minute_ago = now.saturating_sub(60);
    
    let recent_count = state.transactions.values()
        .filter(|tx| {
            (tx.from_user.as_ref() == Some(&user_id.to_string()) || 
             tx.to_user.as_ref() == Some(&user_id.to_string())) &&
            tx.created_at >= one_minute_ago
        })
        .count();
    
    Ok(recent_count < max_per_minute)
}

// ============================================================================
// Fraud Check Result Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct FraudCheckResult {
    pub is_suspicious: bool,
    pub risk_level: RiskLevel,
    pub risk_score: u32,
    pub warnings: Vec<String>,
    pub requires_manual_review: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
}

impl FraudCheckResult {
    pub fn should_block(&self) -> bool {
        self.risk_level == RiskLevel::High
    }
    
    pub fn should_flag(&self) -> bool {
        self.risk_level == RiskLevel::Medium || self.risk_level == RiskLevel::High
    }
}
