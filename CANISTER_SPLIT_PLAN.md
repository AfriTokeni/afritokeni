# Business Logic Canister Split Plan

## Problem
- business_logic_canister: **1.9M / 2MB limit (95% full!)**
- Adding new features is blocked
- Monolithic design makes testing and maintenance difficult

## Solution: Domain-Driven Canisters

Split into 3 domain canisters based on business capabilities:

---

## 1. USER_CANISTER (~400KB)
**Domain:** User identity, authentication, profiles

### Endpoints to Move:
- `register_user()` - User registration
- `user_exists()` - Check if user exists
- `verify_pin()` - PIN authentication
- `change_pin()` - Update PIN
- `link_phone_to_account()` - Link phone to principal
- `get_user_profile()` - Get user details
- `update_user_profile()` - Update user info
- `get_user_by_phone()` - Lookup by phone
- `get_user_by_principal()` - Lookup by principal

### Logic Modules:
- `logic/user_logic.rs` (238 lines)
- Part of `services/data_client.rs` (user operations)

### Dependencies:
- Calls: data_canister
- Called by: USSD, Web, wallet_canister, crypto_canister

---

## 2. WALLET_CANISTER (~800KB)
**Domain:** Fiat money, transfers, fraud detection

### Endpoints to Move:
- `transfer_money()` - Fiat transfers
- `send_money_to_phone()` - Send to phone
- `send_money()` - Alias for send_money_to_phone
- `withdraw_fiat()` - Cash withdrawal via agent
- `get_transfer_fee()` - Calculate fees
- `check_fiat_balance()` - Check fiat balance
- `set_fiat_balance()` - Test helper
- `get_balances()` - Get all balances (fiat + crypto)
- `get_transaction_history()` - Transaction list
- `get_recent_transactions()` - Recent txs

### Logic Modules:
- `logic/transfer_logic.rs` (320 lines)
- `logic/fraud_logic.rs` (243 lines)
- `services/fraud_detection.rs` (221 lines)
- Part of `services/data_client.rs` (balance/transaction operations)
- Part of `services/commission_client.rs` (withdrawal commissions)

### Dependencies:
- Calls: data_canister, deposit_canister, withdrawal_canister, user_canister
- Called by: USSD, Web

---

## 3. CRYPTO_CANISTER (~700KB)
**Domain:** Cryptocurrency operations, swaps, escrow

### Endpoints to Move:
- `buy_crypto()` - Buy BTC/USDC with fiat
- `sell_bitcoin()` - Sell BTC for fiat
- `sell_usdc()` - Sell USDC for fiat
- `send_crypto()` - Send crypto to address
- `send_usdc()` - Send USDC (wrapper)
- `swap_crypto()` - Swap BTC ↔ USDC
- `get_crypto_value_estimate()` - Estimate fiat value
- `check_crypto_balance()` - Check crypto balance
- `set_crypto_balance()` - Test helper
- `sell_crypto_to_agent()` - Create escrow
- `verify_escrow_code()` - Complete escrow
- `get_escrow_status()` - Check escrow
- `cancel_escrow()` - Refund escrow

### Logic Modules:
- `logic/crypto_logic.rs` (288 lines)
- `services/crypto_operations.rs` (440 lines)
- `services/exchange_rate.rs` (283 lines)
- Part of `services/data_client.rs` (crypto balance operations)
- Part of `services/commission_client.rs` (deposit commissions)

### Dependencies:
- Calls: data_canister, exchange_canister, deposit_canister, user_canister
- Called by: USSD, Web

---

## Shared Infrastructure (Stays in Each Canister)

### Access Control:
- `add_authorized_canister()`
- `remove_authorized_canister()`
- `get_authorized_canisters()`
- `verify_authorized_caller()`

### Configuration:
- `set_data_canister_id()`
- `set_deposit_canister_id()`
- `set_withdrawal_canister_id()`
- `set_exchange_canister_id()`
- `set_user_canister_id()` - NEW
- `set_wallet_canister_id()` - NEW
- `set_crypto_canister_id()` - NEW

### Testing:
- `enable_test_mode()`
- `get_audit_log()`

---

## Inter-Canister Communication

### Wallet → User:
- `verify_pin()` - Authenticate before transfer
- `user_exists()` - Validate recipient

### Crypto → User:
- `verify_pin()` - Authenticate before crypto ops
- `user_exists()` - Validate user

### Crypto → Wallet:
- `check_fiat_balance()` - Verify funds for buy_crypto
- `update_fiat_balance()` - Deduct fiat when buying crypto

### USSD/Web → All:
- Direct calls to user_canister, wallet_canister, crypto_canister
- No more single business_logic_canister bottleneck

---

## Migration Steps

### Phase 1: Create Canister Structures (Day 1)
1. ✅ Create `canisters/user_canister/` directory
2. ✅ Create `canisters/wallet_canister/` directory
3. ✅ Create `canisters/crypto_canister/` directory
4. ✅ Copy base structure (Cargo.toml, lib.rs template)
5. ✅ Update `dfx.json` with new canisters

### Phase 2: Move Code (Day 1-2)
1. Move user logic to user_canister
2. Move wallet logic to wallet_canister
3. Move crypto logic to crypto_canister
4. Update imports and dependencies

### Phase 3: Update Clients (Day 2)
1. Update USSD canister to call new canisters
2. Update Web frontend to call new canisters
3. Create client modules for inter-canister calls

### Phase 4: Update Tests (Day 2-3)
1. Update integration tests for new architecture
2. Test inter-canister communication
3. Verify all flows work end-to-end

### Phase 5: Deploy & Verify (Day 3)
1. Deploy all canisters
2. Set canister IDs in each canister
3. Authorize USSD/Web canisters
4. Run smoke tests
5. Monitor WASM sizes

---

## Expected Results

### Before:
```
business_logic_canister: 1.9M (95% full) ⚠️
```

### After:
```
user_canister:    ~400KB (20% full) ✅
wallet_canister:  ~800KB (40% full) ✅
crypto_canister:  ~700KB (35% full) ✅
```

### Benefits:
- ✅ No more size limit issues
- ✅ Clear domain boundaries
- ✅ Independent scaling
- ✅ Easier testing
- ✅ Better team collaboration
- ✅ Faster development cycles

---

## Rollback Plan

If issues arise:
1. Keep old business_logic_canister deployed
2. Route traffic back to old canister
3. Fix issues in new canisters
4. Re-deploy when ready

---

## Timeline

- **Day 1 Morning:** Create structures, move user logic
- **Day 1 Afternoon:** Move wallet logic
- **Day 2 Morning:** Move crypto logic
- **Day 2 Afternoon:** Update USSD/Web clients
- **Day 3:** Update tests, deploy, verify

**Total:** 3 days for complete migration
