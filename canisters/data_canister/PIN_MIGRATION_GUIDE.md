# PIN Migration Guide: HMAC-SHA256 → Argon2id

**Status**: HMAC-SHA256 PIN hashing is DEPRECATED as of v0.2.0
**Timeline**: HMAC functions will be removed in v2.0.0 (Q1 2026)
**Security Priority**: CRITICAL

---

## Why Migrate?

### Security Vulnerability: HMAC-SHA256 for PINs

**Problem**: HMAC-SHA256 is NOT a password hashing algorithm. It was designed for message authentication, not password storage.

**Risk**: If PIN hashes leak (e.g., database breach, memory dump), an attacker can:
- Brute-force all 10,000 four-digit PINs in **seconds**
- Brute-force all 1,000,000 six-digit PINs in **minutes**
- Use GPU/ASIC hardware to accelerate attacks

**Why HMAC is Fast (and Dangerous)**:
```
HMAC-SHA256 speed: ~10 million hashes/second on modern CPU
Time to crack 4-digit PIN: 10,000 / 10,000,000 = 0.001 seconds
Time to crack 6-digit PIN: 1,000,000 / 10,000,000 = 0.1 seconds
```

### Solution: Argon2id Password Hashing

**Argon2id** is the industry-standard password hashing algorithm (OWASP recommended):
- **Memory-hard**: Requires significant RAM (resistant to GPU/ASIC attacks)
- **Time-cost**: Configurable iterations (makes brute-force prohibitively expensive)
- **Parallelism**: Resistant to parallel hardware attacks
- **Winner of Password Hashing Competition (2015)**

**Security Comparison**:
```
Argon2id speed: ~10 hashes/second (configurable)
Time to crack 4-digit PIN: 10,000 / 10 = 1,000 seconds (~17 minutes)
Time to crack 6-digit PIN: 1,000,000 / 10 = 100,000 seconds (~28 hours)
```

With proper Argon2 tuning (higher time cost), cracking becomes infeasible even for 4-digit PINs.

---

## Architecture Change

### OLD Architecture (HMAC-SHA256 - DEPRECATED)

```
┌─────────────────────────────────────────────────────────────┐
│ USSD Canister                                                │
├─────────────────────────────────────────────────────────────┤
│ 1. User enters PIN: "1234"                                   │
│ 2. Generate salt: random_bytes(32)                          │
│ 3. Call data_canister.setup_user_pin(pin, salt)             │
│    ↓                                                         │
│    data_canister hashes PIN with HMAC-SHA256 (FAST!)        │
│    ↓                                                         │
│    Stores: {pin_hash, salt} in user_pins table              │
└─────────────────────────────────────────────────────────────┘

PROBLEM: HMAC-SHA256 is too fast, vulnerable to brute-force
```

### NEW Architecture (Argon2id - RECOMMENDED)

```
┌─────────────────────────────────────────────────────────────┐
│ user_canister (Business Logic Layer)                         │
├─────────────────────────────────────────────────────────────┤
│ 1. User enters PIN: "1234"                                   │
│ 2. Generate salt: SaltString::generate(&mut OsRng)          │
│ 3. Hash PIN with Argon2id (SLOW by design - SECURE!)        │
│    let argon2 = Argon2::default();                          │
│    let hash = argon2.hash_password(pin.as_bytes(), &salt)   │
│    ↓                                                         │
│ 4. Call data_canister.store_pin_hash(user_id, hash)         │
│    ↓                                                         │
│    data_canister stores: {pin_hash} (salt embedded in hash) │
└─────────────────────────────────────────────────────────────┘

BENEFIT: Memory-hard algorithm prevents fast brute-force attacks
```

---

## Migration Path

### For New Users (Immediate)

**Use Argon2id from day 1:**

```rust
// In user_canister (business logic):
use argon2::{
    Argon2,
    PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng}
};

async fn setup_pin_for_new_user(user_id: String, pin: String) -> Result<(), String> {
    // 1. Generate salt
    let salt = SaltString::generate(&mut OsRng);

    // 2. Hash PIN with Argon2id
    let argon2 = Argon2::default();
    let pin_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| format!("Argon2 hash failed: {}", e))?
        .to_string();

    // 3. Store hash in data_canister
    let (result,): (Result<(), String>,) = ic_cdk::call(
        data_canister_id(),
        "store_pin_hash",
        (user_id, pin_hash)
    ).await
    .map_err(|e| format!("Failed to store PIN hash: {:?}", e))?;

    result
}
```

