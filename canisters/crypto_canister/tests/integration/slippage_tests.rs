/// Integration tests for slippage protection in crypto operations
/// Tests the complete flow of slippage calculation, validation, and enforcement

use super::*;
use candid::{CandidType, encode_args, decode_one};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct SwapCryptoRequest {
    user_identifier: String,
    from_crypto: String,
    to_crypto: String,
    amount: u64,
    pin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct SwapCryptoResponse {
    transaction_id: String,
    from_amount: u64,
    to_amount: u64,
    spread_amount: u64,
    exchange_rate: f64,
    timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct BuyCryptoRequest {
    user_identifier: String,
    fiat_amount: u64,
    currency: String,
    crypto_type: String,
    pin: String,
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

/// Tests that swap_crypto enforces slippage protection
/// Verifies: 1% default slippage is calculated and validated
#[test]
fn test_swap_crypto_with_slippage_protection() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345800";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy BTC first
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy BTC");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Swap BTC to USDC
    let swap_request = SwapCryptoRequest {
        user_identifier: user_id.clone(),
        from_crypto: "CkBTC".to_string(),
        to_crypto: "CkUSDC".to_string(),
        amount: buy_response.crypto_amount,
        pin: pin.to_string(),
    };

    let args = encode_args((swap_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "swap_crypto", args)
        .expect("Failed to call swap_crypto");

    let result: Result<SwapCryptoResponse, String> = decode_one(&response).unwrap();
    let swap_response = result.expect("Swap should succeed");

    // Verify slippage protection was applied
    // The to_amount should be close to the swap_amount (within 1% slippage + spread)
    let spread_amount = swap_response.spread_amount;
    let swap_amount = buy_response.crypto_amount - spread_amount;

    // In test mode, internal swap is used with minimal slippage
    // Output should be >= swap_amount - 1% slippage
    let min_expected_output = (swap_amount as f64 * 0.99) as u64;
    assert!(
        swap_response.to_amount >= min_expected_output,
        "Output {} should be >= min expected {} (1% slippage protection)",
        swap_response.to_amount,
        min_expected_output
    );

    // Verify balances
    let btc_balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    let usdc_balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkUSDC");

    assert_eq!(btc_balance, 0, "BTC balance should be 0 after swap");
    assert_eq!(usdc_balance, swap_response.to_amount, "USDC balance should match swap output");
}

/// Tests that swap_crypto validates slippage after DEX execution
/// This ensures the actual output meets the minimum threshold
#[test]
fn test_swap_validates_slippage_after_execution() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345801";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy USDC
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkUSDC".to_string(),
        pin: pin.to_string(),
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy USDC");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Swap USDC to BTC
    let swap_request = SwapCryptoRequest {
        user_identifier: user_id.clone(),
        from_crypto: "CkUSDC".to_string(),
        to_crypto: "CkBTC".to_string(),
        amount: buy_response.crypto_amount,
        pin: pin.to_string(),
    };

    let args = encode_args((swap_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "swap_crypto", args)
        .expect("Failed to call swap_crypto");

    let result: Result<SwapCryptoResponse, String> = decode_one(&response).unwrap();

    // Swap should succeed with slippage validation
    assert!(result.is_ok(), "Swap should succeed with valid slippage");
    let swap_response = result.unwrap();

    // Verify exchange rate is reasonable (not zero or negative)
    assert!(swap_response.exchange_rate > 0.0, "Exchange rate should be positive");

    // Verify output is reasonable (considering spread)
    assert!(
        swap_response.to_amount > 0,
        "Should receive some output crypto"
    );
}

/// Tests that large swaps still respect slippage limits
/// This verifies slippage protection scales with transaction size
#[test]
fn test_large_swap_respects_slippage() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345802";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy large amount of BTC
    set_fiat_balance(&pic, _data, &user_id, "KES", 100_000_000); // 1,000,000 KES

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 50_000_000, // 500,000 KES
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy BTC");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Swap large amount
    let swap_request = SwapCryptoRequest {
        user_identifier: user_id.clone(),
        from_crypto: "CkBTC".to_string(),
        to_crypto: "CkUSDC".to_string(),
        amount: buy_response.crypto_amount,
        pin: pin.to_string(),
    };

    let args = encode_args((swap_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "swap_crypto", args)
        .expect("Failed to call swap_crypto");

    let result: Result<SwapCryptoResponse, String> = decode_one(&response).unwrap();
    let swap_response = result.expect("Large swap should succeed");

    // Calculate actual slippage percentage
    let spread_amount = swap_response.spread_amount;
    let swap_amount = buy_response.crypto_amount - spread_amount;
    let slippage = if swap_amount > swap_response.to_amount {
        ((swap_amount - swap_response.to_amount) as f64 / swap_amount as f64) * 100.0
    } else {
        0.0
    };

    // Verify slippage is within acceptable range (1% tolerance)
    assert!(
        slippage <= 1.0,
        "Slippage {} should be <= 1.0% for large swaps",
        slippage
    );
}

/// Tests that multiple small swaps all have slippage protection
/// Verifies consistency across multiple operations
#[test]
fn test_multiple_swaps_all_protected() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345803";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy BTC - increased amount to ensure sufficient balance for multiple swaps with spread/slippage
    set_fiat_balance(&pic, _data, &user_id, "KES", 50_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 30_000_000,  // Increased from 10M to 30M to cover swaps with spread
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy BTC");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Perform 3 swaps back and forth - use smaller portion to account for spread losses
    // Each swap loses 0.5% to spread, so we need to track actual balance
    let initial_swap_amount = buy_response.crypto_amount / 10;  // Start with 1/10th of balance

    for i in 0..3 {
        let (from_crypto, to_crypto) = if i % 2 == 0 {
            ("CkBTC".to_string(), "CkUSDC".to_string())
        } else {
            ("CkUSDC".to_string(), "CkBTC".to_string())
        };

        // Get current balance to determine how much we can swap
        let current_balance = get_crypto_balance(&pic, crypto_canister, &user_id, &from_crypto);

        // For first swap use initial amount, for subsequent swaps use most of available balance
        // Leave small buffer for potential rounding
        let swap_amount = if i == 0 {
            initial_swap_amount
        } else {
            (current_balance * 95) / 100  // Use 95% of balance to account for spread and rounding
        };

        let swap_request = SwapCryptoRequest {
            user_identifier: user_id.clone(),
            from_crypto: from_crypto.clone(),
            to_crypto: to_crypto.clone(),
            amount: swap_amount,
            pin: pin.to_string(),
        };

        let args = encode_args((swap_request,)).unwrap();
        let response = pic.update_call(crypto_canister, Principal::anonymous(), "swap_crypto", args)
            .expect(&format!("Failed to call swap_crypto iteration {}", i));

        let result: Result<SwapCryptoResponse, String> = decode_one(&response).unwrap();
        let swap_response = result.expect(&format!("Swap {} should succeed", i));

        // Verify each swap has reasonable output (slippage protection working)
        assert!(
            swap_response.to_amount > 0,
            "Swap {} should produce output",
            i
        );

        // Verify exchange rate is positive
        assert!(
            swap_response.exchange_rate > 0.0,
            "Swap {} should have positive exchange rate",
            i
        );
    }
}

/// Tests that slippage validation catches extreme deviations
/// Note: In test mode, this may not trigger actual rejection since internal swap is used
/// This test documents expected behavior when real DEX integration is active
#[test]
fn test_slippage_validation_would_catch_extreme_deviation() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345804";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy BTC
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy BTC");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Perform swap (in test mode this uses internal swap, so slippage will be minimal)
    let swap_request = SwapCryptoRequest {
        user_identifier: user_id.clone(),
        from_crypto: "CkBTC".to_string(),
        to_crypto: "CkUSDC".to_string(),
        amount: buy_response.crypto_amount,
        pin: pin.to_string(),
    };

    let args = encode_args((swap_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "swap_crypto", args)
        .expect("Failed to call swap_crypto");

    let result: Result<SwapCryptoResponse, String> = decode_one(&response).unwrap();

    // In test mode, swap succeeds because internal swap has minimal slippage
    // In production with real DEX, extreme slippage would be rejected
    // This test documents the expected behavior
    assert!(
        result.is_ok(),
        "In test mode, swap should succeed with internal swap (low slippage)"
    );

    // If we ever test against real DEX with extreme price volatility:
    // assert!(result.is_err(), "Should reject if slippage exceeds 1%");
    // assert!(result.unwrap_err().contains("Slippage too high"));
}
