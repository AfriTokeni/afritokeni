/// Integration tests for error message sanitization
/// Ensures that error messages don't leak internal system details like canister IDs,
/// Principal IDs, or internal error codes that could be exploited by attackers

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

#[derive(CandidType, Deserialize, Clone, Debug)]
struct SendCryptoRequest {
    user_identifier: String,
    to_address: String,
    amount: u64,
    crypto_type: String,
    pin: String,
    device_fingerprint: Option<String>,
    geo_location: Option<String>,
}

/// Tests that insufficient balance errors are preserved (user-facing)
/// but don't leak exact internal state
#[test]
fn test_insufficient_balance_error_is_clear() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345900";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Set small balance
    set_fiat_balance(&pic, _data, &user_id, "KES", 100_000); // 1,000 KES

    // Try to buy more than balance
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 10_000_000, // 100,000 KES - more than balance
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

    let error = result.unwrap_err();

    // Error should mention "Insufficient" (user-facing info)
    assert!(error.contains("Insufficient"), "Error should mention insufficient balance");

    // Error should NOT leak canister IDs or Principal IDs
    assert!(!error.contains("-cai"), "Error should not contain canister ID");
    assert!(!error.contains("Principal"), "Error should not contain Principal");
}

/// Tests that invalid PIN errors don't leak user information
#[test]
fn test_invalid_pin_error_is_sanitized() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345901";
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
    assert!(result.is_err(), "Should fail with invalid PIN");

    let error = result.unwrap_err();

    // Error should be generic "Invalid PIN"
    assert!(error.contains("Invalid PIN"), "Error should mention invalid PIN");

    // Error should NOT leak user ID, phone number, or attempt count
    assert!(!error.contains(&user_id), "Error should not contain user ID");
    assert!(!error.contains(phone), "Error should not contain phone number");
    assert!(!error.contains("attempt"), "Error should not mention attempt count");
}

/// Tests that non-existent user errors don't leak system details
#[test]
fn test_user_not_found_error_is_sanitized() {
    let (pic, _data, _user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let request = BuyCryptoRequest {
        user_identifier: "nonexistent-user-12345".to_string(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: "1234".to_string(),
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
    assert!(result.is_err(), "Should fail for non-existent user");

    let error = result.unwrap_err();

    // Error should mention "not found" or "User not found"
    assert!(
        error.contains("not found") || error.contains("Not found"),
        "Error should indicate user not found"
    );

    // Error should NOT leak canister architecture or database details
    assert!(!error.contains("canister"), "Error should not mention canister");
    assert!(!error.contains("database"), "Error should not mention database");
    assert!(!error.contains("storage"), "Error should not mention storage");
}

/// Tests that invalid currency errors are clear but don't leak system info
#[test]
fn test_invalid_currency_error_is_sanitized() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345902";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    // Try with invalid currency
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "INVALID_CURRENCY".to_string(),
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
    assert!(result.is_err(), "Should fail with invalid currency");

    let error = result.unwrap_err();

    // Error should mention invalid currency
    assert!(error.contains("Invalid currency"), "Error should mention invalid currency");

    // Error should NOT leak supported currency list or config details
    // (attackers could use this to enumerate system capabilities)
    assert!(!error.contains("supported:"), "Error should not list all supported currencies");
}

/// Tests that sell crypto with insufficient crypto balance gives clear error
#[test]
fn test_insufficient_crypto_balance_error_is_clear() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345903";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // User has no crypto balance
    // Try to sell crypto they don't have
    let request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: 100_000_000, // 1 BTC in satoshis
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
    assert!(result.is_err(), "Should fail with insufficient crypto balance");

    let error = result.unwrap_err();

    // Error should mention insufficient balance
    assert!(
        error.contains("Insufficient") || error.contains("insufficient"),
        "Error should mention insufficient balance"
    );

    // Error should NOT leak ledger canister IDs or internal balance tracking details
    assert!(!error.contains("ledger"), "Error should not mention ledger implementation");
    assert!(!error.contains("-cai"), "Error should not contain canister ID");
}

/// Tests that send crypto to invalid address gives sanitized error
#[test]
fn test_send_crypto_invalid_address_error_is_sanitized() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345904";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Buy some crypto first
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
    pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");

    // Try to send to invalid address
    let send_request = SendCryptoRequest {
        user_identifier: user_id.clone(),
        to_address: "invalid-address-format".to_string(),
        amount: 1000,
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: None,
        geo_location: None,
    };

    let args = encode_args((send_request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "send_crypto",
        args,
    ).expect("Failed to call send_crypto");

    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with invalid address");

    let error = result.unwrap_err();

    // Error should mention invalid address or format
    assert!(
        error.contains("Invalid") || error.contains("invalid") || error.contains("format"),
        "Error should indicate address issue: {}",
        error
    );

    // Error should NOT leak ledger implementation details
    assert!(!error.contains("ICRC"), "Error should not mention ICRC standard");
    assert!(!error.contains("canister"), "Error should not mention canister architecture");
}

/// Tests that errors don't leak device fingerprint or geo location data
#[test]
fn test_errors_dont_leak_tracking_data() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345905";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    let device_fingerprint = "device-abc123-xyz789".to_string();
    let geo_location = "Nairobi,Kenya,GPS:1.2345,36.7890".to_string();

    // Try to buy with insufficient balance (to trigger error)
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 100_000_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
        device_fingerprint: Some(device_fingerprint.clone()),
        geo_location: Some(geo_location.clone()),
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

    let error = result.unwrap_err();

    // Error should NOT leak device fingerprint or geo location
    // (these are internal tracking mechanisms)
    assert!(
        !error.contains(&device_fingerprint),
        "Error should not contain device fingerprint"
    );
    assert!(
        !error.contains(&geo_location),
        "Error should not contain geo location"
    );
    assert!(!error.contains("GPS"), "Error should not contain GPS data");
}

/// Tests that rate limit errors are generic and don't leak threshold values
#[test]
fn test_rate_limit_error_is_generic() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    let phone = "+254712345906";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    set_fiat_balance(&pic, _data, &user_id, "KES", 100_000_000);

    // Make multiple rapid requests to potentially trigger rate limit
    // Note: Rate limit logic may not trigger in test mode, but we test the error format
    for _i in 0..20 {
        let request = BuyCryptoRequest {
            user_identifier: user_id.clone(),
            fiat_amount: 100_000,
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

        if result.is_err() {
            let error = result.unwrap_err();

            // If rate limit error, verify it's sanitized
            if error.contains("rate limit") || error.contains("Rate limit") {
                // Error should NOT leak specific thresholds or timing windows
                assert!(
                    !error.contains("per minute") && !error.contains("per second"),
                    "Error should not leak rate limit window"
                );
                assert!(
                    !error.contains("10") && !error.contains("requests"),
                    "Error should not leak exact threshold count"
                );
            }
        }
    }
}
