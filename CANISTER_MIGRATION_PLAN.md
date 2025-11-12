# Canister Migration Plan - 4 Domain Canisters

## Executive Summary

**Problem:** business_logic_canister is 1.9M (95% of 2MB limit)

**Solution:** Split into 4 domain-driven canisters

**Timeline:** 3-4 days

**Result:** 
- user_canister: 400KB (20% capacity)
- wallet_canister: 600KB (30% capacity)  
- agent_canister: 700KB (35% capacity)
- crypto_canister: 1.0M (50% capacity)

---

## Architecture Overview

### Before (6 canisters):
```
business_logic_canister (1.9M) ⚠️ TOO BIG
exchange_canister (1.1M)
deposit_canister (1.1M)
withdrawal_canister (1.0M)
data_canister (1.1M)
ussd_canister (1.7M)
```

### After (4 domain canisters):
```
user_canister (400KB)
wallet_canister (600KB)
agent_canister (700KB)
crypto_canister (1.0M)
data_canister (1.1M) - unchanged
ussd_canister (1.7M) - unchanged
```

---

## Phase 1: Create Canister Structures

### 1.1 Create user_canister
**Location:** `canisters/user_canister/`

**Files to create:**
```
user_canister/
├── Cargo.toml
├── user_canister.did
├── src/
│   ├── lib.rs (main API)
│   ├── logic/
│   │   └── user_logic.rs (from business_logic)
│   └── services/
│       └── data_client.rs (user operations only)
└── tests/
    └── integration/
        └── user_tests.rs
```

**Endpoints to implement:**
```rust
// From business_logic_canister/src/lib.rs
#[update] async fn register_user(request: RegisterUserRequest) -> Result<String, String>
#[update] async fn user_exists(user_identifier: String) -> Result<bool, String>
#[update] async fn verify_pin(user_identifier: String, pin: String) -> Result<bool, String>
#[update] async fn change_pin(user_identifier: String, old_pin: String, new_pin: String) -> Result<(), String>
#[update] async fn link_phone_to_account(principal_id: String, phone_number: String) -> Result<(), String>
#[query] fn get_user_profile(user_identifier: String) -> Result<UserProfile, String>
#[update] async fn update_user_profile(user_identifier: String, updates: ProfileUpdates) -> Result<(), String>
#[query] fn get_user_by_phone(phone: String) -> Result<User, String>
#[query] fn get_user_by_principal(principal: String) -> Result<User, String>
```

**Logic modules to move:**
- `business_logic_canister/src/logic/user_logic.rs` (238 lines)
- User-related functions from `services/data_client.rs`

**Dependencies:**
- Calls: data_canister
- Called by: wallet_canister, agent_canister, crypto_canister, ussd_canister, web

---

### 1.2 Create wallet_canister
**Location:** `canisters/wallet_canister/`

**Files to create:**
```
wallet_canister/
├── Cargo.toml
├── wallet_canister.did
├── wallet_config.toml (transfer fees, fraud limits)
├── src/
│   ├── lib.rs (main API)
│   ├── logic/
│   │   ├── transfer_logic.rs (from business_logic)
│   │   └── fraud_logic.rs (from business_logic)
│   └── services/
│       ├── data_client.rs (balance/transaction operations)
│       └── user_client.rs (NEW - calls user_canister)
└── tests/
    └── integration/
        └── wallet_tests.rs
```

**Endpoints to implement:**
```rust
// From business_logic_canister/src/lib.rs
#[update] async fn transfer_money(from: String, to: String, amount: u64, currency: String, pin: String) -> Result<TransactionResult, String>
#[update] async fn send_money_to_phone(from_phone: String, to_phone: String, amount: u64, currency: String, pin: String) -> Result<TransactionResult, String>
#[update] async fn send_money(from_phone: String, to_phone: String, amount: u64, currency: String, pin: String) -> Result<TransactionResult, String>
#[update] fn get_transfer_fee(amount: u64) -> Result<u64, String>
#[update] async fn check_fiat_balance(user_identifier: String, currency: String) -> Result<u64, String>
#[update] async fn set_fiat_balance(user_id: String, currency: String, amount: u64) -> Result<(), String>
#[update] async fn get_transaction_history(user_identifier: String, limit: Option<usize>) -> Result<Vec<Transaction>, String>
#[update] async fn get_recent_transactions(user_identifier: String) -> Result<Vec<Transaction>, String>
```

