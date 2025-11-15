use super::*;
use candid::encode_one;
use candid::{CandidType, encode_args, decode_one};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct CreateEscrowRequest {
    user_identifier: String,
    agent_id: String,
    amount: u64,
    crypto_type: String,
    pin: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct CreateEscrowResponse {
    code: String,
    amount: u64,
    crypto_type: String,
    expires_at: u64,
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
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct CleanupResult {
    escrows_processed: u32,
    escrows_refunded: u32,
    total_refunded_btc: u64,
    total_refunded_usdc: u64,
}

#[test]
fn test_cleanup_expired_escrows_success() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let user_phone = "+254712345750";
    let agent_phone = "+254712345751";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent_id = register_test_user(&pic, user_canister, agent_phone, pin);
    
    // Buy crypto for user
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
    
    // Create escrow
    let escrow_amount = buy_response.crypto_amount / 2;
    let request = CreateEscrowRequest {
        user_identifier: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: escrow_amount,
        crypto_type: "CkBTC".to_string(),
        pin: "1234".to_string(),
    };
    
    let args = encode_one(request).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "create_escrow",
        args,
    ).expect("Failed to call create_escrow");
    
    let result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
    // Response unused - test validates automatic cleanup via timer, not manual operations
    let _escrow_response = result.expect("Create escrow should succeed");

    // Verify user balance reduced
    let balance_before = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance_before, buy_response.crypto_amount - escrow_amount);
    
    // Advance time past expiration (24 hours + 1 second)
    // This will trigger the periodic timer which automatically cleans up expired escrows
    pic.advance_time(std::time::Duration::from_secs(86401));
    
    // Give the timer a moment to execute
    pic.tick();
    
    // Verify user balance restored by automatic cleanup
    let balance_after = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance_after, buy_response.crypto_amount, "Balance should be restored by automatic cleanup");
}

#[test]
fn test_cleanup_no_expired_escrows() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let user_phone = "+254712345752";
    let agent_phone = "+254712345753";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent_id = register_test_user(&pic, user_canister, agent_phone, pin);
    
    // Buy crypto for user
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
    
    // Create escrow
    let escrow_amount = buy_response.crypto_amount / 2;
    let request = CreateEscrowRequest {
        user_identifier: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: escrow_amount,
        crypto_type: "CkBTC".to_string(),
        pin: "1234".to_string(),
    };
    
    let args = encode_one(request).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "create_escrow",
        args,
    ).expect("Failed to call create_escrow");
    
    let result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
    let _escrow_response = result.expect("Create escrow should succeed");
    
    // Run cleanup WITHOUT advancing time (escrow not expired)
    let args = encode_args(()).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "cleanup_expired_escrows",
        args,
    ).expect("Failed to call cleanup_expired_escrows");
    
    let result: Result<CleanupResult, String> = decode_one(&response).unwrap();
    let cleanup_result = result.expect("Cleanup should succeed");
    
    // Verify no refunds
    assert_eq!(cleanup_result.escrows_processed, 1, "Should process 1 escrow");
    assert_eq!(cleanup_result.escrows_refunded, 0, "Should not refund any escrows");
    assert_eq!(cleanup_result.total_refunded_btc, 0, "Should not refund BTC");
    
    // Verify user balance unchanged
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance, buy_response.crypto_amount - escrow_amount, "Balance should remain reduced");
}

#[test]
fn test_cleanup_multiple_expired_escrows() {
    let (pic, _data, user_canister, _wallet_canister, crypto_canister, _ckbtc_ledger, _ckusdc_ledger) = setup_test_environment();
    
    let user_phone = "+254712345754";
    let agent_phone = "+254712345755";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent_id = register_test_user(&pic, user_canister, agent_phone, pin);
    
    // Buy crypto for user
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);
    // Buy crypto (smaller amount to avoid velocity limits)
    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 500_000, // Smaller amount
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");
    
    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");
    
    // Create 2 escrows (very small amounts to avoid rate limits)
    let escrow_amount = buy_response.crypto_amount / 15; // Very small escrows
    
    for i in 0..2 {
        let request = CreateEscrowRequest {
            user_identifier: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: escrow_amount,
            crypto_type: "CkBTC".to_string(),
            pin: "1234".to_string(),
        };
        
        let args = encode_one(request).unwrap();
        let response = pic.update_call(
            crypto_canister,
            Principal::anonymous(),
            "create_escrow",
            args,
        ).expect(&format!("Failed to create escrow {}", i));
        
        let result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
        result.expect(&format!("Create escrow {} should succeed", i));
    }
    
    // Verify user balance reduced by 2x escrow amount
    let balance_before = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance_before, buy_response.crypto_amount - (escrow_amount * 2));
    
    // Advance time past expiration - this triggers automatic cleanup via timer
    pic.advance_time(std::time::Duration::from_secs(86401));
    
    // Give the timer a moment to execute
    pic.tick();
    
    // Verify user balance fully restored by automatic cleanup
    let balance_after = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance_after, buy_response.crypto_amount, "All escrow amounts should be refunded by automatic cleanup");
}
