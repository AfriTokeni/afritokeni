use super::*;

// ============================================================================
// Canister Upgrade Persistence Tests
// Tests that state is preserved across upgrades using stable storage
// ============================================================================

#[test]
fn test_configuration_persists_across_upgrade() {
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
    pic.install_canister(user_canister_id, user_wasm.clone(), vec![], None);

    // Configure data canister ID
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    // Add authorized canister
    let auth_canister = pic.create_canister();
    let auth_arg = encode_one(auth_canister).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to add authorized canister");

    // Enable test mode
    let test_mode_arg = encode_one(()).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "enable_test_mode",
        test_mode_arg,
    ).expect("Failed to enable test mode");

    // Upgrade the canister
    pic.upgrade_canister(user_canister_id, user_wasm, vec![], None)
        .expect("Upgrade should succeed");

    // Verify configuration persisted - try to use the canister
    // If configuration didn't persist, authorized canister check would fail
    pic.update_call(
        user_canister_id,
        auth_canister,
        "user_exists",
        encode_one("test".to_string()).unwrap(),
    ).expect("Authorized canister should still work after upgrade");
}

#[test]
fn test_audit_log_persists_across_upgrade() {
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
    pic.install_canister(user_canister_id, user_wasm.clone(), vec![], None);

    // Configure
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    let test_mode_arg = encode_one(()).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "enable_test_mode",
        test_mode_arg,
    ).expect("Failed to enable test mode");

    // Authorize user_canister to call data_canister
    let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
    pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize user canister");

    // Register a user to create audit entries
    let request = RegisterUserRequest {
        phone_number: Some("+256700123456".to_string()),
        principal_id: None,
        first_name: "Before".to_string(),
        last_name: "Upgrade".to_string(),
        email: "before@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };

    let reg_arg = encode_one(request).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "register_user",
        reg_arg,
    ).expect("Registration should succeed");

    // Get audit log before upgrade
    let audit_arg = encode_one(Some(100u64)).unwrap();
    let response_before = pic.query_call(
        user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        audit_arg.clone(),
    ).expect("get_audit_log should succeed");

    let audit_before: Result<Vec<AuditEntry>, String> = decode_one(&response_before).unwrap();
    let entries_before = audit_before.expect("Should get audit log");
    let count_before = entries_before.len();

    assert!(count_before > 0, "Should have audit entries before upgrade");

    // Verify initialization entry exists
    let init_entry = entries_before.iter().find(|e| e.action == "canister_initialized");
    assert!(init_entry.is_some(), "Should have initialization entry");

    // Verify registration entry exists
    let reg_entry = entries_before.iter().find(|e| e.action == "user_registered");
    assert!(reg_entry.is_some(), "Should have registration entry");

    // Upgrade the canister
    pic.upgrade_canister(user_canister_id, user_wasm, vec![], None)
        .expect("Upgrade should succeed");

    // Get audit log after upgrade
    let response_after = pic.query_call(
        user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        audit_arg,
    ).expect("get_audit_log should succeed after upgrade");

    let audit_after: Result<Vec<AuditEntry>, String> = decode_one(&response_after).unwrap();
    let entries_after = audit_after.expect("Should get audit log after upgrade");

    // Should have all previous entries plus upgrade entry
    assert!(
        entries_after.len() >= count_before,
        "Should have at least as many entries after upgrade (before: {}, after: {})",
        count_before,
        entries_after.len()
    );

    // Verify upgrade entry was added
    let upgrade_entry = entries_after.iter().find(|e| e.action == "canister_upgraded");
    assert!(upgrade_entry.is_some(), "Should have upgrade entry");

    // Verify old entries still exist
    let init_after = entries_after.iter().find(|e| e.action == "canister_initialized");
    assert!(init_after.is_some(), "Initialization entry should persist");

    let reg_after = entries_after.iter().find(|e| e.action == "user_registered");
    assert!(reg_after.is_some(), "Registration entry should persist");
}

#[test]
fn test_multiple_upgrades_preserve_state() {
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
    pic.install_canister(user_canister_id, user_wasm.clone(), vec![], None);

    // Configure
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    let test_mode_arg = encode_one(()).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "enable_test_mode",
        test_mode_arg,
    ).expect("Failed to enable test mode");

    // Authorize user_canister to call data_canister
    let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
    pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize user canister");

    // Perform 3 upgrades
    for i in 1..=3 {
        // Register a user before each upgrade
        let request = RegisterUserRequest {
            phone_number: Some(format!("+25670012345{}", i)),
            principal_id: None,
            first_name: format!("User{}", i),
            last_name: "Test".to_string(),
            email: format!("user{}@example.com", i),
            preferred_currency: "UGX".to_string(),
            pin: "1234".to_string(),
        };

        let reg_arg = encode_one(request).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "register_user",
            reg_arg,
        ).expect("Registration should succeed");

        // Upgrade
        pic.upgrade_canister(user_canister_id, user_wasm.clone(), vec![], None)
            .expect(&format!("Upgrade {} should succeed", i));
    }

    // Get audit log after all upgrades
    let audit_arg = encode_one(Some(100u64)).unwrap();
    let response = pic.query_call(
        user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        audit_arg,
    ).expect("get_audit_log should succeed");

    let audit: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit.expect("Should get audit log");

    // Should have:
    // - 1 initialization
    // - 3 registrations
    // - 3 upgrades
    // - Plus any other entries (PIN verifications, etc.)
    let init_count = entries.iter().filter(|e| e.action == "canister_initialized").count();
    let reg_count = entries.iter().filter(|e| e.action == "user_registered").count();
    let upgrade_count = entries.iter().filter(|e| e.action == "canister_upgraded").count();

    assert_eq!(init_count, 1, "Should have 1 initialization");
    assert_eq!(reg_count, 3, "Should have 3 registrations");
    assert_eq!(upgrade_count, 3, "Should have 3 upgrades");
}

