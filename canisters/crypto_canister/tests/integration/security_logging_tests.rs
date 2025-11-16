/// Security Logging Tests for crypto_canister
///
/// Tests that security events are properly logged when log_suspicious_activity = true
/// These tests will FAIL until the security logging feature is implemented.
///
/// Config: crypto_config.toml [security] log_suspicious_activity = true

use super::*;
use candid::{encode_args, decode_one};

// ============================================================================
// Test 1: PIN Brute Force Detection & Logging
// ============================================================================

#[test]
fn test_pin_brute_force_logging() {
    let (pic, _data, user_canister, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Register test user
    let user_id = register_test_user(&pic, user_canister, "+256700111001", "1234");

    // Attempt 5 failed PINs in rapid succession (within 5 minutes)
    // This should trigger brute force detection logging
    for attempt in 1..=5 {
        // Try to buy crypto with wrong PIN
        let result = pic.update_call(
            crypto_canister,
            Principal::anonymous(),
            "buy_ckbtc",
            encode_args((
                user_id.clone(),
                10000u64, // 100 KES in cents
                "9999".to_string(), // Wrong PIN
            )).unwrap(),
        );

        // Should fail due to wrong PIN
        if let Ok(response) = result {
            let buy_result: Result<String, String> = decode_one(&response).unwrap();
            assert!(buy_result.is_err(), "Buy should fail with wrong PIN on attempt {}", attempt);
            if let Err(error_msg) = buy_result {
                assert!(error_msg.contains("Invalid PIN") || error_msg.contains("pin"),
                        "Error should mention PIN issue, got: {}", error_msg);
            }
        }
    }

    // Query audit log for security events
    let audit_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_security_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    );

    assert!(audit_result.is_ok(), "Should be able to query security audit log");

    let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
        candid::decode_args(&audit_result.unwrap())
            .expect("Failed to decode audit log");

    // EXPECTED TO FAIL: Feature not implemented yet
    // Should find brute force detection log entry
    let brute_force_logs: Vec<_> = audit_entries.iter()
        .filter(|entry| entry.action == "security_pin_brute_force"
                     || entry.details.contains("brute_force")
                     || entry.details.contains("multiple failed PIN attempts"))
        .collect();

    assert!(
        brute_force_logs.len() >= 1,
        "Expected at least 1 brute force security log, found {}. Feature not implemented!",
        brute_force_logs.len()
    );
}

// ============================================================================
// Test 2: Suspicious Transaction Amount Logging
// ============================================================================

#[test]
fn test_suspicious_amount_logging() {
    let (pic, data, user_canister, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Register wealthy test user
    let user_id = register_test_user(&pic, user_canister, "+256700111002", "1234");

    // Give user large fiat balance
    set_fiat_balance(&pic, data, &user_id, "KES", 100_000_000); // 1M KES

    // Attempt to buy large amount of ckBTC (above suspicious threshold of $500 = 50000 cents)
    // Config: fraud_detection.suspicious_amount_cents = 50000
    let result = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_ckbtc",
        encode_args((
            user_id.clone(),
            75_000u64, // 750 KES = ~$7.50 (well above suspicious threshold)
            "1234".to_string(),
        )).unwrap(),
    );

    // Transaction should succeed (logging doesn't block transactions)
    assert!(result.is_ok(), "Large transaction should succeed but be logged");

    // Query audit log for suspicious activity
    let audit_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_security_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    ).expect("Should query audit log");

    let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
        candid::decode_args(&audit_result)
            .expect("Failed to decode audit log");

    // EXPECTED TO FAIL: Feature not implemented yet
    let suspicious_amount_logs: Vec<_> = audit_entries.iter()
        .filter(|entry| entry.action == "security_suspicious_amount"
                     || entry.details.contains("suspicious amount")
                     || entry.details.contains("high value transaction"))
        .collect();

    assert!(
        suspicious_amount_logs.len() >= 1,
        "Expected suspicious amount log, found {}. Feature not implemented!",
        suspicious_amount_logs.len()
    );

    // Verify log contains amount details
    if let Some(log) = suspicious_amount_logs.first() {
        assert!(
            log.details.contains("750") || log.details.contains("75000"),
            "Log should contain transaction amount"
        );
    }
}

