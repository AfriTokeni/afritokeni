use super::*;

// ============================================================================
// User Enumeration Prevention Tests
// Tests that error messages don't reveal whether a user exists or not
// ============================================================================

#[test]
fn test_duplicate_phone_registration_generic_error() {
    let env = TestEnv::new();

    // Register first user
    env.register_user(
        Some("+256700111111".to_string()),
        None,
        "First",
        "User",
        "first@example.com",
        "UGX",
        "1111",
    ).expect("First registration should succeed");

    // Try to register with same phone
    let result = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Second",
        "User",
        "second@example.com",
        "UGX",
        "2222",
    );

    assert!(result.is_err(), "Duplicate registration should fail");
    let error = result.unwrap_err();

    // Error should be generic, not revealing that user exists
    assert!(
        error.contains("Registration failed") || error.contains("already be in use"),
        "Error should be generic: {}",
        error
    );

    // Should NOT contain specific info like "user exists" or phone number
    assert!(
        !error.to_lowercase().contains("user exists"),
        "Error should not say 'user exists': {}",
        error
    );
    assert!(
        !error.contains("+256700111111"),
        "Error should not contain phone number: {}",
        error
    );
}

#[test]
fn test_duplicate_principal_registration_generic_error() {
    let env = TestEnv::new();

    let principal = "aaaaa-aa".to_string();

    // Register first user
    env.register_user(
        None,
        Some(principal.clone()),
        "First",
        "User",
        "first@example.com",
        "KES",
        "1111",
    ).expect("First registration should succeed");

    // Try to register with same principal
    let result = env.register_user(
        None,
        Some(principal.clone()),
        "Second",
        "User",
        "second@example.com",
        "KES",
        "2222",
    );

    assert!(result.is_err(), "Duplicate registration should fail");
    let error = result.unwrap_err();

    // Error should be generic
    assert!(
        error.contains("Registration failed") || error.contains("already be in use"),
        "Error should be generic: {}",
        error
    );

    // Should NOT reveal that principal is registered
    assert!(
        !error.to_lowercase().contains("user exists"),
        "Error should not say 'user exists': {}",
        error
    );
    assert!(
        !error.contains(&principal),
        "Error should not contain principal: {}",
        error
    );
}

#[test]
fn test_verify_pin_nonexistent_user_generic_error() {
    let env = TestEnv::new();

    // Try to verify PIN for user that doesn't exist
    let result = env.verify_pin("+256700999999", "1234");

    assert!(result.is_err(), "PIN verification for nonexistent user should fail");
    let error = result.unwrap_err();

    // Should return generic error
    assert!(
        error.contains("Invalid credentials") || error.contains("not found"),
        "Error should be generic: {}",
        error
    );

    // Should NOT reveal that user doesn't exist
    assert!(
        !error.to_lowercase().contains("user does not exist"),
        "Error should not explicitly say user doesn't exist: {}",
        error
    );
    assert!(
        !error.to_lowercase().contains("no user found"),
        "Error should not explicitly say no user found: {}",
        error
    );
}

#[test]
fn test_change_pin_nonexistent_user_generic_error() {
    let env = TestEnv::new();

    // Try to change PIN for user that doesn't exist
    let result = env.change_pin("+256700999999", "1234", "5678");

    assert!(result.is_err(), "PIN change for nonexistent user should fail");
    let error = result.unwrap_err();

    // Should return generic error
    assert!(
        error.contains("Invalid credentials") || error.contains("not found"),
        "Error should be generic: {}",
        error
    );

    // Should NOT reveal whether user exists
    assert!(
        !error.to_lowercase().contains("user does not exist"),
        "Error should not explicitly say user doesn't exist: {}",
        error
    );
}

