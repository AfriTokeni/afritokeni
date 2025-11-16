/// Enhanced Fraud Detection Tests for agent_canister
///
/// Tests the suspicious_rapid_transactions_threshold feature
/// Config: agent_config.toml [fraud] suspicious_rapid_transactions_threshold = 5
///
/// These tests will FAIL until the feature is properly implemented and wired up.

use super::*;
use candid::{encode_one, decode_one, encode_args};

// ============================================================================
// Test 1: Rapid Deposits Are Rejected
// ============================================================================

#[test]
fn test_rapid_deposits_rejected() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700222001".to_string()),
        None,
        "RapidTest",
        "Agent",
        "rapid_agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700222002".to_string()),
        None,
        "RapidTest",
        "User",
        "rapid_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Create 5 deposit requests rapidly (at threshold)
    for i in 1..=5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 200_000,
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

        assert!(result.is_ok(), "Deposit {} should succeed (at threshold)", i);
    }

    // 6th deposit should be rejected (exceeds suspicious_rapid_transactions_threshold)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 200_000,
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

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = result {
        let deposit_result: Result<CreateDepositResponse, String> = decode_one(&response).unwrap();
        assert!(
            deposit_result.is_err(),
            "6th deposit should be rejected due to rapid transaction threshold. Feature not implemented!"
        );

        if let Err(error) = deposit_result {
            assert!(
                error.contains("Too many transactions") ||
                error.contains("rapid transaction") ||
                error.contains("Please wait"),
                "Error should mention rapid transactions, got: {}",
                error
            );
        }
    } else {
        panic!("Call should succeed but return an error result");
    }
}

// ============================================================================
// Test 2: Rapid Withdrawals Are Rejected
// ============================================================================

#[test]
fn test_rapid_withdrawals_rejected() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700222003".to_string()),
        None,
        "WithdrawTest",
        "Agent",
        "withdraw_agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700222004".to_string()),
        None,
        "WithdrawTest",
        "User",
        "withdraw_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user large balance for withdrawals
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Create 5 withdrawal requests rapidly (at threshold)
    for i in 1..=5 {
        let withdrawal_request = CreateWithdrawalRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100_000,
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

        assert!(result.is_ok(), "Withdrawal {} should succeed (at threshold)", i);
    }

    // 6th withdrawal should be rejected
    let withdrawal_request = CreateWithdrawalRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100_000,
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

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = result {
        let withdrawal_result: Result<CreateWithdrawalResponse, String> = decode_one(&response).unwrap();
        assert!(
            withdrawal_result.is_err(),
            "6th withdrawal should be rejected due to rapid transaction threshold. Feature not implemented!"
        );

        if let Err(error) = withdrawal_result {
            assert!(
                error.contains("Too many transactions") ||
                error.contains("rapid transaction") ||
                error.contains("Please wait"),
                "Error should mention rapid transactions, got: {}",
                error
            );
        }
    } else {
        panic!("Call should succeed but return an error result");
    }
}

// ============================================================================
// Test 3: Combined Transaction Velocity (Deposits + Withdrawals)
// ============================================================================

#[test]
fn test_combined_transaction_velocity() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700222005".to_string()),
        None,
        "Combined",
        "Agent",
        "combined_agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700222006".to_string()),
        None,
        "Combined",
        "User",
        "combined_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Perform 3 deposits
    for _ in 1..=3 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100_000,
            currency: "UGX".to_string(),
            pin: "5678".to_string(),
        };

        let args = encode_one(deposit_request).unwrap();
        env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "create_deposit_request",
            args,
        ).expect("Deposit should succeed");
    }

    // Perform 2 withdrawals
    for _ in 1..=2 {
        let withdrawal_request = CreateWithdrawalRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100_000,
            currency: "UGX".to_string(),
            pin: "5678".to_string(),
        };

        let args = encode_one(withdrawal_request).unwrap();
        env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "create_withdrawal_request",
            args,
        ).expect("Withdrawal should succeed");
    }

    // Now we have 5 total transactions (3 deposits + 2 withdrawals)
    // The 6th transaction (either type) should be rejected

    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100_000,
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

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = result {
        let deposit_result: Result<CreateDepositResponse, String> = decode_one(&response).unwrap();
        assert!(
            deposit_result.is_err(),
            "6th transaction should be rejected (combined deposits + withdrawals). Feature not implemented!"
        );
    } else {
        panic!("Call should succeed but return an error result");
    }
}

// ============================================================================
// Test 4: Rolling Window Enforcement (Old Transactions Expire)
// ============================================================================

#[test]
fn test_rolling_window_enforcement() {
    // This test verifies that the 1-hour rolling window works correctly
    // Transactions older than 1 hour should not count toward the limit

    // NOTE: This test requires time manipulation in PocketIC
    // or a very long test execution (waiting 1+ hours)
    // For now, this is a placeholder for the rust-ic-expert to implement
    // using PocketIC's time advancement features

    // EXPECTED BEHAVIOR:
    // 1. Create 5 transactions at T=0
    // 2. Advance time by 61 minutes
    // 3. Create 5 more transactions (should succeed - old ones expired)
    // 4. 6th new transaction should be rejected (only counting new window)
}

// ============================================================================
// Test 5: Fraud Rejection Logged to Audit Trail
// ============================================================================

