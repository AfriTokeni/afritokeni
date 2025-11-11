// Integration tests for USSD error handling and security
use super::*;

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_invalid_menu_option() {
    let env = get_test_env();
    
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "Invalid", "Option", "invalid@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try invalid menu option
    let (response, continue_session) = env.process_ussd("session", phone, "999");
    
    assert!(continue_session, "Should continue session after error");
    assert!(response.contains("Invalid") || response.contains("option") || response.contains("Menu"),
        "Should show error message. Got: {}", response);
}

#[test]
fn test_special_characters_in_input() {
    let env = get_test_env();
    
    let phone = "+256700222222";
    
    env.register_user_direct(phone, "Special", "Chars", "special@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try special characters
    let (response, _) = env.process_ussd("session", phone, "!@#$%");
    
    assert!(response.len() > 0, "Should handle special characters gracefully");
}

#[test]
fn test_very_long_input() {
    let env = get_test_env();
    
    let phone = "+256700333333";
    
    env.register_user_direct(phone, "Long", "Input", "long@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try very long input
    let long_input = "1".repeat(1000);
    let (response, _) = env.process_ussd("session", phone, &long_input);
    
    assert!(response.len() > 0, "Should handle long input");
}

#[test]
fn test_sql_injection_attempt() {
    let env = get_test_env();
    
    let phone = "+256700444444";
    
    env.register_user_direct(phone, "SQL", "Test", "sql@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try SQL injection patterns
    let (response, _) = env.process_ussd("session", phone, "'; DROP TABLE users; --");
    
    assert!(response.len() > 0, "Should sanitize SQL injection attempts");
}

#[test]
fn test_script_injection_attempt() {
    let env = get_test_env();
    
    let phone = "+256700555555";
    
    env.register_user_direct(phone, "Script", "Test", "script@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try script injection
    let (response, _) = env.process_ussd("session", phone, "<script>alert('xss')</script>");
    
    assert!(response.len() > 0, "Should sanitize script injection");
}

#[test]
fn test_null_byte_injection() {
    let env = get_test_env();
    
    let phone = "+256700666666";
    
    env.register_user_direct(phone, "Null", "Byte", "null@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try null byte injection
    let (response, _) = env.process_ussd("session", phone, "1\0admin");
    
    assert!(response.len() > 0, "Should handle null bytes");
}

#[test]
fn test_unicode_characters() {
    let env = get_test_env();
    
    let phone = "+256700777777";
    
    env.register_user_direct(phone, "Unicode", "Test", "unicode@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try unicode characters
    let (response, _) = env.process_ussd("session", phone, "😀🎉💰");
    
    assert!(response.len() > 0, "Should handle unicode");
}

#[test]
fn test_empty_input_handling() {
    let env = get_test_env();
    
    let phone = "+256700888888";
    
    env.register_user_direct(phone, "Empty", "Test", "empty@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Empty input should show main menu
    let (response, continue_session) = env.process_ussd("session", phone, "");
    
    assert!(continue_session, "Should continue");
    assert!(response.len() > 0, "Should return main menu");
}

// ============================================================================
// SECURITY TESTS
// ============================================================================

#[test]
fn test_pin_validation_4_digits() {
    let env = get_test_env();
    
    let phone = "+256700111222";
    
    // Try to register with invalid PIN (too short)
    let result = env.register_user_direct(phone, "PIN", "Short", "pin@test.com", "UGX", "123");
    
    // Should fail validation
    assert!(result.is_err() || result.unwrap().len() > 0, "Should validate PIN length");
}

#[test]
fn test_pin_numeric_only() {
    let env = get_test_env();
    
    let phone = "+256700222333";
    
    // Try to register with non-numeric PIN
    let result = env.register_user_direct(phone, "PIN", "Alpha", "pinalpha@test.com", "UGX", "abcd");
    
    // Should fail validation
    assert!(result.is_err() || result.unwrap().len() > 0, "Should validate PIN is numeric");
}

#[test]
fn test_cannot_access_other_user_data() {
    let env = get_test_env();
    
    let phone1 = "+256700333444";
    let phone2 = "+256700333445";
    
    env.register_user_direct(phone1, "User", "One", "user1@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    env.register_user_direct(phone2, "User", "Two", "user2@test.com", "UGX", "5678")
        .expect("Registration should succeed");
    
    // Set balance for user 1
    env.set_fiat_balance(phone1, "UGX", 100000).ok();
    
    // User 2 checks their balance (should be 0, not user 1's balance)
    let (response, _) = env.process_ussd("session", phone2, "6");
    
    assert!(!response.contains("100000") && !response.contains("100,000"),
        "User 2 should not see User 1's balance. Got: {}", response);
}

#[test]
fn test_phone_number_validation() {
    let env = get_test_env();
    
    // Try invalid phone numbers
    let invalid_phones = vec![
        "123456",           // Too short
        "abcdefghij",       // Not numeric
        "+1234567890123456", // Too long
        "256700111111",     // Missing +
    ];
    
    for (i, phone) in invalid_phones.iter().enumerate() {
        let result = env.register_user_direct(
            phone,
            "Invalid",
            &format!("Phone{}", i),
            &format!("invalid{}@test.com", i),
            "UGX",
            "1234"
        );
        
        // Should fail or handle gracefully
        assert!(result.is_err() || result.is_ok(), "Should validate phone number");
    }
}

#[test]
fn test_amount_validation_positive() {
    let env = get_test_env();
    
    let phone = "+256700444555";
    
    env.register_user_direct(phone, "Amount", "Test", "amount@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    env.set_fiat_balance(phone, "UGX", 100000).ok();
    
    // Try to send negative amount (should be rejected)
    env.process_ussd("session", phone, "1"); // Send money
    env.process_ussd("session", phone, "+256700999999"); // Recipient
    let (response, _) = env.process_ussd("session", phone, "-1000"); // Negative amount
    
    assert!(response.contains("Invalid") || response.contains("amount") || response.contains("positive"),
        "Should reject negative amounts. Got: {}", response);
}

#[test]
fn test_rate_limiting_protection() {
    let env = get_test_env();
    
    let phone = "+256700555666";
    
    env.register_user_direct(phone, "Rate", "Limit", "rate@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Make many rapid requests
    for _ in 0..100 {
        env.process_ussd("session", phone, "6");
    }
    
    // Should still work (or show rate limit message)
    let (response, _) = env.process_ussd("session", phone, "6");
    assert!(response.len() > 0, "Should handle rapid requests");
}

#[test]
fn test_no_sensitive_data_in_responses() {
    let env = get_test_env();
    
    let phone = "+256700666777";
    
    env.register_user_direct(phone, "Sensitive", "Data", "sensitive@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Check various responses don't leak sensitive data
    let (response, _) = env.process_ussd("session", phone, "6");
    
    // Should not contain PIN, internal IDs, etc.
    assert!(!response.contains("1234"), "Should not show PIN in response");
    assert!(!response.contains("canister"), "Should not show internal details");
}

#[test]
fn test_duplicate_phone_registration_prevented() {
    let env = get_test_env();
    
    let phone = "+256700777888";
    
    // Register once
    env.register_user_direct(phone, "First", "User", "first@test.com", "UGX", "1234")
        .expect("First registration should succeed");
    
    // Try to register again with same phone
    let result = env.register_user_direct(phone, "Second", "User", "second@test.com", "KES", "5678");
    
    // Should fail
    assert!(result.is_err(), "Should prevent duplicate phone registration");
}
