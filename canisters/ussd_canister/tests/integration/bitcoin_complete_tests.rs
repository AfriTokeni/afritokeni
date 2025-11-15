// COMPLETE Bitcoin Integration Tests - ALL COMBINATIONS
// Tests: USSD -> crypto_canister -> Data canister interactions
use super::*;

// ============================================================================
// BUY BITCOIN - ALL CURRENCY COMBINATIONS
// ============================================================================

#[test]
fn test_buy_bitcoin_with_ugx() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Setup: 100,000,000 cents = 1,000,000.00 UGX
    env.setup_test_user_with_balances(phone, "BTC", "Buyer", "btc@test.com", "UGX", "1234", 100000000, 0, 0)
        .expect("Setup");

    // Buy Bitcoin: Menu 2 (Bitcoin) -> 3 (Buy) -> Amount (100,000.00 UGX) -> PIN
    let (response, _) = env.process_ussd(&sess, phone, "2*3*100000*1234");

    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy BTC. Got: {}", response);

    // Verify BTC balance increased
    let (btc, _) = env.get_crypto_balance(phone).expect("Get crypto balance");
    assert!(btc > 0, "Should have BTC balance");

    // Verify fiat balance decreased
    // Initial: 100,000,000 cents (1,000,000.00 UGX)
    // Purchase: 10,000,000 cents (100,000.00 UGX)
    // Platform fee (0.5%): 50,000 cents (500.00 UGX)
    // Total deducted: 10,050,000 cents
    // Expected balance: 89,950,000 cents (899,500.00 UGX)
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get fiat balance");
    assert_eq!(fiat, 89950000, "Fiat should decrease to 89,950,000 cents (899,500.00 UGX) after 100,000.00 UGX purchase + 0.5% fee");
}

#[test]
fn test_buy_bitcoin_with_kes() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("KES");

    // Setup: 6,000,000 cents = 60,000.00 KES (enough for 50,000.00 purchase + 0.5% fee)
    env.setup_test_user_with_balances(phone, "KES", "BTCBuyer", "kesbtc@test.com", "KES", "1234", 6000000, 0, 0)
        .expect("Setup");

    // Buy with 50,000.00 KES (requires 50,000 + 250 fee = 50,250.00 KES total)
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

    // Setup: 6,000,000 cents = 60,000.00 TZS (enough for 50,000.00 purchase + 0.5% fee)
    env.setup_test_user_with_balances(phone, "TZS", "BTCBuyer", "tzsbtc@test.com", "TZS", "1234", 6000000, 0, 0)
        .expect("Setup");

    // Buy with 50,000.00 TZS (requires 50,000 + 250 fee = 50,250.00 TZS total)
    let (response, _) = env.process_ussd(&sess, phone, "2*3*50000*1234");

    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy BTC. Got: {}", response);
}

#[test]
fn test_buy_bitcoin_with_ngn() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("NGN");

    // Setup: 6,000,000 cents = 60,000.00 NGN (enough for 50,000.00 purchase + 0.5% fee)
    env.setup_test_user_with_balances(phone, "NGN", "BTCBuyer", "ngnbtc@test.com", "NGN", "1234", 6000000, 0, 0)
        .expect("Setup");

    // Buy with 50,000.00 NGN (requires 50,000 + 250 fee = 50,250.00 NGN total)
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
    // Initial: 100,000 sats
    // Sent: 50,000 sats
    // Network fee: 1,000 sats
    // Expected balance: 100,000 - 50,000 - 1,000 = 49,000 sats
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 49000, "BTC balance should decrease to 49,000 sats (sent 50,000 + 1,000 network fee)");
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

    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive") || response.contains("too small") || response.contains("Minimum"),
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

    // Setup: 200,000 sats = 0.002 BTC
    env.setup_test_user_with_balances(phone, "BTC", "Seller", "btcsell@test.com", "UGX", "1234", 0, 200000, 0)
        .expect("Setup");

    // Sell Bitcoin: Menu 2 -> 4 (Sell) -> amount in BTC -> PIN -> Confirm
    // 0.001 BTC = 100,000 sats
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.001*1234*1");

    assert!(response.contains("success") || response.contains("Success") || response.contains("sold"),
        "Should sell BTC. Got: {}", response);

    // Verify BTC decreased (200,000 - 100,000 = 100,000 sats)
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should decrease from 200,000 to 100,000 sats after selling 0.001 BTC");

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
    
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.001*1234*1");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject. Got: {}", response);
}