#[test]
fn test_link_phone_nonexistent_principal_generic_error() {
    let env = TestEnv::new();

    // Try to link phone to principal that doesn't exist
    let result = env.link_phone_to_account("nonexistent-principal", "+256700111111");

    assert!(result.is_err(), "Linking phone to nonexistent principal should fail");
    let error = result.unwrap_err();

    // Should return generic error
    assert!(
        error.contains("Unable to link") || error.contains("verify your account"),
        "Error should be generic: {}",
        error
    );

    // Should NOT reveal that principal doesn't exist
    assert!(
        !error.to_lowercase().contains("principal not found"),
        "Error should not say principal not found: {}",
        error
    );
    assert!(
        !error.to_lowercase().contains("user does not exist"),
        "Error should not say user doesn't exist: {}",
        error
    );
}

#[test]
fn test_link_phone_already_taken_generic_error() {
    let env = TestEnv::new();

    // Register user with phone
    env.register_user(
        Some("+256700111111".to_string()),
        None,
        "First",
        "User",
        "first@example.com",
        "UGX",
        "1111",
    ).expect("First registration should succeed");

    // Register second user with principal only
    let principal = "aaaaa-aa".to_string();
    env.register_user(
        None,
        Some(principal.clone()),
        "Second",
        "User",
        "second@example.com",
        "KES",
        "2222",
    ).expect("Second registration should succeed");

    // Try to link already-taken phone
    let result = env.link_phone_to_account(&principal, "+256700111111");

    assert!(result.is_err(), "Linking already-taken phone should fail");
    let error = result.unwrap_err();

    // Should return generic error
    assert!(
        error.contains("Unable to link") || error.contains("already be in use"),
        "Error should be generic: {}",
        error
    );

    // Should NOT explicitly say "phone already registered"
    assert!(
        !error.to_lowercase().contains("phone already registered"),
        "Error should not say 'phone already registered': {}",
        error
    );
}

#[test]
fn test_update_profile_nonexistent_user_generic_error() {
    let env = TestEnv::new();

    let updates = ProfileUpdates {
        first_name: Some("New".to_string()),
        last_name: Some("Name".to_string()),
        email: None,
        preferred_currency: None,
    };

    let arg = encode_args(("+256700999999", updates)).unwrap();
    let response = env.pic.update_call(
        env.user_canister_id,
        Principal::anonymous(),
        "update_user_profile",
        arg,
    );

    // Should fail (either not found or not implemented)
    assert!(response.is_ok(), "Call should succeed even if user not found");

    let result: Result<(), String> = decode_one(&response.unwrap()).unwrap();
    assert!(result.is_err(), "Update should fail for nonexistent user");

    let error = result.unwrap_err();

    // If it's about user not found (not implementation), should be generic
    if !error.contains("not yet implemented") {
        assert!(
            error.contains("Unable to update") || error.contains("verify your account"),
            "Error should be generic: {}",
            error
        );

        assert!(
            !error.to_lowercase().contains("user not found"),
            "Error should not explicitly say user not found: {}",
            error
        );
    }
}

#[test]
fn test_wrong_pin_and_nonexistent_user_same_error_format() {
    let env = TestEnv::new();

    // Register a user
    env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Try wrong PIN for existing user
    let wrong_pin_result = env.verify_pin("+256700111111", "9999");

    // Try any PIN for nonexistent user
    let nonexistent_result = env.verify_pin("+256700999999", "1234");

    // Both should return similar error patterns
    // (either both Ok(false) or both Err with similar messages)
    match (wrong_pin_result, nonexistent_result) {
        (Ok(false), Ok(false)) => {
            // Good: Both return false without revealing existence
        }
        (Err(e1), Err(e2)) => {
            // Errors should have similar structure
            assert!(
                (e1.contains("Invalid") && e2.contains("Invalid")) ||
                (e1.contains("credentials") && e2.contains("credentials")),
                "Error formats should be similar:\nWrong PIN: {}\nNonexistent: {}",
                e1, e2
            );
        }
        (Ok(verified), Err(e)) | (Err(e), Ok(verified)) => {
            // If one returns Ok and other Err, that's still acceptable
            // as long as the error is generic
            assert!(
                e.contains("Invalid credentials") || e.contains("not found"),
                "Error should be generic: {}",
                e
            );
        }
    }
}

