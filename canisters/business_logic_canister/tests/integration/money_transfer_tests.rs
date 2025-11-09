use candid::{encode_args, decode_one, Principal};
use pocket_ic::PocketIc;

const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/business_logic_canister.wasm";
const DATA_WASM: &str = "../../target/wasm32-unknown-unknown/release/data_canister.wasm";

#[derive(candid::CandidType, candid::Deserialize, Debug)]
struct TransactionResult {
    transaction_id: String,
    from_user: String,
    to_user: String,
    amount: u64,
    currency: String,
    new_balance: u64,
    timestamp: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone)]
struct User {
    id: String,
    phone: Option<String>,
    principal: Option<Principal>,
    name: String,
    created_at: u64,
    last_active: u64,
}

fn setup() -> (PocketIc, Principal, Principal) {
    let pic = PocketIc::new();
    
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    let wasm = std::fs::read(WASM_PATH)
        .expect("Failed to read business logic WASM. Run: cargo build --target wasm32-unknown-unknown --release --package business_logic_canister");
    pic.install_canister(canister_id, wasm, vec![], None);
    
    let data_canister_id = pic.create_canister();
    pic.add_cycles(data_canister_id, 2_000_000_000_000);
    let data_wasm = std::fs::read(DATA_WASM)
        .expect("Failed to read data canister WASM. Run: cargo build --target wasm32-unknown-unknown --release --package data_canister");
    pic.install_canister(data_canister_id, data_wasm, vec![], None);
    
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        encode_args((data_canister_id.to_text(),)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to set data canister ID");
    
    (pic, canister_id, data_canister_id)
}

fn create_test_user(pic: &PocketIc, data_canister: Principal, phone: &str, pin_hash: &str, initial_balance: u64) -> String {
    let user_id = format!("user_{}", phone);
    
    // Create user in data canister
    let user = User {
        id: user_id.clone(),
        phone: Some(phone.to_string()),
        principal: None,
        name: "Test User".to_string(),
        created_at: 0,
        last_active: 0,
    };
    
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "set_user",
        encode_args((user,)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to create user");
    
    // Set PIN hash
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "store_pin_hash",
        encode_args((user_id.clone(), pin_hash.to_string())).unwrap(),
    );
    assert!(result.is_ok(), "Failed to store PIN hash");
    
    // Set initial balance
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "set_fiat_balance",
        encode_args((user_id.clone(), "UGX".to_string(), initial_balance)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to set balance");
    
    user_id
}

#[test]
fn test_full_money_transfer_flow_success() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create sender with 100,000 UGX
    let sender_phone = "+256700123456";
    let sender_pin = "1234";
    create_test_user(&pic, data_canister, sender_phone, sender_pin, 100_000);
    
    // Create recipient with 0 UGX
    let recipient_phone = "+256700654321";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Transfer 10,000 UGX from sender to recipient
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            10_000u64,
            "UGX".to_string(),
            sender_pin.to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok(), "Transfer call failed");
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Transfer should succeed: {:?}", response);
    let tx = response.unwrap();
    
    // Verify transaction details
    assert_eq!(tx.amount, 10_000);
    assert_eq!(tx.currency, "UGX");
    assert_eq!(tx.new_balance, 90_000); // 100,000 - 10,000
    assert!(!tx.transaction_id.is_empty());
    
    // Verify sender balance decreased
    let sender_balance_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((format!("user_{}", sender_phone), "UGX".to_string())).unwrap(),
    );
    assert!(sender_balance_result.is_ok());
    let sender_balance: u64 = decode_one(&sender_balance_result.unwrap()).unwrap();
    assert_eq!(sender_balance, 90_000, "Sender balance should be 90,000");
    
    // Verify recipient balance increased
    let recipient_balance_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((format!("user_{}", recipient_phone), "UGX".to_string())).unwrap(),
    );
    assert!(recipient_balance_result.is_ok());
    let recipient_balance: u64 = decode_one(&recipient_balance_result.unwrap()).unwrap();
    assert_eq!(recipient_balance, 10_000, "Recipient balance should be 10,000");
}

#[test]
fn test_transfer_fails_with_insufficient_balance() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create sender with only 5,000 UGX
    let sender_phone = "+256700111111";
    let sender_pin = "1234";
    create_test_user(&pic, data_canister, sender_phone, sender_pin, 5_000);
    
    // Create recipient
    let recipient_phone = "+256700222222";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Try to transfer 10,000 UGX (more than balance)
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            10_000u64,
            "UGX".to_string(),
            sender_pin.to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Transfer should fail with insufficient balance");
    let error = response.unwrap_err();
    assert!(error.contains("insufficient") || error.contains("balance"), "Error should mention insufficient balance: {}", error);
}

#[test]
fn test_transfer_fails_with_wrong_pin() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create users
    let sender_phone = "+256700333333";
    let correct_pin = "1234";
    create_test_user(&pic, data_canister, sender_phone, correct_pin, 100_000);
    
    let recipient_phone = "+256700444444";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Try with wrong PIN
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            10_000u64,
            "UGX".to_string(),
            "9999".to_string(), // Wrong PIN
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Transfer should fail with wrong PIN");
    let error = response.unwrap_err();
    assert!(error.contains("PIN") || error.contains("Invalid"), "Error should mention PIN: {}", error);
}

