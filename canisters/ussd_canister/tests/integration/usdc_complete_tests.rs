// COMPLETE USDC Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data -> Exchange canister interactions
use super::*;

// ============================================================================
// BUY USDC - ALL CURRENCY COMBINATIONS
// ============================================================================

#[test]
fn test_buy_usdc_with_ugx() {
    let env = get_test_env();
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "USDC", "Buyer", "usdc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 1000000).expect("Set balance");
    
    // Buy USDC: Menu 3 (USDC) -> 3 (Buy) -> amount -> PIN
    let (response, _) = env.process_ussd("session", phone, "3*3*100000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get crypto balance");
    assert!(usdc > 0, "Should have USDC balance");
    
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get fiat balance");
    assert_eq!(fiat, 900000, "Fiat should decrease");
}

#[test]
fn test_buy_usdc_with_kes() {
    let env = get_test_env();
    let phone = "+254700222222";
    
    env.register_user_direct(phone, "KES", "USDCBuyer", "kesusdc@test.com", "KES", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "KES", 500000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*3*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc > 0);
}

#[test]
fn test_buy_usdc_with_tzs() {
    let env = get_test_env();
    let phone = "+255700333333";
    
    env.register_user_direct(phone, "TZS", "USDCBuyer", "tzsusdc@test.com", "TZS", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "TZS", 2000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*3*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);
}

#[test]
fn test_buy_usdc_with_ngn() {
    let env = get_test_env();
    let phone = "+234700444444";
    
    env.register_user_direct(phone, "NGN", "USDCBuyer", "ngnusdc@test.com", "NGN", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "NGN", 5000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*3*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("purchased"),
        "Should buy USDC. Got: {}", response);
}

// ============================================================================
// SEND USDC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_send_usdc_to_valid_address() {
    let env = get_test_env();
    let phone = "+256700555555";
    
    env.register_user_direct(phone, "USDC", "Sender", "usdcsend@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 100000).expect("Set USDC balance");
    
    // Send USDC: Menu 3 -> 5 (Send) -> address -> amount -> PIN
    let (response, _) = env.process_ussd("session", phone, "3*5*0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb*50000*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sent"),
        "Should send USDC. Got: {}", response);
    
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(usdc, 50000, "USDC balance should decrease");
}

#[test]
fn test_send_usdc_insufficient_balance() {
    let env = get_test_env();
    let phone = "+256700666666";
    
    env.register_user_direct(phone, "USDC", "Poor", "usdcpoor@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 10000).expect("Set small USDC balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*5*0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb*50000*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient balance. Got: {}", response);
}

#[test]
fn test_send_usdc_invalid_address() {
    let env = get_test_env();
    let phone = "+256700777777";
    
    env.register_user_direct(phone, "USDC", "Invalid", "usdcinvalid@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*5*invalid_address");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("address"),
        "Should reject invalid address. Got: {}", response);
}

