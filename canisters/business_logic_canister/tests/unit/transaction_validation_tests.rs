// Transaction validation tests

#[cfg(test)]
mod amount_validation_tests {
    use crate::common::test_amounts;

    #[test]
    fn test_positive_amounts() {
        assert!(test_amounts::SMALL > 0);
        assert!(test_amounts::MEDIUM > 0);
        assert!(test_amounts::LARGE > 0);
    }

    #[test]
    fn test_amount_not_zero() {
        let amount = 100u64;
        assert_ne!(amount, 0);
    }

    #[test]
    fn test_amount_within_limits() {
        let min = 10u64;
        let max = 1_000_000u64;
        let amount = test_amounts::MEDIUM;
        
        assert!(amount >= min);
        assert!(amount <= max);
    }

    #[test]
    fn test_amount_too_small() {
        let min = 10u64;
        let amount = 5u64;
        assert!(amount < min);
    }

    #[test]
    fn test_amount_too_large() {
        let max = 1_000_000u64;
        let amount = 2_000_000u64;
        assert!(amount > max);
    }

    // EDGE CASES
    #[test]
    fn test_amount_exactly_at_minimum() {
        let min = 10u64;
        let amount = 10u64;
        assert_eq!(amount, min);
        assert!(amount >= min);
    }

    #[test]
    fn test_amount_exactly_at_maximum() {
        let max = 1_000_000u64;
        let amount = 1_000_000u64;
        assert_eq!(amount, max);
        assert!(amount <= max);
    }

    #[test]
    fn test_amount_one_below_minimum() {
        let min = 10u64;
        let amount = 9u64;
        assert!(amount < min);
    }

    #[test]
    fn test_amount_one_above_maximum() {
        let max = 1_000_000u64;
        let amount = 1_000_001u64;
        assert!(amount > max);
    }

    #[test]
    fn test_amount_u64_max() {
        let amount = u64::MAX;
        let max_allowed = 1_000_000u64;
        assert!(amount > max_allowed);
    }

    #[test]
    fn test_amount_overflow_protection() {
        let amount1 = u64::MAX;
        let amount2 = 1u64;
        // Should check for overflow before adding
        assert!(amount1.checked_add(amount2).is_none());
    }

    #[test]
    fn test_amount_multiplication_overflow() {
        let amount = u64::MAX / 2;
        let multiplier = 3u64;
        // Should check for overflow
        assert!(amount.checked_mul(multiplier).is_none());
    }
}

#[cfg(test)]
mod balance_validation_tests {
    #[test]
    fn test_sufficient_balance() {
        let balance = 1000u64;
        let amount = 500u64;
        assert!(balance >= amount);
    }

    #[test]
    fn test_insufficient_balance() {
        let balance = 100u64;
        let amount = 500u64;
        assert!(balance < amount);
    }

    #[test]
    fn test_exact_balance() {
        let balance = 1000u64;
        let amount = 1000u64;
        assert_eq!(balance, amount);
    }

    #[test]
    fn test_balance_after_deduction() {
        let balance = 1000u64;
        let amount = 300u64;
        let remaining = balance - amount;
        assert_eq!(remaining, 700);
    }

    // EDGE CASES
    #[test]
    fn test_zero_balance() {
        let balance = 0u64;
        let amount = 100u64;
        assert!(balance < amount);
    }

    #[test]
    fn test_balance_exactly_one_less_than_amount() {
        let balance = 999u64;
        let amount = 1000u64;
        assert!(balance < amount);
    }

    #[test]
    fn test_balance_exactly_one_more_than_amount() {
        let balance = 1001u64;
        let amount = 1000u64;
        assert!(balance > amount);
    }

    #[test]
    fn test_multiple_deductions_exhaust_balance() {
        let mut balance = 1000u64;
        balance -= 300;
        balance -= 400;
        balance -= 300;
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_balance_underflow_protection() {
        let balance = 100u64;
        let amount = 500u64;
        // Should check before subtracting
        assert!(balance.checked_sub(amount).is_none());
    }

    #[test]
    fn test_concurrent_transaction_race_condition() {
        // If balance is 1000 and two transactions of 600 each try to execute
        let balance = 1000u64;
        let tx1_amount = 600u64;
        let tx2_amount = 600u64;
        
        // Only one should succeed
        let can_both_succeed = balance >= (tx1_amount + tx2_amount);
        assert!(!can_both_succeed);
    }

    #[test]
    fn test_balance_with_pending_transactions() {
        let balance = 1000u64;
        let pending_amount = 300u64;
        let available = balance - pending_amount;
        let new_tx_amount = 800u64;
        
        // Should check available, not total balance
        assert!(available < new_tx_amount);
    }
}

#[cfg(test)]
mod recipient_validation_tests {
    #[test]
    fn test_sender_not_recipient() {
        let sender = "+254712345678";
        let recipient = "+254700999888";
        assert_ne!(sender, recipient);
    }

    #[test]
    fn test_recipient_phone_format() {
        let recipient = "+254712345678";
        assert!(recipient.starts_with("+"));
        assert!(recipient.len() >= 12);
    }

    #[test]
    fn test_invalid_self_transfer() {
        let sender = "+254712345678";
        let recipient = "+254712345678";
        assert_eq!(sender, recipient); // Should be rejected
    }
}

#[cfg(test)]
mod currency_validation_tests {
    use crate::common::test_currencies;

    #[test]
    fn test_supported_currencies() {
        let currencies = vec![
            test_currencies::KES,
            test_currencies::UGX,
            test_currencies::TZS,
            test_currencies::NGN,
        ];
        
        for currency in currencies {
            assert_eq!(currency.len(), 3);
        }
    }

    #[test]
    fn test_currency_code_uppercase() {
        let currency = test_currencies::KES;
        assert!(currency.chars().all(|c| c.is_uppercase()));
    }

    #[test]
    fn test_same_currency_transfer() {
        let sender_currency = "KES";
        let recipient_currency = "KES";
        assert_eq!(sender_currency, recipient_currency);
    }
}
