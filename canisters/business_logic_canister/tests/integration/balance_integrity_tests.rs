use super::*;

// ============================================================================
// CRITICAL: Balance Integrity Tests - Money Conservation Laws
// ============================================================================
// These tests ensure we NEVER create or lose money in the system!

#[test]
fn test_money_conservation_simple_transfer() {
    let env = TestEnv::new();
    
    let alice_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Alice",
        "User",
        "alice@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let bob_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Bob",
        "User",
        "bob@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Initial state: Alice has 100k, Bob has 0
    env.set_fiat_balance(&alice_id, "UGX", 100000).unwrap();
    let total_before = 100000;
    
    // Transfer 30k from Alice to Bob
    env.send_money_to_phone("+256700111111", "+256700222222", 30000, "UGX", "1234").unwrap();
    
    // Check final balances
    let alice_balance = env.check_fiat_balance(&alice_id, "UGX").unwrap();
    let bob_balance = env.check_fiat_balance(&bob_id, "UGX").unwrap();
    
    // CRITICAL: Total money must be conserved!
    let total_after = alice_balance + bob_balance;
    assert_eq!(total_after, total_before, 
        "MONEY CONSERVATION VIOLATED! Before: {}, After: {}, Alice: {}, Bob: {}", 
        total_before, total_after, alice_balance, bob_balance);
    
    assert_eq!(alice_balance, 70000);
    assert_eq!(bob_balance, 30000);
}

#[test]
fn test_money_conservation_multiple_transfers() {
    let env = TestEnv::new();
    
    // Create 3 users
    let user1_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "User1",
        "Test",
        "user1@example.com",
        "UGX",
        "1111",
    ).unwrap();
    
    let user2_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "User2",
        "Test",
        "user2@example.com",
        "UGX",
        "2222",
    ).unwrap();
    
    let user3_id = env.register_user(
        Some("+256700333333".to_string()),
        None,
        "User3",
        "Test",
        "user3@example.com",
        "UGX",
        "3333",
    ).unwrap();
    
    // Initial balances: 300k, 200k, 100k
    env.set_fiat_balance(&user1_id, "UGX", 300000).unwrap();
    env.set_fiat_balance(&user2_id, "UGX", 200000).unwrap();
    env.set_fiat_balance(&user3_id, "UGX", 100000).unwrap();
    let total_before = 600000;
    
    // Do a ring of transfers
    env.send_money_to_phone("+256700111111", "+256700222222", 50000, "UGX", "1111").unwrap();
    env.send_money_to_phone("+256700222222", "+256700333333", 30000, "UGX", "2222").unwrap();
    env.send_money_to_phone("+256700333333", "+256700111111", 20000, "UGX", "3333").unwrap();
    
    // Check final balances
    let user1_final = env.check_fiat_balance(&user1_id, "UGX").unwrap();
    let user2_final = env.check_fiat_balance(&user2_id, "UGX").unwrap();
    let user3_final = env.check_fiat_balance(&user3_id, "UGX").unwrap();
    
    // CRITICAL: Total money MUST be conserved!
    let total_after = user1_final + user2_final + user3_final;
    assert_eq!(total_after, total_before,
        "MONEY CONSERVATION VIOLATED! Before: {}, After: {}", total_before, total_after);
    
    // Verify individual balances
    assert_eq!(user1_final, 270000, "User1: 300k - 50k + 20k = 270k");
    assert_eq!(user2_final, 220000, "User2: 200k + 50k - 30k = 220k");
    assert_eq!(user3_final, 110000, "User3: 100k + 30k - 20k = 110k");
}

#[test]
fn test_money_conservation_with_withdrawals() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "User",
        "Test",
        "user@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+256700999999".to_string()),
        None,
        "Agent",
        "Test",
        "agent@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Initial: User has 200k, Agent has 0
    env.set_fiat_balance(&user_id, "UGX", 200000).unwrap();
    let total_before = 200000;
    
    // User withdraws 50k via agent
    env.withdraw_fiat("+256700111111", 50000, "UGX", &agent_id, "1234").unwrap();
    
    // Check balances
    let user_balance = env.check_fiat_balance(&user_id, "UGX").unwrap();
    
    // After withdrawal, user should have 150k
    // Money leaves the system (goes to real world), so total in system decreases
    assert_eq!(user_balance, 150000, "User should have 150k after withdrawal");
    
    // This is correct - withdrawal removes money from digital system
    let total_after = user_balance;
    assert!(total_after < total_before, "Withdrawal should reduce total money in system");
}

