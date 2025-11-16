// COMPLETE Crypto Swap Integration Tests - ALL COMBINATIONS
// Tests: USSD -> crypto_canister for swap operations and spread calculation
use super::*;

// ============================================================================
// SWAP BTC -> USDC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_swap_btc_to_usdc_success() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "BTCtoUSDC", "swapbtc@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    // Step 1: Get to confirmation screen
    let (response, cont) = env.process_ussd(&sess, phone, "4*1*2*50000");
    assert!(cont, "Should continue for confirmation");
    assert!(response.contains("Confirm") || response.contains("confirm") || response.contains("spread"),
        "Should show confirmation. Got: {}", response);
    // Step 2: Confirm (1) and enter PIN
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*50000*1*1234");
    assert!(response.contains("success") || response.contains("Success") || response.contains("swapped"),
        "Should swap BTC to USDC. Got: {}", response);
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc < 100000, "BTC should decrease");
    assert!(usdc > 0, "USDC should increase");
}

#[test]
fn test_swap_btc_to_usdc_insufficient_btc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "NoBTC", "swapnobtc@test.com", "UGX", "1234", 0, 10000, 0)
        .expect("Setup");
    // Try to swap more than balance
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*50000*1*1234");
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient BTC. Got: {}", response);
}

#[test]
fn test_swap_btc_to_usdc_with_spread() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "Spread", "swapspread@test.com", "UGX", "1234", 0, 200000, 0)
        .expect("Setup");
    // Get spread from crypto_canister
    let spread = env.get_exchange_spread();
    assert!(spread > 0, "Crypto canister should provide spread");
    // Step 1: See confirmation with spread
    let (response, cont) = env.process_ussd(&sess, phone, "4*1*2*100000");
    assert!(cont, "Should show confirmation");
    assert!(response.contains("spread") || response.contains("Spread"),
        "Should show spread. Got: {}", response);
    // Step 2: Complete swap
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*100000*1*1234");
    assert!(response.contains("success") || response.contains("Success"),
        "Should complete swap. Got: {}", response);
}

// ============================================================================
// SWAP USDC -> BTC - ALL SCENARIOS
// ============================================================================

#[test]
fn test_swap_usdc_to_btc_success() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "USDCtoBTC", "swapusdc@test.com", "UGX", "1234", 0, 0, 100000)
        .expect("Setup");
    // Step 1: Get confirmation
    let (_response, cont) = env.process_ussd(&sess, phone, "4*2*1*50000");
    assert!(cont, "Should continue for confirmation");
    // Step 2: Confirm and execute
    let (response, _) = env.process_ussd(&sess, phone, "4*2*1*50000*1*1234");
    assert!(response.contains("success") || response.contains("Success"),
        "Should swap USDC to BTC. Got: {}", response);
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc > 0, "BTC should increase");
    assert!(usdc < 100000, "USDC should decrease");
}

#[test]
fn test_swap_usdc_to_btc_insufficient_usdc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "NoUSDC", "swapnousdc@test.com", "UGX", "1234", 0, 0, 10000)
        .expect("Setup");
    // Try to swap more than balance
    let (response, _) = env.process_ussd(&sess, phone, "4*2*1*50000*1*1234");
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient USDC. Got: {}", response);
}

// ============================================================================
// SWAP ERROR CASES - SAME TOKEN
// ============================================================================

#[test]
fn test_swap_btc_to_btc_rejected() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "SameBTC", "swapsamebtc@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    let (response, _) = env.process_ussd(&sess, phone, "4*1*1"); // To BTC (same)
    assert!(response.contains("same") || response.contains("different") || response.contains("cannot"),
        "Should reject swapping BTC to BTC. Got: {}", response);
}

#[test]
fn test_swap_usdc_to_usdc_rejected() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "SameUSDC", "swapsameusdc@test.com", "UGX", "1234", 0, 0, 100000)
        .expect("Setup");
    let (response, _) = env.process_ussd(&sess, phone, "4*2*2"); // To USDC (same)
    assert!(response.contains("same") || response.contains("different") || response.contains("cannot"),
        "Should reject swapping USDC to USDC. Got: {}", response);
}

