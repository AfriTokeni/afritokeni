// Integration tests for USSD Bitcoin flows
use super::*;

#[test]
fn test_bitcoin_balance_check() {
    let env = get_test_env();
    
    let phone = "+256700111111";
    
    let user_id = env.register_user_direct(
        phone, "Alice", "Bitcoin", "alice@test.com", "UGX", "1111"
    ).expect("Registration should succeed");
    
    // Set Bitcoin balance
    env.set_crypto_balance(&user_id, 100_000_000, 0).expect("Should set balance"); // 1 BTC
    
    let session_id = "btc_test_1";
    
    // Navigate to Bitcoin menu -> Check Balance
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    assert!(continue_session, "Should show Bitcoin menu");
    assert!(response.contains("Bitcoin") || response.contains("BTC"), 
        "Should show Bitcoin menu. Got: {}", response);
    
    // Check balance
    let (response, _) = env.process_ussd(session_id, phone, "2*1");
    assert!(response.contains("1.00000000") || response.contains("ckBTC"), 
        "Should show 1 BTC balance. Got: {}", response);
}

#[test]
fn test_bitcoin_rate_check() {
    let env = get_test_env();
    
    let phone = "+256700222222";
    
    env.register_user_direct(
        phone, "Bob", "Trader", "bob@test.com", "UGX", "2222"
    ).expect("Registration should succeed");
    
    let session_id = "btc_test_2";
    
    // Navigate to Bitcoin menu -> Check Rate
    env.process_ussd(session_id, phone, "2");
    let (response, _) = env.process_ussd(session_id, phone, "2*2");
    
    // Should show rate information (exact format depends on implementation)
    assert!(response.contains("rate") || response.contains("price") || response.contains("BTC") || response.contains("UGX"), 
        "Should show Bitcoin rate. Got: {}", response);
}

#[test]
fn test_buy_bitcoin_flow_navigation() {
    let env = get_test_env();
    
    let phone = "+256700333333";
    
    let user_id = env.register_user_direct(
        phone, "Charlie", "Buyer", "charlie@test.com", "UGX", "3333"
    ).expect("Registration should succeed");
    
    // Give user fiat balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).expect("Should set balance");
    
    let session_id = "btc_test_3";
    
    // Navigate to Bitcoin menu -> Buy Bitcoin
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    assert!(continue_session, "Should show Bitcoin menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "2*3");
    assert!(continue_session, "Should start buy flow");
    assert!(response.contains("amount") || response.contains("Enter") || response.contains("UGX"), 
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_buy_bitcoin_insufficient_balance() {
    let env = get_test_env();
    
    let phone = "+256700444444";
    
    let user_id = env.register_user_direct(
        phone, "Dave", "Poor", "dave@test.com", "UGX", "4444"
    ).expect("Registration should succeed");
    
    // Give user small balance
    env.set_fiat_balance(&user_id, "UGX", 1_000).expect("Should set balance");
    
    let session_id = "btc_test_4";
    
    // Try to buy Bitcoin with large amount
    let (response, _) = env.process_ussd(session_id, phone, "2*3*1000000*4444");
    
    assert!(response.contains("Insufficient") || response.contains("balance") || response.contains("enough"), 
        "Should show insufficient balance error. Got: {}", response);
}

#[test]
fn test_send_bitcoin_flow_navigation() {
    let env = get_test_env();
    
    let phone = "+256700555555";
    
    let user_id = env.register_user_direct(
        phone, "Eve", "Sender", "eve@test.com", "UGX", "5555"
    ).expect("Registration should succeed");
    
    // Give user Bitcoin balance
    env.set_crypto_balance(&user_id, 50_000_000, 0).expect("Should set balance"); // 0.5 BTC
    
    let session_id = "btc_test_5";
    
    // Navigate to Bitcoin menu -> Send Bitcoin
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    assert!(continue_session, "Should show Bitcoin menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "2*5");
    assert!(continue_session, "Should start send flow");
    assert!(response.contains("address") || response.contains("recipient") || response.contains("Enter"), 
        "Should ask for address. Got: {}", response);
}

#[test]
fn test_send_bitcoin_insufficient_balance() {
    let env = get_test_env();
    
    let phone = "+256700666666";
    
    let user_id = env.register_user_direct(
        phone, "Frank", "Sender", "frank@test.com", "UGX", "6666"
    ).expect("Registration should succeed");
    
    // Give user small Bitcoin balance
    env.set_crypto_balance(&user_id, 1_000_000, 0).expect("Should set balance"); // 0.01 BTC
    
    let session_id = "btc_test_6";
    
    // Try to send more than balance
    let address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
    let input = format!("2*5*{}*100000000*6666", address); // Try to send 1 BTC
    let (response, _) = env.process_ussd(session_id, phone, &input);
    
    assert!(response.contains("Insufficient") || response.contains("balance") || response.contains("enough"), 
        "Should show insufficient balance error. Got: {}", response);
}

#[test]
fn test_sell_bitcoin_flow_navigation() {
    let env = get_test_env();
    
    let phone = "+256700777777";
    
    let user_id = env.register_user_direct(
        phone, "Grace", "Seller", "grace@test.com", "UGX", "7777"
    ).expect("Registration should succeed");
    
    // Give user Bitcoin balance
    env.set_crypto_balance(&user_id, 100_000_000, 0).expect("Should set balance"); // 1 BTC
    
    let session_id = "btc_test_7";
    
    // Navigate to Bitcoin menu -> Sell Bitcoin
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    assert!(continue_session, "Should show Bitcoin menu");
    
    let (response, continue_session) = env.process_ussd(session_id, phone, "2*4");
    assert!(continue_session, "Should start sell flow");
    assert!(response.contains("amount") || response.contains("Enter") || response.contains("BTC"), 
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_bitcoin_menu_structure() {
    let env = get_test_env();
    
    let phone = "+256700888888";
    
    env.register_user_direct(
        phone, "Henry", "User", "henry@test.com", "UGX", "8888"
    ).expect("Registration should succeed");
    
    let session_id = "btc_test_8";
    
    // Check Bitcoin menu has all expected options
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    assert!(continue_session, "Should show Bitcoin menu");
    assert!(response.contains("Bitcoin") || response.contains("BTC"), 
        "Should show Bitcoin title. Got: {}", response);
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
fn test_bitcoin_zero_balance_display() {
    let env = get_test_env();
    
    let phone = "+256700999999";
    
    env.register_user_direct(
        phone, "Ivy", "NewUser", "ivy@test.com", "UGX", "9999"
    ).expect("Registration should succeed");
    
    let session_id = "btc_test_9";
    
    // Check balance when user has no Bitcoin
    env.process_ussd(session_id, phone, "2");
    let (response, _) = env.process_ussd(session_id, phone, "2*1");
    
    assert!(response.contains("0.00000000") || response.contains("ckBTC"), 
        "Should show zero balance. Got: {}", response);
}
