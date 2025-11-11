use crate::integration::TestEnv;
use candid::{encode_one, encode_args, decode_one, Principal};

// ============================================================================
// DEPOSIT COMMISSION INTEGRATION TESTS
// ============================================================================
// These tests verify the deposit canister commission system:
// - 0.5% platform fee on deposits
// - 10% agent commission on deposits
// - Agent commission tracking
// - Monthly settlements
// ============================================================================

#[test]
fn test_deposit_canister_deployed() {
    let env = TestEnv::new_with_commission_canisters();
    
    // Verify deposit canister is deployed
    assert!(env.deposit_canister_id.is_some(), "Deposit canister should be deployed");
    
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    // Try to call a query method to verify it's working
    let result = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_all_agent_balances",
        vec![],
    );
    
    match result {
        Ok(_) => println!("✅ Deposit canister responds to queries"),
        Err(e) => panic!("❌ Deposit canister query failed: {:?}", e),
    }
}

#[test]
fn test_create_deposit_request_calculates_commission() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    // First, test if we can call a simple query method
    println!("Testing basic canister functionality...");
    let query_result = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_commission_rate",
        vec![],
    );
    
    match query_result {
        Ok(response) => {
            let rate: u64 = decode_one(&response).expect("Failed to decode commission rate");
            println!("✅ Commission rate: {} basis points", rate);
        }
        Err(e) => {
            panic!("❌ Failed to query commission rate: {:?}", e);
        }
    }
    
    // Create test principals
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    // Create deposit request for 100,000 UGX
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
    
    println!("Creating deposit request...");
    let arg = encode_args((request,)).unwrap();
    let response = env.pic.update_call(
        deposit_canister_id,
        user_principal, // Caller must be user
        "create_deposit_request",
        arg,
    );
    
    // Check if call succeeded
    match response {
        Ok(response) => {
            println!("✅ Call succeeded, decoding response...");
            
            // Decode response - must match exact field order from deposit canister
            #[derive(candid::CandidType, candid::Deserialize, Debug)]
            struct DepositTransaction {
                pub id: u64,
                pub user_principal: Principal,
                pub agent_principal: Principal,
                pub amount_ugx: u64,
                pub commission_ugx: u64,
                pub deposit_code: String,
                pub timestamp: u64,
                pub status: TransactionStatus,
            }
            
            #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
            enum TransactionStatus {
                Pending,
                Confirmed,
                Cancelled,
            }
            
            let result: Result<DepositTransaction, String> = decode_one(&response)
                .expect("Failed to decode response");
            
            match result {
                Ok(deposit) => {
                    println!("✅ Deposit created: {:?}", deposit);
                    
                    // Verify commission calculation: 0.5% of 100,000 = 500 UGX
                    assert_eq!(deposit.commission_ugx, 500, "Commission should be 0.5% = 500 UGX");
                    assert_eq!(deposit.amount_ugx, 100_000);
                    assert_eq!(deposit.status, TransactionStatus::Pending);
                    assert!(deposit.deposit_code.starts_with("DEP"), "Deposit code should start with DEP");
                }
                Err(e) => panic!("❌ Deposit creation failed: {}", e),
            }
        }
        Err(e) => panic!("❌ create_deposit_request call failed: {:?}", e),
    }
}

