// Integration tests for USSD send money flow
use super::*;

#[test]
fn test_send_money_flow_complete() {
    let env = get_test_env();
    
    // Setup: Register sender and receiver
    let sender_phone = "+256700111001";
    let receiver_phone = "+256700111002";
    
    let sender_id = env.register_user_direct(
        sender_phone, "Alice", "Sender", "alice@test.com", "UGX", "1111"
    ).expect("Sender registration should succeed");
    
    env.register_user_direct(
        receiver_phone, "Bob", "Receiver", "bob@test.com", "UGX", "2222"
    ).expect("Receiver registration should succeed");
    
    // Give sender balance (in cents: 10,000,000 cents = 100,000 UGX)
    env.set_fiat_balance(&sender_id, "UGX", 10_000_000).expect("Should set balance");
    
    let session_id = "send_money_1";
    
    // Navigate to send money: Main menu -> Local Currency -> Send Money
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, "1");
    assert!(continue_session, "Should show local currency menu");
    assert!(response.contains("Send") || response.contains("money"), 
        "Should show send money option. Got: {}", response);
    
    // Select send money
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, "1*1");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("phone") || response.contains("number") || response.contains("recipient"), 
        "Should ask for recipient phone. Got: {}", response);
    
    // Enter recipient phone
    let input = format!("1*1*{}", receiver_phone);
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, &input);
    assert!(continue_session, "Session should continue");
    assert!(response.contains("amount") || response.contains("Enter"), 
        "Should ask for amount. Got: {}", response);
    
    // Enter amount
    let input = format!("1*1*{}*50000", receiver_phone);
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, &input);
    assert!(continue_session, "Session should continue");
    assert!(response.contains("PIN") || response.contains("pin"), 
        "Should ask for PIN. Got: {}", response);
    
    // Enter PIN to confirm
    let input = format!("1*1*{}*50000*1111", receiver_phone);
    let (response, _) = env.process_ussd(session_id, sender_phone, &input);
    assert!(response.contains("successful") || response.contains("Success") || response.contains("sent"), 
        "Should show success message. Got: {}", response);
    
    // Verify balances changed
    let sender_balance = env.check_fiat_balance(&sender_id, "UGX")
        .expect("Should check sender balance");
    // Sender sent 50,000 UGX (5,000,000 cents) + 0.5% fee (25,000 cents) = 5,025,000 cents deducted
    // Started with 10,000,000 cents, left with 4,975,000 cents
    assert_eq!(sender_balance, 4_975_000, "Sender should have 4,975,000 cents left (49,750 UGX)");

    let receiver_balance = env.check_fiat_balance(receiver_phone, "UGX")
        .expect("Should check receiver balance");
    // Receiver got 50,000 UGX = 5,000,000 cents
    assert_eq!(receiver_balance, 5_000_000, "Receiver should have 5,000,000 cents (50,000 UGX)");
}

#[test]
fn test_send_money_insufficient_balance() {
    let env = get_test_env();
    
    let sender_phone = "+256700222001";
    let receiver_phone = "+256700222002";
    
    let sender_id = env.register_user_direct(
        sender_phone, "Charlie", "Poor", "charlie@test.com", "UGX", "3333"
    ).expect("Sender registration should succeed");
    
    env.register_user_direct(
        receiver_phone, "Dave", "Rich", "dave@test.com", "UGX", "4444"
    ).expect("Receiver registration should succeed");
    
    // Give sender small balance
    env.set_fiat_balance(&sender_id, "UGX", 10_000).expect("Should set balance");
    
    let session_id = "send_money_2";
    
    // Try to send more than balance
    let input = format!("1*1*{}*50000*3333", receiver_phone);
    let (response, _) = env.process_ussd(session_id, sender_phone, &input);
    
    assert!(response.contains("Insufficient") || response.contains("balance") || response.contains("enough"), 
        "Should show insufficient balance error. Got: {}", response);
}

#[test]
fn test_send_money_invalid_recipient() {
    let env = get_test_env();
    
    let sender_phone = "+256700333001";
    
    let sender_id = env.register_user_direct(
        sender_phone, "Eve", "Sender", "eve@test.com", "UGX", "5555"
    ).expect("Sender registration should succeed");
    
    env.set_fiat_balance(&sender_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "send_money_3";
    
    // Try to send to non-existent user
    let invalid_phone = "+256700999999";
    let input = format!("1*1*{}*10000*5555", invalid_phone);
    let (response, _) = env.process_ussd(session_id, sender_phone, &input);
    
    assert!(response.contains("not found") || response.contains("not registered") || response.contains("does not exist"), 
        "Should show recipient not found error. Got: {}", response);
}

#[test]
fn test_send_money_wrong_pin() {
    let env = get_test_env();
    
    let sender_phone = "+256700444001";
    let receiver_phone = "+256700444002";
    
    let sender_id = env.register_user_direct(
        sender_phone, "Frank", "Sender", "frank@test.com", "UGX", "6666"
    ).expect("Sender registration should succeed");
    
    env.register_user_direct(
        receiver_phone, "Grace", "Receiver", "grace@test.com", "UGX", "7777"
    ).expect("Receiver registration should succeed");
    
    env.set_fiat_balance(&sender_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "send_money_4";
    
    // Try with wrong PIN
    let input = format!("1*1*{}*10000*9999", receiver_phone);
    let (response, _) = env.process_ussd(session_id, sender_phone, &input);
    
    assert!(response.contains("Invalid PIN") || response.contains("incorrect") || response.contains("wrong"), 
        "Should show invalid PIN error. Got: {}", response);
}

#[test]
fn test_send_money_zero_amount() {
    let env = get_test_env();
    
    let sender_phone = "+256700555001";
    let receiver_phone = "+256700555002";
    
    let sender_id = env.register_user_direct(
        sender_phone, "Henry", "Sender", "henry@test.com", "UGX", "8888"
    ).expect("Sender registration should succeed");
    
    env.register_user_direct(
        receiver_phone, "Ivy", "Receiver", "ivy@test.com", "UGX", "9999"
    ).expect("Receiver registration should succeed");
    
    env.set_fiat_balance(&sender_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "send_money_5";
    
    // Try to send zero amount
    let input = format!("1*1*{}*0*8888", receiver_phone);
    let (response, _) = env.process_ussd(session_id, sender_phone, &input);
    
    assert!(response.contains("greater than 0") || response.contains("Invalid amount") || response.contains("zero"), 
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_send_money_navigation_flow() {
    let env = get_test_env();
    
    let sender_phone = "+256700666001";
    
    env.register_user_direct(
        sender_phone, "Jack", "User", "jack@test.com", "UGX", "1234"
    ).expect("Registration should succeed");
    
    let session_id = "send_money_6";
    
    // Step 1: Main menu
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, "");
    assert!(continue_session, "Should show main menu");
    assert!(response.contains("Welcome") || response.contains("menu"), 
        "Should show welcome or menu. Got: {}", response);
    
    // Step 2: Select Local Currency (option 1)
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, "1");
    assert!(continue_session, "Should show local currency menu");
    assert!(response.contains("Send") || response.contains("Balance") || response.contains("Deposit"), 
        "Should show local currency options. Got: {}", response);
    
    // Step 3: Select Send Money (option 1)
    let (response, continue_session) = env.process_ussd(session_id, sender_phone, "1*1");
    assert!(continue_session, "Should start send money flow");
    assert!(response.contains("phone") || response.contains("recipient") || response.contains("number"), 
        "Should ask for recipient. Got: {}", response);
}
