use super::*;

#[test]
fn test_create_and_claim_escrow() {
    let env = TestEnv::new();
    
    // Register user
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    // Register agent
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    // Give user some crypto
    env.update_crypto_balance(&user_id, 100000, 0).expect("Should set crypto balance");
    
    // Create escrow
    let escrow_response = env.create_escrow(&user_id, &agent_id, 50000, "CkBTC", "1234")
        .expect("Escrow creation should succeed");
    
    assert_eq!(escrow_response.amount, 50000);
    assert_eq!(escrow_response.crypto_type, "CkBTC");
    assert!(!escrow_response.code.is_empty());
    
    // Verify escrow exists
    let escrow = env.get_escrow(&escrow_response.code).expect("Should get escrow");
    assert_eq!(escrow.amount, 50000);
    assert_eq!(escrow.user_id, user_id);
    assert_eq!(escrow.agent_id, agent_id);
    assert_eq!(escrow.status, EscrowStatus::Active);
    
    // Agent claims escrow
    env.claim_escrow(&escrow_response.code, &agent_id).expect("Claim should succeed");
    
    // Verify escrow is claimed
    let escrow = env.get_escrow(&escrow_response.code).expect("Should get escrow");
    assert_eq!(escrow.status, EscrowStatus::Claimed);
}

#[test]
fn test_cancel_escrow() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    env.update_crypto_balance(&user_id, 100000, 0).expect("Should set crypto balance");
    
    // Create escrow
    let escrow_response = env.create_escrow(&user_id, &agent_id, 50000, "CkBTC", "1234")
        .expect("Escrow creation should succeed");
    
    // User cancels escrow
    env.cancel_escrow(&escrow_response.code, &user_id, "1234").expect("Cancel should succeed");
    
    // Verify escrow is cancelled
    let escrow = env.get_escrow(&escrow_response.code).expect("Should get escrow");
    assert_eq!(escrow.status, EscrowStatus::Cancelled);
}

#[test]
fn test_escrow_wrong_agent_cannot_claim() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    let agent1_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent1",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    let agent2_id = env.register_user(
        "+254700333333",
        "Charlie",
        "Agent2",
        "charlie@example.com",
        "KES",
        "9999",
    ).expect("Registration should succeed");
    
    env.update_crypto_balance(&user_id, 100000, 0).expect("Should set crypto balance");
    
    // Create escrow for agent1
    let escrow_response = env.create_escrow(&user_id, &agent1_id, 50000, "CkBTC", "1234")
        .expect("Escrow creation should succeed");
    
    // Agent2 tries to claim - should fail
    let result = env.claim_escrow(&escrow_response.code, &agent2_id);
    assert!(result.is_err(), "Wrong agent should not be able to claim");
    assert!(result.unwrap_err().contains("not authorized"));
}

#[test]
fn test_escrow_invalid_pin_fails() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    env.update_crypto_balance(&user_id, 100000, 0).expect("Should set crypto balance");
    
    // Try to create escrow with wrong PIN
    let result = env.create_escrow(&user_id, &agent_id, 50000, "CkBTC", "9999");
    assert!(result.is_err(), "Wrong PIN should fail");
    assert!(result.unwrap_err().contains("Invalid PIN"));
}

#[test]
fn test_escrow_zero_amount_fails() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    env.update_crypto_balance(&user_id, 100000, 0).expect("Should set crypto balance");
    
    // Try to create escrow with zero amount
    let result = env.create_escrow(&user_id, &agent_id, 0, "CkBTC", "1234");
    assert!(result.is_err(), "Zero amount should fail");
    assert!(result.unwrap_err().contains("must be greater than 0"));
}

#[test]
fn test_cancel_escrow_wrong_user_fails() {
    let env = TestEnv::new();
    
    let user1_id = env.register_user(
        "+254700111111",
        "Alice",
        "User1",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    let user2_id = env.register_user(
        "+254700222222",
        "Bob",
        "User2",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    let agent_id = env.register_user(
        "+254700333333",
        "Charlie",
        "Agent",
        "charlie@example.com",
        "KES",
        "9999",
    ).expect("Registration should succeed");
    
    env.update_crypto_balance(&user1_id, 100000, 0).expect("Should set crypto balance");
    
    // User1 creates escrow
    let escrow_response = env.create_escrow(&user1_id, &agent_id, 50000, "CkBTC", "1234")
        .expect("Escrow creation should succeed");
    
    // User2 tries to cancel - should fail
    let result = env.cancel_escrow(&escrow_response.code, &user2_id, "5678");
    assert!(result.is_err(), "Wrong user should not be able to cancel");
    assert!(result.unwrap_err().contains("does not own"));
}

#[test]
fn test_escrow_ckusdc() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        "+254700111111",
        "Alice",
        "User",
        "alice@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");
    
    let agent_id = env.register_user(
        "+254700222222",
        "Bob",
        "Agent",
        "bob@example.com",
        "KES",
        "5678",
    ).expect("Registration should succeed");
    
    // Give user USDC
    env.update_crypto_balance(&user_id, 0, 100000).expect("Should set crypto balance");
    
    // Create USDC escrow
    let escrow_response = env.create_escrow(&user_id, &agent_id, 50000, "CkUSDC", "1234")
        .expect("Escrow creation should succeed");
    
    assert_eq!(escrow_response.crypto_type, "CkUSDC");
    
    // Verify escrow
    let escrow = env.get_escrow(&escrow_response.code).expect("Should get escrow");
    assert_eq!(escrow.crypto_type, CryptoType::CkUSDC);
}
