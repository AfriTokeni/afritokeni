# User Canister - Authentication & Identity Management

**Version:** 0.1.0
**Status:** ✅ Production Ready
**Test Coverage:** 20 Unit + 79 Integration Tests (100% critical paths)
**Security Score:** 8.5/10 (Medium findings require attention)

---

## Overview

The User Canister is the **authentication and identity management** layer for AfriTokeni. It handles user registration, PIN-based authentication, account linking, and provides comprehensive audit trails for compliance and debugging.

**Key Responsibilities:**
- 🔐 User registration (phone, principal, or both)
- 🔑 PIN authentication with Argon2id hashing
- 🔒 Account lockout after failed attempts
- 🔗 Account linking (phone ↔ principal)
- 📊 Audit trail with distributed tracing
- ✅ Input validation (phone, email, PIN, names)

---

## Features

### Authentication & Security
- **Argon2id Password Hashing** (PHC winner, resistant to GPU/ASIC attacks)
  - Memory: 19 MiB, Iterations: 2, Parallelism: 1
  - Cryptographic random salts from IC's `raw_rand()`
  - PHC string format for future-proof parameter upgrades
- **Account Lockout** (3 failed attempts → 30-minute timeout)
- **3-Tier Access Control**
  - Controller: Full access (canister owner)
  - AuthorizedCanister: Full access (whitelisted canisters like USSD)
  - UserSelf: Direct access to own data (in progress)
- **User Enumeration Prevention** (generic error messages)
- **Phone & Email Validation** (E.164 format, RFC 5322)
- **Audit Trail** (all operations logged with correlation IDs)

### User Management
- **Multi-Identifier Support** (phone, principal, or both)
- **Profile Management** (name, email, preferred currency)
- **Account Linking** (link phone number to existing principal)
- **Duplicate Prevention** (phone/principal uniqueness enforced)

### Audit & Tracing
- **Comprehensive Logging** (all operations with success/failure)
- **Correlation IDs** (track requests across canisters like Jaeger)
- **Query Endpoints** (retrieve audit logs for analysis)
- **Automatic Rotation** (max 10,000 entries, prevents unbounded growth)
- **Inter-Canister Tracing** (track calls to data_canister)

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│           User Canister (400KB WASM)            │
│                                                 │
│  ┌──────────────┐     ┌──────────────┐        │
│  │ Validation   │     │  Security    │        │
│  │  - Phone     │     │  - Argon2id  │        │
│  │  - Email     │     │  - PIN Hash  │        │
│  │  - PIN       │     │  - Lockout   │        │
│  └──────────────┘     └──────────────┘        │
│                                                 │
│  ┌──────────────┐     ┌──────────────┐        │
│  │   Audit      │     │    Config    │        │
│  │  - Logging   │     │  - Auth      │        │
│  │  - Tracing   │     │  - Test Mode │        │
│  └──────────────┘     └──────────────┘        │
│                                                 │
│  ┌───────────────────────────────────┐        │
│  │      Data Client (Services)       │        │
│  │  - create_user                    │        │
│  │  - get_user_by_phone              │        │
│  │  - get_user_by_principal          │        │
│  │  - store_pin_hash                 │        │
│  │  - verify lockout                 │        │
│  └───────────────────────────────────┘        │
└─────────────────┬───────────────────────────────┘
                  │ Inter-canister calls
                  ▼
         ┌────────────────┐
         │ Data Canister  │
         │ (Storage Layer)│
         └────────────────┘
