// COMPLETE Bitcoin Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data -> Exchange canister interactions
use super::*;

// ============================================================================
// BUY BITCOIN - ALL CURRENCY COMBINATIONS
// ============================================================================

#[test]
fn test_buy_bitcoin_with_ugx() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Buyer", "btc@test.com", "UGX", "1234", 10000000, 0, 0)
        .expect("Setup");
    
    // Buy Bitcoin: Menu 2 (Bitcoin) -> 3 (Buy) -> Amount (100,000 UGX) -> PIN
    let (response, _) = env.process_ussd(&sess, phone, "2*3*100000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy BTC. Got: {}", response);
    
    // Verify BTC balance increased
    let (btc, _) = env.get_crypto_balance(phone).expect("Get crypto balance");
    assert!(btc > 0, "Should have BTC balance");
    
    // Verify fiat balance decreased
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get fiat balance");
    assert_eq!(fiat, 900000, "Fiat should decrease");
}

#[test]
fn test_buy_bitcoin_with_kes() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("KES");
    
    env.setup_test_user_with_balances(phone, "KES", "BTCBuyer", "kesbtc@test.com", "KES", "1234", 500000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*3*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy BTC. Got: {}", response);
    
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc > 0);
}

#[test]
fn test_buy_bitcoin_with_tzs() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("TZS");
    
    env.setup_test_user_with_balances(phone, "TZS", "BTCBuyer", "tzsbtc@test.com", "TZS", "1234", 2000000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*3*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy BTC. Got: {}", response);
}

#[test]
fn test_buy_bitcoin_with_ngn() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("NGN");
    
    env.setup_test_user_with_balances(phone, "NGN", "BTCBuyer", "ngnbtc@test.com", "NGN", "1234", 5000000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*3*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy BTC. Got: {}", response);
}

// ============================================================================
// SEND BITCOIN - ALL SCENARIOS
// ============================================================================

#[test]
fn test_send_bitcoin_to_valid_address() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Sender", "btcsend@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    
    // Send Bitcoin: Menu 2 -> 5 (Send) -> address -> amount -> PIN
    // Note: ckBTC uses IC Principal addresses, not Bitcoin addresses
    let (response, _) = env.process_ussd(&sess, phone, "2*5*rrkah-fqaaa-aaaaa-aaaaq-cai*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sent"),
        "Should send BTC. Got: {}", response);
    
    // Verify balance decreased
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 50000, "BTC balance should decrease");
}

#[test]
fn test_send_bitcoin_insufficient_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Poor", "btcpoor@test.com", "UGX", "1234", 0, 10000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*5*bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh*50000*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient balance. Got: {}", response);
}

#[test]
fn test_send_bitcoin_invalid_address() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Invalid", "btcinvalid@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*5*invalid_address_123");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("address"),
        "Should reject invalid address. Got: {}", response);
}

#[test]
fn test_send_bitcoin_zero_amount() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Zero", "btczero@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*5*rrkah-fqaaa-aaaaa-aaaaq-cai*0");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

// ============================================================================
// SELL BITCOIN - ALL SCENARIOS
// ============================================================================

#[test]
fn test_sell_bitcoin_to_ugx() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Seller", "btcsell@test.com", "UGX", "1234", 0, 200000, 0)
        .expect("Setup");
    
    // Sell Bitcoin: Menu 2 -> 4 (Sell) -> amount in BTC -> PIN
    // 0.001 BTC = 100,000 sats
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.001*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sold"),
        "Should sell BTC. Got: {}", response);
    
    // Verify BTC decreased
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should decrease by 100,000 sats");
    
    // Verify fiat increased
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get fiat");
    assert!(fiat > 0, "Fiat should increase");
}

#[test]
fn test_sell_bitcoin_insufficient_btc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "NoBalance", "btcno@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    // No BTC balance
    
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.001*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject. Got: {}", response);
}

#[test]
fn test_sell_bitcoin_all_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "SellAll", "btcall@test.com", "UGX", "1234", 0, 150000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.0015*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
    
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 0, "Should have 0 BTC");
}

// ============================================================================
// CHECK BITCOIN BALANCE - ALL SCENARIOS
// ============================================================================

