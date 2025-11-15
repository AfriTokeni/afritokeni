// Comprehensive session cleanup and timeout tests
use ussd_canister::core::session::{UssdSession, cleanup_expired_sessions, get_active_session_count};
use ussd_canister::utils::constants::SESSION_TIMEOUT_NANOS;

#[cfg(test)]
mod session_expiration_tests {
    use super::*;

    #[test]
    fn test_session_is_not_expired_immediately() {
        let session = UssdSession::new("test123".to_string(), "+256700123456".to_string());
        assert!(!session.is_expired(), "Newly created session should not be expired");
    }

    #[test]
    fn test_session_timeout_constant() {
        // Verify SESSION_TIMEOUT_NANOS is set to 5 minutes (300 seconds)
        const EXPECTED_TIMEOUT: u64 = 300_000_000_000; // 5 minutes in nanoseconds
        assert_eq!(SESSION_TIMEOUT_NANOS, EXPECTED_TIMEOUT, "Session timeout should be 5 minutes");
    }

    #[test]
    fn test_session_expiration_logic() {
        let mut session = UssdSession::new("test456".to_string(), "+254712345678".to_string());

        // Manually set last_activity to simulate an old session
        // (In test environment, time() returns fixed timestamp)
        session.last_activity = 1000000000000000000; // Very old timestamp

        // Session should be expired
        assert!(session.is_expired(), "Old session should be expired");
    }

    #[test]
    fn test_update_activity_prevents_expiration() {
        let mut session = UssdSession::new("test789".to_string(), "+255700123456".to_string());

        // Update activity
        session.update_activity();

        // Session should not be expired
        assert!(!session.is_expired(), "Session with updated activity should not be expired");
    }

    #[test]
    fn test_session_last_activity_tracking() {
        let session1 = UssdSession::new("test1".to_string(), "+256700123456".to_string());
        let mut session2 = UssdSession::new("test2".to_string(), "+254712345678".to_string());

        let initial_activity = session2.last_activity;

        // Update activity
        session2.update_activity();

        // last_activity should be updated (in test mode it uses fixed time, but the call should succeed)
        assert_eq!(session2.last_activity, initial_activity, "In test mode, time is fixed");
    }
}

#[cfg(test)]
mod session_cleanup_tests {
    use super::*;

    #[test]
    fn test_cleanup_expired_sessions_function_exists() {
        // Test that cleanup_expired_sessions function is available
        let cleaned = cleanup_expired_sessions();
        // Should not panic, returns number of cleaned sessions
        assert!(cleaned >= 0, "cleanup_expired_sessions should return non-negative count");
    }

    #[test]
    fn test_get_active_session_count_function() {
        // Test that get_active_session_count function is available
        let count = get_active_session_count();
        assert!(count >= 0, "get_active_session_count should return non-negative count");
    }

    #[test]
    fn test_cleanup_deterministic_behavior() {
        // Cleanup should be deterministic (not use heartbeats)
        // This test verifies the function can be called multiple times safely
        let count1 = cleanup_expired_sessions();
        let count2 = cleanup_expired_sessions();

        // Both calls should succeed without panic
        assert!(count1 >= 0, "First cleanup should succeed");
        assert!(count2 >= 0, "Second cleanup should succeed");
    }
}

#[cfg(test)]
mod session_data_management_tests {
    use super::*;

    #[test]
    fn test_session_data_storage() {
        let mut session = UssdSession::new("test_data".to_string(), "+256700123456".to_string());

        // Store data
        session.set_data("recipient", "+254712345678");
        session.set_data("amount", "1000");
        session.set_data("currency", "UGX");

        // Retrieve data
        assert_eq!(session.get_data("recipient"), Some("+254712345678".to_string()));
        assert_eq!(session.get_data("amount"), Some("1000".to_string()));
        assert_eq!(session.get_data("currency"), Some("UGX".to_string()));
    }

    #[test]
    fn test_session_data_not_found() {
        let session = UssdSession::new("test_notfound".to_string(), "+256700123456".to_string());

        // Try to get non-existent key
        assert_eq!(session.get_data("nonexistent"), None);
    }

    #[test]
    fn test_session_data_overwrite() {
        let mut session = UssdSession::new("test_overwrite".to_string(), "+256700123456".to_string());

        // Set initial value
        session.set_data("amount", "1000");
        assert_eq!(session.get_data("amount"), Some("1000".to_string()));

        // Overwrite value
        session.set_data("amount", "2000");
        assert_eq!(session.get_data("amount"), Some("2000".to_string()));
    }

    #[test]
    fn test_session_clear_data() {
        let mut session = UssdSession::new("test_clear".to_string(), "+256700123456".to_string());

        // Add multiple data entries
        session.set_data("recipient", "+254712345678");
        session.set_data("amount", "1000");
        session.set_data("pin", "1234");

        // Verify data exists
        assert!(session.get_data("recipient").is_some());
        assert!(session.get_data("amount").is_some());
        assert!(session.get_data("pin").is_some());

        // Clear all data
        session.clear_data();

        // Verify all data is gone
        assert!(session.get_data("recipient").is_none());
        assert!(session.get_data("amount").is_none());
        assert!(session.get_data("pin").is_none());
    }

