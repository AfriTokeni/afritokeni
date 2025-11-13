/// Pure presentation logic for agent flows (deposit/withdraw)
/// No I/O, no async, no IC calls - fully testable

use crate::logic::validation;

/// Parse USSD input and determine current step
pub fn determine_step(text: &str) -> usize {
    let parts: Vec<&str> = text.split('*').collect();
    if parts.len() <= 2 {
        0
    } else {
        parts.len() - 2
    }
}

/// Extract agent ID from USSD input
pub fn extract_agent_id(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(2).map(|s| s.to_string())
}

/// Extract amount from USSD input
pub fn extract_amount(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(3).map(|s| s.to_string())
}

/// Extract PIN from USSD input (for withdraw)
pub fn extract_pin(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(4).map(|s| s.to_string())
}

/// Calculate deposit fees (no platform fee, just agent commission)
pub fn calculate_deposit_fees(amount: u64) -> DepositFees {
    let agent_commission = (amount as f64 * 0.01).round() as u64; // 1%
    
    DepositFees {
        amount,
        agent_commission,
        net_amount: amount,
    }
}

/// Calculate withdrawal fees (0.5% platform + 10% agent)
pub fn calculate_withdrawal_fees(amount: u64) -> WithdrawalFees {
    let platform_fee = (amount as f64 * 0.005).round() as u64; // 0.5%
    let agent_fee = (amount as f64 * 0.10).round() as u64; // 10%
    let total_fees = platform_fee + agent_fee;
    let net_amount = amount.saturating_sub(total_fees);
    
    WithdrawalFees {
        amount,
        platform_fee,
        agent_fee,
        total_fees,
        net_amount,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DepositFees {
    pub amount: u64,
    pub agent_commission: u64,
    pub net_amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithdrawalFees {
    pub amount: u64,
    pub platform_fee: u64,
    pub agent_fee: u64,
    pub total_fees: u64,
    pub net_amount: u64,
}

/// Validate agent ID format (basic check)
pub fn validate_agent_id(agent_id: &str) -> Result<(), String> {
    if agent_id.is_empty() {
        return Err("Agent ID cannot be empty".to_string());
    }
    if agent_id.len() < 3 {
        return Err("Agent ID too short".to_string());
    }
    Ok(())
}

/// Validate deposit step 1 (agent ID)
pub fn validate_agent_id_step(text: &str) -> Result<String, String> {
    let agent_id = extract_agent_id(text)
        .ok_or_else(|| "No agent ID provided".to_string())?;
    
    validate_agent_id(&agent_id)?;
    Ok(agent_id)
}

/// Validate deposit/withdraw step 2 (amount)
pub fn validate_amount_step(text: &str) -> Result<f64, String> {
    let amount_str = extract_amount(text)
        .ok_or_else(|| "No amount provided".to_string())?;
    
    validation::parse_and_validate_amount(&amount_str)
}

/// Validate withdraw step 3 (PIN)
pub fn validate_pin_step(text: &str) -> Result<String, String> {
    let pin = extract_pin(text)
        .ok_or_else(|| "No PIN provided".to_string())?;
    
    validation::validate_pin_format(&pin)?;
    Ok(pin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_step() {
        assert_eq!(determine_step("1*3"), 0);
        assert_eq!(determine_step("1*3*AGENT001"), 1);
        assert_eq!(determine_step("1*3*AGENT001*10000"), 2);
        assert_eq!(determine_step("1*4*AGENT001*10000*1234"), 3);
    }

    #[test]
    fn test_extract_agent_id() {
        assert_eq!(extract_agent_id("1*3*AGENT001"), Some("AGENT001".to_string()));
        assert_eq!(extract_agent_id("1*3"), None);
    }

    #[test]
    fn test_extract_amount() {
        assert_eq!(extract_amount("1*3*AGENT001*10000"), Some("10000".to_string()));
        assert_eq!(extract_amount("1*3*AGENT001"), None);
    }

    #[test]
    fn test_extract_pin() {
        assert_eq!(extract_pin("1*4*AGENT001*10000*1234"), Some("1234".to_string()));
        assert_eq!(extract_pin("1*4*AGENT001*10000"), None);
    }

    #[test]
    fn test_calculate_deposit_fees() {
        let fees = calculate_deposit_fees(10000);
        assert_eq!(fees.amount, 10000);
        assert_eq!(fees.agent_commission, 100); // 1%
        assert_eq!(fees.net_amount, 10000);
    }

    #[test]
    fn test_calculate_withdrawal_fees() {
        let fees = calculate_withdrawal_fees(10000);
        assert_eq!(fees.amount, 10000);
        assert_eq!(fees.platform_fee, 50); // 0.5%
        assert_eq!(fees.agent_fee, 1000); // 10%
        assert_eq!(fees.total_fees, 1050);
        assert_eq!(fees.net_amount, 8950);
    }

    #[test]
    fn test_validate_agent_id_valid() {
        assert!(validate_agent_id("AGENT001").is_ok());
        assert!(validate_agent_id("ABC").is_ok());
    }

    #[test]
    fn test_validate_agent_id_invalid() {
        assert!(validate_agent_id("").is_err());
        assert!(validate_agent_id("AB").is_err());
    }

    #[test]
    fn test_validate_agent_id_step_valid() {
        let result = validate_agent_id_step("1*3*AGENT001");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "AGENT001");
    }

    #[test]
    fn test_validate_agent_id_step_invalid() {
        assert!(validate_agent_id_step("1*3*AB").is_err());
        assert!(validate_agent_id_step("1*3").is_err());
    }

    #[test]
    fn test_validate_amount_step_valid() {
        let result = validate_amount_step("1*3*AGENT001*10000");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10000.0);
    }

    #[test]
    fn test_validate_amount_step_invalid() {
        assert!(validate_amount_step("1*3*AGENT001*abc").is_err());
        assert!(validate_amount_step("1*3*AGENT001*0").is_err());
        assert!(validate_amount_step("1*3*AGENT001").is_err());
    }

    #[test]
    fn test_validate_pin_step_valid() {
        let result = validate_pin_step("1*4*AGENT001*10000*1234");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1234");
    }

    #[test]
    fn test_validate_pin_step_invalid() {
        assert!(validate_pin_step("1*4*AGENT001*10000*123").is_err());
        assert!(validate_pin_step("1*4*AGENT001*10000").is_err());
    }
}
