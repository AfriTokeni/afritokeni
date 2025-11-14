# USSD Canister Test Failure Analysis
**Date:** 2025-11-14
**Test Run:** Integration + Unit Tests
**Result:** 286/313 passing (91.4%)
**Failures:** 27 tests

## Summary Statistics
- **Total Tests:** 313
- **Passing:** 286 (91.4%)
- **Failing:** 27 (8.6%)
- **Runtime:** 279.87s

## Failure Categories

### Category 1: Routing Issues - Balance Check (6 failures)
**Root Cause:** Tests expect "Check Balance" flow but routing is going to "Send Money" flow instead.

Tests affected:
1. `test_balance_check_after_transaction`
2. `test_balance_check_multiple_currencies`
3. `test_balance_check_with_crypto`
4. `test_balance_precision`
5. `test_local_currency_balance_check`
6. `test_withdraw_balance_check_before`

**Evidence:**
```
Expected: Check Balance flow
Got: "Send Money\nEnter recipient phone number:"
```

**Analysis:**
- Tests are sending input `"1"` expecting balance check
- Router is interpreting `"1"` as "Send Money" (local currency menu option 1)
- The shorthand routing logic prioritizes send_money over balance checks

**Fix Required:**
- Review routing logic in `src/core/routing.rs`
- Check balance flow routing in `src/flows/local_currency/mod.rs`
- Ensure `"1*2"` (or appropriate code) routes to balance check

### Category 2: Routing Issues - Withdraw (5 failures)
**Root Cause:** Tests expect "Withdraw" flow but routing is going to "Send Money" flow instead.

Tests affected:
1. `test_withdraw_balance_check_before`
2. `test_withdraw_invalid_agent_id`
3. `test_withdraw_menu_structure`
4. `test_withdraw_requires_agent_id`
5. `test_withdraw_step_by_step`

**Evidence:**
```
Expected: Withdraw flow with agent selection
Got: "Send Money\nEnter recipient phone number:"
```

**Analysis:**
- Tests trying to access withdraw via `"1*4"` or similar
- Router not recognizing withdraw flow pattern
- Withdraw flow may not be properly integrated with shorthand routing

**Fix Required:**
- Check withdraw routing in `src/flows/local_currency/withdraw.rs`
- Verify routing pattern for withdraw in `src/core/routing.rs`
- Ensure withdraw menu option is accessible

### Category 3: Routing Issues - Withdraw Amount Validation (1 failure)
**Root Cause:** Zero amount validation not working, shows main menu instead of error.

Test affected:
1. `test_withdraw_zero_amount`

**Evidence:**
```
Expected: Error message about zero amount
Got: "Welcome to AfriTokeni!\nMain Menu\n1. Local Currency..."
```

**Analysis:**
- Test sends zero amount for withdrawal
- Expected: Validation error
- Got: Main menu (session may have been reset)
- Validation logic may be missing or bypassed

**Fix Required:**
- Add zero amount validation in `src/flows/local_currency/withdraw.rs`
- Check validation happens before processing withdrawal
- Ensure error is displayed, not main menu

### Category 4: DAO Flow Issues (2 failures)
**Root Cause:** DAO flows not implemented or routing incorrectly.

Tests affected:
1. `test_dao_view_proposals`
2. `test_dao_vote_on_proposal`

**Evidence:**
```
Expected: DAO proposal view or voting flow
Got: Various menu states (not DAO)
```

**Analysis:**
- DAO governance menu option 5 may not be implemented
- Tests using shorthand like `"5"` or `"5*1"` not routing to DAO flows
- DAO module may need integration work

**Fix Required:**
- Review `src/flows/dao/mod.rs` implementation status
- Check DAO routing in `src/core/routing.rs`
- Implement or fix DAO view/vote flows

### Category 5: Bitcoin Sell Issues (3 failures)
**Root Cause:** Sell Bitcoin flow has issues with confirmation or execution.

Tests affected:
1. `test_bitcoin_buy_then_sell`
2. `test_sell_bitcoin_all_balance`
3. `test_sell_bitcoin_to_ugx`

