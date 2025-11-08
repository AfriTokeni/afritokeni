# AfriTokeni 3-Tier Architecture Refactoring Plan

## Current Status
✅ **Working System**: USSD + Data Canister integrated with real ledgers and exchange rates  
⚠️ **Issue**: Business logic mixed in data canister (not ideal for scaling)

## Target Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  PRESENTATION LAYER                      │
│  (UI/UX Only - No Business Logic)                       │
├──────────────────────┬──────────────────────────────────┤
│   USSD Canister      │      Web App (SvelteKit)         │
│   • Parse USSD       │      • React Components          │
│   • Format response  │      • UI State Management       │
│   • Call BL API      │      • Call BL API               │
└──────────┬───────────┴──────────────┬───────────────────┘
           │                          │
           │    SAME BACKEND API      │
           └────────────┬─────────────┘
                        ▼
┌─────────────────────────────────────────────────────────┐
│            BUSINESS LOGIC CANISTER                       │
│  (All Business Rules - Shared by USSD & Web)            │
├─────────────────────────────────────────────────────────┤
│  • transfer_money(from, to, amount, currency, pin)      │
│  • buy_crypto(user, fiat_amount, crypto_type, pin)      │
│  • send_crypto(user, to_address, amount, type, pin)     │
│  • get_balances(user)                                   │
│  • register_user(...)                                   │
│                                                          │
│  ORCHESTRATION:                                         │
│  1. Validate inputs                                     │
│  2. Get user from Data Canister                         │
│  3. Verify PIN                                          │
│  4. Check balance                                       │
│  5. Fraud detection                                     │
│  6. Execute transaction                                 │
│  7. Update balances in Data Canister                    │
│  8. Call ckBTC/ckUSDC ledgers if needed                 │
│  9. Record transaction in Data Canister                 │
│  10. Return result                                      │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              DATA CANISTER (Pure CRUD)                   │
│  (No Business Logic - Just Storage)                     │
├─────────────────────────────────────────────────────────┤
│  • get_user(id) -> User                                 │
│  • set_user(id, data)                                   │
│  • get_balance(user_id, currency) -> u64               │
│  • set_balance(user_id, currency, amount)              │
│  • get_pin_hash(user_id) -> String                     │
│  • set_pin_hash(user_id, hash, salt)                   │
│  • store_transaction(tx)                                │
│  • get_transactions(user_id) -> Vec<Transaction>        │
│                                                          │
│  NO VALIDATION, NO FRAUD DETECTION, NO BUSINESS RULES   │
└─────────────────────────────────────────────────────────┘
```

## Implementation Steps

### Phase 1: Business Logic Canister (NEW)

**File Structure:**
```
canisters/business_logic_canister/
├── Cargo.toml
└── src/
    ├── lib.rs                          # Public API
    ├── models.rs                       # Shared types
    └── services/
        ├── mod.rs
        ├── config.rs                   # Canister IDs config
        ├── data_client.rs              # Calls Data Canister
        ├── ledger_client.rs            # Calls ckBTC/ckUSDC
        ├── exchange_rate_client.rs     # Gets rates
        ├── money_transfer.rs           # Fiat transfer logic
        ├── crypto_operations.rs        # Crypto buy/send logic
        ├── balance_queries.rs          # Balance checks
        ├── user_management.rs          # User registration
        ├── transaction_history.rs      # Transaction queries
        └── fraud_detection.rs          # Fraud checks
