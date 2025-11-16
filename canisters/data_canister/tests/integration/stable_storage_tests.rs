/// Integration tests for stable storage persistence across upgrades
///
/// Tests the critical pre_upgrade → post_upgrade flow to ensure all data
/// (users, balances, transactions, escrows, agent data) survives canister upgrades.
/// This is CRITICAL functionality for production stability.

use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::{
    CreateUserRequest, FiatCurrency, KYCStatus, AgentActivity,
};
use std::{thread, time::Duration};

/// Helper to get the data_canister WASM path
fn get_data_canister_wasm() -> Vec<u8> {
    let wasm_path = std::env::var("DATA_CANISTER_WASM")
        .unwrap_or_else(|_| {
            // Default path relative to workspace root
            // current_dir when running tests from canisters/data_canister is the crate root
            // so we need to go up two levels to get to workspace root
            let mut path = std::env::current_dir().unwrap();
            path.push("../../target/wasm32-unknown-unknown/release/data_canister.wasm");
            path.to_string_lossy().to_string()
        });

    std::fs::read(&wasm_path)
        .unwrap_or_else(|e| panic!("Failed to read WASM from {}: {}. Run 'cargo build --release --target wasm32-unknown-unknown' first", wasm_path, e))
}

/// Helper to upgrade canister with retry logic for rate limiting
/// PocketIC can rate-limit install_code operations when too many happen in quick succession
fn upgrade_canister_with_retry(
    pic: &PocketIc,
    canister_id: Principal,
    wasm: Vec<u8>,
    arg: Vec<u8>,
) -> Result<(), String> {
    let max_retries = 3;
    let retry_delay = Duration::from_secs(2);

    for attempt in 0..max_retries {
        match pic.upgrade_canister(canister_id, wasm.clone(), arg.clone(), None) {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Check if it's a rate limiting error
                if format!("{:?}", e).contains("rate limited") ||
                   format!("{:?}", e).contains("CanisterInstallCodeRateLimited") {
                    if attempt < max_retries - 1 {
                        // Wait and retry
                        thread::sleep(retry_delay);
                        continue;
                    }
                }
                // Non-rate-limit error or final retry exhausted
                return Err(format!("{:?}", e));
            }
        }
    }
    Err("Max retries exceeded".to_string())
}

#[test]
fn test_stable_storage_users_survive_upgrade() {
    let pic = PocketIc::new();

    // Deploy canister
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000); // 2T cycles

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    );

    // Create a user (via anonymous - allowed in test mode when no authorized canisters)
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "test@example.com".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000001".to_string()),
    };

    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        encode_one(user_request).unwrap(),
    );

    assert!(result.is_ok(), "Failed to create user: {:?}", result);

    let user: Result<shared_types::User, String> = decode_one(&result.unwrap()).unwrap();
    assert!(user.is_ok(), "User creation returned error: {:?}", user);
    let user = user.unwrap();
    let user_id = user.id.clone();

    // Verify user exists before upgrade
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id.clone()).unwrap(),
    );

    assert!(get_result.is_ok());
    let user_before: Result<Option<shared_types::User>, String> = decode_one(&get_result.unwrap()).unwrap();
    assert!(user_before.is_ok());
    assert!(user_before.as_ref().unwrap().is_some());
    let user_before = user_before.unwrap().unwrap();
    assert_eq!(user_before.first_name, "John");
    assert_eq!(user_before.last_name, "Doe");

    // UPGRADE CANISTER
    pic.upgrade_canister(
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    ).expect("Upgrade failed");

    // Verify user still exists after upgrade
    let get_result_after = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id.clone()).unwrap(),
    );

    assert!(get_result_after.is_ok());
    let user_after: Result<Option<shared_types::User>, String> = decode_one(&get_result_after.unwrap()).unwrap();
    assert!(user_after.is_ok());
    assert!(user_after.as_ref().unwrap().is_some(), "User not found after upgrade");

    let user_after = user_after.unwrap().unwrap();
    assert_eq!(user_after.id, user_before.id);
    assert_eq!(user_after.first_name, user_before.first_name);
    assert_eq!(user_after.last_name, user_before.last_name);
    assert_eq!(user_after.email, user_before.email);
    assert_eq!(user_after.phone_number, user_before.phone_number);
}