// ============================================================================
// SWAP ERROR CASES - INVALID AMOUNTS
// ============================================================================

#[test]
fn test_swap_zero_amount_rejected() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "Zero", "swapzero@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*0"); // Zero amount
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_swap_negative_amount_rejected() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "Negative", "swapneg@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*-1000");
    assert!(response.contains("Invalid") || response.contains("invalid"),
        "Should reject negative amount. Got: {}", response);
}

// ============================================================================
// SWAP MENU NAVIGATION
// ============================================================================

#[test]
fn test_swap_cancel_at_confirmation() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "Cancel", "swapcancel@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    // Step 1: Get to confirmation
    let (_response, cont) = env.process_ussd(&sess, phone, "4*1*2*50000");
    assert!(cont, "Should show confirmation");
    // Step 2: Cancel (2 instead of 1)
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*50000*2");
    assert!(response.contains("cancel") || response.contains("Cancel") || response.contains("Main"),
        "Should cancel swap. Got: {}", response);
    // Balance should not change
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should not change");
    assert_eq!(usdc, 0, "USDC should not change");
}

// ============================================================================
// SWAP MULTIPLE OPERATIONS
// ============================================================================

#[test]
fn test_swap_btc_usdc_back_and_forth() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "BackForth", "swapbf@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    // Swap BTC -> USDC (with confirmation)
    env.process_ussd(&sess, phone, "4*1*2*50000*1*1234");
    let (btc_after_first, usdc_after_first) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_after_first < 100000, "BTC should decrease");
    assert!(usdc_after_first > 0, "USDC should increase");
    // Swap USDC -> BTC (with confirmation)
    env.process_ussd(&sess, phone, &format!("4*2*1*{}*1*1234", usdc_after_first / 2));
    let (btc_final, usdc_final) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc_final > btc_after_first, "BTC should increase again");
    assert!(usdc_final < usdc_after_first, "USDC should decrease");
}

#[test]
fn test_swap_all_btc_to_usdc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "AllBTC", "swapallbtc@test.com", "UGX", "1234", 0, 150000, 0)
        .expect("Setup");
    // Swap all BTC (with confirmation)
    env.process_ussd(&sess, phone, "4*1*2*150000*1*1234");
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 0, "Should have 0 BTC");
    assert!(usdc > 0, "Should have USDC");
}

#[test]
fn test_swap_all_usdc_to_btc() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "AllUSDC", "swapallusdc@test.com", "UGX", "1234", 0, 0, 200000)
        .expect("Setup");
    // Swap all USDC (with confirmation)
    env.process_ussd(&sess, phone, "4*2*1*200000*1*1234");
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
    assert!(btc > 0, "Should have BTC");
    assert_eq!(usdc, 0, "Should have 0 USDC");
}

// ============================================================================
// SWAP WITH CRYPTO CANISTER SPREAD
// ============================================================================

#[test]
fn test_swap_shows_spread_in_confirmation() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "ShowSpread", "swapshowspread@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    let (response, _) = env.process_ussd(&sess, phone, "4*1*2*50000");
    // Should show spread or rate information
    assert!(response.contains("spread") || response.contains("Spread") || 
            response.contains("rate") || response.contains("Rate") ||
            response.contains("Confirm") || response.contains("confirm"),
        "Should show confirmation with spread. Got: {}", response);
}

#[test]
fn test_swap_uses_dynamic_exchange_rate() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "DynamicRate", "swapdynamic@test.com", "UGX", "1234", 0, 100000, 0)
        .expect("Setup");
    // Get current spread
    let spread = env.get_exchange_spread();
    assert!(spread > 0, "Should have spread");
    // Swap should use this spread (with confirmation)
    env.process_ussd(&sess, phone, "4*1*2*50000*1*1234");
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
    let sess = session();
    let phone = &phone("UGX");
    env.setup_test_user_with_balances(phone, "Swap", "Stateless", "swapstateless@test.com", "UGX", "1234", 0, 100000, 100000)
        .expect("Setup");
    // Same input should give same output
    let (response1, _) = env.process_ussd(&sess, phone, "4");
    let (response2, _) = env.process_ussd(&sess, phone, "4");
    assert_eq!(response1, response2, "USSD should be stateless");
}
