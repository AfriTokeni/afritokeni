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
pub fn validate_sufficient_balance(balance: u64, amount: u64, fee: u64) -> Result<(), String> {
    let total_required = amount.checked_add(fee)
        .ok_or_else(|| "Amount + fee would overflow".to_string())?;
    
    if balance < total_required {
        return Err(format!(
            "Insufficient balance. Have: {}, Need: {} (amount: {} + fee: {})",
            balance, total_required, amount, fee
        ));
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

/// Validates identifier is not empty
pub fn validate_identifier_not_empty(identifier: &str, field_name: &str) -> Result<(), String> {
    if identifier.is_empty() {
        return Err(format!("{} cannot be empty", field_name));
    }
    Ok(())
}

// Removed validate_currency_match() - Not needed as wallet_canister only handles same-currency transfers.
// Cross-currency transfers will be handled by a future exchange service.

/// Calculates new balance after deduction
pub fn calculate_new_balance(current_balance: u64, amount: u64) -> Result<u64, String> {
    current_balance
        .checked_sub(amount)
        .ok_or_else(|| "Balance calculation would underflow".to_string())
}

/// Calculates new balance after addition
pub fn calculate_balance_addition(current_balance: u64, amount: u64) -> Result<u64, String> {
    current_balance
        .checked_add(amount)
        .ok_or_else(|| "Balance calculation would overflow".to_string())
}

/// Generates transaction ID from timestamp
pub fn generate_transaction_id(timestamp: u64) -> String {
    format!("tx_{}", timestamp)
}

/// Calculate fee from amount in basis points
pub fn calculate_fee(amount: u64, fee_basis_points: u64) -> Result<u64, String> {
    if fee_basis_points > 10000 {
        return Err("Fee basis points cannot exceed 10000 (100%)".to_string());
    }
    
    // Fee = (amount * basis_points) / 10000
    let fee = amount
        .checked_mul(fee_basis_points)
        .ok_or_else(|| "Fee calculation would overflow".to_string())?
        .checked_div(10000)
        .ok_or_else(|| "Fee calculation division error".to_string())?;
    
    Ok(fee)
}

// Removed calculate_agent_commission() - Agent commission tracking is handled by agent_canister.
// This function was never used in wallet_canister and belongs in the agent domain.

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
        assert!(validate_sufficient_balance(1000, 500, 50).is_ok());
        assert!(validate_sufficient_balance(1000, 900, 100).is_ok());
    }

    #[test]
    fn test_validate_sufficient_balance_insufficient() {
        let result = validate_sufficient_balance(500, 400, 200);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_validate_sufficient_balance_exact() {
        assert!(validate_sufficient_balance(1000, 950, 50).is_ok());
    }

    #[test]
    fn test_validate_sufficient_balance_overflow() {
        let result = validate_sufficient_balance(1000, u64::MAX, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
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

    // Currency validation tests removed - function was unused and has been removed

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

    // ============================================================================
    // Fee Calculation Tests
    // ============================================================================

    #[test]
    fn test_calculate_fee_valid() {
        // 0.5% fee (50 basis points)
        assert_eq!(calculate_fee(100000, 50).unwrap(), 500);
        
        // 1% fee (100 basis points)
        assert_eq!(calculate_fee(100000, 100).unwrap(), 1000);
        
        // 10% fee (1000 basis points)
        assert_eq!(calculate_fee(100000, 1000).unwrap(), 10000);
    }

    #[test]
    fn test_calculate_fee_zero() {
        assert_eq!(calculate_fee(100000, 0).unwrap(), 0);
    }

    #[test]
    fn test_calculate_fee_max() {
        // 100% fee (10000 basis points)
        assert_eq!(calculate_fee(100000, 10000).unwrap(), 100000);
    }

    #[test]
    fn test_calculate_fee_invalid_bps() {
        let result = calculate_fee(100000, 10001);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot exceed 10000"));
    }

    #[test]
    fn test_calculate_fee_overflow() {
        let result = calculate_fee(u64::MAX, 5000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    // Agent commission tests removed - function was unused and has been removed

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
        let amount = 100000u64;
        let balance = 150000u64;
        let fee = calculate_fee(amount, 50).unwrap(); // 0.5% = 500
        let from_id = "user1";
        let to_id = "user2";

        assert!(validate_amount_positive(amount).is_ok());
        assert!(validate_sufficient_balance(balance, amount, fee).is_ok());
        assert!(validate_not_self_transfer(from_id, to_id).is_ok());

        let new_balance = calculate_new_balance(balance, amount + fee).unwrap();
        assert_eq!(new_balance, 49500);
    }

    #[test]
    fn test_invalid_transfer_scenarios() {
        // Zero amount
        assert!(validate_amount_positive(0).is_err());

        // Insufficient balance
        assert!(validate_sufficient_balance(500, 1000, 100).is_err());

        // Self transfer
        assert!(validate_not_self_transfer("user1", "user1").is_err());
    }
}
