use candid::{encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::{TransactionResult, User};

const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/business_logic_canister.wasm";
const DATA_WASM: &str = "../../target/wasm32-unknown-unknown/release/data_canister.wasm";

fn setup() -> (PocketIc, Principal, Principal) {
    let pic = PocketIc::new();
    
    // Install data canister FIRST
    let data_canister_id = pic.create_canister();
    pic.add_cycles(data_canister_id, 2_000_000_000_000);
    let data_wasm = std::fs::read(DATA_WASM)
        .expect("Failed to read data canister WASM");
    
    // Data canister init expects (Option<String>, Option<String>) for ussd_canister_id and web_canister_id
    let data_init_args = encode_args((None::<String>, None::<String>)).unwrap();
    pic.install_canister(data_canister_id, data_wasm, data_init_args, None);
    
    // Install business logic canister with data_canister_id as init arg
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    let wasm = std::fs::read(WASM_PATH)
        .expect("Failed to read business logic WASM");
    let init_args = encode_args((data_canister_id.to_text(),)).unwrap();
    pic.install_canister(canister_id, wasm, init_args, None);
    
    // Authorize business logic canister to call data canister
    let result = pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        encode_args((canister_id.to_text(),)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to authorize business logic canister");
    
    (pic, canister_id, data_canister_id)
}

fn create_test_user(pic: &PocketIc, data_canister: Principal, phone: &str, pin_hash: &str, initial_balance: u64) -> String {
    use shared_types::{CreateUserData, UserType, FiatCurrency, User};
    
    let user_data = CreateUserData {
        user_type: UserType::User,
        preferred_currency: FiatCurrency::UGX,
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: None,
        phone_number: Some(phone.to_string()),
    };
    
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "create_user",
        encode_args((user_data,)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to create user");
    let user_result: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    let user = user_result.unwrap();
    let user_id = user.id;
    
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "setup_user_pin",
        encode_args((user_id.clone(), pin_hash.to_string(), "test_salt".to_string())).unwrap(),
    );
    assert!(result.is_ok(), "Failed to setup PIN");
    
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
fn test_fraud_detection_limits_loaded_from_config() {
    let (pic, canister_id, _data_canister) = setup();
    
    let result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_fraud_detection_limits",
        encode_args(()).unwrap(),
    );
    
    assert!(result.is_ok());
    let (max_amount, suspicious_threshold): (u64, u64) = candid::decode_args(&result.unwrap()).unwrap();
    
    assert_eq!(max_amount, 10_000_000);
    assert_eq!(suspicious_threshold, 5_000_000);
}

#[test]
fn test_fraud_blocks_transfer_exceeding_max_limit() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create user with 20M UGX (more than max limit)
    let sender_phone = "+256700111111";
    create_test_user(&pic, data_canister, sender_phone, "1234", 20_000_000);
    
    let recipient_phone = "+256700222222";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::anonymous();
    
    // Try to transfer 11M UGX (exceeds 10M limit)
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            11_000_000u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Transfer should be blocked by fraud detection");
    let error = response.unwrap_err();
    assert!(error.contains("blocked") || error.contains("limit") || error.contains("exceeds"), 
        "Error should mention blocking: {}", error);
}

#[test]
fn test_fraud_allows_transfer_at_max_limit() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create user with exactly 10M UGX
    let sender_phone = "+256700333333";
    create_test_user(&pic, data_canister, sender_phone, "1234", 10_000_000);
    
    let recipient_phone = "+256700444444";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::anonymous();
    
    // Transfer exactly 10M UGX (at limit, should succeed)
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            10_000_000u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Transfer at max limit should succeed: {:?}", response);
}

#[test]
fn test_fraud_flags_suspicious_amount() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create user with 10M UGX
    let sender_phone = "+256700555555";
    create_test_user(&pic, data_canister, sender_phone, "1234", 10_000_000);
    
    let recipient_phone = "+256700666666";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::anonymous();
    
    // Transfer 6M UGX (above 5M suspicious threshold but below 10M max)
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            6_000_000u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    // Should succeed but be flagged as suspicious (logged in canister)
    assert!(response.is_ok(), "Suspicious transfer should succeed: {:?}", response);
    let tx = response.unwrap();
    assert_eq!(tx.amount, 6_000_000);
}

#[test]
fn test_rate_limiting_blocks_excessive_transfers() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create users with enough balance for 11 transfers
    let sender_phone = "+256700777777";
    create_test_user(&pic, data_canister, sender_phone, "1234", 1_000_000);
    
    let recipient_phone = "+256700888888";
    create_test_user(&pic, data_canister, recipient_phone, "5678", 0);
    
    let user = Principal::anonymous();
    
    // Make 10 transfers (should all succeed - within rate limit)
    for i in 0..10 {
        let result = pic.update_call(
            canister_id,
            user,
            "send_money_to_phone",
            encode_args((
                sender_phone.to_string(),
                recipient_phone.to_string(),
                1_000u64,
                "UGX".to_string(),
                "1234".to_string(),
            )).unwrap(),
        );
        
        assert!(result.is_ok(), "Transfer {} should succeed", i + 1);
        let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
        assert!(response.is_ok(), "Transfer {} should succeed: {:?}", i + 1, response);
    }
    
    // 11th transfer should be rate limited (max 10 per 5 minutes)
    let result = pic.update_call(
        canister_id,
        user,
        "send_money_to_phone",
        encode_args((
            sender_phone.to_string(),
            recipient_phone.to_string(),
            1_000u64,
            "UGX".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "11th transfer should be rate limited");
    let error = response.unwrap_err();
    assert!(error.contains("rate") || error.contains("Too many") || error.contains("wait"), 
        "Error should mention rate limiting: {}", error);
}
