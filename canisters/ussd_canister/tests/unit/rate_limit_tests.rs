// Rate limiting tests
use ussd_canister::utils::rate_limit;

#[cfg(test)]
mod rate_limit_tests {
    use super::*;

    #[test]
    fn test_rate_limit_allows_first_request() {
        // In test mode, rate limiting is disabled
        let phone = "+254712345678";
        assert!(rate_limit::check_rate_limit(phone));
    }

    #[test]
    fn test_rate_limit_allows_test_phone() {
        // Integration test phone numbers should bypass rate limiting
        let test_phone = "+254700123456";
        assert!(rate_limit::check_rate_limit(test_phone));
    }

    #[test]
    fn test_rate_limit_allows_test_phone_without_plus() {
        let test_phone = "254700123456";
        assert!(rate_limit::check_rate_limit(test_phone));
    }

    #[test]
    fn test_rate_limit_in_test_mode() {
        // In #[cfg(test)], all requests should be allowed
        let phone = "+256700999888";
        for _ in 0..100 {
            assert!(rate_limit::check_rate_limit(phone));
        }
    }
}
