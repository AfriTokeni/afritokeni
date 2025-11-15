/// Transaction helper functions for buy_crypto and sell_crypto operations
/// This module extracts common patterns to reduce code duplication and complexity

use shared_types::{audit, CryptoType};

use crate::logic::fraud_detection;
use crate::services::{user_client, data_client};

/// Result of user verification check
#[allow(dead_code)]
pub struct UserVerificationResult {
    pub user_exists: bool,
}

/// Result of PIN verification with fraud detection
#[allow(dead_code)]
pub struct PinVerificationResult {
    pub verified: bool,
    pub fraud_check_passed: bool,
}

/// Result of comprehensive fraud detection check
#[allow(dead_code)]
pub struct FraudCheckResult {
    pub should_block: bool,
    pub requires_manual_review: bool,
    pub risk_score: u32,
    pub warnings: Vec<String>,
}

/// Common data needed for fraud detection and auditing
pub struct FraudCheckContext<'a> {
    pub user_identifier: &'a str,
    pub amount: u64,
    pub currency: &'a str,
    pub operation: &'a str,
    pub device_fingerprint: Option<&'a str>,
    pub geo_location: Option<&'a str>,
}

/// Verifies user exists
pub async fn verify_user_exists(user_identifier: &str) -> Result<(), String> {
    let user_exists = user_client::user_exists(user_identifier).await?;
    if !user_exists {
        return Err("User not found".to_string());
    }
    Ok(())
}

/// Performs comprehensive PIN verification with exponential backoff
/// Returns true if PIN is valid, false if invalid, or error if backoff active
pub async fn verify_pin_with_backoff(
    user_identifier: &str,
    pin: &str,
    operation_name: &str,
    audit_context: &str,
) -> Result<bool, String> {
    // Check PIN attempts allowed (exponential backoff)
    fraud_detection::check_pin_attempts_allowed(user_identifier)?;

    // Verify PIN
    let verified = user_client::verify_pin(user_identifier, pin).await?;

    if !verified {
        fraud_detection::record_failed_pin_attempt(user_identifier)?;
        audit::log_failure(
            &format!("failed_pin_{}", operation_name),
            Some(user_identifier.to_string()),
            audit_context.to_string(),
        );
        return Err("Invalid PIN".to_string());
    }

    // Reset PIN attempts on success
    fraud_detection::reset_pin_attempts(user_identifier);
    Ok(true)
}

/// Checks operation-specific rate limits
pub fn check_operation_rate_limit(
    user_identifier: &str,
    operation: &str,
    audit_context: &str,
) -> Result<(), String> {
    if !fraud_detection::check_operation_rate_limit(user_identifier, operation)? {
        audit::log_failure(
            "rate_limit_exceeded",
            Some(user_identifier.to_string()),
            audit_context.to_string(),
        );
        return Err("Operation rate limit exceeded. Please try again later".to_string());
    }
    Ok(())
}

/// Performs comprehensive fraud detection and returns result
pub fn perform_fraud_check(
    context: &FraudCheckContext,
) -> Result<FraudCheckResult, String> {
    let fraud_check = fraud_detection::check_transaction(
        context.user_identifier,
        context.amount,
        context.currency,
        context.operation,
        context.device_fingerprint,
        context.geo_location,
    )?;

    if fraud_check.should_block {
        audit::log_failure(
            "transaction_blocked",
            Some(context.user_identifier.to_string()),
            format!(
                "Operation: {} | Amount: {} {} | Risk Score: {} | Warnings: {:?} | Device: {:?} | Location: {:?}",
                context.operation, context.amount, context.currency, fraud_check.risk_score,
                fraud_check.warnings, context.device_fingerprint, context.geo_location
            ),
        );
        return Err(format!("Transaction blocked due to security concerns: {:?}", fraud_check.warnings));
    }

    if fraud_check.requires_manual_review {
        audit::log_success(
            "manual_review_required",
            Some(context.user_identifier.to_string()),
            format!(
                "Operation: {} | Amount: {} {} | Risk Score: {} | Warnings: {:?}",
                context.operation, context.amount, context.currency, fraud_check.risk_score,
                fraud_check.warnings
            ),
        );
    }

    Ok(FraudCheckResult {
        should_block: fraud_check.should_block,
        requires_manual_review: fraud_check.requires_manual_review,
        risk_score: fraud_check.risk_score,
        warnings: fraud_check.warnings,
    })
}

