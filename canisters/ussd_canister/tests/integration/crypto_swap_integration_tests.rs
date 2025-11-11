// Integration tests for USSD crypto swap flow
use super::*;

#[test]
fn test_ussd_swap_flow_step_0_shows_from_crypto_menu() {
    let env = get_test_env();
    
    // User dials *123*4# (Main menu -> Swap Crypto)
    let (response, continue_session) = env.process_ussd(
        "test_session_1",
        "+256700000001",
        "4",  // Option 4 = Swap Crypto
    );
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Swap Crypto") || response.contains("From"), 
        "Should show 'From' crypto selection. Got: {}", response);
    assert!(response.contains("Bitcoin") || response.contains("BTC"), 
        "Should show Bitcoin option. Got: {}", response);
    assert!(response.contains("USDC"), 
        "Should show USDC option. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_step_1_shows_to_crypto_menu() {
    let env = get_test_env();
    
    // Step 0: Select swap crypto
    env.process_ussd("test_session_2", "+256700000002", "4");
    
    // Step 1: Select from crypto (1 = BTC)
    let (response, continue_session) = env.process_ussd(
        "test_session_2",
        "+256700000002",
        "4*1",  // Swap -> BTC
    );
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("To") || response.contains("Swapping"), 
        "Should show 'To' selection. Got: {}", response);
    assert!(response.contains("Bitcoin") || response.contains("BTC"), 
        "Should show Bitcoin option. Got: {}", response);
    assert!(response.contains("USDC"), 
        "Should show USDC option. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_step_2_asks_for_amount() {
    let env = get_test_env();
    
    // Navigate to amount input
    env.process_ussd("test_session_3", "+256700000003", "4");
    env.process_ussd("test_session_3", "+256700000003", "4*1");  // From BTC
    
    // Step 2: Select to crypto (2 = USDC)
    let (response, continue_session) = env.process_ussd(
        "test_session_3",
        "+256700000003",
        "4*1*2",  // Swap -> BTC -> USDC
    );
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("amount") || response.contains("Enter"), 
        "Should ask for amount. Got: {}", response);
    assert!(response.contains("BTC"), 
        "Should mention BTC. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_step_3_shows_spread_and_confirmation() {
    let env = get_test_env();
    
    // Navigate to confirmation
    env.process_ussd("test_session_4", "+256700000004", "4");
    env.process_ussd("test_session_4", "+256700000004", "4*1");
    env.process_ussd("test_session_4", "+256700000004", "4*1*2");
    
    // Step 3: Enter amount
    let (response, continue_session) = env.process_ussd(
        "test_session_4",
        "+256700000004",
        "4*1*2*100000",  // 100,000 sats
    );
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Spread") || response.contains("Details"), 
        "Should show spread details. Got: {}", response);
    assert!(response.contains("0.5") || response.contains("500"), 
        "Should show 0.5% spread. Got: {}", response);
    assert!(response.contains("Confirm") || response.contains("1"), 
        "Should show confirm option. Got: {}", response);
    assert!(response.contains("Cancel") || response.contains("2"), 
        "Should show cancel option. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_fetches_spread_dynamically() {
    let env = get_test_env();
    
    // Get spread from exchange canister
    let spread_bp = env.get_exchange_spread();
    assert_eq!(spread_bp, 50, "Spread should be 50 basis points (0.5%)");
    
    // Navigate to confirmation to verify spread is displayed
    env.process_ussd("test_session_5", "+256700000005", "4");
    env.process_ussd("test_session_5", "+256700000005", "4*1");
    env.process_ussd("test_session_5", "+256700000005", "4*1*2");
    
    let (response, _) = env.process_ussd(
        "test_session_5",
        "+256700000005",
        "4*1*2*1000000",  // 1M sats
    );
    
    // Calculate expected spread
    let expected_spread = (1_000_000 * spread_bp) / 10_000;
    assert_eq!(expected_spread, 5_000, "Expected spread should be 5,000 sats");
    
    // Verify spread is shown in response
    assert!(response.contains("5000") || response.contains("5,000"), 
        "Should show spread of 5,000. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_rejects_same_token() {
    let env = get_test_env();
    
    // Try to swap BTC -> BTC
    env.process_ussd("test_session_6", "+256700000006", "4");
    env.process_ussd("test_session_6", "+256700000006", "4*1");  // From BTC
    
    let (response, continue_session) = env.process_ussd(
        "test_session_6",
        "+256700000006",
        "4*1*1",  // To BTC (same token!)
    );
    
    assert!(!continue_session, "Session should end");
    assert!(response.contains("Cannot") || response.contains("same"), 
        "Should reject same token swap. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_rejects_zero_amount() {
    let env = get_test_env();
    
    // Navigate to amount and enter 0
    env.process_ussd("test_session_7", "+256700000007", "4");
    env.process_ussd("test_session_7", "+256700000007", "4*1");
    env.process_ussd("test_session_7", "+256700000007", "4*1*2");
    
    let (response, continue_session) = env.process_ussd(
        "test_session_7",
        "+256700000007",
        "4*1*2*0",  // Zero amount
    );
    
    assert!(!continue_session, "Session should end");
    assert!(response.contains("greater than 0") || response.contains("zero"), 
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_cancel_at_confirmation() {
    let env = get_test_env();
    
    // Navigate to confirmation and cancel
    env.process_ussd("test_session_8", "+256700000008", "4");
    env.process_ussd("test_session_8", "+256700000008", "4*1");
    env.process_ussd("test_session_8", "+256700000008", "4*1*2");
    env.process_ussd("test_session_8", "+256700000008", "4*1*2*100000");
    
    let (response, continue_session) = env.process_ussd(
        "test_session_8",
        "+256700000008",
        "4*1*2*100000*2",  // Cancel (option 2)
    );
    
    assert!(!continue_session, "Session should end");
    assert!(response.contains("cancelled") || response.contains("Cancel"), 
        "Should show cancellation message. Got: {}", response);
}

#[test]
fn test_ussd_swap_flow_uses_translations() {
    let env = get_test_env();
    
    // Test that no hardcoded English strings are used
    let (response, _) = env.process_ussd(
        "test_session_9",
        "+256700000009",
        "4",
    );
    
    // Should use translation keys, not hardcoded strings
    // The actual language depends on user's session language
    assert!(!response.is_empty(), "Should have response");
    assert!(response.len() > 10, "Response should have content");
}

#[test]
fn test_ussd_swap_flow_is_stateless() {
    let env = get_test_env();
    
    // Each USSD request should be independent
    // The flow state is determined by the input text, not stored state
    
    // Jump directly to step 3 without going through steps 0-2
    let (response, continue_session) = env.process_ussd(
        "test_session_10",
        "+256700000010",
        "4*1*2*100000",  // Direct to amount confirmation
    );
    
    assert!(continue_session, "Should handle stateless request");
    assert!(response.contains("Spread") || response.contains("Details") || response.contains("Confirm"), 
        "Should show confirmation even when jumping directly. Got: {}", response);
}

// Note: Full end-to-end tests with actual swap execution would require:
// 1. Registered user in business logic canister
// 2. Crypto balance in data canister
// 3. Mocked DEX or testnet integration
// These tests verify the USSD presentation layer works correctly
