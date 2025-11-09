use candid::{encode_one, decode_one, Principal};
use pocket_ic::PocketIc;
use withdrawal_canister::{
    CreateWithdrawalRequest, ConfirmWithdrawalRequest,
    WithdrawalTransaction, TransactionStatus
};

const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/withdrawal_canister.wasm";

fn setup() -> (PocketIc, Principal) {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    
    let wasm = std::fs::read(WASM_PATH)
        .expect("Failed to read WASM file. Run: cargo build --target wasm32-unknown-unknown --release");
    
    pic.install_canister(canister_id, wasm, vec![], None);
    
    (pic, canister_id)
}

#[test]
fn test_create_withdrawal_request_success() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 10000,
    };
    
    let result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(request).unwrap(),
    );
    
    assert!(result.is_ok(), "Create withdrawal should succeed");
    
    let response: Result<WithdrawalTransaction, String> = 
        decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok());
    let tx = response.unwrap();
    assert_eq!(tx.amount_ugx, 10000);
    assert_eq!(tx.user_principal, user);
    assert_eq!(tx.agent_principal, agent);
    assert_eq!(tx.status, TransactionStatus::Pending);
}

#[test]
fn test_create_withdrawal_zero_amount_fails() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 0,
    };
    
    let result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(request).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<WithdrawalTransaction, String> = 
        decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), "Amount must be greater than 0");
}

#[test]
fn test_create_withdrawal_wrong_caller_fails() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let wrong_caller = Principal::from_text("2vxsx-fae").unwrap();
    let agent = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    let request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 10000,
    };
    
    let result = pic.update_call(
        canister_id,
        wrong_caller,
        "create_withdrawal_request",
        encode_one(request).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<WithdrawalTransaction, String> = 
        decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), "Caller must be the user");
}

#[test]
fn test_confirm_withdrawal_success() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let create_request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 10000,
    };
    
    let create_result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(create_request).unwrap(),
    ).unwrap();
    
    let create_response: Result<WithdrawalTransaction, String> = 
        decode_one(&create_result).unwrap();
    let tx = create_response.unwrap();
    
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: tx.withdrawal_code.clone(),
        agent_principal: agent,
    };
    
    let confirm_result = pic.update_call(
        canister_id,
        agent,
        "confirm_withdrawal",
        encode_one(confirm_request).unwrap(),
    );
    
    assert!(confirm_result.is_ok());
    let confirm_response: Result<WithdrawalTransaction, String> = 
        decode_one(&confirm_result.unwrap()).unwrap();
    
    assert!(confirm_response.is_ok());
    let confirmed_tx = confirm_response.unwrap();
    assert_eq!(confirmed_tx.status, TransactionStatus::Confirmed);
}

#[test]
fn test_confirm_withdrawal_wrong_agent_fails() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    let wrong_agent = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    
    let create_request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 10000,
    };
    
    let create_result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(create_request).unwrap(),
    ).unwrap();
    
    let create_response: Result<WithdrawalTransaction, String> = 
        decode_one(&create_result).unwrap();
    let tx = create_response.unwrap();
    
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: tx.withdrawal_code.clone(),
        agent_principal: wrong_agent,
    };
    
    let confirm_result = pic.update_call(
        canister_id,
        wrong_agent,
        "confirm_withdrawal",
        encode_one(confirm_request).unwrap(),
    );
    
    assert!(confirm_result.is_ok());
    let confirm_response: Result<WithdrawalTransaction, String> = 
        decode_one(&confirm_result.unwrap()).unwrap();
    
    assert!(confirm_response.is_err());
    assert_eq!(confirm_response.unwrap_err(), "Wrong agent");
}

#[test]
fn test_fee_calculation() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 100000,
    };
    
    let result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(request).unwrap(),
    ).unwrap();
    
    let response: Result<WithdrawalTransaction, String> = 
        decode_one(&result).unwrap();
    let tx = response.unwrap();
    
    assert_eq!(tx.platform_fee_ugx, 500);
    assert_eq!(tx.agent_fee_ugx, 10000);
}

#[test]
fn test_double_confirmation_fails() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let create_request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 10000,
    };
    
    let create_result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(create_request).unwrap(),
    ).unwrap();
    
    let create_response: Result<WithdrawalTransaction, String> = 
        decode_one(&create_result).unwrap();
    let tx = create_response.unwrap();
    
    let confirm_request = ConfirmWithdrawalRequest {
        withdrawal_code: tx.withdrawal_code.clone(),
        agent_principal: agent,
    };
    
    pic.update_call(
        canister_id,
        agent,
        "confirm_withdrawal",
        encode_one(confirm_request).unwrap(),
    ).unwrap();
    
    let confirm_request2 = ConfirmWithdrawalRequest {
        withdrawal_code: tx.withdrawal_code.clone(),
        agent_principal: agent,
    };
    
    let second_confirm = pic.update_call(
        canister_id,
        agent,
        "confirm_withdrawal",
        encode_one(confirm_request2).unwrap(),
    );
    
    assert!(second_confirm.is_ok());
    let response: Result<WithdrawalTransaction, String> = 
        decode_one(&second_confirm.unwrap()).unwrap();
    
    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), "Withdrawal already processed");
}

#[test]
fn test_multiple_withdrawals_same_user() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    for i in 1..=3 {
        let request = CreateWithdrawalRequest {
            user_principal: user,
            agent_principal: agent,
            amount_ugx: 10000 * i,
        };
        
        let result = pic.update_call(
            canister_id,
            user,
            "create_withdrawal_request",
            encode_one(request).unwrap(),
        );
        
        assert!(result.is_ok());
        let response: Result<WithdrawalTransaction, String> = 
            decode_one(&result.unwrap()).unwrap();
        assert!(response.is_ok());
    }
}

#[test]
fn test_withdrawal_code_uniqueness() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let request1 = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 10000,
    };
    
    let request2 = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 20000,
    };
    
    let result1 = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(request1).unwrap(),
    ).unwrap();
    
    let result2 = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(request2).unwrap(),
    ).unwrap();
    
    let tx1: Result<WithdrawalTransaction, String> = decode_one(&result1).unwrap();
    let tx2: Result<WithdrawalTransaction, String> = decode_one(&result2).unwrap();
    
    assert_ne!(tx1.unwrap().withdrawal_code, tx2.unwrap().withdrawal_code);
}

#[test]
fn test_large_withdrawal_amount() {
    let (pic, canister_id) = setup();
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    let agent = Principal::from_text("2vxsx-fae").unwrap();
    
    let request = CreateWithdrawalRequest {
        user_principal: user,
        agent_principal: agent,
        amount_ugx: 1_000_000,
    };
    
    let result = pic.update_call(
        canister_id,
        user,
        "create_withdrawal_request",
        encode_one(request).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<WithdrawalTransaction, String> = 
        decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok());
    let tx = response.unwrap();
    assert_eq!(tx.amount_ugx, 1_000_000);
    assert_eq!(tx.platform_fee_ugx, 5000);
    assert_eq!(tx.agent_fee_ugx, 100000);
}