#[test]
fn test_cannot_double_spend() {
    let env = TestEnv::new();
    
    let sender_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Sender",
        "Test",
        "sender@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver1_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Receiver1",
        "Test",
        "receiver1@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    let receiver2_id = env.register_user(
        Some("+256700333333".to_string()),
        None,
        "Receiver2",
        "Test",
        "receiver2@example.com",
        "UGX",
        "9999",
    ).unwrap();
    
    // Sender has only 100k
    env.set_fiat_balance(&sender_id, "UGX", 100000).unwrap();
    
    // Try to send 60k to receiver1
    let result1 = env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        60000,
        "UGX",
        "1234",
    );
    assert!(result1.is_ok(), "First transfer should succeed");
    
    // Try to send 60k to receiver2 (should fail - only 40k left!)
    let result2 = env.send_money_to_phone(
        "+256700111111",
        "+256700333333",
        60000,
        "UGX",
        "1234",
    );
    assert!(result2.is_err(), "Second transfer should fail - insufficient balance");
    
    // Verify balances
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    let receiver1_balance = env.check_fiat_balance(&receiver1_id, "UGX").unwrap();
    let receiver2_balance = env.check_fiat_balance(&receiver2_id, "UGX").unwrap();
    
    assert_eq!(sender_balance, 40000, "Sender should have 40k left");
    assert_eq!(receiver1_balance, 60000, "Receiver1 should have 60k");
    assert_eq!(receiver2_balance, 0, "Receiver2 should have 0");
    
    // CRITICAL: Total must be 100k
    let total = sender_balance + receiver1_balance + receiver2_balance;
    assert_eq!(total, 100000, "Total money must be conserved!");
}

#[test]
fn test_cannot_transfer_more_than_balance() {
    let env = TestEnv::new();
    
    let sender_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Sender",
        "Test",
        "sender@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Receiver",
        "Test",
        "receiver@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Sender has 50k
    env.set_fiat_balance(&sender_id, "UGX", 50000).unwrap();
    
    // Try to send 100k (more than balance)
    let result = env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        100000,
        "UGX",
        "1234",
    );
    
    assert!(result.is_err(), "Should not allow transfer exceeding balance");
    assert!(result.unwrap_err().contains("Insufficient"), "Error should mention insufficient balance");
    
    // Balance should be unchanged
    let balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    assert_eq!(balance, 50000, "Balance should remain unchanged after failed transfer");
}

#[test]
fn test_can_transfer_exact_balance() {
    let env = TestEnv::new();
    
    let sender_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Sender",
        "Test",
        "sender@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let receiver_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Receiver",
        "Test",
        "receiver@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Sender has 100k
    env.set_fiat_balance(&sender_id, "UGX", 100000).unwrap();
    
    // Transfer exact balance
    let result = env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        100000,
        "UGX",
        "1234",
    );
    
    assert!(result.is_ok(), "Should allow transfer of exact balance");
    
    // Verify balances
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX").unwrap();
    let receiver_balance = env.check_fiat_balance(&receiver_id, "UGX").unwrap();
    
    assert_eq!(sender_balance, 0, "Sender should have 0");
    assert_eq!(receiver_balance, 100000, "Receiver should have all the money");
}

#[test]
fn test_zero_amount_transfer_fails() {
    let env = TestEnv::new();
    
    let sender_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Sender",
        "Test",
        "sender@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Receiver",
        "Test",
        "receiver@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    env.set_fiat_balance(&sender_id, "UGX", 100000).unwrap();
    
    // Try to send 0
    let result = env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        0,
        "UGX",
        "1234",
    );
    
    assert!(result.is_err(), "Should not allow zero amount transfer");
}
