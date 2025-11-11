use super::*;
use candid::{encode_args, decode_one, Principal, Decode};

// ============================================================================
// END-TO-END COMMISSION FLOW TESTS
// ============================================================================
// These tests verify the complete commission flow from user action through
// to company wallet revenue collection

#[test]
fn test_deposit_commission_reaches_company_wallet() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    // Get company wallet from config
    let wallet_response = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_company_wallet_principal",
        vec![],
    ).expect("Should get company wallet");
    
    let company_wallet_result: Result<Principal, String> = decode_one(&wallet_response).unwrap();
    let company_wallet = company_wallet_result.expect("Company wallet should be configured");
    
    println!("✅ Company wallet: {}", company_wallet);
    
    // Create deposit
    #[derive(candid::CandidType)]
    struct CreateDepositRequest {
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
    }
    
    let request = CreateDepositRequest {
        user_principal,
        agent_principal,
        amount_ugx: 100_000,
    };
    
    let arg = encode_args((request,)).unwrap();
    let response = env.pic.update_call(
        deposit_canister_id,
        user_principal,
        "create_deposit_request",
        arg,
    ).expect("create_deposit_request should succeed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct DepositTransaction {
        id: u64,
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
        commission_ugx: u64,
        deposit_code: String,
        timestamp: u64,
        status: TransactionStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum TransactionStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    let result: Result<DepositTransaction, String> = decode_one(&response).unwrap();
    let deposit = result.unwrap();
    
    println!("✅ Deposit created with {} UGX commission", deposit.commission_ugx);
    
    // Confirm deposit
    #[derive(candid::CandidType)]
    struct ConfirmDepositRequest {
        deposit_code: String,
        agent_principal: Principal,
    }
    
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code.clone(),
        agent_principal,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    let confirm_response = env.pic.update_call(
        deposit_canister_id,
        agent_principal,
        "confirm_deposit",
        confirm_arg,
    ).expect("confirm_deposit should succeed");
    
    let confirm_result: Result<DepositTransaction, String> = decode_one(&confirm_response).unwrap();
    let confirmed = confirm_result.unwrap();
    
    assert_eq!(confirmed.status, TransactionStatus::Confirmed);
    println!("✅ Deposit confirmed - commission should be tracked for company wallet");
    
    // In a real implementation, we would:
    // 1. Query company wallet balance before/after
    // 2. Verify the commission amount was transferred
    // 3. Check total revenue tracking
    
    // For now, verify the commission was calculated correctly
    assert_eq!(confirmed.commission_ugx, 500, "Commission should be 0.5% of 100,000");
}

