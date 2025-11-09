# Business Logic Canister - Full Refactoring Plan

## Status: FOUNDATION ADDED, FULL REFACTORING NEEDED

### What's Done:
- ✅ logic.rs created with core validation functions
- ✅ Pocket-IC dependency added
- ✅ 131 tests passing (all existing unit + new logic tests)
- ✅ WASM builds successfully

### What Needs Refactoring:

#### 1. Services to Extract Pure Logic From:

**money_transfer.rs** (4852 bytes)
- transfer_money()
- send_money_to_phone()
- withdraw_fiat()
- EXTRACT: amount validation, balance checks, fee calculation

**crypto_operations.rs** (6983 bytes)
- buy_crypto()
- send_crypto()
- EXTRACT: address validation, amount conversion, fee calculation

**user_management.rs** (2447 bytes)
- register_user()
- verify_pin()
- change_pin()
- link_phone_to_account()
- EXTRACT: PIN validation, phone format, identifier parsing

**fraud_detection.rs** (8143 bytes)
- check_suspicious_transaction()
- detect_patterns()
- EXTRACT: threshold checks, pattern detection logic

**balance_queries.rs** (2213 bytes)
- get_balances()
- check_fiat_balance()
- check_crypto_balance()
- EXTRACT: balance calculation logic

#### 2. Refactoring Pattern (Same as Withdrawal/Deposit):

```rust
// BEFORE:
async fn transfer_money(...) {
    if amount == 0 {
        return Err("Amount must be > 0");
    }
    // ... I/O calls
}

// AFTER:
async fn transfer_money(...) {
    logic::validate_amount_positive(amount)?;
    // ... I/O calls
}
```

#### 3. Integration Tests Needed:

**User Management:**
- test_register_user_success
- test_register_user_duplicate_phone
- test_verify_pin_success
- test_verify_pin_wrong
- test_change_pin_success

**Money Transfer:**
- test_transfer_money_success
- test_transfer_insufficient_balance
- test_transfer_to_self_fails
- test_send_money_to_phone_success

**Crypto Operations:**
- test_buy_crypto_success
- test_buy_crypto_insufficient_funds
- test_send_crypto_success
- test_send_crypto_invalid_address

**Fraud Detection:**
- test_suspicious_amount_flagged
- test_rapid_transactions_flagged
- test_normal_transaction_passes

#### 4. Estimated Work:

- **Time**: 2-3 hours dedicated session
- **Lines to refactor**: ~25,000 lines
- **New logic modules**: 5-6 separate logic modules
- **Integration tests**: 20-30 tests
- **Target coverage**: 80%+

#### 5. Dependencies:

This canister calls:
- Data Canister (for storage)
- ckBTC Ledger (for Bitcoin)
- ckUSDC Ledger (for USDC)

Integration tests will need to mock or set up these dependencies.

#### 6. Priority Order:

1. **User Management** (simplest, good starting point)
2. **Money Transfer** (core functionality)
3. **Crypto Operations** (complex but isolated)
4. **Fraud Detection** (pure logic, should be easy)
5. **Balance Queries** (mostly I/O, minimal logic)

### Commands for Next Session:

```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release --package business_logic_canister

# Run tests
cargo test --package business_logic_canister

# Run coverage
cargo llvm-cov --package business_logic_canister --tests --summary-only
```

### Success Criteria:

- ✅ All services refactored to use pure logic functions
- ✅ 20+ integration tests with Pocket-IC
- ✅ 80%+ code coverage on logic modules
- ✅ All existing 131 tests still passing
- ✅ WASM builds without warnings
- ✅ Clean, maintainable code structure

---

**This is the MOST IMPORTANT canister - it deserves proper attention!**
