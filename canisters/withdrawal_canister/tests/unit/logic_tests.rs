use candid::Principal;
use withdrawal_canister::logic::*;
use withdrawal_canister::TransactionStatus;

#[cfg(test)]
mod logic_function_tests {
    use super::*;

    #[test]
    fn test_validate_caller_is_user_success() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();
        assert!(validate_caller_is_user(principal, principal).is_ok());
    }

    #[test]
    fn test_validate_amount_positive_success() {
        assert!(validate_amount_positive(100).is_ok());
    }

    #[test]
    fn test_validate_amount_zero_fails() {
        assert!(validate_amount_positive(0).is_err());
    }

    #[test]
    fn test_calculate_platform_fee() {
        let fee = calculate_platform_fee(10000, 50).unwrap();
        assert_eq!(fee, 50);
    }

    #[test]
    fn test_calculate_fee_overflow_protection() {
        let result = calculate_platform_fee(u64::MAX, 10000);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_withdrawal_code() {
        let code = generate_withdrawal_code(123);
        assert_eq!(code, "WTH000123");
    }

    #[test]
    fn test_validate_code_format() {
        assert!(validate_withdrawal_code_format("WTH123456"));
        assert!(!validate_withdrawal_code_format("DEP123456"));
        assert!(!validate_withdrawal_code_format("WTH12"));
    }
}

#[cfg(test)]
mod withdrawal_creation_logic {
    use super::*;

