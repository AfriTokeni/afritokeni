# Playground Mode Security Deep Dive

**Analysis Date:** 2025-11-14
**Risk Level:** CRITICAL 🔴
**Production Status:** BLOCKING ISSUE

---

## Overview

Playground mode is a testing feature designed to allow frontend developers to test USSD flows without full user registration. However, **it creates a complete authentication bypass** that allows attackers to create accounts with a known PIN.

---

## How Playground Mode Works

### Configuration

**File:** `canisters/ussd_canister/config.toml` (lines 69-77)

```toml
[playground]
enabled = true                    # ⚠️ Feature toggle
session_id_prefix = "playground_" # Trigger pattern
default_pin = "1234"              # ⚠️ HARDCODED CREDENTIAL
default_currency = "UGX"          # Default for auto-created users
```

### Auto-Registration Logic

**File:** `canisters/ussd_canister/src/api/ussd.rs` (lines 100-129)

```rust
// 1. Check if user is registered
let mut user_registered = match get_user_by_phone(phone_number).await {
    Ok(_) => true,
    Err(_) => false,
};

// 2. PLAYGROUND MODE: Auto-register if session ID starts with "playground_"
let config = get_config();
if !user_registered && config.playground.enabled &&
   session_id.starts_with(&config.playground.session_id_prefix) {

    // Auto-generate email
    let ussd_email = format!("{}@ussd.afritokeni.com",
                             phone_number.replace("+", ""));

    // Register with HARDCODED PIN
    match register_user(
        Some(phone_number.clone()),
        None,
        "Demo".to_string(),
        "User".to_string(),
        ussd_email,
        config.playground.default_pin.clone(),  // ⚠️ PIN = "1234"
        config.playground.default_currency.clone()
    ).await {
        Ok(user_id) => {
            user_registered = true;  // Continue as registered user
        }
        Err(e) if e.contains("already registered") => {
            user_registered = true;  // User already exists, OK
        }
        Err(e) => {
            // Failed to create user
        }
    }
}
```

---

## Attack Scenarios

### Scenario 1: Direct Account Takeover

**Attack Steps:**

1. Attacker sends USSD request with playground session ID:
   ```json
   POST /ussd_webhook
   {
     "sessionId": "playground_attacker_session_001",
     "phoneNumber": "+256700123456",
     "text": ""
   }
   ```

2. USSD canister auto-creates user:
   - Phone: +256700123456
   - PIN: 1234 (hardcoded)
   - Email: 256700123456@ussd.afritokeni.com
   - Name: Demo User

3. Attacker now has full access to account with known PIN "1234"

4. Attacker performs financial operations:
   ```json
   // Sell 1 BTC with known PIN
   {
     "sessionId": "playground_attacker_session_002",
     "phoneNumber": "+256700123456",
     "text": "2*4*1*1234"  // Bitcoin Menu -> Sell -> 1 BTC -> PIN
   }
   ```

**Impact:**
- Complete account control
- Can withdraw funds deposited to this account
- Can perform unlimited transactions (rate limits bypassed for JSON)

---

### Scenario 2: Mass Account Creation

**Attack:**

Attacker creates thousands of playground accounts:

```python
for i in range(10000):
    session_id = f"playground_attack_{i}"
    phone = f"+25670{1000000 + i}"  # Sequential phone numbers

    requests.post(USSD_WEBHOOK, json={
        "sessionId": session_id,
        "phoneNumber": phone,
        "text": ""
    })
```

**Result:**
- 10,000 accounts created, all with PIN "1234"
- Attacker controls all accounts
- Can use for money laundering, fraud schemes
- Wastes canister storage/cycles

**Impact:**
- Resource exhaustion
- Platform reputation damage
- Financial fraud at scale

---

### Scenario 3: Victim Account Takeover

**Attack:**

1. Attacker learns victim's phone number: +256700555555

2. Before victim registers normally, attacker registers them via playground:
   ```json
   {
     "sessionId": "playground_victim_takeover",
     "phoneNumber": "+256700555555",
     "text": ""
   }
   ```

3. Account created with PIN "1234"

4. Victim attempts to register normally → **BLOCKED** (phone already registered)