#[test]
fn test_stable_storage_balances_survive_upgrade() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    );

    // Create user
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000002".to_string()),
    };

    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        encode_one(user_request).unwrap(),
    );

    let user: Result<shared_types::User, String> = decode_one(&result.unwrap()).unwrap();
    let user_id = user.unwrap().id;

    // Set fiat balance
    let set_balance_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "set_fiat_balance",
        encode_args((user_id.clone(), "UGX".to_string(), 500000u64)).unwrap(),
    );
    assert!(set_balance_result.is_ok(), "Failed to set balance: {:?}", set_balance_result);

    // Set crypto balance
    let set_crypto_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "set_crypto_balance",
        encode_args((user_id.clone(), 100000u64, 50000u64)).unwrap(),
    );
    assert!(set_crypto_result.is_ok(), "Failed to set crypto balance: {:?}", set_crypto_result);

    // Get balances before upgrade
    let fiat_balance_before = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((user_id.clone(), FiatCurrency::UGX)).unwrap(),
    ).unwrap();
    let fiat_bal_before: Result<u64, String> = decode_one(&fiat_balance_before).unwrap();
    assert_eq!(fiat_bal_before.unwrap(), 500000);

    let crypto_balance_before = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_crypto_balance",
        encode_one(user_id.clone()).unwrap(),
    ).unwrap();
    let crypto_bal_before: Result<shared_types::CryptoBalance, String> = decode_one(&crypto_balance_before).unwrap();
    let crypto_bal_before = crypto_bal_before.unwrap();
    assert_eq!(crypto_bal_before.ckbtc, 100000);
    assert_eq!(crypto_bal_before.ckusdc, 50000);

    // UPGRADE CANISTER
    pic.upgrade_canister(
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    ).expect("Upgrade failed");

    // Verify balances after upgrade
    let fiat_balance_after = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_fiat_balance",
        encode_args((user_id.clone(), FiatCurrency::UGX)).unwrap(),
    ).unwrap();
    let fiat_bal_after: Result<u64, String> = decode_one(&fiat_balance_after).unwrap();
    assert_eq!(fiat_bal_after.unwrap(), 500000, "Fiat balance not preserved after upgrade");

    let crypto_balance_after = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_crypto_balance",
        encode_one(user_id.clone()).unwrap(),
    ).unwrap();
    let crypto_bal_after: Result<shared_types::CryptoBalance, String> = decode_one(&crypto_balance_after).unwrap();
    let crypto_bal_after = crypto_bal_after.unwrap();
    assert_eq!(crypto_bal_after.ckbtc, 100000, "ckBTC balance not preserved");
    assert_eq!(crypto_bal_after.ckusdc, 50000, "ckUSDC balance not preserved");
}

