/// Security Monitoring Module for crypto_canister
///
/// Tracks security events:
/// - PIN brute force attempts (5+ failures in 5 minutes)
/// - Suspicious transaction amounts (>$500 = 50,000 cents)
/// - High-risk transaction amounts (>$1,000 = 100,000 cents)
/// - Rapid transaction velocity (>10 transactions/hour)
///
/// All tracking uses rolling time windows and auto-cleans old entries.

use ic_cdk::api::time;
use shared_types::audit;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::config::get_config;

// ============================================================================
// Security Event Tracking State
// ============================================================================

thread_local! {
    /// PIN failure tracking per user (user_id -> Vec<timestamp_ns>)
    static PIN_FAILURES: RefCell<HashMap<String, Vec<u64>>> = RefCell::new(HashMap::new());

    /// Transaction velocity tracking per user (user_id -> Vec<timestamp_ns>)
    static TRANSACTION_VELOCITY: RefCell<HashMap<String, Vec<u64>>> = RefCell::new(HashMap::new());
}

// ============================================================================
// Constants
// ============================================================================

const PIN_FAILURE_WINDOW_NS: u64 = 5 * 60 * 1_000_000_000; // 5 minutes in nanoseconds
const PIN_FAILURE_THRESHOLD: usize = 5; // 5 failures triggers brute force alert

const VELOCITY_WINDOW_NS: u64 = 60 * 60 * 1_000_000_000; // 1 hour in nanoseconds
const VELOCITY_THRESHOLD: usize = 10; // 10 transactions/hour triggers alert

// ============================================================================
// Public API
// ============================================================================

/// Track a PIN failure and check if it triggers brute force detection
/// Returns true if brute force threshold exceeded (for optional rate limiting)
pub fn track_pin_failure(user_id: &str) -> bool {
    let now = time();
    let mut triggered = false;

    PIN_FAILURES.with(|failures| {
        let mut failures = failures.borrow_mut();
        let user_failures = failures.entry(user_id.to_string()).or_insert_with(Vec::new);

        // Add new failure
        user_failures.push(now);

        // Clean up old failures (older than 5 minutes)
        user_failures.retain(|&timestamp| now - timestamp <= PIN_FAILURE_WINDOW_NS);

        // Check if threshold exceeded
        if user_failures.len() >= PIN_FAILURE_THRESHOLD {
            triggered = true;

            // Log security event if enabled
            if get_config().security.log_suspicious_activity {
                audit::log_security_event(
                    "pin_brute_force",
                    Some(user_id.to_string()),
                    format!(
                        "Detected {} failed PIN attempts in 5 minutes - potential brute force attack",
                        user_failures.len()
                    )
                );
            }
        }
    });

    triggered
}

/// Check if transaction amount is suspicious and log if needed
/// Returns true if amount triggered security logging
pub fn check_suspicious_amount(user_id: &str, amount_cents: u64) -> bool {
    let config = get_config();

    if !config.security.log_suspicious_activity {
        return false;
    }

    let suspicious_threshold = config.fraud_detection.suspicious_amount_cents;
    let high_risk_threshold = config.fraud_detection.high_risk_amount_cents;

    if amount_cents >= high_risk_threshold {
        audit::log_security_event(
            "high_risk_amount",
            Some(user_id.to_string()),
            format!(
                "High risk transaction: {} cents (${:.2}) - exceeds ${} threshold",
                amount_cents,
                amount_cents as f64 / 100.0,
                high_risk_threshold / 100
            )
        );
        return true;
    }

    if amount_cents >= suspicious_threshold {
        audit::log_security_event(
            "suspicious_amount",
            Some(user_id.to_string()),
            format!(
                "Suspicious transaction: {} cents (${:.2}) - exceeds ${} threshold",
                amount_cents,
                amount_cents as f64 / 100.0,
                suspicious_threshold / 100
            )
        );
        return true;
    }

    false
}

