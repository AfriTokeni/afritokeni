// Integration tests for USSD stateless behavior
// USSD is stateless - each request is independent, state is in the input text
use super::*;

#[test]
fn test_ussd_is_stateless() {
    let env = get_test_env();
    
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "Stateless", "User", "stateless@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Each call should be independent - no session state stored
    // Call 1: Check balance
    let (response1, _) = env.process_ussd("session_1", phone, "6");
    
    // Call 2: Bitcoin menu (completely independent)
    let (response2, _) = env.process_ussd("session_2", phone, "2");
    
    // Call 3: Back to balance (should work independently)
    let (response3, _) = env.process_ussd("session_3", phone, "6");
    
    // Responses should be consistent for same input
    assert_eq!(response1, response3, "Same input should give same output (stateless)");
    assert_ne!(response1, response2, "Different inputs should give different outputs");
}

#[test]
fn test_ussd_input_determines_state() {
    let env = get_test_env();
    
    let phone = "+256700222222";
    
    env.register_user_direct(phone, "Input", "State", "input@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // State is determined by input text, not session
    // Direct navigation via input text
    let (response, _) = env.process_ussd("any_session", phone, "2*1"); // Bitcoin -> Balance
    
    assert!(response.contains("Bitcoin") || response.contains("BTC") || response.contains("Balance"),
        "Should navigate based on input text. Got: {}", response);
}

#[test]
fn test_same_input_same_output() {
    let env = get_test_env();
    
    let phone = "+256700333333";
    
    env.register_user_direct(phone, "Same", "Output", "same@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Call same input multiple times
    let (response1, _) = env.process_ussd("session_a", phone, "6");
    let (response2, _) = env.process_ussd("session_b", phone, "6");
    let (response3, _) = env.process_ussd("session_c", phone, "6");
    
    // All should be identical (stateless)
    assert_eq!(response1, response2, "Stateless: same input = same output");
    assert_eq!(response2, response3, "Stateless: same input = same output");
}

#[test]
fn test_no_session_carryover() {
    let env = get_test_env();
    
    let phone = "+256700444444";
    
    env.register_user_direct(phone, "No", "Carryover", "no@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate in one session
    env.process_ussd("session_1", phone, "2");
    env.process_ussd("session_1", phone, "1");
    
    // New session should start fresh
    let (response, _) = env.process_ussd("session_2", phone, "");
    
    // Should show main menu, not continue from previous session
    assert!(response.contains("Send") || response.contains("Bitcoin") || response.contains("Menu"),
        "New session should start fresh. Got: {}", response);
}

#[test]
fn test_concurrent_requests_independent() {
    let env = get_test_env();
    
    let phone = "+256700555555";
    
    env.register_user_direct(phone, "Concurrent", "User", "concurrent@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Simulate concurrent requests with different inputs
    let (response1, _) = env.process_ussd("concurrent_1", phone, "2");
    let (response2, _) = env.process_ussd("concurrent_2", phone, "3");
    let (response3, _) = env.process_ussd("concurrent_3", phone, "6");
    
    // All should be independent
    assert!(response1.contains("Bitcoin") || response1.contains("BTC"));
    assert!(response2.contains("USDC"));
    assert!(response3.contains("Balance") || response3.contains("UGX"));
}

#[test]
fn test_ussd_text_parsing() {
    let env = get_test_env();
    
    let phone = "+256700666666";
    
    env.register_user_direct(phone, "Parse", "Test", "parse@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Test USSD text format: *123*option1*option2#
    // The text parameter contains the user's navigation
    let (response, _) = env.process_ussd("session", phone, "2*1");
    
    // Should parse and navigate correctly
    assert!(response.len() > 0, "Should parse USSD text input");
}

#[test]
fn test_empty_text_shows_main_menu() {
    let env = get_test_env();
    
    let phone = "+256700777777";
    
    env.register_user_direct(phone, "Empty", "Text", "empty@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Empty text should show main menu
    let (response, _) = env.process_ussd("session", phone, "");
    
    assert!(response.contains("Send") || response.contains("Bitcoin") || response.contains("1"),
        "Empty text should show main menu. Got: {}", response);
}

#[test]
fn test_ussd_idempotency() {
    let env = get_test_env();
    
    let phone = "+256700888888";
    
    env.register_user_direct(phone, "Idempotent", "Test", "idempotent@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Same request should give same result (idempotent)
    let input = "2*1"; // Bitcoin -> Balance
    
    let (response1, _) = env.process_ussd("session_1", phone, input);
    let (response2, _) = env.process_ussd("session_2", phone, input);
    
    assert_eq!(response1, response2, "USSD should be idempotent");
}

#[test]
fn test_different_users_independent() {
    let env = get_test_env();
    
    let phone1 = "+256700999001";
    let phone2 = "+256700999002";
    
    env.register_user_direct(phone1, "User", "One", "user1@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    env.register_user_direct(phone2, "User", "Two", "user2@test.com", "KES", "1234")
        .expect("Registration should succeed");
    
    // Same input, different users
    let (response1, _) = env.process_ussd("session", phone1, "6");
    let (response2, _) = env.process_ussd("session", phone2, "6");
    
    // Should show different currencies
    assert!(response1.contains("UGX"), "User 1 should see UGX");
    assert!(response2.contains("KES"), "User 2 should see KES");
}
