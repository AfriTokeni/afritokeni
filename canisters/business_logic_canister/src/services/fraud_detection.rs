// ============================================================================
// Fraud Detection Service - Business Logic Layer
// ============================================================================

const MAX_TRANSACTION_AMOUNT: u64 = 10_000_000; // 10M in smallest unit (e.g., 100,000 UGX)
const SUSPICIOUS_AMOUNT_THRESHOLD: u64 = 5_000_000; // 5M in smallest unit

pub struct FraudCheckResult {
    pub is_suspicious: bool,
    pub should_block: bool,
    pub warnings: Vec<String>,
}

/// Check if transaction is suspicious or should be blocked
pub fn check_transaction(
    user_id: &str,
    amount: u64,
    currency: &str,
) -> Result<FraudCheckResult, String> {
    let mut warnings = Vec::new();
    let mut is_suspicious = false;
    let mut should_block = false;
    
    // Check 1: Amount too large
    if amount > MAX_TRANSACTION_AMOUNT {
        should_block = true;
        warnings.push(format!("Amount {} exceeds maximum limit", amount));
    }
    
    // Check 2: Suspicious amount
    if amount > SUSPICIOUS_AMOUNT_THRESHOLD {
        is_suspicious = true;
        warnings.push(format!("Large transaction: {}", amount));
    }
    
    // Log for audit
    ic_cdk::println!("🔍 Fraud check for user {}: amount={}, currency={}, suspicious={}, blocked={}", 
        user_id, amount, currency, is_suspicious, should_block);
    
    Ok(FraudCheckResult {
        is_suspicious,
        should_block,
        warnings,
    })
}

/// Check if user should be rate-limited
pub fn check_rate_limit(user_id: &str) -> Result<bool, String> {
    // TODO: Implement rate limiting based on transaction history
    // For now, allow all
    Ok(true)
}
