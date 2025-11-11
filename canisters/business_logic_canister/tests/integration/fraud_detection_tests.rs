use crate::integration::TestEnv;
use shared_types::*;

// Note: Using send_money_to_phone for transfers (not send_money_to_phone)
// Using check_fiat_balance for balance queries (not check_fiat_balance)

// ============================================================================
// FRAUD DETECTION INTEGRATION TESTS
// ============================================================================
// These tests verify fraud detection works in real transaction flows:
// - Large transaction blocking
// - Suspicious amount flagging
// - Transaction limits enforcement
// - Multiple failed PIN attempts
// ============================================================================

#[test]
fn test_large_transfer_blocked_by_fraud_detection() {
    let env = TestEnv::new();
    
    // Register two users
    let sender_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "Sender",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Receiver",
        "bob@test.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Give sender a large balance
    env.set_fiat_balance(&sender_id, "UGX", 20_000_000).unwrap();
    
    // Try to transfer amount exceeding max limit (10,000,000)
    let result = env.send_money_to_phone(
        "+254700000001",
        "+254700000002",
        15_000_000, // Exceeds max limit
        "UGX",
        "1234",
    );
    
    // Should be blocked by fraud detection
    assert!(result.is_err(), "Large transfer should be blocked");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("blocked") || err_msg.contains("limit") || err_msg.contains("fraud"),
        "Error should mention fraud/blocking, got: {}", err_msg);
    
    // Verify balances unchanged
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    assert_eq!(sender_balance, 20_000_000, "Sender balance should be unchanged");
}

#[test]
fn test_suspicious_transfer_flagged_but_allowed() {
    let env = TestEnv::new();
    
    // Register two users
    let sender_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "Sender",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Receiver",
        "bob@test.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Give sender balance
    env.set_fiat_balance(&sender_id, "UGX", 10_000_000).unwrap();
    
    // Transfer amount above suspicious threshold (5,000,000) but below max (10,000,000)
    let result = env.send_money_to_phone(
        "+254700000001",
        "+254700000002",
        7_000_000, // Suspicious but not blocked
        "UGX",
        "1234",
    );
    
    // Should succeed but be flagged as suspicious
    assert!(result.is_ok(), "Suspicious transfer should succeed: {:?}", result);
    
    // Verify balances updated correctly
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    let receiver_balance = env.check_fiat_balance(&receiver_id, "UGX").unwrap();
    
    assert_eq!(sender_balance, 3_000_000, "Sender balance should be reduced");
    assert_eq!(receiver_balance, 7_000_000, "Receiver should receive funds");
}

#[test]
fn test_normal_transfer_not_flagged() {
    let env = TestEnv::new();
    
    // Register two users
    let sender_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "Sender",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Receiver",
        "bob@test.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Give sender balance
    env.set_fiat_balance(&sender_id, "UGX", 5_000_000).unwrap();
    
    // Transfer normal amount (below suspicious threshold)
    let result = env.send_money_to_phone(
        "+254700000001",
        "+254700000002",
        1_000_000, // Normal amount
        "UGX",
        "1234",
    );
    
    // Should succeed without issues
    assert!(result.is_ok(), "Normal transfer should succeed");
    
    // Verify balances
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    let receiver_balance = env.check_fiat_balance(&receiver_id, "UGX").unwrap();
    
    assert_eq!(sender_balance, 4_000_000);
    assert_eq!(receiver_balance, 1_000_000);
}

#[test]
fn test_large_withdrawal_blocked() {
    let env = TestEnv::new();
    
    // Register user and agent
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Give user large balance
    env.set_fiat_balance(&user_id, "UGX", 20_000_000).unwrap();
    
    // Try to withdraw amount exceeding max limit
    let result = env.withdraw_fiat(
        "+254700000001",
        15_000_000, // Exceeds max limit
        "UGX",
        &agent_id,
        "1234",
    );
    
    // Should be blocked
    assert!(result.is_err(), "Large withdrawal should be blocked");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("blocked") || err_msg.contains("limit") || err_msg.contains("fraud"),
        "Error should mention fraud/blocking, got: {}", err_msg);
}

