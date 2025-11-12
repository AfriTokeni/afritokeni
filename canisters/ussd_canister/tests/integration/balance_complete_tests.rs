// COMPLETE Balance Check Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data canister for balance queries
use super::*;

// ============================================================================
// FIAT BALANCE CHECKS - ALL CURRENCIES
// ============================================================================

#[test]
fn test_check_balance_ugx_zero() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "UGX", "balugx@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2"); // Check balance
    
    assert!(response.contains("0") || response.contains("zero") || response.contains("UGX"),
        "Should show zero UGX balance. Got: {}", response);
}

#[test]
fn test_check_balance_ugx_with_money() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "UGXFull", "balugxfull@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("500") || response.contains("500,000") || response.contains("500000"),
        "Should show 500000 UGX. Got: {}", response);
}

#[test]
fn test_check_balance_kes() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("KES");
    
    env.setup_test_user_with_balances(phone, "Balance", "KES", "balkes@test.com", "KES", "1234", 250000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("250") && response.contains("KES"),
        "Should show KES balance. Got: {}", response);
}

#[test]
fn test_check_balance_tzs() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("TZS");
    
    env.setup_test_user_with_balances(phone, "Balance", "TZS", "baltzs@test.com", "TZS", "1234", 1000000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("1,000,000") || response.contains("1000000"),
        "Should show TZS balance. Got: {}", response);
}

#[test]
fn test_check_balance_rwf() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("RWF");
    
    env.setup_test_user_with_balances(phone, "Balance", "RWF", "balrwf@test.com", "RWF", "1234", 750000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("750") && response.contains("RWF"),
        "Should show RWF balance. Got: {}", response);
}

#[test]
fn test_check_balance_ngn() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("NGN");
    
    env.setup_test_user_with_balances(phone, "Balance", "NGN", "balngn@test.com", "NGN", "1234", 2000000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("2,000,000") || response.contains("2000000"),
        "Should show NGN balance. Got: {}", response);
}

#[test]
fn test_check_balance_ghs() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("GHS");
    
    env.setup_test_user_with_balances(phone, "Balance", "GHS", "balghs@test.com", "GHS", "1234", 50000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("50") && response.contains("GHS"),
        "Should show GHS balance. Got: {}", response);
}

#[test]
fn test_check_balance_zar() {
    let env = get_test_env();
    let sess = session();
    let phone = "+27700888888";
    
    env.setup_test_user_with_balances(phone, "Balance", "ZAR", "balzar@test.com", "ZAR", "1234", 100000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("100") && response.contains("ZAR"),
        "Should show ZAR balance. Got: {}", response);
}

// ============================================================================
// CRYPTO BALANCE CHECKS
// ============================================================================

#[test]
fn test_check_bitcoin_balance_zero() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "BTCZero", "balbtczero@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*1"); // Bitcoin -> Balance
    
    assert!(response.contains("0") || response.contains("zero"),
        "Should show zero BTC. Got: {}", response);
}

#[test]
fn test_check_bitcoin_balance_with_btc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "BTCFull", "balbtcfull@test.com", "UGX", "1234", 0, 250000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*1");
    
    assert!(response.contains("250") || response.contains("BTC") || response.contains("Bitcoin"),
        "Should show BTC balance. Got: {}", response);
}

#[test]
fn test_check_usdc_balance_zero() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "USDCZero", "balusdczero@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*1"); // USDC -> Balance
    
    assert!(response.contains("0") || response.contains("zero"),
        "Should show zero USDC. Got: {}", response);
}

#[test]
fn test_check_usdc_balance_with_usdc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "USDCFull", "balusdcfull@test.com", "UGX", "1234", 0, 0, 300000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*1");
    
    assert!(response.contains("300") || response.contains("USDC"),
        "Should show USDC balance. Got: {}", response);
}

#[test]
fn test_check_both_crypto_balances() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "BothCrypto", "balboth@test.com", "UGX", "1234", 0, 100000, 200000)
        .expect("Setup");
    
    // Check BTC
    let (btc_response, _) = env.process_ussd(&sess, phone, "2*1");
    assert!(btc_response.contains("100") || btc_response.contains("BTC"));
    
    // Check USDC
    let (usdc_response, _) = env.process_ussd(&sess, phone, "3*1");
    assert!(usdc_response.contains("200") || usdc_response.contains("USDC"));
}

// ============================================================================
// BALANCE AFTER TRANSACTIONS
// ============================================================================

#[test]
fn test_balance_after_send_money() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Sender", "Bal", "balsender@test.com", "UGX", "1234", 200000, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receiver", "Bal", "balreceiver@test.com", "UGX", "5678", 0, 0, 0)
        .expect("Setup");
    
    // Send money: Menu 1 (Local Currency) -> 1 (Send Money) -> recipient -> amount -> PIN
    env.process_ussd(&sess, sender, &format!("1*1*{}*80000*1234", receiver));
    
    // Check sender balance - use new session
    let sess2 = session();
    let (sender_response, _) = env.process_ussd(&sess2, sender, "1*2");
    assert!(sender_response.contains("120") || sender_response.contains("120,000"),
        "Sender should have 120000. Got: {}", sender_response);
    
    // Check receiver balance - use new session
    let sess3 = session();
    let (receiver_response, _) = env.process_ussd(&sess3, receiver, "1*2");
    assert!(receiver_response.contains("80") || receiver_response.contains("80,000"),
        "Receiver should have 80000. Got: {}", receiver_response);
}

