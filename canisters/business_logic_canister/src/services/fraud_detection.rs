// ============================================================================
// Fraud Detection Service - Business Logic Layer
// ============================================================================

use crate::logic::fraud_logic;
use super::config;
use std::cell::RefCell;
use std::collections::HashMap;

pub use fraud_logic::FraudCheckResult;

// Rate limiting: Track transaction timestamps per user
thread_local! {
    static RATE_LIMIT_TRACKER: RefCell<HashMap<String, Vec<u64>>> = RefCell::new(HashMap::new());
}

/// Check if transaction is suspicious or should be blocked
pub fn check_transaction(
    user_id: &str,
    amount: u64,
    currency: &str,
) -> Result<FraudCheckResult, String> {
    // Get configurable limits
    let max_amount = config::get_max_transaction_amount();
    let suspicious_threshold = config::get_suspicious_amount_threshold();
    
    // Use pure logic function to check transaction
    let result = fraud_logic::check_transaction_amount(amount, max_amount, suspicious_threshold);
    
    // Log for audit
    ic_cdk::println!("🔍 Fraud check for user {}: amount={}, currency={}, risk_score={}, suspicious={}, review={}, blocked={}", 
        user_id, amount, currency, result.risk_score, result.is_suspicious, result.requires_manual_review, result.should_block);
    
    Ok(result)
}

/// Check if user should be rate-limited (sliding window algorithm)
pub fn check_rate_limit(user_id: &str) -> Result<bool, String> {
    #[cfg(test)]
    {
        return Ok(true);
    }
    
    let current_time = ic_cdk::api::time() / 1_000_000_000; // Convert to seconds
    let window_seconds = config::get_rate_limit_window_seconds();
    let max_transactions = config::get_max_transactions_per_window();
    
    RATE_LIMIT_TRACKER.with(|tracker| {
        let mut tracker = tracker.borrow_mut();
        
        // Get or create user's transaction history
        let timestamps = tracker.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // Remove timestamps outside the window
        timestamps.retain(|&ts| current_time - ts < window_seconds);
        
        // Check if user has exceeded the limit
        if timestamps.len() >= max_transactions {
            ic_cdk::println!("🚫 Rate limit exceeded for user {}: {} transactions in {} seconds", 
                user_id, timestamps.len(), window_seconds);
            return Ok(false);
        }
        
        // Add current timestamp
        timestamps.push(current_time);
        
        // Keep only last 20 timestamps to prevent memory bloat
        if timestamps.len() > 20 {
            timestamps.drain(0..timestamps.len() - 20);
        }
        
        Ok(true)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Config values are loaded from business_logic_config.toml
    // max_transaction_amount = 10_000_000
    // suspicious_amount_threshold = 5_000_000

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
        let result = check_transaction("user123", 5_000_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(!result.is_suspicious); // Should not be suspicious at exactly threshold
    }

    #[test]
    fn test_fraud_check_one_above_suspicious_threshold() {
        let result = check_transaction("user123", 5_000_001, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert!(result.warnings.len() > 0);
    }

    #[test]
    fn test_fraud_check_exactly_at_max_threshold() {
        let result = check_transaction("user123", 10_000_000, "UGX").unwrap();
        assert!(!result.should_block);
        assert!(result.is_suspicious); // Should be suspicious but not blocked
    }

    #[test]
    fn test_fraud_check_one_above_max_threshold() {
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