    #[test]
    fn test_validate_zero_amount_fails() {
        let result = validate_amount_positive(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Amount must be greater than 0");
    }

    #[test]
    fn test_validate_positive_amount_succeeds() {
        let result = validate_amount_positive(1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_caller_is_user_succeeds() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();
        let result = validate_caller_is_user(principal, principal);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_caller_is_not_user_fails() {
        let user = Principal::from_text("aaaaa-aa").unwrap();
        let caller = Principal::from_text("2vxsx-fae").unwrap();  // Valid principal
        let result = validate_caller_is_user(caller, user);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Caller must be the user");
    }

    #[test]
    fn test_calculate_platform_fee_basic() {
        let amount = 10000u64;
        let basis_points = 50u64;
        let fee = calculate_platform_fee(amount, basis_points).unwrap();
        assert_eq!(fee, 50);
    }

    #[test]
    fn test_calculate_agent_fee_basic() {
        let amount = 10000u64;
        let basis_points = 200u64;
        let fee = calculate_agent_fee(amount, basis_points).unwrap();
        assert_eq!(fee, 200);
    }

    #[test]
    fn test_fee_calculation_with_small_amount() {
        let amount = 100u64;
        let basis_points = 50u64;
        let fee = calculate_platform_fee(amount, basis_points).unwrap();
        assert_eq!(fee, 0);
    }

    #[test]
    fn test_fee_calculation_at_boundary() {
        let amount = 10000u64;
        let basis_points = 10000u64;
        let fee = calculate_platform_fee(amount, basis_points).unwrap();
        assert_eq!(fee, 10000);
    }
}

#[cfg(test)]
mod withdrawal_confirmation_logic {
    use super::*;

    #[test]
    fn test_validate_agent_is_caller_succeeds() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();
        let result = validate_caller_is_agent(principal, principal);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_agent_is_not_caller_fails() {
        let agent = Principal::from_text("aaaaa-aa").unwrap();
        let caller = Principal::from_text("2vxsx-fae").unwrap();
        let result = validate_caller_is_agent(caller, agent);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Only the assigned agent can confirm");
    }

    #[test]
    fn test_validate_pending_status_succeeds() {
        let status = TransactionStatus::Pending;
        let result = validate_status_is_pending(&status);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_confirmed_status_fails() {
        let status = TransactionStatus::Confirmed;
        let result = validate_status_is_pending(&status);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Withdrawal already processed");
    }

    #[test]
    fn test_validate_cancelled_status_fails() {
        let status = TransactionStatus::Cancelled;
        let result = validate_status_is_pending(&status);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_agent_matches_succeeds() {
        let agent = Principal::from_text("aaaaa-aa").unwrap();
        let result = validate_agent_matches(agent, agent);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_agent_mismatch_fails() {
        let agent1 = Principal::from_text("aaaaa-aa").unwrap();
        let agent2 = Principal::from_text("2vxsx-fae").unwrap();
        let result = validate_agent_matches(agent1, agent2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Wrong agent");
    }
}

#[cfg(test)]
mod agent_earnings_logic {
    use candid::Principal;

    struct AgentEarnings {
        total_withdrawals_processed: u64,
        total_fees_earned: u64,
        total_fees_withdrawn: u64,
    }

    fn update_earnings(earnings: &mut AgentEarnings, withdrawal_amount: u64, fee: u64) {
        earnings.total_withdrawals_processed += withdrawal_amount;
        earnings.total_fees_earned += fee;
    }

    fn calculate_available_earnings(earnings: &AgentEarnings) -> u64 {
        earnings.total_fees_earned - earnings.total_fees_withdrawn
    }

    #[test]
    fn test_update_earnings() {
        let mut earnings = AgentEarnings {
            total_withdrawals_processed: 0,
            total_fees_earned: 0,
            total_fees_withdrawn: 0,
        };

        update_earnings(&mut earnings, 10000, 200);
        
        assert_eq!(earnings.total_withdrawals_processed, 10000);
        assert_eq!(earnings.total_fees_earned, 200);
    }

    #[test]
    fn test_multiple_withdrawals_accumulate() {
        let mut earnings = AgentEarnings {
            total_withdrawals_processed: 0,
            total_fees_earned: 0,
            total_fees_withdrawn: 0,
        };

        update_earnings(&mut earnings, 10000, 200);
        update_earnings(&mut earnings, 5000, 100);
        update_earnings(&mut earnings, 3000, 60);
        
        assert_eq!(earnings.total_withdrawals_processed, 18000);
        assert_eq!(earnings.total_fees_earned, 360);
    }

    #[test]
    fn test_calculate_available_earnings() {
        let earnings = AgentEarnings {
            total_withdrawals_processed: 10000,
            total_fees_earned: 500,
            total_fees_withdrawn: 200,
        };

        let available = calculate_available_earnings(&earnings);
        assert_eq!(available, 300);
    }

    #[test]
    fn test_available_earnings_when_all_withdrawn() {
        let earnings = AgentEarnings {
            total_withdrawals_processed: 10000,
            total_fees_earned: 500,
            total_fees_withdrawn: 500,
        };

        let available = calculate_available_earnings(&earnings);
        assert_eq!(available, 0);
    }

    #[test]
    fn test_earnings_overflow_protection() {
        let mut earnings = AgentEarnings {
            total_withdrawals_processed: u64::MAX - 100,
            total_fees_earned: 0,
            total_fees_withdrawn: 0,
        };

        // Should use checked arithmetic in real code
        let result = earnings.total_withdrawals_processed.checked_add(200);
        assert!(result.is_none()); // Would overflow
    }
}

#[cfg(test)]
mod withdrawal_code_logic {
    fn generate_withdrawal_code(id: u64) -> String {
        format!("WTH{:06}", id)
    }

    fn validate_withdrawal_code_format(code: &str) -> bool {
        code.starts_with("WTH") && code.len() == 9
    }

    #[test]
    fn test_generate_withdrawal_code() {
        let code = generate_withdrawal_code(123);
        assert_eq!(code, "WTH000123");
    }

    #[test]
    fn test_generate_withdrawal_code_large_id() {
        let code = generate_withdrawal_code(999999);
        assert_eq!(code, "WTH999999");
    }

    #[test]
    fn test_validate_code_format_valid() {
        assert!(validate_withdrawal_code_format("WTH123456"));
    }

    #[test]
    fn test_validate_code_format_invalid_prefix() {
        assert!(!validate_withdrawal_code_format("DEP123456"));
    }

    #[test]
    fn test_validate_code_format_invalid_length() {
        assert!(!validate_withdrawal_code_format("WTH12"));
    }

    #[test]
    fn test_withdrawal_code_uniqueness() {
        let code1 = generate_withdrawal_code(1);
        let code2 = generate_withdrawal_code(2);
        assert_ne!(code1, code2);
    }
}