#[test]
fn test_fraud_rejection_logged() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700222007".to_string()),
        None,
        "AuditTest",
        "Agent",
        "audit_agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700222008".to_string()),
        None,
        "AuditTest",
        "User",
        "audit_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Create 6 deposits (6th should be rejected)
    for i in 1..=6 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100_000,
            currency: "UGX".to_string(),
            pin: "5678".to_string(),
        };

        let args = encode_one(deposit_request).unwrap();
        let _ = env.pic.update_call(
            env.agent_canister_id,
            Principal::anonymous(),
            "create_deposit_request",
            args,
        );

        if i == 6 {
            // Query audit log after rejection
            let audit_result = env.pic.query_call(
                env.agent_canister_id,
                Principal::anonymous(),
                "get_failed_operations",
                encode_args((Some(100usize),)).unwrap(),
            );

            // EXPECTED TO FAIL: Audit logging may not be implemented
            assert!(audit_result.is_ok(), "Should be able to query failed operations");

            if let Ok(response) = audit_result {
                let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
                    candid::decode_args(&response)
                        .expect("Failed to decode audit log");

                let fraud_logs: Vec<_> = audit_entries.iter()
                    .filter(|entry| entry.action.contains("fraud")
                                 || entry.action.contains("rapid_transaction")
                                 || entry.details.contains("Too many transactions"))
                    .collect();

                assert!(
                    fraud_logs.len() >= 1,
                    "Expected fraud detection log entry. Feature not implemented!"
                );

                // Verify log contains agent_id
                if let Some(log) = fraud_logs.first() {
                    assert!(
                        log.details.contains(&agent_id) || log.user_id == Some(agent_id.clone()),
                        "Fraud log should reference the agent_id"
                    );
                }
            }
        }
    }
}

// ============================================================================
// Test 6: Normal Velocity Allowed
// ============================================================================

#[test]
fn test_normal_velocity_allowed() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700222009".to_string()),
        None,
        "Normal",
        "Agent",
        "normal_agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700222010".to_string()),
        None,
        "Normal",
        "User",
        "normal_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Failed to set balance");

    // Create exactly 5 deposits (at threshold, should all succeed)
    for i in 1..=5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100_000,
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

        assert!(result.is_ok(), "Deposit {} should succeed", i);

        if let Ok(response) = result {
            let deposit_result: Result<CreateDepositResponse, String> = decode_one(&response).unwrap();
            assert!(
                deposit_result.is_ok(),
                "Deposit {} should succeed (normal velocity at threshold)",
                i
            );
        }
    }
}

// ============================================================================
// Test 7: Per-Agent Tracking (Different Agents Independent)
// ============================================================================

#[test]
fn test_per_agent_tracking() {
    let env = TestEnv::new();

    // Register two agents
    let agent1_id = env.register_user(
        Some("+256700222011".to_string()),
        None,
        "Agent1",
        "TestAgent",
        "agent1@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent1");

    let agent2_id = env.register_user(
        Some("+256700222012".to_string()),
        None,
        "Agent2",
        "TestAgent",
        "agent2@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent2");

    // Register user
    let user_id = env.register_user(
        Some("+256700222013".to_string()),
        None,
        "SharedUser",
        "Test",
        "shared_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 3_000_000).expect("Failed to set balance");

    // Agent 1: Create 5 deposits (at threshold)
    for i in 1..=5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent1_id.clone(),
            amount: 100_000,
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

        assert!(result.is_ok(), "Agent1 deposit {} should succeed", i);
    }

    // Agent 2: Should still be able to do 5 deposits (independent counter)
    for i in 1..=5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent2_id.clone(),
            amount: 100_000,
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

        assert!(
            result.is_ok(),
            "Agent2 deposit {} should succeed (independent tracking)",
            i
        );

        if let Ok(response) = result {
            let deposit_result: Result<CreateDepositResponse, String> = decode_one(&response).unwrap();
            assert!(
                deposit_result.is_ok(),
                "Agent2 should have independent counter from Agent1. Feature may not be implemented!"
            );
        }
    }

    // Agent 1: 6th deposit should be rejected
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent1_id.clone(),
        amount: 100_000,
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

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = result {
        let deposit_result: Result<CreateDepositResponse, String> = decode_one(&response).unwrap();
        assert!(
            deposit_result.is_err(),
            "Agent1's 6th deposit should be rejected. Feature not implemented!"
        );
    } else {
        panic!("Call should succeed but return an error result");
    }
}

// ============================================================================
// Test 8: Confirmed Transactions Count Toward Velocity
// ============================================================================

#[test]
fn test_confirmed_transactions_count() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700222014".to_string()),
        None,
        "ConfirmTest",
        "Agent",
        "confirm_agent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700222015".to_string()),
        None,
        "ConfirmTest",
        "User",
        "confirm_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Create AND confirm 5 deposits
    for i in 1..=5 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 100_000,
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
        let deposit = response.expect("Deposit should be created");

        // Confirm the deposit
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
        ).expect(&format!("Deposit {} confirmation should succeed", i));
    }

    // 6th deposit creation should be rejected (velocity check)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100_000,
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

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = result {
        let deposit_result: Result<CreateDepositResponse, String> = decode_one(&response).unwrap();
        assert!(
            deposit_result.is_err(),
            "6th deposit should be rejected even after confirmations. Feature not implemented!"
        );
    } else {
        panic!("Call should succeed but return an error result");
    }
}
