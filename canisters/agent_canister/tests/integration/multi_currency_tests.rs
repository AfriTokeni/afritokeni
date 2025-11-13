use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_deposit_kes_currency() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+254700000001".to_string()),
        None,
        "KES",
        "Agent",
        "agent@kes.com",
        "KES",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+254700000002".to_string()),
        None,
        "KES",
        "User",
        "user@kes.com",
        "KES",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "KES", 500000).expect("Failed to set balance");
    
    // KES limits: min 10000, max 1000000
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 50000, // Within KES limits
        currency: "KES".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("Deposit should succeed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    assert!(response.is_ok());
    let deposit = response.unwrap();
    assert_eq!(deposit.currency, "KES");
    assert_eq!(deposit.amount, 50000);
}

#[test]
fn test_withdrawal_tzs_currency() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+255700000001".to_string()),
        None,
        "TZS",
        "Agent",
        "agent@tzs.com",
        "TZS",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+255700000002".to_string()),
        None,
        "TZS",
        "User",
        "user@tzs.com",
        "TZS",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "TZS", 3000000).expect("Failed to set balance");
    
    // TZS limits: min 50000, max 5000000
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000, // Within TZS limits
        currency: "TZS".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(withdrawal_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_withdrawal_request",
        args,
    ).expect("Withdrawal should succeed");
    
    let response: Result<CreateWithdrawalResponse, String> = decode_one(&result).unwrap();
    assert!(response.is_ok());
    let withdrawal = response.unwrap();
    assert_eq!(withdrawal.currency, "TZS");
    assert_eq!(withdrawal.amount, 100000);
}

#[test]
fn test_deposit_ngn_above_maximum() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+234700000001".to_string()),
        None,
        "NGN",
        "Agent",
        "agent@ngn.com",
        "NGN",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+234700000002".to_string()),
        None,
        "NGN",
        "User",
        "user@ngn.com",
        "NGN",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "NGN", 5000000).expect("Failed to set balance");
    
    // NGN limits: min 20000, max 2000000
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 3000000, // Above NGN max
        currency: "NGN".to_string(),
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
fn test_withdrawal_zar_below_minimum() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+27700000001".to_string()),
        None,
        "ZAR",
        "Agent",
        "agent@zar.com",
        "ZAR",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+27700000002".to_string()),
        None,
        "ZAR",
        "User",
        "user@zar.com",
        "ZAR",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "ZAR", 100000).expect("Failed to set balance");
    
    // ZAR limits: min 5000, max 500000
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 3000, // Below ZAR min
        currency: "ZAR".to_string(),
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
fn test_multi_currency_agent_balance() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000501".to_string()),
        None,
        "Multi",
        "Agent",
        "multi@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // User 1 - UGX
    let user1_id = env.register_user(
        Some("+256700000502".to_string()),
        None,
        "User1",
        "UGX",
        "user1@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user1");
    
    env.set_fiat_balance(&user1_id, "UGX", 500000).expect("Failed to set balance");
    
    // User 2 - KES
    let user2_id = env.register_user(
        Some("+254700000503".to_string()),
        None,
        "User2",
        "KES",
        "user2@test.com",
        "KES",
        "5678",
    ).expect("Failed to register user2");
    
    env.set_fiat_balance(&user2_id, "KES", 500000).expect("Failed to set balance");
    
    // Deposit in UGX
    let deposit_ugx = CreateDepositRequest {
        user_id: user1_id.clone(),
        agent_id: agent_id.clone(),
        amount: 200000,
        currency: "UGX".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_ugx).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("UGX deposit failed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit = response.expect("UGX deposit creation failed");
    
    // Confirm UGX deposit
    let confirm_ugx = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code,
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_ugx).unwrap();
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    ).expect("UGX deposit confirmation failed");
    
    // Deposit in KES
    let deposit_kes = CreateDepositRequest {
        user_id: user2_id.clone(),
        agent_id: agent_id.clone(),
        amount: 50000,
        currency: "KES".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_kes).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("KES deposit failed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit = response.expect("KES deposit creation failed");
    
    // Confirm KES deposit
    let confirm_kes = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code,
        agent_id: agent_id.clone(),
        agent_pin: "1234".to_string(),
    };
    
    let args = encode_one(confirm_kes).unwrap();
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    ).expect("KES deposit confirmation failed");
    
    // Check UGX balance
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentBalanceResponse {
        agent_id: String,
        currency: String,
        total_deposits: u64,
        total_withdrawals: u64,
        commission_earned: u64,
        commission_paid: u64,
        commission_pending: u64,
    }
    
    let args = encode_args((agent_id.clone(), "UGX".to_string())).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        args,
    ).expect("get_agent_balance UGX failed");
    
    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let ugx_balance = response.expect("Failed to get UGX balance");
    
    // UGX: 200,000 * 10% * 90% = 18,000
    assert_eq!(ugx_balance.commission_earned, 18000);
    
    // Check KES balance
    let args = encode_args((agent_id.clone(), "KES".to_string())).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        args,
    ).expect("get_agent_balance KES failed");
    
    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let kes_balance = response.expect("Failed to get KES balance");
    
    // KES: 50,000 * 10% * 90% = 4,500
    assert_eq!(kes_balance.commission_earned, 4500);
}

#[test]
fn test_deposit_with_default_currency_limits() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+233700000001".to_string()),
        None,
        "GHS",
        "Agent",
        "agent@ghs.com",
        "GHS",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+233700000002".to_string()),
        None,
        "GHS",
        "User",
        "user@ghs.com",
        "GHS",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "GHS", 500000).expect("Failed to set balance");
    
    // GHS has specific limits: min 3000, max 300000
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 150000, // Within GHS limits
        currency: "GHS".to_string(),
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("GHS deposit should succeed");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    assert!(response.is_ok());
}

#[test]
fn test_currency_conversion_not_mixed() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000601".to_string()),
        None,
        "Mixed",
        "Agent",
        "mixed@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000602".to_string()),
        None,
        "Mixed",
        "User",
        "mixed@user.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    // Set balance in UGX
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set UGX balance");
    
    // Try to deposit in KES (user has no KES balance)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 50000,
        currency: "KES".to_string(), // Different currency
        pin: "5678".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("Should create deposit request");
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    // Should succeed - currency validation happens, balance check is separate
    assert!(response.is_ok());
}
