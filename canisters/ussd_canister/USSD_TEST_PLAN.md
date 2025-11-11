# USSD Canister Integration Test Plan

## Objective
Create comprehensive end-to-end integration tests covering ALL possible USSD user flows to ensure:
- No regressions in releases
- All user actions work correctly
- Complete flow coverage from dial-in to completion

## Test Strategy
- **Real canister interactions** (no mocks)
- **Shared TestEnv** for fast execution (~6 seconds for all tests)
- **PocketIC framework** for realistic IC environment
- **Multi-step flow validation** for each user journey

---

## 1. Registration Flows ✅ (Already Created)

### 1.1 Happy Path Registration
- [x] Complete registration with valid inputs
- [x] PIN validation (4 digits)
- [x] Currency auto-detection from phone number
- [x] Manual currency selection
- [x] Success message with user name

### 1.2 Error Cases
- [x] Invalid PIN format (too short, too long, non-numeric)
- [x] Empty first/last name rejection
- [x] Duplicate phone number registration
- [x] Invalid currency selection

### 1.3 Edge Cases
- [ ] Registration with all supported currencies (39 African currencies)
- [ ] Special characters in names
- [ ] Very long names (boundary testing)
- [ ] Session timeout during registration

---

## 2. Send Money Flows ✅ (Already Created)

### 2.1 Happy Path
- [x] Complete send money flow (sender → receiver)
- [x] Balance deduction from sender
- [x] Balance addition to receiver
- [x] Transaction recorded

### 2.2 Error Cases
- [x] Insufficient balance
- [x] Invalid recipient phone number
- [x] Wrong PIN
- [x] Zero amount
- [x] Negative amount

### 2.3 Edge Cases
- [ ] Send to self
- [ ] Send maximum allowed amount
- [ ] Send minimum allowed amount
- [ ] Multiple rapid transfers (rate limiting)
- [ ] Recipient not registered
- [ ] Cross-currency transfers

---

## 3. Bitcoin Operations ✅ (Partially Created)

### 3.1 Check Bitcoin Balance
- [x] Display balance with zero BTC
- [x] Display balance with BTC
- [x] Balance formatting

### 3.2 Check Bitcoin Rate
- [x] Display current BTC rate
- [x] Rate in user's local currency

### 3.3 Buy Bitcoin
- [x] Navigation to buy flow
- [x] Amount input validation
- [ ] Successful purchase
- [x] Insufficient fiat balance
- [ ] Minimum/maximum amount limits
- [ ] Rate calculation accuracy
- [ ] Balance update after purchase

### 3.4 Send Bitcoin
- [x] Navigation to send flow
- [ ] Valid Bitcoin address
- [ ] Invalid Bitcoin address formats
- [ ] Successful send
- [x] Insufficient BTC balance
- [ ] Transaction fee calculation
- [ ] Confirmation with recipient address

### 3.5 Sell Bitcoin
- [x] Navigation to sell flow
- [ ] Successful sell
- [ ] Insufficient BTC balance
- [ ] Rate calculation
- [ ] Fiat balance update

---

## 4. USDC Operations ✅ (Partially Created)

### 4.1 Check USDC Balance
- [x] Display balance with zero USDC
- [x] Display balance with USDC
- [x] Stablecoin precision (2 decimals)

### 4.2 Check USDC Rate
- [x] Display current USDC rate
- [x] Rate in user's local currency

### 4.3 Buy USDC
- [x] Navigation to buy flow
- [ ] Successful purchase
- [x] Insufficient fiat balance
- [ ] Minimum/maximum amount limits
- [ ] Stablecoin characteristics

### 4.4 Send USDC
- [x] Navigation to send flow
- [ ] Successful send
- [x] Insufficient USDC balance
- [ ] Address validation

### 4.5 Sell USDC
- [x] Navigation to sell flow
- [ ] Successful sell
- [ ] Rate calculation

---

## 5. Crypto Swap Flows ✅ (Partially Created)

### 5.1 BTC ↔ USDC Swaps
- [x] Menu navigation
- [x] Select from crypto (BTC/USDC)
- [x] Select to crypto (BTC/USDC)
- [x] Amount input
- [x] Spread display from exchange canister
- [x] Reject same-token swap (BTC → BTC)
- [x] Reject zero amount
- [x] Cancel at confirmation
- [ ] Successful swap execution
- [ ] Balance updates after swap
- [ ] Spread calculation verification

### 5.2 Edge Cases
- [ ] Minimum swap amounts
- [ ] Maximum swap amounts
- [ ] Insufficient balance for swap
- [ ] Rate changes during swap

---

## 6. Withdrawal Flows ✅ (Partially Created)

### 6.1 Happy Path
- [x] Navigation to withdraw
- [ ] Successful withdrawal
- [ ] Agent ID validation
- [ ] Balance deduction
- [ ] Transaction recording

### 6.2 Error Cases
- [x] Insufficient balance
- [x] Zero amount
- [x] Wrong PIN
- [x] Invalid agent ID
- [x] Missing agent ID

### 6.3 Edge Cases
- [x] Large amount withdrawal
- [ ] Withdrawal limits per day
- [ ] Multiple withdrawals
- [ ] Agent commission calculation

---

## 7. Balance Check Flows ✅ (Partially Created)

### 7.1 Fiat Balance
- [x] Single currency balance
- [x] Multiple currencies
- [x] Zero balance display
- [x] Balance formatting (thousands separators)
- [x] Balance precision

