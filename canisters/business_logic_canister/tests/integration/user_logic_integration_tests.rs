/// Integration tests for user_logic module
/// These test the pure business logic functions that don't require I/O
use business_logic_canister::logic::user_logic;

// ============================================================================
// Identifier Validation Tests
// ============================================================================

#[test]
fn test_identifier_validation_comprehensive() {
    // Both None - should fail
    assert!(user_logic::validate_identifier_required(&None, &None).is_err());
    
    // Phone only - should pass
    assert!(user_logic::validate_identifier_required(&Some("+1234567890".to_string()), &None).is_ok());
    
    // Principal only - should pass
    assert!(user_logic::validate_identifier_required(&None, &Some("aaaaa-aa".to_string())).is_ok());
    
    // Both provided - should pass
    assert!(user_logic::validate_identifier_required(
        &Some("+1234567890".to_string()),
        &Some("aaaaa-aa".to_string())
    ).is_ok());
}

// ============================================================================
// PIN Validation Tests
// ============================================================================

#[test]
fn test_pin_validation_valid_pins() {
    assert!(user_logic::validate_pin_format("0000").is_ok());
    assert!(user_logic::validate_pin_format("1234").is_ok());
    assert!(user_logic::validate_pin_format("9999").is_ok());
    assert!(user_logic::validate_pin_format("0123").is_ok());
}

#[test]
fn test_pin_validation_invalid_length() {
    let result = user_logic::validate_pin_format("123");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "PIN must be exactly 4 digits");
    
    let result = user_logic::validate_pin_format("12345");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "PIN must be exactly 4 digits");
    
    let result = user_logic::validate_pin_format("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "PIN must be exactly 4 digits");
}

#[test]
fn test_pin_validation_non_numeric() {
    let result = user_logic::validate_pin_format("12a4");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "PIN must contain only digits");
    
    let result = user_logic::validate_pin_format("abcd");
    assert!(result.is_err());
    
    let result = user_logic::validate_pin_format("12 4");
    assert!(result.is_err());
    
    let result = user_logic::validate_pin_format("12.4");
    assert!(result.is_err());
}

// ============================================================================
// Phone Number Validation Tests
// ============================================================================

#[test]
fn test_phone_validation_valid_numbers() {
    assert!(user_logic::validate_phone_number_format("+1234567890").is_ok());
    assert!(user_logic::validate_phone_number_format("+254712345678").is_ok());
    assert!(user_logic::validate_phone_number_format("+256700000000").is_ok());
    assert!(user_logic::validate_phone_number_format("+233200000000").is_ok());
}

#[test]
fn test_phone_validation_missing_plus() {
    let result = user_logic::validate_phone_number_format("254712345678");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Phone number must start with +");
}

#[test]
fn test_phone_validation_too_short() {
    let result = user_logic::validate_phone_number_format("+123");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Phone number too short");
    
    let result = user_logic::validate_phone_number_format("+");
    assert!(result.is_err());
}

#[test]
fn test_phone_validation_empty() {
    let result = user_logic::validate_phone_number_format("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Phone number cannot be empty");
}

// ============================================================================
// Email Validation Tests
// ============================================================================

#[test]
fn test_email_validation_valid() {
    assert!(user_logic::validate_email_format("user@example.com").is_ok());
    assert!(user_logic::validate_email_format("test.user@domain.co.uk").is_ok());
    assert!(user_logic::validate_email_format("a@b.c").is_ok());
    assert!(user_logic::validate_email_format("user+tag@example.com").is_ok());
}

#[test]
fn test_email_validation_missing_at() {
    let result = user_logic::validate_email_format("userexample.com");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email must contain @");
}

#[test]
fn test_email_validation_missing_domain() {
    let result = user_logic::validate_email_format("user@example");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email must contain a domain");
}

#[test]
fn test_email_validation_empty() {
    let result = user_logic::validate_email_format("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Email cannot be empty");
}

// ============================================================================
// Name Validation Tests
// ============================================================================

#[test]
fn test_name_validation_valid() {
    assert!(user_logic::validate_name("John", "First name").is_ok());
    assert!(user_logic::validate_name("Mary Jane", "First name").is_ok());
    assert!(user_logic::validate_name("O'Brien", "Last name").is_ok());
    assert!(user_logic::validate_name("José", "First name").is_ok());
}

#[test]
fn test_name_validation_too_short() {
    let result = user_logic::validate_name("J", "First name");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "First name must be at least 2 characters");
    
    let result = user_logic::validate_name("D", "Last name");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Last name must be at least 2 characters");
}

#[test]
fn test_name_validation_too_long() {
    let long_name = "a".repeat(51);
    let result = user_logic::validate_name(&long_name, "First name");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "First name must be at most 50 characters");
}

#[test]
fn test_name_validation_empty() {
    let result = user_logic::validate_name("", "First name");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "First name cannot be empty");
}

#[test]
fn test_name_validation_edge_cases() {
    // Exactly 2 characters - should pass
    assert!(user_logic::validate_name("Jo", "First name").is_ok());
    
    // Exactly 50 characters - should pass
    let name_50 = "a".repeat(50);
    assert!(user_logic::validate_name(&name_50, "Last name").is_ok());
}

// ============================================================================
// Salt Generation Tests
// ============================================================================

#[test]
fn test_salt_generation_deterministic() {
    let salt1 = user_logic::generate_salt_from_time(1000);
    let salt2 = user_logic::generate_salt_from_time(1000);
    assert_eq!(salt1, salt2, "Same timestamp should produce same salt");
}

