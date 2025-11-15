/// Integration tests for agent activity storage and retrieval
///
/// Tests the NEW agent activity endpoints for fraud detection:
/// - store_agent_activity (persist fraud detection metrics)
/// - get_agent_activity (retrieve for risk analysis)
///
/// These are critical for monitoring agent behavior patterns.

use candid::{encode_one, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::AgentActivity;

/// Helper to get the data_canister WASM path
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

fn create_test_activity(agent_id: &str, currency: &str, deposits: u64, withdrawals: u64) -> AgentActivity {
    AgentActivity {
        agent_id: agent_id.to_string(),
        currency: currency.to_string(),
        deposits_today: deposits,
        withdrawals_today: withdrawals,
        deposit_volume_today: deposits * 100_000,
        withdrawal_volume_today: withdrawals * 50_000,
        operations_last_hour: vec![1699459200, 1699459300],
        operations_last_24h: vec![1699459200, 1699459300, 1699459400, 1699459500],
        user_agent_pairs: vec![("user123".to_string(), 2), ("user456".to_string(), 1)],
        last_reset: 1699459000,
        last_updated: 1699459500,
    }
}

#[test]
fn test_store_and_retrieve_agent_activity() {
    let (pic, canister_id) = setup_canister();

    let activity = create_test_activity("agent_001", "UGX", 10, 5);

    // Store activity
    let store_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity.clone()).unwrap(),
    );

    assert!(store_result.is_ok(), "Failed to store activity: {:?}", store_result);
    let stored: Result<AgentActivity, String> = decode_one(&store_result.unwrap()).unwrap();
    assert!(stored.is_ok());

    // Retrieve activity
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    );

    assert!(get_result.is_ok());
    let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result.unwrap()).unwrap();
    assert!(retrieved.is_ok());
    assert!(retrieved.as_ref().unwrap().is_some());

    let retrieved = retrieved.unwrap().unwrap();
    assert_eq!(retrieved.agent_id, "agent_001");
    assert_eq!(retrieved.currency, "UGX");
    assert_eq!(retrieved.deposits_today, 10);
    assert_eq!(retrieved.withdrawals_today, 5);
}

#[test]
fn test_update_existing_agent_activity() {
    let (pic, canister_id) = setup_canister();

    // Store initial activity
    let activity1 = create_test_activity("agent_001", "UGX", 10, 5);
    let store1 = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity1).unwrap(),
    );
    assert!(store1.is_ok());

    // Update with new values
    let activity2 = create_test_activity("agent_001", "UGX", 20, 10);
    let store2 = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity2).unwrap(),
    );
    assert!(store2.is_ok());

    // Verify updated values
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();

    let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
    let retrieved = retrieved.unwrap().unwrap();
    assert_eq!(retrieved.deposits_today, 20, "Deposits not updated");
    assert_eq!(retrieved.withdrawals_today, 10, "Withdrawals not updated");
}

#[test]
fn test_multiple_currencies_for_same_agent() {
    let (pic, canister_id) = setup_canister();

    // Store activity for UGX
    let activity_ugx = create_test_activity("agent_001", "UGX", 10, 5);
    let store_ugx = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity_ugx).unwrap(),
    );
    assert!(store_ugx.is_ok());

    // Store activity for NGN
    let activity_ngn = create_test_activity("agent_001", "NGN", 8, 3);
    let store_ngn = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity_ngn).unwrap(),
    );
    assert!(store_ngn.is_ok());

    // Verify both exist independently
    let get_ugx = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();
    let ugx: Result<Option<AgentActivity>, String> = decode_one(&get_ugx).unwrap();
    assert!(ugx.unwrap().is_some());

    let get_ngn = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "NGN".to_string())).unwrap(),
    ).unwrap();
    let ngn: Result<Option<AgentActivity>, String> = decode_one(&get_ngn).unwrap();
    let ngn = ngn.unwrap().unwrap();
    assert_eq!(ngn.currency, "NGN");
    assert_eq!(ngn.deposits_today, 8);
}

#[test]
fn test_multiple_agents_same_currency() {
    let (pic, canister_id) = setup_canister();

    // Store activity for agent_001
    let activity1 = create_test_activity("agent_001", "UGX", 10, 5);
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity1).unwrap(),
    ).unwrap();

    // Store activity for agent_002
    let activity2 = create_test_activity("agent_002", "UGX", 15, 7);
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity2).unwrap(),
    ).unwrap();

    // Verify both exist
    let get1 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();
    let agent1: Result<Option<AgentActivity>, String> = decode_one(&get1).unwrap();
    let agent1 = agent1.unwrap().unwrap();
    assert_eq!(agent1.deposits_today, 10);

    let get2 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_002".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();
    let agent2: Result<Option<AgentActivity>, String> = decode_one(&get2).unwrap();
    let agent2 = agent2.unwrap().unwrap();
    assert_eq!(agent2.deposits_today, 15);
}

#[test]
fn test_get_nonexistent_agent_activity() {
    let (pic, canister_id) = setup_canister();

    // Try to get activity that doesn't exist
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_999".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();

    let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
    assert!(retrieved.is_ok());
    assert!(retrieved.unwrap().is_none(), "Should return None for nonexistent activity");
}

#[test]
fn test_store_activity_with_empty_vectors() {
    let (pic, canister_id) = setup_canister();

    let mut activity = create_test_activity("agent_001", "UGX", 0, 0);
    activity.operations_last_hour = vec![];
    activity.operations_last_24h = vec![];
    activity.user_agent_pairs = vec![];

    let store_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity).unwrap(),
    );

    assert!(store_result.is_ok(), "Should allow empty vectors");

    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();

    let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
    let retrieved = retrieved.unwrap().unwrap();
    assert_eq!(retrieved.operations_last_hour.len(), 0);
    assert_eq!(retrieved.operations_last_24h.len(), 0);
    assert_eq!(retrieved.user_agent_pairs.len(), 0);
}

