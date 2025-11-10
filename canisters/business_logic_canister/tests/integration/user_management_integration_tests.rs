use candid::{encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::{RegisterUserRequest, User};

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

#[test]
fn test_register_user_with_phone() {
    let (pic, canister_id, data_canister) = setup();
    
    let user_principal = Principal::anonymous();
    let phone = "+256700123456";
    let pin = "1234";
    let name = "John Doe";
    
    let request = RegisterUserRequest {
        phone_number: Some(phone.to_string()),
        principal_id: Some(user_principal.to_text()),
        first_name: name.to_string(),
        last_name: "Doe".to_string(),
        email: "test@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: pin.to_string(),
    };
    
    let result = pic.update_call(
        canister_id,
        user_principal,
        "register_user",
        encode_args((request,)).unwrap(),
    );
    
    assert!(result.is_ok(), "Register user call failed");
    let response: Result<String, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "User registration should succeed: {:?}", response);
    let user_id = response.unwrap();
    
    assert!(!user_id.is_empty(), "User ID should not be empty");
    
    // Verify user exists in data canister
    let get_result = pic.query_call(
        data_canister,
        Principal::anonymous(),
        "get_user_by_phone",
        encode_args((phone.to_string(),)).unwrap(),
    );
    assert!(get_result.is_ok());
    let stored_user: Option<User> = decode_one(&get_result.unwrap()).unwrap();
    assert!(stored_user.is_some(), "User should be stored in data canister");
}

#[test]
fn test_register_user_fails_with_invalid_phone() {
    let (pic, canister_id, _data_canister) = setup();
    
    let user_principal = Principal::anonymous();
    
    // Phone without + prefix
    let request = RegisterUserRequest {
        phone_number: Some("256700123456".to_string()), // Missing +
        principal_id: Some(user_principal.to_text()),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "test@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };
    
    let result = pic.update_call(
        canister_id,
        user_principal,
        "register_user",
        encode_args((request,)).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<String, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Should fail with invalid phone format");
    let error = response.unwrap_err();
    assert!(error.contains("phone") || error.contains("format") || error.contains("+"), 
        "Error should mention phone format: {}", error);
}

#[test]
fn test_register_user_fails_with_invalid_pin() {
    let (pic, canister_id, _data_canister) = setup();
    
    let user_principal = Principal::anonymous();
    
    // PIN with only 3 digits
    let request = RegisterUserRequest {
        phone_number: Some("+256700123456".to_string()),
        principal_id: Some(user_principal.to_text()),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "test@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "123".to_string(), // Too short
    };
    
    let result = pic.update_call(
        canister_id,
        user_principal,
        "register_user",
        encode_args((request,)).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<String, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Should fail with invalid PIN");
    let error = response.unwrap_err();
    assert!(error.contains("PIN") || error.contains("4 digits"), 
        "Error should mention PIN format: {}", error);
}

#[test]
fn test_verify_pin_success() {
    let (pic, canister_id, data_canister) = setup();
    
    let phone = "+256700987654";
    let pin = "5678";
    
    // Create user
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
    assert!(result.is_ok());
    let user_result: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    let user = user_result.unwrap();
    let user_id = user.id;
    
    // Setup PIN
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "setup_user_pin",
        encode_args((user_id.clone(), pin.to_string(), "test_salt".to_string())).unwrap(),
    );
    assert!(result.is_ok());
    
    // Verify PIN
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "verify_pin",
        encode_args((phone.to_string(), pin.to_string())).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<bool, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "PIN verification should succeed: {:?}", response);
    assert!(response.unwrap(), "PIN should be valid");
}

#[test]
fn test_verify_pin_fails_with_wrong_pin() {
    let (pic, canister_id, data_canister) = setup();
    
    let phone = "+256700111222";
    let correct_pin = "1234";
    let wrong_pin = "9999";
    
    // Create user
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
    assert!(result.is_ok());
    let user_result: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    let user = user_result.unwrap();
    let user_id = user.id;
    
    // Setup PIN
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "setup_user_pin",
        encode_args((user_id.clone(), correct_pin.to_string(), "test_salt".to_string())).unwrap(),
    );
    assert!(result.is_ok());
    
    // Try wrong PIN
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "verify_pin",
        encode_args((phone.to_string(), wrong_pin.to_string())).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<bool, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "PIN verification call should succeed");
    assert!(!response.unwrap(), "Wrong PIN should return false");
}

#[test]
fn test_link_phone_to_existing_user() {
    let (pic, canister_id, data_canister) = setup();
    
    // Create user without phone
    use shared_types::{CreateUserData, UserType, FiatCurrency, User};
    
    let user_data = CreateUserData {
        user_type: UserType::User,
        preferred_currency: FiatCurrency::UGX,
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: Some("aaaaa-aa".to_string()),
        phone_number: None,
    };
    
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "create_user",
        encode_args((user_data,)).unwrap(),
    );
    assert!(result.is_ok());
    let user_result: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    let user = user_result.unwrap();
    let user_id = user.id;
    
    // Link phone (need to setup PIN first)
    let pin = "1234";
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "setup_user_pin",
        encode_args((user_id.to_string(), pin.to_string(), "test_salt".to_string())).unwrap(),
    );
    assert!(result.is_ok());
    
    let new_phone = "+256700555666";
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "link_phone_to_account",
        encode_args(("aaaaa-aa".to_string(), new_phone.to_string(), pin.to_string())).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<(), String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Phone linking should succeed: {:?}", response);
}
