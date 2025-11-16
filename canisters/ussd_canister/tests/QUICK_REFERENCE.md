# USSD Test Quick Reference Card

## Writing a New Test

```rust
#[test]
fn test_my_feature() {
    let env = get_test_env();           // Get shared environment
    let sess = session();                // Unique session ID
    let phone = &phone("UGX");          // Unique phone number
    
    // Setup user with balances
    env.setup_test_user_with_balances(
        phone, "First", "Last", "email@test.com",
        "UGX", "1234",  // currency, PIN
        100000,         // fiat balance
        0,              // BTC balance (satoshis)
        0               // USDC balance (cents)
    ).expect("Setup");
    
    // Execute USSD flow
    let (response, _) = env.process_ussd(&sess, phone, "1*1*+256700123456*50000*1234");
    
    // Assert results
    assert!(response.contains("success"));
}
```

## USSD Path Cheat Sheet

| Flow | Path Format | Example |
|------|-------------|---------|
| **Send Money** | `1*1*{recipient}*{amount}*{pin}` | `1*1*+256700123456*50000*1234` |
| **Check Balance** | `1*2` | `1*2` |
| **Withdraw** | `1*4*{amount}*{agent}*{pin}` | `1*4*100000*AGENT001*1234` |
| **Buy Bitcoin** | `2*3*{amount}*{pin}` | `2*3*100000*1234` |
| **Send Bitcoin** | `2*5*{address}*{amount}*{pin}` | `2*5*rrkah-fqaaa-aaaaa-aaaaq-cai*50000*1234` |
| **Buy USDC** | `3*3*{amount}*{pin}` | `3*3*100000*1234` |
| **Send USDC** | `3*5*{address}*{amount}*{pin}` | `3*5*rrkah-fqaaa-aaaaa-aaaaq-cai*50000*1234` |
| **Swap BTC→USDC** | `4*1*2*{amount}*1*{pin}` | `4*1*2*50000*1*1234` |
| **Swap USDC→BTC** | `4*2*1*{amount}*1*{pin}` | `4*2*1*50000*1*1234` |

## Helper Functions

### Phone Number Generator
```rust
let phone = &phone("UGX");  // Uganda
let phone = &phone("KES");  // Kenya
let phone = &phone("TZS");  // Tanzania
let phone = &phone("NGN");  // Nigeria
```
Generates unique phone based on test name hash.

### Session ID Generator
```rust
let sess = session();  // Unique per test
```

### Check Balances
```rust
// Fiat balance (returns in currency units, not cents)
let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");

// Crypto balance (returns (btc_satoshis, usdc_cents))
let (btc, usdc) = env.get_crypto_balance(phone).expect("Get balance");
```

### Set Balances (for test setup)
```rust
// Fiat (amount in currency units, converted to cents internally)
env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");

// Crypto (amounts in satoshis and cents)
env.set_crypto_balance(phone, 100000, 50000).expect("Set balance");
```

## Common Patterns

### Test Successful Transaction
```rust
let (response, _) = env.process_ussd(&sess, phone, "1*1*+256700123456*50000*1234");
assert!(response.contains("success") || response.contains("Success"));

let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
assert_eq!(balance, 50000); // Verify balance changed
```

### Test Error Handling
```rust
let (response, _) = env.process_ussd(&sess, phone, "1*1*+256700123456*999999*1234");
assert!(response.contains("Insufficient") || response.contains("insufficient"));
```

### Test Multiple Currencies
```rust
for currency in &["UGX", "KES", "TZS", "NGN"] {
    let phone = &format!("{}1", phone(currency));
    env.setup_test_user_with_balances(
        phone, "Test", "User", "test@test.com",
        currency, "1234", 100000, 0, 0
    ).expect("Setup");
    
    // Test with this currency...
}
```

## Amount Units

| Type | Unit | Example |
|------|------|---------|
| **Fiat** | Currency units (UGX, KES, etc.) | `100000` = 100,000 UGX |
| **Bitcoin** | Satoshis (1 BTC = 100M sats) | `100000` = 0.001 BTC |
| **USDC** | Cents (1 USDC = 100 cents) | `100000` = 1,000 USDC |

**Note**: `setup_test_user_with_balances` and `set_fiat_balance` take amounts in currency units, but internally convert to cents. `check_fiat_balance` returns currency units.

## Running Tests

```bash
# Run all integration tests
cargo test --test lib -- --test-threads=1

# Run specific test file
cargo test --test lib integration::send_money_flow_tests -- --test-threads=1

# Run specific test
cargo test --test lib test_send_money_flow_complete -- --test-threads=1 --nocapture

# Show output
cargo test --test lib -- --test-threads=1 --nocapture
```

## Debugging Tips

1. **Use `--nocapture`** to see `ic_cdk::println!` output
2. **Check balance before and after** transactions
3. **Print the response** if assertion fails:
   ```rust
   assert!(response.contains("success"), "Got: {}", response);
   ```
4. **Verify user exists** before testing:
   ```rust
   let user = env.get_user(phone).expect("Get user");
   assert!(user.is_some(), "User should exist");
   ```

## Common Mistakes

❌ **Missing submenu**: `1*+256700123456*50000*1234`  
✅ **Correct**: `1*1*+256700123456*50000*1234`

❌ **Missing PIN**: `4*1*2*50000*1`  
✅ **Correct**: `4*1*2*50000*1*1234`

❌ **Wrong menu number**: `2*2*100000*1234` (Bitcoin Rate, not Buy)  
✅ **Correct**: `2*3*100000*1234` (Buy Bitcoin)

❌ **Hardcoded phone**: `let phone = "+256700123456";`  
✅ **Correct**: `let phone = &phone("UGX");`

## Need More Details?

See `USSD_MENU_PATHS.md` for complete menu structure and all path formats.