**Logic modules to move:**
- `business_logic_canister/src/logic/transfer_logic.rs` (320 lines)
- `business_logic_canister/src/logic/fraud_logic.rs` (243 lines)
- `business_logic_canister/src/services/fraud_detection.rs` (221 lines)
- Fiat balance functions from `services/data_client.rs`

**Dependencies:**
- Calls: data_canister, user_canister
- Called by: ussd_canister, web

---

### 1.3 Create agent_canister
**Location:** `canisters/agent_canister/`

**Files to create:**
```
agent_canister/
├── Cargo.toml
├── agent_canister.did
├── agent_config.toml (commission rates, settlement rules)
├── src/
│   ├── lib.rs (main API)
│   ├── deposits/
│   │   └── deposit_logic.rs (from deposit_canister)
│   ├── withdrawals/
│   │   └── withdrawal_logic.rs (from withdrawal_canister)
│   ├── agents/
│   │   ├── commission.rs (NEW - unified from both)
│   │   └── settlement.rs (NEW - unified from both)
│   └── services/
│       ├── data_client.rs (transaction operations)
│       └── user_client.rs (NEW - calls user_canister)
└── tests/
    └── integration/
        ├── deposit_tests.rs
        ├── withdrawal_tests.rs
        └── agent_tests.rs
```

**Endpoints to implement:**
```rust
// DEPOSITS (from deposit_canister/src/lib.rs)
#[update] fn create_deposit_request(request: CreateDepositRequest) -> Result<DepositTransaction, String>
#[update] fn confirm_deposit(request: ConfirmDepositRequest) -> Result<DepositTransaction, String>
#[query] fn get_deposit(id: u64) -> Option<DepositTransaction>
#[query] fn get_user_deposits(user: Principal) -> Vec<DepositTransaction>
#[query] fn get_agent_deposits(agent: Principal) -> Vec<DepositTransaction>
#[query] fn get_pending_deposits(agent: Principal) -> Vec<DepositTransaction>

// WITHDRAWALS (from withdrawal_canister/src/lib.rs + business_logic)
#[update] async fn withdraw_fiat(phone_number: String, amount: u64, currency: String, agent_id: String, pin: String) -> Result<TransactionResult, String>
#[update] fn create_withdrawal_request(request: CreateWithdrawalRequest) -> Result<WithdrawalTransaction, String>
#[update] fn confirm_withdrawal(request: ConfirmWithdrawalRequest) -> Result<WithdrawalTransaction, String>
#[update] async fn get_withdrawal_fees(amount: u64) -> Result<WithdrawalFeesResult, String>
#[query] fn get_withdrawal(id: u64) -> Option<WithdrawalTransaction>
#[query] fn get_user_withdrawals(user: Principal) -> Vec<WithdrawalTransaction>
#[query] fn get_agent_withdrawals(agent: Principal) -> Vec<WithdrawalTransaction>
#[query] fn get_pending_withdrawals(agent: Principal) -> Vec<WithdrawalTransaction>

// AGENT MANAGEMENT (NEW - unified from both canisters)
#[query] fn get_agent_balance(agent: Principal) -> Option<AgentBalance>
#[query] fn get_all_agent_balances() -> Vec<AgentBalance>
#[update] fn create_monthly_settlement(month: String) -> Result<Vec<MonthlySettlement>, String>
#[update] fn mark_settlement_paid(month: String, agent: Principal) -> Result<(), String>
#[query] fn get_settlements_for_month(month: String) -> Vec<MonthlySettlement>
#[query] fn get_agent_settlements(agent: Principal) -> Vec<MonthlySettlement>
#[query] fn get_total_revenue() -> u64
#[query] fn get_commission_rate() -> u64
```

**Code to move:**
- **ENTIRE** `deposit_canister/src/` (~1,258 lines)
- **ENTIRE** `withdrawal_canister/src/` (~838 lines)
- Commission functions from `business_logic_canister/src/services/commission_client.rs` (357 lines)
- `withdraw_fiat()` from `business_logic_canister/src/lib.rs`

