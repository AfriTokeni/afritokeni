use super::*;

#[test]
fn test_fraud_detection_blocks_large_transaction() {
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
    
    // Set very high balance
    env.set_fiat_balance(&sender_id, "KES", 20000000).expect("Should set balance");
    
    // Try to transfer amount exceeding max limit (15M KES from config)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        16000000, // Exceeds max_transaction_amount for KES
        "KES",
        "1234",
        None,
    );
    
    assert!(result.is_err(), "Large transaction should be blocked");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("blocked") || error_msg.contains("exceeds"));
}

#[test]
fn test_fraud_detection_allows_normal_transaction() {
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
    
    env.set_fiat_balance(&sender_id, "KES", 1000000).expect("Should set balance");
    
    // Normal transaction (well below limits)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        100000, // Normal amount
        "KES",
        "1234",
        None,
    );
    
    assert!(result.is_ok(), "Normal transaction should succeed");
}

#[test]
fn test_fraud_detection_ugx_limits() {
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
    
    // Set very high balance
    env.set_fiat_balance(&sender_id, "UGX", 500000000).expect("Should set balance");
    
    // Try to transfer amount exceeding UGX max limit (370M UGX from config)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        380000000, // Exceeds max for UGX
        "UGX",
        "1234",
        None,
    );
    
    assert!(result.is_err(), "Transaction exceeding UGX limit should be blocked");
}

#[test]
fn test_fraud_detection_ngn_limits() {
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
    
    env.set_fiat_balance(&sender_id, "NGN", 200000000).expect("Should set balance");
    
    // Try to transfer amount exceeding NGN max limit (150M NGN from config)
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        160000000, // Exceeds max for NGN
        "NGN",
        "1234",
        None,
    );
    
    assert!(result.is_err(), "Transaction exceeding NGN limit should be blocked");
}

#[test]
fn test_fee_calculation_accuracy() {
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
    
    // Transfer 100,000 KES
    let result = env.transfer_fiat(
        &sender_id,
        &recipient_id,
        100000,
        "KES",
        "1234",
        None,
    ).expect("Transfer should succeed");
    
    // Fee should be 0.5% = 500
    assert_eq!(result.fee, 500);
    assert_eq!(result.sender_new_balance, 99500); // 200000 - 100000 - 500
    assert_eq!(result.recipient_new_balance, 100000);
}
