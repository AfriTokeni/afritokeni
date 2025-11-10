# Integration Test Plan for Business Logic Canister

## Critical Financial Operations - MUST BE TESTED

### 1. User Registration & Authentication
- [x] Register user with phone
- [x] Register user with principal
- [ ] Register user with both phone and principal
- [x] Duplicate phone registration fails
- [x] Missing identifier fails
- [ ] Link phone to existing principal account
- [ ] PIN verification
- [ ] PIN lockout after 3 failed attempts
- [ ] Change PIN

### 2. Money Transfers (Fiat)
- [x] Full transfer flow (sender → receiver)
- [x] Insufficient balance fails
- [x] Wrong PIN fails
- [x] Transfer to nonexistent user fails
- [ ] Self-transfer fails
- [ ] Zero amount transfer fails
- [ ] Negative amount transfer fails
- [ ] Transfer with exact balance
- [ ] Multiple rapid transfers (rate limiting)
- [ ] Large transaction flagging
- [ ] Transaction history recording

### 3. Withdrawals (Cash Out)
- [x] Full withdrawal flow via agent
- [x] Insufficient balance fails
- [x] Wrong PIN fails
- [ ] Withdrawal to nonexistent agent fails
- [ ] Multiple transactions recorded
- [ ] Withdrawal limits enforcement

### 4. Crypto Operations
- [ ] Buy crypto with fiat (ckBTC)
- [ ] Buy crypto with fiat (ckUSDC)
- [ ] Send crypto to address
- [ ] Sell crypto to agent (escrow)
- [ ] Get crypto value estimate
- [ ] Check crypto balance
- [ ] Crypto balance updates correctly

### 5. Balance Integrity (CRITICAL!)
- [ ] Money conservation across transfers
- [ ] Balance consistency after multiple operations
- [ ] Cannot create money from thin air
- [ ] Cannot transfer more than balance
- [ ] Balance updates are atomic
- [ ] Concurrent transfer protection

### 6. Fraud Detection
- [ ] Large transaction detection
- [ ] Rate limiting works
- [ ] Suspicious activity flagging
- [ ] Account takeover detection
- [ ] Multiple failed PIN attempts

### 7. Transaction History
- [x] Empty history for new user
- [x] Transactions recorded correctly
- [ ] Pagination works
- [ ] Transaction types correct
- [ ] Timestamps correct

### 8. Edge Cases & Error Handling
- [ ] Invalid currency code
- [ ] Invalid phone format
- [ ] Invalid principal format
- [ ] Canister authorization
- [ ] Audit log recording

## Test Coverage Goals
- **Current:** 11 tests (basic flows only)
- **Target:** 50+ tests (comprehensive coverage)
- **Priority:** Financial integrity tests FIRST

## Missing Functionality to Implement
1. Self-transfer prevention
2. Crypto operations integration with ledgers
3. Rate limiting enforcement
4. Transaction limits
5. Proper error messages for all edge cases
