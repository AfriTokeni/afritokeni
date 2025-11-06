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