// ============================================================================
// Test 3: Rapid Transaction Velocity Logging
// ============================================================================

#[test]
fn test_rapid_transaction_velocity_logging() {
    let (pic, data, user_canister, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Register test user
    let user_id = register_test_user(&pic, user_canister, "+256700111003", "1234");

    // Give user balance
    set_fiat_balance(&pic, data, &user_id, "KES", 500_000); // 5000 KES

    // Perform 12 rapid crypto purchases (above max_buys_per_hour = 10)
    for i in 1..=12 {
        let result = pic.update_call(
            crypto_canister,
            Principal::anonymous(),
            "buy_ckbtc",
            encode_args((
                user_id.clone(),
                1000u64, // Small amount (10 KES)
                "1234".to_string(),
            )).unwrap(),
        );

        if i <= 10 {
            // First 10 should succeed
            assert!(result.is_ok(), "Transaction {} should succeed", i);
        } else {
            // Transactions 11-12 should be logged as suspicious velocity
            // (but may still succeed - depends on implementation)
            let _ = result; // Don't assert - logging vs blocking is implementation choice
        }
    }

    // Query audit log for rapid transaction detection
    let audit_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_security_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    ).expect("Should query audit log");

    let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
        candid::decode_args(&audit_result)
            .expect("Failed to decode audit log");

    // EXPECTED TO FAIL: Feature not implemented yet
    let velocity_logs: Vec<_> = audit_entries.iter()
        .filter(|entry| entry.action == "security_rapid_transactions"
                     || entry.details.contains("rapid transaction")
                     || entry.details.contains("velocity")
                     || entry.details.contains("too many transactions"))
        .collect();

    assert!(
        velocity_logs.len() >= 1,
        "Expected rapid transaction velocity log, found {}. Feature not implemented!",
        velocity_logs.len()
    );
}

// ============================================================================
// Test 4: Logging Disabled When Config = False
// ============================================================================

#[test]
fn test_logging_disabled_when_config_false() {
    // This test verifies that when log_suspicious_activity = false,
    // no security logs are created (only normal transaction logs)

    // NOTE: This test will need to temporarily modify the config or
    // use a separate canister instance with log_suspicious_activity = false
    // For now, we'll just document the expected behavior

    // EXPECTED BEHAVIOR:
    // - When log_suspicious_activity = false in crypto_config.toml
    // - Suspicious activities should NOT create security audit entries
    // - Normal transaction logs should still work
    // - get_security_audit_log() should return empty or only non-security events

    // This test is a placeholder for the rust-ic-expert to implement properly
    // with config mocking or multiple canister instances
}

// ============================================================================
// Test 5: Successful Operations Are Logged
// ============================================================================

#[test]
fn test_successful_operations_logged() {
    let (pic, data, user_canister, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Register test user
    let user_id = register_test_user(&pic, user_canister, "+256700111005", "1234");

    // Give user balance
    set_fiat_balance(&pic, data, &user_id, "KES", 50_000); // 500 KES

    // Perform successful buy operation
    let result = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_ckbtc",
        encode_args((
            user_id.clone(),
            5000u64, // 50 KES
            "1234".to_string(),
        )).unwrap(),
    );

    assert!(result.is_ok(), "Buy should succeed");

    // Query general audit log (not just security events)
    let audit_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    ).expect("Should query audit log");

    let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
        candid::decode_args(&audit_result)
            .expect("Failed to decode audit log");

    // Should find successful buy operation
    let buy_logs: Vec<_> = audit_entries.iter()
        .filter(|entry| entry.action.contains("buy") && entry.success)
        .collect();

    assert!(
        buy_logs.len() >= 1,
        "Expected successful buy operation log, found {}",
        buy_logs.len()
    );
}

// ============================================================================
// Test 6: Audit Log Query Functionality
// ============================================================================

