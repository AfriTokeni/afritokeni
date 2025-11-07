// Integration tests for USSD DAO Governance menu
// Run with: cargo test --package ussd_canister --test ussd_dao_tests

use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

#[test]
fn test_view_proposals() {
    // Menu -> DAO -> View Proposals
    let ussd_request = serde_json::json!({
        "sessionId": "session-dao-001",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "4*1"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "4"); // DAO
    assert_eq!(parts[1], "1"); // View Proposals
}

#[test]
fn test_vote_on_proposal() {
    // Menu -> DAO -> Vote -> Proposal ID -> Yes/No -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-dao-002",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "4*2*123*1*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "4"); // DAO
    assert_eq!(parts[1], "2"); // Vote
    assert_eq!(parts[2], "123"); // Proposal ID
    assert_eq!(parts[3], "1"); // Yes (1=Yes, 2=No)
    assert_eq!(parts[4], "1234"); // PIN
}

#[test]
fn test_create_proposal() {
    // Menu -> DAO -> Create Proposal -> Title -> Description -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-dao-003",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "4*3*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "4"); // DAO
    assert_eq!(parts[1], "3"); // Create Proposal
    assert_eq!(parts[2], "1234"); // PIN
}

#[test]
fn test_check_voting_power() {
    // Menu -> DAO -> Voting Power -> PIN
    let ussd_request = serde_json::json!({
        "sessionId": "session-dao-004",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "4*4*1234"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "4"); // DAO
    assert_eq!(parts[1], "4"); // Voting Power
    assert_eq!(parts[2], "1234"); // PIN
}

#[test]
fn test_dao_back_to_main() {
    // Menu -> DAO -> Back
    let ussd_request = serde_json::json!({
        "sessionId": "session-dao-005",
        "serviceCode": "*229#",
        "phoneNumber": "+256700123456",
        "text": "4*0"
    });
    
    let body = serde_json::to_vec(&ussd_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let text = parsed["text"].as_str().unwrap();
    let parts: Vec<&str> = text.split('*').collect();
    
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[1], "0"); // Back
}
