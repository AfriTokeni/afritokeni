# USSD ↔ Business Logic Gap Analysis

## 🎯 Objective
Map all Business Logic Canister methods to USSD flows and identify missing integrations.

---

## ✅ Business Logic Methods Available

### User Management
- ✅ `register_user(RegisterUserRequest)` → User ID
- ✅ `user_exists(user_identifier)` → bool
- ✅ `verify_pin(user_identifier, pin)` → bool
- ✅ `change_pin(user_identifier, old_pin, new_pin)` → ()
- ✅ `link_phone_to_account(principal_id, phone_number, pin)` → ()

### Money Transfers
- ✅ `transfer_money(from, to, amount, currency, pin)` → Transaction
- ✅ `send_money_to_phone(from_phone, to_phone, amount, currency, pin)` → Transaction
- ✅ `withdraw_fiat(phone_number, amount, agent_id, pin)` → Transaction

### Crypto Operations
- ✅ `buy_crypto(user_identifier, fiat_amount, crypto_type, pin)` → Transaction
- ✅ `send_crypto(user_identifier, to_address, amount, crypto_type, pin)` → Transaction
- ✅ `get_crypto_value_estimate(crypto_amount, crypto_type, currency)` → u64
- ✅ `sell_crypto_to_agent(user_identifier, crypto_amount, crypto_type, agent_id, pin)` → Escrow

### Escrow
- ✅ `verify_escrow_code(code, agent_id, pin)` → Escrow
- ✅ `get_escrow_status(code)` → Escrow
- ✅ `cancel_escrow(code, user_id, pin)` → ()

### Balance Queries
- ✅ `get_balances(user_identifier)` → UserBalances (fiat + crypto)
- ✅ `check_fiat_balance(user_identifier, currency)` → u64
- ✅ `check_crypto_balance(user_identifier, crypto_type)` → u64

### Transaction History
- ✅ `get_transaction_history(user_identifier, limit)` → Vec<Transaction>

---

## 📋 USSD Flows Currently Implemented

### Local Currency (`flows/local_currency/`)
- ✅ `send_money.rs` - Send fiat to another user
- ✅ `deposit.rs` - Deposit fiat via agent
- ✅ `withdraw.rs` - Withdraw fiat via agent

### Bitcoin (`flows/bitcoin/`)
- ✅ `buy.rs` - Buy Bitcoin
- ✅ `send.rs` - Send Bitcoin
- ✅ `sell.rs` - Sell Bitcoin to agent
- ✅ `check_balance.rs` - Check Bitcoin balance

### USDC (`flows/usdc/`)
- ✅ `buy.rs` - Buy USDC
- ✅ `send.rs` - Send USDC
- ✅ `sell.rs` - Sell USDC to agent
- ✅ `check_balance.rs` - Check USDC balance

### DAO (`flows/dao/`)
- ✅ `check_tokens.rs` - Check DAO tokens
- ✅ `vote.rs` - Vote on proposals
- ✅ `create_proposal.rs` - Create new proposal

### Common (`flows/common/`)
- ✅ `balance.rs` - Check all balances
- ✅ `history.rs` - Transaction history
- ✅ `language.rs` - Change language
- ✅ `pin.rs` - Change PIN
- ✅ `help.rs` - Help menu

---

## ❌ MISSING INTEGRATIONS (USSD → Business Logic)

### 1. **Deposit Flow** ⚠️ CRITICAL
**File:** `flows/local_currency/deposit.rs`

**Current Status:** Stub implementation
```rust
// flows/local_currency/deposit.rs (877 bytes)
// Likely just placeholder
```

**Business Logic Methods Available:**
- ❌ No `create_deposit_request` in Business Logic!
- ✅ Commission system has `deposit_canister::create_deposit_request`

**Action Needed:**
- [ ] Add `create_deposit_request` to Business Logic Canister
- [ ] Wire USSD deposit flow to call it
- [ ] Show commission/fees to user

---

### 2. **Withdraw Flow** ⚠️ CRITICAL
**File:** `flows/local_currency/withdraw.rs`

