# USSD Test Fixes - Session 2

## Summary
- **Before**: 119/312 tests passing (38%)
- **After**: 122/312 tests passing (39%)
- **Fixed**: 3 additional tests
- **Total Fixed**: 10 tests from initial 112

## Fixes Applied

### 1. Proper Architecture - Test Balance Setting
**Problem**: Tests were calling `set_fiat_balance` directly on data canister, bypassing business logic layer.

**Root Cause**: USSD is presentation layer only and should not execute business logic. Tests must respect the 3-tier architecture.

**Solution**:
- Added `set_fiat_balance` function to business logic canister as test-only helper
- Updated test environment to call through business logic canister
- Maintains proper architecture: USSD → Business Logic → Data

**Files Modified**:
- `canisters/business_logic_canister/src/lib.rs` - Added `set_fiat_balance` function (lines 965-974)
- `canisters/ussd_canister/tests/integration/mod.rs` - Updated to call business logic (lines 250-257)

### 2. Hardcoded Currency Bug
**Problem**: Bitcoin and USDC buy flows hardcoded "UGX" currency instead of using user's preferred currency.

**Impact**: Tests for KES, TZS, NGN users were failing with "Insufficient balance" because balance was set in their currency but code checked UGX balance.

**Solution**:
- Get currency from session data: `session.get_data("currency")`
- Use dynamic currency in business logic calls
- Update success messages to show actual currency

**Files Modified**:
- `canisters/ussd_canister/src/flows/bitcoin/buy.rs` - Lines 48-59, 67-77
- `canisters/ussd_canister/src/flows/usdc/buy.rs` - Lines 47-56, 64-69

### 3. Session Currency Detection
**Problem**: Currency not set in session when tests bypass registration flow.

**Impact**: Direct USSD calls (like "2*3*50000*1234") had no currency in session, defaulting to UGX.

**Solution**:
- Auto-detect currency from phone number when creating new session
- Made `detect_currency_from_phone` public
- Currency now available for all USSD flows

**Files Modified**:
- `canisters/ussd_canister/src/core/session.rs` - Lines 96-99
- `canisters/ussd_canister/src/core/routing.rs` - Line 182 (made function public)

### 4. Crypto Send Amount Parsing
**Problem**: Bitcoin and USDC send flows were converting amounts incorrectly.
- Bitcoin: Treated input as BTC, multiplied by 100,000,000 to get satoshis
- USDC: Treated input as USDC, multiplied by 1,000,000 to get e6
- Tests: Passed amounts already in smallest units (satoshis/e6)

**Impact**: Test sending 50000 satoshis became 5,000,000,000,000 satoshis, causing "Insufficient balance" errors.

**Solution**:
- Amounts are already in smallest units (satoshis for BTC, e6 for USDC)
- Parse directly as u64 instead of converting from f64
- Fixed test addresses to use IC Principals instead of Bitcoin/Ethereum addresses

**Files Modified**:
- `canisters/ussd_canister/src/flows/bitcoin/send.rs` - Line 63
- `canisters/ussd_canister/src/flows/usdc/send.rs` - Lines 88-90
- `canisters/ussd_canister/tests/integration/bitcoin_complete_tests.rs` - Lines 96, 145
- `canisters/ussd_canister/tests/integration/usdc_complete_tests.rs` - Lines 94, 112, 142, 371

**Note**: ckBTC and ckUSDC are IC tokens, so they use IC Principal addresses (e.g., `rrkah-fqaaa-aaaaa-aaaaq-cai`), not Bitcoin/Ethereum addresses.

## Architecture Principle Reinforced

**USSD Canister = Presentation Layer Only**
- Can query data from other canisters
- Cannot execute business logic
- Cannot directly modify data canister

**Correct Flow**:
```
USSD Canister → Business Logic Canister → Data Canister
```

**Test Setup**:
```rust
// ✅ CORRECT: Call through business logic
env.set_fiat_balance(phone, "KES", 500000) 
  → business_logic_canister.set_fiat_balance()
    → data_canister.set_fiat_balance()

// ❌ WRONG: Direct data canister call
data_canister.set_fiat_balance()
```

## Test Results by Category

### Bitcoin Tests: 15/21 passing (71%)
**Passing**:
- Buy Bitcoin (UGX, KES, TZS)
- Sell Bitcoin
- Send Bitcoin (valid address, insufficient balance, invalid address)
- Bitcoin rate check
- Menu navigation

**Still Failing** (6 tests):
- Complex flows (buy then sell, buy then send)
- Return to main menu
- Zero amount validation

### USDC Tests: 11/22 passing (50%)
**Passing**:
- Buy USDC
- Sell USDC
- Basic send operations
- USDC rate check

**Still Failing** (11 tests):
- Complex flows
- Multi-step operations
- Balance validations

## Next Steps

1. **Investigate Complex Flow Failures**
   - Tests that combine multiple operations (buy → sell, buy → send)
   - Likely session state management issues

2. **Check Zero Amount Validation**
   - Tests expect rejection at step 2 (amount entry)
   - Code might be validating at step 3 (execution)

3. **Analyze Remaining 190 Failures**
   - Pattern recognition across test categories
   - Identify common root causes

4. **Re-enable send_money_complete_tests Module**
   - Currently commented out in mod.rs
   - ~25 additional tests to fix

## Key Learnings

1. **Currency Detection**: Phone number prefixes reliably map to currencies
   - +256 → UGX (Uganda)
   - +254 → KES (Kenya)
   - +255 → TZS (Tanzania)

2. **Amount Units**: Always use smallest units in code
   - Fiat: cents (amount * 100)
   - Bitcoin: satoshis (1 BTC = 100,000,000 sats)
   - USDC: e6/micro-USDC (1 USDC = 1,000,000 e6)

3. **IC Token Addresses**: ckBTC and ckUSDC use IC Principals, not blockchain addresses
   - Valid: `rrkah-fqaaa-aaaaa-aaaaq-cai`
   - Invalid: `bc1q...` (Bitcoin) or `0x...` (Ethereum)

4. **Test Isolation**: Individual tests may pass while full suite fails
   - Indicates test interference or shared state issues
   - Need to investigate session cleanup between tests
