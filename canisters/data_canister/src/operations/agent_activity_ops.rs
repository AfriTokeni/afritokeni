use shared_types::AgentActivity;
#[cfg(not(test))]
use shared_types::audit;
use std::collections::BTreeMap;

/// Get agent activity for a specific agent and currency
///
/// # Arguments
/// * `activities` - Reference to the agent activities storage
/// * `agent_id` - The agent's unique identifier
/// * `currency` - The currency code (e.g., "UGX", "NGN")
///
/// # Returns
/// * `Option<AgentActivity>` - The activity record if it exists, None otherwise
pub fn get_agent_activity(
    activities: &BTreeMap<String, AgentActivity>,
    agent_id: String,
    currency: String,
) -> Option<AgentActivity> {
    let key = format!("{}_{}", agent_id, currency);
    activities.get(&key).cloned()
}

/// Store or update agent activity
///
/// # Arguments
/// * `activities` - Mutable reference to the agent activities storage
/// * `activity` - The activity record to store
///
/// # Returns
/// * `Result<AgentActivity, String>` - The stored activity on success, error message on failure
///
/// # Access Control
/// This operation is canister-only and should be called via the lib.rs endpoint
/// which enforces `verify_canister_access()`
pub fn store_agent_activity(
    activities: &mut BTreeMap<String, AgentActivity>,
    activity: AgentActivity,
) -> Result<AgentActivity, String> {
    // Input validation
    if activity.agent_id.is_empty() {
        return Err("Agent ID cannot be empty".to_string());
    }

    if activity.currency.is_empty() {
        return Err("Currency cannot be empty".to_string());
    }

    // Validate currency format (should be 3 uppercase letters)
    if activity.currency.len() != 3 || !activity.currency.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(format!("Invalid currency format: {}. Expected 3 uppercase letters (e.g., UGX, NGN)", activity.currency));
    }

    let key = format!("{}_{}", activity.agent_id, activity.currency);

    // Store the activity
    activities.insert(key.clone(), activity.clone());

    // Audit log the operation (only in production, not in tests)
    #[cfg(not(test))]
    audit::log_success(
        "store_agent_activity",
        Some(activity.agent_id.clone()),
        format!(
            "Stored activity for currency {} - Deposits: {} (vol: {}), Withdrawals: {} (vol: {}), Ops 1h: {}, Ops 24h: {}",
            activity.currency,
            activity.deposits_today,
            activity.deposit_volume_today,
            activity.withdrawals_today,
            activity.withdrawal_volume_today,
            activity.operations_last_hour.len(),
            activity.operations_last_24h.len()
        )
    );

    Ok(activity)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_activity(agent_id: &str, currency: &str) -> AgentActivity {
        AgentActivity {
            agent_id: agent_id.to_string(),
            currency: currency.to_string(),
            deposits_today: 5,
            withdrawals_today: 3,
            deposit_volume_today: 1_000_000,
            withdrawal_volume_today: 500_000,
            operations_last_hour: vec![1699459200, 1699459300],
            operations_last_24h: vec![1699459200, 1699459300, 1699459400],
            user_agent_pairs: vec![("user123".to_string(), 2), ("user456".to_string(), 1)],
            last_reset: 1699459200,
            last_updated: 1699459300,
        }
    }

    #[test]
    fn test_store_agent_activity_success() {
        let mut activities = BTreeMap::new();
        let activity = create_test_activity("agent_001", "UGX");

        let result = store_agent_activity(&mut activities, activity.clone());

        assert!(result.is_ok());
        assert_eq!(activities.len(), 1);

        let stored = activities.get("agent_001_UGX").unwrap();
        assert_eq!(stored.agent_id, "agent_001");
        assert_eq!(stored.currency, "UGX");
        assert_eq!(stored.deposits_today, 5);
    }

    #[test]
    fn test_store_agent_activity_update_existing() {
        let mut activities = BTreeMap::new();
        let activity1 = create_test_activity("agent_001", "UGX");

        // Store initial activity
        store_agent_activity(&mut activities, activity1).unwrap();
        assert_eq!(activities.len(), 1);

        // Update with new values
        let mut activity2 = create_test_activity("agent_001", "UGX");
        activity2.deposits_today = 10;
        activity2.deposit_volume_today = 2_000_000;

        let result = store_agent_activity(&mut activities, activity2);
        assert!(result.is_ok());
        assert_eq!(activities.len(), 1); // Still only one entry

        let updated = activities.get("agent_001_UGX").unwrap();
        assert_eq!(updated.deposits_today, 10);
        assert_eq!(updated.deposit_volume_today, 2_000_000);
    }

    #[test]
    fn test_store_agent_activity_multiple_currencies() {
        let mut activities = BTreeMap::new();
        let activity_ugx = create_test_activity("agent_001", "UGX");
        let activity_ngn = create_test_activity("agent_001", "NGN");

        store_agent_activity(&mut activities, activity_ugx).unwrap();
        store_agent_activity(&mut activities, activity_ngn).unwrap();

        assert_eq!(activities.len(), 2);
        assert!(activities.contains_key("agent_001_UGX"));
        assert!(activities.contains_key("agent_001_NGN"));
    }

    #[test]
    fn test_store_agent_activity_empty_agent_id() {
        let mut activities = BTreeMap::new();
        let mut activity = create_test_activity("agent_001", "UGX");
        activity.agent_id = "".to_string();

        let result = store_agent_activity(&mut activities, activity);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Agent ID cannot be empty");
        assert_eq!(activities.len(), 0);
    }

    #[test]
    fn test_store_agent_activity_empty_currency() {
        let mut activities = BTreeMap::new();
        let mut activity = create_test_activity("agent_001", "UGX");
        activity.currency = "".to_string();

        let result = store_agent_activity(&mut activities, activity);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Currency cannot be empty");
        assert_eq!(activities.len(), 0);
    }

    #[test]
    fn test_store_agent_activity_invalid_currency_format() {
        let mut activities = BTreeMap::new();

        // Test lowercase
        let mut activity = create_test_activity("agent_001", "ugx");
        let result = store_agent_activity(&mut activities, activity);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid currency format"));

        // Test too long
        activity = create_test_activity("agent_001", "UGXX");
        let result = store_agent_activity(&mut activities, activity);
        assert!(result.is_err());

        // Test too short
        activity = create_test_activity("agent_001", "UG");
        let result = store_agent_activity(&mut activities, activity);
        assert!(result.is_err());

        // Test with numbers
        activity = create_test_activity("agent_001", "U12");
        let result = store_agent_activity(&mut activities, activity);
        assert!(result.is_err());

        assert_eq!(activities.len(), 0);
    }

    #[test]
    fn test_get_agent_activity_exists() {
        let mut activities = BTreeMap::new();
        let activity = create_test_activity("agent_001", "UGX");
        store_agent_activity(&mut activities, activity).unwrap();

        let result = get_agent_activity(&activities, "agent_001".to_string(), "UGX".to_string());

        assert!(result.is_some());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.agent_id, "agent_001");
        assert_eq!(retrieved.currency, "UGX");
        assert_eq!(retrieved.deposits_today, 5);
    }

    #[test]
    fn test_get_agent_activity_not_exists() {
        let activities = BTreeMap::new();

        let result = get_agent_activity(&activities, "agent_999".to_string(), "UGX".to_string());

        assert!(result.is_none());
    }

    #[test]
    fn test_get_agent_activity_wrong_currency() {
        let mut activities = BTreeMap::new();
        let activity = create_test_activity("agent_001", "UGX");
        store_agent_activity(&mut activities, activity).unwrap();

        let result = get_agent_activity(&activities, "agent_001".to_string(), "NGN".to_string());

        assert!(result.is_none());
    }

    #[test]
    fn test_activity_key_format() {
        let mut activities = BTreeMap::new();
        let activity = create_test_activity("agent_123", "KES");
        store_agent_activity(&mut activities, activity).unwrap();

        // Verify the key format is exactly "agent_id_currency"
        assert!(activities.contains_key("agent_123_KES"));
        assert_eq!(activities.len(), 1);
    }

    #[test]
    fn test_activity_with_zero_values() {
        let mut activities = BTreeMap::new();
        let mut activity = create_test_activity("agent_001", "UGX");
        activity.deposits_today = 0;
        activity.withdrawals_today = 0;
        activity.deposit_volume_today = 0;
        activity.withdrawal_volume_today = 0;
        activity.operations_last_hour = vec![];
        activity.operations_last_24h = vec![];
        activity.user_agent_pairs = vec![];

        let result = store_agent_activity(&mut activities, activity);

        assert!(result.is_ok());
        let stored = activities.get("agent_001_UGX").unwrap();
        assert_eq!(stored.deposits_today, 0);
        assert_eq!(stored.operations_last_hour.len(), 0);
    }

    #[test]
    fn test_activity_with_large_values() {
        let mut activities = BTreeMap::new();
        let mut activity = create_test_activity("agent_001", "UGX");
        activity.deposits_today = u64::MAX;
        activity.deposit_volume_today = u64::MAX;
        activity.operations_last_hour = vec![1; 1000]; // Large vector
        activity.operations_last_24h = vec![1; 10000]; // Very large vector

        let result = store_agent_activity(&mut activities, activity);

        assert!(result.is_ok());
        let stored = activities.get("agent_001_UGX").unwrap();
        assert_eq!(stored.deposits_today, u64::MAX);
        assert_eq!(stored.operations_last_hour.len(), 1000);
        assert_eq!(stored.operations_last_24h.len(), 10000);
    }
}
