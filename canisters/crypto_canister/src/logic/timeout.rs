/// Transaction timeout mechanisms for critical operations
/// Prevents hung operations from blocking the canister

use ic_cdk::api::time;

/// Default timeout for critical operations (30 seconds in nanoseconds)
#[allow(dead_code)]
const DEFAULT_OPERATION_TIMEOUT_NS: u64 = 30_000_000_000;

/// Maximum timeout for any operation (2 minutes in nanoseconds)
#[allow(dead_code)]
const MAX_OPERATION_TIMEOUT_NS: u64 = 120_000_000_000;

/// Transaction start time tracker
#[allow(dead_code)]
pub struct TransactionTimer {
    start_time: u64,
    timeout_ns: u64,
    operation_name: String,
}

impl TransactionTimer {
    /// Creates a new transaction timer with default timeout
    pub fn new(operation_name: &str) -> Self {
        Self::with_timeout(operation_name, DEFAULT_OPERATION_TIMEOUT_NS)
    }

    /// Creates a new transaction timer with custom timeout
    pub fn with_timeout(operation_name: &str, timeout_ns: u64) -> Self {
        let timeout_ns = timeout_ns.min(MAX_OPERATION_TIMEOUT_NS);
        Self {
            start_time: time(),
            timeout_ns,
            operation_name: operation_name.to_string(),
        }
    }

    /// Checks if operation has timed out
    pub fn is_timed_out(&self) -> bool {
        let elapsed = time().saturating_sub(self.start_time);
        elapsed >= self.timeout_ns
    }

    /// Returns error if timed out, Ok otherwise
    pub fn check_timeout(&self) -> Result<(), String> {
        if self.is_timed_out() {
            Err(format!(
                "{} operation timed out after {} seconds",
                self.operation_name,
                self.timeout_ns / 1_000_000_000
            ))
        } else {
            Ok(())
        }
    }

    /// Gets elapsed time in nanoseconds
    pub fn elapsed_ns(&self) -> u64 {
        time().saturating_sub(self.start_time)
    }

    /// Gets elapsed time in seconds
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed_ns() as f64 / 1_000_000_000.0
    }

    /// Gets remaining time in nanoseconds
    pub fn remaining_ns(&self) -> u64 {
        self.timeout_ns.saturating_sub(self.elapsed_ns())
    }
}

/// Helper to wrap async operations with timeout checking
/// Usage: wrap with periodic timeout checks in long-running operations
#[allow(dead_code)]
pub struct TimeoutGuard {
    timer: TransactionTimer,
    last_check: u64,
    check_interval_ns: u64,
}

impl TimeoutGuard {
    /// Creates a new timeout guard with default check interval (1 second)
    pub fn new(operation_name: &str) -> Self {
        Self::with_interval(operation_name, DEFAULT_OPERATION_TIMEOUT_NS, 1_000_000_000)
    }

    /// Creates a timeout guard with custom timeout and check interval
    pub fn with_interval(operation_name: &str, timeout_ns: u64, check_interval_ns: u64) -> Self {
        Self {
            timer: TransactionTimer::with_timeout(operation_name, timeout_ns),
            last_check: time(),
            check_interval_ns,
        }
    }

    /// Checks if timeout occurred, returns Err if timed out
    /// Only performs actual check if interval has elapsed (for efficiency)
    pub fn check(&mut self) -> Result<(), String> {
        let now = time();
        if now.saturating_sub(self.last_check) >= self.check_interval_ns {
            self.last_check = now;
            self.timer.check_timeout()?;
        }
        Ok(())
    }

    /// Gets reference to underlying timer
    pub fn timer(&self) -> &TransactionTimer {
        &self.timer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_timer_new() {
        let timer = TransactionTimer::new("test_operation");
        assert_eq!(timer.operation_name, "test_operation");
        assert_eq!(timer.timeout_ns, DEFAULT_OPERATION_TIMEOUT_NS);
        assert!(!timer.is_timed_out());
    }

    #[test]
    fn test_transaction_timer_custom_timeout() {
        let custom_timeout = 5_000_000_000; // 5 seconds
        let timer = TransactionTimer::with_timeout("custom", custom_timeout);
        assert_eq!(timer.timeout_ns, custom_timeout);
    }

    #[test]
    fn test_transaction_timer_max_timeout_enforcement() {
        let huge_timeout = 999_000_000_000; // Way more than max
        let timer = TransactionTimer::with_timeout("test", huge_timeout);
        assert_eq!(timer.timeout_ns, MAX_OPERATION_TIMEOUT_NS);
    }

    #[test]
    fn test_transaction_timer_check_timeout_not_timed_out() {
        let timer = TransactionTimer::new("test");
        assert!(timer.check_timeout().is_ok());
    }

    #[test]
    fn test_transaction_timer_elapsed() {
        let timer = TransactionTimer::new("test");
        // Should have some elapsed time (even if minimal)
        assert!(timer.elapsed_ns() >= 0);
    }

    #[test]
    fn test_timeout_guard_new() {
        let guard = TimeoutGuard::new("test_guard");
        assert_eq!(guard.timer.operation_name, "test_guard");
    }

    #[test]
    fn test_timeout_guard_check_initially_ok() {
        let mut guard = TimeoutGuard::new("test");
        assert!(guard.check().is_ok());
    }

    #[test]
    fn test_timeout_guard_with_custom_interval() {
        let guard = TimeoutGuard::with_interval("test", 10_000_000_000, 500_000_000);
        assert_eq!(guard.check_interval_ns, 500_000_000);
        assert_eq!(guard.timer.timeout_ns, 10_000_000_000);
    }
}
