/// Pure business logic for money transfer operations
/// No I/O, no async, fully testable

/// Validates that amount is positive
pub fn validate_amount_positive(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    Ok(())
}

/// Validates that sender has sufficient balance
pub fn validate_sufficient_balance(balance: u64, amount: u64) -> Result<(), String> {
    if balance < amount {
        return Err(format!("Insufficient balance. Have: {}, Need: {}", balance, amount));
    }
    Ok(())
}

/// Validates that sender and recipient are different
pub fn validate_not_self_transfer(from_id: &str, to_id: &str) -> Result<(), String> {
    if from_id == to_id {
        return Err("Cannot transfer to yourself".to_string());
    }
    Ok(())
}

/// Validates currency code format (3 uppercase letters)
pub fn validate_currency_code(currency: &str) -> Result<(), String> {
    if currency.is_empty() {
        return Err("Currency code cannot be empty".to_string());
    }
    if currency.len() != 3 {
        return Err("Currency code must be exactly 3 characters".to_string());
    }
    if !currency.chars().all(|c| c.is_ascii_uppercase()) {
        return Err("Currency code must be uppercase letters".to_string());
    }
    Ok(())
}

/// Validates amount is within reasonable limits
pub fn validate_amount_within_limit(amount: u64, max_limit: u64) -> Result<(), String> {
    if amount > max_limit {
        return Err(format!("Amount {} exceeds maximum limit {}", amount, max_limit));
    }
    Ok(())
}

/// Calculates new balance after deduction
pub fn calculate_new_balance(current_balance: u64, amount: u64) -> Result<u64, String> {
    current_balance.checked_sub(amount)
        .ok_or_else(|| "Balance calculation would underflow".to_string())
}

/// Calculates new balance after addition
pub fn calculate_balance_addition(current_balance: u64, amount: u64) -> Result<u64, String> {
    current_balance.checked_add(amount)
        .ok_or_else(|| "Balance calculation would overflow".to_string())
}

/// Generates transaction ID from timestamp
pub fn generate_transaction_id(timestamp: u64) -> String {
    format!("tx_{}", timestamp)
}

