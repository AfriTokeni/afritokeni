use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_deposit_with_wrong_user_pin() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000701".to_string()),
        None,
        "PIN",
        "Agent",
        "pin@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000702".to_string()),
        None,
        "PIN",
        "User",
        "pin@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "0000".to_string(), // Wrong PIN
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    );
    
    // Should fail with invalid PIN
    assert!(result.is_err() || {
        let response: Result<CreateDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_withdrawal_with_wrong_user_pin() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000703".to_string()),
        None,
        "PINW",
        "Agent",
        "pinw@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000704".to_string()),
        None,
        "PINW",
        "User",
        "pinw@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "9999".to_string(), // Wrong PIN
    };
    
    let args = encode_one(withdrawal_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_withdrawal_request",
        args,
    );
    
    // Should fail with invalid PIN
    assert!(result.is_err() || {
        let response: Result<CreateWithdrawalResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_confirm_deposit_with_wrong_agent_pin() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000705".to_string()),
        None,
        "PINA",
        "Agent",
        "pina@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000706".to_string()),
        None,
        "PINA",
        "User",
        "pina@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Create deposit with correct user PIN
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
    ).expect("Deposit creation should succeed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit = response.expect("Deposit creation failed");
    
    // Try to confirm with wrong agent PIN
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code,
        agent_id: agent_id.clone(),
        agent_pin: "0000".to_string(), // Wrong agent PIN
    };
    
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    );
    
    // Should fail with invalid agent PIN
    assert!(result.is_err() || {
        let response: Result<ConfirmDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_confirm_withdrawal_with_wrong_agent_pin() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000707".to_string()),
        None,
        "PINAW",
        "Agent",
        "pinaw@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000708".to_string()),
        None,
        "PINAW",
        "User",
        "pinaw@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Create withdrawal with correct user PIN
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
    ).expect("Withdrawal creation should succeed");
    
    let response: Result<CreateWithdrawalResponse, String> = decode_one(&result).unwrap();
    let withdrawal = response.expect("Withdrawal creation failed");
    
    // Try to confirm with wrong agent PIN
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: withdrawal.withdrawal_code,
        agent_id: agent_id.clone(),
        agent_pin: "0000".to_string(), // Wrong agent PIN
    };
    
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_withdrawal",
        args,
    );
    
    // Should fail with invalid agent PIN
    assert!(result.is_err() || {
        let response: Result<ConfirmWithdrawalResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_pin_validation_security() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000709".to_string()),
        None,
        "PINS",
        "Agent",
        "pins@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000710".to_string()),
        None,
        "PINS",
        "User",
        "pins@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Test with correct PIN first
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "5678".to_string(), // Correct PIN
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("Should succeed with correct PIN");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    assert!(response.is_ok(), "Correct PIN should work");
    
    // Test with wrong PIN
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "1111".to_string(), // Wrong PIN
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
    }, "Wrong PIN should fail");
}
