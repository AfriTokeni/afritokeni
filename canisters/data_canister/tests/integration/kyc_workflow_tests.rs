/// Integration tests for KYC workflow
///
/// Tests the complete KYC lifecycle:
/// - User created with NotStarted status
/// - Transition to Pending when KYC submitted
/// - Transition to Approved/Rejected after review
/// - Verify KYC status persists across operations
///
/// CRITICAL for compliance and regulatory requirements.

use candid::{encode_one, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::{CreateUserRequest, KYCStatus};

fn get_data_canister_wasm() -> Vec<u8> {
    let wasm_path = std::env::var("DATA_CANISTER_WASM")
        .unwrap_or_else(|_| {
            let mut path = std::env::current_dir().unwrap();
            path.push("target/wasm32-unknown-unknown/release/data_canister.wasm");
            path.to_string_lossy().to_string()
        });

    std::fs::read(&wasm_path)
        .unwrap_or_else(|e| panic!("Failed to read WASM from {}: {}", wasm_path, e))
}

fn setup_canister() -> (PocketIc, Principal) {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(None::<(Option<String>, Option<String>)>).unwrap(),
        None,
    );

    (pic, canister_id)
}

fn create_test_user(pic: &PocketIc, canister_id: Principal, phone: &str, email: &str) -> String {
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: Some(email.to_string()),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some(phone.to_string()),
    };

    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        encode_one(user_request).unwrap(),
    ).expect("User creation failed");

    let user: Result<shared_types::User, String> = decode_one(&result).unwrap();
    user.unwrap().id
}

#[test]
fn test_new_user_has_not_started_kyc_status() {
    let (pic, canister_id) = setup_canister();

    let user_id = create_test_user(&pic, canister_id, "+256700000001", "test1@example.com");

    // Get user and verify KYC status
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
    let user = user.unwrap().unwrap();

    assert_eq!(user.kyc_status, KYCStatus::NotStarted,
        "New user should have NotStarted KYC status");
}

#[test]
fn test_kyc_status_transition_to_pending() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000002", "test2@example.com");

    // Update KYC status to Pending
    let update_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Pending)).unwrap(),
    );

    assert!(update_result.is_ok(), "Failed to update KYC status to Pending");

    // Verify status changed
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
    let user = user.unwrap().unwrap();

    assert_eq!(user.kyc_status, KYCStatus::Pending,
        "KYC status should be Pending after update");
}

#[test]
fn test_kyc_status_transition_to_approved() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000003", "test3@example.com");

    // Transition: NotStarted → Pending → Approved
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Pending)).unwrap(),
    ).unwrap();

    let update_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Approved)).unwrap(),
    );

    assert!(update_result.is_ok(), "Failed to approve KYC");

    // Verify approved status
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
    let user = user.unwrap().unwrap();

    assert_eq!(user.kyc_status, KYCStatus::Approved,
        "KYC status should be Approved after approval");
}

#[test]
fn test_kyc_status_transition_to_rejected() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000004", "test4@example.com");

    // Transition: NotStarted → Pending → Rejected
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Pending)).unwrap(),
    ).unwrap();

    let update_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Rejected)).unwrap(),
    );

    assert!(update_result.is_ok(), "Failed to reject KYC");

    // Verify rejected status
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
    let user = user.unwrap().unwrap();

    assert_eq!(user.kyc_status, KYCStatus::Rejected,
        "KYC status should be Rejected after rejection");
}

#[test]
fn test_kyc_status_can_be_resubmitted_after_rejection() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000005", "test5@example.com");

    // Reject initially
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Pending)).unwrap(),
    ).unwrap();

    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Rejected)).unwrap(),
    ).unwrap();

    // Resubmit KYC
    let resubmit_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Pending)).unwrap(),
    );

    assert!(resubmit_result.is_ok(), "Should be able to resubmit KYC after rejection");

    // Verify back to Pending
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
    let user = user.unwrap().unwrap();

    assert_eq!(user.kyc_status, KYCStatus::Pending,
        "KYC status should be Pending after resubmission");
}

