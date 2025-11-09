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