5. Victim calls support, but their account already exists with attacker's PIN

6. Attacker drains any funds victim deposits before they can change PIN

**Impact:**
- User lockout
- Fund theft
- Support overhead
- Trust loss

---

## Why This Is CRITICAL

### 1. Complete Authentication Bypass

Playground mode **completely bypasses the PIN security model**:
- Normal registration: User chooses secure 4-digit PIN
- Playground registration: System sets known PIN "1234"
- No PIN verification during auto-registration

### 2. Known Credential

The PIN "1234" is:
- Hardcoded in config.toml (public repository)
- Same for ALL playground users
- Never expires
- Cannot be changed without user intervention

### 3. Easy to Exploit

Attack requirements:
- Send HTTP POST to public USSD webhook
- Set session ID to start with "playground_"
- No authentication required
- No rate limiting (JSON bypass)

### 4. Silent Compromise

Victims may not know they've been compromised:
- Account created before legitimate registration attempt
- No notification sent
- Funds can be stolen immediately upon deposit

---

## Rate Limiting Bypass Amplification

Playground mode is **doubly dangerous** because it also bypasses rate limiting:

**File:** `src/api/ussd.rs` (line 71)

```rust
// Check rate limit (skip for JSON requests which are tests)
if !is_json && !crate::utils::rate_limit::check_rate_limit(&phone_number) {
    // Rate limit error
}
```

**Impact:**
- JSON requests (used by playground) skip rate limiting
- Attacker can:
  - Create unlimited accounts (no rate limit)
  - Attempt unlimited transactions (no rate limit)
  - Brute force other accounts' PINs (no rate limit)

---

## Real-World Exploitation Timeline

### T+0: Attack Begins
- Attacker discovers playground mode in config.toml (public repo)
- Creates script to auto-register accounts

### T+1 hour: Mass Creation
- 10,000 accounts created with known PIN
- All accounts controlled by attacker

### T+24 hours: Fund Deposits
- Legitimate users deposit funds to some of these phone numbers
- Attacker immediately drains funds with known PIN "1234"

### T+1 week: Discovery
- Multiple users report unauthorized withdrawals
- Support team investigates, discovers all victims have "Demo User" accounts
- Playground mode identified as root cause

### T+2 weeks: Incident Response
- Emergency canister upgrade to disable playground mode
- Cannot retroactively secure accounts created with "1234" PIN
- Users must be contacted to change PINs
- Funds may be unrecoverable

**Estimated Impact:**
- Financial loss: Depends on deposits, potentially unlimited
- Reputation damage: Severe
- Legal liability: Potential
- Recovery cost: High (incident response, user support, PR)

---

## Production Detection

### How to Check if Vulnerable

**In canister config:**
```bash
grep "playground" canisters/ussd_canister/config.toml
```

If output shows `enabled = true`, **you are vulnerable**.

**In deployed canister:**
```bash
# Test playground registration
curl -X POST https://your-canister.ic0.app/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "sessionId": "playground_security_test",
    "phoneNumber": "+25670999999",
    "text": ""
  }'
```

If response creates user without PIN prompt, **you are vulnerable**.

---

## Mitigation Strategies

### Option 1: Disable Playground Mode (RECOMMENDED)

**Immediate fix:**

```toml
[playground]
enabled = false  # PRODUCTION
```

**Pros:**
- Simple, immediate
- No code changes required
- Eliminates vulnerability completely

**Cons:**
- Breaks frontend testing workflow
- Developers lose auto-registration convenience

---

### Option 2: Build-Time Toggle

**Code change:**

```rust
// In src/api/ussd.rs, replace lines 100-129 with:

#[cfg(debug_assertions)]  // Only in debug builds
{
    let config = crate::config_loader::get_config();
    if !user_registered && config.playground.enabled &&
       session_id.starts_with(&config.playground.session_id_prefix) {
        // Auto-registration logic (only in debug builds)
    }
}

#[cfg(not(debug_assertions))]  // Production builds
{
    if session_id.starts_with("playground_") {
        ic_cdk::trap("Playground mode not available in production");
    }
}
```

**Pros:**
- Compile-time guarantee playground mode is disabled in production
- Still works in development builds
- Zero-cost abstraction (no runtime check)

