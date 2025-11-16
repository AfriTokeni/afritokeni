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
}

#[test]
fn test_buy_crypto_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    // Register user
    let phone = "+254712345678";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Set fiat balance (100,000 KES = ~$667)
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000); // 100,000 KES in cents
    
    // Buy crypto
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000, // 15,000 KES = ~$100
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to call buy_crypto");
    
    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = result.expect("Buy crypto failed");
    
    // Verify response
    assert!(buy_response.crypto_amount > 0, "Should receive crypto");
    assert_eq!(buy_response.fiat_amount, 1_500_000);
    assert_eq!(buy_response.crypto_type, "CkBTC");
    assert!(buy_response.exchange_rate > 0.0);
    
    // Verify crypto balance
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance, buy_response.crypto_amount, "Crypto balance should match");
}

#[test]
fn test_buy_crypto_insufficient_fiat() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    // Register user
    let phone = "+254712345679";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
    // Set small fiat balance
    set_fiat_balance(&pic, _data, &user_id, "KES", 100_000); // 1,000 KES
    
    // Try to buy more than balance
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 10_000_000, // 100,000 KES - more than balance
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
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

#[test]
fn test_sell_crypto_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();

    // Register user
    let phone = "+254712345680";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);

    // Set fiat balance and buy crypto first
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);

    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };

    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to buy crypto");

    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");

    // Fund user's ledger account so they can transfer tokens when selling
    // User is anonymous in test mode
    fund_user_ledger_account(&pic, ckbtc_ledger, Principal::anonymous(), buy_response.crypto_amount);

    // Now sell half of the crypto
    let sell_amount = buy_response.crypto_amount / 2;
    let sell_request = SellCryptoRequest {
        user_identifier: user_id.clone(),
        crypto_amount: sell_amount,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((sell_request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "sell_crypto",
        args,
    ).expect("Failed to call sell_crypto");
    
    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let sell_response = result.expect("Sell crypto failed");
    
    // Verify response
    assert_eq!(sell_response.crypto_amount, sell_amount);
    assert!(sell_response.fiat_amount > 0, "Should receive fiat");
    
    // Verify crypto balance reduced
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance, buy_response.crypto_amount - sell_amount, "Crypto balance should be reduced");
}

#[test]
fn test_buy_usdc_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    // Register user
    let phone = "+254712345681";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
    // Set fiat balance
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);
    
    // Buy USDC
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000, // 15,000 KES
        currency: "KES".to_string(),
        crypto_type: "CkUSD".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "buy_crypto",
        args,
    ).expect("Failed to call buy_crypto");
    
    let result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = result.expect("Buy USDC failed");
    
    // Verify response
    assert!(buy_response.crypto_amount > 0, "Should receive USDC");
    assert_eq!(buy_response.crypto_type, "CkUSD");
    
    // Verify USDC balance
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkUSD");
    assert_eq!(balance, buy_response.crypto_amount);
}

#[test]
fn test_invalid_pin_buy_crypto() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    // Register user
    let phone = "+254712345682";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
    // Set fiat balance
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);
    
    // Try to buy with wrong PIN
    let request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: "9999".to_string(), // Wrong PIN
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
    assert!(result.unwrap_err().contains("Invalid PIN"), "Error should mention invalid PIN");
}
