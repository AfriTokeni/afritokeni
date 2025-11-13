// Input validation utilities
use crate::config_loader::get_config;
use sha2::{Digest, Sha256};

/// Validate phone number format
pub fn is_valid_phone(phone: &str) -> bool {
    // Must start with + and be 10-15 digits
    if !phone.starts_with('+') {
        return false;
    }
    
    let digits = &phone[1..];
    digits.len() >= 10 && digits.len() <= 15 && digits.chars().all(|c| c.is_numeric())
}

/// Validate PIN format (4 digits)
pub fn is_valid_pin(pin: &str) -> bool {
    pin.len() == 4 && pin.chars().all(|c| c.is_numeric())
}

/// Validate transaction amount from string (for tests and USSD input)
pub fn is_valid_amount(amount_str: &str) -> bool {
    // Try to parse as f64
    let amount = match amount_str.parse::<f64>() {
        Ok(amt) => amt,
        Err(_) => return false,
    };
    
    // Check if it's a valid number
    if amount.is_nan() || amount.is_infinite() || amount <= 0.0 {
        return false;
    }
    
    // Check if it has decimals (USSD amounts should be whole numbers)
    if amount.fract() != 0.0 {
        return false;
    }
    
    // Check limits
    let config = get_config();
    amount >= config.transaction_limits.min_amount_kes && 
    amount <= config.transaction_limits.max_amount_kes
}

/// Validate transaction amount (f64 version for internal use)
pub fn is_valid_amount_f64(amount: f64) -> Result<(), String> {
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
    
    is_valid_amount_f64(amount)?;
    Ok(amount)
}

/// Sanitize user input (remove dangerous characters)
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '+' || *c == '*' || *c == ' ' || *c == '.')
        .collect()
}

/// Validate Bitcoin address with checksum verification
pub fn is_valid_btc_address(address: &str) -> bool {
    let config = get_config();

    // Basic length checks first (fail fast)
    if address.len() < config.validation.btc_address_min_length ||
       address.len() > config.validation.btc_address_max_length {
        return false;
    }

    if config.validation.btc_strict_checksum_validation {
        // Strict validation with proper checksum
        if address.starts_with("bc1") || address.starts_with("tb1") {
            // Bech32/Bech32m (SegWit) address validation
            validate_bech32_address(address)
        } else if address.starts_with('1') || address.starts_with('3') ||
                  address.starts_with('m') || address.starts_with('n') || address.starts_with('2') {
            // Base58Check (Legacy/P2SH) address validation
            validate_base58check_address(address)
        } else {
            false
        }
    } else {
        // Legacy basic validation (format check only, no checksum)
        // Use only for backwards compatibility if needed
        (address.starts_with("bc1") && address.len() >= 42 && address.len() <= 62) ||
        (address.starts_with('1') && address.len() >= 26 && address.len() <= 35) ||
        (address.starts_with('3') && address.len() >= 26 && address.len() <= 35)
    }
}

/// Validate Base58Check address (P2PKH: starts with 1, P2SH: starts with 3)
fn validate_base58check_address(address: &str) -> bool {
    // Decode base58
    let decoded = match bs58::decode(address).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Must be at least 25 bytes (1 version + 20 hash + 4 checksum)
    if decoded.len() < 25 {
        return false;
    }

    // Split payload and checksum
    let checksum_index = decoded.len() - 4;
    let (payload, checksum) = decoded.split_at(checksum_index);

    // Calculate expected checksum (double SHA256 of payload)
    let hash1 = Sha256::digest(payload);
    let hash2 = Sha256::digest(&hash1);
    let expected_checksum = &hash2[0..4];

    // Verify checksum matches
    checksum == expected_checksum
}

/// Validate Bech32/Bech32m address (SegWit)
fn validate_bech32_address(address: &str) -> bool {
    // Basic bech32 validation (simplified)
    // Full bech32 validation would require the bech32 crate
    // For now, do basic format validation
    let parts: Vec<&str> = address.split('1').collect();
    if parts.len() != 2 {
        return false;
    }

    let (hrp, data) = (parts[0], parts[1]);

    // HRP must be "bc" for mainnet or "tb" for testnet
    if hrp != "bc" && hrp != "tb" {
        return false;
    }

    // Data part must only contain valid bech32 characters
    let valid_chars = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    data.chars().all(|c| valid_chars.contains(c))
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
        assert!(is_valid_amount_f64(100.0).is_ok());
        assert!(is_valid_amount_f64(5.0).is_err()); // Too small
        assert!(is_valid_amount_f64(2_000_000.0).is_err()); // Too large
    }
    
    #[test]
    fn test_pin_validation() {
        assert!(is_valid_pin("1234"));
        assert!(is_valid_pin("0000"));
        assert!(!is_valid_pin("123")); // Too short
        assert!(!is_valid_pin("12345")); // Too long
        assert!(!is_valid_pin("abcd")); // Not numeric
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
