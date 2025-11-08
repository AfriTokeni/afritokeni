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

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Normal Cases
    // ============================================================================

    #[test]
    fn test_fraud_check_normal_amount() {
        let result = check_transaction("user123", 1_000_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
        assert_eq!(result.warnings.len(), 0);
    }

    #[test]
    fn test_fraud_check_small_amount() {
        let result = check_transaction("user123", 1_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
    }

    // ============================================================================
    // Boundary Cases
    // ============================================================================

    #[test]
    fn test_fraud_check_exactly_at_suspicious_threshold() {
        let result = check_transaction("user123", SUSPICIOUS_AMOUNT_THRESHOLD, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious); // Should not be suspicious at exactly threshold
    }

    #[test]
    fn test_fraud_check_one_above_suspicious_threshold() {
        let result = check_transaction("user123", SUSPICIOUS_AMOUNT_THRESHOLD + 1, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert!(result.warnings.len() > 0);
        assert!(result.warnings[0].contains("Large transaction"));
    }

    #[test]
    fn test_fraud_check_exactly_at_max_threshold() {
        let result = check_transaction("user123", MAX_TRANSACTION_AMOUNT, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(result.is_suspicious); // Should be suspicious but not blocked
    }

    #[test]
    fn test_fraud_check_one_above_max_threshold() {
        let result = check_transaction("user123", MAX_TRANSACTION_AMOUNT + 1, "UGX").unwrap();
        assert!(result.should_block);
        assert!(result.warnings.len() > 0);
        assert!(result.warnings[0].contains("exceeds maximum limit"));
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[test]
    fn test_fraud_check_zero_amount() {
        let result = check_transaction("user123", 0, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
    }

    #[test]
    fn test_fraud_check_maximum_u64() {
        let result = check_transaction("user123", u64::MAX, "UGX").unwrap();
        assert!(result.should_block);
        assert!(result.warnings.len() > 0);
    }

    #[test]
    fn test_fraud_check_empty_user_id() {
        let result = check_transaction("", 1_000_000, "UGX").unwrap();
        assert!(!result.should_block); // Should still work
    }

    #[test]
    fn test_fraud_check_empty_currency() {
        let result = check_transaction("user123", 1_000_000, "").unwrap();
        assert!(!result.should_block); // Should still work
    }

    #[test]
    fn test_fraud_check_different_currencies() {
        let ugx_result = check_transaction("user123", 5_000_000, "UGX").unwrap();
        let kes_result = check_transaction("user123", 5_000_000, "KES").unwrap();
        let ngn_result = check_transaction("user123", 5_000_000, "NGN").unwrap();
        
        // All should have same behavior (currency-agnostic for now)
        assert_eq!(ugx_result.is_suspicious, kes_result.is_suspicious);
        assert_eq!(ugx_result.is_suspicious, ngn_result.is_suspicious);
    }

    // ============================================================================
    // Multiple Warnings
    // ============================================================================

    #[test]
    fn test_fraud_check_very_large_amount_multiple_warnings() {
        let result = check_transaction("user123", 20_000_000, "UGX").unwrap();
        assert!(result.should_block);
        assert!(result.is_suspicious);
        // Should have warning about exceeding max
        assert!(result.warnings.iter().any(|w| w.contains("exceeds maximum")));
    }

    // ============================================================================
    // Rate Limiting
    // ============================================================================

    #[test]
    fn test_rate_limit_allows_transactions() {
        let result = check_rate_limit("user123").unwrap();
        assert!(result);
    }

    #[test]
    fn test_rate_limit_empty_user_id() {
        let result = check_rate_limit("").unwrap();
        assert!(result); // Should still work
    }

    #[test]
    fn test_rate_limit_special_characters() {
        let result = check_rate_limit("user@#$%^&*()").unwrap();
        assert!(result);
    }
}
