/// Settlement Automation Tests for agent_canister
///
/// Tests the weekly settlement system from credit.settlement config
/// Config: agent_config.toml [credit.settlement]
/// - settlement_frequency = "weekly"
/// - settlement_day_of_week = 1 (Monday)
/// - min_settlement_balance = 50_000 (500 in currency units)
///
/// These tests will FAIL until the automated settlement feature is implemented.

use super::*;
use candid::{encode_one, decode_one, encode_args};

// ============================================================================
// Test 1: Weekly Settlement Trigger on Configured Day
// ============================================================================

#[test]
fn test_weekly_settlement_trigger() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333001".to_string()),
        None,
        "WeeklyAgent",
        "Test",
        "weekly@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333002".to_string()),
        None,
        "WeeklyUser",
        "Test",
        "weekly_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Create and confirm multiple deposits to accumulate commission
    for i in 1..=10 {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: 200_000, // 2000 UGX
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
    let balance_before = response.expect("Failed to get agent balance");

    // Agent should have earned commission
    // 10 deposits * 200,000 * 10% = 200,000 commission
    // Agent keeps 90% = 180,000
    assert!(
        balance_before.commission_pending > 50_000,
        "Agent should have pending commission above settlement threshold"
    );

    // Trigger weekly settlement (manual trigger for testing)
    // In production, this would run via timer on Monday
    let settlement_result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    );

    // EXPECTED TO FAIL: trigger_weekly_settlement method may not exist
    assert!(
        settlement_result.is_ok(),
        "Weekly settlement trigger should exist. Feature not implemented!"
    );

    if let Ok(response) = settlement_result {
        let result: Result<Vec<String>, String> = decode_one(&response).unwrap();
        let settled_agents = result.expect("Settlement should succeed");

        assert!(
            settled_agents.contains(&agent_id),
            "Agent should be included in settlement. Feature not implemented!"
        );
    }

    // Check balance after settlement
    let args = encode_args((agent_id.clone(), "UGX".to_string())).unwrap();
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        args,
    ).expect("get_agent_balance failed");

    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance_after = response.expect("Failed to get agent balance");

    // After settlement:
    // - commission_paid should increase
    // - commission_pending should decrease to 0
    assert_eq!(
        balance_after.commission_pending, 0,
        "Pending commission should be 0 after settlement. Feature not implemented!"
    );

    assert_eq!(
        balance_after.commission_paid, balance_before.commission_earned,
        "All earned commission should be marked as paid. Feature not implemented!"
    );
}

// ============================================================================
// Test 2: Minimum Settlement Amount Threshold
// ============================================================================

#[test]
fn test_minimum_settlement_amount() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333003".to_string()),
        None,
        "SmallAgent",
        "Test",
        "small@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333004".to_string()),
        None,
        "SmallUser",
        "Test",
        "small_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 200_000).expect("Failed to set balance");

    // Create small deposit (below settlement threshold)
    let deposit_request = CreateDepositRequest {
        user_id: user_id.clone(),
        agent_id: agent_id.clone(),
        amount: 100_000, // Commission will be 9,000 (below 50,000 threshold)
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

    // Trigger settlement
    let settlement_result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    );

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = settlement_result {
        let result: Result<Vec<String>, String> = decode_one(&response).unwrap();
        let settled_agents = result.expect("Settlement should succeed");

        // Agent should NOT be settled (below threshold)
        assert!(
            !settled_agents.contains(&agent_id),
            "Agent below minimum threshold should not be settled. Feature not implemented!"
        );
    }

    // Verify commission is still pending
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

    assert_eq!(
        balance.commission_paid, 0,
        "Commission should not be paid (below threshold)"
    );

    assert!(
        balance.commission_pending > 0,
        "Commission should still be pending"
    );
}

// ============================================================================
// Test 3: Settlement Commission Calculation Accuracy
// ============================================================================

