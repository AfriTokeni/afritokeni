// Unit tests module
#[path = "session_tests_new.rs"]
mod session_tests;
mod validation_tests;
mod rate_limit_tests;
mod swap_step_test;

// Request validation tests (previously misnamed as "integration")
mod request_validation;

// New comprehensive tests for security and functionality improvements
mod currency_detection_tests;
mod input_sanitization_tests;
mod session_cleanup_tests;
mod enhanced_phone_validation_tests;

// Flow tests
// Note: send_money_tests requires mock setup - disabled until mocking is properly configured
// mod send_money_tests;
