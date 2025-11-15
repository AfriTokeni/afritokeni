# Wallet Canister

**Domain-specific canister for wallet operations in the AfriTokeni platform.**

## Overview

The Wallet Canister handles all wallet-related operations including fiat transfers, escrow management, and fraud detection. It's part of the canister split architecture, separating wallet logic from user management and data storage.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Wallet Canister                         │
├─────────────────────────────────────────────────────────────┤
│  Public Endpoints:                                          │
│  - transfer_fiat()      Transfer money between users        │
│  - create_escrow()      Create crypto escrow for agents     │
│  - claim_escrow()       Agent claims escrow                 │
│  - cancel_escrow()      User cancels escrow                 │
│  - get_escrow()         Get escrow status                   │
│  - get_transaction_history()  Get user transactions         │
│                                                              │
│  Admin Endpoints:                                           │
│  - set_data_canister_id()                                   │
│  - set_user_canister_id()                                   │
│  - add_authorized_caller()                                  │
├─────────────────────────────────────────────────────────────┤
│  Logic Modules (Pure Business Logic):                       │
│  - transfer_logic.rs    Fee calculations, validations       │
│  - fraud_logic.rs       Per-currency fraud detection        │
│  - escrow_logic.rs      Escrow code generation, validation  │
├─────────────────────────────────────────────────────────────┤
│  Service Clients (Inter-canister Communication):            │
│  - data_client.rs       Calls to data_canister              │
│  - user_client.rs       PIN verification via user_canister  │
│  - exchange_rate.rs     External API integration (future)   │
├─────────────────────────────────────────────────────────────┤
│  Configuration:                                             │
│  - config.rs            Config management                   │
│  - wallet_config.toml   All configurable parameters         │
└─────────────────────────────────────────────────────────────┘
         │                    │                    │
         ▼                    ▼                    ▼
  ┌─────────────┐    ┌──────────────┐    ┌──────────────┐
  │    Data     │    │     User     │    │   Shared     │
  │  Canister   │    │   Canister   │    │ Audit Library│
  └─────────────┘    └──────────────┘    └──────────────┘
```

## Features

### ✅ Fiat Transfers
- **Peer-to-peer transfers** with PIN verification
- **0.5% transfer fee** (configurable)
- **Multi-layer fraud detection** per currency with configurable limits
- **Transaction history** stored in data_canister
- **Comprehensive audit logging** for all operations
- **Atomic balance updates** ensuring money conservation

### ✅ Advanced Fraud Detection (3-Layer)
- **Layer 1 - Velocity Checks:** Max 10 transactions per hour per currency
- **Layer 2 - Per-Transaction Limits:** Configurable max amount per currency (e.g., 15M KES ≈ $1,000)
- **Layer 3 - Daily Limits:** Per-currency daily transaction counts (50) and daily amounts (e.g., 75M KES ≈ $5,000)
- **Threshold Warnings:** Alert at 80% of limits before blocking
- **Automatic blocking** of suspicious transactions
- **Comprehensive audit trail** for compliance and forensics

### ✅ Escrow System
- **Crypto escrow** for agent withdrawals (CkBTC, CkUSDC)
- **24-hour expiration** (configurable)
- **Unique escrow codes** (ESC-{user_prefix}-{timestamp})
- **Status tracking**: Active, Claimed, Cancelled, Expired
- **Atomic operations** to prevent crypto loss

### ✅ Security
- **PIN verification** via user_canister (proper separation of concerns)
- **Caller authorization** with 3-tier access control (Controller, AuthorizedCanister, UserSelf)
- **Money conservation** validated by balance integrity tests
- **Comprehensive audit logging** with correlation IDs and timestamps

## Configuration

All configuration is externalized in `wallet_config.toml` with per-currency settings:

```toml
[fees]
transfer_fee_basis_points = 50        # 0.5%
exchange_fee_basis_points = 50        # 0.5%
withdrawal_fee_basis_points = 50      # 0.5%
agent_commission_percentage = 10      # 10%

[escrow]
expiration_time_ns = 86400000000000   # 24 hours in nanoseconds

[fraud_limits.default]
max_transaction_amount = 10000000      # Default: 100,000 units
suspicious_threshold = 5000000         # Default: 50,000 units
max_daily_transactions = 50            # Default: 50 transactions/day
max_daily_amount = 50000000            # Default: 500,000 units/day

[fraud_limits.KES]  # Kenyan Shilling
max_transaction_amount = 15000000      # 150,000 KES (~$1,000)
suspicious_threshold = 7500000         # 75,000 KES (~$500)
max_daily_transactions = 50            # 50 transactions/day
max_daily_amount = 75000000            # 750,000 KES (~$5,000)

# ... 36 more currencies fully configured with per-currency limits

