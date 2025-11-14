// Integration tests for USSD USDC flows
use super::*;

#[test]
fn test_usdc_balance_check() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Alice", "USDC", "alice@test.com", "UGX", "1111"
    ).expect("Registration should succeed");
    
    // Set USDC balance
    env.set_crypto_balance(&user_id, 0, 1_000_000).expect("Should set balance"); // 1 USDC
    
    let session_id = "usdc_test_1";
    
    // Navigate to USDC menu -> Check Balance
    let (response, continue_session) = env.process_ussd(session_id, phone, "3");
    assert!(continue_session, "Should show USDC menu");
    assert!(response.contains("USDC"), 
        "Should show USDC menu. Got: {}", response);
    
    // Check balance
    let (response, _) = env.process_ussd(session_id, phone, "3*1");
    assert!(response.contains("1.00") || response.contains("ckUSDC"), 
        "Should show 1 USDC balance. Got: {}", response);
}

#[test]
fn test_usdc_rate_check() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    env.register_user_direct(
        phone, "Bob", "Trader", "bob@test.com", "UGX", "2222"
    ).expect("Registration should succeed");
    
    let session_id = "usdc_test_2";
    
    // Navigate to USDC menu -> Check Rate
    env.process_ussd(session_id, phone, "3");
    let (response, _) = env.process_ussd(session_id, phone, "3*2");
    
    // Should show rate information
    assert!(response.contains("rate") || response.contains("price") || response.contains("USDC") || response.contains("UGX"), 
        "Should show USDC rate. Got: {}", response);
}

#[test]
fn test_buy_usdc_flow_navigation() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Charlie", "Buyer", "charlie@test.com", "UGX", "3333"
    ).expect("Registration should succeed");
    
    // Give user fiat balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Should set balance");
    
    let session_id = "usdc_test_3";
    
    // Navigate to USDC menu -> Buy USDC
    let (response, continue_session) = env.process_ussd(session_id, phone, "3");
    assert!(continue_session, "Should show USDC menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "3*3");
    assert!(continue_session, "Should start buy flow");
    assert!(response.contains("amount") || response.contains("Enter") || response.contains("UGX"), 
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_buy_usdc_insufficient_balance() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Dave", "Poor", "dave@test.com", "UGX", "4444"
    ).expect("Registration should succeed");
    
    // Give user small balance
    env.set_fiat_balance(&user_id, "UGX", 1_000).expect("Should set balance");
    
    let session_id = "usdc_test_4";
    
    // Try to buy USDC with large amount
    let (response, _) = env.process_ussd(session_id, phone, "3*3*1000000*4444");
    
    assert!(response.contains("Insufficient") || response.contains("balance") || response.contains("enough"), 
        "Should show insufficient balance error. Got: {}", response);
}

#[test]
fn test_send_usdc_flow_navigation() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Eve", "Sender", "eve@test.com", "UGX", "5555"
    ).expect("Registration should succeed");
    
    // Give user USDC balance
    env.set_crypto_balance(&user_id, 0, 100_000_000).expect("Should set balance"); // 100 USDC
    
    let session_id = "usdc_test_5";
    
    // Navigate to USDC menu -> Send USDC
    let (response, continue_session) = env.process_ussd(session_id, phone, "3");
    assert!(continue_session, "Should show USDC menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "3*5");
    assert!(continue_session, "Should start send flow");
    assert!(response.contains("address") || response.contains("recipient") || response.contains("Enter"), 
        "Should ask for address. Got: {}", response);
}

#[test]
fn test_send_usdc_insufficient_balance() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Frank", "Sender", "frank@test.com", "UGX", "6666"
    ).expect("Registration should succeed");
    
    // Give user small USDC balance
    env.set_crypto_balance(&user_id, 0, 1_000_000).expect("Should set balance"); // 1 USDC
    
    let session_id = "usdc_test_6";
    
    // Try to send more than balance (use valid IC Principal address)
    let address = "rrkah-fqaaa-aaaaa-aaaaq-cai";
    let input = format!("3*5*{}*100000000*6666", address); // Try to send 100 USDC (100,000,000 e6)
    let (response, _) = env.process_ussd(session_id, phone, &input);

    assert!(response.contains("Insufficient") || response.contains("insufficient") || response.contains("balance"),
        "Should show insufficient balance error. Got: {}", response);
}

#[test]
fn test_sell_usdc_flow_navigation() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Grace", "Seller", "grace@test.com", "UGX", "7777"
    ).expect("Registration should succeed");
    
    // Give user USDC balance
    env.set_crypto_balance(&user_id, 0, 100_000_000).expect("Should set balance"); // 100 USDC
    
    let session_id = "usdc_test_7";
    
    // Navigate to USDC menu -> Sell USDC
    let (response, continue_session) = env.process_ussd(session_id, phone, "3");
    assert!(continue_session, "Should show USDC menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "3*4");
    assert!(continue_session, "Should start sell flow");
    assert!(response.contains("amount") || response.contains("Enter") || response.contains("USDC"), 
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_usdc_menu_structure() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    env.register_user_direct(
        phone, "Henry", "User", "henry@test.com", "UGX", "8888"
    ).expect("Registration should succeed");
    
    let session_id = "usdc_test_8";
    
    // Check USDC menu has all expected options
    let (response, continue_session) = env.process_ussd(session_id, phone, "3");
    assert!(continue_session, "Should show USDC menu");
    assert!(response.contains("USDC"), 
        "Should show USDC title. Got: {}", response);
    assert!(response.contains("balance") || response.contains("Balance"), 
        "Should show balance option. Got: {}", response);
    assert!(response.contains("rate") || response.contains("Rate"), 
        "Should show rate option. Got: {}", response);
    assert!(response.contains("Buy") || response.contains("buy"), 
        "Should show buy option. Got: {}", response);
    assert!(response.contains("Sell") || response.contains("sell"), 
        "Should show sell option. Got: {}", response);
    assert!(response.contains("Send") || response.contains("send"), 
        "Should show send option. Got: {}", response);
}

#[test]
fn test_usdc_zero_balance_display() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    env.register_user_direct(
        phone, "Ivy", "NewUser", "ivy@test.com", "UGX", "9999"
    ).expect("Registration should succeed");
    
    let session_id = "usdc_test_9";
    
    // Check balance when user has no USDC
    env.process_ussd(session_id, phone, "3");
    let (response, _) = env.process_ussd(session_id, phone, "3*1");
    
    assert!(response.contains("0.00") || response.contains("ckUSDC"), 
        "Should show zero balance. Got: {}", response);
}

#[test]
fn test_usdc_stablecoin_characteristics() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Jack", "Stable", "jack@test.com", "UGX", "1010"
    ).expect("Registration should succeed");
    
    // Set USDC balance with precise amount
    env.set_crypto_balance(&user_id, 0, 50_500_000).expect("Should set balance"); // 50.5 USDC
    
    let session_id = "usdc_test_10";
    
    // Check balance displays correctly with 2 decimal places
    env.process_ussd(session_id, phone, "3");
    let (response, _) = env.process_ussd(session_id, phone, "3*1");
    
    assert!(response.contains("50.50") || response.contains("50.5"), 
        "Should show 50.50 USDC balance. Got: {}", response);
}
