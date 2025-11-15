/// Integration tests for wallet_canister security features
///
/// Tests cover:
/// - Daily transaction limits (count and amount)
/// - Velocity checking (rapid transactions)
/// - Threshold warnings (80% approaching limit)
/// - Multi-currency daily limits
/// - Audit logging with transaction IDs

use super::*;

// ============================================================================
// Daily Transaction Count Limits
// ============================================================================

#[test]
fn test_daily_transaction_count_limit_enforcement() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    // Set high balance to test count limits (not amount limits)
    env.set_fiat_balance(&sender_id, "KES", 100_000_000).expect("Should set balance");

    // Default max_daily_transactions = 50 (from wallet_config.toml)
    // Perform 50 small transactions (should all succeed)
    for i in 0..50 {
        let result = env.transfer_fiat(
            &sender_id,
            &recipient_id,
            1000, // Small amount to avoid amount limits
            "KES",
            "1234",
            Some(format!("Transaction {}", i + 1)),
        );

        assert!(result.is_ok(), "Transaction {} should succeed (within daily limit)", i + 1);
    }

    // 51st transaction should be blocked (exceeds daily count limit)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1000,
        "KES",
        "1234",
        Some("Transaction 51 - should be blocked".to_string()),
    );

    assert!(result.is_err(), "51st transaction should be blocked");
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Daily transaction limit") || error_msg.contains("blocked"),
        "Error should mention daily limit, got: {}",
        error_msg
    );
}

#[test]
fn test_daily_transaction_count_warning_at_80_percent() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 100_000_000).expect("Should set balance");

    // Default max_daily_transactions = 50
    // 80% of 50 = 40 transactions
    // Perform 39 transactions (below warning threshold)
    for i in 0..39 {
        let result = env.transfer_fiat(
            &sender_id,
            &recipient_id,
            1000,
            "KES",
            "1234",
            Some(format!("Transaction {}", i + 1)),
        );
        assert!(result.is_ok(), "Transaction {} should succeed", i + 1);
    }

    // 40th transaction should succeed but trigger warning (80% threshold)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1000,
        "KES",
        "1234",
        Some("Transaction 40 - warning threshold".to_string()),
    );

    // Transaction should succeed (warnings don't block)
    assert!(result.is_ok(), "40th transaction should succeed with warning");

    // Note: In production, this would generate audit log warning
    // We can't easily verify audit logs in integration tests without reading canister logs
}

// ============================================================================
// Daily Amount Limits
// ============================================================================

#[test]
fn test_daily_amount_limit_enforcement_kes() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    // Set balance higher than daily limit
    env.set_fiat_balance(&sender_id, "KES", 100_000_000).expect("Should set balance");

    // KES max_daily_amount = 75,000,000 (from wallet_config.toml)
    // Transfer in chunks to test daily amount limit

    // Transfer 70,000,000 (below limit)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        70_000_000,
        "KES",
        "1234",
        Some("Large transfer 1".to_string()),
    );
    assert!(result.is_ok(), "First large transfer should succeed");

    // Add more balance
    env.set_fiat_balance(&sender_id, "KES", 100_000_000).expect("Should set balance");

    // Transfer 6,000,000 more (total = 76M, exceeds 75M daily limit)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        6_000_000,
        "KES",
        "1234",
        Some("Transfer exceeding daily amount".to_string()),
    );

    assert!(result.is_err(), "Transfer exceeding daily amount limit should be blocked");
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Daily amount limit") || error_msg.contains("blocked"),
        "Error should mention daily amount limit, got: {}",
        error_msg
    );
}

#[test]
fn test_daily_amount_limit_warning_at_80_percent_ugx() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+256700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+256700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "UGX",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "UGX", 2_000_000_000).expect("Should set balance");

    // UGX max_daily_amount = 1,850,000,000
    // 80% of 1,850,000,000 = 1,480,000,000

    // Transfer 1,450,000,000 (below 80% threshold)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1_450_000_000,
        "UGX",
        "1234",
        Some("Below warning threshold".to_string()),
    );
    assert!(result.is_ok(), "Transfer below 80% should succeed without warning");

    // Add more balance
    env.set_fiat_balance(&sender_id, "UGX", 2_000_000_000).expect("Should set balance");

    // Transfer 50,000,000 more (total = 1,500,000,000, exceeds 80% threshold)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        50_000_000,
        "UGX",
        "1234",
        Some("Trigger warning threshold".to_string()),
    );

    // Should succeed but generate warning
    assert!(result.is_ok(), "Transfer at 81% should succeed with warning");
}

