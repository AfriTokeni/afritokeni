use std::cell::RefCell;
use std::collections::HashMap;

// Use mock time in tests, real time in production
#[cfg(not(test))]
use ic_cdk::api::time;

#[cfg(test)]
thread_local! {
    static MOCK_TIME: RefCell<u64> = RefCell::new(1731574800000000000); // Nov 14, 2025, 9:00 AM UTC
}

#[cfg(test)]
fn time() -> u64 {
    MOCK_TIME.with(|t| {
        let current = *t.borrow();
        // Auto-increment by 1 second each call to simulate time passing
        *t.borrow_mut() += 1_000_000_000;
        current
    })
}

// ============================================================================
// TYPES
// ============================================================================

#[derive(Clone, Debug)]
pub struct FraudCheckResult {
    #[allow(dead_code)]
    pub is_suspicious: bool,
    pub should_block: bool,
    pub requires_manual_review: bool,
    pub risk_score: u32,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug)]
struct TransactionRecord {
    timestamp: u64,
    amount: u64,
    #[allow(dead_code)]
    currency: String,
    #[allow(dead_code)]
    operation: String,
}

#[derive(Clone, Debug)]
struct UserVelocityData {
    transactions: Vec<TransactionRecord>,
    total_24h: u64,
    total_1h: u64,
    #[allow(dead_code)]
    device_fingerprints: Vec<String>,
    #[allow(dead_code)]
    geo_locations: Vec<String>,
}

#[derive(Clone, Debug)]
struct DeviceInfo {
    fingerprint: String,
    #[allow(dead_code)]
    first_seen: u64,
    last_seen: u64,
    transaction_count: u32,
}

// ============================================================================
// STATE
// ============================================================================

thread_local! {
    // Rate limiting: user_id -> Vec<timestamp>
    static RATE_LIMITS: RefCell<HashMap<String, Vec<u64>>> = RefCell::new(HashMap::new());
    
    // Velocity tracking: user_id -> UserVelocityData
    static VELOCITY_DATA: RefCell<HashMap<String, UserVelocityData>> = RefCell::new(HashMap::new());
    
    // Device tracking: user_id -> Vec<DeviceInfo>
    static DEVICE_TRACKING: RefCell<HashMap<String, Vec<DeviceInfo>>> = RefCell::new(HashMap::new());
    
    // Geographic tracking: user_id -> Vec<location>
    static GEO_TRACKING: RefCell<HashMap<String, Vec<String>>> = RefCell::new(HashMap::new());
    
    // Per-operation rate limits: (user_id, operation) -> Vec<timestamp>
    static OPERATION_LIMITS: RefCell<HashMap<(String, String), Vec<u64>>> = RefCell::new(HashMap::new());
    
    // PIN attempt tracking: user_id -> (attempts, last_attempt_time, backoff_until)
    static PIN_ATTEMPTS: RefCell<HashMap<String, (u32, u64, u64)>> = RefCell::new(HashMap::new());
}

// ============================================================================
// CONFIGURATION
// ============================================================================
// All configuration values are now loaded from crypto_config.toml via config module

// ============================================================================
// RATE LIMITING
// ============================================================================

 

/// Check per-operation rate limit
pub fn check_operation_rate_limit(user_id: &str, operation: &str) -> Result<bool, String> {
    use crate::config;
    let cfg = config::get_config();

    OPERATION_LIMITS.with(|limits| {
        let mut limits = limits.borrow_mut();
        let now = time();
        let hour_ago = now.saturating_sub(3600_000_000_000); // 1 hour in nanoseconds

        let key = (user_id.to_string(), operation.to_string());
        let timestamps = limits.entry(key).or_insert_with(Vec::new);

        // Remove old timestamps
        timestamps.retain(|&ts| ts > hour_ago);

        // Check operation-specific limit from config
        let max_for_operation = match operation {
            "buy_crypto" => cfg.fraud_detection.max_buys_per_hour,
            "sell_crypto" => cfg.fraud_detection.max_sells_per_hour,
            "send_crypto" => cfg.fraud_detection.max_transfers_per_hour,
            "create_escrow" => cfg.fraud_detection.max_escrows_per_hour,
            _ => cfg.fraud_detection.max_transactions_per_hour,
        };

        if timestamps.len() >= max_for_operation {
            return Ok(false); // Operation rate limit exceeded
        }

        // Add current timestamp
        timestamps.push(now);
        Ok(true)
    })
}