**Current Status:** Uses `withdraw_fiat` but may not show commissions
```rust
// withdraw_fiat(phone_number, amount, agent_id, pin)
```

**Business Logic Methods Available:**
- ✅ `withdraw_fiat` exists
- ✅ Commission system has `withdrawal_canister::create_withdrawal_request`

**Action Needed:**
- [ ] Verify `withdraw_fiat` calls withdrawal commission canister
- [ ] Show withdrawal fees to user (0.5% platform + 10% agent)
- [ ] Display net amount user will receive

---

### 3. **Crypto Exchange (BTC ↔ USDC)** ⚠️ HIGH PRIORITY
**Missing Flow:** Swap between cryptocurrencies

**Business Logic Methods Available:**
- ❌ No `swap_crypto` method!
- ✅ Commission system has `exchange_canister::swap_tokens`

**Action Needed:**
- [ ] Add `swap_crypto` to Business Logic Canister
- [ ] Create USSD flow for crypto swaps
- [ ] Show 0.5% spread to user

---

### 4. **Agent Operations** 📱 MEDIUM PRIORITY
**Missing Flows:**
- Agent deposit confirmation
- Agent withdrawal confirmation
- Agent earnings check

**Business Logic Methods Available:**
- ❌ No agent-specific methods in Business Logic!
- ✅ Commission canisters have agent methods:
  - `confirm_deposit(deposit_code, agent_principal)`
  - `confirm_withdrawal(withdrawal_code, agent_principal)`
  - `get_agent_balance(agent_principal)`
  - `get_agent_earnings(agent_principal)`

**Action Needed:**
- [ ] Add agent methods to Business Logic Canister
- [ ] Create USSD agent menu
- [ ] Agent can confirm deposits/withdrawals
- [ ] Agent can check earnings

---

### 5. **DAO Operations** 🏛️ LOW PRIORITY
**Current Status:** Flows exist but may not be wired

**Business Logic Methods Available:**
- ❌ No DAO methods visible in Business Logic!
- ❓ Are DAO operations in a separate canister?

**Action Needed:**
- [ ] Verify DAO canister integration
- [ ] Wire USSD DAO flows to DAO canister
- [ ] Or remove DAO flows if not ready

---

### 6. **Transaction History** 📜 MEDIUM PRIORITY
**File:** `flows/common/history.rs`

**Business Logic Methods Available:**
- ✅ `get_transaction_history(user_identifier, limit)`

**Action Needed:**
- [ ] Wire USSD history flow to Business Logic
- [ ] Format transactions for USSD display (limited screen space)
- [ ] Paginate results (show last 5, option for more)

---

### 7. **Balance Checks** 💰 HIGH PRIORITY
**Files:**
- `flows/common/balance.rs` - All balances
- `flows/bitcoin/check_balance.rs` - BTC only
- `flows/usdc/check_balance.rs` - USDC only

**Business Logic Methods Available:**
- ✅ `get_balances(user_identifier)` - All balances
- ✅ `check_fiat_balance(user_identifier, currency)`
- ✅ `check_crypto_balance(user_identifier, crypto_type)`

**Action Needed:**
- [ ] Wire all balance flows to Business Logic
- [ ] Format balances nicely for USSD
- [ ] Show exchange rates (e.g., "0.0001 BTC ≈ 50,000 UGX")

---

### 8. **PIN Change** 🔐 MEDIUM PRIORITY
**File:** `flows/common/pin.rs`

**Business Logic Methods Available:**
- ✅ `change_pin(user_identifier, old_pin, new_pin)`

**Action Needed:**
- [ ] Wire USSD PIN change flow to Business Logic
- [ ] Validate old PIN first
- [ ] Confirm new PIN (enter twice)

---

### 9. **Language Selection** 🌍 LOW PRIORITY
**File:** `flows/common/language.rs`

**Business Logic Methods Available:**
- ❌ No language preference storage in Business Logic!
- ❓ Is language stored in Data Canister?