### For Existing Users (Gradual Migration)

**Migrate on next PIN change or login:**

```rust
// In user_canister:
async fn verify_pin_with_migration(user_id: String, pin: String) -> Result<bool, String> {
    // 1. Get stored PIN hash
    let (stored_hash,): (Result<String, String>,) = ic_cdk::call(
        data_canister_id(),
        "get_pin_hash",
        (user_id.clone(),)
    ).await
    .map_err(|e| format!("Failed to get PIN hash: {:?}", e))??;

    // 2. Detect which system is in use
    let is_argon2 = stored_hash.starts_with("$argon2");

    if is_argon2 {
        // NEW SYSTEM: Verify using Argon2
        let parsed_hash = PasswordHash::new(&stored_hash)
            .map_err(|e| format!("Invalid hash format: {}", e))?;

        if Argon2::default().verify_password(pin.as_bytes(), &parsed_hash).is_ok() {
            // Success - reset failed attempts
            ic_cdk::call::<_, ()>(
                data_canister_id(),
                "reset_pin_attempts",
                (user_id,)
            ).await
            .map_err(|e| format!("Failed to reset attempts: {:?}", e))?;

            Ok(true)
        } else {
            // Failed - increment attempts
            ic_cdk::call::<_, ()>(
                data_canister_id(),
                "increment_failed_attempts",
                (user_id,)
            ).await
            .map_err(|e| format!("Failed to increment attempts: {:?}", e))?;

            Ok(false)
        }
    } else {
        // OLD SYSTEM: Use deprecated HMAC verification, then migrate
        #[allow(deprecated)]
        let (verified,): (Result<bool, String>,) = ic_cdk::call(
            data_canister_id(),
            "verify_user_pin",
            (user_id.clone(), pin.clone())
        ).await
        .map_err(|e| format!("Failed to verify PIN: {:?}", e))??;

        if verified {
            // SUCCESS: Migrate to Argon2 immediately (transparent upgrade)
            ic_cdk::println!("⚠️ Migrating user {} from HMAC to Argon2", user_id);

            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let new_hash = argon2
                .hash_password(pin.as_bytes(), &salt)
                .map_err(|e| format!("Argon2 hash failed: {}", e))?
                .to_string();

            ic_cdk::call::<_, ()>(
                data_canister_id(),
                "store_pin_hash",
                (user_id.clone(), new_hash)
            ).await
            .map_err(|e| format!("Failed to migrate PIN: {:?}", e))?;

            ic_cdk::println!("✅ User {} migrated to Argon2", user_id);
        }

        Ok(verified)
    }
}
```

---

## Deprecated Functions

The following functions are DEPRECATED and will be removed in v2.0.0:

### 1. `setup_user_pin` (Canister Endpoint)
**Status**: Deprecated since v0.2.0
**Replacement**: `store_pin_hash`
**Migration**: Hash PIN in user_canister with Argon2, then call `store_pin_hash`

### 2. `verify_user_pin` (Canister Endpoint)
**Status**: Deprecated since v0.2.0
**Replacement**: `get_pin_hash` + Argon2 verification in user_canister
**Migration**: See "Existing Users" section above

### 3. `change_pin` (Canister Endpoint)
**Status**: Deprecated since v0.2.0
**Replacement**: Implement PIN change in user_canister
**Migration**: Verify old PIN + hash new PIN with Argon2 + call `store_pin_hash`

### 4. `setup_pin_with_salt` (Internal Function)
**Location**: `src/security/pin_ops.rs`
**Status**: Deprecated since v0.2.0
**Replacement**: `store_pin_hash`

### 5. `verify_pin` (Internal Function)
**Location**: `src/security/pin_ops.rs`
**Status**: Deprecated since v0.2.0
**Replacement**: Argon2 verification in user_canister

