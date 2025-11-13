// ============================================================================
// Fraud Detection - Agent-Specific Risk Analysis
// ============================================================================
// Detects suspicious patterns in agent operations
// 100% unit test coverage required
// ============================================================================

use crate::config::get_config;
use std::collections::HashMap;

// ============================================================================
// Fraud Check Result
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FraudCheckResult {
    pub should_block: bool,
    pub risk_score: u8,  // 0-100
    pub warnings: Vec<String>,
}

impl FraudCheckResult {
    pub fn safe() -> Self {
        Self {
            should_block: false,
            risk_score: 0,
            warnings: Vec::new(),
        }
    }

    pub fn suspicious(warnings: Vec<String>) -> Self {
        Self {
            should_block: false,
            risk_score: 50,
            warnings,
        }
    }

    pub fn blocked(reason: String) -> Self {
        Self {
            should_block: true,
            risk_score: 100,
            warnings: vec![reason],
        }
    }
}

// ============================================================================
// Agent Activity Tracking
// ============================================================================

#[derive(Debug, Clone)]
pub struct AgentActivity {
    pub agent_id: String,
    pub deposits_today: u64,
    pub withdrawals_today: u64,
    pub deposit_volume_today: u64,
    pub withdrawal_volume_today: u64,
    pub operations_last_hour: Vec<u64>,  // timestamps
    pub operations_last_24h: Vec<u64>,   // timestamps
    pub user_agent_pairs: HashMap<String, u64>,  // user_id -> count
    pub last_reset: u64,  // timestamp of last daily reset
}

impl AgentActivity {
    pub fn new(agent_id: String, current_time_ns: u64) -> Self {
        Self {
            agent_id,
            deposits_today: 0,
            withdrawals_today: 0,
            deposit_volume_today: 0,
            withdrawal_volume_today: 0,
            operations_last_hour: Vec::new(),
            operations_last_24h: Vec::new(),
            user_agent_pairs: HashMap::new(),
            last_reset: current_time_ns,
        }
    }

    /// Reset daily counters if it's a new day
    pub fn maybe_reset_daily(&mut self, now: u64) {
        let day_in_ns = 86400 * 1_000_000_000;
        
        if now - self.last_reset >= day_in_ns {
            self.deposits_today = 0;
            self.withdrawals_today = 0;
            self.deposit_volume_today = 0;
            self.withdrawal_volume_today = 0;
            self.user_agent_pairs.clear();
            self.last_reset = now;
        }
    }

    /// Clean up old timestamps from velocity tracking
    pub fn cleanup_old_timestamps(&mut self, now: u64) {
        let config = get_config();
        
        let hour_ago = now.saturating_sub(config.fraud.velocity_check_window_1h * 1_000_000_000);
        let day_ago = now.saturating_sub(config.fraud.velocity_check_window_24h * 1_000_000_000);
        
        self.operations_last_hour.retain(|&ts| ts > hour_ago);
        self.operations_last_24h.retain(|&ts| ts > day_ago);
    }

    /// Record a new operation
    pub fn record_operation(&mut self, user_id: &str, is_deposit: bool, amount: u64, now: u64) {
        self.maybe_reset_daily(now);
        self.cleanup_old_timestamps(now);
        
        if is_deposit {
            self.deposits_today += 1;
            self.deposit_volume_today += amount;
        } else {
            self.withdrawals_today += 1;
            self.withdrawal_volume_today += amount;
        }
        
        self.operations_last_hour.push(now);
        self.operations_last_24h.push(now);
        
        *self.user_agent_pairs.entry(user_id.to_string()).or_insert(0) += 1;
    }
}

// ============================================================================
// Fraud Detection Functions
// ============================================================================

/// Check if agent has exceeded daily deposit limit
pub fn check_deposit_limit(activity: &AgentActivity) -> Result<(), String> {
    let config = get_config();
    
    if activity.deposits_today >= config.fraud.max_deposits_per_agent_per_day {
        return Err(format!(
            "Agent has exceeded daily deposit limit ({}/{})",
            activity.deposits_today,
            config.fraud.max_deposits_per_agent_per_day
        ));
    }
    
    Ok(())
}

