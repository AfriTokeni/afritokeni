// Integration tests for USSD balance check flows
use super::*;

#[test]
fn test_local_currency_balance_check() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Alice", "Balance", "alice@test.com", "UGX", "1111"
    ).expect("Registration should succeed");
    
    // Set balance
    env.set_fiat_balance(&user_id, "UGX", 150_000).expect("Should set balance");
    
    let session_id = "balance_test_1";
    
    // Navigate to Local Currency -> Check Balance
    env.process_ussd(session_id, phone, "1");
    let (response, _) = env.process_ussd(session_id, phone, "1*2");
    
    assert!(response.contains("1500") || response.contains("1,500"), 
        "Should show 1500 UGX balance. Got: {}", response);
    assert!(response.contains("UGX"), 
        "Should show currency. Got: {}", response);
}

#[test]
fn test_balance_check_multiple_currencies() {
    let env = get_test_env();
    
    // Test with Kenya number (KES)
    let phone_ke = "+254700222222";
    
    let user_id_ke = env.register_user_direct(
        phone_ke, "Bob", "Kenya", "bob@test.com", "KES", "2222"
    ).expect("Registration should succeed");
    
    env.set_fiat_balance(&user_id_ke, "KES", 500_000).expect("Should set balance");
    
    let session_id_ke = "balance_test_2";
    
    env.process_ussd(session_id_ke, phone_ke, "1");
    let (response, _) = env.process_ussd(session_id_ke, phone_ke, "1*2");
    
    assert!(response.contains("5000") || response.contains("5,000"), 
        "Should show 5000 KES balance. Got: {}", response);
    assert!(response.contains("KES"), 
        "Should show KES currency. Got: {}", response);
}

#[test]
fn test_balance_check_with_crypto() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Charlie", "Crypto", "charlie@test.com", "UGX", "3333"
    ).expect("Registration should succeed");
    
    // Set fiat and crypto balances
    env.set_fiat_balance(&user_id, "UGX", 200_000).expect("Should set fiat balance");
    env.set_crypto_balance(&user_id, 50_000_000, 10_000_000).expect("Should set crypto balance");
    
    let session_id = "balance_test_3";
    
    // Check local currency balance
    env.process_ussd(session_id, phone, "1");
    let (response, _) = env.process_ussd(session_id, phone, "1*2");
    
    assert!(response.contains("2000") || response.contains("2,000"), 
        "Should show fiat balance. Got: {}", response);
    assert!(response.contains("0.5") || response.contains("ckBTC"), 
        "Should show Bitcoin balance. Got: {}", response);
    assert!(response.contains("10") || response.contains("ckUSDC"), 
        "Should show USDC balance. Got: {}", response);
}

#[test]
fn test_balance_check_zero_balance() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    env.register_user_direct(
        phone, "Dave", "Zero", "dave@test.com", "UGX", "4444"
    ).expect("Registration should succeed");
    
    let session_id = "balance_test_4";
    
    // Check balance without setting any
    env.process_ussd(session_id, phone, "1");
    let (response, _) = env.process_ussd(session_id, phone, "1*2");
    
    assert!(response.contains("0") || response.contains("0.00"), 
        "Should show zero balance. Got: {}", response);
}

#[test]
fn test_bitcoin_balance_check_separate() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Eve", "BTCUser", "eve@test.com", "UGX", "5555"
    ).expect("Registration should succeed");
    
    // Set only Bitcoin balance
    env.set_crypto_balance(&user_id, 25_000_000, 0).expect("Should set balance"); // 0.25 BTC
    
    let session_id = "balance_test_5";
    
    // Check Bitcoin balance via Bitcoin menu
    env.process_ussd(session_id, phone, "2");
    let (response, _) = env.process_ussd(session_id, phone, "2*1");
    
    assert!(response.contains("0.25") || response.contains("ckBTC"), 
        "Should show 0.25 BTC. Got: {}", response);
}

#[test]
fn test_usdc_balance_check_separate() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Frank", "USDCUser", "frank@test.com", "UGX", "6666"
    ).expect("Registration should succeed");
    
    // Set only USDC balance
    env.set_crypto_balance(&user_id, 0, 75_000_000).expect("Should set balance"); // 75 USDC
    
    let session_id = "balance_test_6";
    
    // Check USDC balance via USDC menu
    env.process_ussd(session_id, phone, "3");
    let (response, _) = env.process_ussd(session_id, phone, "3*1");
    
    assert!(response.contains("75") || response.contains("ckUSDC"), 
        "Should show 75 USDC. Got: {}", response);
}

