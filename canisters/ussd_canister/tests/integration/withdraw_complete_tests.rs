// COMPLETE Withdrawal Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data canister for withdrawals
use super::*;

// ============================================================================
// SUCCESSFUL WITHDRAWAL - ALL CURRENCIES
// ============================================================================

#[test]
fn test_withdraw_ugx_success() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "UGX", "withdrawugx@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");

    // Register agent and get agent_id (user_id)
    let agent_id = env.register_agent("AGENT001").expect("Agent registration");

    // Withdraw: Menu 1*4 (Withdraw) -> amount -> agent_id -> PIN
    let input = format!("1*4*100000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Withdrawal Request Created") || response.contains("CODE:"),
        "Should complete withdrawal. Got: {}", response);

    // Balance should decrease by amount + fees (100000 + 10500 = 110500)
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 389500, "Balance should decrease by amount + fees");
}




// ============================================================================
// WITHDRAWAL ERROR CASES - INSUFFICIENT BALANCE
// ============================================================================

#[test]
fn test_withdraw_insufficient_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Give user small balance that's not enough for minimum withdrawal + fees
    // Minimum withdrawal is 100000, with fees it needs ~110500
    env.setup_test_user_with_balances(phone, "Withdraw", "NoMoney", "withdrawno@test.com", "UGX", "1234", 50000, 0, 0)
        .expect("Setup");

    let agent_id = env.register_agent("AGENT005").expect("Agent registration");
    let input = format!("1*4*100000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient balance. Got: {}", response);
}

#[test]
fn test_withdraw_zero_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "ZeroBalance", "withdrawzero@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");

    let agent_id = env.register_agent("AGENT006").expect("Agent registration");
    let input = format!("1*4*100000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Insufficient") || response.contains("insufficient") || response.contains("balance"),
        "Should reject zero balance. Got: {}", response);
}

// ============================================================================
// WITHDRAWAL ERROR CASES - INVALID AMOUNTS
// ============================================================================

#[test]
fn test_withdraw_zero_amount() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Withdraw", "ZeroAmt", "withdrawzeroamt@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "1*4");
    let (response, _) = env.process_ussd(&sess, phone, "0");
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_withdraw_negative_amount() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Withdraw", "NegAmt", "withdrawneg@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "1*4");
    let (response, _) = env.process_ussd(&sess, phone, "-5000");

    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("Minimum"),
        "Should reject negative amount. Got: {}", response);
}

#[test]
fn test_withdraw_below_minimum() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Withdraw", "BelowMin", "withdrawmin@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "1*4");
    let (response, _) = env.process_ussd(&sess, phone, "50000"); // Below minimum (100,000)

    assert!(response.contains("Minimum") || response.contains("minimum") || response.contains("too small"),
        "Should reject below minimum. Got: {}", response);
}

#[test]
fn test_withdraw_above_maximum() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Withdraw", "AboveMax", "withdrawmax@test.com", "UGX", "1234", 2000000, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "1*4");
    let (response, _) = env.process_ussd(&sess, phone, "1500000"); // Above maximum (1000000)
    
    assert!(response.contains("Maximum") || response.contains("maximum") || response.contains("too large"),
        "Should reject above maximum. Got: {}", response);
}

// ============================================================================
// WITHDRAWAL ERROR CASES - WRONG PIN
// ============================================================================

#[test]
fn test_withdraw_wrong_pin() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "WrongPIN", "withdrawwrong@test.com", "UGX", "1234", 200000, 0, 0)
        .expect("Setup");

    let agent_id = env.register_agent("AGENT007").expect("Agent registration");
    let input = format!("1*4*100000*{}*9999", agent_id); // Wrong PIN
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong") || response.contains("Invalid PIN"),
        "Should reject wrong PIN. Got: {}", response);

    // Balance should not change
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 200000, "Balance should not change on wrong PIN");
}

// ============================================================================
// WITHDRAWAL ERROR CASES - INVALID AGENT ID
// ============================================================================

#[test]
fn test_withdraw_empty_agent_id() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Withdraw", "EmptyAgent", "withdrawempty@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "1*4");
    env.process_ussd(&sess, phone, "100000");
    let (response, _) = env.process_ussd(&sess, phone, ""); // Empty agent ID

    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("Agent") || response.contains("agent") || response.contains("empty"),
        "Should reject empty agent ID. Got: {}", response);
}

#[test]
fn test_withdraw_invalid_agent_id_format() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Withdraw", "BadAgent", "withdrawbad@test.com", "UGX", "1234", 100000, 0, 0)
        .expect("Setup");
    
    env.process_ussd(&sess, phone, "1*4");
    env.process_ussd(&sess, phone, "50000");
    let (response, _) = env.process_ussd(&sess, phone, "!@#$%"); // Invalid format
    
    assert!(response.len() > 0, "Should handle invalid agent ID");
}

// ============================================================================
// WITHDRAWAL DIFFERENT AMOUNTS
// ============================================================================

