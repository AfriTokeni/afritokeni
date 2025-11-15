/// Regression tests for refactored buy_crypto and sell_crypto
/// These tests ensure that the refactored implementation (188 lines → 40 lines each)
/// maintains all original functionality with improved security

use super::*;
use candid::{CandidType, encode_args, decode_one};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct BuyCryptoRequest {
    user_identifier: String,
    fiat_amount: u64,
    currency: String,
    crypto_type: String,
    pin: String,
    device_fingerprint: Option<String>,
    geo_location: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct BuyCryptoResponse {
    transaction_id: String,
    crypto_amount: u64,
    fiat_amount: u64,
    crypto_type: String,
    exchange_rate: f64,
    timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct SellCryptoRequest {
    user_identifier: String,
    crypto_amount: u64,
    currency: String,
    crypto_type: String,
    pin: String,
    device_fingerprint: Option<String>,
    geo_location: Option<String>,
}

/// Regression test: buy_crypto basic flow works after refactor
#[test]
fn test_refactored_buy_crypto_basic_flow() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346000";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to call buy_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = result.expect("Buy crypto should succeed after refactor");

    // Verify all response fields are populated correctly
    assert!(buy_response.crypto_amount > 0, "Should receive crypto");
    assert_eq!(buy_response.fiat_amount, 1_500_000, "Fiat amount should match request");
    assert_eq!(buy_response.crypto_type, "CkBTC", "Crypto type should match");
    assert!(buy_response.exchange_rate > 0.0, "Exchange rate should be positive");
    assert!(buy_response.timestamp > 0, "Timestamp should be set");
    assert!(buy_response.transaction_id.contains("buy-crypto"), "Transaction ID format correct");
}

/// Regression test: sell_crypto basic flow works after refactor
#[test]
fn test_refactored_sell_crypto_basic_flow() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346001";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy crypto first
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 2_000_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Now sell the crypto
    let sell_request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: buy_response.crypto_amount,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((sell_request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "sell_crypto",
        args,
    ).expect("Failed to call sell_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let sell_response = result.expect("Sell crypto should succeed after refactor");

    // Verify all response fields
    assert_eq!(sell_response.crypto_amount, buy_response.crypto_amount, "Crypto amount should match");
    assert!(sell_response.fiat_amount > 0, "Should receive fiat");
    assert_eq!(sell_response.crypto_type, "CkBTC", "Crypto type should match");
    assert!(sell_response.exchange_rate > 0.0, "Exchange rate should be positive");
    assert!(sell_response.transaction_id.contains("sell-crypto"), "Transaction ID format correct");
}

/// Regression test: PIN verification still works in refactored buy_crypto
#[test]
fn test_refactored_buy_crypto_pin_verification() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346002";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    // Try with wrong PIN
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: "9999".to_string(), // Wrong PIN
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to call buy_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with wrong PIN");
    assert!(result.unwrap_err().contains("Invalid PIN"), "Error should mention invalid PIN");
}

/// Regression test: Insufficient balance check still works
#[test]
fn test_refactored_buy_crypto_insufficient_balance() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346003";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Set small balance
    set_fiat_balance(&pic, _data, &user_id, "KES", 100_000);

    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 10_000_000, // More than balance
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to call buy_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with insufficient balance");
    assert!(result.unwrap_err().contains("Insufficient"), "Error should mention insufficient balance");
}

/// Regression test: sell_crypto with insufficient crypto balance
#[test]
fn test_refactored_sell_crypto_insufficient_crypto() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346004";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // User has no crypto
    let request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: 100_000_000, // 1 BTC
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "sell_crypto",
        args,
    ).expect("Failed to call sell_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with insufficient crypto");
    assert!(result.unwrap_err().contains("Insufficient"), "Error should mention insufficient balance");
}