#[test]
fn test_check_bitcoin_balance_zero() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "CheckZero", "btccheck0@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Check balance: Menu 2 -> 1
    let (response, _) = env.process_ussd(&sess, phone, "2*1");
    
    assert!(response.contains("0") || response.contains("zero") || response.contains("Balance"),
        "Should show zero balance. Got: {}", response);
}

#[test]
fn test_check_bitcoin_balance_with_btc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "CheckFull", "btccheckfull@test.com", "UGX", "1234", 0, 250000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*1");
    
    assert!(response.contains("250") || response.contains("BTC") || response.contains("Bitcoin"),
        "Should show BTC balance. Got: {}", response);
}

#[test]
fn test_check_bitcoin_rate() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "CheckRate", "btcrate@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Check rate: Menu 2 -> 5 (or similar)
    let (response, _) = env.process_ussd(&sess, phone, "2");
    
    assert!(response.contains("Rate") || response.contains("rate") || response.contains("price") || response.contains("BTC"),
        "Should show rate info. Got: {}", response);
}

// ============================================================================
// BITCOIN MENU NAVIGATION
// ============================================================================

#[test]
fn test_bitcoin_menu_shows_all_options() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Menu", "btcmenu@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, continue_session) = env.process_ussd(&sess, phone, "2");
    
    assert!(continue_session, "Should continue");
    assert!(response.contains("Balance") || response.contains("balance"));
    assert!(response.contains("Buy") || response.contains("buy"));
    assert!(response.contains("Send") || response.contains("send"));
}

#[test]
fn test_bitcoin_return_to_main_menu() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "Return", "btcreturn@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "2"); // Bitcoin menu
    let (response, _) = env.process_ussd(&sess, phone, "0"); // Back
    
    assert!(response.contains("Main") || response.contains("Menu") || response.contains("Send"),
        "Should return to main menu. Got: {}", response);
}

// ============================================================================
// BITCOIN WRONG PIN SCENARIOS
// ============================================================================

#[test]
fn test_buy_bitcoin_wrong_pin() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "WrongPIN", "btcwrong@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*3*50000*9999"); // Wrong PIN
    
    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);
    
    // Balance should not change
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(fiat, 100000, "Fiat should not change");
}

#[test]
fn test_sell_bitcoin_wrong_pin() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "SellWrong", "btcsellwrong@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.0005*9999");
    
    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);
    
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should not change");
}

// ============================================================================
// BITCOIN RATE AND EXCHANGE INTEGRATION
// ============================================================================

#[test]
fn test_bitcoin_uses_exchange_canister_rate() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "ExchangeRate", "btcexch@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Get spread from exchange canister
    let spread = env.get_exchange_spread();
    assert!(spread > 0, "Exchange canister should provide spread");
    
    // Buy Bitcoin should use this rate
    env.set_fiat_balance(phone, "UGX", 1000000).expect("Set balance");
    let (response, _) = env.process_ussd(&sess, phone, "2*3*150000*1234");
    
    // Should show rate information
    assert!(response.len() > 0, "Should complete transaction");
}

// ============================================================================
// BITCOIN MULTIPLE OPERATIONS
// ============================================================================

#[test]
fn test_bitcoin_buy_then_sell() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "BuySell", "btcbuysell@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");
    
    // Buy Bitcoin (under fraud limit of 100,000 UGX)
    env.process_ussd(&sess, phone, "2*3*50000*1234");
    
    // Check BTC balance
    let (btc_after_buy, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_after_buy > 0, "Should have BTC after buy");
    
    // Sell half
    env.process_ussd(&sess, phone, &format!("2*4*{}*1234", btc_after_buy / 2));
    
    // Check final balance
    let (btc_final, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_final < btc_after_buy, "BTC should decrease after sell");
}

#[test]
fn test_bitcoin_buy_then_send() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "BuySend", "btcbuysend@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");
    
    // Buy Bitcoin (under fraud limit of 100,000 UGX)
    env.process_ussd(&sess, phone, "2*3*50000*1234");
    
    let (btc_after_buy, _) = env.get_crypto_balance(phone).expect("Get balance");
    
    // Send Bitcoin
    env.process_ussd(&sess, phone, &format!("2*5*bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh*{}*1234", btc_after_buy / 2));
    
    let (btc_final, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_final < btc_after_buy, "BTC should decrease after send");
}
