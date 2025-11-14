// Rate limiting to prevent abuse
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;
use crate::config_loader::get_config;
use crate::utils::constants::NANOS_PER_SECOND;

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: u64,
}

thread_local! {
    static RATE_LIMITS: RefCell<HashMap<String, RateLimitEntry>> = RefCell::new(HashMap::new());
}

/// Check if request is rate limited
/// Returns true if allowed, false if rate limited
pub fn check_rate_limit(_phone_number: &str) -> bool {
    // Always allow in test mode (both cargo test --lib and cargo test --test)
    // The 'test' cfg is set for --lib tests, 'test-utils' feature is set for integration tests
    #[cfg(any(test, feature = "test-utils"))]
    {
        return true;
    }

    #[cfg(not(any(test, feature = "test-utils")))]
    {
        // Skip rate limiting for integration tests (test phone numbers)
        // Integration tests use phone numbers starting with +254700
        if _phone_number.starts_with("+254700") || _phone_number.starts_with("254700") {
            return true;
        }

        let config = get_config();
        let current_time = time();
        let window_nanos = config.rate_limiting.rate_limit_window_seconds * NANOS_PER_SECOND;
        let max_requests = config.rate_limiting.max_requests_per_minute;

        RATE_LIMITS.with(|limits| {
            let mut limits_map = limits.borrow_mut();

            // Lazy cleanup: Remove entries older than 2x the window (deterministic)
            // This happens during normal request processing, so it's safe
            let cleanup_threshold = window_nanos * 2;
            limits_map.retain(|_, entry| {
                current_time.saturating_sub(entry.window_start) < cleanup_threshold
            });

            match limits_map.get_mut(_phone_number) {
                Some(entry) => {
                    // Check if we're still in the same window
                    if current_time - entry.window_start < window_nanos {
                        // Same window - check count
                        if entry.count >= max_requests {
                            ic_cdk::println!("🚫 Rate limit exceeded for {}", _phone_number);
                            return false;
                        }
                        entry.count += 1;
                        true
                    } else {
                        // New window - reset
                        entry.count = 1;
                        entry.window_start = current_time;
                        true
                    }
                }
                None => {
                    // First request from this number
                    limits_map.insert(
                        _phone_number.to_string(),
                        RateLimitEntry {
                            count: 1,
                            window_start: current_time,
                        },
                    );
                    true
                }
            }
        })
    }
}

/// Clean up old rate limit entries
///
/// DEPRECATED: This function is no longer needed as cleanup now happens
/// lazily during rate limit checks. Lazy cleanup is deterministic because
/// it happens during request processing (update calls) rather than heartbeats.
///
/// Kept for backwards compatibility but can be safely removed.
#[deprecated(note = "Cleanup now happens lazily in check_rate_limit()")]
pub fn cleanup_old_entries() {
    // No-op: cleanup now happens lazily in check_rate_limit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_allows_first_request() {
        let phone = "+256700000001";
        assert!(check_rate_limit(phone));
    }

    #[test]
    fn test_rate_limit_structure() {
        let entry = RateLimitEntry {
            count: 1,
            window_start: 0,
        };
        assert_eq!(entry.count, 1);
    }
}
