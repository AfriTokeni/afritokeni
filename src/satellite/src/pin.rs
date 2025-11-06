use crate::juno_store;
use crate::session::UssdSession;
use crate::translations::{Language, TranslationService};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

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
    pin.len() >= 4 && pin.len() <= 6 && pin.chars().all(|c| c.is_numeric())
}

/// Check if user has verified their PIN in this session
pub fn is_pin_verified(session: &UssdSession) -> bool {
    session.data.get("pin_verified").map(|v| v == "true").unwrap_or(false)
}

/// Mark PIN as verified in session
pub fn mark_pin_verified(session: &mut UssdSession) {
    session.set_data("pin_verified", "true");
}

/// Request PIN verification
pub fn request_pin_verification(session: &mut UssdSession, action: &str) -> String {
    let lang = Language::from_code(&session.language);
    session.set_data("pending_action", action);
    session.set_data("awaiting_pin", "true");
    session.step = 999; // Special step for PIN entry
    
    format!("{}\n{}", 
        TranslationService::translate("enter_pin", lang),
        TranslationService::translate("pin_4_6_digits", lang))
}

/// Verify user's PIN
pub async fn verify_user_pin(phone: &str, pin: &str) -> Result<bool, String> {
    // Validate PIN format
    if !is_valid_pin(pin) {
        return Ok(false);
    }
    
    // Get stored PIN hash
    match juno_store::get_user_pin(phone).await {
        Ok(hash) => Ok(verify_pin_hash(pin, &hash)),
        Err(_) => {
            // No PIN set - user must set up PIN first
            Err("PIN not set. Please set up your PIN first.".to_string())
        }
    }
}

/// Handle PIN entry step
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
            // PIN correct - mark as verified and continue with pending action
            mark_pin_verified(session);
            session.data.remove("awaiting_pin");
            let _action = session.get_data("pending_action").cloned().unwrap_or_default();
            session.data.remove("pending_action");
            
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
    juno_store::set_user_pin(phone, &hash).await
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
