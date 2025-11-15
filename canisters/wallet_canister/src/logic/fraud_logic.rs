/// Pure business logic for fraud detection
/// No I/O, no async, fully testable
///
/// NOTE: Some functions are prepared for future fraud detection enhancements:
/// - Daily transaction limits (config exists, not yet enforced in transfer flow)
/// - Velocity checks (multiple rapid transactions)
/// - Pattern-based fraud detection (round numbers, etc.)

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FraudCheckResult {
    pub is_suspicious: bool,
    pub risk_score: u8, // 0-100
    pub requires_manual_review: bool,
    pub should_block: bool,
    pub warnings: Vec<String>,
}

/// Check if transaction amount is suspicious based on thresholds
pub fn check_transaction_amount(
    amount: u64,
    max_amount: u64,
    suspicious_threshold: u64,
) -> FraudCheckResult {
    let mut warnings = Vec::new();
    let mut is_suspicious = false;
    let mut should_block = false;
    let mut risk_score: u8 = 0;
    let mut requires_manual_review = false;

    // Check 1: Amount too large - BLOCK
    if amount > max_amount {
        should_block = true;
        is_suspicious = true;
        risk_score = 100;
        requires_manual_review = true;
        warnings.push(format!(
            "Amount {} exceeds maximum limit {}",
            amount, max_amount
        ));
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

    FraudCheckResult {
        is_suspicious,
        risk_score,
        requires_manual_review,
        should_block,
        warnings,
    }
}

// Removed unused fraud detection functions (is_suspicious_amount, is_round_number, calculate_amount_risk_score)
// These were duplicates of logic in check_transaction_amount() and are not needed for current implementation.
// If pattern-based fraud detection is needed in the future, implement comprehensive logic instead of these helpers.

/// Check daily transaction limits
/// Used in transfer flow for fraud prevention (SECURITY_AUDIT.md recommendation #3)
pub fn check_daily_limits(
    transaction_count: usize,
    total_amount: u64,
    max_transactions: usize,
    max_amount: u64,
) -> FraudCheckResult {
    let mut warnings = Vec::new();
    let mut is_suspicious = false;
    let mut should_block = false;
    let mut risk_score: u8 = 0;
    let mut requires_manual_review = false;

    // Check transaction count
    if transaction_count >= max_transactions {
        should_block = true;
        is_suspicious = true;
        risk_score = 100;
        requires_manual_review = true;
        warnings.push(format!(
            "Daily transaction limit reached: {} >= {}",
            transaction_count, max_transactions
        ));
    }

    // Check total amount
    if total_amount >= max_amount {
        should_block = true;
        is_suspicious = true;
        risk_score = risk_score.max(100);
        requires_manual_review = true;
        warnings.push(format!(
            "Daily amount limit reached: {} >= {}",
            total_amount, max_amount
        ));
    }

    // Warning if approaching limits
    if transaction_count >= (max_transactions * 80) / 100 && !should_block {
        is_suspicious = true;
        risk_score = 50;
        warnings.push(format!(
            "Approaching daily transaction limit: {}/{}",
            transaction_count, max_transactions
        ));
    }

    if total_amount >= (max_amount * 80) / 100 && !should_block {
        is_suspicious = true;
        risk_score = risk_score.max(50);
        warnings.push(format!(
            "Approaching daily amount limit: {}/{}",
            total_amount, max_amount
        ));
    }

    FraudCheckResult {
        is_suspicious,
        risk_score,
        requires_manual_review,
        should_block,
        warnings,
    }
}

/// Check for velocity-based fraud (rapid successive transactions)
/// Detects if a user is making too many transactions too quickly
pub fn check_velocity(
    transaction_count_last_hour: usize,
    max_transactions_per_hour: usize,
) -> FraudCheckResult {
    let mut warnings = Vec::new();
    let mut is_suspicious = false;
    let mut should_block = false;
    let mut risk_score: u8 = 0;
    let mut requires_manual_review = false;

    // Check if velocity limit exceeded
    if transaction_count_last_hour >= max_transactions_per_hour {
        should_block = true;
        is_suspicious = true;
        risk_score = 100;
        requires_manual_review = true;
        warnings.push(format!(
            "Velocity limit exceeded: {} transactions in last hour (max: {})",
            transaction_count_last_hour, max_transactions_per_hour
        ));
    }
    // Warning if approaching velocity limit (80%)
    else if transaction_count_last_hour >= (max_transactions_per_hour * 80) / 100 {
        is_suspicious = true;
        risk_score = 60;
        warnings.push(format!(
            "Approaching velocity limit: {}/{} transactions in last hour",
            transaction_count_last_hour, max_transactions_per_hour
        ));
    }

    FraudCheckResult {
        is_suspicious,
        risk_score,
        requires_manual_review,
        should_block,
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Transaction Amount Check Tests
    // ============================================================================

    #[test]
    fn test_check_transaction_amount_normal() {
        let result = check_transaction_amount(1_000_000, 10_000_000, 5_000_000);
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
        assert!(!result.requires_manual_review);
        assert_eq!(result.risk_score, 0);
    }

    #[test]
    fn test_check_transaction_amount_medium() {
        let result = check_transaction_amount(3_000_000, 10_000_000, 5_000_000);
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
        assert!(!result.requires_manual_review);
        assert_eq!(result.risk_score, 30);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_check_transaction_amount_suspicious() {
        let result = check_transaction_amount(6_000_000, 10_000_000, 5_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert!(result.requires_manual_review);
        assert_eq!(result.risk_score, 70);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_check_transaction_amount_blocked() {
        let result = check_transaction_amount(11_000_000, 10_000_000, 5_000_000);
        assert!(result.should_block);
        assert!(result.is_suspicious);
        assert!(result.requires_manual_review);
        assert_eq!(result.risk_score, 100);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_check_transaction_amount_at_threshold() {
        let result = check_transaction_amount(5_000_000, 10_000_000, 5_000_000);
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
        assert_eq!(result.risk_score, 30);
    }

    #[test]
    fn test_check_transaction_amount_at_max() {
        let result = check_transaction_amount(10_000_000, 10_000_000, 5_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 70);
    }

    // Tests for removed helper functions (is_suspicious_amount, is_round_number, calculate_amount_risk_score)
    // These functions were redundant with check_transaction_amount() and have been removed

    // ============================================================================
    // Daily Limits Tests
    // ============================================================================

    #[test]
    fn test_check_daily_limits_normal() {
        let result = check_daily_limits(10, 1_000_000, 50, 10_000_000);
        assert!(!result.should_block);
        assert!(!result.is_suspicious);
        assert!(!result.requires_manual_review);
        assert_eq!(result.risk_score, 0);
    }

    #[test]
    fn test_check_daily_limits_approaching_count() {
        let result = check_daily_limits(42, 1_000_000, 50, 10_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 50);
        assert!(result.warnings.iter().any(|w| w.contains("transaction limit")));
    }

    #[test]
    fn test_check_daily_limits_approaching_amount() {
        let result = check_daily_limits(10, 8_500_000, 50, 10_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 50);
        assert!(result.warnings.iter().any(|w| w.contains("amount limit")));
    }

    #[test]
    fn test_check_daily_limits_exceeded_count() {
        let result = check_daily_limits(51, 1_000_000, 50, 10_000_000);
        assert!(result.should_block);
        assert!(result.is_suspicious);
        assert!(result.requires_manual_review);
        assert_eq!(result.risk_score, 100);
    }

    #[test]
    fn test_check_daily_limits_exceeded_amount() {
        let result = check_daily_limits(10, 11_000_000, 50, 10_000_000);
        assert!(result.should_block);
        assert!(result.is_suspicious);
        assert!(result.requires_manual_review);
        assert_eq!(result.risk_score, 100);
    }

    #[test]
    fn test_check_daily_limits_both_exceeded() {
        let result = check_daily_limits(51, 11_000_000, 50, 10_000_000);
        assert!(result.should_block);
        assert!(result.is_suspicious);
        assert!(result.requires_manual_review);
        assert_eq!(result.risk_score, 100);
        assert_eq!(result.warnings.len(), 2);
    }

    // ============================================================================
    // Combined Scenario Tests
    // ============================================================================

    #[test]
    fn test_fraud_check_comprehensive() {
        let max = 10_000_000u64;
        let threshold = 5_000_000u64;

        // Normal transaction
        let result = check_transaction_amount(1_000_000, max, threshold);
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 0);

        // Medium transaction
        let result = check_transaction_amount(3_000_000, max, threshold);
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 30);

        // Suspicious transaction
        let result = check_transaction_amount(7_000_000, max, threshold);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 70);

        // Blocked transaction
        let result = check_transaction_amount(15_000_000, max, threshold);
        assert!(result.should_block);
        assert_eq!(result.risk_score, 100);
    }

    // ============================================================================
    // Additional Security Tests - Boundary Conditions
    // ============================================================================

    #[test]
    fn test_velocity_exactly_at_limit() {
        let result = check_velocity(10, 10);
        assert!(result.should_block);
        assert_eq!(result.risk_score, 100);
    }

    #[test]
    fn test_velocity_one_below_limit() {
        let result = check_velocity(9, 10);
        // 9/10 = 90%, which is above 80% warning threshold
        assert!(!result.should_block);
        assert!(result.is_suspicious); // Should warn at 90%
        assert_eq!(result.risk_score, 60);
    }

    #[test]
    fn test_velocity_warning_at_80_percent() {
        let result = check_velocity(8, 10);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 60);
    }

    #[test]
    fn test_daily_limits_exactly_at_count() {
        let result = check_daily_limits(50, 1_000_000, 50, 10_000_000);
        assert!(result.should_block);
        assert_eq!(result.risk_score, 100);
    }

    #[test]
    fn test_daily_limits_exactly_at_amount() {
        let result = check_daily_limits(10, 10_000_000, 50, 10_000_000);
        assert!(result.should_block);
        assert_eq!(result.risk_score, 100);
    }

    #[test]
    fn test_daily_limits_warning_count_80_percent() {
        let result = check_daily_limits(40, 1_000_000, 50, 10_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 50);
    }

    #[test]
    fn test_daily_limits_warning_amount_80_percent() {
        let result = check_daily_limits(10, 8_000_000, 50, 10_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 50);
    }

    #[test]
    fn test_amount_exactly_at_max() {
        let result = check_transaction_amount(10_000_000, 10_000_000, 5_000_000);
        assert!(!result.should_block);
        assert!(result.is_suspicious);
        assert_eq!(result.risk_score, 70);
    }

    #[test]
    fn test_amount_one_over_max() {
        let result = check_transaction_amount(10_000_001, 10_000_000, 5_000_000);
        assert!(result.should_block);
        assert_eq!(result.risk_score, 100);
    }

    #[test]
    fn test_manual_review_required_when_blocked() {
        let r1 = check_transaction_amount(11_000_000, 10_000_000, 5_000_000);
        let r2 = check_daily_limits(51, 1_000_000, 50, 10_000_000);
        let r3 = check_velocity(11, 10);

        assert!(r1.requires_manual_review);
        assert!(r2.requires_manual_review);
        assert!(r3.requires_manual_review);
    }

    #[test]
    fn test_warning_messages_contain_values() {
        let r1 = check_transaction_amount(11_000_000, 10_000_000, 5_000_000);
        assert!(!r1.warnings.is_empty());
        assert!(r1.warnings[0].contains("11000000"));

        let r2 = check_velocity(11, 10);
        assert!(!r2.warnings.is_empty());
        assert!(r2.warnings[0].contains("11"));
    }
}
