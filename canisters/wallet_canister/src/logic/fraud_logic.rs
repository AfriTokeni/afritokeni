/// Pure business logic for fraud detection
/// No I/O, no async, fully testable

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

/// Check if amount is suspicious (above threshold)
pub fn is_suspicious_amount(amount: u64, threshold: u64) -> bool {
    amount >= threshold
}

/// Check if amount is a round number (potential indicator)
pub fn is_round_number(amount: u64) -> bool {
    amount % 10000 == 0 && amount > 0
}

/// Calculate risk score based on amount and threshold
pub fn calculate_amount_risk_score(
    amount: u64,
    max_amount: u64,
    suspicious_threshold: u64,
) -> u8 {
    if amount > max_amount {
        100
    } else if amount > suspicious_threshold {
        70
    } else if amount > suspicious_threshold / 2 {
        30
    } else {
        0
    }
}

/// Check daily transaction limits
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

    // ============================================================================
    // Suspicious Amount Tests
    // ============================================================================

    #[test]
    fn test_is_suspicious_amount_above_threshold() {
        assert!(is_suspicious_amount(1_000_000, 500_000));
        assert!(is_suspicious_amount(500_000, 500_000));
    }

    #[test]
    fn test_is_suspicious_amount_below_threshold() {
        assert!(!is_suspicious_amount(100_000, 500_000));
        assert!(!is_suspicious_amount(0, 500_000));
    }

    // ============================================================================
    // Round Number Tests
    // ============================================================================

    #[test]
    fn test_is_round_number_valid() {
        assert!(is_round_number(10000));
        assert!(is_round_number(100000));
        assert!(is_round_number(1000000));
    }

    #[test]
    fn test_is_round_number_invalid() {
        assert!(!is_round_number(10001));
        assert!(!is_round_number(99999));
        assert!(!is_round_number(0));
    }

    // ============================================================================
    // Risk Score Calculation Tests
    // ============================================================================

    #[test]
    fn test_calculate_amount_risk_score_normal() {
        assert_eq!(
            calculate_amount_risk_score(1_000_000, 10_000_000, 5_000_000),
            0
        );
    }

    #[test]
    fn test_calculate_amount_risk_score_medium() {
        assert_eq!(
            calculate_amount_risk_score(3_000_000, 10_000_000, 5_000_000),
            30
        );
    }

    #[test]
    fn test_calculate_amount_risk_score_suspicious() {
        assert_eq!(
            calculate_amount_risk_score(6_000_000, 10_000_000, 5_000_000),
            70
        );
    }

    #[test]
    fn test_calculate_amount_risk_score_blocked() {
        assert_eq!(
            calculate_amount_risk_score(11_000_000, 10_000_000, 5_000_000),
            100
        );
    }

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
}
