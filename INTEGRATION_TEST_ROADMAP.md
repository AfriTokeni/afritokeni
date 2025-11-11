# AfriTokeni Integration Test Roadmap

## Current Status: 53/53 Tests Passing ✅

---

## Architecture Analysis: Escrow System

### Current Implementation
**Escrow is SPLIT between two layers:**

1. **Frontend (Juno)**: `src/lib/services/escrowService.ts`
   - Stores escrow metadata in Juno DB (`escrow_transactions` collection)
   - Generates 6-digit codes (BTC-123456, USD-123456)
   - Tracks status (pending, funded, completed, refunded, expired)
   - Handles 24-hour expiration logic

2. **Business Logic Canister**: `sell_crypto_to_agent()` function
   - Deducts crypto from user balance
   - Creates transaction with "Pending" status
   - Generates escrow code
   - Validates agent exists

### ⚠️ CRITICAL ISSUE: Escrow is NOT Atomic!

**Current Flow (BROKEN):**
```
1. Business Logic: Deduct crypto from user ❌
2. Business Logic: Create pending transaction ❌
3. Frontend: Store escrow in Juno ❌
4. [If step 3 fails, user loses crypto!]
```

**The Problem:**
- Crypto is deducted BEFORE escrow is confirmed
- If Juno write fails, crypto is lost
- No rollback mechanism
- Same atomicity bug we just fixed!

### 🎯 RECOMMENDATION: Keep Escrow in Business Logic Canister

**Why:**
1. **Atomicity**: All state changes in one canister = atomic operations
2. **Security**: Crypto balances and escrow state stay together
3. **Simplicity**: No cross-system coordination needed
4. **ICP Native**: No need for external escrow canister

**ICP Does NOT Provide Pre-Made Escrow:**
- ICP provides ICRC-1 ledgers (ckBTC, ckUSDC)
- But escrow logic must be custom-built
- Best practice: Keep escrow in same canister as balance management

**Proposed Architecture:**
```
Business Logic Canister:
├── User Balances (fiat + crypto)
├── Escrow State (codes, expiration, status)
├── Transaction Records
└── Escrow Operations:
    ├── create_escrow() - Lock crypto, generate code
    ├── verify_escrow_code() - Release to agent
    ├── refund_expired_escrow() - Auto-refund after 24h
    └── cancel_escrow() - Manual cancel
```

---

## Test Roadmap

### Phase 1: Escrow System Tests (HIGHEST PRIORITY) 🔴

**Why First:** Critical financial security - users can lose money

**Tests to Create:**
1. ✅ Create escrow with valid inputs
2. ✅ Generate unique 6-digit codes (BTC-XXXXXX, USD-XXXXXX)
3. ✅ Verify crypto is locked (deducted from user balance)
4. ✅ Agent can verify code and receive crypto
5. ✅ Invalid code rejection
6. ✅ Wrong agent cannot claim escrow
7. ✅ Expired escrow auto-refunds to user (24 hours)
8. ✅ Cannot double-claim escrow
9. ✅ Escrow atomicity (rollback on failure)
10. ✅ Multiple concurrent escrows

**Estimated:** 10 tests, ~2 hours

**Files to Create:**
- `canisters/business_logic_canister/tests/integration/escrow_tests.rs`

**Helper Functions Needed:**
```rust
// In mod.rs
impl TestEnv {
    pub fn create_escrow(&self, user_id, agent_id, amount, crypto_type, pin) -> Result<String, String>
    pub fn verify_escrow_code(&self, code, agent_id) -> Result<TransactionResult, String>
    pub fn get_escrow_status(&self, code) -> Result<EscrowStatus, String>
    pub fn cancel_escrow(&self, code, user_id, pin) -> Result<(), String>
}
```

---

### Phase 2: Exchange Rate Tests (HIGH PRIORITY) 🟠

**Why Second:** Wrong rates = financial losses for users/agents

**Tests to Create:**
1. ✅ Fiat → Crypto conversion (UGX → ckBTC)
2. ✅ Crypto → Fiat conversion (ckBTC → UGX)
3. ✅ Multi-currency support (all 7 mock currencies)
4. ✅ Rate consistency (buy then sell ≈ original amount)
5. ✅ Zero amount handling
6. ✅ Invalid currency rejection
7. ✅ Large amount calculations (overflow protection)
8. ✅ Decimal precision (no rounding errors)
9. ✅ Exchange rate caching (performance)

**Estimated:** 9 tests, ~1.5 hours

**Files to Create:**
- `canisters/business_logic_canister/tests/integration/exchange_rate_tests.rs`

**Helper Functions Needed:**
```rust
impl TestEnv {
    pub fn calculate_crypto_from_fiat(&self, fiat_amount, currency, crypto_type) -> Result<u64, String>
    pub fn calculate_fiat_from_crypto(&self, crypto_amount, crypto_type, currency) -> Result<u64, String>
    pub fn get_exchange_rate(&self, from_currency, to_currency) -> Result<f64, String>
}
```

---

### Phase 3: Fraud Detection Tests (HIGH PRIORITY) 🟠

**Why Third:** Platform vulnerable to abuse without proper limits

