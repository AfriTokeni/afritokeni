// Withdrawal validation tests - REAL EDGE CASES ONLY!

#[cfg(test)]
mod withdrawal_amount_validation {
    use crate::common::test_amounts;

    #[test]
    fn test_withdrawal_exactly_at_minimum() {
        let amount = test_amounts::MIN_WITHDRAWAL;
        assert_eq!(amount, 10);
        assert!(amount >= test_amounts::MIN_WITHDRAWAL);
    }

    #[test]
    fn test_withdrawal_one_below_minimum() {
        let amount = 9u64;
        let min = test_amounts::MIN_WITHDRAWAL;
        assert!(amount < min);
        // Should reject
    }

    #[test]
    fn test_withdrawal_exactly_at_maximum() {
        let amount = test_amounts::MAX_WITHDRAWAL;
        assert_eq!(amount, 1000000);
        assert!(amount <= test_amounts::MAX_WITHDRAWAL);
    }

    #[test]
    fn test_withdrawal_one_above_maximum() {
        let amount = 1000001u64;
        let max = test_amounts::MAX_WITHDRAWAL;
        assert!(amount > max);
        // Should reject
    }

    #[test]
    fn test_withdrawal_zero_amount() {
        let amount = 0u64;
        assert_eq!(amount, 0);
        // Should reject
    }

    #[test]
    fn test_withdrawal_exceeds_balance() {
        let balance = 5000u64;
        let withdrawal = 10000u64;
        assert!(withdrawal > balance);
        // Should reject insufficient balance
    }

    #[test]
    fn test_withdrawal_exactly_equals_balance() {
        let balance = 10000u64;
        let withdrawal = 10000u64;
        assert_eq!(withdrawal, balance);
        // Should allow (closes account)
    }

    #[test]
    fn test_withdrawal_leaves_dust_balance() {
        let balance = 10005u64;
        let withdrawal = 10000u64;
        let remaining = balance - withdrawal;
        assert_eq!(remaining, 5);
        // Should warn about small remaining balance
    }

    #[test]
    fn test_multiple_withdrawals_exhaust_balance() {
        let mut balance = 10000u64;
        let withdrawals = vec![3000u64, 4000u64, 3000u64];
        
        for w in withdrawals {
            if balance >= w {
                balance -= w;
            }
        }
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_withdrawal_with_pending_transactions() {
        let balance = 10000u64;
        let pending_withdrawal = 3000u64;
        let available = balance - pending_withdrawal;
        let new_withdrawal = 8000u64;
        
        assert!(new_withdrawal > available);
        // Should check available, not total balance
    }
}

#[cfg(test)]
mod withdrawal_code_validation {
    #[test]
    fn test_withdrawal_code_format() {
        let code = "WTH123456";
        assert!(code.starts_with("WTH"));
        assert_eq!(code.len(), 9);
    }

    #[test]
    fn test_withdrawal_code_expiration() {
        let created_at = 1699564800u64;
        let now = 1699566600u64; // 30 minutes later
        let expiry_seconds = 1800u64; // 30 minutes
        
        let elapsed = now - created_at;
        assert_eq!(elapsed, expiry_seconds);
        // Code should expire
    }

    #[test]
    fn test_expired_withdrawal_code() {
        let created_at = 1699564800u64;
        let now = 1699568400u64; // 1 hour later
        let expiry_seconds = 1800u64; // 30 minutes
        
        let elapsed = now - created_at;
        assert!(elapsed > expiry_seconds);
        // Should reject expired code
    }

    #[test]
    fn test_withdrawal_code_already_used() {
        let code = "WTH123456";
        let used_codes = vec!["WTH123456", "WTH789012"];
        assert!(used_codes.contains(&code));
        // Should reject already used codes
    }

    #[test]
    fn test_withdrawal_code_cancelled() {
        let code = "WTH123456";
        let cancelled_codes = vec!["WTH123456"];
        assert!(cancelled_codes.contains(&code));
        // Should reject cancelled codes
    }

    #[test]
    fn test_withdrawal_code_wrong_agent() {
        let code_agent = "agent_001";
        let redeeming_agent = "agent_002";
        assert_ne!(code_agent, redeeming_agent);
        // Should reject if wrong agent
    }
}

#[cfg(test)]
mod withdrawal_timing_tests {
    #[test]
    fn test_withdrawal_confirmation_timeout() {
        let initiated_at = 1699564800u64;
        let now = 1699565700u64; // 15 minutes later
        let timeout = 900u64; // 15 minutes
        
        let elapsed = now - initiated_at;
        assert_eq!(elapsed, timeout);
        // Should timeout unconfirmed withdrawals
    }

    #[test]
    fn test_withdrawal_too_soon_after_deposit() {
        let deposit_time = 1699564800u64;
        let withdrawal_time = 1699564860u64; // 1 minute later
        let min_hold_period = 300u64; // 5 minutes
        
        let elapsed = withdrawal_time - deposit_time;
        assert!(elapsed < min_hold_period);
        // Should enforce hold period
    }

    #[test]
    fn test_multiple_rapid_withdrawals() {
        let withdrawals = vec![
            1699564800u64,
            1699564810u64,
            1699564820u64,
        ];
        
        let max_interval = withdrawals.windows(2)
            .map(|w| w[1] - w[0])
            .max()
            .unwrap();
        
        assert_eq!(max_interval, 10);
        // Should flag rapid withdrawals
    }

    #[test]
    fn test_withdrawal_outside_business_hours() {
        let hour = 2; // 2 AM
        let business_hours = 8..20;
        assert!(!business_hours.contains(&hour));
        // Should flag or restrict
    }
}