**Action Needed:**
- [ ] Add language preference to User model in Data Canister
- [ ] Add `set_language(user_identifier, language)` to Business Logic
- [ ] Wire USSD language flow

---

### 10. **Help Menu** ℹ️ LOW PRIORITY
**File:** `flows/common/help.rs`

**Business Logic Methods Available:**
- N/A (static content)

**Action Needed:**
- [ ] Verify help content is translated
- [ ] Add contact information
- [ ] Add FAQ

---

## 🔧 Required Business Logic Additions

### Critical (Must Have)
1. **`create_deposit_request(user_phone, agent_id, amount)`**
   - Calls `deposit_canister::create_deposit_request`
   - Returns deposit code and commission amount
   - User shows code to agent

2. **`confirm_deposit(deposit_code, agent_phone, pin)`**
   - Calls `deposit_canister::confirm_deposit`
   - Updates user balance
   - Updates agent commission

3. **`confirm_withdrawal(withdrawal_code, agent_phone, pin)`**
   - Calls `withdrawal_canister::confirm_withdrawal`
   - Updates user balance
   - Updates agent earnings

4. **`swap_crypto(user_identifier, from_crypto, to_crypto, amount, pin)`**
   - Calls `exchange_canister::swap_tokens`
   - Shows 0.5% spread
   - Updates crypto balances

### High Priority (Should Have)
5. **`get_agent_balance(agent_phone)`**
   - Returns agent commission balance
   - Calls `deposit_canister::get_agent_balance`

6. **`get_agent_earnings(agent_phone)`**
   - Returns agent withdrawal earnings
   - Calls `withdrawal_canister::get_agent_earnings`

7. **`set_user_language(user_identifier, language)`**
   - Stores language preference
   - Calls `data_canister::update_user`

### Medium Priority (Nice to Have)
8. **`get_deposit_status(deposit_code)`**
   - Check if deposit is confirmed
   - For user to track deposit

9. **`get_withdrawal_status(withdrawal_code)`**
   - Check if withdrawal is confirmed
   - For user to track withdrawal

---

## 📊 Integration Status Summary

| Flow Category | Total Flows | Wired to Business Logic | Missing | Priority |
|---------------|-------------|-------------------------|---------|----------|
| User Management | 5 | ✅ 5 | 0 | ✅ Complete |
| Money Transfers | 3 | ✅ 2 | 1 (deposit) | ⚠️ Critical |
| Crypto Ops | 6 | ✅ 5 | 1 (swap) | ⚠️ High |
| Balance Checks | 3 | ❓ Unknown | TBD | ⚠️ High |
| Agent Ops | 0 | ❌ 0 | 3 | 📱 Medium |
| DAO | 3 | ❓ Unknown | TBD | 🏛️ Low |
| Common | 4 | ❓ Unknown | TBD | 📜 Medium |

---

## 🚀 Implementation Priority

### Phase 1: Critical Gaps (Week 1)
1. Add `create_deposit_request` to Business Logic
2. Add `confirm_deposit` to Business Logic
3. Add `confirm_withdrawal` to Business Logic
4. Wire USSD deposit flow
5. Wire USSD withdrawal flow to show fees

### Phase 2: High Priority (Week 2)
6. Add `swap_crypto` to Business Logic
7. Create USSD crypto swap flow
8. Wire all balance check flows
9. Add exchange rate display

### Phase 3: Agent Features (Week 3)
10. Add agent methods to Business Logic
11. Create USSD agent menu
12. Agent deposit/withdrawal confirmation
13. Agent earnings check

### Phase 4: Polish (Week 4)
14. Wire transaction history
15. Wire PIN change
16. Add language preference storage
17. Test all flows end-to-end

---

## ✅ Next Steps

1. **Review this analysis** - Confirm priorities
2. **Identify missing methods** - Which Business Logic methods need to be added?
3. **Start Phase 1** - Critical deposit/withdrawal integration
4. **Test each integration** - Ensure USSD → Business Logic → Commission Canisters works

**Ready for your confirmation and priorities!** 🎯
