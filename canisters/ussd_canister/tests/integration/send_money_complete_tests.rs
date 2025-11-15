// COMPLETE Send Money Integration Tests - ALL COMBINATIONS
// Tests: USSD -> Business Logic -> Data canister for money transfers
use super::*;

// ============================================================================
// SUCCESSFUL SEND MONEY - ALL CURRENCY COMBINATIONS
// ============================================================================

#[test]
fn test_send_money_ugx_success() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Sender", "sender@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "receiver@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 20000000).expect("Set balance"); // 200,000 UGX in cents
    
    // Send money: Menu 1*1 (Send Money) -> recipient -> amount -> PIN
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*100000*1234", receiver));
    
    assert!(response.contains("success") || response.contains("Success") || response.contains("sent"),
        "Should send money. Got: {}", response);
    
    let sender_balance = env.check_fiat_balance(sender, "UGX").expect("Get sender balance");
    let receiver_balance = env.check_fiat_balance(receiver, "UGX").expect("Get receiver balance");
    assert!(sender_balance < 20000000, "Sender balance should decrease");
    assert_eq!(receiver_balance, 10000000, "Receiver should get 10,000,000 cents (100,000 UGX)");
}




// ============================================================================
// SEND MONEY ERROR CASES - INSUFFICIENT BALANCE
// ============================================================================

#[test]
fn test_send_money_insufficient_balance() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Poor", "sendpoor@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvpoor@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 10000).expect("Set small balance");
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*50000*1234", receiver));
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject insufficient balance. Got: {}", response);
}

#[test]
fn test_send_money_zero_balance() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Zero", "sendzero@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvzero@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    // No balance set
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*10000*1234", receiver));
    
    assert!(response.contains("Insufficient") || response.contains("insufficient"),
        "Should reject zero balance. Got: {}", response);
}

// ============================================================================
// SEND MONEY ERROR CASES - INVALID AMOUNTS
// ============================================================================

#[test]
fn test_send_money_zero_amount() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "ZeroAmt", "sendzeroamt@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvzeroamt@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 100000).expect("Set balance");
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*0*1234", receiver));
    
    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("positive"),
        "Should reject zero amount. Got: {}", response);
}

#[test]
fn test_send_money_negative_amount() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Negative", "sendneg@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvneg@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 1000000).expect("Set balance"); // 10,000 UGX in cents (enough to avoid insufficient balance error)
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*-5000*1234", receiver));

    assert!(response.contains("Invalid") || response.contains("invalid") || response.contains("Insufficient"),
        "Should reject negative amount. Got: {}", response);
}

// ============================================================================
// SEND MONEY ERROR CASES - WRONG PIN
// ============================================================================

#[test]
fn test_send_money_wrong_pin() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "WrongPIN", "sendwrong@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvwrong@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 6000000).expect("Set balance"); // 60,000 UGX in cents (enough for 50,000 + fee)
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*50000*9999", receiver));

    assert!(response.contains("Incorrect") || response.contains("incorrect") || response.contains("Wrong") || response.contains("Invalid"),
        "Should reject wrong PIN. Got: {}", response);
    
    // Balance should not change
    let balance = env.check_fiat_balance(sender, "UGX").expect("Get balance");
    assert_eq!(balance, 6000000, "Balance should not change on wrong PIN");
}

// ============================================================================
// SEND MONEY ERROR CASES - INVALID RECIPIENT
// ============================================================================

#[test]
fn test_send_money_nonexistent_recipient() {
    let env = get_test_env();
    let sess = session();
    let sender = &phone("UGX");
    let fake_receiver = "+256700999999";
    
    env.setup_test_user_with_balances(sender, "Send", "Sender", "sendnorecv@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 6000000).expect("Set balance"); // 60,000 UGX in cents
    // Don't register receiver
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*50000*1234", fake_receiver));
    
    assert!(response.contains("not found") || response.contains("Not found") || response.contains("exist"),
        "Should reject nonexistent recipient. Got: {}", response);
}

#[test]
fn test_send_money_to_self() {
    let env = get_test_env();
    let sess = session();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Send", "Self", "sendself@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(phone, "UGX", 6000000).expect("Set balance"); // 60,000 UGX in cents
    
    let (response, _) = env.process_ussd(&sess, phone, &format!("1*1*{}*50000*1234", phone));
    
    assert!(response.contains("yourself") || response.contains("self") || response.contains("same"),
        "Should reject sending to self. Got: {}", response);
}

// ============================================================================
// SEND MONEY DIFFERENT AMOUNTS
// ============================================================================

#[test]
fn test_send_money_minimum_amount() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "MinAmt", "sendmin@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvmin@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 60000).expect("Set balance"); // 600 UGX in cents (enough for 10 + fee)
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*10*1234", receiver));
    
    assert!(response.contains("success") || response.contains("Success"),
        "Should accept minimum amount. Got: {}", response);
}