**NEW unified agent management:**
- Merge agent balance tracking from both deposit & withdrawal canisters
- Single source of truth for agent commissions
- Unified settlement generation

**Dependencies:**
- Calls: data_canister, user_canister
- Called by: ussd_canister, web

---

### 1.4 Create crypto_canister
**Location:** `canisters/crypto_canister/`

**Files to create:**
```
crypto_canister/
├── Cargo.toml
├── crypto_canister.did
├── crypto_config.toml (spread, DEX settings)
├── src/
│   ├── lib.rs (main API)
│   ├── logic/
│   │   └── crypto_logic.rs (from business_logic)
│   ├── operations/
│   │   ├── buy_sell.rs (NEW - split from crypto_logic)
│   │   ├── transfers.rs (NEW - split from crypto_logic)
│   │   ├── swaps.rs (NEW - from exchange_canister)
│   │   └── escrow.rs (NEW - split from crypto_logic)
│   └── services/
│       ├── data_client.rs (crypto balance operations)
│       ├── user_client.rs (NEW - calls user_canister)
│       ├── wallet_client.rs (NEW - calls wallet_canister)
│       ├── exchange_rate.rs (from business_logic)
│       └── dex_client.rs (from exchange_canister)
└── tests/
    └── integration/
        ├── buy_sell_tests.rs
        ├── transfer_tests.rs
        ├── swap_tests.rs
        └── escrow_tests.rs
```

**Endpoints to implement:**
```rust
// BUY/SELL (from business_logic_canister/src/lib.rs)
#[update] async fn buy_crypto(user_identifier: String, fiat_amount: u64, currency: String, crypto_type: CryptoType, pin: String) -> Result<TransactionResult, String>
#[update] async fn sell_bitcoin(user_identifier: String, crypto_amount: u64, currency: String, pin: String) -> Result<TransactionResult, String>
#[update] async fn sell_usdc(user_identifier: String, crypto_amount: u64, currency: String, pin: String) -> Result<TransactionResult, String>
#[update] async fn get_crypto_value_estimate(crypto_amount: u64, crypto_type: CryptoType, currency: String) -> Result<u64, String>

// TRANSFERS (from business_logic_canister/src/lib.rs)
#[update] async fn send_crypto(user_identifier: String, to_address: String, amount: u64, crypto_type: CryptoType, pin: String) -> Result<TransactionResult, String>
#[update] async fn send_usdc(user_identifier: String, to_address: String, amount: u64, pin: String) -> Result<TransactionResult, String>
#[update] async fn check_crypto_balance(user_identifier: String, crypto_type: CryptoType) -> Result<u64, String>
#[update] async fn set_crypto_balance(user_id: String, ckbtc: u64, ckusdc: u64) -> Result<(), String>

// SWAPS (from business_logic + exchange_canister)
#[update] async fn swap_crypto(user_identifier: String, from_crypto: CryptoType, to_crypto: CryptoType, amount: u64, pin: String) -> Result<SwapResult, String>
#[query] fn get_spread_basis_points() -> u64
#[query] fn get_company_wallet() -> String
#[query] fn get_dex_provider() -> String

// ESCROW (from business_logic_canister/src/lib.rs)
#[update] async fn sell_crypto_to_agent(user_identifier: String, crypto_amount: u64, crypto_type: CryptoType, agent_id: String, pin: String) -> Result<EscrowResult, String>
#[update] async fn verify_escrow_code(code: String, agent_id: String, pin: String) -> Result<TransactionResult, String>
#[update] async fn get_escrow_status(code: String) -> Result<Escrow, String>
#[update] async fn cancel_escrow(code: String, user_id: String, pin: String) -> Result<(), String>
```

**Code to move:**
- `business_logic_canister/src/logic/crypto_logic.rs` (288 lines)
- `business_logic_canister/src/services/crypto_operations.rs` (440 lines)
- `business_logic_canister/src/services/exchange_rate.rs` (283 lines)
- **ENTIRE** `exchange_canister/src/` (~1,164 lines)
- Crypto balance functions from `services/data_client.rs`

**Dependencies:**
- Calls: data_canister, user_canister, wallet_canister, Sonic DEX
- Called by: ussd_canister, web

---

## Phase 2: Update Shared Types

### 2.1 Add Inter-Canister Types
**File:** `canisters/shared_types/src/lib.rs`

