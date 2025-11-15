use super::*;

#[test]
fn test_agent_balance_tracking_after_deposit() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700000201".to_string()),
        None,
        "Balance",
        "User",
        "balance@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    let agent_id = env.register_user(
        Some("+256700000202".to_string()),
        None,
        "Balance",
        "Agent",
        "balance_agent@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    // Create and confirm deposit
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("create_deposit_request call failed");
    
    #[derive(candid::CandidType, candid::Deserialize)]
    struct CreateDepositResponse {
        deposit_code: String,
        amount: u64,
        currency: String,
        agent_commission: u64,
        net_to_user: u64,
        expires_at: u64,
    }
    
    let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
    let deposit_response = response.expect("Deposit creation failed");
    
    // Confirm deposit
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit_response.deposit_code,
        agent_id: agent_id.clone(),
        agent_pin: "5678".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    ).expect("confirm_deposit call failed");
    
    // Check agent balance
    let args = encode_args((agent_id.clone(), "UGX".to_string())).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        args,
    ).expect("get_agent_balance call failed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentBalanceResponse {
        agent_id: String,
        currency: String,
        total_deposits: u64,
        total_withdrawals: u64,
        commission_earned: u64,
        commission_paid: u64,
        commission_pending: u64,
        last_settlement_date: Option<u64>,
    }
    
    let balance_response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance = balance_response.expect("Failed to get agent balance");

    // total_deposits is now a COUNT of deposit operations, not sum of amounts
    assert_eq!(balance.total_deposits, 1, "Expected 1 deposit operation");
    assert_eq!(balance.commission_earned, 9000); // 90% of 10000 commission
    assert_eq!(balance.commission_pending, 9000);
}

#[derive(candid::CandidType, candid::Deserialize)]
struct CreateDepositRequest {
    user_id: String,
    agent_id: String,
    amount: u64,
    currency: String,
    pin: String,
}

#[derive(candid::CandidType, candid::Deserialize)]
struct ConfirmDepositRequest {
    deposit_code: String,
    agent_id: String,
    agent_pin: String,
}
