# 💾 AfriTokeni Data Canister

**Pure CRUD Storage Layer for AfriTokeni's 3-Tier Architecture**

[![Security Audit](https://img.shields.io/badge/Security-Audited-green)](./SECURITY_AUDIT.md)
[![Test Coverage](https://img.shields.io/badge/Coverage-100%25-brightgreen)](./COVERAGE_REPORT.md)
[![Tests](https://img.shields.io/badge/Tests-17%20Passing-success)](#testing)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Features](#features)
- [API Reference](#api-reference)
- [Access Control](#access-control)
- [Data Models](#data-models)
- [Security](#security)
- [Development](#development)
- [Testing](#testing)
- [Deployment](#deployment)
- [Monitoring](#monitoring)

---

## 🎯 Overview

The Data Canister is the **storage layer** in AfriTokeni's 3-tier architecture. It provides pure CRUD operations with **no business logic**, ensuring clean separation of concerns.

### Purpose

- ✅ Persist user profiles, balances, transactions, PINs, escrows
- ✅ Serve data to authorized canisters (Business Logic, USSD, Web)
- ✅ Enable non-custodial user self-access
- ✅ Maintain comprehensive audit trail
- ❌ NO business logic, validation, or fraud detection

### Key Characteristics

| Characteristic | Description |
|----------------|-------------|
| **Type** | Pure Storage Layer |
| **Lines of Code** | 2,107 |
| **Endpoints** | 44 (22 update, 22 query) |
| **Dependencies** | `shared_types`, `ic-cdk`, `candid` |
| **Test Coverage** | 100% critical paths |
| **Security** | Multi-level access control |

---

## 🏗️ Architecture

### 3-Tier System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AFRITOKENI ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │         PRESENTATION LAYER (Tier 1)                  │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  • USSD Canister - Parse USSD input                 │    │
│  │  • Web Canister - HTTP requests                     │    │
│  │  • Mobile App - API calls                           │    │
│  └─────────────────┬───────────────────────────────────┘    │
│                    │                                          │
│                    ▼                                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │       BUSINESS LOGIC LAYER (Tier 2)                 │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  • Business Logic Canister                          │    │
│  │    - Transaction validation                         │    │
│  │    - Fraud detection                                │    │
│  │    - Multi-currency exchange                        │    │
│  │    - Escrow management                              │    │
│  │    - Settlement calculation                         │    │
│  └─────────────────┬───────────────────────────────────┘    │
│                    │                                          │
│                    ▼                                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │         DATA LAYER (Tier 3) ⬅ YOU ARE HERE         │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  • Data Canister (THIS CANISTER)                    │    │
│  │    - Pure CRUD operations                           │    │
│  │    - User storage                                   │    │
│  │    - Balance storage                                │    │
│  │    - Transaction history                            │    │
│  │    - PIN security                                   │    │
│  │    - Escrow storage                                 │    │
│  │    - Audit logging                                  │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Request
     │
     ▼
Presentation Layer (USSD/Web)
     │
     ▼
Business Logic Canister
     │ (validates, applies rules)
     ▼
Data Canister ⬅ YOU ARE HERE
     │ (stores/retrieves)
     ▼
Response
```

---

## ✨ Features

### Core Capabilities

- ✅ **User Management** - Create, read, update user profiles
- ✅ **Balance Management** - Fiat (39 currencies) + Crypto (ckBTC, ckUSDC)
- ✅ **PIN Security** - HMAC-SHA256 + Argon2 with lockout protection
- ✅ **Transaction History** - Immutable transaction records
- ✅ **Escrow System** - Crypto escrow storage
- ✅ **Settlement Tracking** - Monthly agent settlements
- ✅ **Audit Logging** - Comprehensive audit trail using shared library
- ✅ **Non-Custodial Access** - Users can query their own data

### Multi-Currency Support

**39 African Currencies**:
- East Africa: UGX, KES, TZS, RWF, BIF
- West Africa: NGN, GHS, XOF, GMD, SLL, LRD
- Southern Africa: ZAR, BWP, LSL, SZL, NAD, ZMW, MWK
- North Africa: EGP, MAD, TND, DZD, LYD
- Central Africa: XAF, CDF, AOA
- Other: ETB, SOS, SDG, SSP, DJF, ERN, MUR, SCR, MGA, KMF, CVE, STN, MRU

**Crypto Assets**:
- ckBTC (Chain-Key Bitcoin)
- ckUSDC (Chain-Key USDC)

---

## 📡 API Reference

### Admin Endpoints (Controller Only)

#### `add_authorized_canister(canister_id: String) -> Result<(), String>`
Add a canister to the authorized list.

```rust
// Example
add_authorized_canister("rrkah-fqaaa-aaaaa-aaaaq-cai")
```

#### `remove_authorized_canister(canister_id: String) -> Result<(), String>`
Remove a canister from the authorized list.

#### `list_authorized_canisters() -> Result<Vec<String>, String>`
List all authorized canisters.

---

### User Management (Canister Only)

#### `create_user(request: CreateUserRequest) -> Result<User, String>`
Create a new user account.

```rust
CreateUserRequest {
    user_type_str: "User",
    preferred_currency_str: "UGX",
    email: "user@example.com",
    first_name: "Jane",
    last_name: "Doe",
    principal_id: Some("2vxsx-fae"),
    phone_number: Some("+256700000111")
}
```

#### `get_user(user_id: String) -> Result<Option<User>, String>`
Get user by ID (user can access their own).

#### `get_user_by_phone(phone_number: String) -> Result<Option<User>, String>`
Get user by phone number (canister only).

#### `get_user_by_principal(principal_id: String) -> Result<Option<User>, String>`
Get user by Internet Identity principal (canister only).

#### `get_my_user_data() -> Result<Option<User>, String>`
User self-access endpoint.

#### `update_last_active(user_id: String) -> Result<(), String>`
Update user's last active timestamp.

#### `update_user_phone(request: UpdateUserPhoneRequest) -> Result<(), String>`
Update user's phone number.

---

### Balance Operations

#### `get_fiat_balance(user_id: String, currency: FiatCurrency) -> Result<u64, String>`
Get fiat balance for a specific currency.

#### `get_crypto_balance(user_id: String) -> Result<CryptoBalance, String>`
Get crypto balances (ckBTC, ckUSDC).

#### `get_my_balances() -> Result<(Vec<FiatBalance>, CryptoBalance), String>`
User self-access for all balances.

#### `set_fiat_balance(user_id: String, currency: String, amount: u64) -> Result<(), String>`
Set fiat balance (CRUD only, no validation).

#### `deposit_fiat(user_id: String, amount: u64, currency: FiatCurrency, description: Option<String>) -> Result<Transaction, String>`
Record fiat deposit.

#### `withdraw_fiat(user_id: String, amount: u64, currency: FiatCurrency, description: Option<String>) -> Result<Transaction, String>`
Record fiat withdrawal.

#### `transfer_fiat(from_user: String, to_user: String, amount: u64, currency: FiatCurrency, description: Option<String>) -> Result<Transaction, String>`
Record fiat transfer between users.

#### `update_crypto_balance(user_id: String, ckbtc_delta: i64, ckusdc_delta: i64) -> Result<(), String>`
Update crypto balances (delta values).

#### `set_crypto_balance(user_id: String, ckbtc: u64, ckusdc: u64) -> Result<(), String>`
Set crypto balances directly (testing only).

---

### PIN Security

#### `setup_user_pin(request: SetupPinRequest) -> Result<(), String>`
Setup PIN with HMAC-SHA256.

```rust
SetupPinRequest {
    user_id: "user_001",
    pin: "1234",
    salt: "a1b2c3d4..." // hex-encoded
}
```

#### `verify_user_pin(user_id: String, pin: String) -> Result<bool, String>`
Verify PIN (handles lockout automatically).

#### `is_pin_locked(user_id: String) -> Result<bool, String>`
Check if PIN is locked.

#### `get_failed_attempts(user_id: String) -> Result<u32, String>`
Get failed PIN attempt count.

#### `get_remaining_lockout_time(user_id: String) -> Result<u64, String>`
Get remaining lockout time in seconds.

#### `reset_pin_attempts(user_id: String) -> Result<(), String>`
Reset failed attempts (admin only).

#### `store_pin_hash(user_id: String, pin_hash: String) -> Result<(), String>`
Store Argon2 PIN hash.

#### `get_pin_hash(user_id: String) -> Result<String, String>`
Get PIN hash for verification.

#### `increment_failed_attempts(user_id: String) -> Result<(), String>`
Increment failed attempt counter.

#### `change_pin(user_id: String, old_pin: String, new_pin: String, new_salt: String) -> Result<(), String>`
Change PIN (requires old PIN verification).

---

### Transaction & Escrow

#### `store_transaction(tx: Transaction) -> Result<(), String>`
Store transaction record.

#### `get_user_transactions(user_id: String, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Transaction>, String>`
Get user's transaction history.

#### `get_my_transactions(limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Transaction>, String>`
User self-access for transactions.

#### `store_escrow(escrow: Escrow) -> Result<(), String>`
Store escrow record.

#### `get_escrow(code: String) -> Result<Option<Escrow>, String>`
Get escrow by code.

#### `update_escrow_status(code: String, status: EscrowStatus) -> Result<(), String>`
Update escrow status.

#### `delete_escrow(code: String) -> Result<(), String>`
Delete escrow record.

#### `get_active_escrows() -> Result<Vec<Escrow>, String>`
Get all active escrows.

---

### Settlement & Audit

#### `store_settlements(month: String, settlements: Vec<MonthlySettlement>) -> Result<(), String>`
Store monthly settlements.

#### `mark_settlement_paid_record(month: String, agent_principal: String) -> Result<(), String>`
Mark settlement as paid.

#### `get_settlements_for_month(month: String) -> Result<Vec<MonthlySettlement>, String>`
Get settlements for a month.

#### `get_agent_settlements(agent_principal: String) -> Result<Vec<MonthlySettlement>, String>`
Get settlements for an agent.

#### `get_system_stats() -> Result<SystemStats, String>`
Get system statistics (admin only).

#### `get_audit_log(limit: Option<usize>) -> Result<Vec<AuditEntry>, String>`
Get audit log (admin only).

#### `get_audit_log_count() -> Result<usize, String>`
Get audit log entry count.

#### `get_audit_stats() -> Result<AuditStats, String>`
Get audit statistics.

#### `get_failed_operations(limit: Option<usize>) -> Result<Vec<AuditEntry>, String>`
Get failed operations (admin only).

---

## 🔐 Access Control

### Three-Tier Access Model

```
┌─────────────────────────────────────────────────────────────┐
│                    ACCESS CONTROL LAYERS                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  1. CONTROLLER (Platform Admin)                              │
│     ├─ add_authorized_canister                               │
│     ├─ remove_authorized_canister                            │
│     ├─ list_authorized_canisters                             │
│     ├─ get_system_stats                                      │
│     ├─ get_audit_log                                         │
│     └─ get_failed_operations                                 │
│                                                               │
│  2. AUTHORIZED CANISTERS (Business Logic, USSD, Web)         │
│     ├─ All CRUD operations                                   │
│     ├─ create_user, update_user_phone                        │
│     ├─ set_fiat_balance, deposit_fiat, withdraw_fiat         │
│     ├─ transfer_fiat, update_crypto_balance                  │
│     ├─ setup_user_pin, verify_user_pin, change_pin           │
│     ├─ store_transaction, store_escrow                       │
│     └─ store_settlements, mark_settlement_paid               │
│                                                               │
│  3. USER SELF-ACCESS (Non-Custodial)                         │
│     ├─ get_my_user_data                                      │
│     ├─ get_my_balances                                       │
│     └─ get_my_transactions                                   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Authorization Logic

```rust
fn get_access_level(user_id: Option<&str>) -> AccessLevel {
    let caller = msg_caller();
    
    // 1. Check if controller (admin)
    if ic_cdk::api::is_controller(&caller) {
        return AccessLevel::Controller;
    }
    
    // 2. Check if authorized canister
    if AUTHORIZED_CANISTERS.contains(&caller) {
        return AccessLevel::AuthorizedCanister;
    }
    
    // 3. Check if user accessing their own data
    if user_owns_data(caller, user_id) {
        return AccessLevel::UserSelf(user_id);
    }
    
    AccessLevel::Unauthorized
}
```

---

## 📊 Data Models

### User

```rust
pub struct User {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency,
    pub kyc_status: KYCStatus,
    pub is_verified: bool,
    pub created_at: u64,
    pub last_active: u64,
}
```

### FiatBalance

```rust
pub struct FiatBalance {
    pub user_id: String,
    pub currency: FiatCurrency,
    pub balance: u64,
    pub updated_at: u64,
}
```

### CryptoBalance

```rust
pub struct CryptoBalance {
    pub user_id: String,
    pub ckbtc: u64,
    pub ckusdc: u64,
    pub updated_at: u64,
}
```

### Transaction

```rust
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub amount: u64,
    pub currency_type: CurrencyType,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub description: Option<String>,
}
```

### Escrow

```rust
pub struct Escrow {
    pub code: String,
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub crypto_type: CryptoType,
    pub status: EscrowStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub claimed_at: Option<u64>,
}
```

---

## 🔒 Security

### PIN Security

**Dual System**:
1. **HMAC-SHA256** (Legacy) - Salt + PIN → Hash
2. **Argon2** (Modern) - Hash generated in user_canister

**Lockout Protection**:
- Max 3 failed attempts
- 30-minute lockout after 3 failures
- Automatic reset on successful verification

### Audit Logging

**100% Coverage** using shared audit library:

```rust
use shared_types::audit;

// Log success
audit::log_success("user_created", Some(user_id), details);

// Log failure
audit::log_failure("pin_failed", Some(user_id), details);
```

**Events Tracked**:
- User operations (created, updated, linked)
- Balance changes (deposit, withdraw, transfer)
- PIN operations (setup, verified, failed, reset)
- Escrow operations (stored, updated, deleted)
- Admin operations (canister authorized/removed)

### Access Control

- ✅ Controller-only admin operations
- ✅ Canister-only CRUD operations
- ✅ User self-access for non-custodial queries
- ✅ No anonymous access in production
- ✅ All operations audited

---

## 🛠️ Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install dfx (optional)
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

### Project Structure

```
canisters/data_canister/
├── src/
│   ├── lib.rs                    # Main canister logic (1,105 lines)
│   ├── models.rs                 # Data models (197 lines)
│   ├── operations/
│   │   ├── mod.rs
│   │   ├── user_ops.rs           # User CRUD (152 lines)
│   │   └── balance_ops.rs        # Balance CRUD (265 lines)
│   └── security/
│       ├── mod.rs
│       └── pin_ops.rs            # PIN security (312 lines)
├── tests/
│   ├── lib.rs
│   ├── common.rs
│   ├── integration.rs
│   └── unit/
│       ├── mod.rs
│       ├── balance_tests.rs
│       ├── storage_tests.rs
│       └── transaction_tests.rs
├── Cargo.toml
├── README.md                     # This file
├── SECURITY_AUDIT.md             # Security audit report
└── COVERAGE_REPORT.md            # Test coverage report
```

### Build

```bash
# From repository root
cargo build -p data_canister --target wasm32-unknown-unknown --release

# WASM output
# target/wasm32-unknown-unknown/release/data_canister.wasm
```

### Local Development

```bash
# Start local replica
dfx start --background

# Deploy canister
dfx deploy data_canister

# Initialize with authorized canisters
dfx canister call data_canister add_authorized_canister '("rrkah-fqaaa-aaaaa-aaaaq-cai")'
```

---

## 🧪 Testing

### Run All Tests

```bash
cd canisters/data_canister
cargo test --lib
```

**Expected Output**:
```
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured
```

### Test Suites

**Unit Tests (17)**:
- Currency validation (7 tests)
- Type system (4 tests)
- Audit logging (5 tests)
- State management (1 test)

**Integration Tests**:
- Validated via Business Logic Canister (80 tests)
- Inter-canister communication tested
- Data persistence verified

### Coverage

See [COVERAGE_REPORT.md](./COVERAGE_REPORT.md) for detailed coverage analysis.

---

## 🚀 Deployment

### Production Deployment

```bash
# 1. Build WASM
cargo build -p data_canister --target wasm32-unknown-unknown --release

# 2. Deploy to IC
dfx deploy data_canister --network ic

# 3. Initialize authorized canisters
dfx canister call data_canister add_authorized_canister \
  '("business-logic-canister-id")' --network ic

dfx canister call data_canister add_authorized_canister \
  '("ussd-canister-id")' --network ic

dfx canister call data_canister add_authorized_canister \
  '("web-canister-id")' --network ic
```

### Upgrade

```bash
# Build new WASM
cargo build -p data_canister --target wasm32-unknown-unknown --release

# Upgrade canister (state preserved)
dfx canister install data_canister --mode upgrade --network ic
```

---

## 📊 Monitoring

### System Stats

```bash
dfx canister call data_canister get_system_stats
```

**Response**:
```rust
record {
    total_users: 1234,
    total_transactions: 5678,
    total_fiat_balances: 2345,
    total_crypto_balances: 1234,
}
```

### Audit Log

```bash
# Get recent audit entries
dfx canister call data_canister get_audit_log '(opt 100)'

# Get audit statistics
dfx canister call data_canister get_audit_stats

# Get failed operations
dfx canister call data_canister get_failed_operations '(opt 50)'
```

### Performance Metrics

| Operation | Complexity | Performance |
|-----------|-----------|-------------|
| Create User | O(1) | Instant |
| Get User | O(1) | Instant |
| Get Balance | O(1) | Instant |
| Store Transaction | O(1) | Instant |
| Get User Transactions | O(n) | Linear scan |
| Store Escrow | O(1) | Instant |

---

## 📚 Additional Resources

- [Security Audit Report](./SECURITY_AUDIT.md) - Comprehensive security analysis
- [Coverage Report](./COVERAGE_REPORT.md) - Test coverage details
- [Shared Types](../shared_types/src/lib.rs) - Common data types
- [Business Logic Canister](../business_logic_canister/README.md) - Business rules layer

---

## 📄 License

See [LICENSE](../../LICENSE) at the repository root.

---

## 🤝 Contributing

This canister follows strict design principles:

1. **Pure CRUD Only** - No business logic
2. **Shared Types** - Use `shared_types` for all data models
3. **Audit Everything** - Use `shared_types::audit` for logging
4. **Access Control** - Verify caller on every endpoint
5. **No Panics** - Use `Result` types everywhere

---

*Data Canister - Pure Storage Layer for AfriTokeni*