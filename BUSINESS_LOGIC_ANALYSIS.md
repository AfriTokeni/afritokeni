# Business Logic Canister - Detailed Analysis

## All Endpoints by Concern

### 1. USER MANAGEMENT (Identity & Authentication)
```rust
async fn register_user()              // Create new user
async fn user_exists()                 // Check if user exists
async fn verify_pin()                  // Authenticate user
async fn change_pin()                  // Update PIN
async fn link_phone_to_account()      // Link phone to principal
async fn get_user_profile()           // Get user details
async fn update_user_profile()        // Update user info
async fn get_user_by_phone()          // Lookup by phone
async fn get_user_by_principal()      // Lookup by principal
```
**Concern:** User identity, authentication, profile management
**Size:** ~400KB

---

### 2. FIAT MONEY OPERATIONS (Local Currency)
```rust
async fn transfer_money()             // P2P fiat transfer
async fn send_money_to_phone()        // Send to phone number
async fn send_money()                 // Alias for send_money_to_phone
async fn get_transfer_fee()           // Calculate transfer fees
async fn check_fiat_balance()         // Check fiat balance
async fn set_fiat_balance()           // Test helper
async fn get_transaction_history()    // Fiat transaction history
async fn get_recent_transactions()    // Recent fiat txs
```
**Concern:** Fiat transfers, balances, transaction history
**Size:** ~500KB

---

### 3. DEPOSIT OPERATIONS (Cash In via Agents)
```rust
async fn create_deposit_request()     // User requests deposit
async fn confirm_deposit()            // Agent confirms cash received
async fn get_deposit()                // Get deposit details
async fn get_user_deposits()          // User's deposit history
async fn get_agent_deposits()         // Agent's deposits
async fn get_pending_deposits()       // Pending deposits for agent
```
**Concern:** Cash deposits through agents
**Size:** ~300KB (from deposit_canister)

---

### 4. WITHDRAWAL OPERATIONS (Cash Out via Agents)
```rust
async fn withdraw_fiat()              // User withdraws cash
async fn create_withdrawal_request()  // Create withdrawal
async fn confirm_withdrawal()         // Agent confirms cash given
async fn get_withdrawal_fees()        // Calculate withdrawal fees
async fn get_withdrawal()             // Get withdrawal details
async fn get_user_withdrawals()       // User's withdrawal history
async fn get_agent_withdrawals()      // Agent's withdrawals
async fn get_pending_withdrawals()    // Pending for agent
```
**Concern:** Cash withdrawals through agents
**Size:** ~250KB (from withdrawal_canister)

---

### 5. AGENT MANAGEMENT (Commission & Settlements)
```rust
async fn get_agent_balance()          // Agent's commission balance
async fn create_monthly_settlement()  // Generate monthly payouts
async fn mark_settlement_paid()       // Mark settlement as paid
async fn get_settlements_for_month()  // Get month's settlements
async fn get_agent_settlements()      // Agent's settlement history
async fn get_total_revenue()          // Platform revenue
async fn get_commission_rate()        // Commission percentage
```
**Concern:** Agent commission tracking and payouts
**Size:** ~200KB (from deposit + withdrawal canisters)

---

### 6. CRYPTO BUYING/SELLING (Fiat ↔ Crypto)
```rust
async fn buy_crypto()                 // Buy BTC/USDC with fiat
async fn sell_bitcoin()               // Sell BTC for fiat
async fn sell_usdc()                  // Sell USDC for fiat
async fn get_crypto_value_estimate()  // Estimate fiat value
```
**Concern:** Converting between fiat and crypto
**Size:** ~250KB

---

### 7. CRYPTO TRANSFERS (Sending Crypto)
```rust
async fn send_crypto()                // Send BTC/USDC to address
async fn send_usdc()                  // Send USDC (wrapper)
async fn check_crypto_balance()       // Check crypto balance
async fn set_crypto_balance()         // Test helper
```
**Concern:** Sending crypto to external addresses
**Size:** ~150KB

---

### 8. CRYPTO SWAPS (BTC ↔ USDC)
```rust
async fn swap_crypto()                // Swap BTC ↔ USDC
async fn get_spread_basis_points()    // Get swap spread
```
**Concern:** Internal crypto swaps via DEX
**Size:** ~200KB (+ exchange_canister ~300KB)

---

### 9. ESCROW OPERATIONS (P2P Crypto Sales)
```rust
async fn sell_crypto_to_agent()       // Create escrow
async fn verify_escrow_code()         // Complete escrow
async fn get_escrow_status()          // Check escrow
async fn cancel_escrow()              // Refund escrow
```
**Concern:** Secure P2P crypto sales
**Size:** ~200KB

---

### 10. FRAUD DETECTION
```rust
// Embedded in transfer_money, withdraw_fiat, buy_crypto, etc.
// - Rate limiting
// - Amount limits
// - Suspicious transaction detection
```
**Concern:** Security and fraud prevention
**Size:** ~250KB

---

### 11. BALANCE QUERIES (Cross-cutting)
```rust
async fn get_balances()               // Get all balances (fiat + crypto)
```
**Concern:** Unified balance view
**Size:** ~50KB

