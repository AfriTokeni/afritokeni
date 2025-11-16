/// Pure presentation logic for send money flow
/// No I/O, no async, no IC calls - fully testable

use crate::logic::validation;

/// Represents the state of the send money flow
#[derive(Debug, Clone, PartialEq)]
pub enum SendMoneyStep {
    AskRecipient,
    AskAmount,
    ConfirmTransaction { recipient: String, amount: f64, fee: f64 },
    ExecuteTransfer,
}

/// Parse USSD input and determine current step
pub fn determine_step(text: &str) -> usize {
    let parts: Vec<&str> = text.split('*').collect();
    if parts.len() <= 2 {
        0
    } else {
        parts.len() - 2
    }
}

/// Extract recipient phone from USSD input
pub fn extract_recipient(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(2).map(|s| s.to_string())
}

/// Extract amount from USSD input
pub fn extract_amount(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(3).map(|s| s.to_string())
}

/// Extract PIN from USSD input
pub fn extract_pin(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(4).map(|s| s.to_string())
}

/// Calculate fee for transfer (0.5%)
pub fn calculate_transfer_fee(amount: f64) -> f64 {
    (amount * 0.005).round()
}

/// Calculate total required (amount + fee)
pub fn calculate_total_required(amount: f64) -> f64 {
    amount + calculate_transfer_fee(amount)
}

/// Validate send money step 1 (recipient phone)
pub fn validate_recipient_step(text: &str) -> Result<String, String> {
    let recipient = extract_recipient(text)
        .ok_or_else(|| "No recipient provided".to_string())?;
    
    validation::validate_phone_format(&recipient)?;
    Ok(recipient)
}

/// Validate send money step 2 (amount)
pub fn validate_amount_step(text: &str) -> Result<f64, String> {
    let amount_str = extract_amount(text)
        .ok_or_else(|| "No amount provided".to_string())?;
    
    validation::parse_and_validate_amount(&amount_str)
}

/// Validate send money step 3 (PIN)
pub fn validate_pin_step(text: &str) -> Result<String, String> {
    let pin = extract_pin(text)
        .ok_or_else(|| "No PIN provided".to_string())?;
    
    validation::validate_pin_format(&pin)?;
    Ok(pin)
}

/// Check if user has sufficient balance
pub fn check_sufficient_balance(balance: u64, amount: f64) -> Result<(), String> {
    let balance_f64 = balance as f64 / 100.0;
    let total_required = calculate_total_required(amount);
    
    if balance_f64 < total_required {
        return Err(format!(
            "Insufficient balance. You have {:.2}, need {:.2}",
            balance_f64, total_required
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_step() {
        assert_eq!(determine_step("1*1"), 0);
        assert_eq!(determine_step("1*1*+256700000002"), 1);
        assert_eq!(determine_step("1*1*+256700000002*10000"), 2);
        assert_eq!(determine_step("1*1*+256700000002*10000*1234"), 3);
    }

    #[test]
    fn test_extract_recipient() {
        assert_eq!(extract_recipient("1*1*+256700000002"), Some("+256700000002".to_string()));
        assert_eq!(extract_recipient("1*1"), None);
    }

    #[test]
    fn test_extract_amount() {
        assert_eq!(extract_amount("1*1*+256700000002*10000"), Some("10000".to_string()));
        assert_eq!(extract_amount("1*1*+256700000002"), None);
    }

    #[test]
    fn test_extract_pin() {
        assert_eq!(extract_pin("1*1*+256700000002*10000*1234"), Some("1234".to_string()));
        assert_eq!(extract_pin("1*1*+256700000002*10000"), None);
    }

    #[test]
    fn test_calculate_transfer_fee() {
        assert_eq!(calculate_transfer_fee(10000.0), 50.0);
        assert_eq!(calculate_transfer_fee(5000.0), 25.0);
        assert_eq!(calculate_transfer_fee(100.0), 1.0);
    }

    #[test]
    fn test_calculate_total_required() {
        assert_eq!(calculate_total_required(10000.0), 10050.0);
        assert_eq!(calculate_total_required(5000.0), 5025.0);
    }

    #[test]
    fn test_validate_recipient_step_valid() {
        let result = validate_recipient_step("1*1*+256700000002");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "+256700000002");
    }

    #[test]
    fn test_validate_recipient_step_invalid() {
        assert!(validate_recipient_step("1*1*invalid").is_err());
        assert!(validate_recipient_step("1*1*256700000002").is_err()); // Missing +
        assert!(validate_recipient_step("1*1").is_err()); // No recipient
    }

    #[test]
    fn test_validate_amount_step_valid() {
        let result = validate_amount_step("1*1*+256700000002*10000");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10000.0);
    }

    #[test]
    fn test_validate_amount_step_invalid() {
        assert!(validate_amount_step("1*1*+256700000002*abc").is_err());
        assert!(validate_amount_step("1*1*+256700000002*0").is_err());
        assert!(validate_amount_step("1*1*+256700000002").is_err()); // No amount
    }

    #[test]
    fn test_validate_pin_step_valid() {
        let result = validate_pin_step("1*1*+256700000002*10000*1234");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1234");
    }

    #[test]
    fn test_validate_pin_step_invalid() {
        assert!(validate_pin_step("1*1*+256700000002*10000*123").is_err()); // Too short
        assert!(validate_pin_step("1*1*+256700000002*10000*abcd").is_err()); // Not digits
        assert!(validate_pin_step("1*1*+256700000002*10000").is_err()); // No PIN
    }

    #[test]
    fn test_check_sufficient_balance_ok() {
        // Balance: 20000 UGX (2000000 cents), Amount: 10000 UGX, Fee: 50 UGX
        assert!(check_sufficient_balance(2_000_000, 10000.0).is_ok());
    }

    #[test]
    fn test_check_sufficient_balance_insufficient() {
        // Balance: 5000 UGX (500000 cents), Amount: 10000 UGX, Fee: 50 UGX
        assert!(check_sufficient_balance(500_000, 10000.0).is_err());
    }

    #[test]
    fn test_check_sufficient_balance_exact() {
        // Balance: 10050 UGX (1005000 cents), Amount: 10000 UGX, Fee: 50 UGX
        assert!(check_sufficient_balance(1_005_000, 10000.0).is_ok());
    }
}
