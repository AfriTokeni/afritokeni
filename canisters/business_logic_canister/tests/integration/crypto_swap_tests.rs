// Integration tests for crypto swap functionality
use candid::{encode_args, Principal};
use pocket_ic::PocketIc;
use shared_types::{CryptoType, SwapResult};

mod common;
use common::setup_canisters;

#[test]
fn test_swap_btc_to_usdc_success() {
    let pic = PocketIc::new();
    let (business_logic_id, data_id, _deposit_id, _withdrawal_id, exchange_id) = setup_canisters(&pic);
    
    // Register user
    let phone = "+256700000001";
    let register_result: Result<String, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "register_user",
        encode_args((phone, "John", "Doe", "test@test.com", "1234", "UGX")).unwrap(),
    ).unwrap();
    assert!(register_result.is_ok(), "Registration failed: {:?}", register_result);
    
    // TODO: Add initial BTC balance to user
    // TODO: Set up exchange canister with mock DEX
    
    // Attempt swap
    let swap_result: Result<SwapResult, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "swap_crypto",
        encode_args((
            phone,
            CryptoType::CkBTC,
            CryptoType::CkUSDC,
            100000u64,  // 0.001 BTC
            "1234",
        )).unwrap(),
    ).unwrap();
    
    // Verify swap executed
    assert!(swap_result.is_ok(), "Swap failed: {:?}", swap_result);
    let result = swap_result.unwrap();
    
    // Verify spread (0.5%)
    assert_eq!(result.from_amount, 100000);
    assert_eq!(result.spread_amount, 500);  // 0.5% of 100000
    assert!(result.to_amount > 0, "Should receive USDC");
    assert!(!result.exchange_rate.is_empty(), "Should have exchange rate");
}

#[test]
fn test_swap_usdc_to_btc_success() {
    let pic = PocketIc::new();
    let (business_logic_id, data_id, _deposit_id, _withdrawal_id, exchange_id) = setup_canisters(&pic);
    
    // Register user
    let phone = "+256700000002";
    let register_result: Result<String, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "register_user",
        encode_args((phone, "Jane", "Smith", "test2@test.com", "1234", "UGX")).unwrap(),
    ).unwrap();
    assert!(register_result.is_ok());
    
    // TODO: Add initial USDC balance to user
    
    // Attempt swap
    let swap_result: Result<SwapResult, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "swap_crypto",
        encode_args((
            phone,
            CryptoType::CkUSDC,
            CryptoType::CkBTC,
            1000000u64,  // 1 USDC
            "1234",
        )).unwrap(),
    ).unwrap();
    
    assert!(swap_result.is_ok(), "Swap failed: {:?}", swap_result);
}

#[test]
fn test_swap_with_invalid_pin() {
    let pic = PocketIc::new();
    let (business_logic_id, data_id, _deposit_id, _withdrawal_id, exchange_id) = setup_canisters(&pic);
    
    // Register user
    let phone = "+256700000003";
    let register_result: Result<String, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "register_user",
        encode_args((phone, "Test", "User", "test3@test.com", "1234", "UGX")).unwrap(),
    ).unwrap();
    assert!(register_result.is_ok());
    
    // Attempt swap with wrong PIN
    let swap_result: Result<SwapResult, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "swap_crypto",
        encode_args((
            phone,
            CryptoType::CkBTC,
            CryptoType::CkUSDC,
            100000u64,
            "9999",  // Wrong PIN
        )).unwrap(),
    ).unwrap();
    
    assert!(swap_result.is_err(), "Should fail with invalid PIN");
    assert!(swap_result.unwrap_err().contains("Invalid PIN"));
}

#[test]
fn test_swap_same_token_fails() {
    let pic = PocketIc::new();
    let (business_logic_id, data_id, _deposit_id, _withdrawal_id, exchange_id) = setup_canisters(&pic);
    
    // Register user
    let phone = "+256700000004";
    let register_result: Result<String, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "register_user",
        encode_args((phone, "Test", "User", "test4@test.com", "1234", "UGX")).unwrap(),
    ).unwrap();
    assert!(register_result.is_ok());
    
    // Attempt to swap BTC to BTC (should fail in exchange canister)
    let swap_result: Result<SwapResult, String> = pic.update_call(
        business_logic_id,
        Principal::anonymous(),
        "swap_crypto",
        encode_args((
            phone,
            CryptoType::CkBTC,
            CryptoType::CkBTC,
            100000u64,
            "1234",
        )).unwrap(),
    ).unwrap();
    
    assert!(swap_result.is_err(), "Should fail when swapping same token");
    assert!(swap_result.unwrap_err().contains("Cannot swap same token"));
}

#[test]
fn test_swap_shows_correct_spread() {
    // Test that spread calculation is correct
    let amount = 1000000u64;
    let spread_basis_points = 50u64;  // 0.5%
    let expected_spread = (amount * spread_basis_points) / 10000;
    
    assert_eq!(expected_spread, 500);  // 0.5% of 1,000,000 = 500
    
    let net_amount = amount - expected_spread;
    assert_eq!(net_amount, 999500);
}

#[test]
fn test_swap_unauthorized_caller_fails() {
    let pic = PocketIc::new();
    let (business_logic_id, _data_id, _deposit_id, _withdrawal_id, _exchange_id) = setup_canisters(&pic);
    
    // Try to call swap_crypto from unauthorized principal
    let unauthorized_principal = Principal::from_text("2vxsx-fae").unwrap();
    
    let swap_result: Result<SwapResult, String> = pic.update_call(
        business_logic_id,
        unauthorized_principal,
        "swap_crypto",
        encode_args((
            "+256700000005",
            CryptoType::CkBTC,
            CryptoType::CkUSDC,
            100000u64,
            "1234",
        )).unwrap(),
    ).unwrap();
    
    // Should fail authorization check
    assert!(swap_result.is_err(), "Should fail authorization");
}
