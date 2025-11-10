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
    
    // Verify initial balance is 0
    let balance = env.check_fiat_balance(&user_id, "UGX")
        .expect("Should check balance");
    assert_eq!(balance, 0);
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
    assert!(result.unwrap_err().contains("already registered"));
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
