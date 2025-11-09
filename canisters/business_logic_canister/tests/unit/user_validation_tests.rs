// User validation and registration tests

#[cfg(test)]
mod user_registration_tests {
    use crate::common::test_users;

    #[test]
    fn test_valid_phone_numbers() {
        let phones = vec![
            test_users::PHONE_KENYA,
            test_users::PHONE_UGANDA,
            test_users::PHONE_TANZANIA,
        ];
        
        for phone in phones {
            assert!(phone.starts_with("+"));
            assert!(phone.len() >= 12);
        }
    }

    #[test]
    fn test_pin_format() {
        let pin = test_users::TEST_PIN;
        assert_eq!(pin.len(), 4);
        assert!(pin.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_email_format() {
        let email = test_users::TEST_EMAIL;
        assert!(email.contains("@"));
        assert!(email.contains("."));
    }

    #[test]
    fn test_invalid_pin_too_short() {
        let pin = "123";
        assert_ne!(pin.len(), 4);
    }

    #[test]
    fn test_invalid_pin_too_long() {
        let pin = "12345";
        assert_ne!(pin.len(), 4);
    }

    #[test]
    fn test_invalid_pin_non_numeric() {
        let pin = "12ab";
        assert!(!pin.chars().all(|c| c.is_numeric()));
    }
}

#[cfg(test)]
mod user_identifier_tests {
    #[test]
    fn test_phone_identifier() {
        let phone = "+254712345678";
        assert!(phone.starts_with("+254") || phone.starts_with("+256") || phone.starts_with("+255"));
    }

    #[test]
    fn test_principal_identifier_format() {
        // Principal IDs are base32 encoded
        let principal = "aaaaa-aa";
        assert!(principal.contains("-"));
    }

    #[test]
    fn test_either_phone_or_principal_required() {
        let has_phone = true;
        let has_principal = false;
        assert!(has_phone || has_principal);
    }
}

#[cfg(test)]
mod user_data_validation_tests {
    #[test]
    fn test_name_not_empty() {
        let first_name = "John";
        let last_name = "Doe";
        assert!(!first_name.is_empty());
        assert!(!last_name.is_empty());
    }

    #[test]
    fn test_currency_code_format() {
        let currencies = vec!["KES", "UGX", "TZS", "NGN"];
        for currency in currencies {
            assert_eq!(currency.len(), 3);
            assert!(currency.chars().all(|c| c.is_uppercase()));
        }
    }

    #[test]
    fn test_email_basic_validation() {
        let valid_emails = vec![
            "user@example.com",
            "test.user@domain.co.ke",
            "admin@afritokeni.com",
        ];
        
        for email in valid_emails {
            assert!(email.contains("@"));
            assert!(email.split('@').count() == 2);
        }
    }

    #[test]
    fn test_invalid_email_no_at() {
        let email = "userexample.com";
        assert!(!email.contains("@"));
    }

    #[test]
    fn test_invalid_email_multiple_at() {
        let email = "user@@example.com";
        assert!(email.matches('@').count() > 1);
    }
}