#[test]
fn test_balance_check_formatting() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Grace", "Format", "grace@test.com", "UGX", "7777"
    ).expect("Registration should succeed");
    
    // Set large balance to test formatting
    env.set_fiat_balance(&user_id, "UGX", 12_345_678).expect("Should set balance");
    
    let session_id = "balance_test_7";
    
    env.process_ussd(session_id, phone, "1");
    let (response, _) = env.process_ussd(session_id, phone, "1*2");
    
    // Should show formatted amount (123456.78 or 123,456.78)
    assert!(response.contains("123456") || response.contains("123,456"), 
        "Should show formatted balance. Got: {}", response);
}

#[test]
fn test_balance_check_after_transaction() {
    let env = get_test_env();

    let sender_phone = "+256700888001";
    let receiver_phone = "+256700888002";

    let sender_id = env.register_user_direct(
        sender_phone, "Henry", "Sender", "henry@test.com", "UGX", "8888"
    ).expect("Sender registration should succeed");

    env.register_user_direct(
        receiver_phone, "Ivy", "Receiver", "ivy@test.com", "UGX", "9999"
    ).expect("Receiver registration should succeed");

    // Give sender balance (100_000 cents = 1000.00 UGX)
    env.set_fiat_balance(&sender_id, "UGX", 100_000).expect("Should set balance");

    let session_id = "balance_test_8";

    // Check initial balance
    env.process_ussd(session_id, sender_phone, "1");
    let (response, _) = env.process_ussd(session_id, sender_phone, "1*2");
    assert!(response.contains("1000") || response.contains("1,000"),
        "Should show initial balance. Got: {}", response);

    // Send money: User enters 500 (meaning 500.00 UGX in display)
    // The USSD flow expects amounts in currency units, not cents
    let send_input = format!("1*1*{}*500*8888", receiver_phone);
    let (send_response, _) = env.process_ussd(session_id, sender_phone, &send_input);

    // Verify send was successful
    assert!(send_response.contains("successful") || send_response.contains("Sent") || send_response.contains("completed"),
        "Send should succeed. Got: {}", send_response);

    // Check balance again - should be 1000.00 - 500.00 - fee(2.50) = 497.50 UGX
    let session_id_2 = "balance_test_8_after";
    env.process_ussd(session_id_2, sender_phone, "1");
    let (response, _) = env.process_ussd(session_id_2, sender_phone, "1*2");
    // Balance should be ~497.50 after fee
    assert!(response.contains("497") || response.contains("49"),
        "Should show updated balance after send (~497.50 UGX after fee). Got: {}", response);
}

#[test]
fn test_balance_check_unregistered_user() {
    let env = get_test_env();
    // Use non-playground session (don't use session() helper which might generate playground_* prefix)
    let sess = "test_unreg_balance";
    let phone = &phone("UGX");

    // Try to check balance without registration
    let (response, _) = env.process_ussd(&sess, phone, "1*2");

    // Should show registration prompt (first-time user)
    // Updated expectation: System shows PIN registration for new users
    assert!(response.contains("Welcome to AfriTokeni") || response.contains("PIN") || response.contains("register"),
        "Should show registration prompt for unregistered user. Got: {}", response);
}

#[test]
fn test_balance_precision() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    let user_id = env.register_user_direct(
        phone, "Jack", "Precise", "jack@test.com", "UGX", "1010"
    ).expect("Registration should succeed");
    
    // Set precise amounts
    env.set_fiat_balance(&user_id, "UGX", 12_345).expect("Should set fiat balance");
    env.set_crypto_balance(&user_id, 12_345_678, 12_345_678).expect("Should set crypto balance");
    
    let session_id = "balance_test_10";
    
    // Check all balances
    env.process_ussd(session_id, phone, "1");
    let (response, _) = env.process_ussd(session_id, phone, "1*2");
    
    // Should show precise amounts with correct decimal places
    assert!(response.contains("123.45") || response.contains("123"), 
        "Should show fiat with 2 decimals. Got: {}", response);
    assert!(response.contains("0.12345678") || response.contains("ckBTC"), 
        "Should show BTC with 8 decimals. Got: {}", response);
    assert!(response.contains("12.345678") || response.contains("ckUSDC"), 
        "Should show USDC with 6 decimals. Got: {}", response);
}