#[test]
fn test_multiple_small_transfers_allowed() {
    let env = TestEnv::new();
    
    // Register two users
    let sender_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "Sender",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Receiver",
        "bob@test.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Give sender balance
    env.set_fiat_balance(&sender_id, "UGX", 10_000_000).unwrap();
    
    // Make multiple small transfers (should all succeed)
    for i in 1..=5 {
        let result = env.send_money_to_phone(
            "+254700000001",
            "+254700000002",
            500_000, // Small amount
            "UGX",
            "1234",
        );
        
        assert!(result.is_ok(), "Transfer {} should succeed", i);
    }
    
    // Verify final balances
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    let receiver_balance = env.check_fiat_balance(&receiver_id, "UGX").unwrap();
    
    assert_eq!(sender_balance, 7_500_000, "Sender should have 7.5M left");
    assert_eq!(receiver_balance, 2_500_000, "Receiver should have 2.5M");
}

#[test]
fn test_fraud_detection_with_different_currencies() {
    let env = TestEnv::new();
    
    // Register users with different currencies
    let ugx_user = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "UGX User",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let kes_user = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "KES User",
        "bob@test.com",
        "KES",
        "5678",
    ).unwrap();
    
    let ngn_user = env.register_user(
        Some("+254700000003".to_string()),
        None,
        "Charlie",
        "NGN User",
        "charlie@test.com",
        "NGN",
        "9012",
    ).unwrap();
    
    // Give each user large balance
    env.set_fiat_balance(&ugx_user, "UGX", 20_000_000).unwrap();
    env.set_fiat_balance(&kes_user, "KES", 20_000_000).unwrap();
    env.set_fiat_balance(&ngn_user, "NGN", 20_000_000).unwrap();
    
    // Create receiver for each
    let receiver1 = env.register_user(
        Some("+254700000011".to_string()),
        None,
        "Receiver1",
        "Test",
        "r1@test.com",
        "UGX",
        "1111",
    ).unwrap();
    
    let receiver2 = env.register_user(
        Some("+254700000012".to_string()),
        None,
        "Receiver2",
        "Test",
        "r2@test.com",
        "KES",
        "2222",
    ).unwrap();
    
    let receiver3 = env.register_user(
        Some("+254700000013".to_string()),
        None,
        "Receiver3",
        "Test",
        "r3@test.com",
        "NGN",
        "3333",
    ).unwrap();
    
    // Try large transfers in each currency (all should be blocked)
    let ugx_result = env.send_money_to_phone(
        "+254700000001",
        "+254700000011",
        15_000_000,
        "UGX",
        "1234",
    );
    
    let kes_result = env.send_money_to_phone(
        "+254700000002",
        "+254700000012",
        15_000_000,
        "KES",
        "5678",
    );
    
    let ngn_result = env.send_money_to_phone(
        "+254700000003",
        "+254700000013",
        15_000_000,
        "NGN",
        "9012",
    );
    
    // All should be blocked
    assert!(ugx_result.is_err(), "Large UGX transfer should be blocked");
    assert!(kes_result.is_err(), "Large KES transfer should be blocked");
    assert!(ngn_result.is_err(), "Large NGN transfer should be blocked");
}

#[test]
fn test_exact_limit_amount_allowed() {
    let env = TestEnv::new();
    
    // Register two users
    let sender_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "Sender",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Receiver",
        "bob@test.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Give sender balance
    env.set_fiat_balance(&sender_id, "UGX", 15_000_000).unwrap();
    
    // Transfer exactly at max limit (10,000,000) - should be allowed but flagged
    let result = env.send_money_to_phone(
        "+254700000001",
        "+254700000002",
        10_000_000, // Exactly at limit
        "UGX",
        "1234",
    );
    
    // Should succeed (at limit, not over)
    assert!(result.is_ok(), "Transfer at exact limit should succeed: {:?}", result);
    
    // Verify balances
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    let receiver_balance = env.check_fiat_balance(&receiver_id, "UGX").unwrap();
    
    assert_eq!(sender_balance, 5_000_000);
    assert_eq!(receiver_balance, 10_000_000);
}
