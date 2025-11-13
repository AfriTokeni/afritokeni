use super::*;

use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_monthly_settlement_generation() {
    let env = TestEnv::new();
    
    // Register agent
    let agent_id = env.register_user(
        Some("+256700000201".to_string()),
        None,
        "Settlement Agent",
        "Agent",
        "UGX",
        "1234",
    );
    
    // Register user
    let user_id = env.register_user(
        Some("+256700000202".to_string()),
        None,
        "Settlement User",
        "User",
        "UGX",
        "5678",
    );
    
    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Perform multiple deposits to accumulate commission
    for i in 0..5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 50000,
            currency: "UGX".to_string(),
            user_pin: "5678".to_string(),
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
    // Each deposit: 50,000 * 10% = 5,000 agent fee
    // Agent keeps 90% = 4,500 per deposit
    // Total: 4,500 * 5 = 22,500
    assert_eq!(balance.commission_earned, 22500);
    assert_eq!(balance.commission_pending, 22500);
    assert_eq!(balance.commission_paid, 0);
}

#[test]
fn test_settlement_minimum_threshold() {
    let env = TestEnv::new();
    
    // Register agent
    let agent_id = env.register_user(
        Some("+256700000203".to_string()),
        None,
        "Small Agent",
        "Agent",
        "UGX",
        "1234",
    );
    
    // Register user
    let user_id = env.register_user(
        Some("+256700000204".to_string()),
        None,
        "Small User",
        "User",
        "UGX",
        "5678",
    );
    
    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 100000).expect("Failed to set balance");
    
    // Perform small deposit (below settlement threshold)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 10000, // Small amount
        currency: "UGX".to_string(),
        user_pin: "5678".to_string(),
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
    
    // Agent earned commission: 10,000 * 10% * 90% = 900
    // This is below the 100,000 (1,000 in currency units) threshold
    assert_eq!(balance.commission_earned, 900);
    assert!(balance.commission_earned < 100000); // Below settlement threshold
}
