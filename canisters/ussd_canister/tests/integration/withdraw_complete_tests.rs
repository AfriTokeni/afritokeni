// COMPLETE Withdrawal Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data canister for withdrawals
use super::*;

// ============================================================================
// SUCCESSFUL WITHDRAWAL - ALL CURRENCIES
// ============================================================================

#[test]
fn test_withdraw_ugx_success() {
    let env = get_test_env();
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "Withdraw", "UGX", "withdrawugx@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    // Withdraw: Menu 1*4 (Withdraw) -> amount -> agent_id -> PIN
    let (response, _) = env.process_ussd("session", phone, "1*4*100000*AGENT001*1234");
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("withdraw"),
        "Should complete withdrawal. Got: {}", response);
    
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 400000, "Balance should decrease");
}

#[test]
fn test_withdraw_kes_success() {
    let env = get_test_env();
    let phone = "+254700222222";
    
    env.register_user_direct(phone, "Withdraw", "KES", "withdrawkes@test.com", "KES", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "KES", 300000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*50000*AGENT002*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
    
    let balance = env.check_fiat_balance(phone, "KES").expect("Get balance");
    assert_eq!(balance, 250000);
}

#[test]
fn test_withdraw_tzs_success() {
    let env = get_test_env();
    let phone = "+255700333333";
    
    env.register_user_direct(phone, "Withdraw", "TZS", "withdrawtzs@test.com", "TZS", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "TZS", 1000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*200000*AGENT003*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
}

#[test]
fn test_withdraw_ngn_success() {
    let env = get_test_env();
    let phone = "+234700444444";
    
    env.register_user_direct(phone, "Withdraw", "NGN", "withdrawngn@test.com", "NGN", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "NGN", 2000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*500000*AGENT004*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
}

// ============================================================================
// WITHDRAWAL ERROR CASES - INSUFFICIENT BALANCE
// ============================================================================

#[test]
fn test_withdraw_insufficient_balance() {
    let env = get_test_env();
    let phone = "+256700555555";
    
    env.register_user_direct(phone, "Withdraw", "NoMoney", "withdrawno@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 10000).expect("Set small balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*50000*AGENT005*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient balance. Got: {}", response);
}

#[test]
fn test_withdraw_zero_balance() {
    let env = get_test_env();
    let phone = "+256700666666";
    
    env.register_user_direct(phone, "Withdraw", "ZeroBalance", "withdrawzero@test.com", "UGX", "1234")
        .expect("Registration");
    // No balance set
    
    let (response, _) = env.process_ussd("session", phone, "1*4*10000*AGENT006*1234");
    
    assert!(response.contains("Insufficient") || response.contains("insufficient") || response.contains("balance"),
        "Should reject zero balance. Got: {}", response);
}

// ============================================================================
// WITHDRAWAL ERROR CASES - INVALID AMOUNTS
// ============================================================================

#[test]
fn test_withdraw_zero_amount() {
    let env = get_test_env();
    let phone = "+256700777777";
    
    env.register_user_direct(phone, "Withdraw", "ZeroAmt", "withdrawzeroamt@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    env.process_ussd("session", phone, "1*4");
    let (response, _) = env.process_ussd("session", phone, "0");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_withdraw_negative_amount() {
    let env = get_test_env();
    let phone = "+256700888888";
    
    env.register_user_direct(phone, "Withdraw", "NegAmt", "withdrawneg@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    env.process_ussd("session", phone, "1*4");
    let (response, _) = env.process_ussd("session", phone, "-5000");
    
    assert!(response.contains("Invalid") || response.contains("invalid"),
        "Should reject negative amount. Got: {}", response);
}

#[test]
fn test_withdraw_below_minimum() {
    let env = get_test_env();
    let phone = "+256700999999";
    
    env.register_user_direct(phone, "Withdraw", "BelowMin", "withdrawmin@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    env.process_ussd("session", phone, "1*4");
    let (response, _) = env.process_ussd("session", phone, "5"); // Below minimum (10)
    
    assert!(response.contains("Minimum") || response.contains("minimum") || response.contains("too small"),
        "Should reject below minimum. Got: {}", response);
}

#[test]
fn test_withdraw_above_maximum() {
    let env = get_test_env();
    let phone = "+256700101010";
    
    env.register_user_direct(phone, "Withdraw", "AboveMax", "withdrawmax@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 2000000).expect("Set large balance");
    
    env.process_ussd("session", phone, "1*4");
    let (response, _) = env.process_ussd("session", phone, "1500000"); // Above maximum (1000000)
    
    assert!(response.contains("Maximum") || response.contains("maximum") || response.contains("too large"),
        "Should reject above maximum. Got: {}", response);
}

// ============================================================================
// WITHDRAWAL ERROR CASES - WRONG PIN
// ============================================================================

#[test]
fn test_withdraw_wrong_pin() {
    let env = get_test_env();
    let phone = "+256700202020";
    
    env.register_user_direct(phone, "Withdraw", "WrongPIN", "withdrawwrong@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*50000*AGENT007*9999"); // Wrong PIN
    
    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong"),
        "Should reject wrong PIN. Got: {}", response);
    
    // Balance should not change
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 100000, "Balance should not change on wrong PIN");
}

// ============================================================================
// WITHDRAWAL ERROR CASES - INVALID AGENT ID
// ============================================================================

#[test]
fn test_withdraw_empty_agent_id() {
    let env = get_test_env();
    let phone = "+256700303030";
    
    env.register_user_direct(phone, "Withdraw", "EmptyAgent", "withdrawempty@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    env.process_ussd("session", phone, "1*4");
    env.process_ussd("session", phone, "50000");
    let (response, _) = env.process_ussd("session", phone, ""); // Empty agent ID
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("Agent") || response.contains("agent"),
        "Should reject empty agent ID. Got: {}", response);
}

#[test]
fn test_withdraw_invalid_agent_id_format() {
    let env = get_test_env();
    let phone = "+256700404040";
    
    env.register_user_direct(phone, "Withdraw", "BadAgent", "withdrawbad@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    env.process_ussd("session", phone, "1*4");
    env.process_ussd("session", phone, "50000");
    let (response, _) = env.process_ussd("session", phone, "!@#$%"); // Invalid format
    
    assert!(response.len() > 0, "Should handle invalid agent ID");
}

// ============================================================================
// WITHDRAWAL DIFFERENT AMOUNTS
// ============================================================================

#[test]
fn test_withdraw_minimum_amount() {
    let env = get_test_env();
    let phone = "+256700505050";
    
    env.register_user_direct(phone, "Withdraw", "MinAmt", "withdrawminamt@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 10000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*10*AGENT008*1234");
    
    assert!(response.contains("success") || response.contains("Success"),
        "Should accept minimum amount. Got: {}", response);
}

#[test]
fn test_withdraw_maximum_amount() {
    let env = get_test_env();
    let phone = "+256700606060";
    
    env.register_user_direct(phone, "Withdraw", "MaxAmt", "withdrawmaxamt@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 1000000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*1000000*AGENT009*1234");
    
    assert!(response.contains("success") || response.contains("Success"),
        "Should accept maximum amount. Got: {}", response);
}

#[test]
fn test_withdraw_exact_balance() {
    let env = get_test_env();
    let phone = "+256700707070";
    
    env.register_user_direct(phone, "Withdraw", "ExactBal", "withdrawexact@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 75000).expect("Set balance");
    
    let (response, _) = env.process_ussd("session", phone, "1*4*75000*AGENT010*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
    
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 0, "Should have 0 balance");
}

// ============================================================================
// MULTIPLE WITHDRAWALS
// ============================================================================

#[test]
fn test_multiple_withdrawals_same_user() {
    let env = get_test_env();
    let phone = "+256700808080";
    
    env.register_user_direct(phone, "Withdraw", "Multiple", "withdrawmulti@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    // First withdrawal
    env.process_ussd("s1", phone, "1*4*100000*AGENT011*1234");
    
    // Second withdrawal
    env.process_ussd("s2", phone, "1*4*150000*AGENT012*1234");
    
    // Check final balance
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 250000, "Should have 250000 left after two withdrawals");
}

#[test]
fn test_withdraw_after_deposit() {
    let env = get_test_env();
    let phone = "+256700909090";
    
    env.register_user_direct(phone, "Withdraw", "AfterDeposit", "withdrawdep@test.com", "UGX", "1234")
        .expect("Registration");
    
    // Deposit (set balance)
    env.set_fiat_balance(phone, "UGX", 200000).expect("Deposit");
    
    // Withdraw
    let (response, _) = env.process_ussd("session", phone, "1*4*80000*AGENT013*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
    
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 120000);
}

// ============================================================================
// WITHDRAWAL MENU NAVIGATION
// ============================================================================

#[test]
fn test_withdraw_menu_navigation() {
    let env = get_test_env();
    let phone = "+256700010101";
    
    env.register_user_direct(phone, "Withdraw", "Nav", "withdrawnav@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    let (response, continue_session) = env.process_ussd("session", phone, "1*4");
    
    assert!(continue_session, "Should continue session");
    assert!(response.contains("amount") || response.contains("Amount") || response.contains("withdraw"),
        "Should ask for amount. Got: {}", response);
}

#[test]
fn test_withdraw_return_to_main_menu() {
    let env = get_test_env();
    let phone = "+256700020202";
    
    env.register_user_direct(phone, "Withdraw", "Return", "withdrawreturn@test.com", "UGX", "1234")
        .expect("Registration");
    
    env.process_ussd("session", phone, "1*4");
    let (response, _) = env.process_ussd("session", phone, "0"); // Back
    
    assert!(response.contains("Main") || response.contains("Menu") || response.contains("Send"),
        "Should return to main menu. Got: {}", response);
}

// ============================================================================
// WITHDRAWAL WITH AGENT COMMISSION
// ============================================================================

#[test]
fn test_withdraw_with_different_agents() {
    let env = get_test_env();
    let phone = "+256700030303";
    
    env.register_user_direct(phone, "Withdraw", "DiffAgents", "withdrawagents@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 500000).expect("Set balance");
    
    // Withdraw with Agent A
    env.process_ussd("s1", phone, "1*4*100000*AGENT_A*1234");
    
    // Withdraw with Agent B
    env.process_ussd("s2", phone, "1*4*100000*AGENT_B*1234");
    
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 300000, "Should have 300000 left");
}

#[test]
fn test_withdraw_balance_check_before_and_after() {
    let env = get_test_env();
    let phone = "+256700040404";
    
    env.register_user_direct(phone, "Withdraw", "BalCheck", "withdrawbalcheck@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 200000).expect("Set balance");
    
    // Check balance before
    let balance_before = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance_before, 200000);
    
    // Withdraw
    env.process_ussd("session", phone, "1*4*80000*AGENT014*1234");
    
    // Check balance after
    let balance_after = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance_after, 120000);
}