**Add new types:**
```rust
// User canister responses
pub struct UserProfile {
    pub phone_number: String,
    pub principal_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: FiatCurrency,
    pub kyc_status: KycStatus,
    pub created_at: u64,
}

pub struct ProfileUpdates {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub preferred_currency: Option<FiatCurrency>,
}

// Agent canister types
pub struct AgentBalance {
    pub agent: Principal,
    pub deposit_commission: u64,
    pub withdrawal_commission: u64,
    pub total_commission: u64,
    pub last_updated: u64,
}

pub struct MonthlySettlement {
    pub month: String,
    pub agent: Principal,
    pub amount: u64,
    pub paid: bool,
    pub paid_at: Option<u64>,
}

// Crypto canister types
pub struct SwapResult {
    pub from_amount: u64,
    pub to_amount: u64,
    pub spread_amount: u64,
    pub exchange_rate: f64,
}

pub struct EscrowResult {
    pub code: String,
    pub amount: u64,
    pub crypto_type: CryptoType,
    pub expires_at: u64,
}
```

---

## Phase 3: Update Client Canisters

### 3.1 Update USSD Canister
**File:** `canisters/ussd_canister/src/services/`

**Create new client modules:**
```
services/
├── user.rs (NEW - calls user_canister)
├── wallet.rs (NEW - calls wallet_canister)
├── agent.rs (NEW - calls agent_canister)
└── crypto.rs (NEW - calls crypto_canister)
```

**Update flow handlers:**
- `flows/local_currency/send_money.rs` → call wallet_canister
- `flows/local_currency/deposit.rs` → call agent_canister
- `flows/local_currency/withdraw.rs` → call agent_canister
- `flows/bitcoin/*.rs` → call crypto_canister
- `flows/usdc/*.rs` → call crypto_canister
- `flows/crypto/swap.rs` → call crypto_canister

**Configuration:**
```rust
// Add to lib.rs
thread_local! {
    static USER_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static WALLET_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static AGENT_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static CRYPTO_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

#[update]
fn set_user_canister_id(canister_id: String) -> Result<(), String> { ... }

#[update]
fn set_wallet_canister_id(canister_id: String) -> Result<(), String> { ... }

#[update]
fn set_agent_canister_id(canister_id: String) -> Result<(), String> { ... }

#[update]
fn set_crypto_canister_id(canister_id: String) -> Result<(), String> { ... }
```

### 3.2 Update Web Frontend (Satellite)
**Files:** `src/lib/services/`

**Update service files:**
- `src/lib/services/userService.ts` → call user_canister
- `src/lib/services/walletService.ts` (NEW) → call wallet_canister
- `src/lib/services/agentService.ts` → call agent_canister
- `src/lib/services/cryptoService.ts` (NEW) → call crypto_canister

**Update components:**
- All components calling business_logic_canister need to be updated
- Use new domain-specific services

---

## Phase 4: Update Tests

### 4.1 Create New Integration Tests

**user_canister tests:**
```
canisters/user_canister/tests/integration/
├── registration_tests.rs
├── authentication_tests.rs
├── profile_tests.rs
└── linking_tests.rs
```

**wallet_canister tests:**
```
canisters/wallet_canister/tests/integration/
├── transfer_tests.rs
├── balance_tests.rs
├── fraud_detection_tests.rs
└── transaction_history_tests.rs
```

**agent_canister tests:**
```
canisters/agent_canister/tests/integration/
├── deposit_tests.rs
├── withdrawal_tests.rs
├── commission_tests.rs
└── settlement_tests.rs
```

**crypto_canister tests:**
```
canisters/crypto_canister/tests/integration/
├── buy_sell_tests.rs
├── transfer_tests.rs
├── swap_tests.rs
└── escrow_tests.rs
```

### 4.2 Update Existing Tests

**USSD canister tests:**
- Update all integration tests to use new canister IDs
- Update test environment setup to deploy 4 domain canisters
- Update `tests/integration/mod.rs` with new setup

**Business logic canister tests:**
- Archive old tests (for reference)
- Create migration guide for test patterns

---

## Phase 5: Update dfx.json