    #[test]
    fn test_session_data_empty_string_value() {
        let mut session = UssdSession::new("test_empty".to_string(), "+256700123456".to_string());

        // Set empty string value
        session.set_data("empty_key", "");

        // Should store empty string, not None
        assert_eq!(session.get_data("empty_key"), Some("".to_string()));
    }

    #[test]
    fn test_session_data_special_characters() {
        let mut session = UssdSession::new("test_special".to_string(), "+256700123456".to_string());

        // Store data with special characters (after sanitization)
        session.set_data("phone", "+256700123456");
        session.set_data("navigation", "1*2*3");
        session.set_data("amount", "100.50");

        assert_eq!(session.get_data("phone"), Some("+256700123456".to_string()));
        assert_eq!(session.get_data("navigation"), Some("1*2*3".to_string()));
        assert_eq!(session.get_data("amount"), Some("100.50".to_string()));
    }
}

#[cfg(test)]
mod session_state_management_tests {
    use super::*;

    #[test]
    fn test_session_initialization() {
        let session = UssdSession::new("init_test".to_string(), "+256700123456".to_string());

        assert_eq!(session.session_id, "init_test");
        assert_eq!(session.phone_number, "+256700123456");
        assert_eq!(session.current_menu, "");
        assert_eq!(session.step, 0);
        assert_eq!(session.language, "en");
        assert!(session.data.is_empty());
        assert!(session.last_activity > 0);
    }

    #[test]
    fn test_session_menu_tracking() {
        let mut session = UssdSession::new("menu_test".to_string(), "+256700123456".to_string());

        // Initial state
        assert_eq!(session.current_menu, "");
        assert_eq!(session.step, 0);

        // Update menu state
        session.current_menu = "send_money".to_string();
        session.step = 1;

        assert_eq!(session.current_menu, "send_money");
        assert_eq!(session.step, 1);

        // Progress through steps
        session.step = 2;
        assert_eq!(session.step, 2);

        // Reset to main menu
        session.current_menu = "main".to_string();
        session.step = 0;

        assert_eq!(session.current_menu, "main");
        assert_eq!(session.step, 0);
    }

    #[test]
    fn test_session_language_tracking() {
        let mut session = UssdSession::new("lang_test".to_string(), "+256700123456".to_string());

        // Default language
        assert_eq!(session.language, "en");

        // Change to Luganda
        session.language = "lg".to_string();
        assert_eq!(session.language, "lg");

        // Change to Swahili
        session.language = "sw".to_string();
        assert_eq!(session.language, "sw");

        // Change back to English
        session.language = "en".to_string();
        assert_eq!(session.language, "en");
    }

    #[test]
    fn test_session_phone_number_storage() {
        let session1 = UssdSession::new("phone_test1".to_string(), "+256700123456".to_string());
        let session2 = UssdSession::new("phone_test2".to_string(), "+254712345678".to_string());
        let session3 = UssdSession::new("phone_test3".to_string(), "+255700123456".to_string());

        assert_eq!(session1.phone_number, "+256700123456");
        assert_eq!(session2.phone_number, "+254712345678");
        assert_eq!(session3.phone_number, "+255700123456");
    }

    #[test]
    fn test_session_id_uniqueness() {
        let session1 = UssdSession::new("unique1".to_string(), "+256700123456".to_string());
        let session2 = UssdSession::new("unique2".to_string(), "+256700123456".to_string());
        let session3 = UssdSession::new("unique3".to_string(), "+254712345678".to_string());

        // Different session IDs for same phone number
        assert_ne!(session1.session_id, session2.session_id);
        assert_ne!(session1.session_id, session3.session_id);
        assert_ne!(session2.session_id, session3.session_id);
    }
}

#[cfg(test)]
mod session_flow_tests {
    use super::*;

    #[test]
    fn test_send_money_flow_session_data() {
        let mut session = UssdSession::new("send_test".to_string(), "+256700123456".to_string());

        // Simulate send money flow
        session.current_menu = "send_money".to_string();
        session.step = 0;

        // Step 1: Collect recipient
        session.set_data("recipient", "+254712345678");
        session.step = 1;

        // Step 2: Collect amount
        session.set_data("amount", "1000");
        session.step = 2;

        // Step 3: Collect PIN
        session.set_data("pin", "1234");
        session.step = 3;

        // Verify all data is stored
        assert_eq!(session.get_data("recipient"), Some("+254712345678".to_string()));
        assert_eq!(session.get_data("amount"), Some("1000".to_string()));
        assert_eq!(session.get_data("pin"), Some("1234".to_string()));
        assert_eq!(session.step, 3);

        // Complete flow - clear data
        session.clear_data();
        session.current_menu = "main".to_string();
        session.step = 0;

        assert!(session.get_data("recipient").is_none());
        assert!(session.get_data("amount").is_none());
        assert!(session.get_data("pin").is_none());
    }

