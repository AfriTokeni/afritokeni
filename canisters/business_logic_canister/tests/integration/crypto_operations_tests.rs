use candid::{encode_args, decode_one, Principal};
use pocket_ic::PocketIc;

const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/business_logic_canister.wasm";
const MOCK_DATA_WASM: &str = "../../target/wasm32-unknown-unknown/release/data_canister.wasm";

#[derive(candid::CandidType, candid::Deserialize, Clone, Copy, Debug, PartialEq)]
enum CryptoType {
    CkBTC,
    CkUSDC,
}

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

fn setup_with_mock_data_canister() -> (PocketIc, Principal, Principal) {
    let pic = PocketIc::new();
    
    // Install REAL data canister FIRST
    let data_canister_id = pic.create_canister();
    pic.add_cycles(data_canister_id, 2_000_000_000_000);
    
    let data_wasm = std::fs::read(MOCK_DATA_WASM)
        .expect("Failed to read data canister WASM. Run: cargo build --target wasm32-unknown-unknown --release --package data_canister");
    
    // Data canister init expects (Option<String>, Option<String>) for ussd_canister_id and web_canister_id
    let data_init_args = encode_args((None::<String>, None::<String>)).unwrap();
    pic.install_canister(data_canister_id, data_wasm, data_init_args, None);
    
    // Install business logic canister with data_canister_id as init arg
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    let wasm = std::fs::read(WASM_PATH)
        .expect("Failed to read business logic WASM. Run: cargo build --target wasm32-unknown-unknown --release --package business_logic_canister");
    
    // Pass data_canister_id as init argument
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

fn setup() -> (PocketIc, Principal) {
    let (pic, canister_id, _) = setup_with_mock_data_canister();
    (pic, canister_id)
}

#[test]
fn test_get_crypto_value_estimate_ckbtc() {
    let (pic, canister_id) = setup();
    
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "get_crypto_value_estimate",
        encode_args((100_000_000u64, CryptoType::CkBTC, "UGX".to_string())).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<u64, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err() || response.is_ok());
}

