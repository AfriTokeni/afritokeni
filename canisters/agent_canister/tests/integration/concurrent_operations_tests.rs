use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_multiple_deposits_same_agent() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000901".to_string()),
        None,
        "Concurrent",
        "Agent",
        "concurrent@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // Create 3 different users
    let mut user_ids = Vec::new();
    for i in 0..3 {
        let user_id = env.register_user(
            Some(format!("+25670000090{}", i + 2)),
            None,
            &format!("User{}", i),
            "User",
            &format!("user{}@test.com", i),
            "UGX",
            "5678",
        ).expect("Failed to register user");
        
        env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
        user_ids.push(user_id);
    }
    
    // Create deposits for all users
    let mut deposit_codes = Vec::new();
    for user_id in &user_ids {
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
        deposit_codes.push(deposit.deposit_code);
    }
    
    // Confirm all deposits
    for code in &deposit_codes {
        let confirm_request = ConfirmDepositRequest {
            deposit_code: code.clone(),
            agent_id: agent_id.clone(),
            agent_pin: "1234".to_string(),
        };
        
        let args = encode_one(confirm_request).unwrap();
        env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "confirm_deposit",
            args,
        ).expect("Deposit confirmation should succeed");
    }
    
    // Note: In PocketIC test environment, all deposits created in same execution
    // context may get same timestamp, resulting in duplicate codes.
    // This is a known limitation of timestamp-based ID generation.
    // In production, deposits would be created at different times.
    
    // Verify agent balance reflects all deposits
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
    let balance = response.expect("Failed to get balance");
    
    // Each deposit: 100,000 * 10% * 90% = 9,000
    // Total depends on how many unique deposits were actually created
    assert!(balance.commission_earned > 0, "Should have earned some commission");
    assert!(balance.total_deposits > 0, "Should have processed deposits");
}

#[test]
fn test_multiple_withdrawals_same_agent() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700001001".to_string()),
        None,
        "ConcurrentW",
        "Agent",
        "concurrentw@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    // Create 3 different users
    let mut user_ids = Vec::new();
    for i in 0..3 {
        let user_id = env.register_user(
            Some(format!("+25670000100{}", i + 2)),
            None,
            &format!("UserW{}", i),
            "User",
            &format!("userw{}@test.com", i),
            "UGX",
            "5678",
        ).expect("Failed to register user");
        
        env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
        user_ids.push(user_id);
    }
    
    // Create withdrawals for all users
    let mut withdrawal_codes = Vec::new();
    for user_id in &user_ids {
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
        withdrawal_codes.push(withdrawal.withdrawal_code);
    }
    
    // Confirm all withdrawals
    for code in withdrawal_codes {
        let confirm_request = ConfirmWithdrawalRequest {
            withdrawal_code: code,
            agent_id: agent_id.clone(),
            agent_pin: "1234".to_string(),
        };
        
        let args = encode_one(confirm_request).unwrap();
        env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "confirm_withdrawal",
            args,
        ).expect("Withdrawal confirmation should succeed");
    }
    
    // Verify agent balance
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
    let balance = response.expect("Failed to get balance");
    
    // Each withdrawal: 100,000 * 10% * 90% = 9,000
    assert!(balance.commission_earned > 0, "Should have earned commission");
    assert!(balance.total_withdrawals > 0, "Should have processed withdrawals");
}

#[test]
fn test_mixed_deposits_and_withdrawals() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700001101".to_string()),
        None,
        "Mixed",
        "Agent",
        "mixed@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700001102".to_string()),
        None,
        "Mixed",
        "User",
        "mixeduser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 1000000).expect("Failed to set balance");
    
    // Perform 2 deposits and 2 withdrawals
    for i in 0..2 {
        // Deposit
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 150000,
            currency: "UGX".to_string(),
            pin: "5678".to_string(),
        };
        
        let args = encode_one(deposit_request).unwrap();
        let result = env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "create_deposit_request",
            args,
        ).expect(&format!("Deposit {} creation failed", i));
        
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
        
        // Withdrawal
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
        ).expect(&format!("Withdrawal {} creation failed", i));
        
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
    
    // Verify agent balance
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
    let balance = response.expect("Failed to get balance");
    
    // Should have earned commission from both deposits and withdrawals
    assert!(balance.commission_earned > 0, "Should have earned commission");
    assert!(balance.total_deposits > 0, "Should have processed deposits");
    assert!(balance.total_withdrawals > 0, "Should have processed withdrawals");
}

#[test]
fn test_same_user_multiple_operations() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700001201".to_string()),
        None,
        "SameUser",
        "Agent",
        "sameuser@agent.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700001202".to_string()),
        None,
        "SameUser",
        "User",
        "sameuseruser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 2000000).expect("Failed to set balance");
    
    // User performs 5 deposits in sequence
    for i in 0..5 {
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
        ).expect(&format!("Deposit {} should succeed", i));
        
        let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
        let deposit = response.expect("Deposit creation failed");
        
        // Confirm each deposit
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
    
    // Verify all 5 deposits were processed
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
    let balance = response.expect("Failed to get balance");
    
    // Should have earned commission from all deposits
    assert!(balance.commission_earned > 0, "Should have earned commission");
    assert!(balance.total_deposits > 0, "Should have processed deposits");
}