#[test]
fn test_stable_storage_agent_activity_survives_upgrade() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    );

    // Store agent activity
    let activity = AgentActivity {
        agent_id: "agent_001".to_string(),
        currency: "UGX".to_string(),
        deposits_today: 10,
        withdrawals_today: 5,
        deposit_volume_today: 5_000_000,
        withdrawal_volume_today: 2_000_000,
        operations_last_hour: vec![1699459200, 1699459300, 1699459400],
        operations_last_24h: vec![1699459200, 1699459300, 1699459400, 1699459500],
        user_agent_pairs: vec![("user123".to_string(), 3), ("user456".to_string(), 2)],
        last_reset: 1699459000,
        last_updated: 1699459500,
    };

    let store_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity.clone()).unwrap(),
    );
    assert!(store_result.is_ok(), "Failed to store agent activity: {:?}", store_result);

    // Verify before upgrade
    let get_before = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_args(("agent_001", "UGX")).unwrap(),
    ).unwrap();
    let activity_before: Result<Option<AgentActivity>, String> = decode_one(&get_before).unwrap();
    assert!(activity_before.is_ok());
    assert!(activity_before.as_ref().unwrap().is_some());
    let activity_before = activity_before.unwrap().unwrap();
    assert_eq!(activity_before.deposits_today, 10);
    assert_eq!(activity_before.withdrawals_today, 5);
    assert_eq!(activity_before.deposit_volume_today, 5_000_000);

    // UPGRADE CANISTER
    pic.upgrade_canister(
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    ).expect("Upgrade failed");

    // Verify after upgrade
    let get_after = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_args(("agent_001", "UGX")).unwrap(),
    ).unwrap();
    let activity_after: Result<Option<AgentActivity>, String> = decode_one(&get_after).unwrap();
    assert!(activity_after.is_ok());
    assert!(activity_after.as_ref().unwrap().is_some(), "Agent activity lost after upgrade");

    let activity_after = activity_after.unwrap().unwrap();
    assert_eq!(activity_after.agent_id, activity_before.agent_id);
    assert_eq!(activity_after.currency, activity_before.currency);
    assert_eq!(activity_after.deposits_today, activity_before.deposits_today);
    assert_eq!(activity_after.withdrawals_today, activity_before.withdrawals_today);
    assert_eq!(activity_after.deposit_volume_today, activity_before.deposit_volume_today);
    assert_eq!(activity_after.operations_last_hour.len(), activity_before.operations_last_hour.len());
    assert_eq!(activity_after.user_agent_pairs.len(), activity_before.user_agent_pairs.len());
}

#[test]
fn test_stable_storage_multiple_agent_activities_survive_upgrade() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    );

    // Store multiple agent activities (different agents and currencies)
    let activities = vec![
        AgentActivity {
            agent_id: "agent_001".to_string(),
            currency: "UGX".to_string(),
            deposits_today: 10,
            withdrawals_today: 5,
            deposit_volume_today: 5_000_000,
            withdrawal_volume_today: 2_000_000,
            operations_last_hour: vec![],
            operations_last_24h: vec![],
            user_agent_pairs: vec![],
            last_reset: 1699459000,
            last_updated: 1699459500,
        },
        AgentActivity {
            agent_id: "agent_001".to_string(),
            currency: "NGN".to_string(),
            deposits_today: 8,
            withdrawals_today: 3,
            deposit_volume_today: 800_000,
            withdrawal_volume_today: 300_000,
            operations_last_hour: vec![],
            operations_last_24h: vec![],
            user_agent_pairs: vec![],
            last_reset: 1699459000,
            last_updated: 1699459500,
        },
        AgentActivity {
            agent_id: "agent_002".to_string(),
            currency: "UGX".to_string(),
            deposits_today: 15,
            withdrawals_today: 7,
            deposit_volume_today: 7_500_000,
            withdrawal_volume_today: 3_000_000,
            operations_last_hour: vec![],
            operations_last_24h: vec![],
            user_agent_pairs: vec![],
            last_reset: 1699459000,
            last_updated: 1699459500,
        },
    ];

    for activity in &activities {
        let store_result = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_agent_activity",
            encode_one(activity.clone()).unwrap(),
        );
        assert!(store_result.is_ok(), "Failed to store agent activity");
    }

    // UPGRADE CANISTER
    pic.upgrade_canister(
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    ).expect("Upgrade failed");

    // Verify all activities survive
    for activity in activities {
        let get_result = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_agent_activity",
            encode_args((activity.agent_id.as_str(), activity.currency.as_str())).unwrap(),
        ).unwrap();
        let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
        assert!(retrieved.is_ok());
        assert!(retrieved.as_ref().unwrap().is_some(),
            "Activity for agent {} currency {} lost", activity.agent_id, activity.currency);

        let retrieved = retrieved.unwrap().unwrap();
        assert_eq!(retrieved.deposits_today, activity.deposits_today);
        assert_eq!(retrieved.withdrawals_today, activity.withdrawals_today);
    }
}