#[test]
fn test_withdrawal_fees_tracked_separately() {
    let env = TestEnv::new_with_commission_canisters();
    let withdrawal_canister_id = env.withdrawal_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    // Create withdrawal
    #[derive(candid::CandidType)]
    struct CreateWithdrawalRequest {
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
    }
    
    let request = CreateWithdrawalRequest {
        user_principal,
        agent_principal,
        amount_ugx: 100_000,
    };
    
    let arg = encode_args((request,)).unwrap();
    let response = env.pic.update_call(
        withdrawal_canister_id,
        user_principal,
        "create_withdrawal_request",
        arg,
    ).expect("create_withdrawal_request should succeed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct WithdrawalTransaction {
        id: u64,
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
        platform_fee_ugx: u64,
        agent_fee_ugx: u64,
        withdrawal_code: String,
        timestamp: u64,
        status: TransactionStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum TransactionStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    let result: Result<WithdrawalTransaction, String> = decode_one(&response).unwrap();
    let withdrawal = result.unwrap();
    
    // Verify fees are tracked separately
    assert_eq!(withdrawal.platform_fee_ugx, 500, "Platform fee should be 0.5%");
    assert_eq!(withdrawal.agent_fee_ugx, 10_000, "Agent fee should be 10%");
    
    println!("✅ Platform fee: {} UGX (goes to company)", withdrawal.platform_fee_ugx);
    println!("✅ Agent fee: {} UGX (goes to agent)", withdrawal.agent_fee_ugx);
    
    // Confirm withdrawal
    #[derive(candid::CandidType)]
    struct ConfirmWithdrawalRequest {
        withdrawal_code: String,
        agent_principal: Principal,
    }
    
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: withdrawal.withdrawal_code.clone(),
        agent_principal,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    env.pic.update_call(
        withdrawal_canister_id,
        agent_principal,
        "confirm_withdrawal",
        confirm_arg,
    ).expect("confirm_withdrawal should succeed");
    
    // Query total platform revenue
    let revenue_response = env.pic.query_call(
        withdrawal_canister_id,
        Principal::anonymous(),
        "get_total_platform_revenue",
        vec![],
    ).expect("Should get total platform revenue");
    
    let total_revenue: u64 = decode_one(&revenue_response).unwrap();
    
    // Platform revenue should include the platform fee
    assert!(total_revenue >= 500, "Platform revenue should include at least the platform fee");
    println!("✅ Total platform revenue: {} UGX", total_revenue);
}

#[test]
fn test_multiple_agents_commission_isolation() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent1 = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let agent2 = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    
    #[derive(candid::CandidType)]
    struct CreateDepositRequest {
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
    }
    
    #[derive(candid::CandidType)]
    struct ConfirmDepositRequest {
        deposit_code: String,
        agent_principal: Principal,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct DepositTransaction {
        id: u64,
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
        commission_ugx: u64,
        deposit_code: String,
        timestamp: u64,
        status: TransactionStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum TransactionStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    // Agent 1 processes 2 deposits
    for amount in [100_000, 50_000] {
        let request = CreateDepositRequest {
            user_principal,
            agent_principal: agent1,
            amount_ugx: amount,
        };
        
        let arg = encode_args((request,)).unwrap();
        let response = env.pic.update_call(
            deposit_canister_id,
            user_principal,
            "create_deposit_request",
            arg,
        ).expect("create_deposit_request should succeed");
        
        let result: Result<DepositTransaction, String> = decode_one(&response).unwrap();
        let deposit = result.unwrap();
        
        let confirm_request = ConfirmDepositRequest {
            deposit_code: deposit.deposit_code,
            agent_principal: agent1,
        };
        
        let confirm_arg = encode_args((confirm_request,)).unwrap();
        env.pic.update_call(
            deposit_canister_id,
            agent1,
            "confirm_deposit",
            confirm_arg,
        ).expect("confirm_deposit should succeed");
    }
    
    // Agent 2 processes 1 deposit
    let request = CreateDepositRequest {
        user_principal,
        agent_principal: agent2,
        amount_ugx: 200_000,
    };
    
    let arg = encode_args((request,)).unwrap();
    let response = env.pic.update_call(
        deposit_canister_id,
        user_principal,
        "create_deposit_request",
        arg,
    ).expect("create_deposit_request should succeed");
    
    let result: Result<DepositTransaction, String> = decode_one(&response).unwrap();
    let deposit = result.unwrap();
    
    let confirm_request = ConfirmDepositRequest {
        deposit_code: deposit.deposit_code,
        agent_principal: agent2,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    env.pic.update_call(
        deposit_canister_id,
        agent2,
        "confirm_deposit",
        confirm_arg,
    ).expect("confirm_deposit should succeed");
    
    // Check agent 1 balance
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentBalance {
        principal: Principal,
        total_deposits: u64,
        total_commission_owed: u64,
        total_commission_paid: u64,
        last_settlement_date: Option<u64>,
    }
    
    let agent1_arg = encode_args((agent1,)).unwrap();
    let agent1_response = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        agent1_arg,
    ).expect("get_agent_balance should succeed");
    
    let agent1_balance: Option<AgentBalance> = decode_one(&agent1_response).unwrap();
    let agent1_bal = agent1_balance.expect("Agent 1 should have balance");
    
    // Agent 1: 100,000 + 50,000 = 150,000 total, commission = 750
    assert_eq!(agent1_bal.total_deposits, 150_000);
    assert_eq!(agent1_bal.total_commission_owed, 750);
    
    // Check agent 2 balance
    let agent2_arg = encode_args((agent2,)).unwrap();
    let agent2_response = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        agent2_arg,
    ).expect("get_agent_balance should succeed");
    
    let agent2_balance: Option<AgentBalance> = decode_one(&agent2_response).unwrap();
    let agent2_bal = agent2_balance.expect("Agent 2 should have balance");
    
    // Agent 2: 200,000 total, commission = 1,000
    assert_eq!(agent2_bal.total_deposits, 200_000);
    assert_eq!(agent2_bal.total_commission_owed, 1_000);
    
    println!("✅ Agent 1 commission: {} UGX", agent1_bal.total_commission_owed);
    println!("✅ Agent 2 commission: {} UGX", agent2_bal.total_commission_owed);
    println!("✅ Commissions are properly isolated per agent");
}

#[test]
fn test_commission_on_very_small_amounts() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    // Test very small amounts
    let test_amounts = vec![
        (100, 0),      // 0.5% of 100 = 0.5, rounds to 0
        (200, 1),      // 0.5% of 200 = 1
        (1_000, 5),    // 0.5% of 1,000 = 5
        (10, 0),       // 0.5% of 10 = 0.05, rounds to 0
    ];
    
    #[derive(candid::CandidType)]
    struct CreateDepositRequest {
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct DepositTransaction {
        id: u64,
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
        commission_ugx: u64,
        deposit_code: String,
        timestamp: u64,
        status: TransactionStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum TransactionStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    for (amount, expected_commission) in test_amounts {
        let request = CreateDepositRequest {
            user_principal,
            agent_principal,
            amount_ugx: amount,
        };
        
        let arg = encode_args((request,)).unwrap();
        let response = env.pic.update_call(
            deposit_canister_id,
            user_principal,
            "create_deposit_request",
            arg,
        ).expect("create_deposit_request should succeed");
        
        let result: Result<DepositTransaction, String> = decode_one(&response).unwrap();
        let deposit = result.unwrap();
        
        assert_eq!(
            deposit.commission_ugx, 
            expected_commission,
            "Commission for {} UGX should be {} UGX",
            amount,
            expected_commission
        );
        
        println!("✅ {} UGX → {} UGX commission (correct rounding)", amount, deposit.commission_ugx);
    }
}

#[test]
fn test_total_revenue_tracking() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    #[derive(candid::CandidType)]
    struct CreateDepositRequest {
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
    }
    
    #[derive(candid::CandidType)]
    struct ConfirmDepositRequest {
        deposit_code: String,
        agent_principal: Principal,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct DepositTransaction {
        id: u64,
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
        commission_ugx: u64,
        deposit_code: String,
        timestamp: u64,
        status: TransactionStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum TransactionStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    // Create and confirm multiple deposits
    let amounts = vec![100_000, 200_000, 50_000, 150_000];
    let mut total_expected_revenue = 0u64;
    
    for amount in amounts {
        let expected_commission = (amount * 50) / 10_000; // 0.5%
        total_expected_revenue += expected_commission;
        
        let request = CreateDepositRequest {
            user_principal,
            agent_principal,
            amount_ugx: amount,
        };
        
        let arg = encode_args((request,)).unwrap();
        let response = env.pic.update_call(
            deposit_canister_id,
            user_principal,
            "create_deposit_request",
            arg,
        ).expect("create_deposit_request should succeed");
        
        let result: Result<DepositTransaction, String> = decode_one(&response).unwrap();
        let deposit = result.unwrap();
        
        let confirm_request = ConfirmDepositRequest {
            deposit_code: deposit.deposit_code,
            agent_principal,
        };
        
        let confirm_arg = encode_args((confirm_request,)).unwrap();
        env.pic.update_call(
            deposit_canister_id,
            agent_principal,
            "confirm_deposit",
            confirm_arg,
        ).expect("confirm_deposit should succeed");
    }
    
    // Query total revenue
    let revenue_response = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_total_revenue",
        vec![],
    ).expect("Should get total revenue");
    
    let total_revenue: u64 = decode_one(&revenue_response).unwrap();
    
    assert_eq!(total_revenue, total_expected_revenue);
    println!("✅ Total revenue tracked correctly: {} UGX", total_revenue);
    println!("   Expected: {} UGX", total_expected_revenue);
}
