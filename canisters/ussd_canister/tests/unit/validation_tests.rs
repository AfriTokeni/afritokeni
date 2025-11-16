// Comprehensive validation tests
use ussd_canister::utils::validation;

#[cfg(test)]
mod phone_validation_tests {
    use super::*;

    #[test]
    fn test_valid_kenyan_phone() {
        assert!(validation::is_valid_phone("+254712345678"));
        assert!(validation::is_valid_phone("+254700123456"));
    }

    #[test]
    fn test_valid_ugandan_phone() {
        assert!(validation::is_valid_phone("+256700123456"));
        assert!(validation::is_valid_phone("+256712345678"));
    }

    #[test]
    fn test_valid_tanzanian_phone() {
        assert!(validation::is_valid_phone("+255712345678"));
        assert!(validation::is_valid_phone("+255700123456"));
    }

    #[test]
    fn test_invalid_phone_too_short() {
        assert!(!validation::is_valid_phone("+25412"));
        assert!(!validation::is_valid_phone("+254"));
    }

    #[test]
    fn test_invalid_phone_no_plus() {
        assert!(!validation::is_valid_phone("254712345678"));
    }

    #[test]
    fn test_invalid_phone_wrong_country_code() {
        assert!(!validation::is_valid_phone("+1234567890")); // US number
        assert!(!validation::is_valid_phone("+44712345678")); // UK number
    }

    #[test]
    fn test_invalid_phone_empty() {
        assert!(!validation::is_valid_phone(""));
    }

    #[test]
    fn test_invalid_phone_letters() {
        assert!(!validation::is_valid_phone("+254ABC123456"));
    }
}

#[cfg(test)]
mod amount_validation_tests {
    use super::*;

    #[test]
    fn test_valid_amounts() {
        assert!(validation::is_valid_amount("100"));
        assert!(validation::is_valid_amount("1000"));
        assert!(validation::is_valid_amount("50000"));
    }

    #[test]
    fn test_invalid_amount_zero() {
        assert!(!validation::is_valid_amount("0"));
    }

    #[test]
    fn test_invalid_amount_negative() {
        assert!(!validation::is_valid_amount("-100"));
    }

    #[test]
    fn test_invalid_amount_decimal() {
        assert!(!validation::is_valid_amount("100.50"));
    }

    #[test]
    fn test_invalid_amount_letters() {
        assert!(!validation::is_valid_amount("abc"));
        assert!(!validation::is_valid_amount("100abc"));
    }

    #[test]
    fn test_invalid_amount_empty() {
        assert!(!validation::is_valid_amount(""));
    }

    #[test]
    fn test_amount_too_large() {
        // Assuming max is 1,000,000
        assert!(!validation::is_valid_amount("10000000"));
    }
}

#[cfg(test)]
mod pin_validation_tests {
    use super::*;

    #[test]
    fn test_valid_pin() {
        assert!(validation::is_valid_pin("1234"));
        assert!(validation::is_valid_pin("0000"));
        assert!(validation::is_valid_pin("9999"));
    }

    #[test]
    fn test_invalid_pin_too_short() {
        assert!(!validation::is_valid_pin("123"));
    }

    #[test]
    fn test_invalid_pin_too_long() {
        assert!(!validation::is_valid_pin("12345"));
    }

    #[test]
    fn test_invalid_pin_letters() {
        assert!(!validation::is_valid_pin("12ab"));
        assert!(!validation::is_valid_pin("abcd"));
    }

    #[test]
    fn test_invalid_pin_empty() {
        assert!(!validation::is_valid_pin(""));
    }

    #[test]
    fn test_invalid_pin_special_chars() {
        assert!(!validation::is_valid_pin("12#4"));
        assert!(!validation::is_valid_pin("12*4"));
    }
}
