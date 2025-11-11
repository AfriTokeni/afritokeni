use super::*;
use candid::{encode_args, decode_one, Principal};

// ============================================================================
// WITHDRAWAL COMMISSION TESTS
// ============================================================================

#[test]
fn test_withdrawal_canister_deployed() {
    let env = TestEnv::new_with_commission_canisters();
    let withdrawal_canister_id = env.withdrawal_canister_id.unwrap();
    
    // Test basic query to verify canister is responsive
    let response = env.pic.query_call(
        withdrawal_canister_id,
        Principal::anonymous(),
        "get_fee_split",
        vec![],
    );
    
    match response {
        Ok(_) => println!("✅ Withdrawal canister responds to queries"),
        Err(e) => panic!("❌ Withdrawal canister query failed: {:?}", e),
    }
}

#[test]
fn test_create_withdrawal_request_calculates_fees() {
    let env = TestEnv::new_with_commission_canisters();
    let withdrawal_canister_id = env.withdrawal_canister_id.unwrap();
    
    // First, verify fee split
    println!("Testing basic canister functionality...");
    let query_result = env.pic.query_call(
        withdrawal_canister_id,
        Principal::anonymous(),
        "get_fee_split",
        vec![],
    );
    
    match query_result {
        Ok(response) => {
            // The canister returns two separate values, decode them as a tuple
            use candid::Decode;
            let (platform_fee, agent_commission) = Decode!(&response, u64, u64).expect("Failed to decode fee split");
            println!("✅ Platform fee: {} basis points, Agent commission: {} basis points", platform_fee, agent_commission);
        }
        Err(e) => {
            panic!("❌ Failed to query fee split: {:?}", e);
        }
    }
    
    // Create test principals
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    // Create withdrawal request for 100,000 UGX
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
    
    println!("Creating withdrawal request...");
    let arg = encode_args((request,)).unwrap();
    let response = env.pic.update_call(
        withdrawal_canister_id,
        user_principal, // Caller must be user
        "create_withdrawal_request",
        arg,
    );
    
    // Check if call succeeded
    match response {
        Ok(response) => {
            println!("✅ Call succeeded, decoding response...");
            
            // Decode response - must match exact field order from withdrawal canister
            #[derive(candid::CandidType, candid::Deserialize, Debug)]
            struct WithdrawalTransaction {
                pub id: u64,
                pub user_principal: Principal,
                pub agent_principal: Principal,
                pub amount_ugx: u64,
                pub platform_fee_ugx: u64,
                pub agent_fee_ugx: u64,
                pub withdrawal_code: String,
                pub timestamp: u64,
                pub status: TransactionStatus,
            }
            
            #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
            enum TransactionStatus {
                Pending,
                Confirmed,
                Cancelled,
            }
            
            let result: Result<WithdrawalTransaction, String> = decode_one(&response)
                .expect("Failed to decode response");
            
            match result {
                Ok(withdrawal) => {
                    println!("✅ Withdrawal created: {:?}", withdrawal);
                    
                    // Verify fee calculations:
                    // Platform fee: 0.5% of 100,000 = 500 UGX
                    // Agent fee: 10% of 100,000 = 10,000 UGX (agent keeps 100%)
                    assert_eq!(withdrawal.platform_fee_ugx, 500, "Platform fee should be 0.5% = 500 UGX");
                    assert_eq!(withdrawal.agent_fee_ugx, 10_000, "Agent fee should be 10% = 10,000 UGX");
                    assert_eq!(withdrawal.amount_ugx, 100_000);
                    assert_eq!(withdrawal.status, TransactionStatus::Pending);
                    assert!(withdrawal.withdrawal_code.starts_with("WTH"), "Withdrawal code should start with WTH");
                }
                Err(e) => panic!("❌ Withdrawal creation failed: {}", e),
            }
        }
        Err(e) => panic!("❌ create_withdrawal_request call failed: {:?}", e),
    }
}