#[test]
fn test_settlement_commission_calculation() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333005".to_string()),
        None,
        "CalcAgent",
        "Test",
        "calc@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333006".to_string()),
        None,
        "CalcUser",
        "Test",
        "calc_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 2_000_000).expect("Failed to set balance");

    // Create specific number of deposits with known commission
    // Deposit: 200,000 UGX
    // Agent commission: 10% = 20,000
    // Agent keeps: 90% = 18,000
    // Platform gets: 10% = 2,000
    let num_deposits = 5u64;
    let deposit_amount = 200_000u64;
    let expected_agent_commission = num_deposits * 18_000; // 90,000

    for i in 1..=num_deposits {
        let deposit_request = CreateDepositRequest {
            user_id: user_id.clone(),
            agent_id: agent_id.clone(),
            amount: deposit_amount,
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

    // Trigger settlement
    let settlement_result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    );

    // EXPECTED TO FAIL: Feature not implemented
    assert!(
        settlement_result.is_ok(),
        "Settlement should be triggerable. Feature not implemented!"
    );

    // Query settlement records
    let records_result = env.pic.query_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_settlement_records",
        encode_args((agent_id.clone(),)).unwrap(),
    );

    if let Ok(response) = records_result {
        #[derive(candid::CandidType, candid::Deserialize, Debug)]
        struct SettlementRecord {
            settlement_id: String,
            agent_id: String,
            currency: String,
            commission_amount: u64,
            settled_at: u64,
            week_start: u64,
            week_end: u64,
        }

        let (records,): (Vec<SettlementRecord>,) = candid::decode_args(&response)
            .expect("Failed to decode settlement records");

        assert!(
            records.len() >= 1,
            "Should have at least one settlement record. Feature not implemented!"
        );

        let latest = &records[0];
        assert_eq!(
            latest.commission_amount, expected_agent_commission,
            "Settlement commission should be exactly {}. Feature not implemented!",
            expected_agent_commission
        );
    }
}

// ============================================================================
// Test 4: Settlement Creates Transaction Records
// ============================================================================

#[test]
fn test_settlement_transaction_creation() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333007".to_string()),
        None,
        "TxAgent",
        "Test",
        "tx@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333008".to_string()),
        None,
        "TxUser",
        "Test",
        "tx_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Failed to set balance");

    // Create deposits
    for i in 1..=3 {
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
        ).expect(&format!("Deposit {} failed", i));

        let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
        let deposit = response.expect("Deposit creation failed");

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

    // Trigger settlement
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    ).expect("Settlement trigger should work");

    // Query transaction history for agent
    let tx_result = env.pic.query_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_transactions",
        encode_args((agent_id.clone(), Some(100usize))).unwrap(),
    );

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = tx_result {
        use shared_types::TransactionRecord;

        let (transactions,): (Vec<TransactionRecord>,) = candid::decode_args(&response)
            .expect("Failed to decode transactions");

        let settlement_txs: Vec<_> = transactions.iter()
            .filter(|tx| tx.transaction_type == "AgentSettlement"
                      || tx.transaction_type == "Settlement"
                      || tx.transaction_type.contains("settlement"))
            .collect();

        assert!(
            settlement_txs.len() >= 1,
            "Settlement should create transaction record. Feature not implemented!"
        );

        // Verify transaction details
        if let Some(tx) = settlement_txs.first() {
            assert_eq!(
                tx.to_user, Some(agent_id.clone()),
                "Settlement transaction should be TO the agent"
            );

            assert!(
                tx.amount > 0,
                "Settlement transaction should have positive amount"
            );
        }
    }
}

// ============================================================================
// Test 5: Settlement Idempotency (No Double Settlement)
// ============================================================================

