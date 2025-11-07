use crate::utils::datastore;
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::config_loader::get_config;

#[derive(Clone)]
#[allow(dead_code)]
struct PinAttempt {
    count: u32,
    lockout_until: u64,
}

thread_local! {
    static PIN_ATTEMPTS: RefCell<HashMap<String, PinAttempt>> = RefCell::new(HashMap::new());
}

/// Hash PIN using Argon2id with per-user salt
/// Uses phone number as salt to ensure each user has unique hash
/// Even with 4-6 digit PINs, Argon2's computational cost prevents rainbow tables
pub fn hash_pin_with_phone(pin: &str, phone: &str) -> Result<String, String> {
    let argon2 = Argon2::default();
    
    // Use phone number as salt (deterministic but unique per user)
    // This prevents rainbow table attacks while keeping it deterministic
    let salt = SaltString::encode_b64(phone.as_bytes())
        .map_err(|e| format!("Failed to create salt: {}", e))?;
    
    let password_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash PIN: {}", e))?;
    
    Ok(password_hash.to_string())
}

/// Verify PIN matches stored Argon2 hash
#[allow(dead_code)]
pub fn verify_pin_hash(pin: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    
    Argon2::default()
        .verify_password(pin.as_bytes(), &parsed_hash)
        .is_ok()
}

/// Validate PIN format (4-6 digits)
pub fn is_valid_pin(pin: &str) -> bool {
    let config = get_config();
    pin.len() >= config.pin_security.min_pin_length && 
    pin.len() <= config.pin_security.max_pin_length && 
    pin.chars().all(|c| c.is_numeric())
}

/// Check if phone number is locked out from PIN attempts
#[allow(dead_code)]
pub fn check_pin_lockout(phone: &str) -> Result<(), String> {
    let current_time = ic_cdk::api::time();
    
    PIN_ATTEMPTS.with(|attempts| {
        if let Some(attempt) = attempts.borrow().get(phone) {
            if current_time < attempt.lockout_until {
                let remaining_secs = (attempt.lockout_until - current_time) / 1_000_000_000;
                return Err(format!("Too many failed attempts. Try again in {} minutes", remaining_secs / 60));
            }
        }
        Ok(())
    })
}

/// Record failed PIN attempt
#[allow(dead_code)]
pub fn record_failed_attempt(phone: &str) {
    let config = get_config();
    let current_time = ic_cdk::api::time();
    let lockout_nanos = config.pin_security.lockout_duration_minutes * 60 * 1_000_000_000;
    
    PIN_ATTEMPTS.with(|attempts| {
        let mut attempts_map = attempts.borrow_mut();
        
        let attempt = attempts_map.entry(phone.to_string()).or_insert(PinAttempt {
            count: 0,
            lockout_until: 0,
        });
        
        attempt.count += 1;
        
        if attempt.count >= config.pin_security.max_pin_attempts {
            attempt.lockout_until = current_time + lockout_nanos;
            ic_cdk::println!("🔒 Locked out {} for {} minutes after {} failed attempts", 
                phone, config.pin_security.lockout_duration_minutes, attempt.count);
        }
    });
}

/// Reset PIN attempts after successful verification
#[allow(dead_code)]
pub fn reset_pin_attempts(phone: &str) {
    PIN_ATTEMPTS.with(|attempts| {
        attempts.borrow_mut().remove(phone);
    });
}

/// Request PIN verification
#[allow(dead_code)]
pub fn request_pin_verification(session: &mut UssdSession, _action: &str) -> String {
    let lang = Language::from_code(&session.language);
    session.step = 999; // Special step for PIN entry
    
    format!("{}\n{}", 
        TranslationService::translate("enter_pin", lang),
        TranslationService::translate("pin_4_6_digits", lang))
}

/// Verify user's PIN
#[allow(dead_code)]
pub async fn verify_user_pin(phone: &str, pin: &str) -> Result<bool, String> {
    // Check if locked out
    check_pin_lockout(phone)?;
    
    // Validate PIN format
    if !is_valid_pin(pin) {
        record_failed_attempt(phone);
        return Ok(false);
    }
    
    // Get stored PIN hash
    match datastore::get_user_pin(phone).await {
        Ok(hash) => {
            let is_valid = verify_pin_hash(pin, &hash);
            if is_valid {
                reset_pin_attempts(phone);
            } else {
                record_failed_attempt(phone);
            }
            Ok(is_valid)
        },
        Err(_) => {
            // No PIN set - user must set up PIN first
            Err("PIN not set. Please set up your PIN first.".to_string())
        }
    }
}