#[test]
fn test_daily_amount_limit_enforcement_ngn() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+234700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "NGN",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+234700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "NGN",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "NGN", 1_000_000_000).expect("Should set balance");

    // NGN max_daily_amount = 750,000,000
    // Transfer exactly at limit (should succeed)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        750_000_000,
        "NGN",
        "1234",
        Some("At daily limit".to_string()),
    );
    assert!(result.is_ok(), "Transfer at limit should succeed");

    // Add more balance
    env.set_fiat_balance(&sender_id, "NGN", 1_000_000_000).expect("Should set balance");

    // Any additional transfer should be blocked
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1,
        "NGN",
        "1234",
        Some("1 NGN over limit".to_string()),
    );

    assert!(result.is_err(), "Transfer exceeding daily limit should be blocked");
}

// ============================================================================
// Velocity Checking (Rapid Transactions)
// ============================================================================

#[test]
fn test_velocity_limit_10_transactions_per_hour() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 10_000_000).expect("Should set balance");

    // Max velocity = 10 transactions per hour
    // Perform 10 rapid transactions (should all succeed)
    for i in 0..10 {
        let result = env.transfer_fiat(
            &sender_id,
            &recipient_id,
            1000,
            "KES",
            "1234",
            Some(format!("Rapid transaction {}", i + 1)),
        );

        assert!(result.is_ok(), "Transaction {} should succeed (within velocity limit)", i + 1);
    }

    // 11th transaction should be blocked (exceeds velocity limit)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1000,
        "KES",
        "1234",
        Some("Transaction 11 - should be blocked".to_string()),
    );

    assert!(result.is_err(), "11th transaction should be blocked by velocity check");
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Velocity") || error_msg.contains("blocked"),
        "Error should mention velocity limit, got: {}",
        error_msg
    );
}

#[test]
fn test_velocity_warning_at_80_percent() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 10_000_000).expect("Should set balance");

    // Max velocity = 10 transactions/hour
    // 80% of 10 = 8 transactions

    // Perform 7 transactions (below warning threshold)
    for i in 0..7 {
        let result = env.transfer_fiat(
            &sender_id,
            &recipient_id,
            1000,
            "KES",
            "1234",
            Some(format!("Transaction {}", i + 1)),
        );
        assert!(result.is_ok(), "Transaction {} should succeed", i + 1);
    }

    // 8th transaction should succeed but trigger warning (80% threshold)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1000,
        "KES",
        "1234",
        Some("Transaction 8 - warning threshold".to_string()),
    );

    assert!(result.is_ok(), "8th transaction should succeed with warning");
}

// ============================================================================
// Multi-Currency Isolation
// ============================================================================

#[test]
fn test_daily_limits_are_currency_specific() {
    let env = TestEnv::new();

    // Register user with KES preference
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    // Set balances for both KES and UGX
    env.set_fiat_balance(&user_id, "KES", 100_000_000).expect("Should set KES balance");
    env.set_fiat_balance(&user_id, "UGX", 2_000_000_000).expect("Should set UGX balance");

    // Perform 10 KES transactions (max velocity)
    for i in 0..10 {
        let result = env.transfer_fiat(
            &user_id,
            &recipient_id,
            1000,
            "KES",
            "1234",
            Some(format!("KES transaction {}", i + 1)),
        );
        assert!(result.is_ok(), "KES transaction {} should succeed", i + 1);
    }

    // Next KES transaction should be blocked by velocity
    let result = env.transfer_fiat(
        &user_id,
        &recipient_id,
        1000,
        "KES",
        "1234",
        Some("KES transaction 11".to_string()),
    );
    assert!(result.is_err(), "11th KES transaction should be blocked");

    // But UGX transactions should still work (separate currency tracking)
    let result = env.transfer_fiat(
        &user_id,
        &recipient_id,
        1000,
        "UGX",
        "1234",
        Some("UGX transaction 1".to_string()),
    );

    assert!(result.is_ok(), "UGX transaction should succeed (separate currency limit)");
}

// ============================================================================
// Combined Fraud Checks (Multi-Layer)
// ============================================================================

#[test]
fn test_per_transaction_amount_checked_before_daily_limits() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 20_000_000).expect("Should set balance");

    // Try to transfer amount exceeding per-transaction limit
    // KES max_transaction_amount = 15,000,000
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        16_000_000,
        "KES",
        "1234",
        Some("Exceeds per-transaction limit".to_string()),
    );

    assert!(result.is_err(), "Transaction exceeding per-transaction limit should be blocked");
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("exceeds") || error_msg.contains("blocked"),
        "Error should indicate amount check failed, got: {}",
        error_msg
    );
}

