use super::*;

// ============================================================================
// Audit Log Generation Tests
// Tests that all operations are properly logged with correct details
// ============================================================================

#[test]
fn test_user_registration_logged() {
    let env = TestEnv::new();

    // Register a user (phone number deliberately doesn't contain PIN "1234")
    let user_id = env.register_user(
        Some("+256700987650".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find registration entry
    let reg_entry = entries.iter().find(|e| {
        e.action == "user_registered" &&
        e.user_id.as_ref() == Some(&user_id)
    });

    assert!(reg_entry.is_some(), "Registration should be logged");

    let entry = reg_entry.unwrap();
    assert_eq!(entry.success, true, "Registration should be marked as success");
    assert!(entry.details.contains("John"), "Details should contain first name");
    assert!(entry.details.contains("Doe"), "Details should contain last name");
    assert!(entry.details.contains("+256700987650"), "Details should contain phone");
    assert!(entry.details.contains("UGX"), "Details should contain currency");

    // Should NOT contain PIN
    assert!(!entry.details.contains("1234"), "Details should NOT contain PIN");
}

#[test]
fn test_failed_registration_logged() {
    let env = TestEnv::new();

    // Try to register with invalid data
    let result = env.register_user(
        None, // Missing phone and principal
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    );

    assert!(result.is_err(), "Registration should fail");

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find failed registration entry
    let failed_entry = entries.iter().find(|e| {
        e.action == "user_registration_failed"
    });

    assert!(failed_entry.is_some(), "Failed registration should be logged");

    let entry = failed_entry.unwrap();
    assert_eq!(entry.success, false, "Failed operation should be marked as failure");
    assert!(entry.details.contains("Validation error"), "Details should mention validation");
}

#[test]
fn test_pin_verification_logged() {
    let env = TestEnv::new();

    // Register user (phone number deliberately doesn't contain PIN "1234")
    env.register_user(
        Some("+256700987650".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Verify PIN (correct)
    let verified = env.verify_pin("+256700987650", "1234")
        .expect("verify_pin should succeed");
    assert!(verified);

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find PIN verification entry
    let verify_entry = entries.iter().find(|e| e.action == "pin_verified");

    assert!(verify_entry.is_some(), "PIN verification should be logged");

    let entry = verify_entry.unwrap();
    assert_eq!(entry.success, true, "Verification should be marked as success");
    assert!(entry.user_id.is_some(), "Should have user_id");

    // Should NOT contain actual PIN
    assert!(!entry.details.contains("1234"), "Details should NOT contain PIN");
}

#[test]
fn test_failed_pin_verification_logged() {
    let env = TestEnv::new();

    // Register user (phone number deliberately doesn't contain PIN "1234" or "9999")
    env.register_user(
        Some("+256700876500".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Try wrong PIN
    let verified = env.verify_pin("+256700876500", "9999")
        .expect("verify_pin should succeed");
    assert!(!verified);

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find failed verification entry
    let failed_entry = entries.iter().find(|e| e.action == "pin_verification_failed");

    assert!(failed_entry.is_some(), "Failed verification should be logged");

    let entry = failed_entry.unwrap();
    assert_eq!(entry.success, false, "Failed verification should be marked as failure");

    // Should NOT contain actual PIN
    assert!(!entry.details.contains("9999"), "Details should NOT contain attempted PIN");
}

#[test]
fn test_pin_change_logged() {
    let env = TestEnv::new();

    // Register user (phone number deliberately doesn't contain PINs "1234" or "5678")
    env.register_user(
        Some("+256700909000".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Change PIN
    env.change_pin("+256700909000", "1234", "5678")
        .expect("PIN change should succeed");

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find PIN change entry
    let change_entry = entries.iter().find(|e| e.action == "pin_changed");

    assert!(change_entry.is_some(), "PIN change should be logged");

    let entry = change_entry.unwrap();
    assert_eq!(entry.success, true, "PIN change should be marked as success");
    assert!(entry.user_id.is_some(), "Should have user_id");

    // Should NOT contain old or new PIN
    assert!(!entry.details.contains("1234"), "Details should NOT contain old PIN");
    assert!(!entry.details.contains("5678"), "Details should NOT contain new PIN");
}

#[test]
fn test_phone_linking_logged() {
    let env = TestEnv::new();

    let principal = "aaaaa-aa".to_string();

    // Register with principal only
    env.register_user(
        None,
        Some(principal.clone()),
        "Test",
        "User",
        "test@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    // Link phone
    env.link_phone_to_account(&principal, "+256700123456")
        .expect("Phone linking should succeed");

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find phone linking entry
    let link_entry = entries.iter().find(|e| e.action == "phone_linked");

    assert!(link_entry.is_some(), "Phone linking should be logged");

    let entry = link_entry.unwrap();
    assert_eq!(entry.success, true, "Phone linking should be marked as success");
    assert!(entry.details.contains("+256700123456"), "Details should contain phone");
    assert!(entry.details.contains(&principal), "Details should contain principal");
}

#[test]
fn test_failed_phone_linking_logged() {
    let env = TestEnv::new();

    // Register user with phone
    env.register_user(
        Some("+256700123456".to_string()),
        None,
        "First",
        "User",
        "first@example.com",
        "UGX",
        "1111",
    ).expect("Registration should succeed");

    // Register second user with principal
    let principal = "aaaaa-aa".to_string();
    env.register_user(
        None,
        Some(principal.clone()),
        "Second",
        "User",
        "second@example.com",
        "KES",
        "2222",
    ).expect("Registration should succeed");

    // Try to link already-taken phone
    let result = env.link_phone_to_account(&principal, "+256700123456");
    assert!(result.is_err(), "Phone linking should fail");

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find failed phone linking entry
    let failed_entry = entries.iter().find(|e| e.action == "phone_link_failed");

    assert!(failed_entry.is_some(), "Failed phone linking should be logged");

    let entry = failed_entry.unwrap();
    assert_eq!(entry.success, false, "Failed linking should be marked as failure");
    assert!(entry.details.contains("already registered"), "Details should mention already registered");
}

#[test]
fn test_canister_initialization_logged() {
    let env = TestEnv::new();

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Find initialization entry
    let init_entry = entries.iter().find(|e| e.action == "canister_initialized");

    assert!(init_entry.is_some(), "Canister initialization should be logged");

    let entry = init_entry.unwrap();
    assert_eq!(entry.success, true, "Initialization should be marked as success");
    assert!(entry.user_id.is_none(), "Initialization should not have user_id");
}

#[test]
fn test_audit_log_filtering_by_user() {
    let env = TestEnv::new();

    // Register two users
    let user1_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "User",
        "One",
        "user1@example.com",
        "UGX",
        "1111",
    ).expect("Registration should succeed");

    let user2_id = env.register_user(
        Some("+256700222222".to_string()),
        None,
        "User",
        "Two",
        "user2@example.com",
        "UGX",
        "2222",
    ).expect("Registration should succeed");

    // Perform operations for user 1
    env.verify_pin("+256700111111", "1111").ok();
    env.change_pin("+256700111111", "1111", "3333").ok();

    // Perform operations for user 2
    env.verify_pin("+256700222222", "2222").ok();

    // Get audit log for user 1
    let arg1 = encode_args((user1_id.clone(), Some(100u64))).unwrap();
    let response1 = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_user_audit_log",
        arg1,
    ).expect("get_user_audit_log should succeed");

    let audit1: Result<Vec<AuditEntry>, String> = decode_one(&response1).unwrap();
    let entries1 = audit1.expect("Should get user 1 audit log");

    // All entries should be for user 1
    for entry in &entries1 {
        if entry.user_id.is_some() {
            assert_eq!(
                entry.user_id.as_ref().unwrap(),
                &user1_id,
                "All entries should be for user 1"
            );
        }
    }

    // Get audit log for user 2
    let arg2 = encode_args((user2_id.clone(), Some(100u64))).unwrap();
    let response2 = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_user_audit_log",
        arg2,
    ).expect("get_user_audit_log should succeed");

    let audit2: Result<Vec<AuditEntry>, String> = decode_one(&response2).unwrap();
    let entries2 = audit2.expect("Should get user 2 audit log");

    // All entries should be for user 2
    for entry in &entries2 {
        if entry.user_id.is_some() {
            assert_eq!(
                entry.user_id.as_ref().unwrap(),
                &user2_id,
                "All entries should be for user 2"
            );
        }
    }

    // User 1 should have more entries (registration + verification + change)
    assert!(
        entries1.len() > entries2.len(),
        "User 1 should have more audit entries"
    );
}

#[test]
fn test_audit_log_filtering_by_action() {
    let env = TestEnv::new();

    // Register multiple users
    for i in 1..=3 {
        let phone = format!("+25670011111{}", i);
        let email = format!("user{}@example.com", i);
        let last_name = format!("User{}", i); // Use 2+ character last name
        env.register_user(
            Some(phone),
            None,
            "User",
            &last_name,
            &email,
            "UGX",
            "1234",
        ).expect("Registration should succeed");
    }

    // Get audit log for specific action
    let arg = encode_args(("user_registered".to_string(), Some(100u64))).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_by_action",
        arg,
    ).expect("get_audit_by_action should succeed");

    let audit: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit.expect("Should get filtered audit log");

    // All entries should have the requested action
    for entry in &entries {
        assert_eq!(
            entry.action,
            "user_registered",
            "All entries should have action 'user_registered'"
        );
    }

    // Should have at least 3 registration entries
    assert!(entries.len() >= 3, "Should have at least 3 registration entries");
}

#[test]
fn test_audit_log_get_failed_operations() {
    let env = TestEnv::new();

    // Register a user
    env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Generate some failed operations
    let _ = env.verify_pin("+256700123456", "9999"); // Wrong PIN
    let _ = env.change_pin("+256700123456", "9999", "5678"); // Wrong old PIN
    let _ = env.register_user( // Duplicate phone
        Some("+256700123456".to_string()),
        None,
        "Duplicate",
        "User",
        "dup@example.com",
        "UGX",
        "5678",
    );

    // Get failed operations
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_failed_operations",
        arg,
    ).expect("get_failed_operations should succeed");

    let audit: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit.expect("Should get failed operations");

    // All entries should be failures
    for entry in &entries {
        assert_eq!(entry.success, false, "All entries should be failures");
    }

    // Should have at least 3 failed operations
    assert!(entries.len() >= 3, "Should have at least 3 failed operations");
}

#[test]
fn test_audit_stats() {
    let env = TestEnv::new();

    // Register users and perform operations
    for i in 1..=2 {
        let phone = format!("+25670011111{}", i);
        let email = format!("user{}@example.com", i);
        let last_name = format!("User{}", i); // Use 2+ character last name
        env.register_user(
            Some(phone.clone()),
            None,
            "User",
            &last_name,
            &email,
            "UGX",
            "1234",
        ).expect("Registration should succeed");

        // Verify PIN (success)
        env.verify_pin(&phone, "1234").ok();

        // Wrong PIN (failure)
        env.verify_pin(&phone, "9999").ok();
    }

    // Get audit stats
    let arg = encode_one(()).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_stats",
        arg,
    ).expect("get_audit_stats should succeed");

    let stats_result: Result<shared_types::audit::AuditStats, String> = decode_one(&response).unwrap();
    let stats = stats_result.expect("Should get audit stats");

    // Verify stats
    assert!(stats.total_entries > 0, "Should have total entries");
    assert!(stats.successful_operations > 0, "Should have success entries");
    assert!(stats.failed_operations > 0, "Should have failure entries");
    assert!(
        stats.total_entries >= stats.successful_operations + stats.failed_operations,
        "Total should be >= sum of successes and failures"
    );
}

#[test]
fn test_audit_log_timestamps_are_sequential() {
    let env = TestEnv::new();

    // Perform multiple operations
    env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    env.verify_pin("+256700123456", "1234").ok();
    env.change_pin("+256700123456", "1234", "5678").ok();

    // Get audit log
    let arg = encode_one(Some(100u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Verify timestamps are sequential (later events have higher timestamps)
    for i in 1..entries.len() {
        assert!(
            entries[i].timestamp >= entries[i - 1].timestamp,
            "Timestamps should be sequential or equal"
        );
    }
}

#[test]
fn test_audit_log_limit() {
    let env = TestEnv::new();

    // Register multiple users to generate many audit entries
    for i in 1..=10 {
        let phone = format!("+25670012345{}", i);
        let email = format!("user{}@example.com", i);
        env.register_user(
            Some(phone),
            None,
            "User",
            &i.to_string(),
            &email,
            "UGX",
            "1234",
        ).ok();
    }

    // Get audit log with limit of 5
    let arg = encode_one(Some(5u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    let entries = audit_result.expect("Should get audit log");

    // Should return at most 5 entries
    assert!(
        entries.len() <= 5,
        "Should respect limit of 5 entries, got {}",
        entries.len()
    );
}