### 5.1 Remove Old Canisters
```json
{
  "canisters": {
    // REMOVE these:
    // "business_logic_canister": { ... },
    // "exchange_canister": { ... },
    // "deposit_canister": { ... },
    // "withdrawal_canister": { ... },
    
    // ADD these:
    "user_canister": {
      "type": "rust",
      "package": "user_canister",
      "candid": "canisters/user_canister/user_canister.did"
    },
    "wallet_canister": {
      "type": "rust",
      "package": "wallet_canister",
      "candid": "canisters/wallet_canister/wallet_canister.did"
    },
    "agent_canister": {
      "type": "rust",
      "package": "agent_canister",
      "candid": "canisters/agent_canister/agent_canister.did"
    },
    "crypto_canister": {
      "type": "rust",
      "package": "crypto_canister",
      "candid": "canisters/crypto_canister/crypto_canister.did"
    },
    
    // KEEP these unchanged:
    "data_canister": { ... },
    "ussd_canister": { ... },
    "satellite": { ... },
    // ... SNS and ledger canisters ...
  }
}
```

### 5.2 Update Cargo.toml
**File:** `Cargo.toml` (workspace root)

```toml
[workspace]
members = [
    "canisters/shared_types",
    "canisters/data_canister",
    "canisters/user_canister",      # NEW
    "canisters/wallet_canister",    # NEW
    "canisters/agent_canister",     # NEW
    "canisters/crypto_canister",    # NEW
    "canisters/ussd_canister",
    # Remove: business_logic_canister, exchange_canister, deposit_canister, withdrawal_canister
]
```

---

## Phase 6: Deployment & Configuration

### 6.1 Deployment Order
```bash
# 1. Deploy data_canister (if not already deployed)
dfx deploy data_canister

# 2. Deploy domain canisters
dfx deploy user_canister
dfx deploy wallet_canister
dfx deploy agent_canister
dfx deploy crypto_canister

# 3. Deploy presentation canisters
dfx deploy ussd_canister
dfx deploy satellite

# 4. Get canister IDs
USER_ID=$(dfx canister id user_canister)
WALLET_ID=$(dfx canister id wallet_canister)
AGENT_ID=$(dfx canister id agent_canister)
CRYPTO_ID=$(dfx canister id crypto_canister)
DATA_ID=$(dfx canister id data_canister)
USSD_ID=$(dfx canister id ussd_canister)
```

### 6.2 Configuration Setup
```bash
# Configure domain canisters
dfx canister call user_canister set_data_canister_id "(\"$DATA_ID\")"

dfx canister call wallet_canister set_data_canister_id "(\"$DATA_ID\")"
dfx canister call wallet_canister set_user_canister_id "(\"$USER_ID\")"

dfx canister call agent_canister set_data_canister_id "(\"$DATA_ID\")"
dfx canister call agent_canister set_user_canister_id "(\"$USER_ID\")"

dfx canister call crypto_canister set_data_canister_id "(\"$DATA_ID\")"
dfx canister call crypto_canister set_user_canister_id "(\"$USER_ID\")"
dfx canister call crypto_canister set_wallet_canister_id "(\"$WALLET_ID\")"

# Configure USSD canister
dfx canister call ussd_canister set_user_canister_id "(\"$USER_ID\")"
dfx canister call ussd_canister set_wallet_canister_id "(\"$WALLET_ID\")"
dfx canister call ussd_canister set_agent_canister_id "(\"$AGENT_ID\")"
dfx canister call ussd_canister set_crypto_canister_id "(\"$CRYPTO_ID\")"

# Authorize USSD and Web to call domain canisters
dfx canister call user_canister add_authorized_canister "(\"$USSD_ID\")"
dfx canister call wallet_canister add_authorized_canister "(\"$USSD_ID\")"
dfx canister call agent_canister add_authorized_canister "(\"$USSD_ID\")"
dfx canister call crypto_canister add_authorized_canister "(\"$USSD_ID\")"

# Enable test mode for development
dfx canister call user_canister enable_test_mode
dfx canister call wallet_canister enable_test_mode
dfx canister call agent_canister enable_test_mode
dfx canister call crypto_canister enable_test_mode
```

---

## Phase 7: Testing & Verification

### 7.1 Unit Tests
```bash
# Test each canister independently
cd canisters/user_canister && cargo test
cd canisters/wallet_canister && cargo test
cd canisters/agent_canister && cargo test
cd canisters/crypto_canister && cargo test
```

