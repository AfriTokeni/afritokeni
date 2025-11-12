/// PIN Security Module
/// Uses Argon2id for secure PIN hashing (industry standard, winner of Password Hashing Competition)
/// 
/// SECURITY: Uses IC's raw_rand() for cryptographically secure random salt generation
/// This is the correct approach for WASM environments where OsRng is not available

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hash a PIN using Argon2id with cryptographically secure random salt from IC
/// 
/// Returns a PHC string format hash that includes:
/// - Algorithm identifier ($argon2id$)
/// - Version (v=19)
/// - Parameters (m, t, p)
/// - Base64-encoded salt
/// - Base64-encoded hash
/// 
/// Example output: $argon2id$v=19$m=19456,t=2,p=1$c2FsdA$hash...
pub async fn hash_pin(pin: &str) -> Result<String, String> {
    // Get cryptographically secure random bytes from IC's random beacon
    // raw_rand() provides 32 bytes of high-quality randomness
    let random_bytes = ic_cdk::management_canister::raw_rand()
        .await
        .map_err(|err| format!("Failed to get secure randomness from IC: {:?}", err))?;
    
    // Extract 16 bytes for salt (Argon2 standard salt length)
    let salt_bytes: [u8; 16] = random_bytes[..16]
        .try_into()
        .map_err(|_| "Failed to extract salt bytes".to_string())?;
    
    // Encode salt as base64 SaltString (required by password_hash crate)
    let salt = SaltString::encode_b64(&salt_bytes)
        .map_err(|e| format!("Failed to encode salt: {}", e))?;
    
    // Use Argon2id with default secure parameters:
    // - Memory: 19 MiB (m=19456 KiB)
    // - Iterations: 2 (t=2)
    // - Parallelism: 1 (p=1)
    let argon2 = Argon2::default();
    
    // Hash the PIN and get PHC string format
    let password_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| format!("Argon2 hashing failed: {}", e))?;
    
    // Return the complete PHC string (includes all params + salt + hash)
    Ok(password_hash.to_string())
}

/// Verify a PIN against a stored hash
/// The hash string contains all necessary information (salt, params, etc.)
pub fn verify_pin(pin: &str, hash: &str) -> Result<bool, String> {
    // Parse the stored hash
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("Invalid hash format: {}", e))?;
    
    // Use Argon2id for verification
    let argon2 = Argon2::default();
    
    // Verify the PIN
    match argon2.verify_password(pin.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

// NOTE: Unit tests removed because hash_pin() requires IC canister environment (raw_rand())
// PIN security is tested in integration tests instead (tests/integration/pin_security_tests.rs)
//
// The hash_pin function uses ic_cdk::management_canister::raw_rand() which only works
// when running in an actual IC canister, not in regular unit tests.
// Integration tests with PocketIC provide full coverage of PIN hashing and verification.
