# Commission Integration Implementation Plan

## ✅ Option 1 Selected: Multi-Canister Integration

### Current Configuration (from revenue_config.toml)

**Company Wallet:**
```
Principal: ctfzw-zjxmq-in44p-737ub-a73mu-uiuhb-rkehx-42rpn-ukhaf-7yzor-aae
```

**Commission Rates:**
- **Deposit:** 0.5% platform fee + 10% agent commission
- **Withdrawal:** 0.5% platform fee + 10% agent commission  
- **Exchange:** 0.5% spread

### Architecture to Implement

```
┌──────────────────────────────────────────────────────────────┐
│              BUSINESS LOGIC CANISTER                         │
│  • Orchestrates all operations                              │
│  • Calls specialized canisters for revenue collection        │
└──────────────────────────────────────────────────────────────┘
        ↓                    ↓                    ↓
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ DEPOSIT         │  │ WITHDRAWAL      │  │ EXCHANGE        │
│ CANISTER        │  │ CANISTER        │  │ CANISTER        │
│                 │  │                 │  │                 │
│ create_deposit  │  │ create_withdraw │  │ swap_tokens     │
│ confirm_deposit │  │ confirm_withdraw│  │                 │
│ get_agent_bal   │  │ get_agent_earn  │  │                 │
└─────────────────┘  └─────────────────┘  └─────────────────┘
        ↓                    ↓                    ↓
┌──────────────────────────────────────────────────────────────┐
│                      DATA CANISTER                           │
│  • User balances (fiat & crypto)                            │
│  • Transaction history                                       │
└──────────────────────────────────────────────────────────────┘
```

### Implementation Steps

#### Phase 1: Add Canister Client Functions ✅
**File:** `business_logic_canister/src/services/commission_client.rs` (NEW)

**Functions to Add:**
```rust
// Deposit Canister Calls
pub async fn create_deposit_request(
    user_principal: Principal,
    agent_principal: Principal,
    amount_ugx: u64,
) -> Result<DepositTransaction, String>

pub async fn confirm_deposit(
    deposit_code: String,
    agent_principal: Principal,
) -> Result<DepositTransaction, String>

pub async fn get_agent_deposit_balance(
    agent: Principal
) -> Result<Option<AgentBalance>, String>

// Withdrawal Canister Calls
pub async fn create_withdrawal_request(
    user_principal: Principal,
    agent_principal: Principal,
    amount_ugx: u64,
) -> Result<WithdrawalTransaction, String>

pub async fn confirm_withdrawal(
    withdrawal_code: String,
    agent_principal: Principal,
) -> Result<WithdrawalTransaction, String>

pub async fn get_agent_withdrawal_earnings(
    agent: Principal
) -> Result<Option<AgentEarnings>, String>

// Exchange Canister Calls
pub async fn swap_crypto(
    from_token: CryptoToken,
    to_token: CryptoToken,
    amount: u64,
    user_principal: Principal,
) -> Result<SwapResult, String>
```

#### Phase 2: Update Business Logic Functions ✅
**Files to Modify:**
- `business_logic_canister/src/lib.rs` - Add new endpoints
- `business_logic_canister/src/services/crypto_operations.rs` - Update buy/sell functions

**New Endpoints:**
```rust
#[update]
async fn deposit_cash_with_agent(
    user_identifier: String,
    agent_id: String,
    amount_ugx: u64,
    pin: String,
) -> Result<DepositTransaction, String>

#[update]
async fn confirm_cash_deposit(
    deposit_code: String,
    agent_id: String,
) -> Result<DepositTransaction, String>

#[update]
async fn withdraw_cash_via_agent(
    user_identifier: String,
    agent_id: String,
    amount_ugx: u64,
    pin: String,
) -> Result<WithdrawalTransaction, String>

#[update]
async fn confirm_cash_withdrawal(
    withdrawal_code: String,
    agent_id: String,
) -> Result<WithdrawalTransaction, String>
```

#### Phase 3: Create Multi-Canister Test Environment ✅
**File:** `business_logic_canister/tests/integration/mod.rs`

