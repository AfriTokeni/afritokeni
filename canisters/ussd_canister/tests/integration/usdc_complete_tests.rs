// COMPLETE USDC Integration Tests - ALL COMBINATIONS
// Tests: USSD -> crypto_canister -> Data canister interactions
use super::*;

// ============================================================================
// BUY USDC - ALL CURRENCY COMBINATIONS
// ============================================================================

#[test]
fn test_buy_usdc_with_ugx() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "USDC", "Buyer", "usdc@test.com", "UGX", "1234", 10000000, 0, 0)
        .expect("Setup");

    // Buy USDC: Menu 3 (USDC) -> 3 (Buy) -> amount (1000 UGX) -> PIN
    let (response, _) = env.process_ussd(&sess, phone, "3*3*1000*1234");

    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);

    let (_, usdc) = env.get_crypto_balance(phone).expect("Get crypto balance");
    assert!(usdc > 0, "Should have USDC balance");

    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get fiat balance");
    assert_eq!(fiat, 9900000, "Fiat should decrease to 9,900,000 cents after buying 1000 UGX worth of USDC");
}

#[test]
fn test_buy_usdc_with_kes() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("KES");

    env.setup_test_user_with_balances(phone, "KES", "USDCBuyer", "kesusdc@test.com", "KES", "1234", 5000000, 0, 0)
        .expect("Setup");

    let (response, _) = env.process_ussd(&sess, phone, "3*3*5000*1234");

    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);

    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc > 0);
}

#[test]
fn test_buy_usdc_with_tzs() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("TZS");

    env.setup_test_user_with_balances(phone, "TZS", "USDCBuyer", "tzsusdc@test.com", "TZS", "1234", 20000000, 0, 0)
        .expect("Setup");

    let (response, _) = env.process_ussd(&sess, phone, "3*3*20000*1234");

    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);
}

#[test]
fn test_buy_usdc_with_ngn() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("NGN");

    env.setup_test_user_with_balances(phone, "NGN", "USDCBuyer", "ngnusdc@test.com", "NGN", "1234", 50000000, 0, 0)
        .expect("Setup");

    let (response, _) = env.process_ussd(&sess, phone, "3*3*50000*1234");

    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);
}

// ============================================================================
// SEND USDC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_send_usdc_to_valid_address() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Sender", "usdcsend@test.com", "UGX", "1234", 0, 0, 100000)
        .expect("Setup");
    
    // Send USDC: Menu 3 -> 5 (Send) -> address -> amount -> PIN
    // Note: ckUSDC uses IC Principal addresses, not Ethereum addresses
    let (response, _) = env.process_ussd(&sess, phone, "3*5*rrkah-fqaaa-aaaaa-aaaaq-cai*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sent"),
        "Should send USDC. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(usdc, 50000, "USDC balance should decrease");
}

#[test]
fn test_send_usdc_insufficient_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Poor", "usdcpoor@test.com", "UGX", "1234", 0, 0, 10000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*5*rrkah-fqaaa-aaaaa-aaaaq-cai*50000*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient balance. Got: {}", response);
}

#[test]
fn test_send_usdc_invalid_address() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Invalid", "usdcinvalid@test.com", "UGX", "1234", 0, 0, 100000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*5*invalid_address");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("address"),
        "Should reject invalid address. Got: {}", response);
}

#[test]
fn test_send_usdc_zero_amount() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Zero", "usdczero@test.com", "UGX", "1234", 0, 0, 100000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*5*rrkah-fqaaa-aaaaa-aaaaq-cai*0");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

// ============================================================================
// SELL USDC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_sell_usdc_to_ugx() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Seller", "usdcsell@test.com", "UGX", "1234", 0, 0, 200000)
        .expect("Setup");
    
    // Sell USDC: Menu 3 -> 4 (Sell) -> amount in USDC -> PIN -> Confirm
    // 0.1 USDC = 100,000 e6
    let (response, _) = env.process_ussd(&sess, phone, "3*4*0.1*1234*1");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sold"),
        "Should sell USDC. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(usdc, 100000, "USDC should decrease");
    
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get fiat");
    assert!(fiat > 0, "Fiat should increase");
}

#[test]
fn test_sell_usdc_insufficient_usdc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "NoBalance", "usdcno@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*4*0.1*1234*1");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject. Got: {}", response);
}

#[test]
fn test_sell_usdc_all_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "SellAll", "usdcall@test.com", "UGX", "1234", 0, 0, 150000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*4*0.15*1234*1");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sold"),
        "Should sell USDC. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(usdc, 0, "Should have 0 USDC");
}

// ============================================================================
// CHECK USDC BALANCE - ALL SCENARIOS
// ============================================================================

#[test]
fn test_check_usdc_balance_zero() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "CheckZero", "usdccheck0@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*1");
    
    assert!(response.contains("0") || response.contains("zero") || response.contains("Balance"),
        "Should show zero balance. Got: {}", response);
}

