use cucumber::{given, when, then, World};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

// Import REAL satellite functions
// TODO: Make these public in satellite or use a test interface
fn is_valid_pin(pin: &str) -> bool {
    pin.len() >= 4 && pin.len() <= 6 && pin.chars().all(|c| c.is_numeric())
}

fn hash_pin_with_phone(pin: &str, phone: &str) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt = SaltString::encode_b64(phone.as_bytes())
        .map_err(|e| format!("Failed to create salt: {}", e))?;
    let password_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash PIN: {}", e))?;
    Ok(password_hash.to_string())
}

fn verify_pin_hash(pin: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(pin.as_bytes(), &parsed_hash)
        .is_ok()
}

#[derive(Debug, Default, World)]
pub struct PinWorld {
    pin: String,
    phone: String,
    hash: String,
    hash2: String,
    is_valid: bool,
    verification_result: bool,
}

// GIVEN steps
#[given(expr = "a PIN {string}")]
async fn given_pin(world: &mut PinWorld, pin: String) {
    world.pin = pin;
}

#[given(expr = "a phone number {string}")]
async fn given_phone(world: &mut PinWorld, phone: String) {
    world.phone = phone;
}

// WHEN steps
#[when("I validate the PIN")]
async fn validate_pin(world: &mut PinWorld) {
    world.is_valid = is_valid_pin(&world.pin);
}

#[when("I hash the PIN")]
async fn hash_pin(world: &mut PinWorld) {
    world.hash = hash_pin_with_phone(&world.pin, &world.phone)
        .expect("Hashing should succeed");
}

#[when(expr = "I hash the PIN for phone {string}")]
async fn hash_pin_for_phone(world: &mut PinWorld, phone: String) {
    if world.hash.is_empty() {
        world.hash = hash_pin_with_phone(&world.pin, &phone)
            .expect("Hashing should succeed");
    } else {
        world.hash2 = hash_pin_with_phone(&world.pin, &phone)
            .expect("Hashing should succeed");
    }
}

#[when("I hash the PIN twice")]
async fn hash_pin_twice(world: &mut PinWorld) {
    world.hash = hash_pin_with_phone(&world.pin, &world.phone)
        .expect("First hash should succeed");
    world.hash2 = hash_pin_with_phone(&world.pin, &world.phone)
        .expect("Second hash should succeed");
}

#[when("I verify the PIN with the hash")]
async fn verify_pin(world: &mut PinWorld) {
    world.verification_result = verify_pin_hash(&world.pin, &world.hash);
}

#[when(expr = "I verify PIN {string} with the hash")]
async fn verify_different_pin(world: &mut PinWorld, pin: String) {
    world.verification_result = verify_pin_hash(&pin, &world.hash);
}

// THEN steps
#[then("the PIN should be valid")]
async fn pin_should_be_valid(world: &mut PinWorld) {
    assert!(world.is_valid, "Expected PIN '{}' to be valid", world.pin);
}

#[then("the PIN should be invalid")]
async fn pin_should_be_invalid(world: &mut PinWorld) {
    assert!(!world.is_valid, "Expected PIN '{}' to be invalid", world.pin);
}

#[then(expr = "the hash should start with {string}")]
async fn hash_should_start_with(world: &mut PinWorld, prefix: String) {
    assert!(
        world.hash.starts_with(&prefix),
        "Expected hash to start with '{}', got: {}",
        prefix,
        world.hash
    );
}

#[then("the verification should succeed")]
async fn verification_should_succeed(world: &mut PinWorld) {
    assert!(world.verification_result, "Expected PIN verification to succeed");
}

#[then("the verification should fail")]
async fn verification_should_fail(world: &mut PinWorld) {
    assert!(!world.verification_result, "Expected PIN verification to fail");
}

#[then("the hashes should be different")]
async fn hashes_should_be_different(world: &mut PinWorld) {
    assert_ne!(
        world.hash, world.hash2,
        "Expected different hashes for different users"
    );
}

#[then("both hashes should be identical")]
async fn hashes_should_be_identical(world: &mut PinWorld) {
    assert_eq!(
        world.hash, world.hash2,
        "Expected identical hashes for same user"
    );
}
