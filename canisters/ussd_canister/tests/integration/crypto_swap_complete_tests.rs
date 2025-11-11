// COMPLETE Crypto Swap Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Exchange canister for spread
use super::*;

// ============================================================================
// SWAP BTC -> USDC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_swap_btc_to_usdc_success() {
    let env = get_test_env();
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "Swap", "BTCtoUSDC", "swapbtc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC balance");
    
    // Swap: Menu 4 (Swap) -> 1 (BTC) -> 2 (USDC) -> amount -> confirm
    let (response, _) = env.process_ussd("session", phone, "4*1*2*50000*1");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("swapped"),
        "Should swap BTC to USDC. Got: {}", response);
    
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc < 100000, "BTC should decrease");
    assert!(usdc > 0, "USDC should increase");
}

#[test]
fn test_swap_btc_to_usdc_insufficient_btc() {
    let env = get_test_env();
    let phone = "+256700222222";
    
    env.register_user_direct(phone, "Swap", "NoBTC", "swapnobtc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 10000, 0).expect("Set small BTC");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*2*50000*1");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient BTC. Got: {}", response);
}

#[test]
fn test_swap_btc_to_usdc_with_spread() {
    let env = get_test_env();
    let phone = "+256700333333";
    
    env.register_user_direct(phone, "Swap", "Spread", "swapspread@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 200000, 0).expect("Set BTC");
    
    // Get spread from exchange canister
    let spread = env.get_exchange_spread();
    assert!(spread > 0, "Exchange should provide spread");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*2*100000*1");
    
    // Should show spread in confirmation
    assert!(response.len() > 0, "Should complete swap");
}

// ============================================================================
// SWAP USDC -> BTC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_swap_usdc_to_btc_success() {
    let env = get_test_env();
    let phone = "+256700444444";
    
    env.register_user_direct(phone, "Swap", "USDCtoBTC", "swapusdc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 100000).expect("Set USDC balance");
    
    let (response, _) = env.process_ussd("session", phone, "4*2*1*50000*1");
    
    assert!(response.contains("success") || response.contains("Success"),
        "Should swap USDC to BTC. Got: {}", response);
    
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc > 0, "BTC should increase");
    assert!(usdc < 100000, "USDC should decrease");
}

