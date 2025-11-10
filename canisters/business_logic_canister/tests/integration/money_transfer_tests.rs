use super::*;

#[test]
fn test_full_money_transfer_flow() {
    let env = TestEnv::new();
    
    // 1. Register sender
    let sender_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Alice",
        "Sender",
        "alice@example.com",
        "UGX",
        "1234",
    ).expect("Sender registration should succeed");
    
    // 2. Register receiver
    let receiver_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Bob",
        "Receiver",
        "bob@example.com",
        "UGX",
        "5678",
    ).expect("Receiver registration should succeed");
    
    // 3. Give sender initial balance (simulating deposit from agent)
    env.set_fiat_balance(&sender_id, "UGX", 100000)
        .expect("Should set balance");
    
    // 4. Verify sender balance
    let sender_balance_before = env.check_fiat_balance(&sender_id, "UGX")
        .expect("Should check balance");
    assert_eq!(sender_balance_before, 100000);
    
    // 5. Send money from Alice to Bob
    let tx_result = env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        50000,
        "UGX",
        "1234",
    ).expect("Transfer should succeed");
    
    // 6. Verify transaction result
    assert_eq!(tx_result.amount, 50000);
    assert_eq!(tx_result.from_user, sender_id);
    assert_eq!(tx_result.to_user, receiver_id);
    assert_eq!(tx_result.new_balance, 50000); // Sender's new balance
    
    // 7. Verify final balances in data canister
    let sender_balance_after = env.check_fiat_balance(&sender_id, "UGX")
        .expect("Should check sender balance");
    let receiver_balance_after = env.check_fiat_balance(&receiver_id, "UGX")
        .expect("Should check receiver balance");
    
    assert_eq!(sender_balance_after, 50000, "Sender should have 50000 left");
    assert_eq!(receiver_balance_after, 50000, "Receiver should have 50000");
    
    // 8. Verify transaction was recorded
    let sender_txs = env.get_transaction_history(&sender_id, None, None)
        .expect("Should get sender transactions");
    assert_eq!(sender_txs.len(), 1, "Sender should have 1 transaction");
    
    let receiver_txs = env.get_transaction_history(&receiver_id, None, None)
        .expect("Should get receiver transactions");
    assert_eq!(receiver_txs.len(), 1, "Receiver should have 1 transaction");
}

#[test]
fn test_transfer_insufficient_balance_fails() {
    let env = TestEnv::new();
    
    // Register users
    let sender_id = env.register_user(
        Some("+256700333333".to_string()),
        None,
        "Poor",
        "User",
        "poor@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    env.register_user(
        Some("+256700444444".to_string()),
        None,
        "Rich",
        "User",
        "rich@example.com",
        "UGX",
        "5678",
    ).expect("Should register");
    
    // Give sender only 10000
    env.set_fiat_balance(&sender_id, "UGX", 10000).expect("Should set balance");
    
    // Try to send 50000 - should fail
    let result = env.send_money_to_phone(
        "+256700333333",
        "+256700444444",
        50000,
        "UGX",
        "1234",
    );
    
    assert!(result.is_err(), "Should fail with insufficient balance");
    assert!(result.unwrap_err().contains("Insufficient"));
}

#[test]
fn test_transfer_wrong_pin_fails() {
    let env = TestEnv::new();
    
    // Register users
    let sender_id = env.register_user(
        Some("+256700555555".to_string()),
        None,
        "Alice",
        "User",
        "alice2@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    env.register_user(
        Some("+256700666666".to_string()),
        None,
        "Bob",
        "User",
        "bob2@example.com",
        "UGX",
        "5678",
    ).expect("Should register");
    
    env.set_fiat_balance(&sender_id, "UGX", 100000).expect("Should set balance");
    
    // Try with wrong PIN
    let result = env.send_money_to_phone(
        "+256700555555",
        "+256700666666",
        50000,
        "UGX",
        "9999", // Wrong PIN!
    );
    
    assert!(result.is_err(), "Should fail with wrong PIN");
    assert!(result.unwrap_err().contains("PIN"));
}

#[test]
fn test_transfer_to_nonexistent_user_fails() {
    let env = TestEnv::new();
    
    let sender_id = env.register_user(
        Some("+256700777777".to_string()),
        None,
        "Alice",
        "User",
        "alice3@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    env.set_fiat_balance(&sender_id, "UGX", 100000).expect("Should set balance");
    
    // Try to send to non-existent phone
    let result = env.send_money_to_phone(
        "+256700777777",
        "+256700999999", // Doesn't exist
        50000,
        "UGX",
        "1234",
    );
    
    assert!(result.is_err(), "Should fail for nonexistent receiver");
}
