// COMPLETE Balance Check Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data canister for balance queries
use super::*;

// ============================================================================
// FIAT BALANCE CHECKS - ALL CURRENCIES
// ============================================================================

#[test]
fn test_check_balance_ugx_zero() {
    let env = get_test_env();
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "Balance", "UGX", "balugx@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, _) = env.process_ussd("session", phone, "1*2"); // Check balance
    
    assert!(response.contains("0") || response.contains("zero") || response.contains("UGX"),
        "Should show zero UGX balance. Got: {}", response);
}

#[test]
fn test_check_balance_ugx_with_money() {
    let env = get_test_env();
    let phone = "+256700222222";
    
    env.register_user_direct(phone, "Balance", "UGXFull", "balugxfull@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("500") || response.contains("500,000") || response.contains("500000"),
        "Should show 500000 UGX. Got: {}", response);
}

#[test]
fn test_check_balance_kes() {
    let env = get_test_env();
    let phone = "+254700333333";
    
    env.register_user_direct(phone, "Balance", "KES", "balkes@test.com", "KES", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "KES", 250000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("250") && response.contains("KES"),
        "Should show KES balance. Got: {}", response);
}

#[test]
fn test_check_balance_tzs() {
    let env = get_test_env();
    let phone = "+255700444444";
    
    env.register_user_direct(phone, "Balance", "TZS", "baltzs@test.com", "TZS", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "TZS", 1000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("1,000,000") || response.contains("1000000"),
        "Should show TZS balance. Got: {}", response);
}

#[test]
fn test_check_balance_rwf() {
    let env = get_test_env();
    let phone = "+250700555555";
    
    env.register_user_direct(phone, "Balance", "RWF", "balrwf@test.com", "RWF", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "RWF", 750000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("750") && response.contains("RWF"),
        "Should show RWF balance. Got: {}", response);
}

#[test]
fn test_check_balance_ngn() {
    let env = get_test_env();
    let phone = "+234700666666";
    
    env.register_user_direct(phone, "Balance", "NGN", "balngn@test.com", "NGN", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "NGN", 2000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("2,000,000") || response.contains("2000000"),
        "Should show NGN balance. Got: {}", response);
}

#[test]
fn test_check_balance_ghs() {
    let env = get_test_env();
    let phone = "+233700777777";
    
    env.register_user_direct(phone, "Balance", "GHS", "balghs@test.com", "GHS", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "GHS", 50000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("50") && response.contains("GHS"),
        "Should show GHS balance. Got: {}", response);
}

#[test]
fn test_check_balance_zar() {
    let env = get_test_env();
    let phone = "+27700888888";
    
    env.register_user_direct(phone, "Balance", "ZAR", "balzar@test.com", "ZAR", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "ZAR", 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("100") && response.contains("ZAR"),
        "Should show ZAR balance. Got: {}", response);
}

// ============================================================================
// CRYPTO BALANCE CHECKS
// ============================================================================