#[test]
fn test_stable_storage_kyc_status_survives_upgrade() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((None::<String>, None::<String>)).unwrap(),
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
        phone_number: Some("+256700000003".to_string()),
    };

    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        encode_one(user_request).unwrap(),
    );
    let user: Result<shared_types::User, String> = decode_one(&result.unwrap()).unwrap();
    let user_id = user.unwrap().id;

    // Update KYC status to Approved
    let update_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "update_kyc_status",
        encode_args((user_id.clone(), KYCStatus::Approved)).unwrap(),
    );
    assert!(update_result.is_ok(), "Failed to update KYC status");

    // Verify KYC status before upgrade
    let user_before = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id.clone()).unwrap(),
    ).unwrap();
    let user_before: Result<Option<shared_types::User>, String> = decode_one(&user_before).unwrap();
    let user_before = user_before.unwrap().unwrap();
    assert_eq!(user_before.kyc_status, KYCStatus::Approved);

    // UPGRADE CANISTER
    pic.upgrade_canister(
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    ).expect("Upgrade failed");

    // Verify KYC status after upgrade
    let user_after = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_user",
        encode_one(user_id.clone()).unwrap(),
    ).unwrap();
    let user_after: Result<Option<shared_types::User>, String> = decode_one(&user_after).unwrap();
    let user_after = user_after.unwrap().unwrap();
    assert_eq!(user_after.kyc_status, KYCStatus::Approved, "KYC status not preserved after upgrade");
}

#[test]
fn test_stable_storage_empty_state_upgrade() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = get_data_canister_wasm();
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((None::<String>, None::<String>)).unwrap(),
        None,
    );

    // Upgrade without adding any data (edge case)
    upgrade_canister_with_retry(
        &pic,
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
    ).expect("Upgrade of empty canister failed");

    // Verify canister still works after upgrade
    let user_request = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "post-upgrade@example.com".to_string(),
        first_name: "Post".to_string(),
        last_name: "Upgrade".to_string(),
        principal_id: Some(Principal::anonymous().to_text()),
        phone_number: Some("+256700000004".to_string()),
    };

    let result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "create_user",
        encode_one(user_request).unwrap(),
    );

    assert!(result.is_ok(), "Cannot create user after empty upgrade");
}

#[test]
fn test_stable_storage_authorized_canisters_survive_upgrade() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    // Create a fake USSD canister to authorize
    let ussd_canister = pic.create_canister();

    let wasm = get_data_canister_wasm();

    // Initialize with authorized canister
    pic.install_canister(
        canister_id,
        wasm.clone(),
        encode_args((Some(ussd_canister.to_text()), None::<String>)).unwrap(),
        None,
    );

    // Verify authorized canister is set (as controller)
    let controller_principal = pic.get_controllers(canister_id)[0];
    let list_result = pic.query_call(
        canister_id,
        controller_principal,
        "list_authorized_canisters",
        encode_one(()).unwrap(),
    );

    assert!(list_result.is_ok());
    let canisters_before: Result<Vec<String>, String> = decode_one(&list_result.unwrap()).unwrap();
    assert!(canisters_before.is_ok());
    let canisters_before = canisters_before.unwrap();
    assert!(canisters_before.contains(&ussd_canister.to_text()), "USSD canister not in authorized list");

    // UPGRADE CANISTER (without re-initializing authorized canisters)
    upgrade_canister_with_retry(
        &pic,
        canister_id,
        wasm,
        encode_args((None::<String>, None::<String>)).unwrap(),
    ).expect("Upgrade failed");

    // Verify authorized canisters still present
    let list_result_after = pic.query_call(
        canister_id,
        controller_principal,
        "list_authorized_canisters",
        encode_one(()).unwrap(),
    );

    assert!(list_result_after.is_ok());
    let canisters_after: Result<Vec<String>, String> = decode_one(&list_result_after.unwrap()).unwrap();
    assert!(canisters_after.is_ok());
    let canisters_after = canisters_after.unwrap();
    assert!(canisters_after.contains(&ussd_canister.to_text()),
        "Authorized canisters lost after upgrade");
}