/// Check if agent has exceeded daily withdrawal limit
pub fn check_withdrawal_limit(activity: &AgentActivity) -> Result<(), String> {
    let config = get_config();
    
    if activity.withdrawals_today >= config.fraud.max_withdrawals_per_agent_per_day {
        return Err(format!(
            "Agent has exceeded daily withdrawal limit ({}/{})",
            activity.withdrawals_today,
            config.fraud.max_withdrawals_per_agent_per_day
        ));
    }
    
    Ok(())
}

/// Check if agent has exceeded daily volume limit
pub fn check_volume_limit(
    activity: &AgentActivity,
    amount: u64,
    is_deposit: bool,
) -> Result<(), String> {
    let config = get_config();
    
    if is_deposit {
        let new_volume = activity.deposit_volume_today + amount;
        if new_volume > config.fraud.max_deposit_volume_per_day {
            return Err(format!(
                "Agent would exceed daily deposit volume limit ({} + {} > {})",
                activity.deposit_volume_today,
                amount,
                config.fraud.max_deposit_volume_per_day
            ));
        }
    } else {
        let new_volume = activity.withdrawal_volume_today + amount;
        if new_volume > config.fraud.max_withdrawal_volume_per_day {
            return Err(format!(
                "Agent would exceed daily withdrawal volume limit ({} + {} > {})",
                activity.withdrawal_volume_today,
                amount,
                config.fraud.max_withdrawal_volume_per_day
            ));
        }
    }
    
    Ok(())
}

/// Check velocity (operations per time window)
pub fn check_velocity(activity: &AgentActivity) -> FraudCheckResult {
    let config = get_config();
    let mut warnings = Vec::new();
    
    // Check hourly velocity
    if activity.operations_last_hour.len() as u64 >= config.fraud.max_operations_per_hour {
        warnings.push(format!(
            "High velocity: {} operations in last hour (limit: {})",
            activity.operations_last_hour.len(),
            config.fraud.max_operations_per_hour
        ));
    }
    
    // Check daily velocity
    if activity.operations_last_24h.len() as u64 >= config.fraud.max_operations_per_day {
        return FraudCheckResult::blocked(format!(
            "Velocity limit exceeded: {} operations in 24h (limit: {})",
            activity.operations_last_24h.len(),
            config.fraud.max_operations_per_day
        ));
    }
    
    if warnings.is_empty() {
        FraudCheckResult::safe()
    } else {
        FraudCheckResult::suspicious(warnings)
    }
}

/// Check for suspicious user-agent patterns
pub fn check_user_agent_patterns(
    activity: &AgentActivity,
    user_id: &str,
) -> FraudCheckResult {
    let config = get_config();
    let mut warnings = Vec::new();
    
    // Check if same user-agent pair is too frequent
    if let Some(&count) = activity.user_agent_pairs.get(user_id) {
        if count >= config.fraud.suspicious_same_user_agent_threshold {
            warnings.push(format!(
                "Suspicious pattern: User {} has {} transactions with this agent today",
                user_id, count
            ));
        }
    }
    
    if warnings.is_empty() {
        FraudCheckResult::safe()
    } else {
        FraudCheckResult::suspicious(warnings)
    }
}

/// Comprehensive fraud check for deposit
pub fn check_deposit_fraud(
    activity: &AgentActivity,
    user_id: &str,
    amount: u64,
) -> FraudCheckResult {
    let mut all_warnings = Vec::new();
    
    // Check limits
    if let Err(e) = check_deposit_limit(activity) {
        return FraudCheckResult::blocked(e);
    }
    
    if let Err(e) = check_volume_limit(activity, amount, true) {
        return FraudCheckResult::blocked(e);
    }
    
    // Check velocity
    let velocity_result = check_velocity(activity);
    if velocity_result.should_block {
        return velocity_result;
    }
    all_warnings.extend(velocity_result.warnings);
    
    // Check patterns
    let pattern_result = check_user_agent_patterns(activity, user_id);
    all_warnings.extend(pattern_result.warnings);
    
    if all_warnings.is_empty() {
        FraudCheckResult::safe()
    } else {
        FraudCheckResult::suspicious(all_warnings)
    }
}