---

## Proposed Split by Domain

### Option A: 3 Canisters (Your Original Plan)
```
user_canister (400KB)
├── User Management (9 endpoints)

wallet_canister (1.2M)
├── Fiat Operations (8 endpoints)
├── Deposits (6 endpoints)
├── Withdrawals (8 endpoints)
├── Agent Management (7 endpoints)
├── Fraud Detection
└── Balance Queries (fiat)

crypto_canister (1.0M)
├── Buy/Sell (4 endpoints)
├── Transfers (4 endpoints)
├── Swaps (2 endpoints + exchange_canister)
├── Escrow (4 endpoints)
└── Balance Queries (crypto)
```

**Issues:**
- wallet_canister is still big (1.2M)
- Mixes deposits/withdrawals with transfers
- Agent management split across concerns

---

### Option B: 4 Canisters (Better Separation)
```
user_canister (400KB)
├── User Management (9 endpoints)

wallet_canister (600KB)
├── Fiat Transfers (8 endpoints)
├── Fraud Detection
└── Balance Queries (fiat)

agent_canister (700KB)
├── Deposits (6 endpoints)
├── Withdrawals (8 endpoints)
├── Agent Management (7 endpoints)
└── Commission & Settlements

crypto_canister (1.0M)
├── Buy/Sell (4 endpoints)
├── Transfers (4 endpoints)
├── Swaps (2 endpoints + exchange)
├── Escrow (4 endpoints)
└── Balance Queries (crypto)
```

**Benefits:**
- Clear separation: P2P transfers vs Agent operations
- Agent canister handles all agent-related logic
- Each canister < 1M (50% of limit)

---

### Option C: 5 Canisters (Maximum Separation)
```
user_canister (400KB)
├── User Management (9 endpoints)

transfer_canister (500KB)
├── Fiat Transfers (8 endpoints)
├── Fraud Detection
└── Balance Queries (fiat)

agent_canister (700KB)
├── Deposits (6 endpoints)
├── Withdrawals (8 endpoints)
├── Agent Management (7 endpoints)
└── Commission & Settlements

crypto_wallet_canister (600KB)
├── Buy/Sell (4 endpoints)
├── Transfers (4 endpoints)
└── Balance Queries (crypto)

crypto_exchange_canister (500KB)
├── Swaps (2 endpoints + exchange)
├── Escrow (4 endpoints)
└── DEX Integration
```

**Benefits:**
- Maximum flexibility
- Each canister < 700KB (35% of limit)
- Can scale independently

**Drawbacks:**
- More canisters to manage
- More inter-canister calls
- Might be over-engineering

---

## Recommendation

### ✅ **Option B: 4 Canisters**

**Why:**
1. **Clear domain boundaries:**
   - Users = identity
   - Wallet = P2P money
   - Agents = cash in/out + commissions
   - Crypto = all crypto operations

2. **Natural business split:**
   - Agents are a distinct business model (B2B)
   - P2P transfers are different from agent operations
   - Crypto is self-contained

3. **Size is manageable:**
   - Largest is 1.0M (50% of limit)
   - Room to grow in each domain

4. **Not over-engineered:**
   - 4 canisters is reasonable
   - Not too many inter-canister calls
   - Easy to understand

---

## Final Architecture (Option B)

```
┌─────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                    │
│  ┌──────────────┐              ┌──────────────┐        │
│  │ USSD Canister│              │ Web Frontend │        │
│  └──────┬───────┘              └──────┬───────┘        │
└─────────┼──────────────────────────────┼───────────────┘
          │                              │
          └──────────────┬───────────────┘
                         │
┌────────────────────────┼────────────────────────────────┐
│                   DOMAIN LAYER                          │
│                        │                                │
│  ┌─────────┬──────────┴──────────┬──────────┐         │
│  │         │                     │          │         │
│  ▼         ▼                     ▼          ▼         │
│ ┌────┐  ┌────┐  ┌────┐  ┌────┐                       │
│ │USER│  │WALL│  │AGNT│  │CRYP│                       │
│ │    │  │ ET │  │    │  │ TO │                       │
│ │400K│  │600K│  │700K│  │1.0M│                       │
│ └─┬──┘  └─┬──┘  └─┬──┘  └─┬──┘                       │
│   │       │       │       │                           │
└───┼───────┼───────┼───────┼───────────────────────────┘
    │       │       │       │
    └───────┴───────┴───────┴───────────────────────────┐
                                                         │
┌────────────────────────────────────────────────────────┼┐
│                      STORAGE LAYER                     ││
│                         ┌──────▼───────┐              ││
│                         │     DATA     │              ││
│                         │   CANISTER   │              ││
│                         └──────────────┘              ││
└───────────────────────────────────────────────────────┘│
 └───────────────────────────────────────────────────────┘
```

**4 Domain Canisters:**
1. **user_canister** (400KB) - Identity & auth
2. **wallet_canister** (600KB) - P2P fiat transfers
3. **agent_canister** (700KB) - Deposits, withdrawals, commissions
4. **crypto_canister** (1.0M) - All crypto operations

**Total:** 2.7M spread across 4 canisters (average 675KB each)

Does this make more sense?
