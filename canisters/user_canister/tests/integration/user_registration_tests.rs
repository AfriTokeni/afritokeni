use super::*;

#[test]
fn test_full_user_registration_flow() {
    let env = TestEnv::new();
    
    // Register user
    let user_id = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");
    
    // Verify user exists in data canister
    let user = env.get_user(&user_id)
        .expect("Should get user")
        .expect("User should exist");
    
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.phone_number, Some("+256700123456".to_string()));
    assert_eq!(user.user_type, UserType::User);
    assert!(!user.is_verified);
}

#[test]
fn test_duplicate_phone_registration_fails() {
    let env = TestEnv::new();
    
    // Register first user
    env.register_user(
        Some("+256700999999".to_string()),
        None,
        "First",
        "User",
        "first@example.com",
        "UGX",
        "1111",
    ).expect("First registration should succeed");
    
    // Try to register with same phone - should fail
    let result = env.register_user(
        Some("+256700999999".to_string()),
        None,
        "Second",
        "User",
        "second@example.com",
        "UGX",
        "2222",
    );
    
    assert!(result.is_err(), "Duplicate phone should fail");
    assert!(result.unwrap_err().contains("already exists"));
}

#[test]
fn test_registration_without_identifier_fails() {
    let env = TestEnv::new();
    
    let result = env.register_user(
        None,
        None,
        "Invalid",
        "User",
        "invalid@example.com",
        "UGX",
        "1234",
    );
    
    assert!(result.is_err(), "Should require phone or principal");
}

#[test]
fn test_user_exists_check() {
    let env = TestEnv::new();
    
    // Register user
    env.register_user(
        Some("+256700123456".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "UGX",
        "1234",
    ).expect("Registration should succeed");
    
    // Check if user exists
    let exists = env.user_exists("+256700123456").expect("user_exists should succeed");
    assert!(exists, "User should exist");
    
    // Check non-existent user
    let not_exists = env.user_exists("+256700999999").expect("user_exists should succeed");
    assert!(!not_exists, "User should not exist");
}

#[test]
fn test_registration_with_principal() {
    let env = TestEnv::new();
    
    let principal_id = "aaaaa-aa".to_string();
    
    let user_id = env.register_user(
        None,
        Some(principal_id.clone()),
        "Web",
        "User",
        "web@example.com",
        "KES",
        "5678",
    ).expect("Registration with principal should succeed");
    
    // Verify user exists
    let user = env.get_user(&user_id)
        .expect("Should get user")
        .expect("User should exist");
    
    assert_eq!(user.principal_id, Some(principal_id));
    assert_eq!(user.phone_number, None);
}

#[test]
fn test_registration_with_both_phone_and_principal() {
    let env = TestEnv::new();
    
    let user_id = env.register_user(
        Some("+256700123456".to_string()),
        Some("aaaaa-aa".to_string()),
        "Hybrid",
        "User",
        "hybrid@example.com",
        "TZS",
        "9999",
    ).expect("Registration with both identifiers should succeed");
    
    // Verify user has both
    let user = env.get_user(&user_id)
        .expect("Should get user")
        .expect("User should exist");
    
    assert_eq!(user.phone_number, Some("+256700123456".to_string()));
    assert_eq!(user.principal_id, Some("aaaaa-aa".to_string()));
}

#[test]
fn test_invalid_pin_format_fails() {
    let env = TestEnv::new();
    
    // Too short
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "UGX",
        "123", // Invalid: too short
    );
    assert!(result.is_err(), "Short PIN should fail");
    assert!(result.unwrap_err().contains("PIN must be exactly 4 digits"));
    
    // Too long
    let result = env.register_user(
        Some("+256700123457".to_string()),
        None,
        "Jane",
        "Doe",
        "jane@example.com",
        "UGX",
        "12345", // Invalid: too long
    );
    assert!(result.is_err(), "Long PIN should fail");
    
    // Non-numeric
    let result = env.register_user(
        Some("+256700123458".to_string()),
        None,
        "Jack",
        "Doe",
        "jack@example.com",
        "UGX",
        "12ab", // Invalid: non-numeric
    );
    assert!(result.is_err(), "Non-numeric PIN should fail");
}

#[test]
fn test_invalid_phone_format_fails() {
    let env = TestEnv::new();
    
    // Missing +
    let result = env.register_user(
        Some("256700123456".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "Phone without + should fail");
    
    // Too short
    let result = env.register_user(
        Some("+256".to_string()),
        None,
        "Jane",
        "Doe",
        "jane@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "Short phone should fail");
}

#[test]
fn test_invalid_email_format_fails() {
    let env = TestEnv::new();
    
    // Missing @
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "John",
        "Doe",
        "johnexample.com", // Invalid: no @
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "Email without @ should fail");
    
    // Missing domain
    let result = env.register_user(
        Some("+256700123457".to_string()),
        None,
        "Jane",
        "Doe",
        "jane@example", // Invalid: no .
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "Email without domain should fail");
}

#[test]
fn test_invalid_currency_fails() {
    let env = TestEnv::new();
    
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "John",
        "Doe",
        "john@example.com",
        "USD", // Invalid: not supported
        "1234",
    );
    assert!(result.is_err(), "Unsupported currency should fail");
}