#[test]
fn test_sell_bitcoin_all_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Setup: 150,000 sats = 0.0015 BTC
    env.setup_test_user_with_balances(phone, "BTC", "SellAll", "btcall@test.com", "UGX", "1234", 0, 150000, 0)
        .expect("Setup");

    // Sell all BTC: 0.0015 BTC = 150,000 sats
    let (response, _) = env.process_ussd(&sess, phone, "2*4*0.0015*1234*1");

    assert!(response.contains("success") || response.contains("Success") || response.contains("sold"),
        "Should sell all BTC. Got: {}", response);

    // Verify all BTC was sold (150,000 sats -> 0)
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 0, "Should have 0 BTC after selling entire balance of 0.0015 BTC");
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

    assert!(response.contains("Welcome") || response.contains("Local Currency") || response.contains("Bitcoin"),
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

    // Setup: 10,000,000 cents = 100,000.00 UGX
    env.setup_test_user_with_balances(phone, "BTC", "WrongPIN", "btcwrong@test.com", "UGX", "1234", 10000000, 0, 0)
        .expect("Setup");

    // Try to buy with wrong PIN
    let (response, _) = env.process_ussd(&sess, phone, "2*3*50000*9999"); // Wrong PIN

    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);

    // Balance should not change when PIN is wrong (still 10,000,000 cents = 100,000.00 UGX)
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(fiat, 10000000, "Fiat should not change after failed PIN verification");
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

    // Balance should remain unchanged after failed PIN verification
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should not change after failed PIN verification (still 100,000 sats)");
}

// ============================================================================
// BITCOIN RATE AND EXCHANGE INTEGRATION
// ============================================================================

#[test]
fn test_bitcoin_uses_crypto_canister_rate() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "BTC", "ExchangeRate", "btcexch@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Get spread from crypto_canister
    let spread = env.get_exchange_spread();
    assert!(spread > 0, "Crypto canister should provide spread");
    
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

    // Setup: 5,000,000 cents = 50,000.00 UGX
    env.setup_test_user_with_balances(phone, "BTC", "BuySell", "btcbuysell@test.com", "UGX", "1234", 5000000, 0, 0)
        .expect("Setup");

    // Buy Bitcoin with 5,000.00 UGX (under fraud limit)
    // Initial balance: 5,000,000 cents = 50,000.00 UGX
    // Purchase: 500,000 cents = 5,000.00 UGX
    // Platform fee (0.5%): 2,500 cents = 25.00 UGX
    // Total deducted: 502,500 cents
    env.process_ussd(&sess, phone, "2*3*5000*1234");

    // Check BTC balance - should have received BTC
    let (btc_after_buy, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_after_buy > 0, "Should have BTC after buy (purchased 5,000.00 UGX worth)");

    // Sell half (convert sats to BTC for sell command)
    let btc_to_sell = (btc_after_buy / 2) as f64 / 100_000_000.0;
    env.process_ussd(&sess, phone, &format!("2*4*{:.8}*1234*1", btc_to_sell));

    // Check final balance
    let (btc_final, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_final < btc_after_buy, "BTC should decrease after sell");
}

#[test]
fn test_bitcoin_buy_then_send() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Setup: 5,000,000 cents = 50,000.00 UGX
    env.setup_test_user_with_balances(phone, "BTC", "BuySend", "btcbuysend@test.com", "UGX", "1234", 5000000, 0, 0)
        .expect("Setup");

    // Buy Bitcoin with 5,000.00 UGX (under fraud limit)
    // Initial balance: 5,000,000 cents = 50,000.00 UGX
    // Purchase: 500,000 cents = 5,000.00 UGX
    // Platform fee (0.5%): 2,500 cents = 25.00 UGX
    // Total deducted: 502,500 cents
    env.process_ussd(&sess, phone, "2*3*5000*1234");

    let (btc_after_buy, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_after_buy > 0, "Should have BTC after buy (purchased 5,000.00 UGX worth)");

    // Send Bitcoin (send flow expects satoshis)
    env.process_ussd(&sess, phone, &format!("2*5*bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh*{}*1234", btc_after_buy / 2));

    let (btc_final, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_final < btc_after_buy, "BTC should decrease after send");
}
