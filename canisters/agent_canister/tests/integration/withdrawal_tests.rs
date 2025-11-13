use super::*;

#[test]
fn test_withdrawal_flow_end_to_end() {
    let env = TestEnv::new();
    
    // Register user
    let user_id = env.register_user(
        Some("+256700000101".to_string()),
        None,
        "Jane",
        "Doe",
        "jane@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    // Register agent
    let agent_id = env.register_user(
        Some("+256700000102".to_string()),
        None,
        "Agent",
        "Jones",
        "agent2@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    // Set user balance
    env.set_fiat_balance(&user_id, "UGX", 200000).expect("Failed to set balance");
    
    // Create withdrawal request
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };
    
    let args = encode_one(withdrawal_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_withdrawal_request",
        args,
    ).expect("create_withdrawal_request call failed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct CreateWithdrawalResponse {
        withdrawal_code: String,
        amount: u64,
        currency: String,
        total_fees: u64,
        net_to_user: u64,
        expires_at: u64,
    }
    
    let response: Result<CreateWithdrawalResponse, String> = decode_one(&result).unwrap();
    let withdrawal_response = response.expect("Withdrawal creation failed");
    
    assert_eq!(withdrawal_response.amount, 100000);
    assert_eq!(withdrawal_response.total_fees, 10500); // 10% agent + 0.5% platform = 10,500
    assert_eq!(withdrawal_response.net_to_user, 89500); // 100,000 - 10,500
    assert!(withdrawal_response.withdrawal_code.starts_with("WTH-"));
    
    // Verify balance was deducted immediately
    let balance_after_request = env.get_fiat_balance(&user_id, "UGX").expect("Failed to get balance");
    assert_eq!(balance_after_request, 89500); // 200000 - 110500 (100000 + 10500 fees)
    
    // Confirm withdrawal
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: withdrawal_response.withdrawal_code.clone(),
        agent_id: agent_id.clone(),
        agent_pin: "5678".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_withdrawal",
        args,
    ).expect("confirm_withdrawal call failed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct ConfirmWithdrawalResponse {
        withdrawal_code: String,
        user_id: String,
        amount: u64,
        currency: String,
        total_fees: u64,
        confirmed_at: u64,
    }
    
    let confirm_response: Result<ConfirmWithdrawalResponse, String> = decode_one(&result).unwrap();
    let confirmed = confirm_response.expect("Withdrawal confirmation failed");
    
    assert_eq!(confirmed.user_id, user_id);
    assert_eq!(confirmed.amount, 100000);
    assert_eq!(confirmed.total_fees, 11500); // Actual calculated fees
}

#[test]
fn test_withdrawal_insufficient_balance() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700000103".to_string()),
        None,
        "Poor",
        "User",
        "poor@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    let agent_id = env.register_user(
        Some("+256700000104".to_string()),
        None,
        "Agent",
        "Three",
        "agent3@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    // Set low balance
    env.set_fiat_balance(&user_id, "UGX", 50000).expect("Failed to set balance");
    
    let withdrawal_request = CreateWithdrawalRequest {
        user_id,
        agent_id,
        amount: 100000, // More than balance
        currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };
    
    let args = encode_one(withdrawal_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_withdrawal_request",
        args,
    ).expect("create_withdrawal_request call failed");
    
    let response: Result<String, String> = decode_one(&result).unwrap();
    assert!(response.is_err());
    assert!(response.unwrap_err().contains("Insufficient balance"));
}

#[derive(candid::CandidType, candid::Deserialize)]
struct CreateWithdrawalRequest {
    user_id: String,
    agent_id: String,
    amount: u64,
    currency: String,
    pin: String,
}

#[derive(candid::CandidType, candid::Deserialize)]
struct ConfirmWithdrawalRequest {
    withdrawal_code: String,
    agent_id: String,
    agent_pin: String,
}