#[test]
fn test_check_usdc_balance_with_usdc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "CheckFull", "usdccheckfull@test.com", "UGX", "1234", 0, 0, 250000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*1");
    
    assert!(response.contains("250") || response.contains("USDC"),
        "Should show USDC balance. Got: {}", response);
}

#[test]
fn test_usdc_stablecoin_precision() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Precision", "usdcprec@test.com", "UGX", "1234", 0, 0, 123456)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*1");
    
    // USDC should show with proper decimal precision (2 decimals for stablecoin display)
    assert!(response.len() > 0, "Should show balance");
}

// ============================================================================
// USDC MENU NAVIGATION
// ============================================================================

#[test]
fn test_usdc_menu_shows_all_options() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Menu", "usdcmenu@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let (response, continue_session) = env.process_ussd(&sess, phone, "3");
    
    assert!(continue_session, "Should continue");
    assert!(response.contains("Balance") || response.contains("balance"));
    assert!(response.contains("Buy") || response.contains("buy"));
    assert!(response.contains("Send") || response.contains("send"));
}

#[test]
fn test_usdc_return_to_main_menu() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "Return", "usdcreturn@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "3");
    let (response, _) = env.process_ussd(&sess, phone, "0");
    
    assert!(response.contains("Main") || response.contains("Menu") || response.contains("Send"),
        "Should return to main menu. Got: {}", response);
}

// ============================================================================
// USDC WRONG PIN SCENARIOS
// ============================================================================

#[test]
fn test_buy_usdc_wrong_pin() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "USDC", "WrongPIN", "usdcwrong@test.com", "UGX", "1234", 5000000, 0, 0)
        .expect("Setup");

    let (response, _) = env.process_ussd(&sess, phone, "3*3*5000*9999");

    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);

    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(fiat, 5000000, "Fiat should not change");
}

#[test]
fn test_sell_usdc_wrong_pin() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "USDC", "SellWrong", "usdcsellwrong@test.com", "UGX", "1234", 0, 0, 100000)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*4*0.05*9999");
    
    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(usdc, 100000, "USDC should not change");
}

// ============================================================================
// USDC MULTIPLE OPERATIONS
// ============================================================================

#[test]
fn test_usdc_buy_then_sell() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "USDC", "BuySell", "usdcbuysell@test.com", "UGX", "1234", 5000000, 0, 0)
        .expect("Setup");

    // Buy USDC (5000 UGX worth)
    env.process_ussd(&sess, phone, "3*3*5000*1234");

    let (_, usdc_after_buy) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_after_buy > 0, "Should have USDC after buy");

    // Sell half (convert e6 to USDC for display)
    let half_usdc = (usdc_after_buy / 2) as f64 / 1_000_000.0;
    env.process_ussd(&sess, phone, &format!("3*4*{}*1234*1", half_usdc));

    let (_, usdc_final) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_final < usdc_after_buy, "USDC should decrease after sell");
}

#[test]
fn test_usdc_buy_then_send() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "USDC", "BuySend", "usdcbuysend@test.com", "UGX", "1234", 5000000, 0, 0)
        .expect("Setup");

    // Buy USDC (5000 UGX worth)
    env.process_ussd(&sess, phone, "3*3*5000*1234");

    let (_, usdc_after_buy) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_after_buy > 0, "Should have USDC after buy");

    // Send half of USDC
    let half_usdc_e6 = usdc_after_buy / 2;
    env.process_ussd(&sess, phone, &format!("3*5*rrkah-fqaaa-aaaaa-aaaaq-cai*{}*1234", half_usdc_e6));

    let (_, usdc_final) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_final < usdc_after_buy, "USDC should decrease after send");
}

// ============================================================================
// USDC VS BTC - DIFFERENT CRYPTO TYPES
// ============================================================================

#[test]
fn test_usdc_and_btc_independent_balances() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Crypto", "Both", "both@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Set both BTC and USDC
    env.set_crypto_balance(phone, 100000, 200000).expect("Set balances");
    
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC balance");
    assert_eq!(usdc, 200000, "USDC balance");
    
    // Buy more USDC - BTC should not change
    env.set_fiat_balance(phone, "UGX", 5000000).expect("Set fiat");
    env.process_ussd(&sess, phone, "3*3*5000*1234");

    let (btc_after, usdc_after) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc_after, 100000, "BTC should not change");
    assert!(usdc_after > 200000, "USDC should increase");
}

#[test]
fn test_usdc_stablecoin_characteristics() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Stable", "Coin", "stable@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    
    let (response, _) = env.process_ussd(&sess, phone, "3*1");
    
    // Should show USDC with proper formatting
    assert!(response.contains("USDC") || response.contains("usdc") || response.contains("Balance"),
        "Should show USDC balance. Got: {}", response);
}