#[test]
fn test_store_activity_with_large_volumes() {
    let (pic, canister_id) = setup_canister();

    let mut activity = create_test_activity("agent_001", "UGX", 1000, 500);
    activity.deposit_volume_today = u64::MAX / 2;
    activity.withdrawal_volume_today = u64::MAX / 2;
    activity.operations_last_hour = vec![1; 100]; // 100 operations
    activity.operations_last_24h = vec![1; 1000]; // 1000 operations

    let store_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity).unwrap(),
    );

    assert!(store_result.is_ok(), "Should handle large volumes");

    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();

    let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
    let retrieved = retrieved.unwrap().unwrap();
    assert_eq!(retrieved.deposit_volume_today, u64::MAX / 2);
    assert_eq!(retrieved.operations_last_hour.len(), 100);
    assert_eq!(retrieved.operations_last_24h.len(), 1000);
}

#[test]
fn test_fraud_detection_metrics_accuracy() {
    let (pic, canister_id) = setup_canister();

    // Simulate fraud detection scenario: high velocity operations
    let activity = AgentActivity {
        agent_id: "agent_suspicious".to_string(),
        currency: "UGX".to_string(),
        deposits_today: 50, // High count
        withdrawals_today: 45, // High count
        deposit_volume_today: 50_000_000, // 50M UGX
        withdrawal_volume_today: 45_000_000, // 45M UGX
        operations_last_hour: vec![1; 30], // 30 ops in 1 hour (high velocity)
        operations_last_24h: vec![1; 95], // 95 ops in 24h
        user_agent_pairs: vec![
            ("user1".to_string(), 20), // Same user 20 times (suspicious)
            ("user2".to_string(), 15),
            ("user3".to_string(), 10),
        ],
        last_reset: 1699459000,
        last_updated: 1699459500,
    };

    let store_result = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity.clone()).unwrap(),
    );
    assert!(store_result.is_ok());

    // Retrieve and verify fraud metrics are accurate
    let get_result = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_suspicious".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();

    let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
    let retrieved = retrieved.unwrap().unwrap();

    // Verify all fraud detection metrics
    assert_eq!(retrieved.deposits_today, 50);
    assert_eq!(retrieved.withdrawals_today, 45);
    assert_eq!(retrieved.deposit_volume_today, 50_000_000);
    assert_eq!(retrieved.operations_last_hour.len(), 30);
    assert_eq!(retrieved.operations_last_24h.len(), 95);
    assert_eq!(retrieved.user_agent_pairs.len(), 3);

    // Verify user-agent pair frequency (for coordination detection)
    let user1_frequency = retrieved.user_agent_pairs.iter()
        .find(|(user, _)| user == "user1")
        .map(|(_, count)| *count)
        .unwrap();
    assert_eq!(user1_frequency, 20, "User-agent pair frequency not preserved");
}

#[test]
fn test_activity_isolation_between_agents_and_currencies() {
    let (pic, canister_id) = setup_canister();

    // Create a matrix of activities
    let activities = vec![
        ("agent_001", "UGX", 10, 5),
        ("agent_001", "NGN", 8, 3),
        ("agent_001", "KES", 12, 6),
        ("agent_002", "UGX", 15, 7),
        ("agent_002", "NGN", 9, 4),
        ("agent_003", "UGX", 20, 10),
    ];

    // Store all activities
    for (agent, currency, deposits, withdrawals) in &activities {
        let activity = create_test_activity(agent, currency, *deposits, *withdrawals);
        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_agent_activity",
            encode_one(activity).unwrap(),
        ).unwrap();
    }

    // Verify each activity is isolated
    for (agent, currency, expected_deposits, expected_withdrawals) in activities {
        let get_result = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_agent_activity",
            encode_one((agent.to_string(), currency.to_string())).unwrap(),
        ).unwrap();

        let retrieved: Result<Option<AgentActivity>, String> = decode_one(&get_result).unwrap();
        let retrieved = retrieved.unwrap().unwrap();

        assert_eq!(retrieved.agent_id, agent, "Agent ID mismatch");
        assert_eq!(retrieved.currency, currency, "Currency mismatch");
        assert_eq!(retrieved.deposits_today, expected_deposits,
            "Deposits mismatch for {} {}", agent, currency);
        assert_eq!(retrieved.withdrawals_today, expected_withdrawals,
            "Withdrawals mismatch for {} {}", agent, currency);
    }
}

#[test]
fn test_concurrent_updates_to_different_activities() {
    let (pic, canister_id) = setup_canister();

    // Simulate concurrent updates to different agent-currency pairs
    let activity1 = create_test_activity("agent_001", "UGX", 10, 5);
    let activity2 = create_test_activity("agent_002", "NGN", 15, 7);

    // Both should succeed independently
    let store1 = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity1).unwrap(),
    );

    let store2 = pic.update_call(
        canister_id,
        Principal::anonymous(),
        "store_agent_activity",
        encode_one(activity2).unwrap(),
    );

    assert!(store1.is_ok());
    assert!(store2.is_ok());

    // Verify both exist
    let get1 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_001".to_string(), "UGX".to_string())).unwrap(),
    ).unwrap();
    let result1: Result<Option<AgentActivity>, String> = decode_one(&get1).unwrap();
    assert!(result1.unwrap().is_some());

    let get2 = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "get_agent_activity",
        encode_one(("agent_002".to_string(), "NGN".to_string())).unwrap(),
    ).unwrap();
    let result2: Result<Option<AgentActivity>, String> = decode_one(&get2).unwrap();
    assert!(result2.unwrap().is_some());
}
