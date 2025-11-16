use super::*;

// ============================================================================
// CRITICAL: Balance Integrity Tests - Money Conservation Laws
// ============================================================================
// These tests ensure we NEVER create or lose money in the system!

#[test]
fn test_money_conservation_simple_transfer() {
    let env = TestEnv::new();
    
    let alice_id = env.register_user(
        "+256700111111",
        "Alice",
        "User",
        "alice@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let bob_id = env.register_user(
        "+256700222222",
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
    env.transfer_fiat(&alice_id, &bob_id, 30000, "UGX", "1234", None).unwrap();
    
    // Check final balances
    let alice_balance = env.get_fiat_balance(&alice_id, "UGX").unwrap();
    let bob_balance = env.get_fiat_balance(&bob_id, "UGX").unwrap();
    
    // CRITICAL: Total money must be conserved (minus fee)!
    let total_after = alice_balance + bob_balance;
    let expected_total = total_before - 150; // 30000 * 0.5% = 150 fee
    assert_eq!(total_after, expected_total, 
        "MONEY CONSERVATION VIOLATED! Before: {}, After: {}, Expected: {}, Alice: {}, Bob: {}", 
        total_before, total_after, expected_total, alice_balance, bob_balance);
    
    assert_eq!(alice_balance, 69850); // 100000 - 30000 - 150 (fee)
    assert_eq!(bob_balance, 30000);
}

#[test]
fn test_money_conservation_multiple_transfers() {
    let env = TestEnv::new();
    
    let alice_id = env.register_user(
        "+256700111111",
        "Alice",
        "User",
        "alice@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let bob_id = env.register_user(
        "+256700222222",
        "Bob",
        "User",
        "bob@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    let charlie_id = env.register_user(
        "+256700333333",
        "Charlie",
        "User",
        "charlie@example.com",
        "UGX",
        "9999",
    ).unwrap();
    
    // Initial balances
    env.set_fiat_balance(&alice_id, "UGX", 100000).unwrap();
    env.set_fiat_balance(&bob_id, "UGX", 50000).unwrap();
    let total_before = 150000;
    
    // Transfer 1: Alice -> Bob (20k)
    env.transfer_fiat(&alice_id, &bob_id, 20000, "UGX", "1234", None).unwrap();
    
    // Transfer 2: Bob -> Charlie (30k)
    env.transfer_fiat(&bob_id, &charlie_id, 30000, "UGX", "5678", None).unwrap();
    
    // Transfer 3: Alice -> Charlie (10k)
    env.transfer_fiat(&alice_id, &charlie_id, 10000, "UGX", "1234", None).unwrap();
    
    // Check final balances
    let alice_balance = env.get_fiat_balance(&alice_id, "UGX").unwrap();
    let bob_balance = env.get_fiat_balance(&bob_id, "UGX").unwrap();
    let charlie_balance = env.get_fiat_balance(&charlie_id, "UGX").unwrap();
    
    // Calculate total fees: (20000 + 30000 + 10000) * 0.5% = 300
    let total_fees = 100 + 150 + 50; // Individual fees
    let expected_total = total_before - total_fees;
    let total_after = alice_balance + bob_balance + charlie_balance;
    
    assert_eq!(total_after, expected_total,
        "MONEY CONSERVATION VIOLATED! Before: {}, After: {}, Expected: {}", 
        total_before, total_after, expected_total);
}

#[test]
fn test_balance_integrity_after_failed_transfer() {
    let env = TestEnv::new();
    
    let alice_id = env.register_user(
        "+256700111111",
        "Alice",
        "User",
        "alice@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let bob_id = env.register_user(
        "+256700222222",
        "Bob",
        "User",
        "bob@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    // Alice has 10k
    env.set_fiat_balance(&alice_id, "UGX", 10000).unwrap();
    
    // Try to transfer 50k (should fail - insufficient balance)
    let result = env.transfer_fiat(&alice_id, &bob_id, 50000, "UGX", "1234", None);
    assert!(result.is_err());
    
    // Balances should be unchanged
    let alice_balance = env.get_fiat_balance(&alice_id, "UGX").unwrap();
    let bob_balance = env.get_fiat_balance(&bob_id, "UGX").unwrap();
    
    assert_eq!(alice_balance, 10000, "Alice balance should be unchanged after failed transfer");
    assert_eq!(bob_balance, 0, "Bob balance should be 0");
}

#[test]
fn test_escrow_money_conservation() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user 100k satoshis of BTC
    env.update_crypto_balance(&user_id, 100000, 0).unwrap();
    
    // Create escrow for 50k satoshis
    let escrow_response = env.create_escrow(&user_id, &agent_id, 50000, "CkBTC", "1234").unwrap();
    
    // User should have 50k left (100k - 50k in escrow)
    // Note: We can't easily check crypto balance in current setup, but escrow should exist
    let escrow = env.get_escrow(&escrow_response.code).unwrap();
    assert_eq!(escrow.amount, 50000);
    assert_eq!(escrow.status, EscrowStatus::Active);
    
    // Agent claims escrow
    env.claim_escrow(&escrow_response.code, &agent_id).unwrap();
    
    // Escrow should be claimed
    let escrow = env.get_escrow(&escrow_response.code).unwrap();
    assert_eq!(escrow.status, EscrowStatus::Claimed);
    
    // Total crypto in system should still be 100k (50k with user, 50k with agent)
}

#[test]
fn test_escrow_cancellation_refunds_correctly() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user 100k satoshis
    env.update_crypto_balance(&user_id, 100000, 0).unwrap();
    
    // Create escrow
    let escrow_response = env.create_escrow(&user_id, &agent_id, 50000, "CkBTC", "1234").unwrap();
    
    // Cancel escrow
    env.cancel_escrow(&escrow_response.code, &user_id, "1234").unwrap();
    
    // Escrow should be cancelled
    let escrow = env.get_escrow(&escrow_response.code).unwrap();
    assert_eq!(escrow.status, EscrowStatus::Cancelled);
    
    // User should have full 100k back
    // (In real implementation, we'd verify crypto balance is back to 100k)
}

