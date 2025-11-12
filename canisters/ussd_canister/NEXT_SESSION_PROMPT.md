# Next Session Prompt - USSD Integration Tests

## Current Status

**Test Results**: 191/312 tests passing (61%) in full suite, 312/312 (100%) passing individually  
**Root Cause**: Shared test environment causes state pollution between tests  
**Solution**: Implemented but not yet applied to all tests

## Your Task

Update all USSD integration tests to use the new `setup_test_user_with_balances` helper function to achieve 100% pass rate in the full test suite.

## Background

All tests pass individually, proving the code is correct. However, when run together in the shared test environment, 121 tests fail due to balance state pollution from previous tests.

**Solution implemented**: New helper functions in `tests/integration/mod.rs`:
- `setup_test_user()` - Register user and reset all balances to zero
- `setup_test_user_with_balances()` - Register user and set specific balances

## Pattern to Replace

### OLD PATTERN (vulnerable to state pollution):
```rust
#[test]
fn test_buy_bitcoin_with_ugx() {
    let env = get_test_env();
    let phone = "+256700111111";
    
    env.register_user_direct(phone, "BTC", "Buyer", "btc@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 1000000).expect("Set balance");
    
    // Test logic...
}
```

### NEW PATTERN (clean state guaranteed):
```rust
#[test]
fn test_buy_bitcoin_with_ugx() {
    let env = get_test_env();
    let phone = "+256700111111";
    
    env.setup_test_user_with_balances(
        phone, "BTC", "Buyer", "btc@test.com", "UGX", "1234",
        1000000, // fiat balance
        0,       // btc balance
        0        // usdc balance
    ).expect("Setup");
    
    // Test logic...
}
```

## Files to Update

Update ALL test files in `canisters/ussd_canister/tests/integration/`:

1. ✅ `bitcoin_complete_tests.rs` - 1 test already updated as example
2. ❌ `usdc_complete_tests.rs` - ~22 tests
3. ❌ `balance_complete_tests.rs` - ~23 tests
4. ❌ `crypto_swap_complete_tests.rs` - ~25 tests
5. ❌ `withdraw_complete_tests.rs` - ~30 tests
6. ❌ `bitcoin_flow_tests.rs` - ~10 tests
7. ❌ `usdc_flow_tests.rs` - ~10 tests
8. ❌ `balance_check_tests.rs` - ~7 tests
9. ❌ `withdraw_flow_tests.rs` - ~10 tests
10. ❌ `send_money_flow_tests.rs` - ~15 tests
11. ❌ `dao_flow_tests.rs` - ~10 tests
12. ❌ `language_flow_tests.rs` - ~8 tests
13. ❌ `main_menu_tests.rs` - ~5 tests
14. ❌ `error_security_tests.rs` - ~10 tests
15. ❌ `registration_flow_tests.rs` - ~9 tests
16. ❌ Other test files as needed

## Replacement Strategy

For each test file:

1. **Identify the pattern**: Look for tests with:
   - `env.register_user_direct(...)`
   - Followed by `env.set_fiat_balance(...)` and/or `env.set_crypto_balance(...)`

2. **Replace with helper**:
   - If setting fiat balance only: `setup_test_user_with_balances(..., fiat, 0, 0)`
   - If setting crypto only: `setup_test_user_with_balances(..., 0, btc, usdc)`
   - If setting both: `setup_test_user_with_balances(..., fiat, btc, usdc)`
   - If no balances needed: `setup_test_user(...)`

3. **Handle edge cases**:
   - Tests that set balances mid-test: Keep those `set_*_balance` calls
   - Tests with multiple users: Apply pattern to each user
   - Tests that don't register users: Leave as-is

## Verification

After updating all tests, run:

```bash
# Full test suite - should show 312/312 passing
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml

# Expected result:
# test result: ok. 312 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Example: Complete Test Update

**Before**:
```rust
#[test]
fn test_sell_bitcoin_to_ugx() {
    let env = get_test_env();
    let phone = "+256700999999";
    
    env.register_user_direct(phone, "BTC", "Seller", "btcsell@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_crypto_balance(phone, 200000, 0).expect("Set BTC balance");
    
    let (response, _) = env.process_ussd("session", phone, "2*4*0.001*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
    
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should decrease by 100,000 sats");
}
```

**After**:
```rust
#[test]
fn test_sell_bitcoin_to_ugx() {
    let env = get_test_env();
    let phone = "+256700999999";
    
    env.setup_test_user_with_balances(
        phone, "BTC", "Seller", "btcsell@test.com", "UGX", "1234",
        0,      // fiat balance
        200000, // btc balance (200,000 satoshis)
        0       // usdc balance
    ).expect("Setup");
    
    let (response, _) = env.process_ussd("session", phone, "2*4*0.001*1234");
    
    assert!(response.contains("success") || response.contains("Success"));
    
    let (btc, _) = env.get_crypto_balance(phone).expect("Get balance");
    assert_eq!(btc, 100000, "BTC should decrease by 100,000 sats");
}
```

## Tips for Efficiency

1. **Use find & replace with regex** in your editor
2. **Process one file at a time** and verify it compiles
3. **Run tests frequently** to catch issues early
4. **Start with smaller test files** to build confidence
5. **Keep the pattern consistent** across all files

## Success Criteria

✅ All 312 tests pass in full suite run  
✅ Test execution time remains ~30 seconds  
✅ No compilation errors  
✅ All tests use the new helper pattern consistently

## Reference Documentation

- **Test helpers**: `canisters/ussd_canister/tests/integration/mod.rs` (lines 352-398)
- **Example updated test**: `bitcoin_complete_tests.rs` line 14
- **Full analysis**: `FINAL_TEST_REPORT.md`

## Estimated Time

**2-4 hours** depending on approach:
- Manual updates: ~4 hours
- Semi-automated (regex): ~2 hours

Good luck! This is straightforward but requires attention to detail. The pattern is simple and consistent across all tests.