#[test]
fn test_test_mode_persists_across_upgrade() {
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
    pic.install_canister(user_canister_id, user_wasm.clone(), vec![], None);

    // Configure
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    // Authorize user_canister to call data_canister
    let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
    pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize user canister");

    // Enable test mode
    let test_mode_arg = encode_one(()).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "enable_test_mode",
        test_mode_arg,
    ).expect("Failed to enable test mode");

    // Verify test mode works (any caller can call)
    let exists_arg = encode_one("test".to_string()).unwrap();
    let result_before = pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "user_exists",
        exists_arg.clone(),
    );
    assert!(result_before.is_ok(), "Test mode should allow anonymous caller before upgrade");

    // Upgrade
    pic.upgrade_canister(user_canister_id, user_wasm, vec![], None)
        .expect("Upgrade should succeed");

    // Verify test mode still works after upgrade
    let result_after = pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "user_exists",
        exists_arg,
    );
    assert!(result_after.is_ok(), "Test mode should still allow anonymous caller after upgrade");
}

#[test]
fn test_authorized_canisters_persist_across_upgrade() {
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
    pic.install_canister(user_canister_id, user_wasm.clone(), vec![], None);

    // Configure
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    // Add multiple authorized canisters
    let auth1 = pic.create_canister();
    let auth2 = pic.create_canister();
    let auth3 = pic.create_canister();

    for auth_canister in &[auth1, auth2, auth3] {
        let auth_arg = encode_one(*auth_canister).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to add authorized canister");
    }

    // Authorize user_canister to call data_canister
    let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
    pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize user canister");

    // Verify all authorized canisters can call before upgrade
    for auth_canister in &[auth1, auth2, auth3] {
        let exists_arg = encode_one("test".to_string()).unwrap();
        let result = pic.update_call(
            user_canister_id,
            *auth_canister,
            "user_exists",
            exists_arg,
        );
        assert!(result.is_ok(), "Authorized canister should be able to call before upgrade");
    }

    // Upgrade
    pic.upgrade_canister(user_canister_id, user_wasm, vec![], None)
        .expect("Upgrade should succeed");

    // Verify all authorized canisters can still call after upgrade
    for auth_canister in &[auth1, auth2, auth3] {
        let exists_arg = encode_one("test".to_string()).unwrap();
        let result = pic.update_call(
            user_canister_id,
            *auth_canister,
            "user_exists",
            exists_arg,
        );
        assert!(result.is_ok(), "Authorized canister should still be able to call after upgrade");
    }
}

#[test]
fn test_data_canister_id_persists_across_upgrade() {
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
    pic.install_canister(user_canister_id, user_wasm.clone(), vec![], None);

    // Configure data canister ID
    let config_arg = encode_args((data_canister_id,)).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "set_data_canister_id",
        config_arg,
    ).expect("Failed to configure data canister ID");

    // Enable test mode
    let test_mode_arg = encode_one(()).unwrap();
    pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "enable_test_mode",
        test_mode_arg,
    ).expect("Failed to enable test mode");

    // Authorize user_canister to call data_canister
    let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
    pic.update_call(
        data_canister_id,
        Principal::anonymous(),
        "add_authorized_canister",
        auth_arg,
    ).expect("Failed to authorize user canister");

    // Register a user (requires data_canister_id to be set)
    let request = RegisterUserRequest {
        phone_number: Some("+256700123456".to_string()),
        principal_id: None,
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };

    let reg_arg = encode_one(request).unwrap();
    let result_before = pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "register_user",
        reg_arg.clone(),
    );
    assert!(result_before.is_ok(), "Registration should work before upgrade");

    // Upgrade
    pic.upgrade_canister(user_canister_id, user_wasm, vec![], None)
        .expect("Upgrade should succeed");

    // Try to register another user (should still work if data_canister_id persisted)
    let request2 = RegisterUserRequest {
        phone_number: Some("+256700123457".to_string()),
        principal_id: None,
        first_name: "Test".to_string(),
        last_name: "User2".to_string(),
        email: "test2@example.com".to_string(),
        preferred_currency: "UGX".to_string(),
        pin: "1234".to_string(),
    };

    let reg_arg2 = encode_one(request2).unwrap();
    let result_after = pic.update_call(
        user_canister_id,
        Principal::anonymous(),
        "register_user",
        reg_arg2,
    );
    assert!(result_after.is_ok(), "Registration should still work after upgrade (data_canister_id persisted)");
}
