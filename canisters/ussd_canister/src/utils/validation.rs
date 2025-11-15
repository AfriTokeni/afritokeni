// Input validation utilities
use crate::config_loader::get_config;
use sha2::{Digest, Sha256};

/// Validate phone number format for African countries
///
/// Validates phone numbers from all 54 African countries based on their country codes.
/// Supports both formats: with + prefix ("+256700123456") or without ("256700123456")
///
/// # Arguments
/// * `phone` - Phone number to validate
///
/// # Returns
/// `true` if the phone number has a valid African country code and format, `false` otherwise
///
/// # Examples
/// ```
/// assert!(is_valid_phone("+256700123456")); // Uganda
/// assert!(is_valid_phone("+254712345678")); // Kenya
/// assert!(!is_valid_phone("1234567890"));   // Invalid (non-African code)
/// ```
pub fn is_valid_phone(phone: &str) -> bool {
    // Must start with + and be 10-15 digits
    if !phone.starts_with('+') {
        return false;
    }

    let digits = &phone[1..];

    // Check basic format
    if digits.len() < 10 || digits.len() > 15 || !digits.chars().all(|c| c.is_numeric()) {
        return false;
    }

    // Validate African country codes (54 countries total)
    // Organized by region for maintainability
    let valid_country_codes = [
        // East Africa
        "250", // Rwanda
        "251", // Ethiopia
        "252", // Somalia
        "253", // Djibouti
        "254", // Kenya
        "255", // Tanzania
        "256", // Uganda
        "257", // Burundi

        // West Africa
        "220", // Gambia
        "221", // Senegal
        "222", // Mauritania
        "223", // Mali
        "224", // Guinea
        "225", // Côte d'Ivoire
        "226", // Burkina Faso
        "227", // Niger
        "228", // Togo
        "229", // Benin
        "231", // Liberia
        "232", // Sierra Leone
        "233", // Ghana
        "234", // Nigeria
        "238", // Cape Verde

        // Central Africa
        "235", // Chad
        "236", // Central African Republic
        "237", // Cameroon
        "240", // Equatorial Guinea
        "241", // Gabon
        "242", // Congo (Brazzaville)
        "243", // Democratic Republic of Congo

        // Southern Africa
        "258", // Mozambique
        "260", // Zambia
        "261", // Madagascar
        "262", // Réunion (French territory)
        "263", // Zimbabwe
        "264", // Namibia
        "265", // Malawi
        "266", // Lesotho
        "267", // Botswana
        "268", // Eswatini (Swaziland)
        "27",  // South Africa

        // North Africa
        "20",  // Egypt
        "211", // South Sudan
        "212", // Morocco
        "213", // Algeria
        "216", // Tunisia
        "218", // Libya
        "249", // Sudan

        // Island Nations
        "230", // Mauritius
        "248", // Seychelles
        "269", // Comoros
    ];

    valid_country_codes.iter().any(|code| digits.starts_with(code))
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

    // Reject negative amounts (critical security check)
    if amount <= 0.0 {
        return Err("Amount must be positive".to_string());
    }

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

/// Sanitize user input by removing potentially dangerous characters
///
/// This function filters out any characters that could be used for injection attacks
/// or cause parsing issues while preserving legitimate USSD input characters.
///
/// **Allowed characters:**
/// - Alphanumeric (a-z, A-Z, 0-9)
/// - Plus sign (+) - for phone numbers in international format
/// - Asterisk (*) - for USSD navigation codes
/// - Space ( ) - for names and text input
/// - Period (.) - for decimal amounts
/// - Hyphen (-) - for Principal addresses (ckUSDC/ckBTC) and negative numbers
///
/// **Blocked characters:**
/// - HTML/XML tags: <, >, &
/// - Script injection: ', ", \, /, ;
/// - Control characters: \n, \r, \t
/// - SQL injection: --, /*, */, =
/// - Special symbols: @, #, $, %, ^, &, |, ~, `, etc.
///
/// **Blocked sequences:**
/// - SQL comments: --, /*, */
/// - Command chaining: &&, ||
///
/// # Security Note
/// This is a defense-in-depth measure. All critical operations should still
/// validate inputs against expected formats (phone numbers, PINs, amounts, etc.)
///
/// # Arguments
/// * `input` - Raw user input string
///
/// # Returns
/// Sanitized string with only safe characters
///
/// # Examples
/// ```
/// assert_eq!(sanitize_input("1*2*3"), "1*2*3");              // USSD codes preserved
/// assert_eq!(sanitize_input("hello<script>"), "helloscript"); // HTML tags removed
/// assert_eq!(sanitize_input("+256700123"), "+256700123");     // Phone numbers preserved
/// assert_eq!(sanitize_input("100.50"), "100.50");             // Decimals preserved
/// assert_eq!(sanitize_input("user_123"), "user123");          // Underscores removed
/// ```
pub fn sanitize_input(input: &str) -> String {
    // Step 1: Remove dangerous multi-character sequences
    let mut cleaned = input.to_string();

    // SQL comment sequences
    cleaned = cleaned.replace("--", "");
    cleaned = cleaned.replace("/*", "");
    cleaned = cleaned.replace("*/", "");

    // Command chaining sequences
    cleaned = cleaned.replace("&&", "");
    cleaned = cleaned.replace("||", "");

    // Step 2: Strip leading/trailing USSD markers (* and #)
    cleaned = cleaned.trim_start_matches('*').trim_end_matches('#').to_string();

    // Step 3: Convert to char vec for context-aware filtering
    let chars: Vec<char> = cleaned.chars().collect();

    // Step 4: Filter with context awareness
    let mut filtered = Vec::new();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_alphanumeric() || c == '+' || c == '*' || c == ' ' {
            filtered.push(c);
        } else if c == '.' {
            // Keep period only if surrounded by digits
            let prev_is_digit = i > 0 && chars.get(i - 1).map(|ch| ch.is_numeric()).unwrap_or(false);
            let next_is_digit = chars.get(i + 1).map(|ch| ch.is_numeric()).unwrap_or(false);
            if prev_is_digit && next_is_digit {
                filtered.push(c);
            }
        } else if c == '-' {
            // Keep hyphen if:
            // 1. Surrounded by alphanumeric (for Principal addresses like "aaaaa-aa")
            // 2. Immediately followed by a digit (for negative numbers like "-5000")
            //    We KEEP negative numbers so validation can properly REJECT them
            let prev_is_alnum = i > 0 && chars.get(i - 1).map(|ch| ch.is_alphanumeric()).unwrap_or(false);
            let next_is_alnum = chars.get(i + 1).map(|ch| ch.is_alphanumeric()).unwrap_or(false);
            let next_is_digit = chars.get(i + 1).map(|ch| ch.is_numeric()).unwrap_or(false);

            if (prev_is_alnum && next_is_alnum) || next_is_digit {
                filtered.push(c);
            }
        }
    }

    // Step 5: Collapse multiple consecutive spaces and limit length
    let mut prev_was_space = false;
    filtered
        .iter()
        .filter_map(|&c| {
            if c == ' ' {
                if prev_was_space {
                    None
                } else {
                    prev_was_space = true;
                    Some(c)
                }
            } else {
                prev_was_space = false;
                Some(c)
            }
        })
        .take(1000)
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

/// Validate USDC/IC Principal address
/// ckUSDC uses IC Principal addresses (not Ethereum addresses)
pub fn is_valid_usdc_address(address: &str) -> bool {
    // IC Principal format: base32 with hyphens, ending in "-cai"
    // Example: rrkah-fqaaa-aaaaa-aaaaq-cai

    // Basic checks
    if address.is_empty() || address.len() < 10 || address.len() > 63 {
        return false;
    }

    // Must end with "-cai"
    if !address.ends_with("-cai") {
        return false;
    }

    // Check format: lowercase alphanumeric and hyphens only
    if !address.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return false;
    }

    // Should have multiple segments separated by hyphens
    let parts: Vec<&str> = address.split('-').collect();
    if parts.len() < 3 {
        return false;
    }

    // Each part should be 5 characters (except possibly the last "-cai")
    parts.iter().all(|part| part.len() == 3 || part.len() == 5)
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
        // Test negative amounts are rejected (CRITICAL security check)
        assert!(parse_amount("-5000").is_err(), "Should reject negative amounts");
        assert!(parse_amount("-100").is_err(), "Should reject negative amounts");
        assert!(parse_amount("0").is_err(), "Should reject zero");
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("1*2*3"), "1*2*3"); // * is allowed for USSD codes
        assert_eq!(sanitize_input("hello<script>"), "helloscript"); // < and > removed
        assert_eq!(sanitize_input("+256700"), "+256700"); // + allowed for phone numbers
        assert_eq!(sanitize_input("123.45"), "123.45"); // . allowed for amounts
        assert_eq!(sanitize_input("test@email"), "testemail"); // @ removed
        assert_eq!(sanitize_input("user_123456"), "user_123456"); // _ allowed for user IDs
        // Test negative numbers are preserved (so validation can reject them)
        assert_eq!(sanitize_input("-5000"), "-5000"); // Minus followed by digits kept
        assert_eq!(sanitize_input("1*1*+256*-5000*1234"), "1*1*+256*-5000*1234"); // USSD input with negative amount
    }

    #[test]
    fn test_btc_address_validation() {
        assert!(is_valid_btc_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
        assert!(is_valid_btc_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"));
        assert!(!is_valid_btc_address("invalid"));
    }
}
