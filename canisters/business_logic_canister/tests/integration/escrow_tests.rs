use crate::integration::TestEnv;
use shared_types::*;

// ============================================================================
// ESCROW SYSTEM TESTS (CRITICAL 🔴)
// ============================================================================
// These tests verify the escrow system atomicity and prevent the critical bug
// where crypto could be deducted but escrow metadata lost if Juno write fails.
// ALL escrow logic is now in Business Logic Canister for atomicity.
// ============================================================================

#[test]
fn test_create_escrow_with_valid_inputs() {
    let env = TestEnv::new();
    
    // Register user with crypto balance
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    // Register agent
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 1_000_000, 500_000).unwrap();
    
    // Create escrow
    let escrow_code = env.create_escrow(
        "+254700000001",
        100_000,
        CryptoType::CkBTC,
        &agent_id,
        "1234",
    ).unwrap();
    
    // Verify escrow code format (BTC-XXXXXX or USD-XXXXXX)
    assert!(escrow_code.starts_with("BTC-") || escrow_code.starts_with("USD-"));
    assert_eq!(escrow_code.len(), 10); // "BTC-123456"
    
    // Verify crypto was deducted from user
    let (btc_balance, _) = env.get_crypto_balance(&user_id).unwrap();
    assert_eq!(btc_balance, 900_000, "Crypto should be deducted from user");
    
    // Verify escrow status
    let escrow = env.get_escrow_status(&escrow_code).unwrap();
    assert_eq!(escrow.status, EscrowStatus::Active);
    assert_eq!(escrow.amount, 100_000);
    assert_eq!(escrow.user_id, user_id);
    assert_eq!(escrow.agent_id, agent_id);
}

#[test]
fn test_generate_unique_escrow_codes() {
    let env = TestEnv::new();
    
    // Register user and agent
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 10_000_000, 5_000_000).unwrap();
    
    // Create multiple escrows
    let code1 = env.create_escrow("+254700000001", 100_000, CryptoType::CkBTC, &agent_id, "1234").unwrap();
    let code2 = env.create_escrow("+254700000001", 200_000, CryptoType::CkUSDC, &agent_id, "1234").unwrap();
    let code3 = env.create_escrow("+254700000001", 300_000, CryptoType::CkBTC, &agent_id, "1234").unwrap();
    
    // Verify codes are unique
    assert_ne!(code1, code2);
    assert_ne!(code2, code3);
    assert_ne!(code1, code3);
    
    // Verify correct prefixes
    assert!(code1.starts_with("BTC-"));
    assert!(code2.starts_with("USD-"));
    assert!(code3.starts_with("BTC-"));
}

#[test]
fn test_crypto_locked_in_escrow() {
    let env = TestEnv::new();
    
    // Register user and agent
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 1_000_000, 0).unwrap();
    
    let initial_balance = env.get_crypto_balance(&user_id).unwrap().0;
    
    // Create escrow
    let _escrow_code = env.create_escrow(
        "+254700000001",
        300_000,
        CryptoType::CkBTC,
        &agent_id,
        "1234",
    ).unwrap();
    
    // Verify crypto is deducted (locked)
    let new_balance = env.get_crypto_balance(&user_id).unwrap().0;
    assert_eq!(new_balance, initial_balance - 300_000, "Crypto should be locked in escrow");
    
    // User should not be able to spend locked crypto
    let result = env.send_crypto(
        "+254700000001",
        "aaaaa-aa", // dummy address
        800_000, // More than available balance
        CryptoType::CkBTC,
        "1234",
    );
    
    assert!(result.is_err(), "Should not be able to spend locked crypto");
}

#[test]
fn test_agent_can_claim_escrow_with_valid_code() {
    let env = TestEnv::new();
    
    // Register user and agent
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 1_000_000, 0).unwrap();
    
    // Create escrow
    let escrow_code = env.create_escrow(
        "+254700000001",
        200_000,
        CryptoType::CkBTC,
        &agent_id,
        "1234",
    ).unwrap();
    
    let agent_initial_balance = env.get_crypto_balance(&agent_id).unwrap().0;
    
    // Agent claims escrow
    let result = env.verify_escrow_code(&escrow_code, &agent_id).unwrap();
    
    assert_eq!(result.amount, 200_000);
    assert_eq!(result.to_user, agent_id);
    
    // Verify agent received crypto
    let agent_new_balance = env.get_crypto_balance(&agent_id).unwrap().0;
    assert_eq!(agent_new_balance, agent_initial_balance + 200_000, "Agent should receive crypto");
    
    // Verify escrow status changed to Claimed
    let escrow = env.get_escrow_status(&escrow_code).unwrap();
    assert_eq!(escrow.status, EscrowStatus::Claimed);
    assert!(escrow.claimed_at.is_some());
}

#[test]
fn test_invalid_code_rejection() {
    let env = TestEnv::new();
    
    // Register agent
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Try to claim with invalid code
    let result = env.verify_escrow_code("BTC-999999", &agent_id);
    
    assert!(result.is_err(), "Invalid code should be rejected");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found") || err_msg.contains("Invalid"));
}