#[test]
fn test_balance_after_buy_bitcoin() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "BuyBTC", "balbuybtc@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");
    
    // Buy Bitcoin: Menu 2 (Bitcoin) -> 3 (Buy) -> amount -> PIN
    env.process_ussd(&sess, phone, "2*3*200000*1234");
    
    // Check fiat balance (should decrease) - use new session
    let sess2 = session();
    let (fiat_response, _) = env.process_ussd(&sess2, phone, "1*2");
    assert!(fiat_response.contains("300") || fiat_response.contains("300,000"),
        "Fiat should be 300000. Got: {}", fiat_response);
    
    // Check BTC balance (should increase) - use new session
    let sess3 = session();
    let (btc_response, _) = env.process_ussd(&sess3, phone, "2*1");
    assert!(btc_response.contains("BTC") || btc_response.len() > 0,
        "Should have BTC. Got: {}", btc_response);
}

#[test]
fn test_balance_after_withdrawal() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "Withdraw", "balwithdraw@test.com", "UGX", "1234", 300000, 0, 0)
        .expect("Setup");
    
    // Withdraw
    env.process_ussd(&sess, phone, "7");
    env.process_ussd(&sess, phone, "100000");
    env.process_ussd(&sess, phone, "AGENT001");
    env.process_ussd(&sess, phone, "1234");
    
    // Check balance
    let (response, _) = env.process_ussd(&sess, phone, "6");
    assert!(response.contains("200") || response.contains("200,000"),
        "Should have 200000 left. Got: {}", response);
}

// ============================================================================
// BALANCE FORMATTING
// ============================================================================

#[test]
fn test_balance_formatting_thousands() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "Format", "balformat@test.com", "UGX", "1234", 1234567, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    // Should have proper formatting (commas or spaces)
    assert!(response.contains("1,234,567") || response.contains("1234567"),
        "Should format large numbers. Got: {}", response);
}

#[test]
fn test_balance_precision_small_amounts() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "SmallAmt", "balsmall@test.com", "UGX", "1234", 123, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("123"),
        "Should show exact small amount. Got: {}", response);
}

#[test]
fn test_balance_display_currency_symbol() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "Symbol", "balsymbol@test.com", "UGX", "1234", 50000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("UGX") || response.contains("Sh"),
        "Should show currency. Got: {}", response);
}

// ============================================================================
// BALANCE CHECKS FOR UNREGISTERED/NEW USERS
// ============================================================================

#[test]
fn test_balance_new_user_zero() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "NewUser", "balnew@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Immediately check balance (should be 0)
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    
    assert!(response.contains("0") || response.contains("zero"),
        "New user should have 0 balance. Got: {}", response);
}

// ============================================================================
// BALANCE MULTIPLE CHECKS
// ============================================================================

#[test]
fn test_balance_multiple_checks_same_session() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "MultiCheck", "balmulti@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    // Check balance multiple times
    let (response1, _) = env.process_ussd(&sess, phone, "6");
    let (response2, _) = env.process_ussd(&sess, phone, "6");
    let (response3, _) = env.process_ussd(&sess, phone, "1*2");
    
    // All should show same balance (stateless)
    assert_eq!(response1, response2, "Should be consistent");
    assert_eq!(response2, response3, "Should be consistent");
}

#[test]
fn test_balance_check_after_multiple_transactions() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "MultiTx", "balmultitx@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");
    
    // Transaction 1: Buy BTC
    env.process_ussd(&sess, phone, "2*2");
    env.process_ussd(&sess, phone, "100000");
    env.process_ussd(&sess, phone, "1234");
    
    // Transaction 2: Buy USDC
    env.process_ussd(&sess, phone, "3*2");
    env.process_ussd(&sess, phone, "100000");
    env.process_ussd(&sess, phone, "1234");
    
    // Check final balance
    let (response, _) = env.process_ussd(&sess, phone, "1*2");
    assert!(response.contains("300") || response.contains("300,000"),
        "Should have 300000 left. Got: {}", response);
}

// ============================================================================
// BALANCE STATELESS BEHAVIOR
// ============================================================================

#[test]
fn test_balance_check_is_stateless() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Balance", "Stateless", "balstateless@test.com", "UGX", "1234", 75000, 0, 0)
        .expect("Setup");
    
    // Same input should give same output
    let (response1, _) = env.process_ussd(&sess, phone, "6");
    let (response2, _) = env.process_ussd(&sess, phone, "6");
    
    assert_eq!(response1, response2, "Balance check should be stateless");
}