#[test]
fn test_settlement_idempotency() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333009".to_string()),
        None,
        "IdempotentAgent",
        "Test",
        "idempotent@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333010".to_string()),
        None,
        "IdempotentUser",
        "Test",
        "idempotent_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Failed to set balance");

    // Create deposits
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
        ).expect(&format!("Deposit {} failed", i));

        let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
        let deposit = response.expect("Deposit creation failed");

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

    // Get balance before settlement
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
        args.clone(),
    ).expect("get_agent_balance failed");

    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance_before = response.expect("Failed to get agent balance");

    // Trigger settlement FIRST time
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    ).expect("First settlement should succeed");

    // Get balance after first settlement
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        args.clone(),
    ).expect("get_agent_balance failed");

    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance_after_first = response.expect("Failed to get agent balance");

    assert_eq!(
        balance_after_first.commission_pending, 0,
        "Pending should be 0 after settlement"
    );

    let first_commission_paid = balance_after_first.commission_paid;

    // Trigger settlement SECOND time (should be idempotent)
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    ).expect("Second settlement should succeed (idempotent)");

    // Get balance after second settlement
    let result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        args,
    ).expect("get_agent_balance failed");

    let response: Result<AgentBalanceResponse, String> = decode_one(&result).unwrap();
    let balance_after_second = response.expect("Failed to get agent balance");

    // EXPECTED TO FAIL: Feature not implemented
    assert_eq!(
        balance_after_second.commission_paid, first_commission_paid,
        "Commission paid should not change on second settlement (idempotency). Feature not implemented!"
    );

    assert_eq!(
        balance_after_second.commission_pending, 0,
        "Pending should still be 0 after second settlement"
    );
}

// ============================================================================
// Test 6: Settlement Dry Run Mode
// ============================================================================

#[test]
fn test_settlement_dry_run() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333011".to_string()),
        None,
        "DryRunAgent",
        "Test",
        "dryrun@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333012".to_string()),
        None,
        "DryRunUser",
        "Test",
        "dryrun_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Failed to set balance");

    // Create deposits
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
        ).expect(&format!("Deposit {} failed", i));

        let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
        let deposit = response.expect("Deposit creation failed");

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

    // Trigger DRY RUN settlement
    let dry_run_result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement_dry_run",
        encode_args(()).unwrap(),
    );

    // EXPECTED TO FAIL: Feature not implemented
    assert!(
        dry_run_result.is_ok(),
        "Dry run settlement should exist. Feature not implemented!"
    );

    if let Ok(response) = dry_run_result {
        #[derive(candid::CandidType, candid::Deserialize, Debug)]
        struct DryRunResult {
            agents_to_settle: Vec<String>,
            total_commission: u64,
            estimated_transactions: usize,
        }

        let result: Result<DryRunResult, String> = decode_one(&response).unwrap();
        let dry_run = result.expect("Dry run should succeed");

        assert!(
            dry_run.agents_to_settle.contains(&agent_id),
            "Agent should be in dry run results"
        );

        assert!(
            dry_run.total_commission > 0,
            "Should have commission to settle"
        );
    }

    // Verify balances are UNCHANGED after dry run
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

    assert!(
        balance.commission_pending > 0,
        "Dry run should NOT modify pending commission. Feature not implemented!"
    );

    assert_eq!(
        balance.commission_paid, 0,
        "Dry run should NOT pay commission. Feature not implemented!"
    );
}

// ============================================================================
// Test 7: Settlement Audit Trail
// ============================================================================