/// Comprehensive fraud check for withdrawal
pub fn check_withdrawal_fraud(
    activity: &AgentActivity,
    user_id: &str,
    amount: u64,
) -> FraudCheckResult {
    let mut all_warnings = Vec::new();
    
    // Check limits
    if let Err(e) = check_withdrawal_limit(activity) {
        return FraudCheckResult::blocked(e);
    }
    
    if let Err(e) = check_volume_limit(activity, amount, false) {
        return FraudCheckResult::blocked(e);
    }
    
    // Check velocity
    let velocity_result = check_velocity(activity);
    if velocity_result.should_block {
        return velocity_result;
    }
    all_warnings.extend(velocity_result.warnings);
    
    // Check patterns
    let pattern_result = check_user_agent_patterns(activity, user_id);
    all_warnings.extend(pattern_result.warnings);
    
    if all_warnings.is_empty() {
        FraudCheckResult::safe()
    } else {
        FraudCheckResult::suspicious(all_warnings)
    }
}

// ============================================================================
// Unit Tests (100% Coverage Required)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::init_config;

    fn setup() {
        init_config();
    }

    #[test]
    fn test_agent_activity_new() {
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        assert_eq!(activity.agent_id, "agent123");
        assert_eq!(activity.deposits_today, 0);
        assert_eq!(activity.withdrawals_today, 0);
    }

    #[test]
    fn test_agent_activity_record_deposit() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.record_operation("user456", true, 10000, 1620328630000000000);
        
        assert_eq!(activity.deposits_today, 1);
        assert_eq!(activity.deposit_volume_today, 10000);
        assert_eq!(activity.operations_last_hour.len(), 1);
    }

    #[test]
    fn test_agent_activity_record_withdrawal() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.record_operation("user456", false, 5000, 1620328630000000000);
        
        assert_eq!(activity.withdrawals_today, 1);
        assert_eq!(activity.withdrawal_volume_today, 5000);
        assert_eq!(activity.operations_last_hour.len(), 1);
    }

    #[test]
    fn test_check_deposit_limit_ok() {
        setup();
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        assert!(check_deposit_limit(&activity).is_ok());
    }

    #[test]
    fn test_check_deposit_limit_exceeded() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.deposits_today = 101; // max is 100
        
        let result = check_deposit_limit(&activity);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeded daily deposit limit"));
    }

    #[test]
    fn test_check_withdrawal_limit_ok() {
        setup();
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        assert!(check_withdrawal_limit(&activity).is_ok());
    }

    #[test]
    fn test_check_withdrawal_limit_exceeded() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.withdrawals_today = 51; // max is 50
        
        let result = check_withdrawal_limit(&activity);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeded daily withdrawal limit"));
    }

    #[test]
    fn test_check_volume_limit_deposit_ok() {
        setup();
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        assert!(check_volume_limit(&activity, 1000000, true).is_ok());
    }

    #[test]
    fn test_check_volume_limit_deposit_exceeded() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.deposit_volume_today = 49000000;
        
        let result = check_volume_limit(&activity, 2000000, true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceed daily deposit volume limit"));
    }

    #[test]
    fn test_check_velocity_safe() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.operations_last_hour.push(1620328630000000000);
        
        let result = check_velocity(&activity);
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 0);
    }

    #[test]
    fn test_check_user_agent_patterns_safe() {
        setup();
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        
        let result = check_user_agent_patterns(&activity, "user456");
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 0);
    }

    #[test]
    fn test_check_user_agent_patterns_suspicious() {
        setup();
        let mut activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        activity.user_agent_pairs.insert("user456".to_string(), 11); // threshold is 10
        
        let result = check_user_agent_patterns(&activity, "user456");
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 50);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_check_deposit_fraud_safe() {
        setup();
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        
        let result = check_deposit_fraud(&activity, "user456", 10000);
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 0);
    }

    #[test]
    fn test_check_withdrawal_fraud_safe() {
        setup();
        let activity = AgentActivity::new("agent123".to_string(), 1620328630000000000);
        
        let result = check_withdrawal_fraud(&activity, "user456", 5000);
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 0);
    }

    #[test]
    fn test_fraud_check_result_safe() {
        let result = FraudCheckResult::safe();
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 0);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_fraud_check_result_suspicious() {
        let result = FraudCheckResult::suspicious(vec!["Warning".to_string()]);
        assert!(!result.should_block);
        assert_eq!(result.risk_score, 50);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_fraud_check_result_blocked() {
        let result = FraudCheckResult::blocked("Blocked".to_string());
        assert!(result.should_block);
        assert_eq!(result.risk_score, 100);
        assert_eq!(result.warnings.len(), 1);
    }
}