**Add to TestEnv:**
```rust
pub struct TestEnv {
    pub pic: PocketIc,
    pub data_canister_id: Principal,
    pub business_canister_id: Principal,
    pub deposit_canister_id: Principal,      // NEW
    pub withdrawal_canister_id: Principal,   // NEW
    pub exchange_canister_id: Principal,     // NEW
}

impl TestEnv {
    pub fn new() -> Self {
        let pic = PocketIc::new();
        
        // Deploy all canisters
        let data_canister_id = deploy_data_canister(&pic);
        let deposit_canister_id = deploy_deposit_canister(&pic);
        let withdrawal_canister_id = deploy_withdrawal_canister(&pic);
        let exchange_canister_id = deploy_exchange_canister(&pic);
        let business_canister_id = deploy_business_canister(
            &pic,
            data_canister_id,
            deposit_canister_id,
            withdrawal_canister_id,
            exchange_canister_id,
        );
        
        Self {
            pic,
            data_canister_id,
            business_canister_id,
            deposit_canister_id,
            withdrawal_canister_id,
            exchange_canister_id,
        }
    }
    
    // Helper methods for commission testing
    pub fn deposit_cash(...) -> Result<DepositTransaction, String>
    pub fn confirm_deposit(...) -> Result<DepositTransaction, String>
    pub fn get_agent_commission_owed(...) -> Result<u64, String>
    pub fn withdraw_cash(...) -> Result<WithdrawalTransaction, String>
    pub fn confirm_withdrawal(...) -> Result<WithdrawalTransaction, String>
    pub fn get_company_wallet_balance(...) -> Result<u64, String>
}
```

#### Phase 4: Implement Commission Tests ✅

**Test Files to Create:**

1. **`deposit_commission_tests.rs`** (7 tests)
   - ✅ Create deposit calculates 0.5% commission
   - ✅ Confirm deposit updates agent balance
   - ✅ Multiple deposits accumulate commission
   - ✅ Agent commission owed tracked correctly
   - ✅ Platform fee separate from agent commission
   - ✅ Deposit limits enforced
   - ✅ Invalid deposit code rejected

2. **`withdrawal_commission_tests.rs`** (8 tests)
   - ✅ Platform fee calculated (0.5%)
   - ✅ Agent fee calculated (10%)
   - ✅ Agent earns correct amount
   - ✅ Multiple withdrawals accumulate
   - ✅ Withdrawal limits enforced
   - ✅ Invalid withdrawal code rejected
   - ✅ Commission deposited to company wallet
   - ✅ Agent earnings tracked

3. **`exchange_commission_tests.rs`** (6 tests)
   - ✅ ckBTC → ckUSDC spread (0.5%)
   - ✅ ckUSDC → ckBTC spread (0.5%)
   - ✅ User receives correct amount after spread
   - ✅ Spread deposited to company wallet
   - ✅ Large swap spread calculation
   - ✅ Small swap spread calculation

4. **`revenue_tracking_tests.rs`** (5 tests)
   - ✅ Total revenue across all canisters
   - ✅ Revenue by canister type
   - ✅ Agent commission vs platform revenue
   - ✅ Company wallet balance verification
   - ✅ Revenue audit trail

### Test Scenarios with Examples

#### Deposit Commission Test
```rust
#[test]
fn test_deposit_commission_calculation() {
    let env = TestEnv::new();
    
    // User deposits 100,000 UGX with agent
    let deposit = env.deposit_cash(
        user_id,
        agent_id,
        100_000,
        "1234"
    ).unwrap();
    
    // Platform fee: 0.5% = 500 UGX
    assert_eq!(deposit.platform_fee_ugx, 500);
    
    // Agent commission: 10% = 10,000 UGX
    assert_eq!(deposit.agent_commission_ugx, 10_000);
    
    // User receives: 100,000 - 500 = 99,500 UGX
    let balance = env.check_fiat_balance(&user_id, "UGX").unwrap();
    assert_eq!(balance, 99_500);
}
```

