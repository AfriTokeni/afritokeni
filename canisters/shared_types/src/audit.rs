/// Shared Audit Trail Module for All AfriTokeni Canisters
///
/// Provides distributed tracing capabilities similar to Jaeger for IC canisters.
/// Uses correlation IDs to track requests across inter-canister calls.
///
/// Usage in any canister:
/// ```rust
/// use shared_types::audit;
///
/// // Log successful operation
/// audit::log_success("user_created", Some(user_id), "Created new user".to_string());
///
/// // Log failed operation
/// audit::log_failure("pin_failed", Some(user_id), "Invalid PIN".to_string());
///
/// // Log inter-canister call
/// audit::log_inter_canister_call("data_canister", "get_user", Some(user_id));
/// ```

use candid::CandidType;
use ic_cdk::api::{msg_caller, time};
use serde::Deserialize;
use std::cell::RefCell;

use super::AuditEntry;

thread_local! {
    /// Audit log storage (kept in canister state)
    /// Automatically rotates to prevent unbounded growth
    static AUDIT_LOG: RefCell<Vec<AuditEntry>> = RefCell::new(Vec::new());
}

/// Maximum number of audit entries to keep (prevents unbounded growth)
const MAX_AUDIT_ENTRIES: usize = 10_000;

/// Log an audit entry with automatic correlation ID generation
pub fn log_audit(
    action: &str,
    user_id: Option<String>,
    details: String,
    success: bool,
) {
    let entry = AuditEntry {
        timestamp: time() / 1_000_000_000, // Convert nanoseconds to seconds
        action: action.to_string(),
        caller: msg_caller().to_text(),
        user_id: user_id.clone(),
        details: details.clone(),
        success,
    };

    AUDIT_LOG.with(|log| {
        let mut log = log.borrow_mut();
        log.push(entry.clone());

        // Rotate log if it exceeds max size
        if log.len() > MAX_AUDIT_ENTRIES {
            log.remove(0);
        }
    });

    // Also log to IC console for real-time debugging
    let status = if success { "✅" } else { "❌" };
    ic_cdk::println!(
        "{} AUDIT [{}] {} | Caller: {} | User: {:?} | {}",
        status,
        action,
        entry.timestamp,
        entry.caller,
        user_id,
        details
    );
}

/// Log a successful operation
pub fn log_success(action: &str, user_id: Option<String>, details: String) {
    log_audit(action, user_id, details, true);
}

/// Log a failed operation
pub fn log_failure(action: &str, user_id: Option<String>, details: String) {
    log_audit(action, user_id, details, false);
}

/// Log a security event (always uses "security_" prefix for easy filtering)
pub fn log_security_event(event_type: &str, user_id: Option<String>, details: String) {
    let security_action = if event_type.starts_with("security_") {
        event_type.to_string()
    } else {
        format!("security_{}", event_type)
    };
    log_audit(&security_action, user_id, details, false); // Security events marked as failures for alerting
}

/// Log an inter-canister call (for distributed tracing)
pub fn log_inter_canister_call(
    target_canister: &str,
    method: &str,
    user_id: Option<String>,
) {
    let correlation_id = generate_correlation_id();
    log_success(
        "inter_canister_call",
        user_id,
        format!(
            "Calling {}::{} | CorrelationID: {}",
            target_canister, method, correlation_id
        ),
    );
}

/// Log an inter-canister call result
pub fn log_inter_canister_result(
    target_canister: &str,
    method: &str,
    user_id: Option<String>,
    success: bool,
    error: Option<&str>,
) {
    let details = if success {
        format!("Successfully called {}::{}", target_canister, method)
    } else {
        format!(
            "Failed calling {}::{} | Error: {}",
            target_canister,
            method,
            error.unwrap_or("Unknown")
        )
    };

    log_audit("inter_canister_result", user_id, details, success);
}

/// Generate a correlation ID for tracing across canisters
/// Format: {timestamp}-{caller_short}-{random}
fn generate_correlation_id() -> String {
    let timestamp = time();
    let caller_short = &msg_caller().to_text()[..8]; // First 8 chars of principal
    format!("{}-{}", timestamp, caller_short)
}

/// Get recent audit entries (for debugging and monitoring)
pub fn get_audit_log(limit: Option<usize>) -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let limit = limit.unwrap_or(100).min(1000); // Max 1000 entries per query

        if log.len() <= limit {
            log.clone()
        } else {
            log[log.len() - limit..].to_vec()
        }
    })
}

/// Get audit entries for a specific user
pub fn get_user_audit_log(user_id: &str, limit: Option<usize>) -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let limit = limit.unwrap_or(100).min(1000);

        log.iter()
            .rev() // Most recent first
            .filter(|entry| {
                entry.user_id.as_ref().map(|id| id == user_id).unwrap_or(false)
            })
            .take(limit)
            .cloned()
            .collect()
    })
}

/// Get audit entries by action type
pub fn get_audit_by_action(action: &str, limit: Option<usize>) -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let limit = limit.unwrap_or(100).min(1000);

        log.iter()
            .rev() // Most recent first
            .filter(|entry| entry.action == action)
            .take(limit)
            .cloned()
            .collect()
    })
}

/// Get failed operations (for debugging)
pub fn get_failed_operations(limit: Option<usize>) -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let limit = limit.unwrap_or(100).min(1000);

        log.iter()
            .rev() // Most recent first
            .filter(|entry| !entry.success)
            .take(limit)
            .cloned()
            .collect()
    })
}

/// Get security-specific audit entries (all entries with "security_" prefix)
pub fn get_security_audit_log(limit: Option<usize>) -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let limit = limit.unwrap_or(100).min(1000);

        log.iter()
            .rev() // Most recent first
            .filter(|entry| entry.action.starts_with("security_"))
            .take(limit)
            .cloned()
            .collect()
    })
}

/// Get security audit entries for a specific user
pub fn get_user_security_log(user_id: &str, limit: Option<usize>) -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let limit = limit.unwrap_or(100).min(1000);

        log.iter()
            .rev() // Most recent first
            .filter(|entry| {
                entry.action.starts_with("security_")
                && entry.user_id.as_ref().map(|id| id == user_id).unwrap_or(false)
            })
            .take(limit)
            .cloned()
            .collect()
    })
}

/// Get audit log statistics
pub fn get_audit_stats() -> AuditStats {
    AUDIT_LOG.with(|log| {
        let log = log.borrow();
        let total = log.len();
        let successful = log.iter().filter(|e| e.success).count();
        let failed = total - successful;

        // Count by action type
        let mut action_counts = std::collections::HashMap::new();
        for entry in log.iter() {
            *action_counts.entry(entry.action.clone()).or_insert(0) += 1;
        }

        AuditStats {
            total_entries: total,
            successful_operations: successful,
            failed_operations: failed,
            unique_actions: action_counts.len(),
            most_common_action: action_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(action, _)| action.clone()),
        }
    })
}

/// Audit statistics response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditStats {
    pub total_entries: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub unique_actions: usize,
    pub most_common_action: Option<String>,
}

/// Get all audit entries (for upgrade persistence)
pub fn get_all_entries() -> Vec<AuditEntry> {
    AUDIT_LOG.with(|log| log.borrow().clone())
}

/// Restore audit entries from stable memory (used during post_upgrade)
pub fn restore_entries(entries: Vec<AuditEntry>) {
    AUDIT_LOG.with(|log| {
        *log.borrow_mut() = entries;
    });
}
