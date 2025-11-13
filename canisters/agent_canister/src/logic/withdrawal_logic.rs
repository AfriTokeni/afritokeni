// ============================================================================
// Withdrawal Logic - Pure Business Logic (No I/O)
// ============================================================================
// Handles withdrawal validation, fee calculation, and code generation
// 100% unit test coverage required
// ============================================================================

use crate::config::{get_config, get_limits_for_currency};

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate withdrawal amount is within limits for currency
pub fn validate_withdrawal_amount(amount: u64, currency: &str) -> Result<(), String> {
    if amount == 0 {
        return Err("Withdrawal amount must be greater than 0".to_string());
    }
    
    let limits = get_limits_for_currency(currency);
    
    if amount < limits.min_withdrawal {
        return Err(format!(
            "Withdrawal amount {} is below minimum {} for {}",
            amount, limits.min_withdrawal, currency
        ));
    }
    
    if amount > limits.max_withdrawal {
        return Err(format!(
            "Withdrawal amount {} exceeds maximum {} for {}",
            amount, limits.max_withdrawal, currency
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

/// Validate user has sufficient balance for withdrawal + fees
pub fn validate_sufficient_balance(
    user_balance: u64,
    withdrawal_amount: u64,
    total_fees: u64,
) -> Result<(), String> {
    let total_required = withdrawal_amount.saturating_add(total_fees);
    
    if user_balance < total_required {
        return Err(format!(
            "Insufficient balance. Have: {}, Need: {} (withdrawal: {} + fees: {})",
            user_balance, total_required, withdrawal_amount, total_fees
        ));
    }
    
    Ok(())
}

// ============================================================================
// Fee Calculation (Per Whitepaper)
// ============================================================================
// Platform Revenue = 0.5% on operation + 10% of agent's fee
// Agent Keeps = 90% of their fee
// User Pays = withdrawal amount + agent fee + platform operation fee
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawalFees {
    pub agent_fee: u64,              // Total fee agent charges user
    pub agent_keeps: u64,            // What agent actually receives (90%)
    pub platform_from_fee: u64,      // Platform's 10% of agent fee
    pub platform_operation_fee: u64, // Platform's 0.5% of withdrawal
    pub total_platform_revenue: u64, // Total platform earns
    pub total_fees: u64,             // Total fees user pays
    pub net_to_agent: u64,           // Cash agent gives to user
}

pub fn calculate_withdrawal_fees(amount: u64) -> Result<WithdrawalFees, String> {
    let config = get_config();
    
    // Agent fee (10% of withdrawal amount)
    let agent_fee = (amount * config.fees.withdrawal.agent_commission_basis_points) / 10_000;
    
    // Platform operation fee (0.5% of withdrawal amount)
    let platform_operation_fee = (amount * config.fees.withdrawal.platform_operation_fee_basis_points) / 10_000;
    
    // Platform's cut of agent fee (10% of agent fee)
    let platform_from_fee = (agent_fee * config.fees.withdrawal.platform_commission_cut_percentage) / 100;
    
    // Agent keeps 90% of their fee
    let agent_keeps = agent_fee.saturating_sub(platform_from_fee);
    
    // Total platform revenue
    let total_platform_revenue = platform_operation_fee + platform_from_fee;
    
    // Total fees user pays
    let total_fees = agent_fee + platform_operation_fee;
    
    // Net cash agent gives to user (withdrawal amount - fees)
    let net_to_agent = amount.saturating_sub(total_fees);
    
    Ok(WithdrawalFees {
        agent_fee,
        agent_keeps,
        platform_from_fee,
        platform_operation_fee,
        total_platform_revenue,
        total_fees,
        net_to_agent,
    })
}

// ============================================================================
// Code Generation
// ============================================================================

pub fn generate_withdrawal_code(withdrawal_id: u64, agent_prefix: &str) -> String {
    let config = get_config();
    let timestamp = ic_cdk::api::time() / 1_000_000; // milliseconds
    
    format!(
        "{}-{}-{}-{}",
        config.codes.withdrawal_code_prefix,
        agent_prefix,
        withdrawal_id,
        timestamp
    )
}

pub fn validate_withdrawal_code_format(code: &str) -> Result<(), String> {
    let config = get_config();
    
    if !code.starts_with(&config.codes.withdrawal_code_prefix) {
        return Err(format!(
            "Invalid withdrawal code format. Must start with {}",
            config.codes.withdrawal_code_prefix
        ));
    }
    
    let parts: Vec<&str> = code.split('-').collect();
    if parts.len() != 4 {
        return Err("Invalid withdrawal code format. Expected format: WTH-{prefix}-{id}-{timestamp}".to_string());
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
    fn test_validate_withdrawal_amount_zero() {
        setup();
        let result = validate_withdrawal_amount(0, "KES");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Withdrawal amount must be greater than 0");
    }

    #[test]
    fn test_validate_withdrawal_amount_below_minimum() {
        setup();
        let result = validate_withdrawal_amount(5000, "KES"); // min is 10000
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("below minimum"));
    }

    #[test]
    fn test_validate_withdrawal_amount_above_maximum() {
        setup();
        let result = validate_withdrawal_amount(600000, "KES"); // max is 500000
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_withdrawal_amount_valid() {
        setup();
        let result = validate_withdrawal_amount(50000, "KES");
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

    #[test]
    fn test_validate_sufficient_balance_ok() {
        setup();
        let result = validate_sufficient_balance(100000, 50000, 5000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_sufficient_balance_insufficient() {
        setup();
        let result = validate_sufficient_balance(50000, 50000, 5000);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_validate_sufficient_balance_exact() {
        setup();
        let result = validate_sufficient_balance(55000, 50000, 5000);
        assert!(result.is_ok());
    }

    // Fee Calculation Tests (Per Whitepaper)
    
    #[test]
    fn test_calculate_withdrawal_fees_100000() {
        setup();
        let fees = calculate_withdrawal_fees(100000).unwrap();
        
        // Agent fee: 10% of 100000 = 10000
        assert_eq!(fees.agent_fee, 10000);
        
        // Platform operation fee: 0.5% of 100000 = 500
        assert_eq!(fees.platform_operation_fee, 500);
        
        // Platform's cut of fee: 10% of 10000 = 1000
        assert_eq!(fees.platform_from_fee, 1000);
        
        // Agent keeps: 10000 - 1000 = 9000 (90% of fee)
        assert_eq!(fees.agent_keeps, 9000);
        
        // Total platform revenue: 500 + 1000 = 1500
        assert_eq!(fees.total_platform_revenue, 1500);
        
        // Total fees: 10000 + 500 = 10500
        assert_eq!(fees.total_fees, 10500);
        
        // Net to agent: 100000 - 10500 = 89500
        assert_eq!(fees.net_to_agent, 89500);
    }

    #[test]
    fn test_calculate_withdrawal_fees_500000() {
        setup();
        let fees = calculate_withdrawal_fees(500000).unwrap();
        
        assert_eq!(fees.agent_fee, 50000);              // 10%
        assert_eq!(fees.platform_operation_fee, 2500);  // 0.5%
        assert_eq!(fees.platform_from_fee, 5000);       // 10% of fee
        assert_eq!(fees.agent_keeps, 45000);            // 90% of fee
        assert_eq!(fees.total_platform_revenue, 7500);  // 2500 + 5000
        assert_eq!(fees.total_fees, 52500);             // 50000 + 2500
        assert_eq!(fees.net_to_agent, 447500);          // 500000 - 52500
    }

    #[test]
    fn test_calculate_withdrawal_fees_small_amount() {
        setup();
        let fees = calculate_withdrawal_fees(10000).unwrap();
        
        assert_eq!(fees.agent_fee, 1000);
        assert_eq!(fees.platform_operation_fee, 50);
        assert_eq!(fees.platform_from_fee, 100);
        assert_eq!(fees.agent_keeps, 900);
        assert_eq!(fees.total_platform_revenue, 150);
        assert_eq!(fees.total_fees, 1050);
        assert_eq!(fees.net_to_agent, 8950);
    }

    // Code Generation Tests
    
    #[test]
    fn test_generate_withdrawal_code_format() {
        setup();
        let code = generate_withdrawal_code(456, "AGT002");
        
        assert!(code.starts_with("WTH-"));
        assert!(code.contains("AGT002"));
        assert!(code.contains("456"));
        
        let parts: Vec<&str> = code.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "WTH");
        assert_eq!(parts[1], "AGT002");
        assert_eq!(parts[2], "456");
    }

    #[test]
    fn test_validate_withdrawal_code_format_valid() {
        setup();
        let result = validate_withdrawal_code_format("WTH-AGT002-456-1234567890");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_withdrawal_code_format_invalid_prefix() {
        setup();
        let result = validate_withdrawal_code_format("DEP-AGT002-456-1234567890");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Must start with WTH"));
    }

    #[test]
    fn test_validate_withdrawal_code_format_invalid_parts() {
        setup();
        let result = validate_withdrawal_code_format("WTH-AGT002-456");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Expected format"));
    }

    // Edge Cases
    
    #[test]
    fn test_calculate_withdrawal_fees_no_overflow() {
        setup();
        let fees = calculate_withdrawal_fees(u64::MAX / 10).unwrap();
        assert!(fees.agent_fee > 0);
        assert!(fees.total_platform_revenue > 0);
    }

    #[test]
    fn test_validate_sufficient_balance_overflow_protection() {
        setup();
        let result = validate_sufficient_balance(u64::MAX, u64::MAX - 1000, 2000);
        assert!(result.is_err());
    }
}