#[test]
fn test_kyc_status_for_multiple_users() {
    let (pic, canister_id) = setup_canister();

    // Create multiple users with different KYC statuses
    let user1_id = create_test_user(&pic, canister_id, "+256700000006", "user1@example.com");
    let user2_id = create_test_user(&pic, canister_id, "+256700000007", "user2@example.com");
    let user3_id = create_test_user(&pic, canister_id, "+256700000008", "user3@example.com");

    // User 1: Approved
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user1_id.clone(), KYCStatus::Approved)).unwrap(),
    ).unwrap();

    // User 2: Pending
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user2_id.clone(), KYCStatus::Pending)).unwrap(),
    ).unwrap();

    // User 3: Rejected
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user3_id.clone(), KYCStatus::Rejected)).unwrap(),
    ).unwrap();

    // Verify all statuses are independent
    let user1 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user1_id).unwrap(),
    ).unwrap();
    let user1: Result<Option<shared_types::User>, String> = decode_one(&user1).unwrap();
    assert_eq!(user1.unwrap().unwrap().kyc_status, KYCStatus::Approved);

    let user2 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user2_id).unwrap(),
    ).unwrap();
    let user2: Result<Option<shared_types::User>, String> = decode_one(&user2).unwrap();
    assert_eq!(user2.unwrap().unwrap().kyc_status, KYCStatus::Pending);

    let user3 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user3_id).unwrap(),
    ).unwrap();
    let user3: Result<Option<shared_types::User>, String> = decode_one(&user3).unwrap();
    assert_eq!(user3.unwrap().unwrap().kyc_status, KYCStatus::Rejected);
}

#[test]
fn test_kyc_status_persists_across_multiple_queries() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000009", "test9@example.com");

    // Set to Approved
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Approved)).unwrap(),
    ).unwrap();

    // Query multiple times to ensure consistency
    for _ in 0..5 {
        let get_result = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_user",
            encode_one(user_id.clone()).unwrap(),
        ).unwrap();

        let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
        let user = user.unwrap().unwrap();

        assert_eq!(user.kyc_status, KYCStatus::Approved,
            "KYC status should persist across queries");
    }
}

#[test]
fn test_kyc_update_for_nonexistent_user() {
    let (pic, canister_id) = setup_canister();

    let update_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one(("nonexistent_user".to_string(), KYCStatus::Approved)).unwrap(),
    );

    // Should fail
    if let Ok(response) = update_result {
        let result: Result<(), String> = decode_one(&response).unwrap();
        assert!(result.is_err(), "Should fail to update KYC for nonexistent user");
    }
}

#[test]
fn test_kyc_workflow_with_balance_operations() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000010", "test10@example.com");

    // Set balance before KYC approval
    let set_balance = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "set_fiat_balance",
        encode_one((user_id.clone(), "UGX".to_string(), 100000u64)).unwrap(),
    );
    assert!(set_balance.is_ok());

    // Approve KYC
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Approved)).unwrap(),
    ).unwrap();

    // Verify user has both balance and KYC approved
    let get_user = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id.clone()).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_user).unwrap();
    let user = user.unwrap().unwrap();
    assert_eq!(user.kyc_status, KYCStatus::Approved);

    let get_balance = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_one((user_id, shared_types::FiatCurrency::UGX)).unwrap(),
    ).unwrap();

    let balance: Result<u64, String> = decode_one(&get_balance).unwrap();
    assert_eq!(balance.unwrap(), 100000);
}

#[test]
fn test_agent_user_kyc_workflow() {
    let (pic, canister_id) = setup_canister();

    // Create agent user
    let agent_request = CreateUserRequest {
        user_type_str: "Agent".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: Some("agent@example.com".to_string()),
        first_name: "Agent".to_string(),
        last_name: "Smith".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000011".to_string()),
    };

    let create_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        encode_one(agent_request).unwrap(),
    ).unwrap();

    let agent: Result<shared_types::User, String> = decode_one(&create_result).unwrap();
    let agent_id = agent.unwrap().id;

    // Agents should also go through KYC
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((agent_id.clone(), KYCStatus::Pending)).unwrap(),
    ).unwrap();

    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((agent_id.clone(), KYCStatus::Approved)).unwrap(),
    ).unwrap();

    // Verify agent KYC approved
    let get_agent = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(agent_id).unwrap(),
    ).unwrap();

    let agent: Result<Option<shared_types::User>, String> = decode_one(&get_agent).unwrap();
    let agent = agent.unwrap().unwrap();
    assert_eq!(agent.kyc_status, KYCStatus::Approved, "Agent should have approved KYC");
}

#[test]
fn test_kyc_status_direct_transition_not_started_to_approved() {
    let (pic, canister_id) = setup_canister();
    let user_id = create_test_user(&pic, canister_id, "+256700000012", "test12@example.com");

    // Jump directly from NotStarted to Approved (admin override scenario)
    let update_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_one((user_id.clone(), KYCStatus::Approved)).unwrap(),
    );

    assert!(update_result.is_ok(), "Should allow direct transition to Approved");

    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id).unwrap(),
    ).unwrap();

    let user: Result<Option<shared_types::User>, String> = decode_one(&get_result).unwrap();
    let user = user.unwrap().unwrap();

    assert_eq!(user.kyc_status, KYCStatus::Approved,
        "Direct transition to Approved should work");
}
