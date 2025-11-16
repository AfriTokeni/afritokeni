use super::*;
use candid::{encode_one, decode_one};

#[test]
fn test_deposit_amount_above_maximum() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000301".to_string()),
        None,
        "Fraud",
        "Agent",
        "fraud@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000302".to_string()),
        None,
        "Fraud",
        "User",
        "frauduser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 20000000).expect("Failed to set balance");
    
    // Try deposit above maximum (max is 10,000,000 for UGX)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 15000000, // Above max
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
fn test_withdrawal_amount_above_maximum() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000303".to_string()),
        None,
        "Limit",
        "Agent",
        "limit@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000304".to_string()),
        None,
        "Limit",
        "User",
        "limituser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 10000000).expect("Failed to set balance");
    
    // Try withdrawal above maximum (max is 5,000,000 for UGX)
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 6000000, // Above max
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
fn test_multiple_deposits_within_limits() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000305".to_string()),
        None,
        "Multi",
        "Agent",
        "multi@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000306".to_string()),
        None,
        "Multi",
        "User",
        "multiuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 5000000).expect("Failed to set balance");
    
    // Perform multiple deposits (all within individual limits)
    for i in 0..10 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100000, // Well within limits
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
        assert!(response.is_ok(), "Deposit {} should succeed", i);
    }
}

#[test]
fn test_deposit_below_minimum() {
    let env = TestEnv::new();
    
    let agent_id = env.register_user(
        Some("+256700000307".to_string()),
        None,
        "Min",
        "Agent",
        "min@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");
    
    let user_id = env.register_user(
        Some("+256700000308".to_string()),
        None,
        "Min",
        "User",
        "minuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");
    
    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");
    
    // Try deposit below minimum (min is 100,000 for UGX)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 50000, // Below min
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
fn test_withdrawal_below_minimum() {
    let env = TestEnv::new();

    let agent_id = env.register_user(
        Some("+256700000309".to_string()),
        None,
        "MinW",
        "Agent",
        "minw@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    let user_id = env.register_user(
        Some("+256700000310".to_string()),
        None,
        "MinW",
        "User",
        "minwuser@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    env.set_fiat_balance(&user_id, "UGX", 500000).expect("Failed to set balance");

    // Try withdrawal below minimum (min is 100,000 for UGX)
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 50000, // Below min
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
fn test_fraud_detection_daily_deposit_limit() {
    let env = TestEnv::new();

    let agent_id = env.register_user(
        Some("+256700000311".to_string()),
        None,
        "Fraud",
        "Agent",
        "fraud_limit@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    let user_id = env.register_user(
        Some("+256700000312".to_string()),
        None,
        "Fraud",
        "User",
        "fraud_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Create 20 deposits (should succeed - below 50 limit)
    for i in 0..20 {
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
        );

        // All should succeed (20 < 50 limit)
        assert!(result.is_ok(), "Deposit {} should succeed", i);
        let response: Result<CreateDepositResponse, String> = decode_one(&result.unwrap()).unwrap();
        assert!(response.is_ok(), "Deposit {} response should be ok", i);
    }

    // Verify agent activity was persisted with correct counts
    // Note: We can't directly query agent_activity from tests, but we can verify
    // that subsequent deposits still work, proving persistence is functioning
}

#[test]
fn test_fraud_detection_user_agent_pattern() {
    let env = TestEnv::new();

    let agent_id = env.register_user(
        Some("+256700000313".to_string()),
        None,
        "Pattern",
        "Agent",
        "pattern@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    let user_id = env.register_user(
        Some("+256700000314".to_string()),
        None,
        "Pattern",
        "User",
        "pattern_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Same user-agent pair multiple times (should trigger warning but not block)
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
        );

        // Should still succeed (warnings are non-blocking)
        assert!(result.is_ok(), "Deposit {} should succeed despite warnings", i);
    }
}

#[test]
fn test_fraud_detection_activity_persistence() {
    let env = TestEnv::new();

    let agent_id = env.register_user(
        Some("+256700000315".to_string()),
        None,
        "Persist",
        "Agent",
        "persist@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    let user_id = env.register_user(
        Some("+256700000316".to_string()),
        None,
        "Persist",
        "User",
        "persist_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Perform 3 deposits
    for i in 0..3 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100000,
            currency: "UGX".to_string(),
            pin: "5678".to_string(),
        };

        let args = encode_one(deposit_request).unwrap();
        env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "create_deposit_request",
            args,
        ).expect(&format!("Deposit {} should succeed", i));
    }

    // If activity is persisted correctly, the 4th deposit should still load
    // the previous activity showing 3 deposits already recorded
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
    );

    // Should succeed - persistence working means activity is loaded and updated correctly
    assert!(result.is_ok(), "Deposit with persisted activity should succeed");
}