### 7.2 Crypto Balance
- [x] Bitcoin balance separate check
- [x] USDC balance separate check
- [x] Combined crypto balance
- [x] Zero crypto balance

### 7.3 Balance After Transactions
- [x] Balance update after send money
- [ ] Balance update after buy crypto
- [ ] Balance update after sell crypto
- [ ] Balance update after swap
- [ ] Balance update after withdrawal

---

## 8. DAO Operations (NEW - To Create)

### 8.1 View Proposals
- [ ] List active proposals
- [ ] Proposal details display
- [ ] Voting status

### 8.2 Vote on Proposals
- [ ] Cast YES vote
- [ ] Cast NO vote
- [ ] Vote confirmation
- [ ] Cannot vote twice
- [ ] Voting power calculation

### 8.3 Create Proposal (if allowed)
- [ ] Proposal creation flow
- [ ] Validation of proposal data
- [ ] Minimum stake requirement

---

## 9. Language Selection (NEW - To Create)

### 9.1 Language Menu
- [ ] Display available languages (English, Luganda, Swahili)
- [ ] Select language
- [ ] Persist language preference

### 9.2 Translation Verification
- [ ] All menus translated
- [ ] Error messages translated
- [ ] Success messages translated
- [ ] No hardcoded English strings

---

## 10. Main Menu Navigation (NEW - To Create)

### 10.1 Menu Structure
- [ ] Display all options
- [ ] Navigate to each submenu
- [ ] Return to main menu from submenus
- [ ] Invalid option handling

### 10.2 Menu Options
- [ ] 1. Send Money
- [ ] 2. Bitcoin
- [ ] 3. USDC
- [ ] 4. Swap Crypto
- [ ] 5. DAO
- [ ] 6. Check Balance
- [ ] 7. Withdraw
- [ ] 8. Language
- [ ] 0. Exit

---

## 11. Session Management (NEW - To Create)

### 11.1 Session Lifecycle
- [ ] New session creation
- [ ] Session continuation
- [ ] Session timeout
- [ ] Session cleanup

### 11.2 Multi-Step Flows
- [ ] State preservation across steps
- [ ] Data validation at each step
- [ ] Cancel mid-flow
- [ ] Resume after interruption

---

## 12. Error Handling & Edge Cases (NEW - To Create)

### 12.1 Network/Canister Errors
- [ ] Business logic canister unavailable
- [ ] Data canister unavailable
- [ ] Exchange canister unavailable
- [ ] Timeout handling

### 12.2 Input Validation
- [ ] Special characters in input
- [ ] Very long input strings
- [ ] Empty input
- [ ] Invalid characters

### 12.3 Rate Limiting
- [ ] Too many requests per minute
- [ ] Rate limit message display
- [ ] Rate limit reset

---

## 13. Security Tests (NEW - To Create)

### 13.1 PIN Security
- [ ] PIN attempts limit (3 tries)
- [ ] Account lockout after failed attempts
- [ ] PIN change flow
- [ ] PIN reset flow

### 13.2 Authorization
- [ ] Cannot access other user's data
- [ ] Cannot perform actions without PIN
- [ ] Session hijacking prevention

---

## 14. Multi-Currency Support (NEW - To Create)

### 14.1 All 39 African Currencies
- [ ] KES (Kenya)
- [ ] UGX (Uganda)
- [ ] TZS (Tanzania)
- [ ] RWF (Rwanda)
- [ ] NGN (Nigeria)
- [ ] GHS (Ghana)
- [ ] ZAR (South Africa)
- [ ] ... (32 more)

### 14.2 Currency Operations
- [ ] Send money in each currency
- [ ] Buy crypto with each currency
- [ ] Withdraw in each currency
- [ ] Exchange rate accuracy

---

## 15. Performance & Load Tests (NEW - To Create)

### 15.1 Concurrent Users
- [ ] Multiple users simultaneously
- [ ] No data corruption
- [ ] No race conditions

### 15.2 Large Datasets
- [ ] User with many transactions
- [ ] User with high balances
- [ ] Transaction history pagination

---

## Test Execution Strategy

### Phase 1: Core Flows (Current)
- Registration ✅
- Send Money ✅
- Bitcoin Operations ✅
- USDC Operations ✅
- Balance Checks ✅
- Withdrawals ✅
- Crypto Swaps ✅

### Phase 2: Extended Flows (Next)
- DAO Operations
- Language Selection
- Main Menu Navigation
- Session Management

### Phase 3: Edge Cases & Security
- Error Handling
- Security Tests
- Multi-Currency
- Performance Tests

---

## Success Criteria

✅ **100% flow coverage** - Every user action has a test
✅ **Fast execution** - All tests run in <10 seconds
✅ **No mocks** - Real canister interactions only
✅ **Regression prevention** - Catch breaking changes before release
✅ **Clear failures** - Easy to identify what broke and why

---

## Current Status

- **Tests Created**: 61
- **Tests Passing**: ~5-10 (fixing authorization issues)
- **Execution Time**: ~6 seconds (40-60x faster than before!)
- **Coverage**: ~40% of planned flows

## Next Steps

1. Fix remaining authorization issues in current tests
2. Add missing test cases for existing flows (buy/sell/swap execution)
3. Create DAO operation tests
4. Create language selection tests
5. Create comprehensive error handling tests
6. Add security and performance tests