**Cons:**
- Requires code change + rebuild
- Must maintain separate dev/prod build pipelines

---

### Option 3: Environment-Based Toggle (NOT RECOMMENDED)

**Concept:** Check canister environment variable to enable/disable

**Why NOT recommended:**
- Environment variables can be changed post-deployment
- No compile-time guarantee
- Leaves attack surface exposed

---

### Option 4: Secure Playground Mode (COMPLEX)

**Design:**

1. Generate unique PIN per playground session:
   ```rust
   use ic_cdk::api::management_canister::main::raw_rand;

   let random_bytes = raw_rand().await?;
   let pin = format!("{:04}", u16::from_be_bytes([random_bytes[0], random_bytes[1]]) % 10000);
   ```

2. Return PIN to user in response:
   ```json
   {
     "response": "Welcome! Your temporary PIN is: 5837\n\n1. Local Currency\n2. Bitcoin...",
     "continueSession": true
   }
   ```

3. User must use generated PIN for transactions

**Pros:**
- Maintains testing convenience
- Eliminates known-PIN vulnerability

**Cons:**
- Complex implementation
- Still bypasses rate limiting
- PIN displayed in cleartext (less secure than normal registration)
- Doesn't solve mass account creation problem

---

## Recommended Action Plan

### Immediate (Within 24 hours)

1. **Disable playground mode in production config:**
   ```bash
   sed -i 's/enabled = true/enabled = false/' canisters/ussd_canister/config.toml
   ```

2. **Deploy updated config to production canister**

3. **Audit existing users for playground accounts:**
   ```rust
   // Query users with email matching pattern: *@ussd.afritokeni.com
   // Filter for "Demo User" names
   // Flag for mandatory PIN reset
   ```

### Short-Term (Within 1 week)

4. **Implement build-time toggle** (Option 2 above)

5. **Add detection monitoring:**
   ```rust
   // In handle_ussd_webhook():
   if session_id.starts_with("playground_") {
       ic_cdk::println!("🚨 SECURITY: Playground session detected in production!");
       // Alert ops team
   }
   ```

6. **Force PIN reset for playground accounts:**
   - Send SMS to affected users
   - Require PIN change on next login

### Long-Term (Within 1 month)

7. **Remove playground mode code entirely** from production codebase

8. **Create separate test canister** for frontend development:
   - Isolated from production
   - Uses mock data, not real user_canister
   - Can have relaxed security for testing

9. **Implement proper staging environment:**
   - Separate canister deployment
   - Test data only
   - No production credentials

---

## Testing Strategy Post-Fix

### For Frontend Developers

**Without Playground Mode:**

1. **Use staging environment:**
   - Deploy separate canister instance
   - Use test user accounts with known PINs
   - Isolated from production data

2. **Mock USSD responses:**
   - Create local mock server
   - Simulates USSD flows without real canisters
   - Faster, safer, no blockchain dependency

3. **Integration test suite:**
   - Automated tests using PocketIC
   - Pre-created test users with known PINs
   - Runs in CI/CD pipeline

**Example staging setup:**
```bash
# Deploy to staging subnet
dfx deploy --network staging ussd_canister

# Create test users
dfx canister call user_canister register_user '(
  record {
    phone_number = "+254700000001";
    first_name = "Test";
    last_name = "User";
    email = "test@staging.afritokeni.com";
    pin = "1234";  # OK in staging
    preferred_currency = "UGX"
  }
)'
```

---

## Conclusion

Playground mode represents a **complete authentication bypass** that:
- Allows account creation with known PIN "1234"
- Bypasses rate limiting
- Enables mass account takeover
- Is trivial to exploit

**This is a BLOCKING issue for production deployment.**

**Recommended fix:** Disable playground mode immediately via config change, then implement build-time toggle for long-term solution.

**Timeline:**
- Immediate fix: < 1 hour (config change + deploy)
- Permanent fix: < 1 week (build-time toggle + code review)
- Cleanup: < 1 month (remove code, migrate testing to staging)

---

**Security Priority:** CRITICAL
**Recommended Action:** Disable immediately
**Next Review:** After fix applied, verify no playground accounts exist in production
