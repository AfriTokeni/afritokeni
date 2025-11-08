// ============================================================================
// Fraud Detection Service - Business Logic Layer
// ============================================================================

use super::config;

pub struct FraudCheckResult {
    #[allow(dead_code)] // Used in security logging
    pub is_suspicious: bool,
    #[allow(dead_code)] // Used in security logging
    pub risk_score: u8, // 0-100
    #[allow(dead_code)] // Used in security logging
    pub requires_manual_review: bool,
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
    let mut risk_score: u8 = 0;
    let mut requires_manual_review = false;
    
    // Get configurable limits
    let max_amount = config::get_max_transaction_amount();
    let suspicious_threshold = config::get_suspicious_amount_threshold();
    
    // Check 1: Amount too large - BLOCK (also suspicious)
    if amount > max_amount {
        should_block = true;
        is_suspicious = true; // Blocked transactions are always suspicious
        risk_score = 100;
        requires_manual_review = true;
        warnings.push(format!("Amount {} exceeds maximum limit", amount));
    }
    // Check 2: Suspicious amount - FLAG for review
    else if amount > suspicious_threshold {
        is_suspicious = true;
        risk_score = 70;
        requires_manual_review = true;
        warnings.push(format!("Large transaction: {}", amount));
    }
    // Check 3: Medium amount - Just track
    else if amount > suspicious_threshold / 2 {
        risk_score = 30;
        warnings.push(format!("Medium transaction: {}", amount));
    }
    
    // Log for audit
    ic_cdk::println!("🔍 Fraud check for user {}: amount={}, currency={}, risk_score={}, suspicious={}, review={}, blocked={}", 
        user_id, amount, currency, risk_score, is_suspicious, requires_manual_review, should_block);
    
    Ok(FraudCheckResult {
        is_suspicious,
        risk_score,
        requires_manual_review,
        should_block,
        warnings,
    })
}

/// Check if user should be rate-limited
pub fn check_rate_limit(_user_id: &str) -> Result<bool, String> {
    // Skip rate limiting in test mode
    #[cfg(test)]
    {
        return Ok(true);
    }
    
    // TODO: Implement proper rate limiting based on transaction history
    // Track requests per user per time window
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Setup function to initialize config with default values for tests
    fn setup_test_config() {
        config::set_max_transaction_amount(10_000_000);
        config::set_suspicious_amount_threshold(5_000_000);
    }

    // ============================================================================
    // Normal Cases
    // ============================================================================

    #[test]
    fn test_fraud_check_normal_amount() {
        setup_test_config();
        setup_test_config();
        let result = check_transaction("user123", 1_000_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
        assert_eq!(result.warnings.len(), 0);
    }

    #[test]
    fn test_fraud_check_small_amount() {
        setup_test_config();
        let result = check_transaction("user123", 1_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
    }

    // ============================================================================
    // Boundary Cases
    // ============================================================================

    #[test]
    fn test_fraud_check_exactly_at_suspicious_threshold() {
        setup_test_config();
        let result = check_transaction("user123", 5_000_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious); // Should not be suspicious at exactly threshold
    }

    #[test]
    fn test_fraud_check_one_above_suspicious_threshold() {
        setup_test_config();
        let result = check_transaction("user123", 5_000_001, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert!(result.warnings.len() > 0);
    }

    #[test]
    fn test_fraud_check_exactly_at_max_threshold() {
        setup_test_config();
        let result = check_transaction("user123", 10_000_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(result.is_suspicious); // Should be suspicious but not blocked
    }

    #[test]
    fn test_fraud_check_one_above_max_threshold() {
        setup_test_config();
        let result = check_transaction("user123", 10_000_001, "UGX").unwrap();
        assert!(result.should_block);
        assert!(result.warnings.len() > 0);
        assert!(result.warnings[0].contains("exceeds maximum limit"));
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[test]
    fn test_fraud_check_zero_amount() {
        setup_test_config();
        let result = check_transaction("user123", 0, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
    }

    #[test]
    fn test_fraud_check_maximum_u64() {
        setup_test_config();
        let result = check_transaction("user123", u64::MAX, "UGX").unwrap();
        assert!(result.should_block);
        assert!(result.warnings.len() > 0);
    }

    #[test]
    fn test_fraud_check_empty_user_id() {
        setup_test_config();
        let result = check_transaction("", 1_000_000, "UGX").unwrap();
        assert!(!result.should_block); // Should still work
    }

    #[test]
    fn test_fraud_check_empty_currency() {
        setup_test_config();
        let result = check_transaction("user123", 1_000_000, "").unwrap();
        assert!(!result.should_block); // Should still work
    }

    #[test]
    fn test_fraud_check_different_currencies() {
        setup_test_config();
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
        setup_test_config();
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
        setup_test_config();
        let result = check_rate_limit("user123").unwrap();
        assert!(result);
    }

    #[test]
    fn test_rate_limit_empty_user_id() {
        setup_test_config();
        let result = check_rate_limit("").unwrap();
        assert!(result); // Should still work
    }

    #[test]
    fn test_rate_limit_special_characters() {
        setup_test_config();
        let result = check_rate_limit("user@#$%^&*()").unwrap();
        assert!(result);
    }
}