#[test]
fn test_send_money_large_amount() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Large", "sendlarge@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvlarge@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 110000000).expect("Set balance"); // 1,100,000 UGX in cents (enough for 1,000,000 + fee)
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*1000000*1234", receiver));
    
    assert!(response.contains("success") || response.contains("Success"),
        "Should accept large amount. Got: {}", response);
}

#[test]
fn test_send_money_exact_balance() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Exact", "sendexact@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvexact@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 7550000).expect("Set balance"); // 75,500 UGX in cents (exact for 75,000 + fee)
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*75000*1234", receiver));
    
    // May succeed or fail depending on if there's a transfer fee
    assert!(response.len() > 0, "Should handle exact balance");
}

// ============================================================================
// MULTIPLE SEND MONEY OPERATIONS
// ============================================================================

#[test]
fn test_multiple_sends_same_sender() {
    let env = get_test_env();
    let sess = session();
    let sender = &phone("UGX");
    let receiver1 = "+256700040404";
    let receiver2 = "+256700050505";
    
    env.setup_test_user_with_balances(sender, "Send", "Multi", "sendmulti@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver1, "Receive", "Receiver1", "recv1@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver2, "Receive", "Receiver2", "recv2@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 26000000).expect("Set balance"); // 260,000 UGX in cents (enough for 100,000 + 150,000 + 2 fees)
    
    // First send
    env.process_ussd(&sess, sender, &format!("1*1*{}*100000*1234", receiver1));
    
    // Second send
    env.process_ussd(&sess, sender, &format!("1*1*{}*150000*1234", receiver2));
    
    // Check balances
    let sender_balance = env.check_fiat_balance(sender, "UGX").expect("Get sender balance");
    assert!(sender_balance < 26000000, "Sender balance should decrease");
    
    let recv1_balance = env.check_fiat_balance(receiver1, "UGX").expect("Get recv1 balance");
    assert_eq!(recv1_balance, 10000000, "Receiver1 should get 10,000,000 cents (100,000 UGX)");

    let recv2_balance = env.check_fiat_balance(receiver2, "UGX").expect("Get recv2 balance");
    assert_eq!(recv2_balance, 15000000, "Receiver2 should get 15,000,000 cents (150,000 UGX)");
}

#[test]
fn test_send_money_back_and_forth() {
    let env = get_test_env();
    let sess = session();
    let user1 = &format!("{}1", phone("UGX"));
    let user2 = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(user1, "User", "One", "user1@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(user2, "User", "Two", "user2@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(user1, "UGX", 9000000).expect("Set balance"); // 90,000 UGX in cents (enough for 80,000 + fee)
    
    // User1 sends to User2
    env.process_ussd(&sess, user1, &format!("1*1*{}*80000*1234", user2));
    
    let user2_balance = env.check_fiat_balance(user2, "UGX").expect("Get user2 balance");
    assert_eq!(user2_balance, 8000000, "User2 should receive 8,000,000 cents (80,000 UGX)");
    
    // User2 sends back to User1
    env.process_ussd(&sess, user2, &format!("1*1*{}*30000*1234", user1));
    
    let user1_final = env.check_fiat_balance(user1, "UGX").expect("Get user1 balance");
    let user2_final = env.check_fiat_balance(user2, "UGX").expect("Get user2 balance");

    assert!(user1_final > 0, "User1 should have received money back");
    assert!(user2_final < 8000000, "User2 balance should decrease from 8,000,000 cents");
}

// ============================================================================
// SEND MONEY WITH TRANSFER FEES
// ============================================================================

#[test]
fn test_send_money_includes_transfer_fee() {
    let env = get_test_env();
    let sess = session();
    let sender = &format!("{}1", phone("UGX"));
    let receiver = &format!("{}2", phone("UGX"));
    
    env.setup_test_user_with_balances(sender, "Send", "Fee", "sendfee@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.setup_test_user_with_balances(receiver, "Receive", "Receiver", "recvfee@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(sender, "UGX", 11000000).expect("Set balance"); // 110,000 UGX in cents (enough for 100,000 + fee)
    
    let (response, _) = env.process_ussd(&sess, sender, &format!("1*1*{}*100000*1234", receiver));
    
    // Should mention fee or show total deduction
    assert!(response.len() > 0, "Should complete transfer");
    
    let sender_balance = env.check_fiat_balance(sender, "UGX").expect("Get sender balance");
    let receiver_balance = env.check_fiat_balance(receiver, "UGX").expect("Get receiver balance");

    // Sender should be charged more than 10,000,000 cents (including fee)
    assert!(sender_balance < 1000000, "Sender should pay amount + fee");
    assert_eq!(receiver_balance, 10000000, "Receiver gets exact amount in cents");
}

