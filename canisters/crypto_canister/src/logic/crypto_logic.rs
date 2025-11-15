/// Pure business logic for cryptocurrency operations
/// No I/O, no async, fully testable

/// Validates crypto address format
pub fn validate_crypto_address(address: &str, crypto_type: &str) -> Result<(), String> {
    if address.is_empty() {
        return Err("Address cannot be empty".to_string());
    }
    
    match crypto_type {
        "BTC" | "Bitcoin" | "CkBTC" => {
            if address.len() < 26 || address.len() > 62 {
                return Err("Invalid Bitcoin address length".to_string());
            }
        }
        "USDC" | "Ethereum" | "CkUSDC" => {
            // ckUSDC on ICP uses IC Principal addresses, not Ethereum addresses
            // IC Principal format: base32 with hyphens, ending in "-cai"
            // Example: rrkah-fqaaa-aaaaa-aaaaq-cai
            if address.len() < 10 || address.len() > 63 {
                return Err("Invalid USDC address length".to_string());
            }

            // Must end with "-cai" for IC Principal or start with "0x" for Ethereum
            if !address.ends_with("-cai") && !address.starts_with("0x") {
                return Err("Invalid USDC address format (must be IC Principal ending in '-cai')".to_string());
            }

            // Additional validation for IC Principal format
            if address.ends_with("-cai") {
                // Check format: lowercase alphanumeric and hyphens only
                if !address.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
                    return Err("Invalid IC Principal address format".to_string());
                }

                // Should have multiple segments separated by hyphens
                let parts: Vec<&str> = address.split('-').collect();
                if parts.len() < 3 {
                    return Err("Invalid IC Principal address format".to_string());
                }
            } else if address.starts_with("0x") {
                // Ethereum address validation: must be exactly 42 characters (0x + 40 hex digits)
                if address.len() != 42 {
                    return Err("Invalid Ethereum address length (must be 42 characters)".to_string());
                }
                // Validate hex characters
                if !address[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err("Invalid Ethereum address format (must contain only hex digits)".to_string());
                }
            }
        }
        _ => return Err(format!("Unsupported crypto type: {}", crypto_type)),
    }
    
    Ok(())
}

/// Validates crypto amount is positive
pub fn validate_crypto_amount_positive(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Crypto amount must be greater than 0".to_string());
    }
    Ok(())
}

/// Validates sufficient crypto balance
pub fn validate_sufficient_crypto_balance(balance: u64, amount: u64) -> Result<(), String> {
    if balance < amount {
        return Err(format!("Insufficient crypto balance. Have: {}, Need: {}", balance, amount));
    }
    Ok(())
}

/// Validates crypto amount can be calculated from fiat
/// NOTE: Actual calculation happens in exchange_rate service (async)
/// This is just validation logic
#[allow(dead_code)]
pub fn validate_crypto_calculation_inputs(
    fiat_amount: u64,
    crypto_type: &str,
) -> Result<(), String> {
    if fiat_amount == 0 {
        return Err("Fiat amount must be greater than 0".to_string());
    }
    
    match crypto_type {
        "CkBTC" | "BTC" | "CkUSDC" | "USDC" => Ok(()),
        _ => Err(format!("Unsupported crypto type: {}", crypto_type)),
    }
}

/// Validates fiat amount for crypto purchase
pub fn validate_fiat_amount_for_crypto(fiat_amount: u64) -> Result<(), String> {
    if fiat_amount == 0 {
        return Err("Fiat amount must be greater than 0".to_string());
    }
    Ok(())
}

/// Calculates new crypto balance after deduction
#[allow(dead_code)]
pub fn calculate_crypto_balance_deduction(balance: u64, amount: u64) -> Result<u64, String> {
    balance.checked_sub(amount)
        .ok_or_else(|| "Crypto balance calculation would underflow".to_string())
}

