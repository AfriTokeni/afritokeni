/// Pure business logic for user management operations
/// No I/O, no async, fully testable

/// Validates that at least one identifier (phone or principal) is provided
pub fn validate_identifier_required(
    phone_number: &Option<String>,
    principal_id: &Option<String>,
) -> Result<(), String> {
    if phone_number.is_none() && principal_id.is_none() {
        return Err("Either phone number or principal ID is required".to_string());
    }
    Ok(())
}

/// Validates PIN format (must be exactly 4 digits)
pub fn validate_pin_format(pin: &str) -> Result<(), String> {
    if pin.len() != 4 {
        return Err("PIN must be exactly 4 digits".to_string());
    }
    if !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err("PIN must contain only digits".to_string());
    }
    Ok(())
}

/// Validates phone number format (must start with + and be at least 10 chars)
pub fn validate_phone_number_format(phone: &str) -> Result<(), String> {
    if phone.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }
    if !phone.starts_with('+') {
        return Err("Phone number must start with +".to_string());
    }
    if phone.len() < 10 {
        return Err("Phone number too short".to_string());
    }
    Ok(())
}

/// Validates email format (basic check)
pub fn validate_email_format(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    if !email.contains('@') {
        return Err("Email must contain @".to_string());
    }
    if !email.contains('.') {
        return Err("Email must contain a domain".to_string());
    }
    Ok(())
}

/// Validates user name (first or last name)
pub fn validate_name(name: &str, field: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err(format!("{} cannot be empty", field));
    }
    if name.len() < 2 {
        return Err(format!("{} must be at least 2 characters", field));
    }
    if name.len() > 50 {
        return Err(format!("{} must be at most 50 characters", field));
    }
    Ok(())
}

/// Generates a deterministic salt from timestamp
#[allow(dead_code)]
pub fn generate_salt_from_time(time: u64) -> String {
    let salt_bytes: Vec<u8> = (0..32).map(|i| ((time >> (i % 8)) ^ i) as u8).collect();
    hex::encode(salt_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_identifier_required_both_none() {
        let result = validate_identifier_required(&None, &None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Either phone number or principal ID is required"
        );
    }

    #[test]
    fn test_validate_identifier_required_phone_provided() {
        let result = validate_identifier_required(&Some("+1234567890".to_string()), &None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_identifier_required_principal_provided() {
        let result = validate_identifier_required(&None, &Some("aaaaa-aa".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_identifier_required_both_provided() {
        let result = validate_identifier_required(
            &Some("+1234567890".to_string()),
            &Some("aaaaa-aa".to_string()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_pin_format_valid() {
        assert!(validate_pin_format("1234").is_ok());
        assert!(validate_pin_format("0000").is_ok());
        assert!(validate_pin_format("9999").is_ok());
    }

    #[test]
    fn test_validate_pin_format_too_short() {
        let result = validate_pin_format("123");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "PIN must be exactly 4 digits");
    }

    #[test]
    fn test_validate_pin_format_too_long() {
        let result = validate_pin_format("12345");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "PIN must be exactly 4 digits");
    }

    #[test]
    fn test_validate_pin_format_non_digits() {
        let result = validate_pin_format("12a4");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "PIN must contain only digits");
    }

    #[test]
    fn test_validate_phone_number_format_valid() {
        assert!(validate_phone_number_format("+1234567890").is_ok());
        assert!(validate_phone_number_format("+254712345678").is_ok());
    }

    #[test]
    fn test_validate_phone_number_format_no_plus() {
        let result = validate_phone_number_format("1234567890");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Phone number must start with +");
    }

    #[test]
    fn test_validate_phone_number_format_too_short() {
        let result = validate_phone_number_format("+123");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Phone number too short");
    }

    #[test]
    fn test_validate_phone_number_format_empty() {
        let result = validate_phone_number_format("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Phone number cannot be empty");
    }

    #[test]
    fn test_validate_email_format_valid() {
        assert!(validate_email_format("user@example.com").is_ok());
        assert!(validate_email_format("test.user@domain.co.uk").is_ok());
    }

    #[test]
    fn test_validate_email_format_no_at() {
        let result = validate_email_format("userexample.com");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email must contain @");
    }

    #[test]
    fn test_validate_email_format_no_domain() {
        let result = validate_email_format("user@example");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email must contain a domain");
    }

    #[test]
    fn test_validate_email_format_empty() {
        let result = validate_email_format("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email cannot be empty");
    }

    #[test]
    fn test_validate_name_valid() {
        assert!(validate_name("John", "First name").is_ok());
        assert!(validate_name("Mary Jane", "First name").is_ok());
    }

    #[test]
    fn test_validate_name_too_short() {
        let result = validate_name("J", "First name");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "First name must be at least 2 characters");
    }

    #[test]
    fn test_validate_name_too_long() {
        let long_name = "a".repeat(51);
        let result = validate_name(&long_name, "Last name");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Last name must be at most 50 characters");
    }

    #[test]
    fn test_validate_name_empty() {
        let result = validate_name("", "First name");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "First name cannot be empty");
    }

    #[test]
    fn test_generate_salt_from_time_deterministic() {
        let salt1 = generate_salt_from_time(1000);
        let salt2 = generate_salt_from_time(1000);
        assert_eq!(salt1, salt2);
    }

    #[test]
    fn test_generate_salt_from_time_different() {
        let salt1 = generate_salt_from_time(1000);
        let salt2 = generate_salt_from_time(2000);
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_generate_salt_from_time_length() {
        let salt = generate_salt_from_time(1000);
        assert_eq!(salt.len(), 64); // 32 bytes = 64 hex chars
    }
}
