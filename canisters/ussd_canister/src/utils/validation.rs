// Input validation utilities
use crate::config_loader::get_config;

/// Validate phone number format
pub fn is_valid_phone(phone: &str) -> bool {
    // Must start with + and be 10-15 digits
    if !phone.starts_with('+') {
        return false;
    }
    
    let digits = &phone[1..];
    digits.len() >= 10 && digits.len() <= 15 && digits.chars().all(|c| c.is_numeric())
}

/// Validate transaction amount
pub fn is_valid_amount(amount: f64) -> Result<(), String> {
    let config = get_config();
    
    if amount < config.transaction_limits.min_amount_kes {
        return Err(format!("Amount too small. Minimum is {} KES", config.transaction_limits.min_amount_kes));
    }
    
    if amount > config.transaction_limits.max_amount_kes {
        return Err(format!("Amount too large. Maximum is {} KES", config.transaction_limits.max_amount_kes));
    }
    
    if amount.is_nan() || amount.is_infinite() {
        return Err("Invalid amount".to_string());
    }
    
    Ok(())
}

/// Parse and validate amount from string
pub fn parse_amount(amount_str: &str) -> Result<f64, String> {
    let amount = amount_str.parse::<f64>()
        .map_err(|_| "Invalid amount format".to_string())?;
    
    is_valid_amount(amount)?;
    Ok(amount)
}

/// Sanitize user input (remove dangerous characters)
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '+' || *c == '*' || *c == ' ' || *c == '.')
        .collect()
}

/// Validate Bitcoin address (basic check)
pub fn is_valid_btc_address(address: &str) -> bool {
    // Basic validation - starts with bc1 (Bech32) or 1/3 (Legacy/P2SH)
    (address.starts_with("bc1") && address.len() >= 42 && address.len() <= 62) ||
    (address.starts_with('1') && address.len() >= 26 && address.len() <= 35) ||
    (address.starts_with('3') && address.len() >= 26 && address.len() <= 35)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_numbers() {
        assert!(is_valid_phone("+256700123456"));
        assert!(is_valid_phone("+254712345678"));
        assert!(!is_valid_phone("256700123456")); // Missing +
        assert!(!is_valid_phone("+256")); // Too short
    }

    #[test]
    fn test_amount_validation() {
        assert!(is_valid_amount(100.0).is_ok());
        assert!(is_valid_amount(5.0).is_err()); // Too small
        assert!(is_valid_amount(2_000_000.0).is_err()); // Too large
    }

    #[test]
    fn test_parse_amount() {
        assert_eq!(parse_amount("100").unwrap(), 100.0);
        assert_eq!(parse_amount("100.50").unwrap(), 100.5);
        assert!(parse_amount("abc").is_err());
        assert!(parse_amount("5").is_err()); // Too small
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("1*2*3"), "123");
        assert_eq!(sanitize_input("hello<script>"), "helloscript");
        assert_eq!(sanitize_input("+256700"), "+256700");
    }

    #[test]
    fn test_btc_address_validation() {
        assert!(is_valid_btc_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
        assert!(is_valid_btc_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"));
        assert!(!is_valid_btc_address("invalid"));
    }
}
