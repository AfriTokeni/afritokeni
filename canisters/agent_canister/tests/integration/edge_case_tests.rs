use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_deposit_with_zero_amount() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000401".to_string()),
        None,
        "Zero",
        "Agent",
        "zero@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000402".to_string()),
        None,
        "Zero",
        "User",
        "zerouser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 0, // Zero amount
        currency: "UGX".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    );
    
    // Should fail
    assert!(result.is_err() || {
        let response: Result<CreateDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_withdrawal_with_zero_amount() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000403".to_string()),
        None,
        "ZeroW",
        "Agent",
        "zerow@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000404".to_string()),
        None,
        "ZeroW",
        "User",
        "zerowuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 100000).expect("Failed to set balance");
    
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 0, // Zero amount
        currency: "UGX".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(withdrawal_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_withdrawal_request",
        args,
    );
    
    // Should fail
    assert!(result.is_err() || {
        let response: Result<CreateWithdrawalResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_deposit_and_withdrawal_same_user() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000405".to_string()),
        None,
        "Both",
        "Agent",
        "both@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000406".to_string()),
        None,
        "Both",
        "User",
        "bothuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    // Initial balance
    env.set_fiat_balance(&user_id, "UGX", 200000).expect("Failed to set balance");
    
    // Deposit
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("Deposit failed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit = response.expect("Deposit creation failed");
    
    // Confirm deposit
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code,
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    ).expect("Deposit confirmation failed");
    
    // Check balance after deposit
    let balance_after_deposit = env.get_fiat_balance(&user_id, "UGX").expect("Failed to get balance");
    assert_eq!(balance_after_deposit, 290000); // 200000 + 90000 (100000 - 10% fee)
    
    // Now withdrawal
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(withdrawal_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_withdrawal_request",
        args,
    ).expect("Withdrawal failed");
    
    let response: Result<CreateWithdrawalResponse, String> = decode_one(&result).unwrap();
    let withdrawal = response.expect("Withdrawal creation failed");
    
    // Confirm withdrawal
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: withdrawal.withdrawal_code,
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_withdrawal",
        args,
    ).expect("Withdrawal confirmation failed");
    
    // Check final balance
    let final_balance = env.get_fiat_balance(&user_id, "UGX").expect("Failed to get balance");
    // 290000 - 100000 - 10500 (fees) = 179500
    assert_eq!(final_balance, 179500);
}

#[test]
fn test_invalid_currency() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000407".to_string()),
        None,
        "Currency",
        "Agent",
        "currency@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000408".to_string()),
        None,
        "Currency",
        "User",
        "currencyuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "INVALID".to_string(), // Invalid currency
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    );
    
    // Should fail
    assert!(result.is_err() || {
        let response: Result<CreateDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_double_confirmation_attempt() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000409".to_string()),
        None,
        "Double",
        "Agent",
        "double@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000410".to_string()),
        None,
        "Double",
        "User",
        "doubleuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Create deposit
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("Deposit failed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit = response.expect("Deposit creation failed");
    
    // First confirmation
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code.clone(),
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_request.clone()).unwrap();
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    ).expect("First confirmation should succeed");
    
    // Second confirmation attempt (should fail)
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    );
    
    // Should fail (already confirmed)
    assert!(result.is_err() || {
        let response: Result<ConfirmDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}
