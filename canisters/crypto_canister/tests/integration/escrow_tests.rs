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
struct VerifyEscrowRequest {
    code: String,
    agent_id: String,
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
fn test_create_escrow_success() {
    let (pic, _data, user_canister, wallet_canister, crypto_canister) = setup_test_environment();
    
    // Register user and agent
    let user_phone = "+254712345710";
    let agent_phone = "+254712345711";
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
    
    // Create escrow for half of the crypto bought
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
    let escrow_response = result.expect("Create escrow should succeed");
    
    assert!(!escrow_response.code.is_empty(), "Should return escrow code");
    assert!(escrow_response.code.starts_with("ESC-"), "Code should start with ESC-");
    assert_eq!(escrow_response.amount, buy_response.crypto_amount / 2);
    assert!(escrow_response.expires_at > 0, "Should have expiration time");
    
    // Verify user balance reduced
    let balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(balance, buy_response.crypto_amount / 2, "User balance should be reduced");
}

#[test]
fn test_verify_escrow_success() {
    let (pic, _data, user_canister, wallet_canister, crypto_canister) = setup_test_environment();
    
    let user_phone = "+254712345712";
    let agent_phone = "+254712345713";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent_id = register_test_user(&pic, user_canister, agent_phone, pin);
    
    // Buy crypto and create escrow
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
    
    let escrow_request = CreateEscrowRequest {
        user_identifier: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: buy_response.crypto_amount,
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((escrow_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "create_escrow", args)
        .expect("Failed to create escrow");
    
    let create_result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
    let escrow_response = create_result.expect("Create escrow should succeed");
    
    // Agent verifies and claims escrow
    let verify_request = VerifyEscrowRequest {
        code: escrow_response.code.clone(),
        agent_id: agent_id.clone(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((verify_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "verify_escrow", args)
        .expect("Failed to call verify_escrow");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    let tx_id = result.expect("Verify escrow should succeed");
    
    assert!(!tx_id.is_empty(), "Should return transaction ID");
    
    // Verify agent received crypto
    let agent_balance = get_crypto_balance(&pic, crypto_canister, &agent_id, "CkBTC");
    assert_eq!(agent_balance, buy_response.crypto_amount, "Agent should receive crypto");
    
    // Verify user balance is still 0
    let user_balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(user_balance, 0, "User balance should be 0");
}

#[test]
fn test_cancel_escrow_success() {
    let (pic, _data, user_canister, wallet_canister, crypto_canister) = setup_test_environment();
    
    let user_phone = "+254712345714";
    let agent_phone = "+254712345715";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent_id = register_test_user(&pic, user_canister, agent_phone, pin);
    
    // Buy crypto and create escrow
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
    
    let escrow_request = CreateEscrowRequest {
        user_identifier: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: buy_response.crypto_amount,
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((escrow_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "create_escrow", args)
        .expect("Failed to create escrow");
    
    let create_result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
    let escrow_response = create_result.expect("Create escrow should succeed");
    
    // User cancels escrow
    let args = encode_args((escrow_response.code.clone(), user_id.clone(), pin.to_string())).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "cancel_escrow", args)
        .expect("Failed to call cancel_escrow");
    
    let result: Result<(), String> = decode_one(&response).unwrap();
    result.expect("Cancel escrow should succeed");
    
    // Verify user got crypto back
    let user_balance = get_crypto_balance(&pic, crypto_canister, &user_id, "CkBTC");
    assert_eq!(user_balance, buy_response.crypto_amount, "User should get crypto back");
}

#[test]
fn test_escrow_wrong_agent() {
    let (pic, _data, user_canister, wallet_canister, crypto_canister) = setup_test_environment();
    
    let user_phone = "+254712345716";
    let agent1_phone = "+254712345717";
    let agent2_phone = "+254712345718";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent1_id = register_test_user(&pic, user_canister, agent1_phone, pin);
    let agent2_id = register_test_user(&pic, user_canister, agent2_phone, pin);
    
    // Buy crypto and create escrow for agent1
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
    
    let escrow_request = CreateEscrowRequest {
        user_identifier: user_id.clone(),
        agent_id: agent1_id.clone(),
        amount: buy_response.crypto_amount,
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((escrow_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "create_escrow", args)
        .expect("Failed to create escrow");
    
    let create_result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
    let escrow_response = create_result.expect("Create escrow should succeed");
    
    // Agent2 tries to claim (should fail)
    let verify_request = VerifyEscrowRequest {
        code: escrow_response.code.clone(),
        agent_id: agent2_id.clone(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((verify_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "verify_escrow", args)
        .expect("Failed to call verify_escrow");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with wrong agent");
    assert!(result.unwrap_err().contains("not authorized"), "Error should mention authorization");
}

#[test]
fn test_escrow_insufficient_balance() {
    let (pic, _data, user_canister, wallet_canister, crypto_canister) = setup_test_environment();
    
    let user_phone = "+254712345719";
    let agent_phone = "+254712345720";
    let pin = "1234";
    let user_id = register_test_user(&pic, user_canister, user_phone, pin);
    let agent_id = register_test_user(&pic, user_canister, agent_phone, pin);
    
    // Buy small amount of crypto
    set_fiat_balance(&pic, _data, &user_id, "KES", 10_000_000);
    
    let buy_request = BuyCryptoRequest {
        user_identifier: user_id.clone(),
        fiat_amount: 150_000, // Small amount
        currency: "KES".to_string(),
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((buy_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "buy_crypto", args)
        .expect("Failed to buy crypto");
    
    let buy_result: Result<BuyCryptoResponse, String> = decode_one(&response).unwrap();
    let buy_response = buy_result.expect("Buy should succeed");
    
    // Try to create escrow for more than balance
    let escrow_request = CreateEscrowRequest {
        user_identifier: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: buy_response.crypto_amount * 2, // More than balance
        crypto_type: "CkBTC".to_string(),
        pin: pin.to_string(),
    };
    
    let args = encode_args((escrow_request,)).unwrap();
    let response = pic.update_call(crypto_canister, Principal::anonymous(), "create_escrow", args)
        .expect("Failed to call create_escrow");
    
    let result: Result<CreateEscrowResponse, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail with insufficient balance");
    assert!(result.unwrap_err().contains("Insufficient"), "Error should mention insufficient balance");
}