#### Withdrawal Commission Test
```rust
#[test]
fn test_withdrawal_commission_split() {
    let env = TestEnv::new();
    
    // User withdraws 100,000 UGX via agent
    let withdrawal = env.withdraw_cash(
        user_id,
        agent_id,
        100_000,
        "1234"
    ).unwrap();
    
    // Platform fee: 0.5% = 500 UGX
    assert_eq!(withdrawal.platform_fee_ugx, 500);
    
    // Agent fee: 10% = 10,000 UGX
    assert_eq!(withdrawal.agent_fee_ugx, 10_000);
    
    // Agent keeps: 10,000 UGX
    // Platform gets: 500 UGX
    // User pays: 100,000 + 500 + 10,000 = 110,500 UGX total
}
```

#### Exchange Spread Test
```rust
#[test]
fn test_exchange_spread_collection() {
    let env = TestEnv::new();
    
    // User swaps 1,000,000 satoshis (0.01 BTC)
    let swap = env.swap_crypto(
        CryptoToken::CkBTC,
        CryptoToken::CkUSDC,
        1_000_000,
        user_principal
    ).unwrap();
    
    // Spread: 0.5% = 5,000 satoshis
    assert_eq!(swap.spread_amount, 5_000);
    
    // User receives: 995,000 satoshis worth of ckUSDC
    assert_eq!(swap.output_amount, 995_000);
    
    // Company wallet receives: 5,000 satoshis
    let company_balance = env.get_company_wallet_balance(
        CryptoToken::CkBTC
    ).unwrap();
    assert_eq!(company_balance, 5_000);
}
```

### Expected Test Results

**After Implementation:**
```
✅ Business Logic Tests:        80 passing
✅ Deposit Commission Tests:     7 passing
✅ Withdrawal Commission Tests:  8 passing
✅ Exchange Commission Tests:    6 passing
✅ Revenue Tracking Tests:       5 passing
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   TOTAL:                      106 passing (100%)
```

### Build & Deploy Commands

**Build All Canisters:**
```bash
cargo build --release --target wasm32-unknown-unknown --package data_canister
cargo build --release --target wasm32-unknown-unknown --package deposit_canister
cargo build --release --target wasm32-unknown-unknown --package withdrawal_canister
cargo build --release --target wasm32-unknown-unknown --package exchange_canister
cargo build --release --target wasm32-unknown-unknown --package business_logic_canister
```

**Run Tests:**
```bash
cargo test --package business_logic_canister --test '*' -- --test-threads=1
```

### Configuration Required

**Business Logic Canister Init Args:**
```rust
pub struct BusinessLogicInitArgs {
    pub data_canister_id: Principal,
    pub deposit_canister_id: Principal,
    pub withdrawal_canister_id: Principal,
    pub exchange_canister_id: Principal,
}
```

**Revenue Config (already exists):**
- ✅ Company wallet principal
- ✅ Commission rates
- ✅ Limits

### Success Criteria

**Phase 1 Complete:**
- [ ] Commission client functions added
- [ ] Business logic endpoints created
- [ ] All canisters compile

**Phase 2 Complete:**
- [ ] Multi-canister test environment working
- [ ] All 26 commission tests passing
- [ ] Company wallet deposits verified

**Phase 3 Complete:**
- [ ] End-to-end revenue flow tested
- [ ] Agent settlements validated
- [ ] Documentation updated

### Timeline

**Estimated:** 2-3 hours for full implementation
- Phase 1: 30 minutes (client functions)
- Phase 2: 45 minutes (test environment)
- Phase 3: 60 minutes (tests)
- Phase 4: 30 minutes (verification & docs)

### Next Immediate Steps

1. ✅ Create `commission_client.rs`
2. ✅ Add canister IDs to business logic config
3. ✅ Update TestEnv for multi-canister deployment
4. ✅ Implement first deposit commission test
5. ✅ Verify commission calculation
6. ✅ Continue with remaining tests

---

**Status:** 🚀 READY TO IMPLEMENT  
**Approach:** Multi-canister integration (Option 1)  
**Expected Outcome:** 106/106 tests passing with full commission validation
