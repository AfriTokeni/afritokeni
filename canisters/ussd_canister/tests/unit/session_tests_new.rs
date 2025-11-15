// Comprehensive session management tests

#[cfg(test)]
mod session_creation_tests {
    
    use crate::common::{create_test_session, test_phones};

    #[test]
    fn test_create_session_with_defaults() {
        let session = create_test_session(test_phones::VALID_KENYA, "test_123");
        
        assert_eq!(session.session_id, "test_123");
        assert_eq!(session.phone_number, test_phones::VALID_KENYA);
        assert_eq!(session.current_menu, "main");
        assert_eq!(session.step, 0);
        assert_eq!(session.language, "en");
        assert!(session.data.is_empty());
    }

    #[test]
    fn test_session_id_format() {
        let session = create_test_session(test_phones::VALID_UGANDA, "session_456");
        assert!(session.session_id.starts_with("session_"));
    }

    #[test]
    fn test_playground_session_detection() {
        let session = create_test_session(test_phones::VALID_KENYA, "playground_session_789");
        assert!(session.session_id.starts_with("playground_"));
    }
}

#[cfg(test)]
mod session_data_tests {
    
    use crate::common::create_test_session;

    #[test]
    fn test_set_and_get_data() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        session.data.insert("pin".to_string(), "1234".to_string());
        session.data.insert("amount".to_string(), "1000".to_string());
        
        assert_eq!(session.data.get("pin"), Some(&"1234".to_string()));
        assert_eq!(session.data.get("amount"), Some(&"1000".to_string()));
    }

    #[test]
    fn test_get_nonexistent_data() {
        let session = create_test_session("+254712345678", "test_123");
        assert_eq!(session.data.get("nonexistent"), None);
    }

    #[test]
    fn test_update_data() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        session.data.insert("amount".to_string(), "1000".to_string());
        assert_eq!(session.data.get("amount"), Some(&"1000".to_string()));
        
        session.data.insert("amount".to_string(), "2000".to_string());
        assert_eq!(session.data.get("amount"), Some(&"2000".to_string()));
    }

    #[test]
    fn test_clear_data() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        session.data.insert("pin".to_string(), "1234".to_string());
        session.data.insert("amount".to_string(), "1000".to_string());
        assert_eq!(session.data.len(), 2);
        
        session.data.clear();
        assert!(session.data.is_empty());
    }
}

#[cfg(test)]
mod session_navigation_tests {
    
    use crate::common::{create_test_session, create_session_at};

    #[test]
    fn test_navigate_to_menu() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        session.current_menu = "bitcoin".to_string();
        assert_eq!(session.current_menu, "bitcoin");
        
        session.current_menu = "usdc".to_string();
        assert_eq!(session.current_menu, "usdc");
    }

    #[test]
    fn test_step_progression() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        assert_eq!(session.step, 0);
        
        session.step = 1;
        assert_eq!(session.step, 1);
        
        session.step = 2;
        assert_eq!(session.step, 2);
    }

    #[test]
    fn test_reset_to_main_menu() {
        let mut session = create_session_at("+254712345678", "test_123", "bitcoin", 3);
        
        session.current_menu = "main".to_string();
        session.step = 0;
        
        assert_eq!(session.current_menu, "main");
        assert_eq!(session.step, 0);
    }

    #[test]
    fn test_language_change() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        assert_eq!(session.language, "en");
        
        session.language = "sw".to_string();
        assert_eq!(session.language, "sw");
        
        session.language = "lg".to_string();
        assert_eq!(session.language, "lg");
    }
}

#[cfg(test)]
mod session_timeout_tests {
    
    use crate::common::create_test_session;

    #[test]
    fn test_session_activity_timestamp() {
        let mut session = create_test_session("+254712345678", "test_123");
        
        let now = 1699564800; // Example timestamp
        session.last_activity = now;
        
        assert_eq!(session.last_activity, now);
    }

    #[test]
    fn test_session_timeout_calculation() {
        let session_timeout = 180; // 3 minutes
        let now = 1000;
        let last_activity = 800;
        
        let elapsed = now - last_activity;
        assert_eq!(elapsed, 200);
        assert!(elapsed > session_timeout);
    }

    #[test]
    fn test_session_not_timed_out() {
        let session_timeout = 180;
        let now = 1000;
        let last_activity = 950;
        
        let elapsed = now - last_activity;
        assert!(elapsed < session_timeout);
    }
}