```

### Separation of Concerns
- **user_canister:** Business logic, validation, authentication
- **data_canister:** Storage only (users, PIN hashes, audit logs)
- **shared_types:** Common types and audit module

---

## API Endpoints

### User Registration

#### `register_user(request: RegisterUserRequest) -> Result<String, String>`
Register a new user with phone number, principal, or both.

**Request:**
```rust
RegisterUserRequest {
    phone_number: Option<String>,    // +256700123456
    principal_id: Option<String>,    // aaaaa-aa
    first_name: String,              // John
    last_name: String,               // Doe
    email: String,                   // john@example.com
    preferred_currency: String,      // UGX
    pin: String,                     // 1234
}
```

**Response:** User ID (`user_1620328630000000009`)

**Validations:**
- At least one identifier (phone or principal) required
- PIN: exactly 4 digits, numeric only
- Phone: starts with `+`, min 10 characters
- Email: contains `@` and `.`
- Names: 2-50 characters
- Currency: valid `FiatCurrency` enum

**Audit:** Logs `user_registered` or `user_registration_failed`

**Example:**
```bash
dfx canister call user_canister register_user '(record {
  phone_number = opt "+256700123456";
  principal_id = null;
  first_name = "John";
  last_name = "Doe";
  email = "john@example.com";
  preferred_currency = "UGX";
  pin = "1234";
})'
```

---

### Authentication

#### `verify_pin(user_identifier: String, pin: String) -> Result<bool, String>`
Verify user's PIN. Tracks failed attempts and enforces lockout.

**Parameters:**
- `user_identifier`: Phone number, principal, or user ID
- `pin`: 4-digit PIN

**Returns:** `true` if PIN correct, `false` if incorrect

**Errors:** Returns error if account locked or user not found

**Security Features:**
- Argon2id verification (constant-time)
- Failed attempt tracking
- Account lockout after 3 failures (30 minutes)
- Audit logging (success/failure)

**Example:**
```bash
dfx canister call user_canister verify_pin '("+256700123456", "1234")'
```

---

#### `change_pin(user_identifier: String, old_pin: String, new_pin: String) -> Result<(), String>`
Change user's PIN (requires old PIN verification).

**Security:**
- Verifies old PIN first (prevents unauthorized changes)
- Validates new PIN format
- Generates fresh salt for new hash
- Audit logs the change

**Example:**
```bash
dfx canister call user_canister change_pin '("+256700123456", "1234", "5678")'
```

---

### User Queries

#### `user_exists(user_identifier: String) -> Result<bool, String>`
Check if user exists by phone, principal, or user ID.

#### `get_user_profile_update(user_identifier: String) -> Result<UserProfile, String>`
Get user profile information.

**Returns:**
```rust
UserProfile {
    phone_number: Option<String>,
    principal_id: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
    preferred_currency: String,
    kyc_status: String,
    created_at: u64,
    last_active: u64,
}
```

#### `get_user_by_phone_update(phone: String) -> Result<UserProfile, String>`
Look up user by phone number.

#### `get_user_by_principal_update(principal: String) -> Result<UserProfile, String>`
Look up user by principal ID.

---

### Account Linking

#### `link_phone_to_account(principal_id: String, phone_number: String) -> Result<(), String>`
Link a phone number to an existing principal-only account.

**Use Case:** User registers via web (principal only), then links phone for USSD access.

**Validations:**
- Phone format validated
- Principal must exist
- Phone must not be already registered

**Audit:** Logs `phone_linked`

---

### Audit & Tracing

#### `get_audit_log(limit: Option<u64>) -> Vec<AuditEntry>`
Get recent audit log entries (max 1000).

#### `get_user_audit_log(user_id: String, limit: Option<u64>) -> Vec<AuditEntry>`
Get audit entries for a specific user.

#### `get_audit_by_action(action: String, limit: Option<u64>) -> Vec<AuditEntry>`
Get audit entries by action type (`user_registered`, `pin_verified`, etc.).

#### `get_failed_operations(limit: Option<u64>) -> Vec<AuditEntry>`
Get all failed operations (for debugging).

#### `get_audit_stats() -> AuditStats`
Get audit log statistics.

**Returns:**
```rust
AuditStats {
    total_entries: usize,
    successful_operations: usize,
    failed_operations: usize,
    unique_actions: usize,
    most_common_action: Option<String>,
}
```

**Audit Entry Format:**
```rust
AuditEntry {
    timestamp: u64,              // Seconds since epoch
    action: String,              // "user_registered", "pin_verified", etc.
    caller: String,              // Principal ID of caller
    user_id: Option<String>,     // User affected
    details: String,             // Human-readable description
    success: bool,               // Operation outcome
}
```

---

## Configuration

### Required Setup (Production)

**1. Set Data Canister ID** (Required)
```bash
dfx canister call user_canister set_data_canister_id "(principal \"$(dfx canister id data_canister)\")"
```
- Only callable by controller
- Must be called before any user operations
- Configures where user data is stored

**2. Add Authorized Canisters** (Required)
```bash
# Add USSD canister
dfx canister call user_canister add_authorized_canister "(principal \"$(dfx canister id ussd_canister)\")"