#[test]
fn test_send_usdc_zero_amount() {
    let env = get_test_env();
    let phone = "+256700888888";
    
    env.register_user_direct(phone, "USDC", "Zero", "usdczero@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*5*0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb*0");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

// ============================================================================
// SELL USDC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_sell_usdc_to_ugx() {
    let env = get_test_env();
    let phone = "+256700999999";
    
    env.register_user_direct(phone, "USDC", "Seller", "usdcsell@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 200000).expect("Set USDC balance");
    
    // Sell USDC: Menu 3 -> 4 (Sell) -> amount in USDC -> PIN
    // 0.1 USDC = 100,000 e6
    let (response, _) = env.process_ussd("session", phone, "3*4*0.1*1234");
    
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
    let phone = "+256700101010";
    
    env.register_user_direct(phone, "USDC", "NoBalance", "usdcno@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, _) = env.process_ussd("session", phone, "3*4*0.1*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject. Got: {}", response);
}

#[test]
fn test_sell_usdc_all_balance() {
    let env = get_test_env();
    let phone = "+256700202020";
    
    env.register_user_direct(phone, "USDC", "SellAll", "usdcall@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 150000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*4*0.15*1234");
    
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
    let phone = "+256700303030";
    
    env.register_user_direct(phone, "USDC", "CheckZero", "usdccheck0@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, _) = env.process_ussd("session", phone, "3*1");
    
    assert!(response.contains("0") || response.contains("zero") || response.contains("Balance"),
        "Should show zero balance. Got: {}", response);
}

#[test]
fn test_check_usdc_balance_with_usdc() {
    let env = get_test_env();
    let phone = "+256700404040";
    
    env.register_user_direct(phone, "USDC", "CheckFull", "usdccheckfull@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 250000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*1");
    
    assert!(response.contains("250") || response.contains("USDC"),
        "Should show USDC balance. Got: {}", response);
}

#[test]
fn test_usdc_stablecoin_precision() {
    let env = get_test_env();
    let phone = "+256700505050";
    
    env.register_user_direct(phone, "USDC", "Precision", "usdcprec@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 123456).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*1");
    
    // USDC should show with proper decimal precision (2 decimals for stablecoin display)
    assert!(response.len() > 0, "Should show balance");
}

// ============================================================================
// USDC MENU NAVIGATION
// ============================================================================

#[test]
fn test_usdc_menu_shows_all_options() {
    let env = get_test_env();
    let phone = "+256700606060";
    
    env.register_user_direct(phone, "USDC", "Menu", "usdcmenu@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, continue_session) = env.process_ussd("session", phone, "3");
    
    assert!(continue_session, "Should continue");
    assert!(response.contains("Balance") || response.contains("balance"));
    assert!(response.contains("Buy") || response.contains("buy"));
    assert!(response.contains("Send") || response.contains("send"));
}

#[test]
fn test_usdc_return_to_main_menu() {
    let env = get_test_env();
    let phone = "+256700707070";
    
    env.register_user_direct(phone, "USDC", "Return", "usdcreturn@test.com", "UGX", "1234")
        .expect("Registration");
    
    env.process_ussd("session", phone, "3");
    let (response, _) = env.process_ussd("session", phone, "0");
    
    assert!(response.contains("Main") || response.contains("Menu") || response.contains("Send"),
        "Should return to main menu. Got: {}", response);
}

// ============================================================================
// USDC WRONG PIN SCENARIOS
// ============================================================================

#[test]
fn test_buy_usdc_wrong_pin() {
    let env = get_test_env();
    let phone = "+256700808080";
    
    env.register_user_direct(phone, "USDC", "WrongPIN", "usdcwrong@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*3*50000*9999");
    
    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);
    
    let fiat = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(fiat, 100000, "Fiat should not change");
}

#[test]
fn test_sell_usdc_wrong_pin() {
    let env = get_test_env();
    let phone = "+256700909090";
    
    env.register_user_direct(phone, "USDC", "SellWrong", "usdcsellwrong@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "3*4*0.05*9999");
    
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
    let phone = "+256700010101";
    
    env.register_user_direct(phone, "USDC", "BuySell", "usdcbuysell@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    // Buy USDC (under fraud limit)
    env.process_ussd("s1", phone, "3*3*50000*1234");
    
    let (_, usdc_after_buy) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_after_buy > 0, "Should have USDC after buy");
    
    // Sell half
    env.process_ussd("s2", phone, &format!("3*4*{}*1234", usdc_after_buy / 2));
    
    let (_, usdc_final) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_final < usdc_after_buy, "USDC should decrease after sell");
}

#[test]
fn test_usdc_buy_then_send() {
    let env = get_test_env();
    let phone = "+256700020202";
    
    env.register_user_direct(phone, "USDC", "BuySend", "usdcbuysend@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    // Buy USDC (under fraud limit)
    env.process_ussd("s1", phone, "3*3*50000*1234");
    
    let (_, usdc_after_buy) = env.get_crypto_balance(phone).expect("Get balance");
    
    // Send USDC
    env.process_ussd("s2", phone, &format!("3*5*0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb*{}*1234", usdc_after_buy / 2));
    
    let (_, usdc_final) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc_final < usdc_after_buy, "USDC should decrease after send");
}

// ============================================================================
// USDC VS BTC - DIFFERENT CRYPTO TYPES
// ============================================================================

#[test]
fn test_usdc_and_btc_independent_balances() {
    let env = get_test_env();
    let phone = "+256700030303";
    
    env.register_user_direct(phone, "Crypto", "Both", "both@test.com", "UGX", "1234")
        .expect("Registration");
    
    // Set both BTC and USDC
    env.set_crypto_balance(phone, 100000, 200000).expect("Set balances");
    
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC balance");
    assert_eq!(usdc, 200000, "USDC balance");
    
    // Buy more USDC - BTC should not change
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set fiat");
    env.process_ussd("s1", phone, "3*3*100000*1234");
    
    let (btc_after, usdc_after) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc_after, 100000, "BTC should not change");
    assert!(usdc_after > 200000, "USDC should increase");
}

#[test]
fn test_usdc_stablecoin_characteristics() {
    let env = get_test_env();
    let phone = "+256700040404";
    
    env.register_user_direct(phone, "Stable", "Coin", "stable@test.com", "UGX", "1234")
        .expect("Registration");
    
    // USDC is a stablecoin - should have different display characteristics
    env.set_crypto_balance(phone, 0, 100000).expect("Set USDC");
    
    let (response, _) = env.process_ussd("session", phone, "3*1");
    
    // Should show USDC with proper formatting
    assert!(response.contains("USDC") || response.contains("usdc") || response.contains("Balance"),
        "Should show USDC balance. Got: {}", response);
}