#[test]
fn test_salt_generation_different_times() {
    let salt1 = user_logic::generate_salt_from_time(1000);
    let salt2 = user_logic::generate_salt_from_time(2000);
    assert_ne!(salt1, salt2, "Different timestamps should produce different salts");
}

#[test]
fn test_salt_generation_length() {
    let salt = user_logic::generate_salt_from_time(1000);
    assert_eq!(salt.len(), 64, "Salt should be 64 hex characters (32 bytes)");
}

#[test]
fn test_salt_generation_hex_format() {
    let salt = user_logic::generate_salt_from_time(1000);
    assert!(salt.chars().all(|c| c.is_ascii_hexdigit()), "Salt should only contain hex characters");
}

#[test]
fn test_salt_generation_various_timestamps() {
    let timestamps = vec![0, 1, 100, 1000, 10000, 100000, u64::MAX];
    let mut salts = Vec::new();
    
    for ts in timestamps {
        let salt = user_logic::generate_salt_from_time(ts);
        assert_eq!(salt.len(), 64);
        assert!(!salts.contains(&salt), "Each timestamp should produce unique salt");
        salts.push(salt);
    }
}

// ============================================================================
// Combined Validation Tests (Real-world Scenarios)
// ============================================================================

#[test]
fn test_valid_user_registration_data() {
    // Simulate a valid user registration
    let phone = Some("+254712345678".to_string());
    let principal = None;
    let first_name = "John";
    let last_name = "Doe";
    let email = "john@example.com";
    let pin = "1234";
    
    assert!(user_logic::validate_identifier_required(&phone, &principal).is_ok());
    assert!(user_logic::validate_phone_number_format(phone.as_ref().unwrap()).is_ok());
    assert!(user_logic::validate_name(first_name, "First name").is_ok());
    assert!(user_logic::validate_name(last_name, "Last name").is_ok());
    assert!(user_logic::validate_email_format(email).is_ok());
    assert!(user_logic::validate_pin_format(pin).is_ok());
}

#[test]
fn test_invalid_user_registration_scenarios() {
    // No identifier
    assert!(user_logic::validate_identifier_required(&None, &None).is_err());
    
    // Invalid phone format
    assert!(user_logic::validate_phone_number_format("254712345678").is_err());
    
    // Invalid name (too short)
    assert!(user_logic::validate_name("J", "First name").is_err());
    
    // Invalid email (no @)
    assert!(user_logic::validate_email_format("johnexample.com").is_err());
    
    // Invalid PIN (too short)
    assert!(user_logic::validate_pin_format("123").is_err());
}

#[test]
fn test_link_phone_validation_scenario() {
    // Valid scenario
    let phone = "+254712345678";
    let pin = "1234";
    
    assert!(user_logic::validate_phone_number_format(phone).is_ok());
    assert!(user_logic::validate_pin_format(pin).is_ok());
    
    // Invalid phone
    assert!(user_logic::validate_phone_number_format("254712345678").is_err());
    
    // Invalid PIN
    assert!(user_logic::validate_pin_format("12345").is_err());
}

// ============================================================================
// Edge Cases and Security Tests
// ============================================================================

#[test]
fn test_special_characters_in_names() {
    // Names with special characters should be allowed
    assert!(user_logic::validate_name("O'Brien", "Last name").is_ok());
    assert!(user_logic::validate_name("Mary-Jane", "First name").is_ok());
    assert!(user_logic::validate_name("José", "First name").is_ok());
}

#[test]
fn test_whitespace_in_names() {
    // Names with spaces should be allowed
    assert!(user_logic::validate_name("Mary Jane", "First name").is_ok());
    assert!(user_logic::validate_name("Van Der Berg", "Last name").is_ok());
}

#[test]
fn test_pin_security_patterns() {
    // All valid 4-digit PINs should pass
    assert!(user_logic::validate_pin_format("0000").is_ok());
    assert!(user_logic::validate_pin_format("1111").is_ok());
    assert!(user_logic::validate_pin_format("1234").is_ok());
    assert!(user_logic::validate_pin_format("9999").is_ok());
}

#[test]
fn test_international_phone_numbers() {
    // Various country codes
    assert!(user_logic::validate_phone_number_format("+1234567890").is_ok()); // US
    assert!(user_logic::validate_phone_number_format("+254712345678").is_ok()); // Kenya
    assert!(user_logic::validate_phone_number_format("+256700000000").is_ok()); // Uganda
    assert!(user_logic::validate_phone_number_format("+233200000000").is_ok()); // Ghana
    assert!(user_logic::validate_phone_number_format("+27123456789").is_ok()); // South Africa
    assert!(user_logic::validate_phone_number_format("+234800000000").is_ok()); // Nigeria
}

#[test]
fn test_email_edge_cases() {
    // Valid edge cases
    assert!(user_logic::validate_email_format("a@b.c").is_ok());
    assert!(user_logic::validate_email_format("user+tag@example.com").is_ok());
    assert!(user_logic::validate_email_format("user.name@example.co.uk").is_ok());
    
    // Invalid edge cases - basic validation only catches missing @ or domain
    // Note: Our validation is intentionally basic for now
    assert!(user_logic::validate_email_format("user@").is_err()); // No domain
    assert!(user_logic::validate_email_format("user").is_err()); // No @ or domain
    assert!(user_logic::validate_email_format("example.com").is_err()); // No @
}