#[test]
fn test_audit_log_query_functionality() {
    let (pic, _data, _user, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Test that we can query different types of audit logs

    // 1. Get all audit logs
    let all_logs_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    );
    assert!(all_logs_result.is_ok(), "Should query all audit logs");

    // 2. Get security-specific audit logs
    let security_logs_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_security_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    );
    assert!(security_logs_result.is_ok(), "Should query security audit logs");

    // 3. Get failed operations
    let failed_ops_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_failed_operations",
        encode_args((Some(100usize),)).unwrap(),
    );
    assert!(failed_ops_result.is_ok(), "Should query failed operations");

    // EXPECTED TO FAIL: These query methods may not exist yet
    // The rust-ic-expert should implement:
    // - get_audit_log(limit: Option<usize>) -> Vec<AuditEntry>
    // - get_security_audit_log(limit: Option<usize>) -> Vec<AuditEntry>
    // - get_failed_operations(limit: Option<usize>) -> Vec<AuditEntry>
}

// ============================================================================
// Test 7: High Risk Amount Detection
// ============================================================================

#[test]
fn test_high_risk_amount_detection() {
    let (pic, data, user_canister, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Register test user
    let user_id = register_test_user(&pic, user_canister, "+256700111007", "1234");

    // Give user very large balance
    set_fiat_balance(&pic, data, &user_id, "KES", 200_000_000); // 2M KES

    // Attempt transaction above high_risk_amount_cents = 100000 ($1000)
    let result = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_ckbtc",
        encode_args((
            user_id.clone(),
            150_000u64, // 1500 KES = ~$15 (above high risk threshold)
            "1234".to_string(),
        )).unwrap(),
    );

    // Should succeed but be logged as high risk
    assert!(result.is_ok(), "High risk transaction should succeed but be logged");

    // Query audit log
    let audit_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_security_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    ).expect("Should query audit log");

    let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
        candid::decode_args(&audit_result)
            .expect("Failed to decode audit log");

    // EXPECTED TO FAIL: Feature not implemented
    let high_risk_logs: Vec<_> = audit_entries.iter()
        .filter(|entry| entry.action == "security_high_risk_amount"
                     || entry.details.contains("high risk")
                     || entry.details.contains("large amount"))
        .collect();

    assert!(
        high_risk_logs.len() >= 1,
        "Expected high risk amount log, found {}. Feature not implemented!",
        high_risk_logs.len()
    );
}

// ============================================================================
// Test 8: Multiple Security Events for Same User
// ============================================================================

#[test]
fn test_multiple_security_events_same_user() {
    let (pic, data, user_canister, _wallet, crypto_canister, _ckbtc, _ckusdc) = setup_test_environment();

    // Register test user
    let user_id = register_test_user(&pic, user_canister, "+256700111008", "1234");

    // Give user balance
    set_fiat_balance(&pic, data, &user_id, "KES", 500_000);

    // Trigger multiple types of security events:

    // 1. Wrong PIN attempts
    for _ in 1..=3 {
        let _ = pic.update_call(
            crypto_canister,
            Principal::anonymous(),
            "buy_ckbtc",
            encode_args((user_id.clone(), 1000u64, "9999".to_string())).unwrap(),
        );
    }

    // 2. Rapid transactions
    for _ in 1..=11 {
        let _ = pic.update_call(
            crypto_canister,
            Principal::anonymous(),
            "buy_ckbtc",
            encode_args((user_id.clone(), 1000u64, "1234".to_string())).unwrap(),
        );
    }

    // Query user-specific audit log
    let audit_result = pic.query_call(
        crypto_canister,
        Principal::anonymous(),
        "get_user_security_log",
        encode_args((user_id.clone(), Some(100usize))).unwrap(),
    );

    // EXPECTED TO FAIL: get_user_security_log may not exist
    assert!(audit_result.is_ok(), "Should query user-specific security log");

    let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
        candid::decode_args(&audit_result.unwrap())
            .expect("Failed to decode audit log");

    // Should have entries for both PIN failures and velocity issues
    let pin_failures = audit_entries.iter()
        .filter(|e| e.details.contains("PIN") || e.details.contains("brute_force"))
        .count();

    let velocity_issues = audit_entries.iter()
        .filter(|e| e.details.contains("rapid") || e.details.contains("velocity"))
        .count();

    assert!(
        pin_failures > 0 && velocity_issues > 0,
        "Should have both PIN failure and velocity logs for same user. Feature not implemented!"
    );
}
