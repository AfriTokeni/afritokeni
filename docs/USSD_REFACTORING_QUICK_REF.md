# USSD Flow Refactoring - Quick Reference

## 🎯 The Pattern (Copy-Paste Template)

```rust
pub async fn handle_FLOW_NAME(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // 1. Extract ALL parameters upfront
    let param1_opt = parts.get(INDEX).map(|s| s.trim()).filter(|s| !s.is_empty());
    let param2_opt = parts.get(INDEX).map(|s| s.trim()).filter(|s| !s.is_empty());
    let param3_opt = parts.get(INDEX).map(|s| s.trim()).filter(|s| !s.is_empty());

    // 2. SHORTHAND MODE: If all params present, execute
    if let (Some(p1), Some(p2), Some(p3)) = (param1_opt, param2_opt, param3_opt) {
        return execute_FLOW_NAME(session, p1, p2, p3, lang).await;
    }

    // 3. INTERACTIVE MODE: Guide through missing steps
    if param1_opt.is_none() {
        return (format!("Enter param1:"), true);
    }
    if param2_opt.is_none() {
        session.set_data("param1", param1_opt.unwrap());
        return (format!("Enter param2:"), true);
    }
    if param3_opt.is_none() {
        session.set_data("param1", param1_opt.unwrap());
        session.set_data("param2", param2_opt.unwrap());
        return (format!("Enter param3:"), true);
    }

    (format!("Invalid input"), false)
}

// 4. Extraction function (validation + execution)
async fn execute_FLOW_NAME(
    session: &mut UssdSession,
    param1: &str,
    param2: &str,
    param3: &str,
    lang: Language,
) -> (String, bool) {
    // Validate all params
    // Call canister
    // Clear session
    // Return result
}
```

---

## 📋 Flow Checklist

### 1. Withdraw (1*4)
- [ ] Parameters: AMOUNT → AGENT → PIN
- [ ] File: `flows/local_currency/withdraw.rs`
- [ ] Test: `integration::withdraw_flow_tests`
- [ ] Status: ✅ Refactored (needs debug)

### 2. Deposit (1*3)
- [ ] Parameters: AGENT → AMOUNT → CONFIRMATION
- [ ] File: `flows/local_currency/deposit.rs`
- [ ] Test: `integration::deposit_flow_tests`
- [ ] Status: ❌ Needs refactoring

### 3. Bitcoin Buy (2*2*1)
- [ ] Parameters: AMOUNT → PIN
- [ ] File: `flows/bitcoin/buy.rs`
- [ ] Test: `integration::bitcoin_flow_tests`
- [ ] Status: ❌ Needs refactoring

### 4. Bitcoin Sell (2*2*2)
- [ ] Parameters: AMOUNT → CONFIRMATION → PIN
- [ ] File: `flows/bitcoin/sell.rs`
- [ ] Test: `integration::bitcoin_complete_tests`
- [ ] Status: ❌ Needs refactoring

### 5. Bitcoin Send (2*3)
- [ ] Parameters: RECIPIENT → AMOUNT → PIN
- [ ] File: `flows/bitcoin/send.rs`
- [ ] Test: `integration::bitcoin_flow_tests`
- [ ] Status: ❌ Needs refactoring

### 6. USDC Buy (3*2*1)
- [ ] Parameters: AMOUNT → PIN
- [ ] File: `flows/usdc/buy.rs`
- [ ] Test: `integration::usdc_flow_tests`
- [ ] Status: ❌ Needs refactoring

### 7. USDC Sell (3*2*2)
- [ ] Parameters: AMOUNT → CONFIRMATION → PIN
- [ ] File: `flows/usdc/sell.rs`
- [ ] Test: `integration::usdc_complete_tests`
- [ ] Status: ❌ Needs refactoring

### 8. USDC Send (3*3)
- [ ] Parameters: RECIPIENT → AMOUNT → PIN
- [ ] File: `flows/usdc/send.rs`
- [ ] Test: `integration::usdc_flow_tests`
- [ ] Status: ❌ Needs refactoring

### 9. Crypto Swap (4)
- [ ] Parameters: FROM → TO → AMOUNT → PIN
- [ ] File: `flows/crypto/swap.rs`
- [ ] Test: `integration::crypto_swap_*_tests`
- [ ] Status: ❌ Needs refactoring

### 10. DAO View Proposals (5*1)
- [ ] Parameters: Varies
- [ ] File: `flows/dao/*.rs`
- [ ] Test: `integration::dao_flow_tests`
- [ ] Status: ❌ Needs refactoring

---

## 🔍 Debug Commands

```bash
# Compile
cargo build

# Test specific flow
cargo test --test lib integration::FLOW_tests -- --test-threads=1

# Test with full output
cargo test --test lib integration::FLOW_tests::test_name -- --nocapture

# See all failures
cargo test --test lib 2>&1 | grep -E "(FAILED|test result:)"

# Count passing/failing
cargo test --test lib 2>&1 | grep "test result:" | tail -1
```

---

## 🐛 Common Errors & Fixes

| Error Message | Cause | Fix |
|---------------|-------|-----|
| "Withdrawal cancelled" | Wrong parameter order | Check test file for actual order |
| "Invalid input" | Shorthand detection broken | Verify all params extracted correctly |
| Compilation error | Missing import | Add `use crate::services::*;` |
| Test timeout | Infinite loop | Check session.clear_data() in all paths |
| Wrong error message | Test expects specific text | Match test expectation exactly |

---

## 📊 Progress Tracking

```bash
# Before: 119 failing / 238 total (50%)
# Target: 0 failing / 238 total (100%)

# Run this to check current status:
cargo test --test lib 2>&1 | grep "test result:" | tail -1

# Example output:
# test result: FAILED. 150 passed; 88 failed; 0 ignored
#              ^^^^^^        ^^^^^^    ^^^^^^^^^^
#              Status    Passing    Failing
```

---

## ⚡ Speed Tips

1. **Work in Order**: Deposit → Bitcoin Buy → Bitcoin Sell → USDC (similar patterns)
2. **Copy-Paste**: Use template above, adjust indexes
3. **Test Often**: Compile + test after each small change
4. **Debug Logging**: Add `ic_cdk::println!` liberally
5. **Check Tests First**: Look at test file to confirm parameter order

---

## 🎓 Learning Resources

- Full Guide: `docs/USSD_FLOW_REFACTORING_GUIDE.md`
- Withdraw Example: `canisters/ussd_canister/src/flows/local_currency/withdraw.rs`
- Test Examples: `canisters/ussd_canister/tests/integration/*_tests.rs`

---

**Quick Start**: Copy template → Adjust parameters → Test → Debug → Repeat