/// Validates identifier is not empty
pub fn validate_identifier_not_empty(identifier: &str, field_name: &str) -> Result<(), String> {
    if identifier.is_empty() {
        return Err(format!("{} cannot be empty", field_name));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Amount Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_amount_positive_valid() {
        assert!(validate_amount_positive(1).is_ok());
        assert!(validate_amount_positive(100).is_ok());
        assert!(validate_amount_positive(u64::MAX).is_ok());
    }

    #[test]
    fn test_validate_amount_positive_zero() {
        let result = validate_amount_positive(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Amount must be greater than 0");
    }

    // ============================================================================
    // Balance Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_sufficient_balance_ok() {
        assert!(validate_sufficient_balance(1000, 500).is_ok());
        assert!(validate_sufficient_balance(1000, 1000).is_ok());
    }

    #[test]
    fn test_validate_sufficient_balance_insufficient() {
        let result = validate_sufficient_balance(500, 1000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_validate_sufficient_balance_zero() {
        let result = validate_sufficient_balance(0, 100);
        assert!(result.is_err());
    }

    // ============================================================================
    // Self-Transfer Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_not_self_transfer_different() {
        assert!(validate_not_self_transfer("user1", "user2").is_ok());
        assert!(validate_not_self_transfer("+254712345678", "+254700000000").is_ok());
    }

    #[test]
    fn test_validate_not_self_transfer_same() {
        let result = validate_not_self_transfer("user1", "user1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot transfer to yourself");
    }

    #[test]
    fn test_validate_not_self_transfer_case_sensitive() {
        // Should be case-sensitive
        assert!(validate_not_self_transfer("User1", "user1").is_ok());
    }

    // ============================================================================
    // Currency Code Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_currency_code_valid() {
        assert!(validate_currency_code("USD").is_ok());
        assert!(validate_currency_code("KES").is_ok());
        assert!(validate_currency_code("UGX").is_ok());
        assert!(validate_currency_code("GHS").is_ok());
    }

    #[test]
    fn test_validate_currency_code_wrong_length() {
        let result = validate_currency_code("US");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Currency code must be exactly 3 characters");

        let result = validate_currency_code("USDT");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_currency_code_lowercase() {
        let result = validate_currency_code("usd");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Currency code must be uppercase letters");
    }

    #[test]
    fn test_validate_currency_code_empty() {
        let result = validate_currency_code("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Currency code cannot be empty");
    }

    #[test]
    fn test_validate_currency_code_with_numbers() {
        let result = validate_currency_code("US1");
        assert!(result.is_err());
    }

    // ============================================================================
    // Amount Limit Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_amount_within_limit_ok() {
        assert!(validate_amount_within_limit(100, 1000).is_ok());
        assert!(validate_amount_within_limit(1000, 1000).is_ok());
    }

    #[test]
    fn test_validate_amount_within_limit_exceeds() {
        let result = validate_amount_within_limit(1001, 1000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum limit"));
    }

    // ============================================================================
    // Balance Calculation Tests
    // ============================================================================

    #[test]
    fn test_calculate_new_balance_valid() {
        assert_eq!(calculate_new_balance(1000, 500).unwrap(), 500);
        assert_eq!(calculate_new_balance(1000, 1000).unwrap(), 0);
        assert_eq!(calculate_new_balance(u64::MAX, 1).unwrap(), u64::MAX - 1);
    }

    #[test]
    fn test_calculate_new_balance_underflow() {
        let result = calculate_new_balance(500, 1000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("underflow"));
    }

    #[test]
    fn test_calculate_balance_addition_valid() {
        assert_eq!(calculate_balance_addition(1000, 500).unwrap(), 1500);
        assert_eq!(calculate_balance_addition(0, 1000).unwrap(), 1000);
    }

    #[test]
    fn test_calculate_balance_addition_overflow() {
        let result = calculate_balance_addition(u64::MAX, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    // ============================================================================
    // Transaction ID Generation Tests
    // ============================================================================

    #[test]
    fn test_generate_transaction_id_format() {
        let tx_id = generate_transaction_id(1234567890);
        assert_eq!(tx_id, "tx_1234567890");
    }

    #[test]
    fn test_generate_transaction_id_unique() {
        let tx_id1 = generate_transaction_id(1000);
        let tx_id2 = generate_transaction_id(2000);
        assert_ne!(tx_id1, tx_id2);
    }

    #[test]
    fn test_generate_transaction_id_deterministic() {
        let tx_id1 = generate_transaction_id(1000);
        let tx_id2 = generate_transaction_id(1000);
        assert_eq!(tx_id1, tx_id2);
    }

    // ============================================================================
    // Identifier Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_identifier_not_empty_valid() {
        assert!(validate_identifier_not_empty("user123", "User ID").is_ok());
        assert!(validate_identifier_not_empty("+254712345678", "Phone").is_ok());
    }

    #[test]
    fn test_validate_identifier_not_empty_invalid() {
        let result = validate_identifier_not_empty("", "User ID");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User ID cannot be empty");
    }

    // ============================================================================
    // Combined Validation Tests (Real-world Scenarios)
    // ============================================================================

    #[test]
    fn test_valid_transfer_scenario() {
        // Valid transfer: 500 from balance of 1000
        let amount = 500u64;
        let balance = 1000u64;
        let from_id = "user1";
        let to_id = "user2";
        let currency = "KES";

        assert!(validate_amount_positive(amount).is_ok());
        assert!(validate_sufficient_balance(balance, amount).is_ok());
        assert!(validate_not_self_transfer(from_id, to_id).is_ok());
        assert!(validate_currency_code(currency).is_ok());

        let new_balance = calculate_new_balance(balance, amount).unwrap();
        assert_eq!(new_balance, 500);
    }

    #[test]
    fn test_invalid_transfer_scenarios() {
        // Zero amount
        assert!(validate_amount_positive(0).is_err());

        // Insufficient balance
        assert!(validate_sufficient_balance(500, 1000).is_err());

        // Self transfer
        assert!(validate_not_self_transfer("user1", "user1").is_err());

        // Invalid currency
        assert!(validate_currency_code("usd").is_err());
    }

    #[test]
    fn test_edge_case_exact_balance() {
        let balance = 1000u64;
        let amount = 1000u64;

        assert!(validate_sufficient_balance(balance, amount).is_ok());
        let new_balance = calculate_new_balance(balance, amount).unwrap();
        assert_eq!(new_balance, 0);
    }
}
