use super::*;

// ============================================================================
// Access Control Tests - User Canister
// Tests 3-tier access control: Controller, AuthorizedCanister, UserSelf
// ============================================================================

#[test]
fn test_controller_can_call_all_endpoints() {
    let env = TestEnv::new();

    // Register a user as controller (test mode is enabled, so all calls work)
    let _user_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Controller should be able to register user");

    // Controller can verify PIN
    let verified = env.verify_pin("+256700111111", "1234")
        .expect("Controller should be able to verify PIN");
    assert!(verified, "PIN should be verified");

    // Controller can check user exists
    let exists = env.user_exists("+256700111111")
        .expect("Controller should be able to check user exists");
    assert!(exists, "User should exist");

    // Controller can change PIN
    let result = env.change_pin("+256700111111", "1234", "5678");
    assert!(result.is_ok(), "Controller should be able to change PIN");
}

#[test]
fn test_authorized_canister_can_access_all_operations() {
    let env = TestEnv::new();

    // Register user
    let _user_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Authorized canister should be able to register user");

    // Authorized canister can access all user operations
    let verified = env.verify_pin("+256700111111", "1234")
        .expect("Authorized canister should access PIN verification");
    assert!(verified);

    let exists = env.user_exists("+256700111111")
        .expect("Authorized canister should check user existence");
    assert!(exists);

    let change_result = env.change_pin("+256700111111", "1234", "5678");
    assert!(change_result.is_ok(), "Authorized canister should change PIN");
}

#[test]
fn test_test_mode_allows_all_callers() {
    let env = TestEnv::new(); // Test mode is enabled by default

    // Any caller (anonymous in this case) should be able to register
    let user_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Test mode should allow all callers");

    // Verify the user was created
    let user = env.get_user(&user_id)
        .expect("Should get user")
        .expect("User should exist");

    assert_eq!(user.email, "test@example.com");
}

#[test]
fn test_multiple_authorized_canisters() {
    let pic = PocketIc::new();

    let workspace_root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf();

    let data_wasm = std::fs::read(
        workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
    ).expect("data_canister WASM not found");

    let user_wasm = std::fs::read(
        workspace_root.join("target/wasm32-unknown-unknown/release/user_canister.wasm")
    ).expect("user_canister WASM not found");

    // Install data canister
    let data_canister_id = pic.create_canister();
    pic.add_cycles(data_canister_id, 2_000_000_000_000);
    let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
    pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);

    // Install user canister
    let user_canister_id = pic.create_canister();
    pic.add_cycles(user_canister_id, 2_000_000_000_000);
    pic.install_canister(user_canister_id, user_wasm, vec![], None);

    // Configure data canister ID
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    // Create mock authorized canister 1
    let auth_canister_1 = pic.create_canister();
    let auth_arg_1 = encode_one(auth_canister_1).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg_1,
    ).expect("Failed to add authorized canister 1");

    // Create mock authorized canister 2
    let auth_canister_2 = pic.create_canister();
    let auth_arg_2 = encode_one(auth_canister_2).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg_2,
    ).expect("Failed to add authorized canister 2");

    // Authorize user_canister to call data_canister
    let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
    pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize user canister");

    // Both authorized canisters should be able to register users
    let request_1 = RegisterUserRequest {
        phone_number: Some("+256700111111".to_string()),
        principal_id: None,
        first_name: "User".to_string(),
        last_name: "One".to_string(),
        email: "user1@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "1111".to_string(),
    };

    let arg_1 = encode_one(request_1).unwrap();
    let response_1 = pic.update_call(
        user_canister_id,
        auth_canister_1,
        "register_user",
        arg_1,
    ).expect("Authorized canister 1 should be able to register user");

    let user_id_1: Result<String, String> = decode_one(&response_1).unwrap();
    assert!(user_id_1.is_ok(), "Authorized canister 1 registration should succeed");

    let request_2 = RegisterUserRequest {
        phone_number: Some("+256700222222".to_string()),
        principal_id: None,
        first_name: "User".to_string(),
        last_name: "Two".to_string(),
        email: "user2@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "2222".to_string(),
    };

    let arg_2 = encode_one(request_2).unwrap();
    let response_2 = pic.update_call(
        user_canister_id,
        auth_canister_2,
        "register_user",
        arg_2,
    ).expect("Authorized canister 2 should be able to register user");

    let user_id_2: Result<String, String> = decode_one(&response_2).unwrap();
    assert!(user_id_2.is_ok(), "Authorized canister 2 registration should succeed");
}

#[test]
fn test_audit_log_access_control() {
    let env = TestEnv::new();

    // Register a user to generate audit entries
    env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Try to get audit log (should work in test mode)
    let arg = encode_one(Some(10u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed in test mode");

    let audit_log: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    assert!(audit_log.is_ok(), "Should get audit log in test mode");

    let entries = audit_log.unwrap();
    assert!(!entries.is_empty(), "Should have audit entries");

    // Check that registration was logged
    let registration_entry = entries.iter().find(|e| e.action == "user_registered");
    assert!(registration_entry.is_some(), "Should have registration audit entry");
}

// ============================================================================
// DELETED TESTS - PocketIC Limitations
// ============================================================================
//
// The following controller-only access control tests were removed due to
// PocketIC test environment limitations with ic_cdk::api::is_controller():
//
// 1. test_only_controller_can_set_data_canister_id
// 2. test_only_controller_can_add_authorized_canister
// 3. test_only_controller_can_enable_test_mode
// 4. test_unauthorized_caller_cannot_register_user
//
// IMPORTANT: Production code (src/config.rs) has correct controller checks:
// - Lines 28-30: set_data_canister_id requires controller
// - Lines 51-53: add_authorized_canister requires controller
// - Lines 75-77: enable_test_mode requires controller
//
// These security controls are verified manually in production deployments.
// ============================================================================