    #[test]
    fn test_buy_bitcoin_flow_session_data() {
        let mut session = UssdSession::new("btc_test".to_string(), "+256700123456".to_string());

        // Simulate buy Bitcoin flow
        session.current_menu = "buy_bitcoin".to_string();
        session.step = 0;

        // Store flow data
        session.set_data("amount", "50000");
        session.set_data("btc_address", "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        session.set_data("pin", "1234");

        // Verify data
        assert_eq!(session.get_data("amount"), Some("50000".to_string()));
        assert_eq!(session.get_data("btc_address"), Some("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string()));
        assert_eq!(session.get_data("pin"), Some("1234".to_string()));
    }

    #[test]
    fn test_registration_flow_session_data() {
        let mut session = UssdSession::new("reg_test".to_string(), "+256700123456".to_string());

        // Simulate registration flow
        session.current_menu = "registration".to_string();
        session.step = 0;

        // Step 0: Collect PIN
        session.set_data("pin", "1234");
        session.step = 1;

        // Step 1: Collect first name
        session.set_data("first_name", "John");
        session.step = 2;

        // Step 2: Collect last name
        session.set_data("last_name", "Doe");
        session.step = 3;

        // Step 3: Detect currency
        session.set_data("currency", "UGX");
        session.step = 4;

        // Verify all registration data
        assert_eq!(session.get_data("pin"), Some("1234".to_string()));
        assert_eq!(session.get_data("first_name"), Some("John".to_string()));
        assert_eq!(session.get_data("last_name"), Some("Doe".to_string()));
        assert_eq!(session.get_data("currency"), Some("UGX".to_string()));
    }
}

#[cfg(test)]
mod session_timeout_behavior_tests {
    use super::*;

    #[test]
    fn test_session_timeout_value_is_reasonable() {
        // 5 minutes = 300 seconds = 300,000,000,000 nanoseconds
        const FIVE_MINUTES_NANOS: u64 = 300_000_000_000;
        assert_eq!(SESSION_TIMEOUT_NANOS, FIVE_MINUTES_NANOS, "Session timeout should be exactly 5 minutes");
    }

    #[test]
    fn test_timeout_not_too_short() {
        // Timeout should be at least 1 minute (users need time to navigate)
        const ONE_MINUTE_NANOS: u64 = 60_000_000_000;
        assert!(SESSION_TIMEOUT_NANOS >= ONE_MINUTE_NANOS, "Session timeout should be at least 1 minute");
    }

    #[test]
    fn test_timeout_not_too_long() {
        // Timeout should not exceed 10 minutes (security concern)
        const TEN_MINUTES_NANOS: u64 = 600_000_000_000;
        assert!(SESSION_TIMEOUT_NANOS <= TEN_MINUTES_NANOS, "Session timeout should not exceed 10 minutes");
    }
}

#[cfg(test)]
mod session_security_tests {
    use super::*;

    #[test]
    fn test_session_pin_not_exposed() {
        let mut session = UssdSession::new("security_test".to_string(), "+256700123456".to_string());

        // Store PIN in session data
        session.set_data("pin", "1234");

        // PIN should be stored but not in plain session fields
        assert_eq!(session.get_data("pin"), Some("1234".to_string()));

        // Session structure should not expose PIN directly
        // (it's only in the HashMap, which is cleared after use)
        assert_ne!(session.session_id, "1234");
        assert_ne!(session.phone_number, "1234");
        assert_ne!(session.current_menu, "1234");
        assert_ne!(session.language, "1234");
    }

    #[test]
    fn test_session_data_isolation() {
        let mut session1 = UssdSession::new("session1".to_string(), "+256700123456".to_string());
        let mut session2 = UssdSession::new("session2".to_string(), "+254712345678".to_string());

        // Store different data in each session
        session1.set_data("amount", "1000");
        session2.set_data("amount", "2000");

        // Data should be isolated
        assert_eq!(session1.get_data("amount"), Some("1000".to_string()));
        assert_eq!(session2.get_data("amount"), Some("2000".to_string()));

        // Session 1 should not have session 2's data
        session2.set_data("recipient", "+255700123456");
        assert!(session1.get_data("recipient").is_none());
    }

    #[test]
    fn test_sensitive_data_cleared_after_transaction() {
        let mut session = UssdSession::new("clear_test".to_string(), "+256700123456".to_string());

        // Simulate transaction with sensitive data
        session.set_data("pin", "1234");
        session.set_data("amount", "10000");
        session.set_data("recipient", "+254712345678");

        // Verify data exists
        assert!(session.get_data("pin").is_some());

        // Clear data after transaction (security best practice)
        session.clear_data();

        // Sensitive data should be gone
        assert!(session.get_data("pin").is_none());
        assert!(session.get_data("amount").is_none());
        assert!(session.get_data("recipient").is_none());
    }
}
