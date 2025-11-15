use super::*;
use candid::{CandidType, encode_args, decode_one};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct SendCryptoRequest {
    user_identifier: String,
    to_address: String,
    amount: u64,
    crypto_type: String,
    pin: String,
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

#[test]
fn test_send_crypto_btc_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    // Register user and buy crypto first
    let phone = "+254712345690";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
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
        .expect("Failed to buy crypto");
    
    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");
    
    // Now send crypto
    let send_request = SendCryptoRequest {
        user_identifier: user_id.clone(),
        to_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(), // Valid BTC address
        amount: buy_response.crypto_amount / 2,
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((send_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "send_crypto", args)
        .expect("Failed to call send_crypto");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    let tx_id = result.expect("Send crypto failed");
    
    assert!(!tx_id.is_empty(), "Should return transaction ID");
    
    // Verify balance reduced
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance, buy_response.crypto_amount / 2, "Balance should be half");
}

#[test]
fn test_send_crypto_usdc_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let phone = "+254712345691";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
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
        .expect("Failed to buy crypto");
    
    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");
    
    // Send USDC
    let send_request = SendCryptoRequest {
        user_identifier: user_id.clone(),
        to_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0".to_string(), // Valid ETH address
        amount: buy_response.crypto_amount / 2,
        crypto_type: "CkUSDC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((send_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "send_crypto", args)
        .expect("Failed to call send_crypto");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    result.expect("Send USDC should succeed");
}

#[test]
fn test_send_crypto_insufficient_balance() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let phone = "+254712345692";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
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
        .expect("Failed to buy crypto");
    
    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");
    
    // Try to send more than balance
    let send_request = SendCryptoRequest {
        user_identifier: user_id.clone(),
        to_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        amount: buy_response.crypto_amount * 2, // More than balance
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((send_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "send_crypto", args)
        .expect("Failed to call send_crypto");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with insufficient balance");
    assert!(result.unwrap_err().contains("Insufficient"), "Error should mention insufficient balance");
}

#[test]
fn test_send_crypto_invalid_address() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let phone = "+254712345693";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);
    
    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((buy_request,)).unwrap();
    pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");
    
    // Try to send to invalid address
    let send_request = SendCryptoRequest {
        user_identifier: user_id.clone(),
        to_address: "invalid_address".to_string(),
        amount: 1000,
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((send_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "send_crypto", args)
        .expect("Failed to call send_crypto");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with invalid address");
    assert!(result.unwrap_err().contains("Invalid"), "Error should mention invalid address");
}

#[test]
fn test_send_crypto_invalid_pin() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let phone = "+254712345694";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, phone, pin);
    
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);
    
    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 1_500_000,
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((buy_request,)).unwrap();
    pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");
    
    // Try to send with wrong PIN
    let send_request = SendCryptoRequest {
        user_identifier: user_id.clone(),
        to_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        amount: 1000,
        crypto_type: "CkBTC".to_string(),
        pin: "9999".to_string(), // Wrong PIN
    };
    
    let args = encode_args((send_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "send_crypto", args)
        .expect("Failed to call send_crypto");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with invalid PIN");
    assert!(result.unwrap_err().contains("Invalid PIN"), "Error should mention invalid PIN");
}