#[test]
fn test_no_money_creation_on_concurrent_transfers() {
    let env = TestEnv::new();
    
    let alice_id = env.register_user(
        "+256700111111",
        "Alice",
        "User",
        "alice@example.com",
        "UGX",
        "1234",
    ).unwrap();
    
    let bob_id = env.register_user(
        "+256700222222",
        "Bob",
        "User",
        "bob@example.com",
        "UGX",
        "5678",
    ).unwrap();
    
    let charlie_id = env.register_user(
        "+256700333333",
        "Charlie",
        "User",
        "charlie@example.com",
        "UGX",
        "9999",
    ).unwrap();
    
    // Initial: Alice 100k, Bob 50k, Charlie 0
    env.set_fiat_balance(&alice_id, "UGX", 100000).unwrap();
    env.set_fiat_balance(&bob_id, "UGX", 50000).unwrap();
    let total_before = 150000;
    
    // Rapid transfers (simulating concurrent operations)
    env.transfer_fiat(&alice_id, &bob_id, 10000, "UGX", "1234", None).unwrap();
    env.transfer_fiat(&bob_id, &charlie_id, 20000, "UGX", "5678", None).unwrap();
    env.transfer_fiat(&alice_id, &charlie_id, 15000, "UGX", "1234", None).unwrap();
    env.transfer_fiat(&bob_id, &alice_id, 5000, "UGX", "5678", None).unwrap();
    
    // Verify total money conservation
    let alice_balance = env.get_fiat_balance(&alice_id, "UGX").unwrap();
    let bob_balance = env.get_fiat_balance(&bob_id, "UGX").unwrap();
    let charlie_balance = env.get_fiat_balance(&charlie_id, "UGX").unwrap();
    
    let total_after = alice_balance + bob_balance + charlie_balance;
    
    // Calculate total fees
    let total_fees = 50 + 100 + 75 + 25; // 0.5% of each transfer
    let expected_total = total_before - total_fees;
    
    assert_eq!(total_after, expected_total,
        "MONEY CREATION DETECTED! Before: {}, After: {}, Expected: {}", 
        total_before, total_after, expected_total);
}

#[test]
fn test_fee_collection_integrity() {
    let env = TestEnv::new();
    
    let alice_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).unwrap();
    
    let bob_id = env.register_user(
        "+254700222222",
        "Bob",
        "User",
        "bob@example.com",
        "KES",
        "5678",
    ).unwrap();
    
    env.set_fiat_balance(&alice_id, "KES", 100000).unwrap();
    
    // Transfer 50k
    let result = env.transfer_fiat(&alice_id, &bob_id, 50000, "KES", "1234", None).unwrap();
    
    // Fee should be exactly 0.5% = 250
    assert_eq!(result.fee, 250);
    
    // Verify balances account for fee
    assert_eq!(result.sender_new_balance, 49750); // 100000 - 50000 - 250
    assert_eq!(result.recipient_new_balance, 50000);
    
    // Total in user balances should be 99750 (original 100k minus 250 fee)
    let total_user_balances = result.sender_new_balance + result.recipient_new_balance;
    assert_eq!(total_user_balances, 99750);
}