**Evidence:**
```
Expected: Bitcoin sell confirmation or success
Got: Main menu or flow restart
```

**Analysis:**
- Buy flow works (many buy tests pass)
- Sell flow has issues after confirmation step
- May be related to recent confirmation step addition mentioned in user context

**Fix Required:**
- Review `src/flows/bitcoin/sell.rs` confirmation logic
- Check if sell confirmation is properly integrated
- Verify state management during sell flow

### Category 6: USDC Issues (6 failures)
**Root Cause:** USDC sell and send flows have multiple issues.

Tests affected:
1. `test_sell_usdc_all_balance`
2. `test_sell_usdc_to_ugx`
3. `test_send_usdc_to_valid_address`
4. `test_send_usdc_zero_amount`
5. `test_usdc_buy_then_sell`
6. `test_usdc_buy_then_send`

**Evidence:**
```
Expected: USDC sell/send success or validation
Got: Various errors or wrong menu states
```

**Analysis:**
- Similar to Bitcoin issues but more widespread
- Send and sell both affected
- Zero amount validation missing
- Confirmation step issues similar to Bitcoin

**Fix Required:**
- Review `src/flows/usdc/sell.rs` and `src/flows/usdc/send.rs`
- Add zero amount validation
- Fix confirmation step integration
- Check ICRC-2 approval system integration

### Category 7: Stateless USSD Issues (2 failures)
**Root Cause:** Session management issues preventing true stateless behavior.

Tests affected:
1. `test_ussd_idempotency`
2. `test_ussd_is_stateless`

**Evidence:**
```
Expected: Idempotent responses for same input
Got: Different responses or session conflicts
```

**Analysis:**
- Tests verify that USSD can work statelessly (using `*` shorthand)
- Session state may be interfering with stateless mode
- May need session-less routing mode

**Fix Required:**
- Review session management in `src/api/ussd.rs`
- Check if shorthand inputs can bypass session state
- Implement true stateless mode for asterisk (*) separated inputs

### Category 8: Amount Validation (1 failure)
**Root Cause:** Positive amount validation test failing.

Test affected:
1. `test_amount_validation_positive`

**Evidence:**
```
Expected: Validation to accept positive amounts
Got: Unknown error or rejection
```

**Analysis:**
- Test checks that positive amounts are accepted
- Validation logic may be too strict or has bug
- Related to other amount validation issues

**Fix Required:**
- Review amount validation in `src/validation/mod.rs`
- Ensure positive numbers are correctly accepted
- Check for edge cases in parsing

### Category 9: Unit Test - Rate Limiting (2 failures)
**Root Cause:** Unit tests calling IC system API outside canister context.

Tests affected:
1. `test_rate_limit_allows_first_request`
2. `test_rate_limit_in_test_mode`

**Evidence:**
```
panicked at ic0-1.0.1/src/sys.rs:329:9:
time should only be called inside canisters.
```

**Analysis:**
- Unit tests trying to call `ic_cdk::api::time()` directly
- This only works in canister runtime, not unit tests
- Need to mock time or skip these as unit tests

**Fix Required:**
- Convert to integration tests (which run in PocketIC)
- OR mock time function for unit tests
- OR use `#[cfg(target_arch = "wasm32")]` guards

## Priority Fixes

### High Priority (Blocking Major Features)
1. **Balance Check Routing** (6 tests) - Core feature completely broken
2. **Withdraw Routing** (5 tests) - Critical cash-out feature broken
3. **USDC Flows** (6 tests) - Major crypto feature broken

### Medium Priority (Partial Feature Issues)
4. **Bitcoin Sell** (3 tests) - Buy works, but sell broken
5. **DAO Flows** (2 tests) - Governance feature not accessible
6. **Stateless USSD** (2 tests) - UX issue, not core functionality

### Low Priority (Minor Issues)
7. **Withdraw Zero Validation** (1 test) - Edge case handling
8. **Amount Validation** (1 test) - Edge case handling
9. **Rate Limit Unit Tests** (2 tests) - Test infrastructure issue

## Recommended Fix Order