/// Records device fingerprint with audit logging
pub fn record_device_fingerprint(
    user_identifier: &str,
    fingerprint: &str,
) -> Result<(), String> {
    fraud_detection::record_device_fingerprint(user_identifier, fingerprint)?;
    audit::log_success(
        "device_recorded",
        Some(user_identifier.to_string()),
        format!("Device: {}", fingerprint),
    );
    Ok(())
}

/// Records geo location with audit logging
pub fn record_geo_location(
    user_identifier: &str,
    location: &str,
) -> Result<(), String> {
    fraud_detection::record_geo_location(user_identifier, location)?;
    audit::log_success(
        "location_recorded",
        Some(user_identifier.to_string()),
        format!("Location: {}", location),
    );
    Ok(())
}

/// Records device and location information if provided
pub fn record_device_and_location(
    user_identifier: &str,
    device_fingerprint: Option<&str>,
    geo_location: Option<&str>,
) -> Result<(), String> {
    if let Some(fingerprint) = device_fingerprint {
        record_device_fingerprint(user_identifier, fingerprint)?;
    }
    if let Some(location) = geo_location {
        record_geo_location(user_identifier, location)?;
    }
    Ok(())
}

/// Records transaction for velocity tracking
pub fn record_transaction_for_velocity(
    user_identifier: &str,
    amount: u64,
    currency: &str,
    operation: &str,
) -> Result<(), String> {
    fraud_detection::record_transaction(user_identifier, amount, currency, operation)
}

/// Calculates exchange rate for display purposes
pub fn calculate_exchange_rate(amount1: u64, amount2: u64) -> f64 {
    if amount2 > 0 {
        amount1 as f64 / amount2 as f64
    } else {
        0.0
    }
}

/// Gets crypto balance for specific crypto type
#[allow(dead_code)]
pub async fn get_crypto_balance_by_type(
    user_identifier: &str,
    crypto_type: CryptoType,
) -> Result<u64, String> {
    let (ckbtc_balance, ckusdc_balance) = data_client::get_crypto_balance(user_identifier).await?;
    let balance = match crypto_type {
        CryptoType::CkBTC => ckbtc_balance,
        CryptoType::CkUSDC => ckusdc_balance,
    };
    Ok(balance)
}

/// Calculates crypto balance delta for buy/sell operations
pub fn calculate_crypto_delta(
    amount: u64,
    crypto_type: CryptoType,
    is_credit: bool,
) -> (i64, i64) {
    let signed_amount = if is_credit {
        amount as i64
    } else {
        -(amount as i64)
    };

    match crypto_type {
        CryptoType::CkBTC => (signed_amount, 0i64),
        CryptoType::CkUSDC => (0i64, signed_amount),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_exchange_rate() {
        assert_eq!(calculate_exchange_rate(100, 50), 2.0);
        assert_eq!(calculate_exchange_rate(50, 100), 0.5);
        assert_eq!(calculate_exchange_rate(100, 0), 0.0);
        assert_eq!(calculate_exchange_rate(0, 100), 0.0);
    }

    #[test]
    fn test_calculate_crypto_delta_btc_credit() {
        let (btc, usdc) = calculate_crypto_delta(1000, CryptoType::CkBTC, true);
        assert_eq!(btc, 1000);
        assert_eq!(usdc, 0);
    }

    #[test]
    fn test_calculate_crypto_delta_btc_debit() {
        let (btc, usdc) = calculate_crypto_delta(1000, CryptoType::CkBTC, false);
        assert_eq!(btc, -1000);
        assert_eq!(usdc, 0);
    }

    #[test]
    fn test_calculate_crypto_delta_usdc_credit() {
        let (btc, usdc) = calculate_crypto_delta(500, CryptoType::CkUSDC, true);
        assert_eq!(btc, 0);
        assert_eq!(usdc, 500);
    }

    #[test]
    fn test_calculate_crypto_delta_usdc_debit() {
        let (btc, usdc) = calculate_crypto_delta(500, CryptoType::CkUSDC, false);
        assert_eq!(btc, 0);
        assert_eq!(usdc, -500);
    }
}