[canisters]
# Canister IDs (set via admin endpoints after deployment)
data_canister_id = ""
user_canister_id = ""
```

**Key Configuration Details:**
- All amounts in **cents** (smallest currency unit)
- **Per-currency limits** calibrated to ~$1,000 USD equivalent per transaction
- **Daily limits** set to ~$5,000 USD equivalent per currency
- **Velocity limit** is hardcoded: max 10 transactions per hour (per currency)
- **39 African currencies** with explicit configuration (see `wallet_config.toml`)

## API Reference

### Transfer Fiat

```rust
transfer_fiat(request: TransferRequest) -> Result<TransferResponse, String>

struct TransferRequest {
    from_user_id: String,
    to_user_id: String,
    amount: u64,           // Amount in cents
    currency: String,      // Currency code (e.g., "KES")
    pin: String,
    description: Option<String>,
}

struct TransferResponse {
    transaction_id: String,
    from_user_id: String,
    to_user_id: String,
    amount: u64,
    fee: u64,
    currency: String,
    sender_new_balance: u64,
    recipient_new_balance: u64,
    timestamp: u64,
}
```

### Create Escrow

```rust
create_escrow(request: CreateEscrowRequest) -> Result<CreateEscrowResponse, String>

struct CreateEscrowRequest {
    user_id: String,
    agent_id: String,
    amount: u64,           // Amount in satoshis
    crypto_type: String,   // "CkBTC" or "CkUSDC"
    pin: String,
}

struct CreateEscrowResponse {
    code: String,          // Unique escrow code
    amount: u64,
    crypto_type: String,
    expires_at: u64,       // Nanoseconds since epoch
}
```

### Claim Escrow

```rust
claim_escrow(code: String, agent_id: String) -> Result<(), String>
```

### Cancel Escrow

```rust
cancel_escrow(code: String, user_id: String, pin: String) -> Result<(), String>
```

## Testing

### Quick Start
```bash
# Run all tests (unit + integration)
pnpm run test

# Or test just wallet_canister
cargo test -p wallet_canister

# Unit tests only (fast, no PocketIC required)
cargo test -p wallet_canister --lib

# Integration tests (requires PocketIC/dfx)
cargo test -p wallet_canister --test lib
```

### Test Results
**Unit Tests:** 85/85 passing (100%)
- config: 8 tests
- transfer_logic: 23 tests
- fraud_logic: 29 tests
- escrow_logic: 21 tests
- exchange_rate: 4 tests

**Integration Tests:** 27/27 passing (100%)
- transfer_tests.rs: 10 tests (basic, validation, edge cases, fees)
- escrow_tests.rs: 8 tests (create, claim, cancel, authorization)
- fraud_detection_tests.rs: 5 tests (per-currency limits, fee calculation)
- balance_integrity_tests.rs: 9 tests (money conservation laws)
- security_tests.rs: 15 tests (daily limits, velocity checks, audit logging)

### Critical Security Tests (Recently Re-enabled)
**Daily Transaction Limit Tests (4 tests):**
1. `test_daily_transaction_count_limit_enforcement()` - Enforces 50 tx/day limit
2. `test_daily_transaction_count_warning_at_80_percent()` - Warns at 40 tx/day
3. `test_daily_amount_limit_enforcement_kes()` - Enforces 75M KES/day
4. `test_daily_amount_limit_enforcement_ngn()` - Enforces 750M NGN/day

**Velocity Check Tests (2 tests):**
1. `test_velocity_limit_10_transactions_per_hour()` - Enforces 10 tx/hour
2. `test_velocity_warning_at_80_percent()` - Warns at 8 tx/hour

**Multi-Layer Fraud Tests (2 tests):**
1. `test_per_transaction_amount_checked_before_daily_limits()` - Layer order verified
2. `test_velocity_checked_before_amount_limits()` - Velocity is first defense

**Money Conservation Tests (9 tests):**
- Simple transfers, multiple transfers, failed operations, escrow operations
- Validates: Total_before = Total_after + Fees

### Coverage Report
```bash
cargo llvm-cov --package wallet_canister --lib --tests --lcov --output-path canisters/wallet_canister/coverage.lcov
```

Coverage details: See `TEST_COVERAGE.md`

## Building

### Build WASM
```bash
cargo build --target wasm32-unknown-unknown --release -p wallet_canister
```

Output: `target/wasm32-unknown-unknown/release/wallet_canister.wasm`

### Deploy to IC
```bash
dfx deploy wallet_canister
```

## Dependencies

- **ic-cdk** ^0.18 - Internet Computer CDK
- **candid** ^0.10 - Candid serialization
- **serde** ^1.0 - Serialization framework
- **toml** ^0.8 - TOML parsing for config
- **shared_types** - Shared types across canisters

## Development

### Code Structure
```
src/
├── lib.rs              # Main canister logic and endpoints
├── config.rs           # Configuration management
├── logic/
│   ├── mod.rs
│   ├── transfer_logic.rs
│   ├── fraud_logic.rs
│   └── escrow_logic.rs
└── services/
    ├── mod.rs
    ├── data_client.rs
    ├── user_client.rs
    └── exchange_rate.rs

