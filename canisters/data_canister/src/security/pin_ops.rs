// ============================================================================
// PIN SECURITY MODULE - Argon2id System
// ============================================================================
//
// SECURITY ARCHITECTURE:
//
// This module provides secure PIN management using Argon2id password hashing.
//
// Argon2id Benefits:
//    - Memory-hard algorithm designed for password hashing
//    - Resistant to GPU/ASIC attacks
//    - Hashing performed in user_canister (business logic layer)
//    - data_canister provides pure storage only
//    - OWASP recommended for password hashing
//
// Usage:
// - New PINs: Use store_pin_hash() with Argon2 hashes from user_canister
// - Verification: Get hash via get_pin_hash(), verify in user_canister
// - Failed attempts: Track via increment_failed_attempts()/reset_attempts()
// - Lockout: Automatic after 3 failed attempts for 30 minutes
//
// ============================================================================

use crate::models::*;
use crate::DataCanisterState;
use ic_cdk::api::time;
use shared_types::audit;

const MAX_PIN_ATTEMPTS: u32 = 3;
const PIN_LOCKOUT_DURATION: u64 = 30 * 60; // 30 minutes in seconds

/// Reset PIN attempts (admin function)
pub fn reset_attempts(
    state: &mut DataCanisterState,
    user_id: String,
) -> Result<(), String> {
    let now = time() / 1_000_000_000;

    let user_pin = state.user_pins.get_mut(&user_id)
        .ok_or("PIN not set for user")?;

    user_pin.failed_attempts = 0;
    user_pin.locked_until = None;
    user_pin.updated_at = now;

    // Log audit using shared library
    audit::log_success(
        "pin_attempts_reset",
        Some(user_id),
        "PIN attempts reset".to_string()
    );

    Ok(())
}

/// Check if PIN is locked
pub fn is_pin_locked(
    state: &DataCanisterState,
    user_id: String,
) -> Result<bool, String> {
    let now = time() / 1_000_000_000;

    let user_pin = state.user_pins.get(&user_id)
        .ok_or("PIN not set for user")?;

    if let Some(locked_until) = user_pin.locked_until {
        Ok(now < locked_until)
    } else {
        Ok(false)
    }
}

/// Get remaining lockout time in seconds
pub fn get_remaining_lockout_time(
    state: &DataCanisterState,
    user_id: String,
) -> Result<u64, String> {
    let now = time() / 1_000_000_000;

    let user_pin = state.user_pins.get(&user_id)
        .ok_or("PIN not set for user")?;

    if let Some(locked_until) = user_pin.locked_until {
        if now < locked_until {
            Ok(locked_until - now)
        } else {
            Ok(0)
        }
    } else {
        Ok(0)
    }
}

/// Get failed attempt count
pub fn get_failed_attempts(
    state: &DataCanisterState,
    user_id: String,
) -> Result<u32, String> {
    let user_pin = state.user_pins.get(&user_id)
        .ok_or("PIN not set for user")?;

    Ok(user_pin.failed_attempts)
}

// ============================================================================
// MODERN ARGON2 SYSTEM (RECOMMENDED)
// ============================================================================
//
// ARCHITECTURE:
// 1. PIN hashing happens in user_canister using Argon2id algorithm
// 2. data_canister provides pure storage (no hashing logic)
// 3. PIN verification happens in user_canister using argon2::verify_hash
// 4. Failed attempt tracking still managed by data_canister
//
// SECURITY BENEFITS:
// - Memory-hard algorithm (resistant to hardware attacks)
// - Configurable cost parameters (time, memory, parallelism)
// - Salt embedded in hash string (no separate salt storage)
// - Industry-standard password hashing (OWASP recommended)
//
// USAGE PATTERN:
// ```rust
// // In user_canister (business logic):
// use argon2::{Argon2, PasswordHasher};
// let salt = SaltString::generate(&mut OsRng);
// let argon2 = Argon2::default();
// let pin_hash = argon2.hash_password(pin.as_bytes(), &salt)?.to_string();
//
// // Store in data_canister:
// data_canister.store_pin_hash(user_id, pin_hash).await?;
//
// // Verify PIN in user_canister:
// let stored_hash = data_canister.get_pin_hash(user_id).await?;
// let parsed_hash = PasswordHash::new(&stored_hash)?;
// if argon2.verify_password(pin.as_bytes(), &parsed_hash).is_ok() {
//     // PIN correct - reset failed attempts
//     data_canister.reset_pin_attempts(user_id).await?;
// } else {
//     // PIN incorrect - increment failed attempts
//     data_canister.increment_failed_attempts(user_id).await?;
// }
// ```
//
// ============================================================================

/// Store PIN hash (Argon2 hash from user_canister)
/// This is pure storage - no hashing logic here
pub fn store_pin_hash(
    state: &mut DataCanisterState,
    user_id: String,
    pin_hash: String,
) -> Result<(), String> {
    let now = time() / 1_000_000_000;

    // Check if user exists
    if !state.users.contains_key(&user_id) {
        return Err("User not found".to_string());
    }

    // Create or update PIN record
    // Note: Argon2 hash includes embedded salt - no separate salt field needed
    let user_pin = UserPin {
        user_id: user_id.clone(),
        pin_hash,
        failed_attempts: 0,
        locked_until: None,
        created_at: now,
        updated_at: now,
    };

    state.user_pins.insert(user_id.clone(), user_pin);

    // Log audit using shared library
    audit::log_success(
        "pin_hash_stored",
        Some(user_id),
        "PIN hash stored (Argon2)".to_string()
    );

    Ok(())
}

/// Get PIN hash for verification
pub fn get_pin_hash(
    state: &DataCanisterState,
    user_id: String,
) -> Result<String, String> {
    let user_pin = state.user_pins.get(&user_id)
        .ok_or_else(|| format!("PIN not set for user: {}", user_id))?;
    
    Ok(user_pin.pin_hash.clone())
}

/// Increment failed PIN attempts and handle lockout
pub fn increment_failed_attempts(
    state: &mut DataCanisterState,
    user_id: String,
) -> Result<(), String> {
    let now = time() / 1_000_000_000;
    
    let mut user_pin = state.user_pins.get(&user_id)
        .ok_or_else(|| format!("PIN not set for user: {}", user_id))?
        .clone();
    
    user_pin.failed_attempts += 1;
    user_pin.updated_at = now;
    
    // Lock account after MAX_PIN_ATTEMPTS
    if user_pin.failed_attempts >= MAX_PIN_ATTEMPTS {
        user_pin.locked_until = Some(now + PIN_LOCKOUT_DURATION);
    }
    
    state.user_pins.insert(user_id.clone(), user_pin.clone());
    
    // Log audit using shared library
    audit::log_failure(
        "pin_attempt_failed",
        Some(user_id),
        format!("Failed PIN attempt #{}", user_pin.failed_attempts)
    );
    
    Ok(())
}