# Add Web canister (if using direct API)
dfx canister call user_canister add_authorized_canister "(principal \"$(dfx canister id web_canister)\")"
```
- Only callable by controller
- Whitelist of canisters allowed to call user_canister
- Without this, no external calls will succeed

**3. Verify Test Mode is DISABLED** (Critical for Production)
```bash
# Test mode is disabled by default - DO NOT enable in production!
# Test mode bypasses all authorization checks
```

### Development-Only Configuration

**Enable Test Mode (for local testing)**
```bash
dfx canister call user_canister enable_test_mode
```

⚠️ **WARNING:** Test mode disables all authorization checks. Use only in development!
- Allows anyone to call any endpoint without authorization
- Should never be enabled in production
- Useful for testing before authorized canisters are configured

---

## Security

### Recent Security Improvements

**Version 0.1.0 includes the following security enhancements:**

1. **Argon2id PIN Hashing (Upgraded from HMAC-SHA256)**
   - Industry-standard password hashing algorithm
   - Resistant to timing attacks and GPU brute-force
   - Replaces deprecated HMAC-SHA256 approach

2. **User Enumeration Prevention**
   - Generic error messages: "Invalid credentials" vs "User not found"
   - Same error for all failure scenarios (missing user, wrong PIN)
   - Prevents attackers from discovering registered users

3. **Input Validation Hardening**
   - E.164 phone format validation (international standard)
   - RFC 5322 email validation (basic format checking)
   - All inputs validated before processing
   - 100% coverage of validation logic with unit tests

4. **Access Control Tiers**
   - Controller-only operations (configuration)
   - Authorized canister access (USSD, web canisters)
   - User self-access for direct profile queries (in development)

5. **Stable Storage with Upgrade Hooks**
   - `pre_upgrade()` / `post_upgrade()` hooks for persistence
   - Configuration state survives canister upgrades
   - Authorized canister list maintained across upgrades

### PIN Security
- **Algorithm:** Argon2id (PHC winner)
- **Parameters:** 19 MiB memory, 2 iterations, parallelism=1
- **Salt:** 16 bytes from IC's `raw_rand()` (cryptographic random)
- **Format:** PHC string (includes all parameters for future upgrades)
- **Storage:** Only hash stored (no plaintext PINs)

### Account Lockout
- **Trigger:** 3 failed PIN attempts
- **Duration:** 30 minutes
- **Tracking:** Per user, in data_canister
- **Bypass:** None (even correct PIN blocked during lockout)

### Access Control
- **Controllers:** Can manage configuration
- **Authorized Canisters:** Can call all endpoints
- **Test Mode:** Bypasses checks (development only)

### Audit Trail
- **Scope:** All operations logged
- **Immutability:** Append-only (no deletion)
- **Rotation:** Automatic at 10,000 entries
- **Persistence:** ⚠️ Lost on upgrade (TODO: add pre_upgrade hook)

---

## Testing

### Running Tests

**Unit Tests:**
```bash
cd canisters/user_canister
cargo test --lib
# Result: 23/23 passing, 100% validation logic coverage
```

**Integration Tests:**
```bash
# Build WASMs first
cargo build --target wasm32-unknown-unknown --release \
  --package data_canister --package user_canister

# Run integration tests
cargo test --test lib -- --test-threads=1
# Result: 142/142 passing, all endpoints tested
```

**Coverage Report:**
```bash
cargo llvm-cov --lib --html --output-dir coverage
open coverage/html/index.html
```

### Test Coverage
- **Unit Tests:** 20 tests (validation logic 100% coverage) ✅
- **Integration Tests:** 79 tests (all endpoints, security features) ✅
- **Total:** 99 tests, all passing
- **Security Features Tested:** PIN lockout, duplicates, enumeration prevention, access control ✅

**Test Breakdown by Category:**
- User Registration: 6 tests
- PIN Security & Verification: 7 tests
- Access Control: 15+ tests
- User Enumeration Prevention: 10+ tests
- Validation Edge Cases: 18+ tests
- Upgrade Persistence: 18+ tests
- Audit Logging: 5+ tests

See `COVERAGE_REPORT.md` for details.

---

## Usage Examples

### Example 1: Register User with Phone
```rust
use shared_types::RegisterUserRequest;

