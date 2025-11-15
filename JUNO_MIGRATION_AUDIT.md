# Juno Datastore Migration Audit Report

**Date:** 2025-11-15
**Issue:** Application failing with `juno.collections.error.not_found (Datastore - user_roles)`
**Root Cause:** Frontend code attempting to access obsolete Juno datastore collections that should have been migrated to domain canisters.

---

## Executive Summary

AfriTokeni has migrated from a monolithic architecture to a 4-domain canister system, but significant portions of the frontend still reference obsolete Juno datastore collections for business-critical data. This creates:

1. **Data inconsistency** between Juno and canister storage
2. **Security vulnerabilities** (bypassing canister access control)
3. **Authentication failures** (PIN verification not using Argon2 hashing)
4. **Runtime errors** (accessing non-existent Juno collections)

---

## Fixed Issues (Immediate)

### 1. roleGuard.ts - CRITICAL FIX ✅

**Problem:** Attempting to fetch user roles from non-existent `user_roles` Juno collection.

**Fix Applied:**
- Removed `getDoc({ collection: "user_roles" })` call
- Replaced with `userCanisterService.getUserByPrincipal(principalId)`
- Role now fetched from `user_canister` as `user_type` field (User/Agent/Admin)
- Updated security comment to reflect canister-based access control

**File:** `/src/lib/auth/roleGuard.ts`

**Impact:** Fixes the immediate error preventing app from loading.

---

## Identified Issues (Require Future Migration)

### 2. webHookServices.ts - DELETED ✅

**Status:** File completely removed (no longer needed).

**Obsolete Juno Collections Used:**
- ❌ `"users"` → Should use `user_canister.get_user_profile()`
- ❌ `"balances"` → Should use `wallet_canister.get_fiat_balance()`
- ❌ `"transactions"` → Should use `wallet_canister` (fiat) or `crypto_canister` (crypto)
- ❌ `"agents"` → Metadata acceptable, balances must use `agent_canister`
- ❌ `"deposit_requests"` → Should use `agent_canister.process_deposit()`

**Why Critical:**
- **1,555 lines** of code using Juno for business logic
- Used by USSD webhook for SMS-based banking
- Bypasses canister access control and audit logging
- PIN verification not using Argon2 hashing from `user_canister`
- Balances out of sync with canister state

**Migration Plan:**
1. Create new `ussdCanisterService.ts` that wraps domain canister calls
2. Replace all user operations with `userCanisterService`
3. Replace all balance operations with `walletCanisterService`
4. Replace all agent operations with `agentCanisterService`
5. Replace all crypto operations with `cryptoCanisterService`
6. Update USSD webhook to use domain canisters directly
7. Delete `webHookServices.ts` once migration complete

**File:** `/src/lib/services/webHookServices.ts`

---

### 3. juno/userService.ts - ADMIN DASHBOARD ⚠️

**Status:** Documented with deprecation warnings.

**Obsolete Collections:**
- ❌ `"users"` → Should use `user_canister`

**Usage:**
- Admin dashboard user management (`/admin` routes)
- User statistics and charts
- KYC status tracking

**Migration Plan:**
1. Update admin dashboard to call `userCanisterService`
2. Add admin-specific endpoints to `user_canister` if needed
3. Keep for backward compatibility until admin dashboard migrated

**File:** `/src/lib/services/juno/userService.ts`

---

### 4. juno/agentService.ts - ADMIN DASHBOARD ⚠️

**Status:** Documented with deprecation warnings.

**Obsolete Collections:**
- ❌ `"agents"` → Should use `agent_canister` for business data

**Acceptable Juno Usage:**
- ✅ Agent metadata (location, business name, operating hours)
- ✅ Agent reviews (UI display data)
- ✅ Profile images (already in storage collection)

**Unacceptable Juno Usage:**
- ❌ Agent balances (revenue, commission, cashBalance, digitalBalance)
- ❌ Agent status changes that affect transactions

**Migration Plan:**
1. Split into two services:
   - `agentMetadataService.ts` (Juno) - UI-only data
   - Use `agentCanisterService` for all business logic
2. Update admin dashboard to fetch balances from `agent_canister`

**File:** `/src/lib/services/juno/agentService.ts`

---

### 5. juno/transactionService.ts - ADMIN DASHBOARD ⚠️

**Status:** Documented with deprecation warnings.

**Obsolete Collections:**
- ❌ `"transactions"` → Should use `data_canister.get_user_transactions()`

**Usage:**
- Admin dashboard transaction monitoring
- Transaction statistics and charts
- Transaction filtering

**Migration Plan:**
1. Create `adminDashboardService.ts` that aggregates from:
   - `wallet_canister` for fiat transactions
   - `crypto_canister` for crypto transactions
   - `agent_canister` for deposit/withdrawal operations
   - `data_canister` for historical queries
