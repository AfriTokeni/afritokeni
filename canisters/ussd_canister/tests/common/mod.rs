// Common test utilities and helpers
use ussd_canister::core::session::UssdSession;
use std::collections::HashMap;

/// Create a test session with default values
pub fn create_test_session(phone: &str, session_id: &str) -> UssdSession {
    UssdSession {
        session_id: session_id.to_string(),
        phone_number: phone.to_string(),
        current_menu: "main".to_string(),
        step: 0,
        language: "en".to_string(),
        data: HashMap::new(),
        last_activity: 0,
    }
}

/// Create a test session with custom menu and step
pub fn create_session_at(phone: &str, session_id: &str, menu: &str, step: u32) -> UssdSession {
    UssdSession {
        session_id: session_id.to_string(),
        phone_number: phone.to_string(),
        current_menu: menu.to_string(),
        step,
        language: "en".to_string(),
        data: HashMap::new(),
        last_activity: 0,
    }
}

/// Test phone numbers for different scenarios
pub mod test_phones {
    pub const VALID_KENYA: &str = "+254712345678";
    pub const VALID_UGANDA: &str = "+256700123456";
    pub const VALID_TANZANIA: &str = "+255712345678";
    pub const INVALID_SHORT: &str = "+25412";
    pub const INVALID_NO_PLUS: &str = "254712345678";
    pub const TEST_PHONE: &str = "+254700123456"; // For integration tests (bypasses rate limit)
}

/// Test session IDs
pub mod test_sessions {
    pub const NORMAL: &str = "test_session_123";
    pub const PLAYGROUND: &str = "playground_session_456";
}
