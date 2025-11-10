# Testing Guide

## Overview

AfriTokeni uses a hybrid testing approach due to a known limitation in PocketIC's Candid encoding for multi-argument functions.

## Quick Summary

✅ **Enums & Type Safety**: We use enums internally for type safety  
✅ **API Boundary**: String arguments for inter-canister calls (workaround for Candid bug)  
✅ **Testing**: PocketIC for simple canisters, local replica for `business_logic_canister`

## Architecture: Enums vs Strings

### Why Strings at API Boundary?

```rust
// ❌ This fails in PocketIC (and sometimes ic_cdk::call)
#[update]
fn create_user(user_type: UserType, currency: FiatCurrency, ...) { }

// ✅ This works everywhere
#[update]
fn create_user(user_type_str: String, currency_str: String, ...) {
    // Convert to enums immediately for type safety
    let user_type = UserType::from_str(&user_type_str)?;
    let currency = FiatCurrency::from_string(&currency_str)?;
    // ... rest uses enums
}
```

**Benefits:**
- ✅ Works with PocketIC and local replica
- ✅ Type-safe internally (enums)
- ✅ Clear error messages for invalid inputs
- ✅ Follows "anti-corruption layer" pattern

## Test Types

### 1. Unit Tests (Rust)
Tests individual canister logic without inter-canister calls.

```bash
npm run test:rust
```

**Includes:**
- `data_canister` - ✅ Uses PocketIC
- `deposit_canister` - ✅ Uses PocketIC  
- `withdrawal_canister` - ✅ Uses PocketIC
- `exchange_canister` - ✅ Uses PocketIC
- `ussd_canister` - ✅ Uses PocketIC

**Excludes:**
- `business_logic_canister` - ❌ PocketIC has multi-argument encoding bug (167 passing, 22 failing)

### 2. Integration Tests (Local IC Replica)
Tests inter-canister communication using a local IC replica.

```bash
# Full integration test suite
npm run test:integration:replica

# Or step-by-step:
npm run test:integration:replica:setup   # Start replica & deploy
# Manually test via dfx canister call
npm run test:integration:replica:cleanup # Stop replica
```

**Why Local Replica?**
- PocketIC's `encode_args` macro encodes tuples as a single record instead of separate arguments
- This causes "Fail to decode argument 0 from table0" errors
- Local IC replica handles multi-argument calls correctly
- Bug reported: https://github.com/dfinity/pocketic/issues/[issue-number]

### 3. PocketIC Tests (Legacy)
The `business_logic_canister` still has PocketIC tests, but they will fail until the bug is fixed.

```bash
npm run test:integration:pocketic  # ⚠️ Will fail with known bug
```

## CI/CD Pipeline

The GitHub Actions workflow runs:

1. **Canister Unit Tests** - All canisters except `business_logic_canister`
2. **Integration Tests** - Deploys to local replica and verifies deployment

```yaml
# .github/workflows/ci.yml
test-canisters:
  run: cargo test --workspace --exclude business_logic_canister

test-integration:
  run: pnpm run test:integration:replica:setup
```

## Manual Testing

### Test create_user on Local Replica

```bash
# Start and deploy
npm run icp:start
npm run icp:deploy

# Test create_user
dfx canister call data_canister create_user \
  '("User", "UGX", "test@example.com", "John", "Doe", null, opt "+256700123456")'

# Should return:
# (variant { Ok = record { id = "user_..."; ... } })

# Cleanup
npm run icp:stop
```

### Test via business_logic_canister

```bash
# After deployment above
dfx canister call business_logic_canister register_user \
  '(record { 
    user_type = variant { User }; 
    preferred_currency = variant { UGX }; 
    email = "test@example.com"; 
    first_name = "John"; 
    last_name = "Doe"; 
    phone_number = opt "+256700123456" 
  })'
```

## Known Issues

### PocketIC Multi-Argument Bug

**Problem:**
```rust
// This fails in PocketIC
encode_args((arg1, arg2, arg3, ...))
// Encodes as: record { arg1, arg2, arg3 }
// Instead of: separate arguments
```

**Error:**
```
Fail to decode argument 0 from table0 to text
Caused by: Subtyping error: Type mismatch
```

**Workaround:**
Use local IC replica for integration tests until PocketIC is fixed.

**Tracking:**
- GitHub Issue: https://github.com/dfinity/pocketic/issues/[number]
- Forum Discussion: https://forum.dfinity.org/t/how-to-do-multiple-params-on-pocketic-query-call/36167

## Future Plans

Once PocketIC bug is fixed:
1. Migrate all integration tests back to PocketIC
2. Remove local replica requirement from CI/CD
3. Update this documentation

## Contributing

When adding new tests:
- ✅ Use PocketIC for simple canisters (single-argument methods)
- ✅ Use local replica for complex inter-canister calls
- ✅ Document any workarounds in code comments
- ✅ Update this guide if testing approach changes