**Tests to Create:**
1. ✅ Rate limiting (max 10 requests/minute per user)
2. ✅ Large transaction alerts (>100K UGX)
3. ✅ Suspicious pattern detection
4. ✅ Multiple failed PIN attempts lockout
5. ✅ Rapid transaction blocking
6. ✅ Rate limit reset after cooldown
7. ✅ Different users have separate limits

**Estimated:** 7 tests, ~1 hour

**Files to Create:**
- `canisters/business_logic_canister/tests/integration/fraud_detection_tests.rs`

**Helper Functions Needed:**
```rust
impl TestEnv {
    pub fn check_rate_limit(&self, user_id) -> bool
    pub fn trigger_rate_limit(&self, user_id) // Helper to hit limit
    pub fn wait_for_rate_limit_reset(&self) // Simulate time passage
}
```

---

### Phase 4: DAO Governance Tests (MEDIUM PRIORITY) 🟡

**Why Fourth:** DAO is in production (ICP SNS) but needs integration tests

**Tests to Create:**
1. ✅ Create governance proposal
2. ✅ Vote on proposal (YES/NO/ABSTAIN)
3. ✅ Token locking during vote
4. ✅ Vote weight calculation (1 AFRI = 1 vote)
5. ✅ Proposal approval (51% threshold)
6. ✅ Quorum requirement (10% minimum)
7. ✅ Double-vote prevention
8. ✅ Token unlock after proposal ends
9. ✅ Proposal expiration (7 days)
10. ✅ Treasury spending approval

**Estimated:** 10 tests, ~2 hours

**Files to Create:**
- `canisters/business_logic_canister/tests/integration/dao_governance_tests.rs`

**Note:** Since you're using ICP SNS, these tests will interact with SNS canisters

---

### Phase 5: Agent Operations Tests (MEDIUM PRIORITY) 🟡

**Why Fifth:** Core business model, but basic tests already exist

**Additional Tests Needed:**
1. ✅ Agent verification (KYC check)
2. ✅ Agent rating system
3. ✅ Agent liquidity requirements
4. ✅ Agent commission calculation
5. ✅ Agent location-based fees (urban vs rural)
6. ✅ Agent availability status
7. ✅ Agent transaction history

**Estimated:** 7 tests, ~1 hour

**Files to Modify:**
- `canisters/business_logic_canister/tests/integration/deposit_withdrawal_tests.rs`

---

### Phase 6: Multi-Language Support Tests (LOW PRIORITY) 🟢

**Why Last:** UX issue, not financial security

**Tests to Create:**
1. ✅ Language switching (English, Luganda, Swahili)
2. ✅ Translated error messages
3. ✅ Currency formatting per locale
4. ✅ USSD menu translation

**Estimated:** 4 tests, ~30 minutes

**Files to Create:**
- `canisters/business_logic_canister/tests/integration/language_tests.rs`

---

## Summary

| Phase | Priority | Tests | Time | Status |
|-------|----------|-------|------|--------|
| **1. Escrow System** | 🔴 CRITICAL | 10 | 2h | ⏳ TODO |
| **2. Exchange Rates** | 🟠 HIGH | 9 | 1.5h | ⏳ TODO |
| **3. Fraud Detection** | 🟠 HIGH | 7 | 1h | ⏳ TODO |
| **4. DAO Governance** | 🟡 MEDIUM | 10 | 2h | ⏳ TODO |
| **5. Agent Operations** | 🟡 MEDIUM | 7 | 1h | ⏳ TODO |
| **6. Multi-Language** | 🟢 LOW | 4 | 0.5h | ⏳ TODO |
| **TOTAL** | | **47** | **8h** | |

**Current:** 53 tests passing
**Target:** 100 tests passing
**New Tests:** 47

---

## Implementation Order

### Week 1: Critical Financial Security
1. **Day 1-2:** Escrow System (10 tests)
   - Fix atomicity issue first
   - Move escrow logic to Business Logic Canister
   - Create comprehensive tests

2. **Day 3:** Exchange Rates (9 tests)
   - Verify calculation accuracy
   - Test all supported currencies

3. **Day 4:** Fraud Detection (7 tests)
   - Implement rate limiting
   - Test security boundaries

### Week 2: Governance & Operations
4. **Day 5-6:** DAO Governance (10 tests)
   - Integrate with ICP SNS
   - Test voting mechanisms

5. **Day 7:** Agent Operations (7 tests)
   - Extend existing tests
   - Add agent-specific validations

6. **Day 8:** Multi-Language (4 tests)
   - Quick UX validation

---

## Next Steps

1. **Immediate:** Start with Escrow System tests
2. **Fix Architecture:** Move escrow state to Business Logic Canister
3. **Ensure Atomicity:** All escrow operations must be atomic
4. **Run Tests:** Maintain 100% pass rate throughout

---

## Questions to Resolve

1. **DAO Integration:** Which ICP SNS functions do we call for governance?
2. **Escrow Migration:** Should we migrate existing Juno escrow data?
3. **Auto-Refund:** How to implement 24-hour timer in canister?
4. **Agent KYC:** Where is agent verification data stored?

---

**Ready to start with Escrow System tests?**
