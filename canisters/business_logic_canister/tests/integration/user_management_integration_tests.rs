use candid::{encode_args, decode_one, Principal};
use pocket_ic::PocketIc;

const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/business_logic_canister.wasm";
const DATA_WASM: &str = "../../target/wasm32-unknown-unknown/release/data_canister.wasm";

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
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
        .expect("Failed to read business logic WASM");
    pic.install_canister(canister_id, wasm, vec![], None);
    
    let data_canister_id = pic.create_canister();
    pic.add_cycles(data_canister_id, 2_000_000_000_000);
    let data_wasm = std::fs::read(DATA_WASM)
        .expect("Failed to read data canister WASM");
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

#[test]
fn test_register_user_with_phone() {
    let (pic, canister_id, data_canister) = setup();
    
    let user_principal = Principal::from_text("aaaaa-aa").unwrap();
    let phone = "+256700123456";
    let pin = "1234";
    let name = "John Doe";
    
    let result = pic.update_call(
        canister_id,
        user_principal,
        "register_user",
        encode_args((
            Some(phone.to_string()),
            Some(user_principal),
            name.to_string(),
            pin.to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok(), "Register user call failed");
    let response: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "User registration should succeed: {:?}", response);
    let user = response.unwrap();
    
    assert_eq!(user.phone, Some(phone.to_string()));
    assert_eq!(user.principal, Some(user_principal));
    assert_eq!(user.name, name);
    assert!(!user.id.is_empty());
    
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
    
    let user_principal = Principal::from_text("aaaaa-aa").unwrap();
    
    // Phone without + prefix
    let result = pic.update_call(
        canister_id,
        user_principal,
        "register_user",
        encode_args((
            Some("256700123456".to_string()), // Missing +
            Some(user_principal),
            "John Doe".to_string(),
            "1234".to_string(),
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_err(), "Should fail with invalid phone format");
    let error = response.unwrap_err();
    assert!(error.contains("phone") || error.contains("format") || error.contains("+"), 
        "Error should mention phone format: {}", error);
}

#[test]
fn test_register_user_fails_with_invalid_pin() {
    let (pic, canister_id, _data_canister) = setup();
    
    let user_principal = Principal::from_text("aaaaa-aa").unwrap();
    
    // PIN with only 3 digits
    let result = pic.update_call(
        canister_id,
        user_principal,
        "register_user",
        encode_args((
            Some("+256700123456".to_string()),
            Some(user_principal),
            "John Doe".to_string(),
            "123".to_string(), // Too short
        )).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    
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
    let user_id = format!("user_{}", phone);
    
    // Create user manually
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
    assert!(result.is_ok());
    
    // Store PIN hash
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "store_pin_hash",
        encode_args((user_id.clone(), pin.to_string())).unwrap(),
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
    let user_id = format!("user_{}", phone);
    
    // Create user
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
    assert!(result.is_ok());
    
    // Store correct PIN
    let result = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "store_pin_hash",
        encode_args((user_id.clone(), correct_pin.to_string())).unwrap(),
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
    let user_id = "user_test_123";
    let user = User {
        id: user_id.to_string(),
        phone: None,
        principal: Some(Principal::from_text("aaaaa-aa").unwrap()),
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
    assert!(result.is_ok());
    
    // Link phone
    let new_phone = "+256700555666";
    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "link_phone",
        encode_args((user_id.to_string(), new_phone.to_string())).unwrap(),
    );
    
    assert!(result.is_ok());
    let response: Result<User, String> = decode_one(&result.unwrap()).unwrap();
    
    assert!(response.is_ok(), "Phone linking should succeed: {:?}", response);
    let updated_user = response.unwrap();
    assert_eq!(updated_user.phone, Some(new_phone.to_string()));
}