/// Track a transaction and check for rapid velocity
/// Returns true if velocity threshold exceeded (for optional rate limiting)
pub fn track_transaction_velocity(user_id: &str) -> bool {
    let now = time();
    let mut triggered = false;

    TRANSACTION_VELOCITY.with(|velocity| {
        let mut velocity = velocity.borrow_mut();
        let user_txns = velocity.entry(user_id.to_string()).or_insert_with(Vec::new);

        // Add new transaction
        user_txns.push(now);

        // Clean up old transactions (older than 1 hour)
        user_txns.retain(|&timestamp| now - timestamp <= VELOCITY_WINDOW_NS);

        // Check if threshold exceeded
        if user_txns.len() > VELOCITY_THRESHOLD {
            triggered = true;

            // Log security event if enabled
            if get_config().security.log_suspicious_activity {
                audit::log_security_event(
                    "rapid_transactions",
                    Some(user_id.to_string()),
                    format!(
                        "Detected {} transactions in 1 hour - suspicious velocity pattern",
                        user_txns.len()
                    )
                );
            }
        }
    });

    triggered
}

/// Clear all tracking data for a user (useful for testing or admin cleanup)
pub fn clear_user_tracking(user_id: &str) {
    PIN_FAILURES.with(|failures| {
        failures.borrow_mut().remove(user_id);
    });

    TRANSACTION_VELOCITY.with(|velocity| {
        velocity.borrow_mut().remove(user_id);
    });
}

/// Get tracking statistics (for debugging/monitoring)
pub fn get_tracking_stats() -> TrackingStats {
    let pin_tracked_users = PIN_FAILURES.with(|f| f.borrow().len());
    let velocity_tracked_users = TRANSACTION_VELOCITY.with(|v| v.borrow().len());

    TrackingStats {
        users_with_pin_failures: pin_tracked_users,
        users_with_velocity_tracking: velocity_tracked_users,
    }
}

#[derive(Debug, Clone)]
pub struct TrackingStats {
    pub users_with_pin_failures: usize,
    pub users_with_velocity_tracking: usize,
}

// ============================================================================
// Periodic Cleanup (prevents unbounded growth)
// ============================================================================

/// Clean up old tracking data across all users
/// Should be called periodically (e.g., every hour via timer)
pub fn cleanup_old_tracking_data() {
    let now = time();
    let mut cleaned_pin = 0;
    let mut cleaned_velocity = 0;

    PIN_FAILURES.with(|failures| {
        let mut failures = failures.borrow_mut();
        failures.retain(|_user_id, timestamps| {
            timestamps.retain(|&ts| now - ts <= PIN_FAILURE_WINDOW_NS);
            if timestamps.is_empty() {
                cleaned_pin += 1;
                false // Remove empty entries
            } else {
                true
            }
        });
    });

    TRANSACTION_VELOCITY.with(|velocity| {
        let mut velocity = velocity.borrow_mut();
        velocity.retain(|_user_id, timestamps| {
            timestamps.retain(|&ts| now - ts <= VELOCITY_WINDOW_NS);
            if timestamps.is_empty() {
                cleaned_velocity += 1;
                false // Remove empty entries
            } else {
                true
            }
        });
    });

    ic_cdk::println!(
        "Security monitoring cleanup: removed {} PIN tracking entries, {} velocity tracking entries",
        cleaned_pin,
        cleaned_velocity
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_failure_tracking() {
        let user_id = "test_user";

        // First 4 failures should not trigger
        for _ in 0..4 {
            assert!(!track_pin_failure(user_id));
        }

        // 5th failure should trigger
        assert!(track_pin_failure(user_id));

        // Cleanup
        clear_user_tracking(user_id);
    }

    #[test]
    fn test_transaction_velocity() {
        let user_id = "test_user";

        // First 10 transactions should not trigger
        for _ in 0..10 {
            assert!(!track_transaction_velocity(user_id));
        }

        // 11th transaction should trigger
        assert!(track_transaction_velocity(user_id));

        // Cleanup
        clear_user_tracking(user_id);
    }
}