/// Handle PIN entry step
#[allow(dead_code)]
pub async fn handle_pin_entry(
    pin: &str,
    session: &mut UssdSession,
) -> Result<(String, bool), String> {
    let lang = Language::from_code(&session.language);
    
    // Validate PIN format
    if !is_valid_pin(pin) {
        return Ok((
            format!("{}\n{}", 
                TranslationService::translate("invalid_pin", lang),
                TranslationService::translate("pin_4_6_digits", lang)),
            true
        ));
    }
    
    // Verify PIN
    match verify_user_pin(&session.phone_number, pin).await {
        Ok(true) => {
            // PIN correct - reset step and continue
            session.step = 0;
            
            // Return success message and let the flow continue
            Ok((
                TranslationService::translate("pin_verified", lang).to_string(),
                true
            ))
        }
        Ok(false) => {
            // PIN incorrect
            Ok((
                format!("{}\n{}", 
                    TranslationService::translate("incorrect_pin", lang),
                    TranslationService::translate("try_again", lang)),
                true
            ))
        }
        Err(e) => Err(format!("PIN verification error: {}", e)),
    }
}

/// Setup new PIN for user
pub async fn setup_pin(phone: &str, pin: &str) -> Result<(), String> {
    if !is_valid_pin(pin) {
        return Err("Invalid PIN format".to_string());
    }
    
    let hash = hash_pin_with_phone(pin, phone)?;
    datastore::set_user_pin(phone, &hash).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_pin() {
        // Valid PINs
        assert!(is_valid_pin("1234"));
        assert!(is_valid_pin("123456"));
        assert!(is_valid_pin("0000"));
        
        // Invalid PINs
        assert!(!is_valid_pin("123")); // Too short
        assert!(!is_valid_pin("1234567")); // Too long
        assert!(!is_valid_pin("12a4")); // Contains letter
        assert!(!is_valid_pin("12 4")); // Contains space
        assert!(!is_valid_pin("")); // Empty
    }

    #[test]
    fn test_hash_pin_with_phone() {
        let phone = "+256700123456";
        let pin = "1234";
        
        // Hash should succeed
        let hash = hash_pin_with_phone(pin, phone).expect("Hashing should succeed");
        
        // Hash should not be empty
        assert!(!hash.is_empty());
        
        // Hash should start with $argon2
        assert!(hash.starts_with("$argon2"));
        
        // Same PIN with same phone should produce same hash
        let hash2 = hash_pin_with_phone(pin, phone).expect("Hashing should succeed");
        assert_eq!(hash, hash2);
        
        // Same PIN with different phone should produce different hash
        let hash3 = hash_pin_with_phone(pin, "+256700999999").expect("Hashing should succeed");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_verify_pin_hash() {
        let phone = "+256700123456";
        let correct_pin = "1234";
        let wrong_pin = "5678";
        
        let hash = hash_pin_with_phone(correct_pin, phone).expect("Hashing should succeed");
        
        // Correct PIN should verify
        assert!(verify_pin_hash(correct_pin, &hash));
        
        // Wrong PIN should not verify
        assert!(!verify_pin_hash(wrong_pin, &hash));
        
        // Invalid hash should not verify
        assert!(!verify_pin_hash(correct_pin, "invalid_hash"));
    }

    #[test]
    fn test_pin_security() {
        // Test that same PIN for different users produces different hashes
        let pin = "1234";
        let user1 = "+256700111111";
        let user2 = "+256700222222";
        
        let hash1 = hash_pin_with_phone(pin, user1).expect("Hashing should succeed");
        let hash2 = hash_pin_with_phone(pin, user2).expect("Hashing should succeed");
        
        // Hashes MUST be different (prevents rainbow table attacks)
        assert_ne!(hash1, hash2, "Same PIN for different users must produce different hashes");
        
        // Each hash should verify with correct PIN
        assert!(verify_pin_hash(pin, &hash1));
        assert!(verify_pin_hash(pin, &hash2));
    }

    #[test]
    fn test_pin_brute_force_resistance() {
        // Test that Argon2 makes brute force expensive
        let phone = "+256700123456";
        let pin = "1234";
        
        let start = std::time::Instant::now();
        let _hash = hash_pin_with_phone(pin, phone).expect("Hashing should succeed");
        let duration = start.elapsed();
        
        // Argon2 should take at least 10ms (prevents fast brute force)
        assert!(duration.as_millis() >= 10, "Hashing should be computationally expensive");
    }
}
