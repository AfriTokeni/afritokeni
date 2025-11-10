use candid::{encode_one, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;

#[test]
fn test_data_canister_create_user_direct() {
    let pic = PocketIc::new();
    
    // Load WASM
    let wasm_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent().unwrap()
        .parent().unwrap()
        .join("target/wasm32-unknown-unknown/release/data_canister.wasm");
    
    let wasm = std::fs::read(&wasm_path)
        .expect("data_canister WASM not found");
    
    // Install canister
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);
    let init_arg = encode_one((None::<String>, None::<String>)).unwrap();
    pic.install_canister(canister_id, wasm, init_arg, None);
    
    // Authorize ourselves
    let auth_arg = encode_one(Principal::anonymous().to_text()).unwrap();
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize");
    
    // Create user request
    let request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: None,
        phone_number: Some("+256700000000".to_string()),
    };
    
    let arg = encode_one(request).unwrap();
    
    // Call create_user
    let response = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        arg,
    ).expect("create_user call failed");
    
    // Decode response
    let result: Result<User, String> = decode_one(&response)
        .expect("Failed to decode response");
    
    assert!(result.is_ok(), "User creation should succeed: {:?}", result);
    let user = result.unwrap();
    assert_eq!(user.email, "test@example.com");
    println!("✅ Direct data_canister test PASSED! User: {:?}", user);
}