#[test]
fn test_confirm_deposit_updates_agent_balance() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    let user_principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
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
        status: DepositStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum DepositStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    let result: Result<DepositTransaction, String> = decode_one(&response).unwrap();
    let deposit = result.unwrap();
    let deposit_code = deposit.deposit_code.clone();
    
    // Confirm deposit
    #[derive(candid::CandidType)]
    struct ConfirmDepositRequest {
        deposit_code: String,
        agent_principal: Principal,
    }
    
    let confirm_request = ConfirmDepositRequest {
        deposit_code,
        agent_principal,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    let confirm_response = env.pic.update_call(
        deposit_canister_id,
        agent_principal, // Caller must be agent
        "confirm_deposit",
        confirm_arg,
    ).expect("confirm_deposit should succeed");
    
    let confirm_result: Result<DepositTransaction, String> = decode_one(&confirm_response).unwrap();
    let confirmed_deposit = confirm_result.unwrap();
    
    assert_eq!(confirmed_deposit.status, DepositStatus::Confirmed);
    
    // Check agent balance
    let balance_arg = encode_args((agent_principal,)).unwrap();
    let balance_response = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        balance_arg,
    ).expect("get_agent_balance should succeed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentBalance {
        principal: Principal,
        total_deposits: u64,
        total_commission_owed: u64,
        total_commission_paid: u64,
        last_settlement_date: Option<u64>,
    }
    
    let balance_result: Option<AgentBalance> = decode_one(&balance_response).unwrap();
    let agent_balance = balance_result.expect("Agent should have balance");
    
    // Verify agent owes 500 UGX commission
    assert_eq!(agent_balance.total_deposits, 100_000);
    assert_eq!(agent_balance.total_commission_owed, 500);
    assert_eq!(agent_balance.total_commission_paid, 0);
}

#[test]
fn test_multiple_deposits_accumulate_commission() {
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
        status: DepositStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum DepositStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    // Create and confirm 3 deposits
    let amounts = vec![100_000, 200_000, 50_000];
    let expected_commissions = vec![500, 1_000, 250]; // 0.5% of each
    
    for amount in amounts.iter() {
        // Create deposit
        let request = CreateDepositRequest {
            user_principal,
            agent_principal,
            amount_ugx: *amount,
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
        
        // Confirm deposit
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
    
    // Check total commission owed
    let balance_arg = encode_args((agent_principal,)).unwrap();
    let balance_response = env.pic.query_call(
        deposit_canister_id,
        Principal::anonymous(),
        "get_agent_balance",
        balance_arg,
    ).expect("get_agent_balance should succeed");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct AgentBalance {
        principal: Principal,
        total_deposits: u64,
        total_commission_owed: u64,
        total_commission_paid: u64,
        last_settlement_date: Option<u64>,
    }
    
    let balance_result: Option<AgentBalance> = decode_one(&balance_response).unwrap();
    let agent_balance = balance_result.expect("Agent should have balance");
    
    // Total deposits: 350,000 UGX
    // Total commission: 500 + 1,000 + 250 = 1,750 UGX
    assert_eq!(agent_balance.total_deposits, 350_000);
    assert_eq!(agent_balance.total_commission_owed, 1_750);
}

#[test]
fn test_invalid_deposit_code_rejected() {
    let env = TestEnv::new_with_commission_canisters();
    let deposit_canister_id = env.deposit_canister_id.unwrap();
    
    let agent_principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    #[derive(candid::CandidType)]
    struct ConfirmDepositRequest {
        deposit_code: String,
        agent_principal: Principal,
    }
    
    let confirm_request = ConfirmDepositRequest {
        deposit_code: "DEP-999999".to_string(), // Invalid code
        agent_principal,
    };
    
    let confirm_arg = encode_args((confirm_request,)).unwrap();
    let confirm_response = env.pic.update_call(
        deposit_canister_id,
        agent_principal,
        "confirm_deposit",
        confirm_arg,
    ).expect("confirm_deposit call should complete");
    
    #[derive(candid::CandidType, candid::Deserialize, Debug)]
    struct DepositTransaction {
        id: u64,
        user_principal: Principal,
        agent_principal: Principal,
        amount_ugx: u64,
        commission_ugx: u64,
        deposit_code: String,
        timestamp: u64,
        status: DepositStatus,
    }
    
    #[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq)]
    enum DepositStatus {
        Pending,
        Confirmed,
        Cancelled,
    }
    
    let result: Result<DepositTransaction, String> = decode_one(&confirm_response).unwrap();
    
    assert!(result.is_err(), "Invalid deposit code should be rejected");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found") || err_msg.contains("Deposit code"), 
        "Error should mention deposit code not found: {}", err_msg);
}