// ============================================================================
// PIN ATTEMPT TRACKING WITH EXPONENTIAL BACKOFF
// ============================================================================

/// Check if PIN attempts are allowed (with exponential backoff)
pub fn check_pin_attempts_allowed(user_id: &str) -> Result<bool, String> {
    use crate::config;
    let cfg = config::get_config();

    PIN_ATTEMPTS.with(|attempts| {
        let attempts = attempts.borrow();
        let now = time();

        if let Some((count, _last_attempt, backoff_until)) = attempts.get(user_id) {
            // Check if still in backoff period
            if now < *backoff_until {
                let remaining_ns = backoff_until.saturating_sub(now);
                let remaining_secs = remaining_ns / 1_000_000_000;
                return Err(format!("Too many failed PIN attempts. Try again in {} seconds", remaining_secs));
            }

            // Check if max attempts exceeded
            if *count >= cfg.fraud_detection.max_pin_attempts {
                return Err("Maximum PIN attempts exceeded. Account temporarily locked".to_string());
            }
        }

        Ok(true)
    })
}

/// Record failed PIN attempt (with exponential backoff)
pub fn record_failed_pin_attempt(user_id: &str) -> Result<(), String> {
    use crate::config;
    let cfg = config::get_config();

    PIN_ATTEMPTS.with(|attempts| {
        let mut attempts = attempts.borrow_mut();
        let now = time();

        let (new_count, backoff_duration) = if let Some((count, _last_attempt, _backoff_until)) = attempts.get(user_id) {
            let new_count = count + 1;
            // Exponential backoff: 1min, 2min, 4min, 8min, 16min, ...
            let backoff = (cfg.fraud_detection.initial_backoff_ns * 2_u64.pow(new_count.saturating_sub(1)))
                .min(cfg.fraud_detection.max_backoff_ns);
            (new_count, backoff)
        } else {
            (1, cfg.fraud_detection.initial_backoff_ns)
        };

        let backoff_until = now + backoff_duration;
        attempts.insert(user_id.to_string(), (new_count, now, backoff_until));

        Ok(())
    })
}

/// Reset PIN attempts after successful verification
pub fn reset_pin_attempts(user_id: &str) {
    PIN_ATTEMPTS.with(|attempts| {
        attempts.borrow_mut().remove(user_id);
    });
}

// ============================================================================
// VELOCITY CHECKS
// ============================================================================

/// Record transaction for velocity tracking
pub fn record_transaction(
    user_id: &str,
    amount: u64,
    currency: &str,
    operation: &str,
) -> Result<(), String> {
    VELOCITY_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let now = time();
        
        let velocity = data.entry(user_id.to_string()).or_insert_with(|| UserVelocityData {
            transactions: Vec::new(),
            total_24h: 0,
            total_1h: 0,
            device_fingerprints: Vec::new(),
            geo_locations: Vec::new(),
        });
        
        // Add new transaction
        velocity.transactions.push(TransactionRecord {
            timestamp: now,
            amount,
            currency: currency.to_string(),
            operation: operation.to_string(),
        });
        
        // Clean up old transactions (older than 24 hours)
        let day_ago = now.saturating_sub(86400_000_000_000); // 24 hours
        velocity.transactions.retain(|tx| tx.timestamp > day_ago);
        
        // Recalculate totals
        let hour_ago = now.saturating_sub(3600_000_000_000); // 1 hour
        velocity.total_24h = velocity.transactions.iter().map(|tx| tx.amount).sum();
        velocity.total_1h = velocity.transactions.iter()
            .filter(|tx| tx.timestamp > hour_ago)
            .map(|tx| tx.amount)
            .sum();
        
        Ok(())
    })
}

