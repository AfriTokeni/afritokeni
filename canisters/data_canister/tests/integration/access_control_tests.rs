/// Integration tests for access control enforcement
///
/// Tests the 3-tier access control system:
/// - Controller (admin - dfx controller)
/// - AuthorizedCanister (USSD/Web canisters)
/// - UserSelf (users accessing their own data)
/// - Unauthorized (rejected)
///
/// CRITICAL for security: ensures only authorized entities can access data.

use candid::{encode_one, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::{CreateUserRequest, AgentActivity, KYCStatus};

fn get_data_canister_wasm() -> Vec<u8> {
    let wasm_path = std::env::var("DATA_CANISTER_WASM")
        .unwrap_or_else(|_| {
            let mut path = std::env::current_dir().unwrap();
            path.push("../../target/wasm32-unknown-unknown/release/data_canister.wasm");
            path.to_string_lossy().to_string()
        });

    std::fs::read(&wasm_path)
        .unwrap_or_else(|e| panic!("Failed to read WASM from {}: {}", wasm_path, e))
}

#[test]
fn test_controller_can_add_authorized_canister() {
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

    // Get controller principal
    let controller = pic.get_controllers(canister_id)[0];

    // Create a fake authorized canister
    let fake_canister = pic.create_canister();

    // Controller should be able to add authorized canister
    let add_result = pic.update_call(
        canister_id,
        controller,
        "add_authorized_canister",
        encode_one(fake_canister.to_text()).unwrap(),
    );

    assert!(add_result.is_ok(), "Controller should be able to add authorized canister");

    // Verify it was added
    let list_result = pic.query_call(
        canister_id,
        controller,
        "list_authorized_canisters",
        encode_one(()).unwrap(),
    ).unwrap();

    let canisters: Result<Vec<String>, String> = decode_one(&list_result).unwrap();
    assert!(canisters.is_ok());
    assert!(canisters.unwrap().contains(&fake_canister.to_text()));
}

#[test]
fn test_non_controller_cannot_add_authorized_canister() {
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

    // Create random principal (not controller)
    let random_principal = Principal::from_text("aaaaa-aa").unwrap();
    let fake_canister = pic.create_canister();

    // Non-controller should NOT be able to add authorized canister
    let add_result = pic.update_call(
        canister_id,
        random_principal,
        "add_authorized_canister",
        encode_one(fake_canister.to_text()).unwrap(),
    );

    // Should fail or return error
    if let Ok(response) = add_result {
        let result: Result<(), String> = decode_one(&response).unwrap();
        assert!(result.is_err(), "Non-controller should not be able to add authorized canister");
        assert!(result.unwrap_err().contains("Unauthorized"));
    }
}

#[test]
fn test_authorized_canister_can_create_user() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    // Create authorized canister
    let ussd_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        // Initialize with USSD canister as authorized
        encode_one(Some((Some(ussd_canister.to_text()), None::<String>))).unwrap(),
        None,
    );

    // Authorized canister should be able to create user
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000001".to_string()),
    };

    let create_result = pic.update_call(
        canister_id,
        ussd_canister,
        "create_user",
        encode_one(user_request).unwrap(),
    );

    assert!(create_result.is_ok(), "Authorized canister should be able to create user");
    let user: Result<shared_types::User, String> = decode_one(&create_result.unwrap()).unwrap();
    assert!(user.is_ok());
}

#[test]
fn test_unauthorized_canister_cannot_create_user() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    // Create authorized canister
    let ussd_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(Some((Some(ussd_canister.to_text()), None::<String>))).unwrap(),
        None,
    );

    // Create UNAUTHORIZED canister
    let random_canister = pic.create_canister();

    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000002".to_string()),
    };

    let create_result = pic.update_call(
        canister_id,
        random_canister, // NOT authorized
        "create_user",
        encode_one(user_request).unwrap(),
    );

    // Should fail
    if let Ok(response) = create_result {
        let result: Result<shared_types::User, String> = decode_one(&response).unwrap();
        assert!(result.is_err(), "Unauthorized canister should not be able to create user");
        assert!(result.unwrap_err().contains("Unauthorized"));
    }
}

#[test]
fn test_authorized_canister_can_store_agent_activity() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let agent_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(Some((Some(agent_canister.to_text()), None::<String>))).unwrap(),
        None,
    );

    let activity = AgentActivity {
        agent_id: "agent_001".to_string(),
        currency: "UGX".to_string(),
        deposits_today: 10,
        withdrawals_today: 5,
        deposit_volume_today: 1_000_000,
        withdrawal_volume_today: 500_000,
        operations_last_hour: vec![],
        operations_last_24h: vec![],
        user_agent_pairs: vec![],
        last_reset: 1699459000,
        last_updated: 1699459500,
    };

    let store_result = pic.update_call(
        canister_id,
        agent_canister,
        "store_agent_activity",
        encode_one(activity).unwrap(),
    );

    assert!(store_result.is_ok(), "Authorized canister should be able to store agent activity");
}

#[test]
fn test_unauthorized_canister_cannot_store_agent_activity() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let authorized_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(Some((Some(authorized_canister.to_text()), None::<String>))).unwrap(),
        None,
    );

    let unauthorized_canister = pic.create_canister();

    let activity = AgentActivity {
        agent_id: "agent_001".to_string(),
        currency: "UGX".to_string(),
        deposits_today: 10,
        withdrawals_today: 5,
        deposit_volume_today: 1_000_000,
        withdrawal_volume_today: 500_000,
        operations_last_hour: vec![],
        operations_last_24h: vec![],
        user_agent_pairs: vec![],
        last_reset: 1699459000,
        last_updated: 1699459500,
    };

    let store_result = pic.update_call(
        canister_id,
        unauthorized_canister, // NOT authorized
        "store_agent_activity",
        encode_one(activity).unwrap(),
    );

    if let Ok(response) = store_result {
        let result: Result<AgentActivity, String> = decode_one(&response).unwrap();
        assert!(result.is_err(), "Unauthorized canister should not be able to store agent activity");
        assert!(result.unwrap_err().contains("Unauthorized"));
    }
}

