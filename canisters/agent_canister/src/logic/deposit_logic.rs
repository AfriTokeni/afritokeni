// ============================================================================
// Deposit Logic - Pure Business Logic (No I/O)
// ============================================================================
// Handles deposit validation, commission calculation, and code generation
// 100% unit test coverage required
// ============================================================================

use crate::config::{get_config, get_limits_for_currency};

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate deposit amount is within limits for currency
pub fn validate_deposit_amount(amount: u64, currency: &str) -> Result<(), String> {
    if amount == 0 {
        return Err("Deposit amount must be greater than 0".to_string());
    }
    
    let limits = get_limits_for_currency(currency);
    
    if amount < limits.min_deposit {
        return Err(format!(
            "Deposit amount {} is below minimum {} for {}",
            amount, limits.min_deposit, currency
        ));
    }
    
    if amount > limits.max_deposit {
        return Err(format!(
            "Deposit amount {} exceeds maximum {} for {}",
            amount, limits.max_deposit, currency
        ));
    }
    
    Ok(())
}

/// Validate currency is supported
pub fn validate_currency(currency: &str) -> Result<(), String> {
    use shared_types::FiatCurrency;
    
    FiatCurrency::from_string(currency)
        .map(|_| ())
        .map_err(|e| format!("Invalid currency: {}", e))
}

// ============================================================================
// Commission Calculation (Per Whitepaper)
// ============================================================================
// Platform Revenue = 0.5% on operation + 10% of agent's commission
// Agent Keeps = 90% of their commission
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositFees {
    pub agent_commission: u64,      // Total commission agent charges user
    pub agent_keeps: u64,            // What agent actually receives (90%)
    pub platform_from_commission: u64, // Platform's 10% of agent commission
    pub platform_operation_fee: u64, // Platform's 0.5% of deposit
    pub total_platform_revenue: u64, // Total platform earns
    pub net_to_user_balance: u64,    // What gets added to user's balance
}

pub fn calculate_deposit_fees(amount: u64) -> Result<DepositFees, String> {
    let config = get_config();
    
    // Agent commission (10% of deposit amount)
    let agent_commission = (amount * config.fees.deposit.agent_commission_basis_points) / 10_000;
    
    // Platform operation fee (0.5% of deposit amount)
    let platform_operation_fee = (amount * config.fees.deposit.platform_operation_fee_basis_points) / 10_000;
    
    // Platform's cut of agent commission (10% of agent commission)
    let platform_from_commission = (agent_commission * config.fees.deposit.platform_commission_cut_percentage) / 100;
    
    // Agent keeps 90% of their commission
    let agent_keeps = agent_commission.saturating_sub(platform_from_commission);
    
    // Total platform revenue
    let total_platform_revenue = platform_operation_fee + platform_from_commission;
    
    // Net amount added to user's balance (deposit - agent commission)
    let net_to_user_balance = amount.saturating_sub(agent_commission);
    
    Ok(DepositFees {
        agent_commission,
        agent_keeps,
        platform_from_commission,
        platform_operation_fee,
        total_platform_revenue,
        net_to_user_balance,
    })
}

// ============================================================================
// Code Generation
// ============================================================================

pub fn generate_deposit_code(deposit_id: u64, agent_prefix: &str) -> String {
    let config = get_config();
    let timestamp = ic_cdk::api::time() / 1_000_000; // milliseconds
    
    format!(
        "{}-{}-{}-{}",
        config.codes.deposit_code_prefix,
        agent_prefix,
        deposit_id,
        timestamp
    )
}

pub fn validate_deposit_code_format(code: &str) -> Result<(), String> {
    let config = get_config();
    
    if !code.starts_with(&config.codes.deposit_code_prefix) {
        return Err(format!(
            "Invalid deposit code format. Must start with {}",
            config.codes.deposit_code_prefix
        ));
    }
    
    let parts: Vec<&str> = code.split('-').collect();
    if parts.len() != 4 {
        return Err("Invalid deposit code format. Expected format: DEP-{prefix}-{id}-{timestamp}".to_string());
    }
    
    Ok(())
}