/// Check velocity limits
pub fn check_velocity_limits(user_id: &str, amount: u64) -> Result<Vec<String>, String> {
    use crate::config;
    let cfg = config::get_config();

    VELOCITY_DATA.with(|data| {
        let data = data.borrow();
        let mut warnings = Vec::new();

        if let Some(velocity) = data.get(user_id) {
            // Check 1-hour volume
            if velocity.total_1h + amount > cfg.fraud_detection.max_1h_volume_cents {
                warnings.push(format!(
                    "High transaction volume in last hour: ${:.2}",
                    (velocity.total_1h + amount) as f64 / 100.0
                ));
            }

            // Check 24-hour volume
            if velocity.total_24h + amount > cfg.fraud_detection.max_24h_volume_cents {
                warnings.push(format!(
                    "High transaction volume in last 24 hours: ${:.2}",
                    (velocity.total_24h + amount) as f64 / 100.0
                ));
            }

            // Check transaction frequency
            let now = time();
            let hour_ago = now.saturating_sub(3600_000_000_000);
            let recent_count = velocity.transactions.iter()
                .filter(|tx| tx.timestamp > hour_ago)
                .count();

            if recent_count >= cfg.fraud_detection.max_transactions_per_hour {
                warnings.push(format!("High transaction frequency: {} transactions in last hour", recent_count));
            }
        }

        Ok(warnings)
    })
}

// ============================================================================
// DEVICE FINGERPRINTING
// ============================================================================

/// Record device fingerprint
pub fn record_device_fingerprint(user_id: &str, fingerprint: &str) -> Result<(), String> {
    DEVICE_TRACKING.with(|tracking| {
        let mut tracking = tracking.borrow_mut();
        let now = time();
        
        let devices = tracking.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // Check if device already exists
        if let Some(device) = devices.iter_mut().find(|d| d.fingerprint == fingerprint) {
            device.last_seen = now;
            device.transaction_count += 1;
        } else {
            // New device
            devices.push(DeviceInfo {
                fingerprint: fingerprint.to_string(),
                first_seen: now,
                last_seen: now,
                transaction_count: 1,
            });
        }
        
        Ok(())
    })
}

/// Check device fingerprint for suspicious activity
pub fn check_device_fingerprint(user_id: &str, fingerprint: &str) -> Result<Vec<String>, String> {
    DEVICE_TRACKING.with(|tracking| {
        let tracking = tracking.borrow();
        let mut warnings = Vec::new();
        
        if let Some(devices) = tracking.get(user_id) {
            // Check if this is a new device
            let is_new_device = !devices.iter().any(|d| d.fingerprint == fingerprint);
            
            if is_new_device {
                warnings.push("Transaction from new device".to_string());
            }
            
            // Check for too many devices
            if devices.len() > 5 {
                warnings.push(format!("User has {} registered devices", devices.len()));
            }
            
            // Check for device switching pattern (multiple devices in short time)
            let now = time();
            let hour_ago = now.saturating_sub(3600_000_000_000);
            let recent_devices: Vec<_> = devices.iter()
                .filter(|d| d.last_seen > hour_ago)
                .collect();
            
            if recent_devices.len() > 2 {
                warnings.push(format!("Multiple devices used in last hour: {}", recent_devices.len()));
            }
        }
        
        Ok(warnings)
    })
}

// ============================================================================
// GEOGRAPHIC ANALYSIS
// ============================================================================

/// Record geographic location
pub fn record_geo_location(user_id: &str, location: &str) -> Result<(), String> {
    GEO_TRACKING.with(|tracking| {
        let mut tracking = tracking.borrow_mut();
        
        let locations = tracking.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // Add location if not already present
        if !locations.contains(&location.to_string()) {
            locations.push(location.to_string());
        }
        
        // Keep only last 10 locations
        if locations.len() > 10 {
            locations.remove(0);
        }
        
        Ok(())
    })
}

