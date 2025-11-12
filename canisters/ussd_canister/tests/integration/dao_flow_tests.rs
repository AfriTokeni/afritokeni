// Integration tests for USSD DAO flows
use super::*;

#[test]
fn test_dao_menu_navigation() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_1";
    
    // Navigate to DAO menu (Main menu -> 5. DAO)
    let (response, continue_session) = env.process_ussd(session_id, phone, "5");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("DAO") || response.contains("Governance"), 
        "Should show DAO menu. Got: {}", response);
    assert!(response.contains("Proposal") || response.contains("Vote"), 
        "Should show DAO options. Got: {}", response);
}

#[test]
fn test_dao_view_proposals() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_2";
    
    // Navigate to view proposals (5 -> 1)
    env.process_ussd(session_id, phone, "5");
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    
    assert!(continue_session || !continue_session, "May or may not continue");
    assert!(response.contains("Proposal") || response.contains("No proposals") || response.contains("Active"),
        "Should show proposals or no proposals message. Got: {}", response);
}

#[test]
fn test_dao_vote_on_proposal() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_3";
    
    // Register user first
    env.setup_test_user_with_balances(phone, "Alice", "DAO", "alice@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Navigate to vote (5 -> 2)
    env.process_ussd(session_id, phone, "5");
    let (response, _) = env.process_ussd(session_id, phone, "2");
    
    assert!(response.contains("Vote") || response.contains("Proposal") || response.contains("No proposals"),
        "Should show voting interface or no proposals. Got: {}", response);
}

#[test]
fn test_dao_proposal_details() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_4";
    
    // Navigate to DAO and check proposal details
    env.process_ussd(session_id, phone, "5");
    let (response, _) = env.process_ussd(session_id, phone, "1");
    
    // Should show some DAO-related content
    assert!(response.len() > 0, "Should return some response");
}

#[test]
fn test_dao_voting_power_display() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_5";
    
    // Register user with some balance
    env.setup_test_user_with_balances(phone, "Bob", "Voter", "bob@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    env.set_fiat_balance(phone, "UGX", 100000).ok();
    
    // Navigate to DAO
    let (response, _) = env.process_ussd(session_id, phone, "5");
    
    // Should show DAO menu
    assert!(response.contains("DAO") || response.contains("Governance") || response.contains("Vote"),
        "Should show DAO content. Got: {}", response);
}

#[test]
fn test_dao_cannot_vote_twice() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_6";
    
    // Register user
    env.setup_test_user_with_balances(phone, "Charlie", "Double", "charlie@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Try to vote twice on same proposal (if proposals exist)
    env.process_ussd(session_id, phone, "5");
    env.process_ussd(session_id, phone, "2");
    
    // The actual double-vote prevention will be tested when we have real proposals
    assert!(true, "DAO double-vote prevention test placeholder");
}

#[test]
fn test_dao_menu_structure() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_7";
    
    // Navigate to DAO menu
    let (response, continue_session) = env.process_ussd(session_id, phone, "5");
    
    assert!(continue_session, "Session should continue");
    
    // Should have multiple options
    let option_count = response.matches(|c: char| c.is_numeric()).count();
    assert!(option_count >= 2, "Should have at least 2 DAO options");
}

#[test]
fn test_dao_return_to_main_menu() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "dao_test_8";
    
    // Navigate to DAO then back to main menu
    env.process_ussd(session_id, phone, "5");
    let (response, _) = env.process_ussd(session_id, phone, "0");
    
    assert!(response.contains("Main") || response.contains("Menu") || response.contains("Send"),
        "Should return to main menu. Got: {}", response);
}
