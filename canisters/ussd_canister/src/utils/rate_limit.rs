// Rate limiting to prevent abuse
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;

const MAX_REQUESTS_PER_MINUTE: u32 = 10;
const RATE_LIMIT_WINDOW_NANOS: u64 = 60 * 1_000_000_000; // 1 minute

#[derive(Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: u64,
}

thread_local! {
    static RATE_LIMITS: RefCell<HashMap<String, RateLimitEntry>> = RefCell::new(HashMap::new());
}

/// Check if request is rate limited
/// Returns true if allowed, false if rate limited
pub fn check_rate_limit(phone_number: &str) -> bool {
    let current_time = time();
    
    RATE_LIMITS.with(|limits| {
        let mut limits_map = limits.borrow_mut();
        
        match limits_map.get_mut(phone_number) {
            Some(entry) => {
                // Check if we're still in the same window
                if current_time - entry.window_start < RATE_LIMIT_WINDOW_NANOS {
                    // Same window - check count
                    if entry.count >= MAX_REQUESTS_PER_MINUTE {
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
    let current_time = time();
    
    RATE_LIMITS.with(|limits| {
        let mut limits_map = limits.borrow_mut();
        limits_map.retain(|_, entry| {
            current_time - entry.window_start < RATE_LIMIT_WINDOW_NANOS * 2
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