### 6. `change_pin` (Internal Function)
**Location**: `src/security/pin_ops.rs`
**Status**: Deprecated since v0.2.0
**Replacement**: Implement in user_canister

---

## Recommended Argon2 Parameters

### Production Settings (High Security)
```rust
use argon2::{Argon2, Algorithm, Version, Params};

let params = Params::new(
    65536,  // m_cost (memory): 64 MB
    3,      // t_cost (iterations): 3 passes
    4,      // p_cost (parallelism): 4 threads
    None    // output_len: default (32 bytes)
)?;

let argon2 = Argon2::new(
    Algorithm::Argon2id,
    Version::V0x13,
    params
);
```

**Benchmark**: ~100-200ms on modern hardware (good balance of security and UX)

### Testing Settings (Lower Security, Faster)
```rust
let params = Params::new(
    19456,  // m_cost: 19 MB
    2,      // t_cost: 2 passes
    1,      // p_cost: 1 thread
    None
)?;
```

**Benchmark**: ~10-20ms on modern hardware (faster for tests)

---

## Migration Timeline

### Phase 1: Immediate (v0.2.0) - CURRENT
- ✅ Add deprecation warnings to HMAC functions
- ✅ Document security vulnerability
- ✅ Provide migration guide
- ✅ Implement Argon2 storage functions (`store_pin_hash`, `get_pin_hash`)

### Phase 2: Q4 2025 (v0.3.0)
- Implement automatic migration on login (user_canister)
- Add migration progress metrics (% of users migrated)
- Monitor for HMAC usage in production logs

### Phase 3: Q1 2026 (v1.0.0)
- Target: 90%+ users migrated to Argon2
- Deprecation warnings become errors
- Disable HMAC endpoints for new users

### Phase 4: Q2 2026 (v2.0.0)
- Remove HMAC-SHA256 functions entirely
- Remove deprecated canister endpoints
- Clean up migration code

---

## Testing Migration

### Unit Test Example
```rust
#[test]
fn test_argon2_pin_verification() {
    use argon2::{Argon2, PasswordHasher, PasswordVerifier};
    use argon2::password_hash::{SaltString, rand_core::OsRng};

    let pin = "123456";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash PIN
    let hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Verify correct PIN
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    assert!(argon2.verify_password(pin.as_bytes(), &parsed_hash).is_ok());

    // Verify incorrect PIN
    assert!(argon2.verify_password(b"wrong", &parsed_hash).is_err());
}
```

### Integration Test Example
```rust
#[test]
fn test_pin_migration_flow() {
    let pic = PocketIc::new();
    let data_canister = deploy_data_canister(&pic);
    let user_canister = deploy_user_canister(&pic);

    // 1. Create user with OLD HMAC system
    let user_id = "user123".to_string();
    let pin = "1234".to_string();

    #[allow(deprecated)]
    data_canister.setup_user_pin(user_id.clone(), pin.clone(), salt).await.unwrap();

    // 2. Verify PIN (should trigger migration)
    let verified = user_canister.verify_pin(user_id.clone(), pin.clone()).await.unwrap();
    assert!(verified);

    // 3. Verify hash is now Argon2 format
    let hash = data_canister.get_pin_hash(user_id).await.unwrap();
    assert!(hash.starts_with("$argon2"));
}
```

---

## Security Checklist

Before removing HMAC functions in v2.0.0:

- [ ] 90%+ users migrated to Argon2 (check via metrics)
- [ ] No HMAC endpoints called in last 30 days (check audit logs)
- [ ] All canisters updated to use Argon2 flow (user_canister, ussd_canister)
- [ ] Migration tests passing (unit + integration)
- [ ] Argon2 performance acceptable (<200ms for PIN verification)
- [ ] Backward compatibility no longer required
- [ ] Security audit completed on Argon2 implementation

---

## Support

**Questions**: See `/Users/sdicola/CascadeProjects/afritokeni-mvp/CLAUDE.md`
**Security Issues**: Report to platform admin immediately
**Migration Help**: Check integration tests in `canisters/data_canister/tests/integration/`

---

*This migration guide is based on the security audit recommendations in `SECURITY_AUDIT.md` (lines 390-423).*