#[test]
fn test_settlement_audit_trail() {
    let env = TestEnv::new();

    // Register agent
    let agent_id = env.register_user(
        Some("+256700333013".to_string()),
        None,
        "AuditAgent",
        "Test",
        "audit@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent");

    // Register user
    let user_id = env.register_user(
        Some("+256700333014".to_string()),
        None,
        "AuditUser",
        "Test",
        "audit_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Failed to set balance");

    // Create deposits
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
        ).expect(&format!("Deposit {} failed", i));

        let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
        let deposit = response.expect("Deposit creation failed");

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

    // Trigger settlement
    env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    ).expect("Settlement should succeed");

    // Query audit log for settlement events
    let audit_result = env.pic.query_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        encode_args((Some(100usize),)).unwrap(),
    );

    // EXPECTED TO FAIL: Audit logging may not be implemented
    if let Ok(response) = audit_result {
        let (audit_entries,): (Vec<shared_types::AuditEntry>,) =
            candid::decode_args(&response)
                .expect("Failed to decode audit log");

        let settlement_logs: Vec<_> = audit_entries.iter()
            .filter(|entry| entry.action.contains("settlement")
                         || entry.action == "agent_settlement"
                         || entry.action == "weekly_settlement")
            .collect();

        assert!(
            settlement_logs.len() >= 1,
            "Settlement should be logged to audit trail. Feature not implemented!"
        );

        // Verify log contains agent_id and commission amount
        if let Some(log) = settlement_logs.first() {
            assert!(
                log.details.contains(&agent_id) || log.user_id == Some(agent_id.clone()),
                "Settlement log should reference agent_id"
            );

            assert!(
                log.success,
                "Settlement log should mark operation as successful"
            );
        }
    }
}

// ============================================================================
// Test 8: Multiple Agents Settlement in One Trigger
// ============================================================================

#[test]
fn test_multiple_agents_settlement() {
    let env = TestEnv::new();

    // Register 3 agents
    let agent1_id = env.register_user(
        Some("+256700333015".to_string()),
        None,
        "Agent1",
        "Multi",
        "agent1@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent1");

    let agent2_id = env.register_user(
        Some("+256700333016".to_string()),
        None,
        "Agent2",
        "Multi",
        "agent2@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent2");

    let agent3_id = env.register_user(
        Some("+256700333017".to_string()),
        None,
        "Agent3",
        "Multi",
        "agent3@test.com",
        "UGX",
        "1234",
    ).expect("Failed to register agent3");

    // Register user
    let user_id = env.register_user(
        Some("+256700333018".to_string()),
        None,
        "MultiUser",
        "Test",
        "multi_user@test.com",
        "UGX",
        "5678",
    ).expect("Failed to register user");

    // Give user large balance
    env.set_fiat_balance(&user_id, "UGX", 5_000_000).expect("Failed to set balance");

    // Create deposits for each agent (all above threshold)
    for agent_id in &[&agent1_id, &agent2_id, &agent3_id] {
        for i in 1..=5 {
            let deposit_request = CreateDepositRequest {
                user_id: user_id.clone(),
                agent_id: agent_id.to_string(),
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
            ).expect(&format!("Deposit {} for agent {} failed", i, agent_id));

            let response: Result<CreateDepositResponse, String> = decode_one(&result).unwrap();
            let deposit = response.expect("Deposit creation failed");

            let confirm_request = ConfirmDepositRequest {
                deposit_code: deposit.deposit_code,
                agent_id: agent_id.to_string(),
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
    }

    // Trigger settlement
    let settlement_result = env.pic.update_call(
        env.agent_canister_id,
        Principal::anonymous(),
        "trigger_weekly_settlement",
        encode_args(()).unwrap(),
    );

    // EXPECTED TO FAIL: Feature not implemented
    if let Ok(response) = settlement_result {
        let result: Result<Vec<String>, String> = decode_one(&response).unwrap();
        let settled_agents = result.expect("Settlement should succeed");

        // All 3 agents should be settled
        assert_eq!(
            settled_agents.len(), 3,
            "Should settle all 3 agents. Feature not implemented!"
        );

        assert!(
            settled_agents.contains(&agent1_id),
            "Agent1 should be settled"
        );

        assert!(
            settled_agents.contains(&agent2_id),
            "Agent2 should be settled"
        );

        assert!(
            settled_agents.contains(&agent3_id),
            "Agent3 should be settled"
        );
    }
}
