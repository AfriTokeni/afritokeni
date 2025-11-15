/// Pure business logic for user management operations
/// No I/O, no async, fully testable
///
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

/// Validates phone number format (E.164 standard)
///
/// E.164 format: +[country code][subscriber number]
/// - Must start with +
/// - Country code: 1-3 digits
/// - Total length: 10-15 characters (including +)
/// - Only digits after +
///
/// Examples:
/// - +256712345678 (Uganda)
/// - +254712345678 (Kenya)
/// - +1234567890 (US)
pub fn validate_phone_number_format(phone: &str) -> Result<(), String> {
    if phone.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }

    if !phone.starts_with('+') {
        return Err("Phone number must be in E.164 format (start with +)".to_string());
    }

    // Remove the + prefix for validation
    let digits = &phone[1..];

    // Check if all remaining characters are digits
    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err("Phone number must contain only digits after +".to_string());
    }

    // E.164 specifies 15 digits maximum (excluding +)
    // Minimum is typically 8-10 digits (country code + number)
    let len = digits.len();
    if len < 8 {
        return Err("Phone number too short (minimum 8 digits)".to_string());
    }
    if len > 15 {
        return Err("Phone number too long (maximum 15 digits)".to_string());
    }

    // Validate country code exists (1-3 digits at start)
    // This is a basic check - we don't validate against all country codes
    if len >= 10 {
        Ok(())
    } else {
        Err("Phone number format invalid".to_string())
    }
}

/// Validates email format (RFC 5322 basic validation)
///
/// Requirements:
/// - Must contain exactly one @
/// - Local part (before @) must be 1-64 characters
/// - Domain part (after @) must contain at least one dot
/// - Domain must be 1-255 characters
/// - No spaces allowed
/// - Must not start or end with dot or @
///
/// Examples:
/// - user@example.com ✓
/// - test.user@domain.co.uk ✓
/// - user+tag@example.com ✓
/// - user @example.com ✗ (space)
/// - @example.com ✗ (no local part)
pub fn validate_email_format(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    // Trim whitespace for validation
    let email = email.trim();

    // Check for spaces
    if email.contains(' ') {
        return Err("Email cannot contain spaces".to_string());
    }

    // Must contain exactly one @
    let at_count = email.chars().filter(|&c| c == '@').count();
    if at_count == 0 {
        return Err("Email must contain @".to_string());
    }
    if at_count > 1 {
        return Err("Email must contain only one @".to_string());
    }

    // Split into local and domain parts
    let parts: Vec<&str> = email.split('@').collect();
    let local = parts[0];
    let domain = parts[1];

    // Validate local part (before @)
    if local.is_empty() {
        return Err("Email local part cannot be empty".to_string());
    }
    if local.len() > 64 {
        return Err("Email local part too long (max 64 characters)".to_string());
    }
    if local.starts_with('.') || local.ends_with('.') {
        return Err("Email local part cannot start or end with dot".to_string());
    }

    // Validate domain part (after @)
    if domain.is_empty() {
        return Err("Email domain cannot be empty".to_string());
    }
    if domain.len() > 255 {
        return Err("Email domain too long (max 255 characters)".to_string());
    }
    if !domain.contains('.') {
        return Err("Email domain must contain at least one dot".to_string());
    }
    if domain.starts_with('.') || domain.ends_with('.') {
        return Err("Email domain cannot start or end with dot".to_string());
    }
    if domain.starts_with('-') || domain.ends_with('-') {
        return Err("Email domain cannot start or end with hyphen".to_string());
    }

    // Validate domain has valid TLD (at least 2 chars after last dot)
    if let Some(last_dot_pos) = domain.rfind('.') {
        let tld = &domain[last_dot_pos + 1..];
        if tld.len() < 2 {
            return Err("Email domain TLD must be at least 2 characters".to_string());
        }
        if !tld.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Email domain TLD must contain only letters".to_string());
        }
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
        assert_eq!(result.unwrap_err(), "Phone number must be in E.164 format (start with +)");
    }

    #[test]
    fn test_validate_phone_number_format_too_short() {
        let result = validate_phone_number_format("+123");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Phone number too short (minimum 8 digits)");
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
        assert_eq!(result.unwrap_err(), "Email domain must contain at least one dot");
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
}
