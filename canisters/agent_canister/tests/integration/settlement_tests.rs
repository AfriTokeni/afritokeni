use super::*;
use candid::{encode_one, decode_one, encode_args};

#[test]
fn test_monthly_settlement_generation() {
    let env = TestEnv::new();
    
    // Register agent
    let agent_id = env.register_user(
        Some("+256700000201".to_string()),
        None,
        "Settlement",
        "Agent",
        "agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // Register user
    let user_id = env.register_user(
        Some("+256700000202".to_string()),
        None,
        "Settlement",
        "User",
        "user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Perform multiple deposits to accumulate commission
    for i in 0..5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 200000,  // Changed from 50000 to be above minimum
            currency: "UGX".to_string(),
            pin: "5678".to_string(),
        };
        
        let args = encode_one(deposit_request).unwrap();
        let result = env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "create_deposit_request",
            args,
        ).expect(&format!("Deposit {} failed", i));
        
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
    }
    
    // Check agent balance has accumulated commission
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
    ).expect("get_agent_balance failed");
    
    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance = response.expect("Failed to get agent balance");
    
    // Agent should have earned commission from 5 deposits
    // Each deposit: 200,000 * 10% = 20,000 agent fee
    // Agent keeps 90% = 18,000 per deposit
    // Total: 18,000 * 5 = 90,000
    assert_eq!(balance.commission_earned, 90000);
    assert_eq!(balance.commission_pending, 90000);
    assert_eq!(balance.commission_paid, 0);
}

#[test]
fn test_settlement_minimum_threshold() {
    let env = TestEnv::new();
    
    // Register agent
    let agent_id = env.register_user(
        Some("+256700000203".to_string()),
        None,
        "Small",
        "Agent",
        "small@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // Register user
    let user_id = env.register_user(
        Some("+256700000204".to_string()),
        None,
        "Small",
        "User",
        "smalluser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 100000).expect("Failed to set balance");
    
    // Perform small deposit (below settlement threshold but above minimum)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000, // Minimum amount
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
    
    // Check agent balance
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
    ).expect("get_agent_balance failed");
    
    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance = response.expect("Failed to get agent balance");
    
    // Agent earned commission: 100,000 * 10% * 90% = 9,000
    // This is below the 100,000 settlement threshold
    assert_eq!(balance.commission_earned, 9000);
    assert!(balance.commission_earned < 100000); // Below settlement threshold
}

#[test]
fn test_agent_balance_after_multiple_withdrawals() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000205".to_string()),
        None,
        "Multi",
        "Agent",
        "multi@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000206".to_string()),
        None,
        "Multi",
        "User",
        "multiuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    // Give user large balance
    env.set_fiat_balance(&user_id, "UGX", 1000000).expect("Failed to set balance");
    
    // Perform 3 withdrawals
    for _ in 0..3 {
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
    }
    
    // Check agent balance
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
    ).expect("get_agent_balance failed");
    
    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance = response.expect("Failed to get agent balance");
    
    // 3 withdrawals
    assert_eq!(balance.total_withdrawals, 3);
    // Commission: 3 * (100,000 * 10% * 90%) = 3 * 9,000 = 27,000
    assert_eq!(balance.commission_earned, 27000);
}