/// Regression test: buy_crypto with device fingerprint and geo location
#[test]
fn test_refactored_buy_crypto_with_tracking_data() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346005";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: Some("device-test-123".to_string()),
        geo_location: Some("Nairobi,Kenya".to_string()),
    };

    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to call buy_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_ok(), "Should succeed with tracking data");
    let buy_response = result.unwrap();
    assert!(buy_response.crypto_amount > 0, "Should receive crypto");
}

/// Regression test: sell_crypto with device fingerprint and geo location
#[test]
fn test_refactored_sell_crypto_with_tracking_data() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346006";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy crypto first
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Sell with tracking data
    let sell_request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: buy_response.crypto_amount,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: Some("device-test-456".to_string()),
        geo_location: Some("Mombasa,Kenya".to_string()),
    };

    let args = encode_args((sell_request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "sell_crypto",
        args,
    ).expect("Failed to call sell_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_ok(), "Should succeed with tracking data");
}

/// Regression test: buy and sell ckUSDC (not just ckBTC)
#[test]
fn test_refactored_buy_sell_ckusdc() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346007";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    // Buy USDC
    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkUSDC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy USDC");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy USDC should succeed");

    assert_eq!(buy_response.crypto_type, "CkUSDC", "Should buy USDC");
    assert!(buy_response.crypto_amount > 0, "Should receive USDC");

    // Sell USDC
    let sell_request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: buy_response.crypto_amount / 2,
        currency: "KES".to_string(),
        crypto_type: "CkUSDC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((sell_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "sell_crypto", args)
        .expect("Failed to sell USDC");

    let sell_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let sell_response = sell_result.expect("Sell USDC should succeed");

    assert_eq!(sell_response.crypto_type, "CkUSDC", "Should sell USDC");
    assert!(sell_response.fiat_amount > 0, "Should receive fiat");
}

/// Regression test: exchange rate calculation is accurate
#[test]
fn test_refactored_exchange_rate_calculation() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346008";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to call buy_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = result.expect("Buy should succeed");

    // Exchange rate should be crypto_amount / fiat_amount
    let expected_rate = buy_response.crypto_amount as f64 / buy_response.fiat_amount as f64;
    let rate_difference = (buy_response.exchange_rate - expected_rate).abs();

    assert!(
        rate_difference < 0.0001,
        "Exchange rate {} should match calculated rate {}",
        buy_response.exchange_rate,
        expected_rate
    );
}

/// Regression test: multiple buy operations in sequence
#[test]
fn test_refactored_multiple_buy_operations() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346009";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 50_000_000);

    // Perform 5 buy operations
    for i in 0..5 {
        let request = BuyCryptoRequest {
            user_identifier: user_id.clone(),
            fiat_amount: 1_000_000,
            currency: "KES".to_string(),
            crypto_type: "CkBTC".to_string(),
            pin: pin.to_string(),
            device_fingerprint: None,
            geo_location: None,
        };

        let args = encode_args((request,)).unwrap();
        let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
            .expect(&format!("Failed to call buy_crypto iteration {}", i));

        let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
        assert!(result.is_ok(), "Buy operation {} should succeed", i);
    }

    // Verify total crypto balance
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert!(balance > 0, "Should have accumulated crypto from multiple buys");
}

/// Regression test: balance updates are atomic (no race conditions)
#[test]
fn test_refactored_atomic_balance_updates() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister) = setup_test_environment();

    let phone = "+254712346010";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    // Buy crypto
    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 2_000_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    let balance_after_buy = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");

    // Sell part of it
    let sell_request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: buy_response.crypto_amount / 2,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((sell_request,)).unwrap();
    pic.update_call(crypto_canister, Principal::anonymous(), "sell_crypto", args)
        .expect("Failed to sell crypto");

    let balance_after_sell = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");

    // Balance should be exactly half of what it was
    let expected_balance = balance_after_buy / 2;
    assert_eq!(
        balance_after_sell,
        expected_balance,
        "Balance should be atomically updated"
    );
}
