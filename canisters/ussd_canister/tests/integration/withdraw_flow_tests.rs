// Integration tests for USSD withdraw flow
use super::*;

#[test]
fn test_withdraw_flow_navigation() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Alice", "Withdraw", "alice@test.com", "UGX", "1111"
    ).expect("Registration should succeed");
    
    // Give user balance
    env.set_fiat_balance(&user_id, "UGX", 500_000).expect("Should set balance");
    
    let session_id = "withdraw_test_1";
    
    // Navigate to Local Currency -> Withdraw
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    assert!(continue_session, "Should show local currency menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "1*4");
    assert!(continue_session, "Should start withdraw flow");
    assert!(response.contains("amount") || response.contains("Enter") || response.contains("withdraw"), 
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_withdraw_insufficient_balance() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Bob", "Poor", "bob@test.com", "UGX", "2222"
    ).expect("Registration should succeed");
    
    // Give user small balance
    env.set_fiat_balance(&user_id, "UGX", 10_000).expect("Should set balance");
    
    let session_id = "withdraw_test_2";
    
    // Try to withdraw more than balance
    let agent_id = "AGENT001";
    let input = format!("1*4*100000*{}*2222", agent_id);
    let (response, _) = env.process_ussd(session_id, phone, &input);
    
    assert!(response.contains("Insufficient") || response.contains("balance") || response.contains("enough"), 
        "Should show insufficient balance error. Got: {}", response);
}

#[test]
fn test_withdraw_zero_amount() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Charlie", "User", "charlie@test.com", "UGX", "3333"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "withdraw_test_3";
    
    // Try to withdraw zero amount
    let agent_id = "AGENT002";
    let input = format!("1*4*0*{}*3333", agent_id);
    let (response, _) = env.process_ussd(session_id, phone, &input);
    
    assert!(response.contains("greater than 0") || response.contains("Invalid amount") || response.contains("zero"), 
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_withdraw_wrong_pin() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Dave", "User", "dave@test.com", "UGX", "4444"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "withdraw_test_4";
    
    // Try with wrong PIN
    let agent_id = "AGENT003";
    let input = format!("1*4*50000*{}*9999", agent_id);
    let (response, _) = env.process_ussd(session_id, phone, &input);
    
    assert!(response.contains("Invalid PIN") || response.contains("incorrect") || response.contains("wrong"), 
        "Should show invalid PIN error. Got: {}", response);
}

#[test]
fn test_withdraw_requires_agent_id() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Eve", "User", "eve@test.com", "UGX", "5555"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "withdraw_test_5";
    
    // Navigate to withdraw and enter amount
    env.process_ussd(session_id, phone, "1");
    let (response, continue_session) = env.process_ussd(session_id, phone, "1*4");
    assert!(continue_session, "Should start withdraw flow");
    
    // After entering amount, should ask for agent ID
    let (response, continue_session) = env.process_ussd(session_id, phone, "1*4*50000");
    assert!(continue_session, "Should continue to ask for agent");
    assert!(response.contains("agent") || response.contains("Agent") || response.contains("code"), 
        "Should ask for agent ID. Got: {}", response);
}

#[test]
fn test_withdraw_menu_structure() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    env.register_user_direct(
        phone, "Frank", "User", "frank@test.com", "UGX", "6666"
    ).expect("Registration should succeed");
    
    let session_id = "withdraw_test_6";
    
    // Check local currency menu has withdraw option
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    assert!(continue_session, "Should show local currency menu");
    assert!(response.contains("Withdraw") || response.contains("withdraw"), 
        "Should show withdraw option. Got: {}", response);
}

#[test]
fn test_withdraw_step_by_step() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Grace", "User", "grace@test.com", "UGX", "7777"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "UGX", 200_000).expect("Should set balance");
    
    let session_id = "withdraw_test_7";
    
    // Step 1: Main menu
    let (response, continue_session) = env.process_ussd(session_id, phone, "");
    assert!(continue_session, "Should show main menu");
    
    // Step 2: Select Local Currency
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    assert!(continue_session, "Should show local currency menu");
    assert!(response.contains("Withdraw") || response.contains("withdraw"), 
        "Should show withdraw option. Got: {}", response);
    
    // Step 3: Select Withdraw
    let (response, continue_session) = env.process_ussd(session_id, phone, "1*4");
    assert!(continue_session, "Should start withdraw flow");
    assert!(response.contains("amount") || response.contains("Enter"), 
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_withdraw_balance_check_before() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Henry", "User", "henry@test.com", "UGX", "8888"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "UGX", 150_000).expect("Should set balance");
    
    let session_id = "withdraw_test_8";
    
    // Check balance first
    env.process_ussd(session_id, phone, "1");
    let (response, _) = env.process_ussd(session_id, phone, "1*2");
    assert!(response.contains("1500") || response.contains("1,500"), 
        "Should show initial balance. Got: {}", response);
    
    // Now try to withdraw (should work if balance is sufficient)
    let session_id_2 = "withdraw_test_8_withdraw";
    env.process_ussd(session_id_2, phone, "1");
    let (response, continue_session) = env.process_ussd(session_id_2, phone, "1*4");
    assert!(continue_session, "Should allow withdraw with sufficient balance");
}

#[test]
fn test_withdraw_invalid_agent_id() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Ivy", "User", "ivy@test.com", "UGX", "9999"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id, "UGX", 100_000).expect("Should set balance");
    
    let session_id = "withdraw_test_9";
    
    // Try with empty agent ID
    let input = "1*4*50000**9999";
    let (response, _) = env.process_ussd(session_id, phone, input);
    
    // Should show error about agent ID
    assert!(response.contains("agent") || response.contains("Agent") || response.contains("Invalid") || response.contains("required"), 
        "Should show agent ID error. Got: {}", response);
}

#[test]
fn test_withdraw_large_amount() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Jack", "Rich", "jack@test.com", "UGX", "1010"
    ).expect("Registration should succeed");
    
    // Give user large balance
    env.set_fiat_balance(&user_id, "UGX", 10_000_000).expect("Should set balance");
    
    let session_id = "withdraw_test_10";
    
    // Try to withdraw large amount
    let agent_id = "AGENT010";
    let input = format!("1*4*5000000*{}*1010", agent_id);
    let (response, _) = env.process_ussd(session_id, phone, &input);
    
    // Should either succeed or show fraud detection warning
    assert!(
        response.contains("successful") || response.contains("Success") || 
        response.contains("suspicious") || response.contains("limit"),
        "Should handle large withdrawal. Got: {}", response
    );
}