/// Calculates new crypto balance after addition
#[allow(dead_code)]
pub fn calculate_crypto_balance_addition(balance: u64, amount: u64) -> Result<u64, String> {
    balance.checked_add(amount)
        .ok_or_else(|| "Crypto balance calculation would overflow".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Crypto Address Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_crypto_address_bitcoin_valid() {
        assert!(validate_crypto_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", "BTC").is_ok());
        assert!(validate_crypto_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq", "Bitcoin").is_ok());
        assert!(validate_crypto_address("3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy", "CkBTC").is_ok());
    }

    #[test]
    fn test_validate_crypto_address_bitcoin_invalid_length() {
        let result = validate_crypto_address("short", "BTC");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid Bitcoin address length"));
        
        let too_long = "a".repeat(63);
        let result = validate_crypto_address(&too_long, "BTC");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_crypto_address_ethereum_valid() {
        assert!(validate_crypto_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0", "USDC").is_ok());
        assert!(validate_crypto_address("0x0000000000000000000000000000000000000000", "Ethereum").is_ok());
        assert!(validate_crypto_address("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF", "CkUSDC").is_ok());
    }

    #[test]
    fn test_validate_crypto_address_ethereum_invalid() {
        // Missing 0x prefix
        let result = validate_crypto_address("742d35Cc6634C0532925a3b844Bc9e7595f0bEb0", "USDC");
        assert!(result.is_err());
        
        // Wrong length
        let result = validate_crypto_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", "USDC");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_crypto_address_empty() {
        let result = validate_crypto_address("", "BTC");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Address cannot be empty");
    }

    #[test]
    fn test_validate_crypto_address_unsupported_type() {
        let result = validate_crypto_address("someaddress", "DOGE");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported crypto type"));
    }

    // ============================================================================
    // Crypto Amount Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_crypto_amount_positive_valid() {
        assert!(validate_crypto_amount_positive(1).is_ok());
        assert!(validate_crypto_amount_positive(1000).is_ok());
        assert!(validate_crypto_amount_positive(u64::MAX).is_ok());
    }

    #[test]
    fn test_validate_crypto_amount_positive_zero() {
        let result = validate_crypto_amount_positive(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Crypto amount must be greater than 0");
    }

    // ============================================================================
    // Crypto Balance Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_sufficient_crypto_balance_ok() {
        assert!(validate_sufficient_crypto_balance(1000, 500).is_ok());
        assert!(validate_sufficient_crypto_balance(1000, 1000).is_ok());
    }

    #[test]
    fn test_validate_sufficient_crypto_balance_insufficient() {
        let result = validate_sufficient_crypto_balance(500, 1000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient crypto balance"));
    }

    // ============================================================================
    // Crypto Calculation Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_crypto_calculation_inputs_valid() {
        assert!(validate_crypto_calculation_inputs(100_000_000, "CkBTC").is_ok());
        assert!(validate_crypto_calculation_inputs(1000, "CkUSDC").is_ok());
        assert!(validate_crypto_calculation_inputs(1, "BTC").is_ok());
        assert!(validate_crypto_calculation_inputs(1, "USDC").is_ok());
    }

    #[test]
    fn test_validate_crypto_calculation_inputs_zero() {
        let result = validate_crypto_calculation_inputs(0, "CkBTC");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Fiat amount must be greater than 0");
    }

    #[test]
    fn test_validate_crypto_calculation_inputs_unsupported() {
        let result = validate_crypto_calculation_inputs(1000, "DOGE");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported crypto type"));
    }

    // ============================================================================
    // Fiat Amount Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_fiat_amount_for_crypto_valid() {
        assert!(validate_fiat_amount_for_crypto(1).is_ok());
        assert!(validate_fiat_amount_for_crypto(1000).is_ok());
        assert!(validate_fiat_amount_for_crypto(u64::MAX).is_ok());
    }

    #[test]
    fn test_validate_fiat_amount_for_crypto_zero() {
        let result = validate_fiat_amount_for_crypto(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Fiat amount must be greater than 0");
    }

    // ============================================================================
    // Crypto Balance Calculation Tests
    // ============================================================================

    #[test]
    fn test_calculate_crypto_balance_deduction_valid() {
        assert_eq!(calculate_crypto_balance_deduction(1000, 500).unwrap(), 500);
        assert_eq!(calculate_crypto_balance_deduction(1000, 1000).unwrap(), 0);
    }

    #[test]
    fn test_calculate_crypto_balance_deduction_underflow() {
        let result = calculate_crypto_balance_deduction(500, 1000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("underflow"));
    }

    #[test]
    fn test_calculate_crypto_balance_addition_valid() {
        assert_eq!(calculate_crypto_balance_addition(1000, 500).unwrap(), 1500);
        assert_eq!(calculate_crypto_balance_addition(0, 1000).unwrap(), 1000);
    }

    #[test]
    fn test_calculate_crypto_balance_addition_overflow() {
        let result = calculate_crypto_balance_addition(u64::MAX, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    // ============================================================================
    // Combined Validation Tests (Real-world Scenarios)
    // ============================================================================

    #[test]
    fn test_valid_crypto_purchase_scenario() {
        let fiat_amount = 100_000_000u64;
        let crypto_type = "CkBTC";

        assert!(validate_fiat_amount_for_crypto(fiat_amount).is_ok());
        assert!(validate_crypto_calculation_inputs(fiat_amount, crypto_type).is_ok());
    }

    #[test]
    fn test_valid_crypto_send_scenario() {
        let amount = 1000u64;
        let balance = 5000u64;
        let address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0";

        assert!(validate_crypto_amount_positive(amount).is_ok());
        assert!(validate_sufficient_crypto_balance(balance, amount).is_ok());
        assert!(validate_crypto_address(address, "USDC").is_ok());

        let new_balance = calculate_crypto_balance_deduction(balance, amount).unwrap();
        assert_eq!(new_balance, 4000);
    }

    #[test]
    fn test_invalid_crypto_scenarios() {
        // Zero amount
        assert!(validate_crypto_amount_positive(0).is_err());

        // Insufficient balance
        assert!(validate_sufficient_crypto_balance(500, 1000).is_err());

        // Invalid address
        assert!(validate_crypto_address("invalid", "BTC").is_err());

        // Zero fiat
        assert!(validate_fiat_amount_for_crypto(0).is_err());
    }
}
