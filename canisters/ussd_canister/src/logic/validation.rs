/// Pure validation logic for USSD inputs
/// No I/O, no async, no IC calls - fully testable

/// Validates phone number format
pub fn validate_phone_format(phone: &str) -> Result<(), String> {
    if phone.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }
    if !phone.starts_with('+') {
        return Err("Phone number must start with +".to_string());
    }
    if phone.len() < 10 {
        return Err("Phone number too short".to_string());
    }
    // Check if it contains only digits after the +
    let digits = &phone[1..];
    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err("Phone number must contain only digits after +".to_string());
    }
    Ok(())
}

/// Validates amount format and returns parsed value
pub fn parse_and_validate_amount(amount_str: &str) -> Result<f64, String> {
    if amount_str.is_empty() {
        return Err("Amount cannot be empty".to_string());
    }
    
    let amount = amount_str.parse::<f64>()
        .map_err(|_| "Invalid amount format".to_string())?;
    
    if amount <= 0.0 {
        return Err("Amount must be greater than zero".to_string());
    }
    
    if amount > 100_000_000.0 {
        return Err("Amount too large".to_string());
    }
    
    Ok(amount)
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

/// Validates Bitcoin address format (basic check)
pub fn validate_btc_address_format(address: &str) -> Result<(), String> {
    if address.is_empty() {
        return Err("Bitcoin address cannot be empty".to_string());
    }
    if address.len() < 26 || address.len() > 62 {
        return Err("Invalid Bitcoin address length".to_string());
    }
    // Basic check - should start with 1, 3, or bc1
    if !address.starts_with('1') && !address.starts_with('3') && !address.starts_with("bc1") {
        return Err("Invalid Bitcoin address format".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone_format_valid() {
        assert!(validate_phone_format("+256700000001").is_ok());
        assert!(validate_phone_format("+254712345678").is_ok());
    }

    #[test]
    fn test_validate_phone_format_invalid() {
        assert!(validate_phone_format("").is_err());
        assert!(validate_phone_format("256700000001").is_err()); // Missing +
        assert!(validate_phone_format("+256").is_err()); // Too short
        assert!(validate_phone_format("+256abc123").is_err()); // Contains letters
    }

    #[test]
    fn test_parse_and_validate_amount_valid() {
        assert_eq!(parse_and_validate_amount("100").unwrap(), 100.0);
        assert_eq!(parse_and_validate_amount("1000.50").unwrap(), 1000.50);
    }

    #[test]
    fn test_parse_and_validate_amount_invalid() {
        assert!(parse_and_validate_amount("").is_err());
        assert!(parse_and_validate_amount("abc").is_err());
        assert!(parse_and_validate_amount("0").is_err());
        assert!(parse_and_validate_amount("-100").is_err());
        assert!(parse_and_validate_amount("999999999").is_err()); // Too large
    }

    #[test]
    fn test_validate_pin_format_valid() {
        assert!(validate_pin_format("1234").is_ok());
        assert!(validate_pin_format("0000").is_ok());
    }

    #[test]
    fn test_validate_pin_format_invalid() {
        assert!(validate_pin_format("123").is_err()); // Too short
        assert!(validate_pin_format("12345").is_err()); // Too long
        assert!(validate_pin_format("abcd").is_err()); // Not digits
        assert!(validate_pin_format("12a4").is_err()); // Contains letter
    }

    #[test]
    fn test_validate_btc_address_format_valid() {
        assert!(validate_btc_address_format("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").is_ok());
        assert!(validate_btc_address_format("3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy").is_ok());
        assert!(validate_btc_address_format("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq").is_ok());
    }

    #[test]
    fn test_validate_btc_address_format_invalid() {
        assert!(validate_btc_address_format("").is_err());
        assert!(validate_btc_address_format("abc").is_err()); // Too short
        assert!(validate_btc_address_format("2invalidaddress").is_err()); // Wrong prefix
    }
}
