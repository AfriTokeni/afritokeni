use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_deposit_code_format_validation() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000801".to_string()),
        None,
        "Code",
        "Agent",
        "code@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000802".to_string()),
        None,
        "Code",
        "User",
        "code@user.com",
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
    ).expect("Deposit creation should succeed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit = response.expect("Deposit creation failed");
    
    // Verify code format: DEP-{prefix}-{id}-{timestamp}
    assert!(deposit.deposit_code.starts_with("DEP-"));
    let parts: Vec<&str> = deposit.deposit_code.split('-').collect();
    assert_eq!(parts.len(), 4, "Deposit code should have 4 parts");
}

#[test]
fn test_withdrawal_code_format_validation() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000803".to_string()),
        None,
        "CodeW",
        "Agent",
        "codew@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000804".to_string()),
        None,
        "CodeW",
        "User",
        "codew@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Create withdrawal
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
    
    // Verify code format: WTH-{prefix}-{id}-{timestamp}
    assert!(withdrawal.withdrawal_code.starts_with("WTH-"));
    let parts: Vec<&str> = withdrawal.withdrawal_code.split('-').collect();
    assert_eq!(parts.len(), 4, "Withdrawal code should have 4 parts");
}

#[test]
fn test_invalid_deposit_code_rejection() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000805".to_string()),
        None,
        "Invalid",
        "Agent",
        "invalid@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // Try to confirm with invalid code format
    let confirm_request = ConfirmDepositRequest {
        deposit_code: "INVALID-CODE".to_string(),
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    );
    
    // Should fail with invalid code format
    assert!(result.is_err() || {
        let response: Result<ConfirmDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}

#[test]
fn test_nonexistent_deposit_code() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000806".to_string()),
        None,
        "Nonexist",
        "Agent",
        "nonexist@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // Try to confirm with valid format but nonexistent code
    let confirm_request = ConfirmDepositRequest {
        deposit_code: "DEP-agent-999999-1234567890".to_string(),
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    );
    
    // Should fail - code doesn't exist
    assert!(result.is_err() || {
        let response: Result<ConfirmDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        response.is_err()
    });
}
