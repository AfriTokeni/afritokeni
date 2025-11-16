use super::*;

#[test]
fn test_successful_fiat_transfer() {
    let env = TestEnv::new();
    
    // Register sender
    let sender_id = env.register_user(
        "+254700111111",
        "Alice",
        "Sender",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Sender registration should succeed");
    
    // Register recipient
    let recipient_id = env.register_user(
        "+254700222222",
        "Bob",
        "Recipient",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Recipient registration should succeed");
    
    // Set sender balance
    env.set_fiat_balance(&sender_id, "KES", 100000).expect("Should set balance");
    
    // Transfer
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        50000,
        "KES",
        "1234",
        Some("Test transfer".to_string()),
    ).expect("Transfer should succeed");
    
    // Verify response
    assert_eq!(result.amount, 50000);
    assert_eq!(result.fee, 250); // 0.5% of 50000
    assert_eq!(result.currency, "KES");
    assert_eq!(result.sender_new_balance, 49750); // 100000 - 50000 - 250
    assert_eq!(result.recipient_new_balance, 50000);
    
    // Verify balances in data canister
    let sender_balance = env.get_fiat_balance(&sender_id, "KES").expect("Should get balance");
    assert_eq!(sender_balance, 49750);
    
    let recipient_balance = env.get_fiat_balance(&recipient_id, "KES").expect("Should get balance");
    assert_eq!(recipient_balance, 50000);
}

#[test]
fn test_transfer_insufficient_balance() {
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
    
    // Set low balance
    env.set_fiat_balance(&sender_id, "KES", 1000).expect("Should set balance");
    
    // Try to transfer more than balance
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        50000,
        "KES",
        "1234",
        None,
    );
    
    assert!(result.is_err(), "Transfer should fail");
    assert!(result.unwrap_err().contains("Insufficient balance"));
}

#[test]
fn test_transfer_invalid_pin() {
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
    
    env.set_fiat_balance(&sender_id, "KES", 100000).expect("Should set balance");
    
    // Try with wrong PIN
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        50000,
        "KES",
        "9999", // Wrong PIN
        None,
    );
    
    assert!(result.is_err(), "Transfer should fail");
    assert!(result.unwrap_err().contains("Invalid PIN"));
}

#[test]
fn test_transfer_to_self_fails() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "KES", 100000).expect("Should set balance");
    
    // Try to transfer to self
    let result = env.transfer_fiat(
        &user_id,
        &user_id,
        50000,
        "KES",
        "1234",
        None,
    );
    
    assert!(result.is_err(), "Transfer to self should fail");
    assert!(result.unwrap_err().contains("Cannot transfer to yourself"));
}

#[test]
fn test_transfer_zero_amount_fails() {
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
    
    env.set_fiat_balance(&sender_id, "KES", 100000).expect("Should set balance");
    
    // Try to transfer zero
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        0,
        "KES",
        "1234",
        None,
    );
    
    assert!(result.is_err(), "Zero amount transfer should fail");
    assert!(result.unwrap_err().contains("Amount must be greater than 0"));
}

#[test]
fn test_transaction_history() {
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
    
    env.set_fiat_balance(&sender_id, "KES", 100000).expect("Should set balance");
    
    // Make transfer
    env.transfer_fiat(
        &sender_id,
        &recipient_id,
        50000,
        "KES",
        "1234",
        Some("Payment 1".to_string()),
    ).expect("Transfer should succeed");
    
    // Get transaction history
    let history = env.get_transaction_history(&sender_id, Some(10), Some(0))
        .expect("Should get history");
    
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].amount, 50000);
    assert_eq!(history[0].from_user, Some(sender_id.clone()));
    assert_eq!(history[0].to_user, Some(recipient_id.clone()));
}

#[test]
fn test_multiple_transfers() {
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
    
    env.set_fiat_balance(&sender_id, "KES", 200000).expect("Should set balance");
    
    // Transfer 1
    env.transfer_fiat(&sender_id, &recipient_id, 50000, "KES", "1234", None)
        .expect("Transfer 1 should succeed");
    
    // Transfer 2
    env.transfer_fiat(&sender_id, &recipient_id, 30000, "KES", "1234", None)
        .expect("Transfer 2 should succeed");
    
    // Verify final balances
    let sender_balance = env.get_fiat_balance(&sender_id, "KES").expect("Should get balance");
    // 200000 - 50000 - 250 - 30000 - 150 = 119600
    assert_eq!(sender_balance, 119600);
    
    let recipient_balance = env.get_fiat_balance(&recipient_id, "KES").expect("Should get balance");
    assert_eq!(recipient_balance, 80000);
}

#[test]
fn test_transfer_different_currencies() {
    let env = TestEnv::new();
    
    // User with UGX
    let user1_id = env.register_user(
        "+256700111111",
        "Alice",
        "User1",
        "alice@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");
    
    // User with KES
    let user2_id = env.register_user(
        "+254700222222",
        "Bob",
        "User2",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user1_id, "UGX", 1000000).expect("Should set balance");
    env.set_fiat_balance(&user2_id, "UGX", 500000).expect("Should set balance");
    
    // Transfer UGX (both users can have UGX balance)
    let result = env.transfer_fiat(&user1_id, &user2_id, 100000, "UGX", "1234", None)
        .expect("Transfer should succeed");
    
    assert_eq!(result.currency, "UGX");
    assert_eq!(result.amount, 100000);
}