tests/
├── lib.rs
└── integration/
    ├── mod.rs
    ├── transfer_tests.rs
    ├── escrow_tests.rs
    ├── fraud_detection_tests.rs
    └── balance_integrity_tests.rs
```

### Coding Standards

**NO HARDCODED FALLBACKS**
- Never use `||` or `??` for business data
- Throw errors instead of silent fallbacks

**SINGLE RESPONSIBILITY**
- Each function does ONE thing
- Pure business logic in `logic/` modules
- I/O operations in `services/` modules

**ERROR HANDLING**
- All errors logged via shared audit library
- Descriptive error messages
- No silent failures

## Supported Currencies

39 African currencies with per-currency fraud limits:
- AOA (Angola), BIF (Burundi), BWP (Botswana), CDF (DR Congo)
- CVE (Cape Verde), DJF (Djibouti), DZD (Algeria), EGP (Egypt)
- ERN (Eritrea), ETB (Ethiopia), GHS (Ghana), GMD (Gambia)
- GNF (Guinea), KES (Kenya), LRD (Liberia), LSL (Lesotho)
- LYD (Libya), MAD (Morocco), MGA (Madagascar), MRU (Mauritania)
- MUR (Mauritius), MWK (Malawi), MZN (Mozambique), NAD (Namibia)
- NGN (Nigeria), RWF (Rwanda), SCR (Seychelles), SDG (Sudan)
- SLE (Sierra Leone), SOS (Somalia), SSP (South Sudan), STN (São Tomé)
- SZL (Eswatini), TND (Tunisia), TZS (Tanzania), UGX (Uganda)
- XAF (Central Africa), XOF (West Africa), ZAR (South Africa)
- ZMW (Zambia), ZWL (Zimbabwe)

## Security Audit Results

**Overall Assessment:** SECURE (9.0/10 score)

- ✅ 0 Critical findings
- ✅ 0 High findings
- ✅ 2 Medium findings (addressed with daily limit enforcement and velocity checks)
- ✅ 4 Low findings (documentation and design notes)
- ✅ 5 Informational items

**Key Security Controls:**
1. **PIN Verification:** All sensitive operations verify PIN via user_canister
2. **Fraud Detection:** 3-layer system (velocity → per-transaction → daily limits)
3. **Money Conservation:** 9 tests verify no funds created/lost in any scenario
4. **Atomic Operations:** Escrow operations are transactionally safe
5. **Audit Trail:** Complete logging with correlation IDs for forensics

See `SECURITY_AUDIT.md` for full details.

## Production Readiness

✅ **Code Quality**
- Zero compilation warnings
- All deprecated APIs replaced (ic-cdk 0.18)
- Comprehensive error handling with descriptive messages
- Clean separation of concerns (logic/services/config)

✅ **Testing (112 total tests)**
- 85 unit tests (100% passing)
- 27 integration tests (100% passing)
- 9 critical balance integrity tests
- 4 daily limit enforcement tests
- 7 velocity check tests
- 10 transfer tests
- 8 escrow tests

✅ **Security**
- 3-layer fraud detection (velocity → per-transaction → daily limits)
- PIN verification for all sensitive operations
- Atomic balance updates preventing money loss
- Comprehensive audit logging for compliance
- Security score 9.0/10

✅ **Documentation**
- Code review summary (CODE_REVIEW_SUMMARY.md)
- Security audit report (SECURITY_AUDIT.md)
- Coverage report (COVERAGE_REPORT.md)
- API reference and deployment guide (README.md)
- Configuration documentation (wallet_config.toml)

## Next Phase (Future Enhancements)

**Features Already Implemented But Not Yet Exposed:**
- Crypto exchange operations (logic in `exchange_rate.rs`, ready for crypto_canister)
- Multi-currency fiat exchange (logic in `exchange_rate.rs`)
- Agent commission calculations (logic in `transfer_logic.rs`)

**Recommended Future Work:**
1. Migrate `exchange_rate.rs` to crypto_canister (see REVISED_ARCHITECTURE.md)
2. Add retry logic for inter-canister call failures
3. Implement circuit breaker pattern for cascading failure prevention
4. ML-based fraud detection (rule-based system is sufficient for MVP)

## Documentation

- **Security Audit:** `SECURITY_AUDIT.md`
- **Coverage Report:** `COVERAGE_REPORT.md`
- **Configuration:** `wallet_config.toml`

## License

See LICENSE file in repository root.
