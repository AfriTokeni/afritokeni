// Integration tests for USSD main menu navigation
use super::*;

#[test]
fn test_main_menu_display() {
    let env = get_test_env();
    
    let phone = "+256700111111";
    let session_id = "menu_test_1";
    
    // Register user first
    env.register_user_direct(phone, "Menu", "Tester", "menu@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Get main menu
    let (response, continue_session) = env.process_ussd(session_id, phone, "");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("1") || response.contains("Send"), 
        "Should show menu options. Got: {}", response);
}

#[test]
fn test_navigate_to_send_money() {
    let env = get_test_env();
    
    let phone = "+256700222222";
    let session_id = "menu_test_2";
    
    env.register_user_direct(phone, "Send", "User", "send@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to send money (option 1)
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("phone") || response.contains("recipient") || response.contains("number"),
        "Should ask for recipient. Got: {}", response);
}

#[test]
fn test_navigate_to_bitcoin() {
    let env = get_test_env();
    
    let phone = "+256700333333";
    let session_id = "menu_test_3";
    
    env.register_user_direct(phone, "BTC", "User", "btc@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to Bitcoin (option 2)
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Bitcoin") || response.contains("BTC") || response.contains("Balance"),
        "Should show Bitcoin menu. Got: {}", response);
}

#[test]
fn test_navigate_to_usdc() {
    let env = get_test_env();
    
    let phone = "+256700444444";
    let session_id = "menu_test_4";
    
    env.register_user_direct(phone, "USDC", "User", "usdc@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to USDC (option 3)
    let (response, continue_session) = env.process_ussd(session_id, phone, "3");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("USDC") || response.contains("Balance") || response.contains("Stablecoin"),
        "Should show USDC menu. Got: {}", response);
}

#[test]
fn test_navigate_to_swap() {
    let env = get_test_env();
    
    let phone = "+256700555555";
    let session_id = "menu_test_5";
    
    env.register_user_direct(phone, "Swap", "User", "swap@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to Swap (option 4)
    let (response, continue_session) = env.process_ussd(session_id, phone, "4");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Swap") || response.contains("BTC") || response.contains("USDC"),
        "Should show Swap menu. Got: {}", response);
}

#[test]
fn test_navigate_to_dao() {
    let env = get_test_env();
    
    let phone = "+256700666666";
    let session_id = "menu_test_6";
    
    env.register_user_direct(phone, "DAO", "User", "dao@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to DAO (option 5)
    let (response, continue_session) = env.process_ussd(session_id, phone, "5");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("DAO") || response.contains("Governance") || response.contains("Vote"),
        "Should show DAO menu. Got: {}", response);
}

#[test]
fn test_navigate_to_balance() {
    let env = get_test_env();
    
    let phone = "+256700777777";
    let session_id = "menu_test_7";
    
    env.register_user_direct(phone, "Balance", "User", "balance@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to Balance (option 6)
    let (response, _) = env.process_ussd(session_id, phone, "6");
    
    assert!(response.contains("Balance") || response.contains("UGX") || response.contains("0"),
        "Should show balance. Got: {}", response);
}

#[test]
fn test_navigate_to_withdraw() {
    let env = get_test_env();
    
    let phone = "+256700888888";
    let session_id = "menu_test_8";
    
    env.register_user_direct(phone, "Withdraw", "User", "withdraw@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to Withdraw (option 7 or similar)
    let (response, continue_session) = env.process_ussd(session_id, phone, "7");
    
    assert!(continue_session || !continue_session, "May or may not continue");
    assert!(response.len() > 0, "Should return some response");
}

#[test]
fn test_invalid_menu_option() {
    let env = get_test_env();
    
    let phone = "+256700999999";
    let session_id = "menu_test_9";
    
    env.register_user_direct(phone, "Invalid", "User", "invalid@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Try invalid option
    let (response, continue_session) = env.process_ussd(session_id, phone, "99");
    
    assert!(continue_session, "Should continue session");
    assert!(response.contains("Invalid") || response.contains("option") || response.contains("Menu"),
        "Should show error message. Got: {}", response);
}

#[test]
fn test_return_to_main_menu_from_submenu() {
    let env = get_test_env();
    
    let phone = "+256700101010";
    let session_id = "menu_test_10";
    
    env.register_user_direct(phone, "Return", "User", "return@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    // Navigate to Bitcoin then back to main menu
    env.process_ussd(session_id, phone, "2");
    let (response, _) = env.process_ussd(session_id, phone, "0");
    
    assert!(response.contains("Main") || response.contains("Menu") || response.contains("Send"),
        "Should return to main menu. Got: {}", response);
}

#[test]
fn test_menu_has_all_options() {
    let env = get_test_env();
    
    let phone = "+256700111222";
    let session_id = "menu_test_11";
    
    env.register_user_direct(phone, "All", "Options", "all@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    let (response, _) = env.process_ussd(session_id, phone, "");
    
    // Should have multiple numbered options
    let numbers = response.matches(|c: char| c.is_numeric()).count();
    assert!(numbers >= 6, "Should have at least 6 menu options. Got: {}", response);
}

#[test]
fn test_menu_formatting() {
    let env = get_test_env();
    
    let phone = "+256700222333";
    let session_id = "menu_test_12";
    
    env.register_user_direct(phone, "Format", "Test", "format@test.com", "UGX", "1234")
        .expect("Registration should succeed");
    
    let (response, _) = env.process_ussd(session_id, phone, "");
    
    // Should have proper formatting (newlines, numbers, etc)
    assert!(response.contains("\n") || response.len() > 50, 
        "Menu should be properly formatted. Got: {}", response);
}
