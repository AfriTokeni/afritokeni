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

#[test]
fn test_swap_btc_to_usdc_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let phone = "+254712345700";
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
    
    assert!(swap_response.to_amount > 0, "Should receive USDC");
    assert!(swap_response.spread_amount > 0, "Should have spread");
    
    // Verify balances
    let btc_balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    let usdc_balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkUSDC");
    
    assert_eq!(btc_balance, 0, "BTC balance should be 0");
    assert_eq!(usdc_balance, swap_response.to_amount, "USDC balance should match");
}

#[test]
fn test_swap_same_crypto_error() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let phone = "+254712345701";
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
        .expect("Failed to buy BTC");
    
    // Try to swap BTC to BTC
    let swap_request = SwapCryptoRequest {
        user_identifier: user_id.clone(),
        from_crypto: "CkBTC".to_string(),
        to_crypto: "CkBTC".to_string(),
        amount: 1000,
        pin: pin.to_string(),
    };
    
    let args = encode_args((swap_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "swap_crypto", args)
        .expect("Failed to call swap_crypto");
    
    let result: Result<SwapCryptoResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail when swapping same crypto");
    assert!(result.unwrap_err().contains("same"), "Error should mention same crypto");
}