#[test]
fn test_velocity_checked_before_amount_limits() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 100_000_000).expect("Should set balance");

    // Perform 10 transactions (max velocity)
    for i in 0..10 {
        let result = env.transfer_fiat(
            &sender_id,
            &recipient_id,
            1000,
            "KES",
            "1234",
            Some(format!("Transaction {}", i + 1)),
        );
        assert!(result.is_ok());
    }

    // 11th transaction should be blocked by velocity even if amount is valid
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        1000,
        "KES",
        "1234",
        Some("Blocked by velocity".to_string()),
    );

    assert!(result.is_err(), "Should be blocked by velocity check");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Velocity"), "Error should mention velocity");
}

// ============================================================================
// Transaction ID and Audit Logging
// ============================================================================

#[test]
fn test_transaction_id_generated_for_successful_transfer() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 1_000_000).expect("Should set balance");

    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        100_000,
        "KES",
        "1234",
        Some("Test transfer".to_string()),
    ).expect("Transfer should succeed");

    // Verify transaction_id is generated and not empty
    assert!(!result.transaction_id.is_empty(), "Transaction ID should not be empty");

    // Transaction ID should start with "tx_" prefix (based on transfer_logic implementation)
    assert!(
        result.transaction_id.starts_with("tx_"),
        "Transaction ID should start with 'tx_' prefix"
    );

    // Transaction ID should be unique (timestamp-based)
    let result2 = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        50_000,
        "KES",
        "1234",
        Some("Test transfer 2".to_string()),
    ).expect("Second transfer should succeed");

    assert_ne!(
        result.transaction_id,
        result2.transaction_id,
        "Transaction IDs should be unique"
    );
}

#[test]
fn test_transaction_history_contains_all_transfers() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 10_000_000).expect("Should set balance");

    // Perform 3 transfers
    let tx1 = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        100_000,
        "KES",
        "1234",
        Some("Transfer 1".to_string()),
    ).expect("Transfer 1 should succeed");

    let tx2 = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        200_000,
        "KES",
        "1234",
        Some("Transfer 2".to_string()),
    ).expect("Transfer 2 should succeed");

    let tx3 = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        300_000,
        "KES",
        "1234",
        Some("Transfer 3".to_string()),
    ).expect("Transfer 3 should succeed");

    // Get transaction history
    let history = env.get_transaction_history(&sender_id, Some(10), Some(0))
        .expect("Should retrieve transaction history");

    // Should have at least 3 transactions
    assert!(history.len() >= 3, "History should contain at least 3 transactions");

    // Verify transaction IDs are in history
    let tx_ids: Vec<String> = history.iter().map(|tx| tx.id.clone()).collect();
    assert!(tx_ids.contains(&tx1.transaction_id), "History should contain tx1");
    assert!(tx_ids.contains(&tx2.transaction_id), "History should contain tx2");
    assert!(tx_ids.contains(&tx3.transaction_id), "History should contain tx3");

    // Verify amounts
    let amounts: Vec<u64> = history.iter().map(|tx| tx.amount).collect();
    assert!(amounts.contains(&100_000), "History should contain 100,000 KES transfer");
    assert!(amounts.contains(&200_000), "History should contain 200,000 KES transfer");
    assert!(amounts.contains(&300_000), "History should contain 300,000 KES transfer");
}

// ============================================================================
// Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_exactly_at_per_transaction_limit() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 20_000_000).expect("Should set balance");

    // KES max_transaction_amount = 15,000,000
    // Transfer exactly at limit (should succeed)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        15_000_000,
        "KES",
        "1234",
        Some("Exactly at limit".to_string()),
    );

    assert!(result.is_ok(), "Transfer exactly at per-transaction limit should succeed");
}

#[test]
fn test_one_over_per_transaction_limit() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 20_000_000).expect("Should set balance");

    // KES max_transaction_amount = 15,000,000
    // Transfer 1 over limit (should be blocked)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        15_000_001,
        "KES",
        "1234",
        Some("One over limit".to_string()),
    );

    assert!(result.is_err(), "Transfer one over per-transaction limit should be blocked");
}

#[test]
fn test_zero_daily_transactions_before_first_transfer() {
    let env = TestEnv::new();

    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");

    env.set_fiat_balance(&sender_id, "KES", 1_000_000).expect("Should set balance");

    // First transfer should succeed (0 previous transactions)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        100_000,
        "KES",
        "1234",
        Some("First transfer".to_string()),
    );

    assert!(result.is_ok(), "First transfer should succeed with zero previous transactions");
}
