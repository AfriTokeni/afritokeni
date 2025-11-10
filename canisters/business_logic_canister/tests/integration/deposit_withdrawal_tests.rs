use super::*;

#[test]
fn test_full_withdrawal_flow() {
    let env = TestEnv::new();
    
    // 1. Register user
    let user_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Alice",
        "User",
        "alice@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    // 2. Register agent
    let agent_id = env.register_user(
        Some("+256700999999".to_string()),
        None,
        "Agent",
        "Smith",
        "agent@example.com",
        "UGX",
        "5678",
    ).expect("Should register agent");
    
    // 3. Give user initial balance
    env.set_fiat_balance(&user_id, "UGX", 100000)
        .expect("Should set balance");
    
    // 4. User withdraws cash via agent
    let tx_result = env.withdraw_fiat(
        "+256700111111",
        50000,
        "UGX",
        &agent_id,
        "1234",
    ).expect("Withdrawal should succeed");
    
    // 5. Verify transaction
    assert_eq!(tx_result.amount, 50000);
    assert_eq!(tx_result.from_user, user_id);
    assert_eq!(tx_result.to_user, agent_id);
    assert_eq!(tx_result.new_balance, 50000); // User's remaining balance
    
    // 6. Verify balances via business_logic
    let user_balance = env.check_fiat_balance(&user_id, "UGX")
        .expect("Should check balance");
    assert_eq!(user_balance, 50000, "User should have 50000 left");
    
    // 7. DIRECTLY verify balance in data_canister (not through business_logic!)
    let data_balance = env.get_fiat_balance_from_data_canister(&user_id, "UGX")
        .expect("Should get balance from data canister");
    assert_eq!(data_balance, 50000, "Data canister balance should match!");
    
    // 8. Verify transaction recorded in data_canister
    let user_txs = env.get_transaction_history(&user_id, None, None)
        .expect("Should get transactions");
    assert_eq!(user_txs.len(), 1, "Should have 1 withdrawal transaction");
    assert_eq!(user_txs[0].transaction_type, TransactionType::WithdrawFiat);
    
    // 9. DIRECTLY verify transaction in data_canister
    let data_txs = env.get_transactions_from_data_canister(&user_id, None, None)
        .expect("Should get transactions from data canister");
    assert_eq!(data_txs.len(), 1, "Data canister should have 1 transaction");
}

#[test]
fn test_withdrawal_insufficient_balance_fails() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "Poor",
        "User",
        "poor@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    let agent_id = env.register_user(
        Some("+256700888888".to_string()),
        None,
        "Agent",
        "Two",
        "agent2@example.com",
        "UGX",
        "5678",
    ).expect("Should register agent");
    
    // Give user only 10000
    env.set_fiat_balance(&user_id, "UGX", 10000)
        .expect("Should set balance");
    
    // Try to withdraw 50000
    let result = env.withdraw_fiat(
        "+256700222222",
        50000,
        "UGX",
        &agent_id,
        "1234",
    );
    
    assert!(result.is_err(), "Should fail with insufficient balance");
    assert!(result.unwrap_err().contains("Insufficient"));
}

#[test]
fn test_withdrawal_wrong_pin_fails() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700333333".to_string()),
        None,
        "Alice",
        "User",
        "alice2@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    let agent_id = env.register_user(
        Some("+256700777777".to_string()),
        None,
        "Agent",
        "Three",
        "agent3@example.com",
        "UGX",
        "5678",
    ).expect("Should register agent");
    
    env.set_fiat_balance(&user_id, "UGX", 100000)
        .expect("Should set balance");
    
    // Try with wrong PIN
    let result = env.withdraw_fiat(
        "+256700333333",
        50000,
        "UGX",
        &agent_id,
        "9999", // Wrong PIN!
    );
    
    assert!(result.is_err(), "Should fail with wrong PIN");
    assert!(result.unwrap_err().contains("PIN"));
}

#[test]
fn test_multiple_transactions_recorded() {
    let env = TestEnv::new();
    
    // Register 3 users
    let user1_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "User",
        "One",
        "user1@example.com",
        "UGX",
        "1234",
    ).expect("Should register");
    
    let user2_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "User",
        "Two",
        "user2@example.com",
        "UGX",
        "5678",
    ).expect("Should register");
    
    let agent_id = env.register_user(
        Some("+256700999999".to_string()),
        None,
        "Agent",
        "One",
        "agent@example.com",
        "UGX",
        "9999",
    ).expect("Should register");
    
    // Give user1 balance
    env.set_fiat_balance(&user1_id, "UGX", 200000)
        .expect("Should set balance");
    
    // User1 sends to User2
    env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        50000,
        "UGX",
        "1234",
    ).expect("Transfer should succeed");
    
    // User1 withdraws via agent
    env.withdraw_fiat(
        "+256700111111",
        30000,
        "UGX",
        &agent_id,
        "1234",
    ).expect("Withdrawal should succeed");
    
    // User1 sends to User2 again
    env.send_money_to_phone(
        "+256700111111",
        "+256700222222",
        20000,
        "UGX",
        "1234",
    ).expect("Transfer should succeed");
    
    // Verify User1 has 3 transactions
    let user1_txs = env.get_transaction_history(&user1_id, None, None)
        .expect("Should get transactions");
    assert_eq!(user1_txs.len(), 3, "User1 should have 3 transactions");
    
    // Verify User2 has 2 transactions (2 receives)
    let user2_txs = env.get_transaction_history(&user2_id, None, None)
        .expect("Should get transactions");
    assert_eq!(user2_txs.len(), 2, "User2 should have 2 transactions");
    
    // Verify final balances
    let user1_balance = env.check_fiat_balance(&user1_id, "UGX")
        .expect("Should check balance");
    assert_eq!(user1_balance, 100000, "User1: 200000 - 50000 - 30000 - 20000 = 100000");
    
    let user2_balance = env.check_fiat_balance(&user2_id, "UGX")
        .expect("Should check balance");
    assert_eq!(user2_balance, 70000, "User2: 50000 + 20000 = 70000");
}
