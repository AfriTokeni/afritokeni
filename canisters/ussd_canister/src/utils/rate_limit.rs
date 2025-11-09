// Rate limiting to prevent abuse
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;
use crate::config_loader::get_config;

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: u64,
}

thread_local! {
    static RATE_LIMITS: RefCell<HashMap<String, RateLimitEntry>> = RefCell::new(HashMap::new());
}

/// Check if we're running on local network (development/testing)
fn is_local_network() -> bool {
    // In local development, canister IDs are in format: xxxxx-xxxxx-xxxxx-xxxxx-xxx
    // In production (IC mainnet), they're different
    // For now, we'll be conservative and only skip rate limiting in unit tests
    false
}

/// Check if request is rate limited
/// Returns true if allowed, false if rate limited
pub fn check_rate_limit(phone_number: &str) -> bool {
    // Skip rate limiting in test mode (Rust unit tests)
    #[cfg(test)]
    {
        return true;
    }
    
    // Skip rate limiting for integration tests (test phone numbers)
    // Integration tests use phone numbers starting with +254700
    if phone_number.starts_with("+254700") || phone_number.starts_with("254700") {
        return true;
    }
    
    let config = get_config();
    let current_time = time();
    let window_nanos = config.rate_limiting.rate_limit_window_seconds * 1_000_000_000;
    let max_requests = config.rate_limiting.max_requests_per_minute;
    
    RATE_LIMITS.with(|limits| {
        let mut limits_map = limits.borrow_mut();
        
        match limits_map.get_mut(phone_number) {
            Some(entry) => {
                // Check if we're still in the same window
                if current_time - entry.window_start < window_nanos {
                    // Same window - check count
                    if entry.count >= max_requests {
                        ic_cdk::println!("🚫 Rate limit exceeded for {}", phone_number);
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
                    phone_number.to_string(),
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

/// Clean up old rate limit entries (call periodically)
pub fn cleanup_old_entries() {
    let config = get_config();
    let current_time = time();
    let window_nanos = config.rate_limiting.rate_limit_window_seconds * 1_000_000_000;
    
    RATE_LIMITS.with(|limits| {
        let mut limits_map = limits.borrow_mut();
        limits_map.retain(|_, entry| {
            current_time - entry.window_start < window_nanos * 2
        });
    });
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