### Phase 1: Routing Architecture (Fix ~17 tests)
1. Analyze current routing logic in `src/core/routing.rs`
2. Map all expected shorthand codes to flows
3. Fix balance check routing (6 tests)
4. Fix withdraw routing (5 tests)
5. Fix DAO routing (2 tests)
6. Fix stateless routing (2 tests)
7. Add amount validation (2 tests)

**Expected Result:** 303/313 passing (96.8%)

### Phase 2: Crypto Flow Confirmation (Fix ~9 tests)
1. Review Bitcoin sell confirmation in `src/flows/bitcoin/sell.rs`
2. Review USDC sell confirmation in `src/flows/usdc/sell.rs`
3. Review USDC send confirmation in `src/flows/usdc/send.rs`
4. Ensure confirmation step doesn't break flow state
5. Fix Bitcoin sell (3 tests)
6. Fix USDC flows (6 tests)

**Expected Result:** 312/313 passing (99.7%)

### Phase 3: Test Infrastructure (Fix ~1 test)
1. Move rate limit tests to integration or add mocks
2. Fix amount validation edge case

**Expected Result:** 313/313 passing (100%)

## Architecture Insights

### Routing System
The current routing system has two modes:
1. **Session-based:** Traditional USSD menu navigation (tracked in session state)
2. **Shorthand:** Direct access via asterisk-separated codes (e.g., `1*2*3`)

**Issue:** Shorthand routing is not properly mapped to all flows.

**Solution:** Create a comprehensive routing table that maps:
- `1` → Local Currency Menu
- `1*1` → Send Money
- `1*2` → Check Balance
- `1*3` → Deposit
- `1*4` → Withdraw
- `2` → Bitcoin Menu
- `2*1` → Buy Bitcoin
- etc.

### Session State Management
Sessions track:
- `current_menu`: Which flow is active
- `step`: Current step within flow
- `data`: Flow-specific state (recipient, amount, etc.)

**Issue:** Shorthand inputs may not properly initialize or respect session state.

**Solution:** Ensure shorthand routing:
1. Creates appropriate session state
2. Sets correct `current_menu` and `step`
3. Validates state transitions
4. Handles stateless mode (no session creation)

### Confirmation Step Integration
Recent changes added confirmation steps to sell flows.

**Issue:** Confirmation may reset session or not properly track state.

**Solution:**
1. Confirmation should be a distinct `step` in the flow
2. Session must persist through confirmation
3. Back button should return to previous step, not reset flow

## Files Requiring Changes

### Critical Files
1. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/core/routing.rs`
   - Fix shorthand routing logic
   - Add comprehensive route mapping
   - Handle stateless mode

2. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/flows/local_currency/mod.rs`
   - Fix balance check routing
   - Fix withdraw routing

3. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/flows/local_currency/withdraw.rs`
   - Add zero amount validation
   - Fix routing pattern

4. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/flows/bitcoin/sell.rs`
   - Fix confirmation step state management

5. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/flows/usdc/sell.rs`
   - Fix confirmation step state management

6. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/flows/usdc/send.rs`
   - Fix confirmation step state management
   - Add zero amount validation

### Secondary Files
7. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/flows/dao/mod.rs`
   - Implement or fix DAO flows

8. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/src/validation/mod.rs`
   - Fix amount validation edge cases

9. `/Users/sdicola/CascadeProjects/afritokeni-mvp/canisters/ussd_canister/tests/unit/rate_limit_tests.rs`
   - Convert to integration tests or add mocks

## Next Steps

1. Start with routing.rs analysis and documentation
2. Create routing table mapping all expected codes
3. Fix balance check routing first (highest impact)
4. Test and iterate
5. Move to withdraw, then crypto flows
6. Address edge cases last

## Test Commands

```bash
# Run specific test category
cargo test --test lib integration::balance_check_tests -- --test-threads=1

# Run single test
cargo test --test lib integration::balance_check_tests::test_local_currency_balance_check -- --test-threads=1

# Run all integration tests
cargo test --test lib integration -- --test-threads=1

# Run with output
cargo test --test lib integration::balance_check_tests -- --test-threads=1 --nocapture
```
