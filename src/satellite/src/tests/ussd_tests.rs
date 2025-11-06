use crate::ussd::process_ussd_menu;

#[test]
fn test_new_session_shows_main_menu() {
    let (response, continue_session) = process_ussd_menu("", "+254700000000");
    
    assert!(continue_session, "New session should continue");
    assert!(response.contains("Welcome to AfriTokeni"), "Should show welcome message");
    assert!(response.contains("1. Check Balance"), "Should show option 1");
    assert!(response.contains("2. Send Money"), "Should show option 2");
    assert!(response.contains("3. Buy ckBTC"), "Should show option 3");
    assert!(response.contains("4. Buy ckUSDC"), "Should show option 4");
    assert!(response.contains("5. Withdraw"), "Should show option 5");
    assert!(response.contains("0. Exit"), "Should show option 0");
}

#[test]
fn test_check_balance_ends_session() {
    let (response, continue_session) = process_ussd_menu("1", "+254700000000");
    
    assert!(!continue_session, "Check balance should end session");
    assert!(response.contains("Balance"), "Should show balance");
    assert!(response.contains("KES"), "Should show KES balance");
    assert!(response.contains("ckBTC"), "Should show ckBTC balance");
    assert!(response.contains("ckUSDC"), "Should show ckUSDC balance");
}

#[test]
fn test_send_money_step1_asks_for_recipient() {
    let (response, continue_session) = process_ussd_menu("2", "+254700000000");
    
    assert!(continue_session, "Should continue to ask for recipient");
    assert!(response.contains("recipient"), "Should ask for recipient");
    assert!(response.contains("phone number"), "Should mention phone number");
}

#[test]
fn test_send_money_step2_asks_for_amount() {
    let (response, continue_session) = process_ussd_menu("2*254711111111", "+254700000000");
    
    assert!(continue_session, "Should continue to ask for amount");
    assert!(response.contains("amount"), "Should ask for amount");
    assert!(response.contains("KES"), "Should mention KES");
}

#[test]
fn test_send_money_step3_confirms_transaction() {
    let (response, continue_session) = process_ussd_menu("2*254711111111*100", "+254700000000");
    
    assert!(!continue_session, "Should end after confirmation");
    assert!(response.contains("Sending"), "Should confirm sending");
    assert!(response.contains("100"), "Should show amount");
    assert!(response.contains("254711111111"), "Should show recipient");
    assert!(response.contains("pending"), "Should mention pending status");
}

#[test]
fn test_buy_ckbtc_asks_for_amount() {
    let (response, continue_session) = process_ussd_menu("3", "+254700000000");
    
    assert!(continue_session, "Should continue to ask for amount");
    assert!(response.contains("amount"), "Should ask for amount");
    assert!(response.contains("ckBTC"), "Should mention ckBTC");
}

#[test]
fn test_buy_ckbtc_confirms_purchase() {
    let (response, continue_session) = process_ussd_menu("3*500", "+254700000000");
    
    assert!(!continue_session, "Should end after confirmation");
    assert!(response.contains("Buying"), "Should confirm buying");
    assert!(response.contains("ckBTC"), "Should mention ckBTC");
    assert!(response.contains("500"), "Should show amount");
}

#[test]
fn test_buy_ckusdc_asks_for_amount() {
    let (response, continue_session) = process_ussd_menu("4", "+254700000000");
    
    assert!(continue_session, "Should continue to ask for amount");
    assert!(response.contains("amount"), "Should ask for amount");
    assert!(response.contains("ckUSDC"), "Should mention ckUSDC");
}

#[test]
fn test_buy_ckusdc_confirms_purchase() {
    let (response, continue_session) = process_ussd_menu("4*1000", "+254700000000");
    
    assert!(!continue_session, "Should end after confirmation");
    assert!(response.contains("Buying"), "Should confirm buying");
    assert!(response.contains("ckUSDC"), "Should mention ckUSDC");
    assert!(response.contains("1000"), "Should show amount");
}

#[test]
fn test_withdraw_asks_for_amount() {
    let (response, continue_session) = process_ussd_menu("5", "+254700000000");
    
    assert!(continue_session, "Should continue to ask for amount");
    assert!(response.contains("amount"), "Should ask for amount");
    assert!(response.contains("withdraw"), "Should mention withdraw");
}

#[test]
fn test_withdraw_confirms() {
    let (response, continue_session) = process_ussd_menu("5*200", "+254700000000");
    
    assert!(!continue_session, "Should end after confirmation");
    assert!(response.contains("Withdrawing"), "Should confirm withdrawing");
    assert!(response.contains("200"), "Should show amount");
    assert!(response.contains("agent"), "Should mention agent");
}

#[test]
fn test_exit_option() {
    let (response, continue_session) = process_ussd_menu("0", "+254700000000");
    
    assert!(!continue_session, "Exit should end session");
    assert!(response.contains("Thank you"), "Should show thank you message");
    assert!(response.contains("AfriTokeni"), "Should mention AfriTokeni");
}

#[test]
fn test_invalid_option() {
    let (response, continue_session) = process_ussd_menu("99", "+254700000000");
    
    assert!(!continue_session, "Invalid option should end session");
    assert!(response.contains("Invalid"), "Should show invalid message");
}

#[test]
fn test_security_sql_injection_treated_as_invalid() {
    let (response, continue_session) = process_ussd_menu("1'; DROP TABLE users; --", "+254700000000");
    
    assert!(!continue_session, "Should end session");
    assert!(response.contains("Invalid"), "Should treat as invalid input");
}

#[test]
fn test_security_xss_treated_as_invalid() {
    let (response, continue_session) = process_ussd_menu("<script>alert('xss')</script>", "+254700000000");
    
    assert!(!continue_session, "Should end session");
    assert!(response.contains("Invalid"), "Should treat as invalid input");
}

#[test]
fn test_unicode_characters_handled() {
    let (response, continue_session) = process_ussd_menu("1*émoji🎉", "+254700000000");
    
    // Should not panic, should return some response
    assert!(!response.is_empty(), "Should return a response");
}

#[test]
fn test_very_long_input() {
    let long_input = "1*".to_string() + &"9".repeat(1000);
    let (response, continue_session) = process_ussd_menu(&long_input, "+254700000000");
    
    // Should handle gracefully
    assert!(!response.is_empty(), "Should return a response");
    assert!(!continue_session, "Should end session for invalid input");
}

#[test]
fn test_special_characters_in_amount() {
    let (response, continue_session) = process_ussd_menu("3*#*0*9", "+254700000000");
    
    // Should handle special characters
    assert!(!response.is_empty(), "Should return a response");
}

#[test]
fn test_empty_parts_in_flow() {
    let (response, continue_session) = process_ussd_menu("2**", "+254700000000");
    
    // Should handle empty parts
    assert!(!response.is_empty(), "Should return a response");
}