### 7.2 Integration Tests
```bash
# Test USSD flows end-to-end
cd canisters/ussd_canister
cargo test --test lib -- --test-threads=1

# Expected: All 312 tests should pass
```

### 7.3 Smoke Tests
```bash
# Test critical user flows
1. Register new user (USSD)
2. Deposit cash via agent
3. Send money P2P
4. Buy Bitcoin
5. Swap BTC to USDC
6. Withdraw cash via agent
7. Check all balances
```

### 7.4 Size Verification
```bash
# Check WASM sizes
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Expected:
# user_canister.wasm: ~400KB
# wallet_canister.wasm: ~600KB
# agent_canister.wasm: ~700KB
# crypto_canister.wasm: ~1.0M
```

---

## Timeline & Milestones

### Day 1: Setup & User Canister
- ✅ Create canister structures
- ✅ Implement user_canister
- ✅ Write user_canister tests
- ✅ Update shared_types

### Day 2: Wallet & Agent Canisters
- ✅ Implement wallet_canister
- ✅ Implement agent_canister (merge deposit + withdrawal)
- ✅ Write wallet & agent tests
- ✅ Create unified agent management

### Day 3: Crypto Canister
- ✅ Implement crypto_canister (merge exchange)
- ✅ Write crypto tests
- ✅ Update USSD canister clients
- ✅ Update Web frontend services

### Day 4: Integration & Deployment
- ✅ Run all integration tests
- ✅ Deploy to local testnet
- ✅ Configure all canisters
- ✅ Run smoke tests
- ✅ Document new architecture

---

## Rollback Plan

If critical issues arise:

1. **Keep old canisters deployed** (don't delete)
2. **Switch USSD/Web back** to old canister IDs
3. **Fix issues** in new canisters
4. **Re-deploy** when ready
5. **Gradual migration** - route some traffic to new, some to old

---

## Success Criteria

### ✅ Functional
- [ ] All 312 USSD tests pass
- [ ] All critical user flows work end-to-end
- [ ] No data loss during migration
- [ ] Inter-canister communication works

### ✅ Performance
- [ ] All canisters < 1.5M (75% of limit)
- [ ] Response times < 2 seconds
- [ ] No cycle exhaustion

### ✅ Code Quality
- [ ] All endpoints documented
- [ ] All tests passing
- [ ] No duplicate code
- [ ] Clear domain boundaries

---

## Post-Migration Tasks

1. **Archive old canisters** (keep for 30 days)
2. **Update documentation** (README, architecture diagrams)
3. **Monitor production** (cycles, errors, performance)
4. **Optimize** (identify bottlenecks, improve efficiency)
5. **Plan next features** (now have room to grow!)

---

## Key Files Reference

### Documentation
- `BUSINESS_LOGIC_ANALYSIS.md` - Endpoint breakdown
- `REVISED_ARCHITECTURE.md` - Architecture overview
- `CANISTER_SPLIT_PLAN.md` - Original 3-way split
- `CANISTER_MIGRATION_PLAN.md` - This file

### Code Locations
- Old: `canisters/business_logic_canister/src/`
- New: `canisters/{user,wallet,agent,crypto}_canister/src/`

### Test Locations
- USSD: `canisters/ussd_canister/tests/integration/`
- Domain: `canisters/{user,wallet,agent,crypto}_canister/tests/`

---

## Questions & Decisions

### Q: Why 4 canisters instead of 3?
**A:** Agent operations (deposits/withdrawals) are a distinct B2B business model with commission tracking and settlements. Mixing with P2P transfers would create a 1.2M canister.

### Q: Why absorb exchange/deposit/withdrawal canisters?
**A:** They're thin wrappers with no complex logic. Absorbing them reduces inter-canister calls, eliminates duplicate agent management code, and simplifies deployment.

### Q: What about data_canister?
**A:** Stays unchanged. It's pure storage with no business logic - perfect as is.

### Q: What about USSD canister size (1.7M)?
**A:** It's presentation logic with lots of translations. We'll monitor but it's expected to be large. If needed, we can split by language or feature later.

---

## Next Steps

**Ready to start? Begin with Phase 1.1: Create user_canister**

Good luck! 🚀