#[test]
fn test_controller_can_list_authorized_canisters() {
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

    let controller = pic.get_controllers(canister_id)[0];

    let list_result = pic.query_call(
        canister_id,
        controller,
        "list_authorized_canisters",
        encode_one(()).unwrap(),
    );

    assert!(list_result.is_ok(), "Controller should be able to list authorized canisters");
    let canisters: Result<Vec<String>, String> = decode_one(&list_result.unwrap()).unwrap();
    assert!(canisters.is_ok());
}

#[test]
fn test_non_controller_cannot_list_authorized_canisters() {
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

    let random_principal = Principal::from_text("aaaaa-aa").unwrap();

    let list_result = pic.query_call(
        canister_id,
        random_principal,
        "list_authorized_canisters",
        encode_one(()).unwrap(),
    );

    if let Ok(response) = list_result {
        let result: Result<Vec<String>, String> = decode_one(&response).unwrap();
        assert!(result.is_err(), "Non-controller should not be able to list authorized canisters");
        assert!(result.unwrap_err().contains("Unauthorized"));
    }
}

#[test]
fn test_controller_can_remove_authorized_canister() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let authorized_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(Some((Some(authorized_canister.to_text()), None::<String>))).unwrap(),
        None,
    );

    let controller = pic.get_controllers(canister_id)[0];

    // Remove the authorized canister
    let remove_result = pic.update_call(
        canister_id,
        controller,
        "remove_authorized_canister",
        encode_one(authorized_canister.to_text()).unwrap(),
    );

    assert!(remove_result.is_ok(), "Controller should be able to remove authorized canister");

    // Verify it was removed
    let list_result = pic.query_call(
        canister_id,
        controller,
        "list_authorized_canisters",
        encode_one(()).unwrap(),
    ).unwrap();

    let canisters: Result<Vec<String>, String> = decode_one(&list_result).unwrap();
    assert!(canisters.is_ok());
    assert!(!canisters.unwrap().contains(&authorized_canister.to_text()),
        "Canister should be removed from authorized list");
}

#[test]
fn test_authorized_canister_can_update_kyc_status() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let web_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(Some((None::<String>, Some(web_canister.to_text())))).unwrap(),
        None,
    );

    // Create user first
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "kyc@example.com".to_string(),
        first_name: "KYC".to_string(),
        last_name: "Test".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000003".to_string()),
    };

    let create_result = pic.update_call(
        canister_id,
        web_canister,
        "create_user",
        encode_one(user_request).unwrap(),
    );
    let user: Result<shared_types::User, String> = decode_one(&create_result.unwrap()).unwrap();
    let user_id = user.unwrap().id;

    // Update KYC status
    let update_result = pic.update_call(
        canister_id,
        web_canister,
        "update_kyc_status",
        encode_one((user_id, KYCStatus::Approved)).unwrap(),
    );

    assert!(update_result.is_ok(), "Authorized canister should be able to update KYC status");
}

#[test]
fn test_unauthorized_principal_cannot_update_kyc_status() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let web_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        encode_one(Some((None::<String>, Some(web_canister.to_text())))).unwrap(),
        None,
    );

    // Create user
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "kyc@example.com".to_string(),
        first_name: "KYC".to_string(),
        last_name: "Test".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000004".to_string()),
    };

    let create_result = pic.update_call(
        canister_id,
        web_canister,
        "create_user",
        encode_one(user_request).unwrap(),
    );
    let user: Result<shared_types::User, String> = decode_one(&create_result.unwrap()).unwrap();
    let user_id = user.unwrap().id;

    // Try to update KYC from random principal
    let random_principal = Principal::from_text("aaaaa-aa").unwrap();
    let update_result = pic.update_call(
        canister_id,
        random_principal,
        "update_kyc_status",
        encode_one((user_id, KYCStatus::Approved)).unwrap(),
    );

    if let Ok(response) = update_result {
        let result: Result<(), String> = decode_one(&response).unwrap();
        assert!(result.is_err(), "Unauthorized principal should not be able to update KYC");
        assert!(result.unwrap_err().contains("Unauthorized"));
    }
}

#[test]
fn test_multiple_authorized_canisters() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let ussd_canister = pic.create_canister();
    let web_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm,
        // Initialize with both USSD and Web
        encode_one(Some((Some(ussd_canister.to_text()), Some(web_canister.to_text())))).unwrap(),
        None,
    );

    // Both should be able to create users
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "ussd@example.com".to_string(),
        first_name: "USSD".to_string(),
        last_name: "User".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000005".to_string()),
    };

    let ussd_result = pic.update_call(
        canister_id,
        ussd_canister,
        "create_user",
        encode_one(user_request.clone()).unwrap(),
    );
    assert!(ussd_result.is_ok(), "USSD canister should be authorized");

    let user_request2 = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "web@example.com".to_string(),
        first_name: "Web".to_string(),
        last_name: "User".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000006".to_string()),
    };

    let web_result = pic.update_call(
        canister_id,
        web_canister,
        "create_user",
        encode_one(user_request2).unwrap(),
    );
    assert!(web_result.is_ok(), "Web canister should be authorized");
}