#[test]
fn test_wrong_agent_cannot_claim_escrow() {
    let env = TestEnv::new();
    
    // Register user and two agents
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent1_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    let agent2_id = env.register_user(
        Some("+254700000003".to_string()),
        None,
        "Charlie",
        "Agent",
        "charlie@agent.com",
        "KES",
        "9012",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 1_000_000, 0).unwrap();
    
    // Create escrow for agent1
    let escrow_code = env.create_escrow(
        "+254700000001",
        200_000,
        CryptoType::CkBTC,
        &agent1_id,
        "1234",
    ).unwrap();
    
    // Agent2 tries to claim
    let result = env.verify_escrow_code(&escrow_code, &agent2_id);
    
    assert!(result.is_err(), "Wrong agent should not be able to claim");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not authorized") || err_msg.contains("Wrong agent"));
}

#[test]
fn test_cannot_double_claim_escrow() {
    let env = TestEnv::new();
    
    // Register user and agent
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 1_000_000, 0).unwrap();
    
    // Create escrow
    let escrow_code = env.create_escrow(
        "+254700000001",
        200_000,
        CryptoType::CkBTC,
        &agent_id,
        "1234",
    ).unwrap();
    
    // First claim succeeds
    let result1 = env.verify_escrow_code(&escrow_code, &agent_id);
    assert!(result1.is_ok(), "First claim should succeed");
    
    // Second claim fails
    let result2 = env.verify_escrow_code(&escrow_code, &agent_id);
    assert!(result2.is_err(), "Second claim should fail");
    let err_msg = result2.unwrap_err();
    assert!(err_msg.contains("already claimed") || err_msg.contains("Claimed"));
}

#[test]
fn test_escrow_atomicity_rollback_on_failure() {
    let env = TestEnv::new();
    
    // Register user with insufficient crypto
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user only 50,000 crypto
    env.set_crypto_balance(&user_id, 50_000, 0).unwrap();
    
    let initial_balance = env.get_crypto_balance(&user_id).unwrap().0;
    
    // Try to create escrow for 100,000 (more than balance)
    let result = env.create_escrow(
        "+254700000001",
        100_000,
        CryptoType::CkBTC,
        &agent_id,
        "1234",
    );
    
    assert!(result.is_err(), "Should fail due to insufficient balance");
    
    // Verify balance unchanged (atomicity)
    let final_balance = env.get_crypto_balance(&user_id).unwrap().0;
    assert_eq!(final_balance, initial_balance, "Balance should be unchanged on failure");
}

#[test]
fn test_user_can_cancel_unclaimed_escrow() {
    let env = TestEnv::new();
    
    // Register user and agent
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 1_000_000, 0).unwrap();
    
    let initial_balance = env.get_crypto_balance(&user_id).unwrap().0;
    
    // Create escrow
    let escrow_code = env.create_escrow(
        "+254700000001",
        200_000,
        CryptoType::CkBTC,
        &agent_id,
        "1234",
    ).unwrap();
    
    // User cancels escrow
    let result = env.cancel_escrow(&escrow_code, &user_id, "1234");
    assert!(result.is_ok(), "User should be able to cancel escrow");
    
    // Verify crypto refunded
    let final_balance = env.get_crypto_balance(&user_id).unwrap().0;
    assert_eq!(final_balance, initial_balance, "Crypto should be refunded");
    
    // Verify escrow status
    let escrow = env.get_escrow_status(&escrow_code).unwrap();
    assert_eq!(escrow.status, EscrowStatus::Cancelled);
}

#[test]
fn test_multiple_concurrent_escrows() {
    let env = TestEnv::new();
    
    // Register user and multiple agents
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "Alice",
        "User",
        "alice@test.com",
        "KES",
        "1234",
    ).unwrap();
    
    let agent1_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "Bob",
        "Agent",
        "bob@agent.com",
        "KES",
        "5678",
    ).unwrap();
    
    let agent2_id = env.register_user(
        Some("+254700000003".to_string()),
        None,
        "Charlie",
        "Agent",
        "charlie@agent.com",
        "KES",
        "9012",
    ).unwrap();
    
    // Give user crypto
    env.set_crypto_balance(&user_id, 10_000_000, 5_000_000).unwrap();
    
    // Create multiple escrows
    let code1 = env.create_escrow("+254700000001", 100_000, CryptoType::CkBTC, &agent1_id, "1234").unwrap();
    let code2 = env.create_escrow("+254700000001", 200_000, CryptoType::CkUSDC, &agent2_id, "1234").unwrap();
    let code3 = env.create_escrow("+254700000001", 150_000, CryptoType::CkBTC, &agent1_id, "1234").unwrap();
    
    // Verify all escrows are active
    assert_eq!(env.get_escrow_status(&code1).unwrap().status, EscrowStatus::Active);
    assert_eq!(env.get_escrow_status(&code2).unwrap().status, EscrowStatus::Active);
    assert_eq!(env.get_escrow_status(&code3).unwrap().status, EscrowStatus::Active);
    
    // Verify balances deducted correctly
    let (btc_balance, usdc_balance) = env.get_crypto_balance(&user_id).unwrap();
    assert_eq!(btc_balance, 10_000_000 - 100_000 - 150_000, "BTC should be locked");
    assert_eq!(usdc_balance, 5_000_000 - 200_000, "USDC should be locked");
    
    // Agents claim their escrows
    env.verify_escrow_code(&code1, &agent1_id).unwrap();
    env.verify_escrow_code(&code2, &agent2_id).unwrap();
    
    // Verify claims
    assert_eq!(env.get_escrow_status(&code1).unwrap().status, EscrowStatus::Claimed);
    assert_eq!(env.get_escrow_status(&code2).unwrap().status, EscrowStatus::Claimed);
    assert_eq!(env.get_escrow_status(&code3).unwrap().status, EscrowStatus::Active);
}
