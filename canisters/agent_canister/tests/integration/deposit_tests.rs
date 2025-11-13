use super::*;

#[test]
fn test_deposit_flow_end_to_end() {
    let env = TestEnv::new();
    
    // Register user
    let user_id = env.register_user(
        Some("+256700000001".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    // Register agent
    let agent_id = env.register_user(
        Some("+256700000002".to_string()),
        None,
        "Agent",
        "Smith",
        "agent@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    // Create deposit request
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
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
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
    
    assert_eq!(deposit_response.amount, 100000);
    assert_eq!(deposit_response.agent_commission, 10000); // 10%
    assert_eq!(deposit_response.net_to_user, 90000);
    assert!(deposit_response.deposit_code.starts_with("DEP-"));
    
    // Confirm deposit
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit_response.deposit_code.clone(),
        agent_id: agent_id.clone(),
        agent_pin: "5678".to_string(),
    };
    
    let args = encode_one(confirm_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "confirm_deposit",
        args,
    ).expect("confirm_deposit call failed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct ConfirmDepositResponse {
        deposit_code: String,
        user_id: String,
        amount: u64,
        currency: String,
        agent_commission: u64,
        confirmed_at: u64,
    }
    
    let confirm_response: Result<ConfirmDepositResponse, String> = decode_one(&result).unwrap();
    let confirmed = confirm_response.expect("Deposit confirmation failed");
    
    assert_eq!(confirmed.user_id, user_id);
    assert_eq!(confirmed.amount, 100000);
    
    // Verify user balance increased by net amount (90000)
    let balance = env.get_fiat_balance(&user_id, "UGX").expect("Failed to get balance");
    assert_eq!(balance, 90000);
}

#[test]
fn test_deposit_amount_below_minimum() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700000003".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    let agent_id = env.register_user(
        Some("+256700000004".to_string()),
        None,
        "Test",
        "Agent",
        "agent@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    let deposit_request = CreateDepositRequest {
        user_id,
        agent_id,
        amount: 50000, // Below minimum of 100000
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
    
    let response: Result<String, String> = decode_one(&result).unwrap();
    assert!(response.is_err());
    assert!(response.unwrap_err().contains("below minimum"));
}

#[test]
fn test_deposit_amount_above_maximum() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700000005".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    let agent_id = env.register_user(
        Some("+256700000006".to_string()),
        None,
        "Test",
        "Agent",
        "agent@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    let deposit_request = CreateDepositRequest {
        user_id,
        agent_id,
        amount: 20000000, // Above maximum of 10000000
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
    
    let response: Result<String, String> = decode_one(&result).unwrap();
    assert!(response.is_err());
    assert!(response.unwrap_err().contains("exceeds maximum"));
}

#[test]
fn test_deposit_invalid_pin() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700000007".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Failed to register user");
    
    let agent_id = env.register_user(
        Some("+256700000008".to_string()),
        None,
        "Test",
        "Agent",
        "agent@example.com",
        "UGX",
        "5678",
    ).expect("Failed to register agent");
    
    let deposit_request = CreateDepositRequest {
        user_id,
        agent_id,
        amount: 100000,
        currency: "UGX".to_string(),
        pin: "9999".to_string(), // Wrong PIN
    };
    
    let args = encode_one(deposit_request).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "create_deposit_request",
        args,
    ).expect("create_deposit_request call failed");
    
    let response: Result<String, String> = decode_one(&result).unwrap();
    assert!(response.is_err());
    assert!(response.unwrap_err().contains("Invalid PIN"));
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