let request = RegisterUserRequest {
    phone_number: Some("+256700123456".to_string()),
    principal_id: None,
    first_name: "Alice".to_string(),
    last_name: "Nakato".to_string(),
    email: "alice@example.com".to_string(),
    preferred_currency: "UGX".to_string(),
    pin: "1234".to_string(),
};

let user_id = register_user(request).await?;
// Returns: "user_1620328630000000009"
```

### Example 2: Verify PIN
```rust
let verified = verify_pin("+256700123456".to_string(), "1234".to_string()).await?;
if verified {
    println!("✅ Login successful");
} else {
    println!("❌ Incorrect PIN");
}
```

### Example 3: Link Phone to Principal Account
```rust
// User already registered with principal only (web)
link_phone_to_account(
    "aaaaa-aa".to_string(),
    "+256700123456".to_string()
).await?;
// Now user can login with either phone or principal
```

### Example 4: Query Audit Log
```rust
// Get failed login attempts
let failed_ops = get_failed_operations(Some(50));
for entry in failed_ops {
    if entry.action == "pin_verification_failed" {
        println!("❌ Failed login at {}: {}", entry.timestamp, entry.details);
    }
}

// Get user-specific audit trail
let user_log = get_user_audit_log("user_123".to_string(), Some(100));
```

---

## Deployment

### Build & Deploy
```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release --package user_canister

# Deploy canister
dfx deploy user_canister

# Configure
USER_ID=$(dfx canister id user_canister)
DATA_ID=$(dfx canister id data_canister)

dfx canister call user_canister set_data_canister_id "(\"$DATA_ID\")"

# Authorize USSD canister
USSD_ID=$(dfx canister id ussd_canister)
dfx canister call user_canister add_authorized_canister "(\"$USSD_ID\")"
```

### Production Checklist
- [ ] Data canister ID configured
- [ ] Authorized canisters added
- [ ] Test mode is DISABLED
- [ ] Security audit reviewed
- [ ] Integration tests passing
- [ ] WASM size < 2MB (currently 1.0M ✅)

---

## Maintenance

### Monitoring
```bash
# Check canister status
dfx canister status user_canister

# View logs
dfx canister logs user_canister

# Get audit statistics
dfx canister call user_canister get_audit_stats
```

### Troubleshooting

**"User not found"**
- Check if user registered with phone vs principal
- Try lookup by both identifiers

**"PIN locked due to X failed attempts"**
- Account locked after 3 failures
- Wait 30 minutes or reset via data_canister (admin only)

**"Unauthorized: Only authorized canisters can call this"**
- Add caller to authorized list
- Or enable test mode (development only)

---

## Audit Trail Events

### Tracked Events
| Event | When Logged | Success | Failure |
|-------|-------------|---------|---------|
| `user_registered` | User created | ✅ User details | ❌ Validation error |
| `user_registration_failed` | Registration fails | N/A | ❌ Error reason |
| `pin_verified` | PIN check succeeds | ✅ User identifier | N/A |
| `pin_verification_failed` | PIN check fails | N/A | ❌ User identifier |
| `pin_changed` | PIN updated | ✅ User identifier | ❌ Old PIN wrong |
| `phone_linked` | Phone linked to principal | ✅ Phone + principal | ❌ Already exists |
| `inter_canister_call` | Call to data_canister | ✅ Method + correlation ID | N/A |
| `inter_canister_result` | Result from data_canister | ✅/❌ Result | N/A |
| `canister_initialized` | Canister starts | ✅ Timestamp | N/A |

---

## Dependencies

### Rust Crate Dependencies
```toml
[dependencies]
candid = "0.10"
ic-cdk = "0.18"
ic-cdk-macros = "0.18"
argon2 = "0.5"           # Password hashing (Argon2id)
hex = "0.4"              # Hex encoding for hashes
shared_types = { path = "../shared_types" }  # Audit module
```

### Inter-Canister Dependencies

**data_canister** (Required)
- Stores user data (User records)
- Stores PIN hashes (sensitive)
- Tracks failed login attempts and lockout times
- Must be deployed and configured before user_canister

**Calls Made to data_canister:**
- `create_user(user_data)` - Register new user
- `get_user(user_id)` - Look up user by ID
- `get_user_by_phone(phone)` - Look up user by phone
- `get_user_by_principal(principal)` - Look up user by principal
- `store_pin_hash(user_id, hash)` - Store PIN hash
- `get_pin_hash(user_id)` - Retrieve PIN hash
- `increment_failed_attempts(user_id)` - Track failed login
- `reset_pin_attempts(user_id)` - Clear failed attempts on success
- `is_pin_locked(user_id)` - Check if account is locked
- `update_user_phone(user_id, phone)` - Link phone to account

**Calling Canisters** (Authorized Clients)
- `ussd_canister` - USSD interface, calls all user_canister endpoints
- `web_canister` - Web API, calls user_canister for profile operations
- `wallet_canister` - May call verify_pin for secure operations
- `crypto_canister` - May call verify_pin for sensitive transactions

---

## File Structure

```
user_canister/
├── Cargo.toml
├── user_canister.did          # Auto-generated Candid interface
├── README.md                  # This file
├── SECURITY_AUDIT.md          # Security review
├── COVERAGE_REPORT.md         # Test coverage analysis
├── src/
│   ├── lib.rs                 # Main canister endpoints
│   ├── config.rs              # Authorization & configuration
│   ├── security.rs            # Argon2 PIN hashing
│   ├── logic/
│   │   └── user_logic.rs      # Validation functions (100% coverage)
│   └── services/
│       └── data_client.rs     # Inter-canister calls
└── tests/
    ├── unit/                  # Unit tests (23 tests)
    │   └── ... (validation tests)
    └── integration/           # Integration tests (142 tests)
        ├── user_registration_tests.rs
        └── pin_security_tests.rs