#[test]
fn test_transfer_validates_zero_amount() {
    let (pic, canister_id, data_canister) = setup();
    
    let sender_phone = "+256700555555";
    create_test_user(&pic, data_canister, sender_phone, "1234", 100_000);
    
    let recipient_phone = "+256700666666";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Try to transfer 0 amount
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            0u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Transfer should fail with zero amount");
    let error = response.unwrap_err();
    assert!(error.contains("amount") || error.contains("greater than 0"), "Error should mention amount: {}", error);
}

#[test]
fn test_transfer_to_nonexistent_user_fails() {
    let (pic, canister_id, data_canister) = setup();
    
    let sender_phone = "+256700777777";
    create_test_user(&pic, data_canister, sender_phone, "1234", 100_000);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Try to transfer to non-existent user
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            "+256700999999".to_string(), // Non-existent
            10_000u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Transfer should fail for non-existent recipient");
    let error = response.unwrap_err();
    assert!(error.contains("not found") || error.contains("User"), "Error should mention user not found: {}", error);
}

// ============================================================================
// EDGE CASES - Multiple Currencies, Large Amounts, Transaction History
// ============================================================================

#[test]
fn test_transfer_with_different_currency_kes() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create users with KES (Kenyan Shillings)
    let sender_phone = "+254700111111";
    let sender_id = create_test_user(&pic, data_canister, sender_phone, "1234", 0);
    
    // Set KES balance
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "set_fiat_balance",
        encode_args((sender_id.clone(), "KES".to_string(), 50_000u64)).unwrap(),
    );
    assert!(result.is_ok());
    
    let recipient_phone = "+254700222222";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Transfer 10,000 KES
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            10_000u64,
            "KES".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "KES transfer should succeed: {:?}", response);
    let tx = response.unwrap();
    assert_eq!(tx.currency, "KES");
    assert_eq!(tx.amount, 10_000);
}

#[test]
fn test_transfer_with_nigerian_naira() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create users with NGN (Nigerian Naira)
    let sender_phone = "+234800111111";
    let sender_id = create_test_user(&pic, data_canister, sender_phone, "1234", 0);
    
    // Set NGN balance
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "set_fiat_balance",
        encode_args((sender_id.clone(), "NGN".to_string(), 200_000u64)).unwrap(),
    );
    assert!(result.is_ok());
    
    let recipient_phone = "+234800222222";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Transfer 50,000 NGN
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            50_000u64,
            "NGN".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "NGN transfer should succeed: {:?}", response);
    let tx = response.unwrap();
    assert_eq!(tx.currency, "NGN");
}

#[test]
fn test_multiple_transfers_update_balance_correctly() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create sender with 1M UGX
    let sender_phone = "+256700123123";
    create_test_user(&pic, data_canister, sender_phone, "1234", 1_000_000);
    
    // Create recipient
    let recipient_phone = "+256700456456";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Make 3 transfers of 100k each
    for i in 1..=3 {
        let result = pic.update_call(
            canister_id,
            user,
            "send_money_to_phone",
            encode_args((
                sender_phone.to_string(),
                recipient_phone.to_string(),
                100_000u64,
                "UGX".to_string(),
                "1234".to_string(),
            )).unwrap(),
        );
        
        assert!(result.is_ok(), "Transfer {} should succeed", i);
        let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
        assert!(response.is_ok(), "Transfer {} should succeed: {:?}", i, response);
        
        let tx = response.unwrap();
        let expected_balance = 1_000_000 - (i * 100_000);
        assert_eq!(tx.new_balance, expected_balance, "Balance after transfer {} should be {}", i, expected_balance);
    }
    
    // Verify final balances
    let sender_balance_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((format!("user_{}", sender_phone), "UGX".to_string())).unwrap(),
    );
    assert!(sender_balance_result.is_ok());
    let sender_balance: u64 = decode_one(&sender_balance_result.unwrap()).unwrap();
    assert_eq!(sender_balance, 700_000, "Sender final balance should be 700k");
    
    let recipient_balance_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((format!("user_{}", recipient_phone), "UGX".to_string())).unwrap(),
    );
    assert!(recipient_balance_result.is_ok());
    let recipient_balance: u64 = decode_one(&recipient_balance_result.unwrap()).unwrap();
    assert_eq!(recipient_balance, 300_000, "Recipient final balance should be 300k");
}

#[test]
fn test_transfer_with_exact_balance_leaves_zero() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create sender with exactly 50,000 UGX
    let sender_phone = "+256700789789";
    create_test_user(&pic, data_canister, sender_phone, "1234", 50_000);
    
    let recipient_phone = "+256700987987";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Transfer all 50,000 UGX
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            50_000u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Transfer of exact balance should succeed: {:?}", response);
    let tx = response.unwrap();
    assert_eq!(tx.new_balance, 0, "Sender balance should be 0");
    
    // Verify sender has 0 balance
    let sender_balance_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((format!("user_{}", sender_phone), "UGX".to_string())).unwrap(),
    );
    assert!(sender_balance_result.is_ok());
    let sender_balance: u64 = decode_one(&sender_balance_result.unwrap()).unwrap();
    assert_eq!(sender_balance, 0);
}

#[test]
fn test_transfer_one_unit_above_balance_fails() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create sender with 50,000 UGX
    let sender_phone = "+256700321321";
    create_test_user(&pic, data_canister, sender_phone, "1234", 50_000);
    
    let recipient_phone = "+256700654654";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::from_text("aaaaa-aa").unwrap();
    
    // Try to transfer 50,001 UGX (1 more than balance)
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            50_001u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Transfer should fail");
    let error = response.unwrap_err();
    assert!(error.contains("insufficient") || error.contains("balance"), 
        "Error should mention insufficient balance: {}", error);
}