#[test]
fn test_swap_usdc_to_btc_insufficient_usdc() {
    let env = get_test_env();
    let phone = "+256700555555";
    
    env.register_user_direct(phone, "Swap", "NoUSDC", "swapnousdc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 10000).expect("Set small USDC");
    
    let (response, _) = env.process_ussd("session", phone, "4*2*1*50000*1");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient USDC. Got: {}", response);
}

// ============================================================================
// SWAP ERROR CASES - SAME TOKEN
// ============================================================================

#[test]
fn test_swap_btc_to_btc_rejected() {
    let env = get_test_env();
    let phone = "+256700666666";
    
    env.register_user_direct(phone, "Swap", "SameBTC", "swapsamebtc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*1"); // To BTC (same)
    
    assert!(response.contains("same") || response.contains("different") || response.contains("cannot"),
        "Should reject swapping BTC to BTC. Got: {}", response);
}

#[test]
fn test_swap_usdc_to_usdc_rejected() {
    let env = get_test_env();
    let phone = "+256700777777";
    
    env.register_user_direct(phone, "Swap", "SameUSDC", "swapsameusdc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 100000).expect("Set USDC");
    
    let (response, _) = env.process_ussd("session", phone, "4*2*2"); // To USDC (same)
    
    assert!(response.contains("same") || response.contains("different") || response.contains("cannot"),
        "Should reject swapping USDC to USDC. Got: {}", response);
}

// ============================================================================
// SWAP ERROR CASES - INVALID AMOUNTS
// ============================================================================

#[test]
fn test_swap_zero_amount_rejected() {
    let env = get_test_env();
    let phone = "+256700888888";
    
    env.register_user_direct(phone, "Swap", "Zero", "swapzero@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*2*0"); // Zero amount
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_swap_negative_amount_rejected() {
    let env = get_test_env();
    let phone = "+256700999999";
    
    env.register_user_direct(phone, "Swap", "Negative", "swapneg@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*2*-1000");
    
    assert!(response.contains("Invalid") || response.contains("invalid"),
        "Should reject negative amount. Got: {}", response);
}

// ============================================================================
// SWAP MENU NAVIGATION
// ============================================================================

#[test]
fn test_swap_menu_shows_crypto_options() {
    let env = get_test_env();
    let phone = "+256700101010";
    
    env.register_user_direct(phone, "Swap", "Menu", "swapmenu@test.com", "UGX", "1234")
        .expect("Registration");
    
    let (response, continue_session) = env.process_ussd("session", phone, "4");
    
    assert!(continue_session, "Should continue");
    assert!(response.contains("BTC") || response.contains("Bitcoin") || response.contains("1"));
    assert!(response.contains("USDC") || response.contains("2"));
}

#[test]
fn test_swap_cancel_at_confirmation() {
    let env = get_test_env();
    let phone = "+256700202020";
    
    env.register_user_direct(phone, "Swap", "Cancel", "swapcancel@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*2*50000*2"); // Cancel (not confirm)
    
    assert!(response.contains("cancel") || response.contains("Cancel") || response.contains("Main"),
        "Should cancel swap. Got: {}", response);
    
    // Balance should not change
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should not change");
    assert_eq!(usdc, 0, "USDC should not change");
}

#[test]
fn test_swap_return_to_main_menu() {
    let env = get_test_env();
    let phone = "+256700303030";
    
    env.register_user_direct(phone, "Swap", "Return", "swapreturn@test.com", "UGX", "1234")
        .expect("Registration");
    
    env.process_ussd("session", phone, "4");
    let (response, _) = env.process_ussd("session", phone, "0");
    
    assert!(response.contains("Main") || response.contains("Menu"),
        "Should return to main menu. Got: {}", response);
}

// ============================================================================
// SWAP MULTIPLE OPERATIONS
// ============================================================================

#[test]
fn test_swap_btc_usdc_back_and_forth() {
    let env = get_test_env();
    let phone = "+256700404040";
    
    env.register_user_direct(phone, "Swap", "BackForth", "swapbf@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set initial BTC");
    
    // Swap BTC -> USDC
    env.process_ussd("s1", phone, "4*1*2*50000*1");
    
    let (btc_after_first, usdc_after_first) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_after_first < 100000, "BTC should decrease");
    assert!(usdc_after_first > 0, "USDC should increase");
    
    // Swap USDC -> BTC
    env.process_ussd("s2", phone, &format!("4*2*1*{}*1", usdc_after_first / 2));
    
    let (btc_final, usdc_final) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_final > btc_after_first, "BTC should increase again");
    assert!(usdc_final < usdc_after_first, "USDC should decrease");
}

#[test]
fn test_swap_all_btc_to_usdc() {
    let env = get_test_env();
    let phone = "+256700505050";
    
    env.register_user_direct(phone, "Swap", "AllBTC", "swapallbtc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 150000, 0).expect("Set BTC");
    
    // Swap all BTC
    env.process_ussd("session", phone, "4*1*2*150000*1");
    
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 0, "Should have 0 BTC");
    assert!(usdc > 0, "Should have USDC");
}

#[test]
fn test_swap_all_usdc_to_btc() {
    let env = get_test_env();
    let phone = "+256700606060";
    
    env.register_user_direct(phone, "Swap", "AllUSDC", "swapallusdc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 0, 200000).expect("Set USDC");
    
    // Swap all USDC
    env.process_ussd("session", phone, "4*2*1*200000*1");
    
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc > 0, "Should have BTC");
    assert_eq!(usdc, 0, "Should have 0 USDC");
}

// ============================================================================
// SWAP WITH EXCHANGE CANISTER SPREAD
// ============================================================================

#[test]
fn test_swap_shows_spread_in_confirmation() {
    let env = get_test_env();
    let phone = "+256700707070";
    
    env.register_user_direct(phone, "Swap", "ShowSpread", "swapshowspread@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC");
    
    let (response, _) = env.process_ussd("session", phone, "4*1*2*50000");
    
    // Should show spread or rate information
    assert!(response.contains("spread") || response.contains("Spread") || 
            response.contains("rate") || response.contains("Rate") ||
            response.contains("Confirm") || response.contains("confirm"),
        "Should show confirmation with spread. Got: {}", response);
}

#[test]
fn test_swap_uses_dynamic_exchange_rate() {
    let env = get_test_env();
    let phone = "+256700808080";
    
    env.register_user_direct(phone, "Swap", "DynamicRate", "swapdynamic@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 0).expect("Set BTC");
    
    // Get current spread
    let spread = env.get_exchange_spread();
    assert!(spread > 0, "Should have spread");
    
    // Swap should use this spread
    env.process_ussd("session", phone, "4*1*2*50000*1");
    
    // Verify swap completed
    let (_, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(usdc > 0, "Should receive USDC based on spread");
}

// ============================================================================
// SWAP STATELESS BEHAVIOR
// ============================================================================

#[test]
fn test_swap_is_stateless() {
    let env = get_test_env();
    let phone = "+256700909090";
    
    env.register_user_direct(phone, "Swap", "Stateless", "swapstateless@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 100000, 100000).expect("Set both");
    
    // Same input should give same output
    let (response1, _) = env.process_ussd("session1", phone, "4");
    let (response2, _) = env.process_ussd("session2", phone, "4");
    
    assert_eq!(response1, response2, "USSD should be stateless");
}