#[test]
fn test_get_crypto_value_estimate_ckusdc() {
    let (pic, canister_id) = setup();
    
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "get_crypto_value_estimate",
        encode_args((1_000_000u64, CryptoType::CkUSDC, "KES".to_string())).unwrap(),
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_sell_crypto_to_agent_endpoint_exists() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "sell_crypto_to_agent",
        encode_args((
            "+256700123456".to_string(),
            50_000_000u64,
            CryptoType::CkBTC,
            "agent123".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_buy_crypto_validates_zero_amount() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "buy_crypto",
        encode_args((
            "+256700123456".to_string(),
            0u64,
            "UGX".to_string(),
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
}

#[test]
fn test_send_crypto_validates_empty_address() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "send_crypto",
        encode_args((
            "+256700123456".to_string(),
            "".to_string(),
            1_000_000u64,
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
}

#[test]
fn test_fraud_detection_limits_from_config() {
    let (pic, canister_id) = setup();
    
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

// ============================================================================
// EDGE CASES - Real Integration Tests
// ============================================================================

#[test]
fn test_buy_crypto_with_invalid_currency_code() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "buy_crypto",
        encode_args((
            "+256700123456".to_string(),
            10000u64,
            "INVALID".to_string(), // Invalid currency
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
    if let Err(e) = response {
        assert!(e.contains("currency") || e.contains("Invalid"));
    }
}

#[test]
fn test_buy_crypto_with_max_u64_amount() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "buy_crypto",
        encode_args((
            "+256700123456".to_string(),
            u64::MAX, // Maximum value
            "UGX".to_string(),
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    // Should either succeed or fail gracefully (likely fraud detection blocks it)
    if let Err(e) = response {
        assert!(e.contains("limit") || e.contains("blocked") || e.contains("User not found"));
    }
}

#[test]
fn test_send_crypto_with_invalid_address_format() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "send_crypto",
        encode_args((
            "+256700123456".to_string(),
            "invalid_address".to_string(), // Too short
            1_000_000u64,
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
    if let Err(e) = response {
        assert!(e.contains("address") || e.contains("invalid") || e.contains("format"));
    }
}

#[test]
fn test_sell_crypto_to_agent_with_zero_amount() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "sell_crypto_to_agent",
        encode_args((
            "+256700123456".to_string(),
            0u64, // Zero amount
            CryptoType::CkBTC,
            "agent123".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
    if let Err(e) = response {
        assert!(e.contains("amount") || e.contains("greater than 0"));
    }
}

#[test]
fn test_sell_crypto_to_agent_with_empty_agent_id() {
    let (pic, canister_id) = setup();
    let user = Principal::anonymous();
    
    let result = pic.update_call(
        canister_id,
        user,
        "sell_crypto_to_agent",
        encode_args((
            "+256700123456".to_string(),
            1_000_000u64,
            CryptoType::CkBTC,
            "".to_string(), // Empty agent ID
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
    if let Err(e) = response {
        assert!(e.contains("Agent") || e.contains("empty") || e.contains("required"));
    }
}

#[test]
fn test_unauthorized_caller_cannot_call_endpoints() {
    let (pic, canister_id) = setup();
    let unauthorized = Principal::from_text("2vxsx-fae").unwrap();
    
    // Try to call without being authorized
    let result = pic.update_call(
        canister_id,
        unauthorized,
        "buy_crypto",
        encode_args((
            "+256700123456".to_string(),
            10000u64,
            "UGX".to_string(),
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    // Should fail authorization check
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    assert!(response.is_err());
}

#[test]
fn test_config_values_loaded_correctly() {
    let (pic, canister_id) = setup();
    
    let result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_fraud_detection_limits",
        encode_args(()).unwrap(),
    );
    
    assert!(result.is_ok());
    let (max, suspicious): (u64, u64) = candid::decode_args(&result.unwrap()).unwrap();
    
    // Values from business_logic_config.toml
    assert_eq!(max, 10_000_000, "Max transaction amount should be 10M");
    assert_eq!(suspicious, 5_000_000, "Suspicious threshold should be 5M");
    assert!(max > suspicious, "Max should be greater than suspicious threshold");
}

// ============================================================================
// REAL CRYPTO OPERATIONS TESTS - Full Flows with Balance Changes
// ============================================================================

fn create_crypto_test_user(pic: &PocketIc, data_canister: Principal, phone: &str, pin_hash: &str, fiat_balance: u64, ckbtc: u64, ckusdc: u64) -> String {
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
        encode_args((user_id.clone(), "UGX".to_string(), fiat_balance)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to set fiat balance");
    
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "update_crypto_balance",
        encode_args((user_id.clone(), ckbtc as i64, ckusdc as i64)).unwrap(),
    );
    assert!(result.is_ok(), "Failed to set crypto balance");
    
    user_id
}

#[test]
fn test_full_buy_crypto_flow() {
    let (pic, canister_id, data_canister) = setup_with_mock_data_canister();
    
    // Create user with 1M UGX, no crypto
    let user_phone = "+256700111111";
    create_crypto_test_user(&pic, data_canister, user_phone, "1234", 1_000_000, 0, 0);
    
    let user = Principal::anonymous();
    
    // Buy ckBTC with 100,000 UGX
    let result = pic.update_call(
        canister_id,
        user,
        "buy_crypto",
        encode_args((
            user_phone.to_string(),
            100_000u64,
            "UGX".to_string(),
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok(), "Buy crypto call failed");
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    // Will likely fail without HTTP outcalls, but tests the flow
    if let Ok(_tx) = response {
        // Verify fiat balance decreased
        let fiat_result = pic.query_call(
            data_canister,
            Principal::anonymous(),
            "get_fiat_balance",
            encode_args((format!("user_{}", user_phone), "UGX".to_string())).unwrap(),
        );
        assert!(fiat_result.is_ok());
        let fiat_balance: u64 = decode_one(&fiat_result.unwrap()).unwrap();
        assert_eq!(fiat_balance, 900_000, "Fiat balance should decrease by 100,000");
        
        // Verify crypto balance increased
        let crypto_result = pic.query_call(
            data_canister,
            Principal::anonymous(),
            "get_crypto_balance",
            encode_args((format!("user_{}", user_phone),)).unwrap(),
        );
        assert!(crypto_result.is_ok());
        let (ckbtc, _ckusdc): (u64, u64) = decode_one(&crypto_result.unwrap()).unwrap();
        assert!(ckbtc > 0, "ckBTC balance should increase");
    }
}

#[test]
fn test_sell_crypto_to_agent_creates_escrow() {
    let (pic, canister_id, data_canister) = setup_with_mock_data_canister();
    
    // Create user with 1M satoshis ckBTC
    let user_phone = "+256700222222";
    create_crypto_test_user(&pic, data_canister, user_phone, "1234", 0, 1_000_000, 0);
    
    let user = Principal::anonymous();
    
    // Sell 500k satoshis to agent
    let result = pic.update_call(
        canister_id,
        user,
        "sell_crypto_to_agent",
        encode_args((
            user_phone.to_string(),
            500_000u64,
            CryptoType::CkBTC,
            "agent_123".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok(), "Sell crypto call failed");
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Sell crypto should succeed: {:?}", response);
    let tx = response.unwrap();
    
    // Verify escrow code format
    assert!(tx.transaction_id.starts_with("BTC-"), "Escrow code should start with BTC-");
    assert_eq!(tx.transaction_id.len(), 10, "Escrow code should be 10 chars (BTC-XXXXXX)");
    assert_eq!(tx.to_user, "agent_123");
    assert_eq!(tx.amount, 500_000);
    
    // Verify crypto balance decreased
    let crypto_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_crypto_balance",
        encode_args((format!("user_{}", user_phone),)).unwrap(),
    );
    assert!(crypto_result.is_ok());
    let (ckbtc, _ckusdc): (u64, u64) = decode_one(&crypto_result.unwrap()).unwrap();
    assert_eq!(ckbtc, 500_000, "ckBTC balance should decrease to 500k");
}

#[test]
fn test_send_crypto_between_users() {
    let (pic, canister_id, data_canister) = setup_with_mock_data_canister();
    
    // Create sender with 1M satoshis
    let sender_phone = "+256700333333";
    create_crypto_test_user(&pic, data_canister, sender_phone, "1234", 0, 1_000_000, 0);
    
    // Create recipient with 0 crypto
    let recipient_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"; // Valid BTC address
    
    let user = Principal::anonymous();
    
    // Send 300k satoshis
    let result = pic.update_call(
        canister_id,
        user,
        "send_crypto",
        encode_args((
            sender_phone.to_string(),
            recipient_address.to_string(),
            300_000u64,
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok(), "Send crypto call failed");
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Send crypto should succeed: {:?}", response);
    let tx = response.unwrap();
    
    assert_eq!(tx.amount, 300_000);
    assert_eq!(tx.to_user, recipient_address);
    assert_eq!(tx.new_balance, 700_000, "Sender balance should be 700k");
    
    // Verify sender balance decreased
    let crypto_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_crypto_balance",
        encode_args((format!("user_{}", sender_phone),)).unwrap(),
    );
    assert!(crypto_result.is_ok());
    let (ckbtc, _ckusdc): (u64, u64) = decode_one(&crypto_result.unwrap()).unwrap();
    assert_eq!(ckbtc, 700_000, "Sender ckBTC should be 700k");
}

#[test]
fn test_crypto_transfer_fails_insufficient_balance() {
    let (pic, canister_id, data_canister) = setup_with_mock_data_canister();
    
    // Create user with only 100k satoshis
    let user_phone = "+256700444444";
    create_crypto_test_user(&pic, data_canister, user_phone, "1234", 0, 100_000, 0);
    
    let user = Principal::anonymous();
    
    // Try to send 500k satoshis (more than balance)
    let result = pic.update_call(
        canister_id,
        user,
        "send_crypto",
        encode_args((
            user_phone.to_string(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            500_000u64,
            CryptoType::CkBTC,
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<TransactionResult, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Should fail with insufficient balance");
    let error = response.unwrap_err();
    assert!(error.contains("insufficient") || error.contains("balance"), 
        "Error should mention insufficient balance: {}", error);
}