```

**Key Services:**

#### `money_transfer.rs`
```rust
pub async fn transfer_money(
    from: String,
    to: String,
    amount: u64,
    currency: String,
    pin: String,
) -> Result<TransactionResult, String> {
    // 1. Get users from data canister
    let from_user = data_client::get_user_by_identifier(&from).await?;
    let to_user = data_client::get_user_by_identifier(&to).await?;
    
    // 2. Verify PIN
    let pin_verified = data_client::verify_pin(&from_user.id, &pin).await?;
    if !pin_verified {
        return Err("Invalid PIN".to_string());
    }
    
    // 3. Check balance
    let balance = data_client::get_balance(&from_user.id, &currency).await?;
    if balance < amount {
        return Err("Insufficient balance".to_string());
    }
    
    // 4. Fraud detection
    fraud_detection::check_transaction(&from_user.id, amount, &currency)?;
    
    // 5. Update balances
    data_client::set_balance(&from_user.id, &currency, balance - amount).await?;
    let to_balance = data_client::get_balance(&to_user.id, &currency).await?;
    data_client::set_balance(&to_user.id, &currency, to_balance + amount).await?;
    
    // 6. Record transaction
    let tx = Transaction {
        id: generate_tx_id(),
        from_user: from_user.id.clone(),
        to_user: to_user.id.clone(),
        amount,
        currency: currency.clone(),
        timestamp: ic_cdk::api::time(),
    };
    data_client::store_transaction(&tx).await?;
    
    // 7. Return result
    Ok(TransactionResult {
        transaction_id: tx.id,
        from_user: from_user.id,
        to_user: to_user.id,
        amount,
        currency,
        new_balance: balance - amount,
        timestamp: tx.timestamp,
    })
}
```

#### `crypto_operations.rs`
```rust
pub async fn buy_crypto(
    user_id: String,
    fiat_amount: u64,
    fiat_currency: String,
    crypto_type: CryptoType,
    pin: String,
) -> Result<TransactionResult, String> {
    // 1. Get user and verify PIN
    let user = data_client::get_user(&user_id).await?;
    let verified = data_client::verify_pin(&user.id, &pin).await?;
    if !verified { return Err("Invalid PIN".to_string()); }
    
    // 2. Get exchange rate
    let rates = exchange_rate_client::get_rates().await?;
    let crypto_amount = match crypto_type {
        CryptoType::CkBTC => rates.fiat_to_btc(fiat_amount, &fiat_currency)?,
        CryptoType::CkUSDC => rates.fiat_to_usdc(fiat_amount, &fiat_currency)?,
    };
    
    // 3. Check fiat balance
    let fiat_balance = data_client::get_balance(&user.id, &fiat_currency).await?;
    if fiat_balance < fiat_amount {
        return Err("Insufficient fiat balance".to_string());
    }
    
    // 4. Fraud check
    fraud_detection::check_transaction(&user.id, fiat_amount, &fiat_currency)?;
    
    // 5. Deduct fiat
    data_client::set_balance(&user.id, &fiat_currency, fiat_balance - fiat_amount).await?;
    
    // 6. Add crypto (in data canister tracking)
    let crypto_balance = data_client::get_crypto_balance(&user.id, crypto_type).await?;
    data_client::set_crypto_balance(&user.id, crypto_type, crypto_balance + crypto_amount).await?;
    
    // 7. Call actual ledger (ckBTC or ckUSDC)
    ledger_client::mint_to_user(&user, crypto_type, crypto_amount).await?;
    
    // 8. Record transaction
    data_client::store_transaction(&tx).await?;
    
    Ok(result)
}
```

### Phase 2: Refactor Data Canister (SIMPLIFY)

**Remove from Data Canister:**
- ❌ `transfer_fiat()` - move to Business Logic
- ❌ `deposit_fiat()` - move to Business Logic
- ❌ Fraud detection - move to Business Logic
- ❌ Balance validation - move to Business Logic

**Keep in Data Canister (Pure CRUD):**
- ✅ `get_user(id)` 
- ✅ `set_user(id, data)`
- ✅ `get_balance(user_id, currency)`
- ✅ `set_balance(user_id, currency, amount)`
- ✅ `get_pin_hash(user_id)`
- ✅ `verify_pin_hash(user_id, pin)` - just hash comparison
- ✅ `store_transaction(tx)`
- ✅ `get_transactions(user_id)`

### Phase 3: Update USSD Canister (SIMPLIFY)

**Before (Current):**
```rust
// USSD has business logic
let client = data_canister_client::create_client()?;
let user = client.get_user_by_phone(&phone).await?;
let verified = client.verify_user_pin(&user.id, pin).await?;
// ... more logic ...
client.transfer_fiat(...).await?;
```

**After (Clean):**
```rust
// USSD just calls business logic
let bl_client = business_logic_client::create_client()?;
let result = bl_client.transfer_money(
    phone,
    recipient_phone,
    amount,
    "UGX",
    pin
).await?;

// Format for USSD
format!("Success! Sent {} UGX to {}", amount, recipient_phone)
```

### Phase 4: Update Web App (SIMPLIFY)

**Before (Current):**
```typescript
// Web app calls Juno directly
const balance = await getBalance(userId);
```

**After (Clean):**
```typescript
// Web app calls Business Logic Canister (same as USSD!)
import { business_logic_canister } from '@/lib/canisters';

const balances = await business_logic_canister.get_balances(principalId);
const result = await business_logic_canister.transfer_money(
  myPrincipal,
  recipientPrincipal,
  amount,
  currency,
  pin
);
```

## Benefits of This Architecture

### 1. **Code Reuse**
- ✅ USSD and Web use **identical backend**
- ✅ Write business logic **once**, use everywhere
- ✅ Easy to add mobile app later

### 2. **Separation of Concerns**
- ✅ USSD = UI/UX for feature phones
- ✅ Web = UI/UX for browsers
- ✅ Business Logic = All rules and validation
- ✅ Data = Pure storage

### 3. **Easier Testing**
- ✅ Test business logic independently
- ✅ Mock data canister for unit tests
- ✅ Test USSD/Web UI separately

### 4. **Better Security**
- ✅ All validation in one place
- ✅ Fraud detection centralized
- ✅ Easier to audit

### 5. **Scalability**
- ✅ Can upgrade business logic without touching UI
- ✅ Can add new UIs (mobile, API) easily
- ✅ Data canister stays simple and stable

## Migration Path

### Week 1: Build Business Logic Canister
- [ ] Implement all services
- [ ] Add comprehensive tests
- [ ] Deploy to testnet

### Week 2: Refactor Data Canister
- [ ] Remove business logic
- [ ] Simplify to pure CRUD
- [ ] Update tests

### Week 3: Update USSD
- [ ] Replace data canister calls with business logic calls
- [ ] Test all 27 scenarios
- [ ] Verify on testnet

### Week 4: Update Web App
- [ ] Replace Juno calls with business logic calls
- [ ] Test all web flows
- [ ] Verify both USSD and Web work

### Week 5: Deploy to Production
- [ ] Final testing
- [ ] Deploy all canisters
- [ ] Monitor Uganda launch

## Next Steps

1. **Complete business_logic_canister implementation**
2. **Simplify data_canister to pure CRUD**
3. **Update USSD to use business logic**
4. **Update Web to use business logic**
5. **Test everything**
6. **Deploy**

This is the right architecture for long-term success! 🚀