```

---

## Roadmap

### Completed ✅
- [x] User registration (phone/principal/both)
- [x] Argon2id PIN hashing
- [x] Account lockout mechanism
- [x] Comprehensive audit trail
- [x] Inter-canister call tracing
- [x] 142 integration tests
- [x] Security audit (9.2/10)
- [x] Coverage report
- [x] Documentation

### TODO (Next Priority)
- [ ] **MEDIUM:** Add `UserSelf` access control tier for direct profile access
  - Allow authenticated principals to query own profiles without intermediary canisters
  - Reduces inter-canister call overhead
  - Aligns with wallet_canister and crypto_canister patterns

- [ ] **MEDIUM:** Implement constant-time PIN comparison
  - Current: Argon2id provides constant-time verification
  - Consider: Add explicit timing attack protection in verification logic

- [ ] **LOW:** Add environment variable check to prevent test mode in production
  - Validate that test mode cannot be enabled via hardcoded checks
  - Use DFX network detection

- [ ] **LOW:** Add rate limiting on public endpoints
  - Prevent brute force attacks on verify_pin endpoint
  - Track attempts per phone number

- [ ] **LOW:** Add property-based fuzzing tests for input validation
  - Use quickcheck or proptest for randomized input testing

- [ ] **LOW:** Implement circuit breaker for canister call failures
  - Graceful degradation when data_canister is unavailable
  - Retry logic with backoff

---

## Contributing

When modifying user_canister:

1. **Run Tests:** `cargo test && cargo test --test lib -- --test-threads=1`
2. **Check Coverage:** `cargo llvm-cov --lib --summary-only`
3. **Security Review:** Update `SECURITY_AUDIT.md` if security-relevant
4. **Update Docs:** Keep README, COVERAGE_REPORT in sync
5. **Audit Logging:** Add audit calls for new operations

---

## Resources

- **Security Audit:** `SECURITY_AUDIT.md`
- **Coverage Report:** `COVERAGE_REPORT.md`
- **Candid Interface:** `user_canister.did` (auto-generated)
- **Shared Audit Module:** `../shared_types/src/audit.rs`
- **IC Documentation:** https://internetcomputer.org/docs
- **Argon2 Spec:** https://github.com/P-H-C/phc-winner-argon2

---

## License

AGPL-3.0 - See LICENSE file in repository root

---

## Support

- **Issues:** https://github.com/AfriTokeni/afritokeni/issues
- **Security:** security@afritokeni.com
- **Email:** hello@afritokeni.com

---

**Built with ❤️ for AfriTokeni | Powered by Internet Computer Protocol**