2. Update admin dashboard charts to use canister data

**File:** `/src/lib/services/juno/transactionService.ts`

---

## Verified Correct Usage

### juno.config.ts ✅

**Status:** CORRECT - Only storage collections defined.

**Valid Collections:**
- ✅ `profile-images` (storage)
- ✅ `agent-profile-images` (storage)
- ✅ `kyc_documents` (storage)

**No datastore collections defined** - this is intentional and correct.

**File:** `/juno.config.ts`

---

### reviewService.ts ✅

**Status:** CORRECT - Uses `data_canister` for reviews.

**Implementation:**
- Reviews stored on-chain in `data_canister`
- No Juno datastore usage
- Properly uses `dataCanisterService.getAgentReviews()`

**File:** `/src/lib/services/juno/reviewService.ts`

---

## Architecture Alignment

### Current Domain Canister Architecture

```
user_canister (~400KB)
├── User registration & authentication
├── PIN management (Argon2 hashing)
├── Profile management
└── Phone/Principal linking

wallet_canister (~600KB)
├── P2P fiat transfers
├── Balance queries
├── Transaction history
└── Fraud detection

agent_canister (~700KB)
├── Deposit operations
├── Withdrawal operations
├── Agent commission tracking
└── Monthly settlements

crypto_canister (~1.0M)
├── Buy/Sell ckBTC & ckUSDC
├── Crypto transfers
├── Token swaps
├── Escrow management
└── DEX integration (Sonic)

data_canister (Pure storage)
├── Users (backup/sync)
├── Balances (source of truth)
├── Transactions (audit trail)
└── Agent reviews
```

### What Should Be in Juno

**ONLY file storage and UI metadata:**
- ✅ Profile images
- ✅ Agent profile images
- ✅ KYC document uploads
- ✅ Agent metadata (location, business name, hours)
- ✅ User preferences (UI settings)

**NEVER business data:**
- ❌ User authentication (use `user_canister`)
- ❌ Balances (use `wallet_canister` or `agent_canister`)
- ❌ Transactions (use domain canisters + `data_canister`)
- ❌ PINs (use `user_canister` with Argon2)

---

## Migration Priority

### Immediate (Completed) ✅
1. ✅ Fix `roleGuard.ts` to prevent app crashes

### High Priority (Next Sprint)
2. ⚠️ Migrate `webHookServices.ts` to domain canisters
   - **Impact:** USSD banking operations
   - **Risk:** Data inconsistency, security vulnerabilities
   - **Effort:** 3-5 days

3. ⚠️ Update admin dashboard to use canister services
   - **Impact:** Admin monitoring and management
   - **Risk:** Stale data in dashboards
   - **Effort:** 2-3 days

### Medium Priority (Future)
4. Split `agentService.ts` into metadata vs. business logic
5. Remove obsolete Juno datastore references from all remaining files

---

## Testing Recommendations

After migration, verify:

1. **User Authentication:**
   - ✅ Web login redirects to correct dashboard based on role from `user_canister`
   - ✅ USSD PIN verification uses `user_canister.verify_pin()`
   - ✅ Role changes reflect immediately

2. **Balance Operations:**
   - ✅ Fiat balances match between `wallet_canister` and UI
   - ✅ Agent balances match between `agent_canister` and UI
   - ✅ No stale balance data from Juno

3. **Transaction Flow:**
   - ✅ Send money updates `wallet_canister` and `data_canister`
   - ✅ Deposit/withdrawal updates `agent_canister`
   - ✅ Transaction history comes from `data_canister`

4. **Admin Dashboard:**
   - ✅ User stats reflect canister data
   - ✅ Agent stats reflect canister data
   - ✅ Transaction charts show real-time canister data

---

## Files Modified in This Audit

1. `/src/lib/auth/roleGuard.ts` - Fixed user role fetching
2. `/src/lib/services/webHookServices.ts` - Added critical deprecation warnings
3. `/src/lib/services/juno/userService.ts` - Added deprecation warnings
4. `/src/lib/services/juno/agentService.ts` - Added deprecation warnings
5. `/src/lib/services/juno/transactionService.ts` - Added deprecation warnings
6. `/JUNO_MIGRATION_AUDIT.md` - This report

---

## Next Steps

1. **Immediate:** Test that app loads without `user_roles` error
2. **This Week:** Create migration ticket for `webHookServices.ts`
3. **Next Sprint:** Implement domain canister integration for USSD
4. **Following Sprint:** Migrate admin dashboard to canister services
5. **Post-Migration:** Delete obsolete Juno service files

---

## Contacts

- **Canister Migration Plan:** See `CANISTER_MIGRATION_PLAN.md`
- **Architecture Details:** See `REVISED_ARCHITECTURE.md`
- **Business Logic Analysis:** See `BUSINESS_LOGIC_ANALYSIS.md`