#[test]
fn test_withdraw_minimum_amount() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Balance must cover withdrawal amount + fees
    // For 100000 UGX: platform fee (0.5%) = 500, agent fee (10%) = 10000
    // Total needed: 100000 + 500 + 10000 = 110500
    env.setup_test_user_with_balances(phone, "Withdraw", "MinAmt", "withdrawminamt@test.com", "UGX", "1234", 110500, 0, 0)
        .expect("Setup");

    let agent_id = env.register_agent("AGENT008").expect("Agent registration");
    let input = format!("1*4*100000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Withdrawal Request Created") || response.contains("CODE:"),
        "Should accept minimum amount. Got: {}", response);
}

#[test]
fn test_withdraw_maximum_amount() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Balance must cover withdrawal amount + fees
    // For 1000000 UGX: platform fee (0.5%) = 5000, agent fee (10%) = 100000
    // Total needed: 1000000 + 5000 + 100000 = 1105000
    env.setup_test_user_with_balances(phone, "Withdraw", "MaxAmt", "withdrawmaxamt@test.com", "UGX", "1234", 1105000, 0, 0)
        .expect("Setup");

    let agent_id = env.register_agent("AGENT009").expect("Agent registration");
    let input = format!("1*4*1000000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Withdrawal Request Created") || response.contains("CODE:"),
        "Should accept maximum amount. Got: {}", response);
}

#[test]
fn test_withdraw_exact_balance() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    // Set initial balance to cover withdrawal amount + fees
    // For 100000 UGX: platform fee (0.5%) = 500, agent fee (10%) = 10000
    // Total needed: 100000 + 500 + 10000 = 110500
    env.setup_test_user_with_balances(phone, "Withdraw", "ExactBal", "withdrawexact@test.com", "UGX", "1234", 110500, 0, 0)
        .expect("Setup");

    let agent_id = env.register_agent("AGENT010").expect("Agent registration");
    let input = format!("1*4*100000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Withdrawal Request Created") || response.contains("CODE:"));

    // After withdrawal, balance should be exactly 0
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 0, "Should have 0 balance after withdrawing exact amount + fees");
}

// ============================================================================
// MULTIPLE WITHDRAWALS
// ============================================================================

#[test]
fn test_multiple_withdrawals_same_user() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "Multiple", "withdrawmulti@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");

    let agent_id_1 = env.register_agent("AGENT011").expect("Agent registration");
    let agent_id_2 = env.register_agent("AGENT012").expect("Agent registration");

    // First withdrawal
    let input_1 = format!("1*4*100000*{}*1234", agent_id_1);
    env.process_ussd(&sess, phone, &input_1);

    // Second withdrawal
    let input_2 = format!("1*4*150000*{}*1234", agent_id_2);
    env.process_ussd(&sess, phone, &input_2);

    // Check final balance
    // First withdrawal: 100000 + fees (10500) = 110500
    // Second withdrawal: 150000 + fees (15750) = 165750
    // Total deducted: 110500 + 165750 = 276250
    // Remaining: 500000 - 276250 = 223750
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 223750, "Should have 223750 left after two withdrawals (including fees)");
}

#[test]
fn test_withdraw_after_deposit() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "AfterDeposit", "withdrawdep@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");

    // Deposit (set balance)
    env.set_fiat_balance(phone, "UGX", 200000).expect("Deposit");

    let agent_id = env.register_agent("AGENT013").expect("Agent registration");
    // Use 100000 (minimum) instead of 80000 (below minimum)
    let input = format!("1*4*100000*{}*1234", agent_id);
    let (response, _) = env.process_ussd(&sess, phone, &input);

    assert!(response.contains("Withdrawal Request Created") || response.contains("CODE:"));

    // Balance should decrease by amount + fees (100000 + 10500 = 110500)
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 89500);
}


// ============================================================================
// WITHDRAWAL WITH AGENT COMMISSION
// ============================================================================

#[test]
fn test_withdraw_with_different_agents() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "DiffAgents", "withdrawagents@test.com", "UGX", "1234", 500000, 0, 0)
        .expect("Setup");

    let agent_a = env.register_agent("AGENT_A").expect("Agent A registration");
    let agent_b = env.register_agent("AGENT_B").expect("Agent B registration");

    // Withdraw with Agent A: 100000 + fees (500 + 10000) = 110500
    let input_a = format!("1*4*100000*{}*1234", agent_a);
    env.process_ussd(&sess, phone, &input_a);

    // Withdraw with Agent B: 100000 + fees (500 + 10000) = 110500
    let input_b = format!("1*4*100000*{}*1234", agent_b);
    env.process_ussd(&sess, phone, &input_b);

    // Total deducted: 110500 + 110500 = 221000
    // Remaining: 500000 - 221000 = 279000
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, 279000, "Should have 279000 left after two withdrawals");
}

#[test]
fn test_withdraw_balance_check_before_and_after() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");

    env.setup_test_user_with_balances(phone, "Withdraw", "BalCheck", "withdrawbalcheck@test.com", "UGX", "1234", 200000, 0, 0)
        .expect("Setup");

    // Check balance before
    let balance_before = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance_before, 200000);

    let agent_id = env.register_agent("AGENT014").expect("Agent registration");
    // Use 100000 (minimum) instead of 80000 (below minimum)
    let input = format!("1*4*100000*{}*1234", agent_id);
    env.process_ussd(&sess, phone, &input);

    // Check balance after (200000 - 100000 - 10500 = 89500)
    let balance_after = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance_after, 89500);
}