#[test]
fn test_confirm_withdrawal_updates_agent_earnings() {
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
    let withdrawal_code = withdrawal.withdrawal_code.clone();
    
    // Confirm withdrawal
    #[derive(candid::CandidType)]
    struct ConfirmWithdrawalRequest {
        withdrawal_code: String,
        agent_principal: Principal,
    }
    
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code,
        agent_principal,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    let confirm_response = env.pic.update_call(
        withdrawal_canister_id,
        agent_principal, // Caller must be agent
        "confirm_withdrawal",
        confirm_arg,
    ).expect("confirm_withdrawal should succeed");
    
    let confirm_result: Result<WithdrawalTransaction, String> = decode_one(&confirm_response).unwrap();
    let confirmed_withdrawal = confirm_result.unwrap();
    
    assert_eq!(confirmed_withdrawal.status, TransactionStatus::Confirmed);
    
    // Check agent earnings
    let earnings_arg = encode_args((agent_principal,)).unwrap();
    let earnings_response = env.pic.query_call(
        withdrawal_canister_id,
        Principal::anonymous(),
        "get_agent_earnings",
        earnings_arg,
    ).expect("get_agent_earnings should succeed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentEarnings {
        principal: Principal,
        total_withdrawals_processed: u64,
        total_fees_earned: u64,
        total_fees_withdrawn: u64,
        last_withdrawal_date: Option<u64>,
    }
    
    let earnings_result: Option<AgentEarnings> = decode_one(&earnings_response).unwrap();
    let agent_earnings = earnings_result.expect("Agent should have earnings");
    
    // Agent earns 100% of 10,000 = 10,000 UGX
    assert_eq!(agent_earnings.total_withdrawals_processed, 100_000);
    assert_eq!(agent_earnings.total_fees_earned, 10_000);
    assert_eq!(agent_earnings.total_fees_withdrawn, 0);
}

#[test]
fn test_multiple_withdrawals_accumulate_earnings() {
    let env = TestEnv::new_with_commission_canisters();
    let withdrawal_canister_id = env.withdrawal_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    #[derive(candid::CandidType)]
    struct CreateWithdrawalRequest {
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
    }
    
    #[derive(candid::CandidType)]
    struct ConfirmWithdrawalRequest {
        withdrawal_code: String,
        agent_principal: Principal,
    }
    
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
    
    // Create and confirm multiple withdrawals
    let amounts = vec![100_000, 200_000, 50_000];
    let _expected_earnings = vec![10_000, 20_000, 5_000]; // 10% of each
    
    for amount in amounts {
        // Create withdrawal
        let request = CreateWithdrawalRequest {
            user_principal,
            agent_principal,
            amount_ugx: amount,
        };
        
        let arg = encode_args((request,)).unwrap();
        let response = env.pic.update_call(
            withdrawal_canister_id,
            user_principal,
            "create_withdrawal_request",
            arg,
        ).expect("create_withdrawal_request should succeed");
        
        let result: Result<WithdrawalTransaction, String> = decode_one(&response).unwrap();
        let withdrawal = result.unwrap();
        
        // Confirm withdrawal
        let confirm_request = ConfirmWithdrawalRequest {
            withdrawal_code: withdrawal.withdrawal_code,
            agent_principal,
        };
        
        let confirm_arg = encode_args((confirm_request,)).unwrap();
        env.pic.update_call(
            withdrawal_canister_id,
            agent_principal,
            "confirm_withdrawal",
            confirm_arg,
        ).expect("confirm_withdrawal should succeed");
    }
    
    // Check accumulated earnings
    let earnings_arg = encode_args((agent_principal,)).unwrap();
    let earnings_response = env.pic.query_call(
        withdrawal_canister_id,
        Principal::anonymous(),
        "get_agent_earnings",
        earnings_arg,
    ).expect("get_agent_earnings should succeed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentEarnings {
        principal: Principal,
        total_withdrawals_processed: u64,
        total_fees_earned: u64,
        total_fees_withdrawn: u64,
        last_withdrawal_date: Option<u64>,
    }
    
    let earnings_result: Option<AgentEarnings> = decode_one(&earnings_response).unwrap();
    let agent_earnings = earnings_result.expect("Agent should have earnings");
    
    // Total withdrawals: 350,000 UGX
    // Total earnings: 10,000 + 20,000 + 5,000 = 35,000 UGX
    assert_eq!(agent_earnings.total_withdrawals_processed, 350_000);
    assert_eq!(agent_earnings.total_fees_earned, 35_000);
}

#[test]
fn test_invalid_withdrawal_code_rejected() {
    let env = TestEnv::new_with_commission_canisters();
    let withdrawal_canister_id = env.withdrawal_canister_id.unwrap();
    
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    #[derive(candid::CandidType)]
    struct ConfirmWithdrawalRequest {
        withdrawal_code: String,
        agent_principal: Principal,
    }
    
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: "WD-999999".to_string(), // Invalid code
        agent_principal,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    let confirm_response = env.pic.update_call(
        withdrawal_canister_id,
        agent_principal,
        "confirm_withdrawal",
        confirm_arg,
    ).expect("Call should succeed but return error");
    
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
    
    let result: Result<WithdrawalTransaction, String> = decode_one(&confirm_response).unwrap();
    
    assert!(result.is_err(), "Invalid withdrawal code should be rejected");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found") || err_msg.contains("Withdrawal code"), 
        "Error should mention withdrawal code not found: {}", err_msg);
}