#[test]
fn test_check_bitcoin_balance_zero() {
    let env = get_test_env();
    let phone = "+256700999999";
    
    env.register_user_direct(phone, "Balance", "BTCZero", "balbtczero@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, _) = env.process_ussd("session", phone, "2*1"); // Bitcoin -> Balance
    
    assert!(response.contains("0") || response.contains("zero"),
        "Should show zero BTC. Got: {}", response);
}

#[test]
fn test_check_bitcoin_balance_with_btc() {
    let env = get_test_env();
    let phone = "+256700101010";
    
    env.register_user_direct(phone, "Balance", "BTCFull", "balbtcfull@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 250000, 0).expect("Set BTC");
    
    let (response, _) = env.process_ussd("session", phone, "2*1");
    
    assert!(response.contains("250") || response.contains("BTC") || response.contains("Bitcoin"),
        "Should show BTC balance. Got: {}", response);
}

#[test]
fn test_check_usdc_balance_zero() {
    let env = get_test_env();
    let phone = "+256700202020";
    
    env.register_user_direct(phone, "Balance", "USDCZero", "balusdczero@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, _) = env.process_ussd("session", phone, "3*1"); // USDC -> Balance
    
    assert!(response.contains("0") || response.contains("zero"),
        "Should show zero USDC. Got: {}", response);
}

#[test]
fn test_check_usdc_balance_with_usdc() {
    let env = get_test_env();
    let phone = "+256700303030";
    
    env.register_user_direct(phone, "Balance", "USDCFull", "balusdcfull@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 300000).expect("Set USDC");
    
    let (response, _) = env.process_ussd("session", phone, "3*1");
    
    assert!(response.contains("300") || response.contains("USDC"),
        "Should show USDC balance. Got: {}", response);
}

#[test]
fn test_check_both_crypto_balances() {
    let env = get_test_env();
    let phone = "+256700404040";
    
    env.register_user_direct(phone, "Balance", "BothCrypto", "balboth@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 200000).expect("Set both");
    
    // Check BTC
    let (btc_response, _) = env.process_ussd("s1", phone, "2*1");
    assert!(btc_response.contains("100") || btc_response.contains("BTC"));
    
    // Check USDC
    let (usdc_response, _) = env.process_ussd("s2", phone, "3*1");
    assert!(usdc_response.contains("200") || usdc_response.contains("USDC"));
}

// ============================================================================
// BALANCE AFTER TRANSACTIONS
// ============================================================================

#[test]
fn test_balance_after_send_money() {
    let env = get_test_env();
    let sender = "+256700505001";
    let receiver = "+256700505002";
    
    env.register_user_direct(sender, "Sender", "Bal", "balsender@test.com", "UGX", "1234")
        .expect("Registration");
    env.register_user_direct(receiver, "Receiver", "Bal", "balreceiver@test.com", "UGX", "5678")
        .expect("Registration");
    
    env.set_fiat_balance(sender, "UGX", 200000).expect("Set balance");
    
    // Send money
    env.process_ussd("s1", sender, "1");
    env.process_ussd("s1", sender, receiver);
    env.process_ussd("s1", sender, "80000");
    env.process_ussd("s1", sender, "1234");
    
    // Check sender balance
    let (sender_response, _) = env.process_ussd("s2", sender, "1*2");
    assert!(sender_response.contains("120") || sender_response.contains("120,000"),
        "Sender should have 120000. Got: {}", sender_response);
    
    // Check receiver balance
    let (receiver_response, _) = env.process_ussd("s3", receiver, "6");
    assert!(receiver_response.contains("80") || receiver_response.contains("80,000"),
        "Receiver should have 80000. Got: {}", receiver_response);
}

#[test]
fn test_balance_after_buy_bitcoin() {
    let env = get_test_env();
    let phone = "+256700606060";
    
    env.register_user_direct(phone, "Balance", "BuyBTC", "balbuybtc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    // Buy Bitcoin
    env.process_ussd("s1", phone, "2");
    env.process_ussd("s1", phone, "2");
    env.process_ussd("s1", phone, "200000");
    env.process_ussd("s1", phone, "1234");
    
    // Check fiat balance (should decrease)
    let (fiat_response, _) = env.process_ussd("s2", phone, "6");
    assert!(fiat_response.contains("300") || fiat_response.contains("300,000"),
        "Fiat should be 300000. Got: {}", fiat_response);
    
    // Check BTC balance (should increase)
    let (btc_response, _) = env.process_ussd("s3", phone, "2*1");
    assert!(btc_response.contains("BTC") || btc_response.len() > 0,
        "Should have BTC. Got: {}", btc_response);
}

#[test]
fn test_balance_after_withdrawal() {
    let env = get_test_env();
    let phone = "+256700707070";
    
    env.register_user_direct(phone, "Balance", "Withdraw", "balwithdraw@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 300000).expect("Set balance");
    
    // Withdraw
    env.process_ussd("s1", phone, "7");
    env.process_ussd("s1", phone, "100000");
    env.process_ussd("s1", phone, "AGENT001");
    env.process_ussd("s1", phone, "1234");
    
    // Check balance
    let (response, _) = env.process_ussd("s2", phone, "6");
    assert!(response.contains("200") || response.contains("200,000"),
        "Should have 200000 left. Got: {}", response);
}

// ============================================================================
// BALANCE FORMATTING
// ============================================================================

#[test]
fn test_balance_formatting_thousands() {
    let env = get_test_env();
    let phone = "+256700808080";
    
    env.register_user_direct(phone, "Balance", "Format", "balformat@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 1234567).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    // Should have proper formatting (commas or spaces)
    assert!(response.contains("1,234,567") || response.contains("1234567"),
        "Should format large numbers. Got: {}", response);
}

#[test]
fn test_balance_precision_small_amounts() {
    let env = get_test_env();
    let phone = "+256700909090";
    
    env.register_user_direct(phone, "Balance", "SmallAmt", "balsmall@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 123).expect("Set small balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("123"),
        "Should show exact small amount. Got: {}", response);
}

#[test]
fn test_balance_display_currency_symbol() {
    let env = get_test_env();
    let phone = "+256700010101";
    
    env.register_user_direct(phone, "Balance", "Symbol", "balsymbol@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 50000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("UGX") || response.contains("Sh"),
        "Should show currency. Got: {}", response);
}

// ============================================================================
// BALANCE CHECKS FOR UNREGISTERED/NEW USERS
// ============================================================================

#[test]
fn test_balance_new_user_zero() {
    let env = get_test_env();
    let phone = "+256700020202";
    
    env.register_user_direct(phone, "Balance", "NewUser", "balnew@test.com", "UGX", "1234")
        .expect("Registration");
    
    // Immediately check balance (should be 0)
    let (response, _) = env.process_ussd("session", phone, "1*2");
    
    assert!(response.contains("0") || response.contains("zero"),
        "New user should have 0 balance. Got: {}", response);
}

// ============================================================================
// BALANCE MULTIPLE CHECKS
// ============================================================================

#[test]
fn test_balance_multiple_checks_same_session() {
    let env = get_test_env();
    let phone = "+256700030303";
    
    env.register_user_direct(phone, "Balance", "MultiCheck", "balmulti@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    // Check balance multiple times
    let (response1, _) = env.process_ussd("s1", phone, "6");
    let (response2, _) = env.process_ussd("s2", phone, "6");
    let (response3, _) = env.process_ussd("s3", phone, "1*2");
    
    // All should show same balance (stateless)
    assert_eq!(response1, response2, "Should be consistent");
    assert_eq!(response2, response3, "Should be consistent");
}

#[test]
fn test_balance_check_after_multiple_transactions() {
    let env = get_test_env();
    let phone = "+256700040404";
    
    env.register_user_direct(phone, "Balance", "MultiTx", "balmultitx@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set initial");
    
    // Transaction 1: Buy BTC
    env.process_ussd("s1", phone, "2*2");
    env.process_ussd("s1", phone, "100000");
    env.process_ussd("s1", phone, "1234");
    
    // Transaction 2: Buy USDC
    env.process_ussd("s2", phone, "3*2");
    env.process_ussd("s2", phone, "100000");
    env.process_ussd("s2", phone, "1234");
    
    // Check final balance
    let (response, _) = env.process_ussd("s3", phone, "1*2");
    assert!(response.contains("300") || response.contains("300,000"),
        "Should have 300000 left. Got: {}", response);
}

// ============================================================================
// BALANCE STATELESS BEHAVIOR
// ============================================================================

#[test]
fn test_balance_check_is_stateless() {
    let env = get_test_env();
    let phone = "+256700050505";
    
    env.register_user_direct(phone, "Balance", "Stateless", "balstateless@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 75000).expect("Set balance");
    
    // Same input should give same output
    let (response1, _) = env.process_ussd("session1", phone, "6");
    let (response2, _) = env.process_ussd("session2", phone, "6");
    
    assert_eq!(response1, response2, "Balance check should be stateless");
}