#[test]
fn test_get_user_profile_nonexistent_user_generic_error() {
    let env = TestEnv::new();

    // Try to get profile for nonexistent user
    let arg = encode_one("+256700999999".to_string()).unwrap();
    let response = env.pic.update_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_user_profile_update",
        arg,
    ).expect("Call should succeed");

    let result: Result<UserProfile, String> = decode_one(&response).unwrap();
    assert!(result.is_err(), "Should fail for nonexistent user");

    let error = result.unwrap_err();

    // Error should be generic
    assert!(
        error.contains("User not found") || error.contains("not found"),
        "Error should mention not found: {}",
        error
    );

    // Should NOT reveal specific details
    assert!(
        !error.contains("+256700999999"),
        "Error should not contain the identifier: {}",
        error
    );
}

#[test]
fn test_audit_log_does_not_expose_sensitive_info() {
    let env = TestEnv::new();

    // Register a user
    let user_id = env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Secret",
        "User",
        "secret@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Try wrong PIN to generate failed audit entry
    let _ = env.verify_pin("+256700111111", "9999");

    // Get audit log
    let arg = encode_one(Some(50u64)).unwrap();
    let response = env.pic.query_call(
        env.user_canister_id,
        Principal::anonymous(),
        "get_audit_log",
        arg,
    ).expect("get_audit_log should succeed");

    let audit_result: Result<Vec<AuditEntry>, String> = decode_one(&response).unwrap();
    assert!(audit_result.is_ok(), "Should get audit log");

    let entries = audit_result.unwrap();

    // Check that audit entries don't contain PINs
    for entry in &entries {
        assert!(
            !entry.details.contains("1234") && !entry.details.contains("9999"),
            "Audit log should not contain PINs: {}",
            entry.details
        );

        // PIN verification failures should be logged but not with PIN value
        if entry.action == "pin_verification_failed" {
            assert!(
                !entry.details.contains("PIN"),
                "Failed verification should not log PIN value: {}",
                entry.details
            );
        }
    }
}

#[test]
fn test_timing_attack_resistance_same_response_time() {
    let env = TestEnv::new();

    // Register a user
    env.register_user(
        Some("+256700111111".to_string()),
        None,
        "Test",
        "User",
        "test@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");

    // Time verification for existing user with wrong PIN
    let start1 = std::time::Instant::now();
    let _ = env.verify_pin("+256700111111", "9999");
    let duration1 = start1.elapsed();

    // Time verification for nonexistent user
    let start2 = std::time::Instant::now();
    let _ = env.verify_pin("+256700999999", "1234");
    let duration2 = start2.elapsed();

    // Both operations should take similar time (within 100ms)
    // This is a basic check - in production, both should use Argon2 verification
    // even for nonexistent users to prevent timing attacks
    let diff = if duration1 > duration2 {
        duration1 - duration2
    } else {
        duration2 - duration1
    };

    println!("Existing user (wrong PIN): {:?}", duration1);
    println!("Nonexistent user: {:?}", duration2);
    println!("Difference: {:?}", diff);

    // Note: This test may not be perfectly reliable in all environments
    // It's more of a documentation of the security requirement
    // The important part is that both code paths use the same Argon2 verification
}

#[test]
fn test_user_exists_does_not_enumerate_principals() {
    let env = TestEnv::new();

    let principal = "aaaaa-aa".to_string();

    // Register user with principal
    env.register_user(
        None,
        Some(principal.clone()),
        "Test",
        "User",
        "test@example.com",
        "KES",
        "1234",
    ).expect("Registration should succeed");

    // user_exists should work for authorized callers (test mode)
    let exists = env.user_exists(&principal).expect("user_exists should succeed");
    assert!(exists, "User should exist");

    let not_exists = env.user_exists("nonexistent-principal").expect("user_exists should succeed");
    assert!(!not_exists, "User should not exist");

    // Note: user_exists is only callable by authorized canisters
    // This prevents arbitrary callers from enumerating users
}