// ============================================================================
// Unit Tests (100% Coverage Required)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::init_config;

    fn setup() {
        init_config();
    }

    // Validation Tests
    
    #[test]
    fn test_validate_deposit_amount_zero() {
        setup();
        let result = validate_deposit_amount(0, "KES");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Deposit amount must be greater than 0");
    }

    #[test]
    fn test_validate_deposit_amount_below_minimum() {
        setup();
        let result = validate_deposit_amount(5000, "KES"); // min is 10000
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("below minimum"));
    }

    #[test]
    fn test_validate_deposit_amount_above_maximum() {
        setup();
        let result = validate_deposit_amount(2000000, "KES"); // max is 1000000
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_deposit_amount_valid() {
        setup();
        let result = validate_deposit_amount(50000, "KES");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_currency_valid() {
        setup();
        assert!(validate_currency("KES").is_ok());
        assert!(validate_currency("UGX").is_ok());
        assert!(validate_currency("NGN").is_ok());
    }

    #[test]
    fn test_validate_currency_invalid() {
        setup();
        let result = validate_currency("XXX");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid currency"));
    }

    // Commission Calculation Tests (Per Whitepaper)
    
    #[test]
    fn test_calculate_deposit_fees_100000() {
        setup();
        let fees = calculate_deposit_fees(100000).unwrap();
        
        // Agent commission: 10% of 100000 = 10000
        assert_eq!(fees.agent_commission, 10000);
        
        // Platform operation fee: 0.5% of 100000 = 500
        assert_eq!(fees.platform_operation_fee, 500);
        
        // Platform's cut of commission: 10% of 10000 = 1000
        assert_eq!(fees.platform_from_commission, 1000);
        
        // Agent keeps: 10000 - 1000 = 9000 (90% of commission)
        assert_eq!(fees.agent_keeps, 9000);
        
        // Total platform revenue: 500 + 1000 = 1500
        assert_eq!(fees.total_platform_revenue, 1500);
        
        // Net to user: 100000 - 10000 = 90000
        assert_eq!(fees.net_to_user_balance, 90000);
    }

    #[test]
    fn test_calculate_deposit_fees_1000000() {
        setup();
        let fees = calculate_deposit_fees(1000000).unwrap();
        
        assert_eq!(fees.agent_commission, 100000);       // 10%
        assert_eq!(fees.platform_operation_fee, 5000);   // 0.5%
        assert_eq!(fees.platform_from_commission, 10000); // 10% of commission
        assert_eq!(fees.agent_keeps, 90000);             // 90% of commission
        assert_eq!(fees.total_platform_revenue, 15000);  // 5000 + 10000
        assert_eq!(fees.net_to_user_balance, 900000);    // 1000000 - 100000
    }

    #[test]
    fn test_calculate_deposit_fees_small_amount() {
        setup();
        let fees = calculate_deposit_fees(10000).unwrap();
        
        assert_eq!(fees.agent_commission, 1000);
        assert_eq!(fees.platform_operation_fee, 50);
        assert_eq!(fees.platform_from_commission, 100);
        assert_eq!(fees.agent_keeps, 900);
        assert_eq!(fees.total_platform_revenue, 150);
        assert_eq!(fees.net_to_user_balance, 9000);
    }

    // Code Generation Tests
    
    #[test]
    fn test_generate_deposit_code_format() {
        setup();
        let code = generate_deposit_code(123, "AGT001");
        
        assert!(code.starts_with("DEP-"));
        assert!(code.contains("AGT001"));
        assert!(code.contains("123"));
        
        let parts: Vec<&str> = code.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "DEP");
        assert_eq!(parts[1], "AGT001");
        assert_eq!(parts[2], "123");
    }

    #[test]
    fn test_validate_deposit_code_format_valid() {
        setup();
        let result = validate_deposit_code_format("DEP-AGT001-123-1234567890");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_deposit_code_format_invalid_prefix() {
        setup();
        let result = validate_deposit_code_format("WTH-AGT001-123-1234567890");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Must start with DEP"));
    }

    #[test]
    fn test_validate_deposit_code_format_invalid_parts() {
        setup();
        let result = validate_deposit_code_format("DEP-AGT001-123");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected format"));
    }

    // Edge Cases
    
    #[test]
    fn test_calculate_deposit_fees_no_overflow() {
        setup();
        let fees = calculate_deposit_fees(u64::MAX / 10).unwrap();
        assert!(fees.agent_commission > 0);
        assert!(fees.total_platform_revenue > 0);
    }
}