/// Check geographic location for suspicious activity
pub fn check_geo_location(user_id: &str, location: &str) -> Result<Vec<String>, String> {
    GEO_TRACKING.with(|tracking| {
        let tracking = tracking.borrow();
        let mut warnings = Vec::new();
        
        if let Some(locations) = tracking.get(user_id) {
            // Check if this is a new location
            let is_new_location = !locations.contains(&location.to_string());
            
            if is_new_location && !locations.is_empty() {
                warnings.push(format!("Transaction from new location: {}", location));
            }
            
            // Check for impossible travel (different countries in short time)
            // This is a simplified check - in production, use actual geo-distance
            if locations.len() > 1 {
                let last_location = &locations[locations.len() - 1];
                if last_location != location && !location.starts_with(last_location) {
                    warnings.push(format!("Location change detected: {} -> {}", last_location, location));
                }
            }
        }
        
        Ok(warnings)
    })
}

// ============================================================================
// COMPREHENSIVE FRAUD CHECK
// ============================================================================

/// Comprehensive fraud check with all enhancements
pub fn check_transaction(
    user_id: &str,
    amount: u64,
    currency: &str,
    operation: &str,
    device_fingerprint: Option<&str>,
    geo_location: Option<&str>,
) -> Result<FraudCheckResult, String> {
    use crate::config;
    let mut warnings = Vec::new();
    let mut risk_score = 0;

    // Get config limits based on currency and operation
    let cfg = config::get_config();

    // Determine max single and daily limits based on currency
    let (max_single, max_daily) = match (currency, operation) {
        // For crypto transfers, use crypto-specific limits
        ("CkBTC" | "BTC", "send_crypto") => (
            cfg.limits.btc.max_single_transfer_satoshis,
            cfg.limits.btc.max_daily_transfer_satoshis
        ),
        ("CkUSD" | "USDC", "send_crypto") => (
            cfg.limits.usdc.max_single_transfer_cents,
            cfg.limits.usdc.max_daily_transfer_cents
        ),
        // For buy/sell operations, use fiat limits
        (_, "buy_crypto") => (
            cfg.limits.default.max_single_purchase_cents,
            cfg.limits.default.max_daily_purchase_cents
        ),
        (_, "sell_crypto") => (
            cfg.limits.default.max_single_sale_cents,
            cfg.limits.default.max_daily_sale_cents
        ),
        // Default for other operations
        _ => (
            cfg.limits.default.max_single_purchase_cents,
            cfg.limits.default.max_daily_purchase_cents
        ),
    };

    // Helper to format amounts based on currency
    let format_amount = |amt: u64| -> f64 {
        match currency {
            "CkBTC" | "BTC" => amt as f64 / 100_000_000.0,  // Satoshis: 8 decimals
            "CkUSD" | "USDC" => amt as f64 / 1_000_000.0,  // USDC: 6 decimals
            _ => amt as f64 / 100.0,  // Fiat: 2 decimals (cents)
        }
    };

    // Check 1: Amount-based risk using config limits
    // Flag if amount > 5% of max single limit (early warning)
    let warning_threshold = max_single / 20;
    if amount > warning_threshold {
        warnings.push(format!("Large transaction amount: ${:.2}", format_amount(amount)));
        risk_score += 20;
    }

    // Flag if amount > 10% of max single limit (high risk)
    let high_risk_threshold = max_single / 10;
    if amount > high_risk_threshold {
        warnings.push(format!("Very large transaction: ${:.2}", format_amount(amount)));
        risk_score += 30;
    }

    // ENFORCE hard limit - block if exceeds max single
    if amount > max_single {
        return Err(format!(
            "Transaction amount ${:.2} exceeds maximum allowed ${:.2}",
            format_amount(amount),
            format_amount(max_single)
        ));
    }
    
    // Check 2: Velocity checks (with daily limit enforcement)
    match check_velocity_limits(user_id, amount) {
        Ok(velocity_warnings) => {
            if !velocity_warnings.is_empty() {
                risk_score += 25;
                warnings.extend(velocity_warnings);
            }
        }
        Err(e) => return Err(e),
    }

    // Check 2b: Enforce daily limit from config
    VELOCITY_DATA.with(|data| {
        let data = data.borrow();
        if let Some(velocity) = data.get(user_id) {
            if velocity.total_24h + amount > max_daily {
                return Err(format!(
                    "Daily transaction limit exceeded. Total: ${:.2}, Limit: ${:.2}",
                    format_amount(velocity.total_24h + amount),
                    format_amount(max_daily)
                ));
            }
        }
        Ok::<(), String>(())
    })?;
    
    // Check 3: Device fingerprint
    if let Some(fingerprint) = device_fingerprint {
        match check_device_fingerprint(user_id, fingerprint) {
            Ok(device_warnings) => {
                if !device_warnings.is_empty() {
                    risk_score += 15;
                    warnings.extend(device_warnings);
                }
            }
            Err(e) => return Err(e),
        }
    }
    
    // Check 4: Geographic location
    if let Some(location) = geo_location {
        match check_geo_location(user_id, location) {
            Ok(geo_warnings) => {
                if !geo_warnings.is_empty() {
                    risk_score += 15;
                    warnings.extend(geo_warnings);
                }
            }
            Err(e) => return Err(e),
        }
    }
    
    // NOTE: Operation-specific rate limit is checked before calling check_transaction
    // to avoid double-counting timestamps. Each operation (buy_crypto, sell_crypto,
    // send_crypto, create_escrow) checks the rate limit explicitly before calling
    // this function, so we don't check it again here.

    // Determine action based on risk score
    let should_block = risk_score >= 80;
    let requires_manual_review = risk_score >= 50;
    let is_suspicious = !warnings.is_empty();

    // Log suspicious activity if security config requires it
    #[cfg(not(test))]
    {
        if is_suspicious && config::should_log_suspicious_activity() {
            shared_types::audit::log_success(
                "suspicious_activity_detected",
                Some(user_id.to_string()),
                format!("Operation: {} | Amount: {} {} | Risk Score: {} | Warnings: {:?} | Device: {:?} | Location: {:?}",
                    operation, amount, currency, risk_score, warnings, device_fingerprint, geo_location)
            );
        }
    }

    Ok(FraudCheckResult {
        is_suspicious,
        should_block,
        requires_manual_review,
        risk_score,
        warnings,
    })
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Check if amount is suspicious
#[allow(dead_code)]
pub fn is_suspicious_amount(amount: u64, _currency: &str) -> bool {
    use crate::config;
    let cfg = config::get_config();
    amount > cfg.fraud_detection.suspicious_amount_cents
}

/// Calculate risk score
#[allow(dead_code)]
pub fn calculate_risk_score(amount: u64, _currency: &str) -> u32 {
    use crate::config;
    let cfg = config::get_config();
    if amount > cfg.fraud_detection.high_risk_amount_cents {
        50
    } else if amount > cfg.fraud_detection.suspicious_amount_cents {
        30
    } else {
        0
    }
}

/// Determine if transaction should be blocked
#[allow(dead_code)]
pub fn should_block_transaction(risk_score: u32) -> bool {
    risk_score >= 80
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    fn test_rate_limit() {
        config::init_config();
        let cfg = config::get_config();
        let user_id = "test_user";

        // First call should succeed
        assert!(check_operation_rate_limit(user_id, "buy_crypto").unwrap());

        // Should allow up to max_buys_per_hour
        for _ in 1..cfg.fraud_detection.max_buys_per_hour {
            assert!(check_operation_rate_limit(user_id, "buy_crypto").unwrap());
        }

        // Next call should fail
        assert!(!check_operation_rate_limit(user_id, "buy_crypto").unwrap());
    }

    #[test]
    fn test_operation_rate_limit() {
        config::init_config();
        let cfg = config::get_config();
        let user_id = "test_user";

        // Should allow up to max_buys_per_hour
        for _ in 0..cfg.fraud_detection.max_buys_per_hour {
            assert!(check_operation_rate_limit(user_id, "buy_crypto").unwrap());
        }

        // Next call should fail
        assert!(!check_operation_rate_limit(user_id, "buy_crypto").unwrap());

        // Different operation should still work
        assert!(check_operation_rate_limit(user_id, "sell_crypto").unwrap());
    }
    
    #[test]
    fn test_pin_attempts_backoff() {
        config::init_config();
        let cfg = config::get_config();
        let user_id = "test_user";

        // First attempt should be allowed
        assert!(check_pin_attempts_allowed(user_id).is_ok());

        // Record failed attempts
        for _ in 0..3 {
            record_failed_pin_attempt(user_id).unwrap();
            // Advance time past backoff period (4 minutes after 3rd attempt)
            MOCK_TIME.with(|t| *t.borrow_mut() += 300_000_000_000); // +5 minutes
        }

        // Should still be allowed (under max, and past backoff)
        assert!(check_pin_attempts_allowed(user_id).is_ok());

        // Record more failures to reach max
        for _ in 0..(cfg.fraud_detection.max_pin_attempts - 3) {
            record_failed_pin_attempt(user_id).unwrap();
        }

        // Should be blocked now (max_pin_attempts = 5 by default)
        assert!(check_pin_attempts_allowed(user_id).is_err());
    }
    
    #[test]
    fn test_velocity_tracking() {
        config::init_config();
        let user_id = "test_user";

        // Record some transactions
        record_transaction(user_id, 10_000, "USD", "buy_crypto").unwrap();
        record_transaction(user_id, 20_000, "USD", "buy_crypto").unwrap();

        // Check velocity
        let warnings = check_velocity_limits(user_id, 5_000).unwrap();
        assert!(warnings.is_empty()); // Should be under limits

        // Try large amount
        let warnings = check_velocity_limits(user_id, 2_000_000).unwrap();
        assert!(!warnings.is_empty()); // Should trigger warning
    }
    
    #[test]
    fn test_device_fingerprinting() {
        let user_id = "test_user";
        let device1 = "device_fingerprint_1";
        let device2 = "device_fingerprint_2";
        
        // First device
        record_device_fingerprint(user_id, device1).unwrap();
        let warnings = check_device_fingerprint(user_id, device1).unwrap();
        assert!(warnings.is_empty()); // Known device
        
        // New device
        let warnings = check_device_fingerprint(user_id, device2).unwrap();
        assert!(!warnings.is_empty()); // Should warn about new device
    }
    
    #[test]
    fn test_geo_tracking() {
        let user_id = "test_user";
        let location1 = "US-CA";
        let location2 = "US-NY";
        
        // First location
        record_geo_location(user_id, location1).unwrap();
        let warnings = check_geo_location(user_id, location1).unwrap();
        assert!(warnings.is_empty()); // Known location
        
        // New location
        let warnings = check_geo_location(user_id, location2).unwrap();
        assert!(!warnings.is_empty()); // Should warn about new location
    }
    
    #[test]
    fn test_comprehensive_fraud_check() {
        // Initialize config for test
        config::init_config();

        let user_id = "test_user";

        // Low risk transaction
        let result = check_transaction(
            user_id,
            1_000, // $10
            "USD",
            "buy_crypto",
            Some("device_1"),
            Some("US-CA"),
        ).unwrap();
        
        assert!(!result.should_block);
        assert!(result.risk_score < 50);
        
        // High risk transaction (over threshold: 5% of 100M = 5M, so use 6M to trigger)
        let result = check_transaction(
            user_id,
            6_000_000, // $60,000 (over 5% threshold of $1M max)
            "USD",
            "buy_crypto",
            Some("new_device"),
            Some("CN"),
        ).unwrap();

        // With new thresholds, risk score comes from:
        // - Large amount (>5% of max): +20
        // - New device: +15
        // - New location: +15
        // Total expected: ~50 risk score
        assert!(result.is_suspicious);
        assert!(result.risk_score >= 20, "Expected risk_score >= 20, got {}", result.risk_score);
    }
}
