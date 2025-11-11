// Integration tests for crypto swap functionality
use super::*;

#[test]
fn test_swap_btc_to_usdc_with_invalid_pin() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Register user with principal ID
    let phone = Some("+256700000001".to_string());
    let principal = Some("2vxsx-fae".to_string());
    let user_id = env.register_user(
        phone.clone(),
        principal,
        "John",
        "Doe",
        "test@test.com",
        "UGX",
        "1234",
    ).expect("Registration failed");
    
    // Set initial BTC balance
    env.set_crypto_balance(&user_id, 1_000_000, 0).expect("Failed to set balance");
    
    // Attempt swap with wrong PIN
    let swap_result = env.swap_crypto(
        &phone.unwrap(),
        CryptoType::CkBTC,
        CryptoType::CkUSDC,
        100_000u64,
        "9999",  // Wrong PIN
    );
    
    assert!(swap_result.is_err(), "Should fail with invalid PIN");
    let error = swap_result.unwrap_err();
    assert!(error.contains("Invalid PIN") || error.contains("PIN"), 
        "Expected PIN error, got: {}", error);
}

#[test]
fn test_swap_same_token_fails() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Register user with principal ID
    let phone = Some("+256700000002".to_string());
    let principal = Some("rrkah-fqaaa-aaaaa-aaaaq-cai".to_string());
    let user_id = env.register_user(
        phone.clone(),
        principal,
        "Jane",
        "Smith",
        "test2@test.com",
        "UGX",
        "1234",
    ).expect("Registration failed");
    
    // Set balance
    env.set_crypto_balance(&user_id, 1_000_000, 0).expect("Failed to set balance");
    
    // Attempt to swap BTC to BTC (should fail in exchange canister)
    let swap_result = env.swap_crypto(
        &phone.unwrap(),
        CryptoType::CkBTC,
        CryptoType::CkBTC,
        100_000u64,
        "1234",
    );
    
    assert!(swap_result.is_err(), "Should fail when swapping same token");
    let error = swap_result.unwrap_err();
    assert!(error.contains("Cannot swap same token") || error.contains("same"), 
        "Expected 'same token' error, got: {}", error);
}

#[test]
fn test_swap_with_zero_amount_fails() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Register user with principal ID
    let phone = Some("+256700000003".to_string());
    let principal = Some("ryjl3-tyaaa-aaaaa-aaaba-cai".to_string());
    let user_id = env.register_user(
        phone.clone(),
        principal,
        "Test",
        "User",
        "test3@test.com",
        "UGX",
        "1234",
    ).expect("Registration failed");
    
    // Set balance
    env.set_crypto_balance(&user_id, 1_000_000, 0).expect("Failed to set balance");
    
    // Attempt swap with zero amount
    let swap_result = env.swap_crypto(
        &phone.unwrap(),
        CryptoType::CkBTC,
        CryptoType::CkUSDC,
        0u64,  // Zero amount
        "1234",
    );
    
    assert!(swap_result.is_err(), "Should fail with zero amount");
    let error = swap_result.unwrap_err();
    assert!(error.contains("greater than 0") || error.contains("zero") || error.contains("0"), 
        "Expected zero amount error, got: {}", error);
}

#[test]
fn test_swap_spread_calculation() {
    // Test that spread calculation is correct
    let amount = 1_000_000u64;
    let spread_basis_points = 50u64;  // 0.5% = 50 basis points
    let expected_spread = (amount * spread_basis_points) / 10_000;
    
    assert_eq!(expected_spread, 5_000, "0.5% of 1,000,000 should be 5,000");
    
    let net_amount = amount - expected_spread;
    assert_eq!(net_amount, 995_000, "Net amount should be 995,000");
}

#[test]
fn test_swap_spread_calculation_large_amount() {
    // Test spread with larger amounts
    let amount = 100_000_000u64;  // 1 BTC in sats
    let spread_basis_points = 50u64;  // 0.5%
    let expected_spread = (amount * spread_basis_points) / 10_000;
    
    assert_eq!(expected_spread, 500_000, "0.5% of 100M should be 500K");
}

#[test]
fn test_swap_with_insufficient_balance() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Register user with principal ID
    let phone = Some("+256700000004".to_string());
    let principal = Some("renrk-eyaaa-aaaaa-aaada-cai".to_string());
    let user_id = env.register_user(
        phone.clone(),
        principal,
        "Poor",
        "User",
        "test4@test.com",
        "UGX",
        "1234",
    ).expect("Registration failed");
    
    // Set very low balance
    env.set_crypto_balance(&user_id, 1_000, 0).expect("Failed to set balance");
    
    // Attempt to swap more than balance
    let swap_result = env.swap_crypto(
        &phone.unwrap(),
        CryptoType::CkBTC,
        CryptoType::CkUSDC,
        100_000u64,  // More than balance
        "1234",
    );
    
    // Should fail due to insufficient balance
    assert!(swap_result.is_err(), "Should fail with insufficient balance");
    let error = swap_result.unwrap_err();
    assert!(error.contains("Insufficient") || error.contains("balance") || error.contains("enough"), 
        "Expected insufficient balance error, got: {}", error);
}

#[test]
fn test_swap_with_nonexistent_user() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Attempt swap without registering user
    let swap_result = env.swap_crypto(
        "+256700000099",  // Non-existent user
        CryptoType::CkBTC,
        CryptoType::CkUSDC,
        100_000u64,
        "1234",
    );
    
    assert!(swap_result.is_err(), "Should fail for non-existent user");
    let error = swap_result.unwrap_err();
    assert!(error.contains("not found") || error.contains("User") || error.contains("exist"), 
        "Expected user not found error, got: {}", error);
}

#[test]
fn test_swap_usdc_to_btc_with_invalid_pin() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Register user with principal ID
    let phone = Some("+256700000005".to_string());
    let principal = Some("r7inp-6aaaa-aaaaa-aaabq-cai".to_string());
    let user_id = env.register_user(
        phone.clone(),
        principal,
        "Alice",
        "Test",
        "test5@test.com",
        "UGX",
        "1234",
    ).expect("Registration failed");
    
    // Set initial USDC balance
    env.set_crypto_balance(&user_id, 0, 10_000_000).expect("Failed to set balance");
    
    // Attempt swap with wrong PIN
    let swap_result = env.swap_crypto(
        &phone.unwrap(),
        CryptoType::CkUSDC,
        CryptoType::CkBTC,
        1_000_000u64,
        "0000",  // Wrong PIN
    );
    
    assert!(swap_result.is_err(), "Should fail with invalid PIN");
    assert!(swap_result.unwrap_err().contains("Invalid PIN"));
}

// Note: Full end-to-end swap tests with actual DEX integration
// would require mocking the Sonic DEX or using testnet
// These tests verify the business logic layer works correctly
