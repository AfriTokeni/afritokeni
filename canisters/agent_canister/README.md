# 🤝 AfriTokeni Agent Canister

**Agent-Facilitated Cash-to-Digital Gateway for AfriTokeni**

[![Security Audit](https://img.shields.io/badge/Security-Audited-green)](./SECURITY_AUDIT.md)
[![Test Coverage](https://img.shields.io/badge/Coverage-100%25-brightgreen)](./COVERAGE_REPORT.md)
[![Tests](https://img.shields.io/badge/Tests-91%20Passing-success)](#testing)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Features](#features)
- [API Reference](#api-reference)
- [Commission Structure](#commission-structure)
- [Security](#security)
- [Development](#development)
- [Testing](#testing)
- [Deployment](#deployment)
- [Monitoring](#monitoring)

---

## 🎯 Overview

The Agent Canister orchestrates **cash-to-digital** and **digital-to-cash** transactions through a network of trusted agents. It handles deposit/withdrawal flows, commission tracking, fraud detection, and settlement generation.

### Purpose

- ✅ Facilitate agent-mediated deposits (cash → digital balance)
- ✅ Facilitate agent-mediated withdrawals (digital balance → cash)
- ✅ Track agent commissions and settlements
- ✅ Enforce transaction limits and fraud detection
- ✅ Generate unique transaction codes
- ✅ Support multi-currency operations

### Key Characteristics

| Characteristic | Description |
|----------------|-------------|
| **Type** | Business Logic + Orchestration |
| **Lines of Code** | ~3,500 |
| **Endpoints** | 12 (6 deposit, 6 withdrawal) |
| **Dependencies** | `user_canister`, `wallet_canister`, `data_canister` |
| **Test Coverage** | 100% (91 tests: 51 unit + 40 integration) |
| **Security** | PIN verification, authorization, fraud detection |

---

## 🏗️ Architecture

### Agent Canister Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              AGENT CANISTER DEPENDENCIES                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │         PRESENTATION LAYER (Callers)                 │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  • USSD Canister - Agent operations via USSD        │    │
│  │  • Web Canister - Agent operations via web          │    │
│  │  • Mobile App - Agent operations via API            │    │
│  └─────────────────┬───────────────────────────────────┘    │
│                    │                                          │
│                    ▼                                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │       AGENT CANISTER ⬅ YOU ARE HERE                 │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  RESPONSIBILITIES:                                   │    │
│  │  • Deposit orchestration (cash → digital)           │    │
│  │  • Withdrawal orchestration (digital → cash)        │    │
│  │  • Commission calculation & tracking                │    │
│  │  • Fraud detection & limits                         │    │
│  │  • Transaction code generation                      │    │
│  │  • Settlement generation                            │    │
│  └─────────────────┬───────────────────────────────────┘    │
│                    │                                          │
│                    ▼                                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │         DEPENDENT CANISTERS                          │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │                                                      │    │
│  │  ┌──────────────────────────────────────────────┐   │    │
│  │  │  USER CANISTER                               │   │    │
│  │  │  • verify_pin(user_id, pin)                  │   │    │
│  │  │  • Lockout protection                        │   │    │
│  │  └──────────────────────────────────────────────┘   │    │
│  │                                                      │    │
│  │  ┌──────────────────────────────────────────────┐   │    │
│  │  │  WALLET CANISTER                             │   │    │
│  │  │  • get_fiat_balance(user_id, currency)       │   │    │
│  │  │  • add_fiat_balance(user_id, currency, amt)  │   │    │
│  │  │  • deduct_fiat_balance(user_id, currency, amt)│  │    │
│  │  └──────────────────────────────────────────────┘   │    │
│  │                                                      │    │
│  │  ┌──────────────────────────────────────────────┐   │    │
│  │  │  DATA CANISTER                               │   │    │
│  │  │  • store_deposit_transaction(tx)             │   │    │
│  │  │  • store_withdrawal_transaction(tx)          │   │    │
│  │  │  • update_agent_balance(agent_id, currency)  │   │    │
│  │  │  • get_agent_balance(agent_id, currency)     │   │    │
│  │  └──────────────────────────────────────────────┘   │    │
│  │                                                      │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Agent Transaction Flow

```
DEPOSIT FLOW (Cash → Digital)
┌──────────┐      ┌──────────┐      ┌──────────┐
│   User   │      │  Agent   │      │ Canister │
└────┬─────┘      └────┬─────┘      └────┬─────┘
     │                 │                  │
     │ 1. Brings cash  │                  │
     ├────────────────>│                  │
     │                 │                  │
     │                 │ 2. Create deposit│
     │                 │    (user PIN)    │
     │                 ├─────────────────>│
     │                 │                  │
     │                 │ 3. Deposit code  │
     │                 │<─────────────────┤
     │                 │                  │
     │ 4. Gives cash   │                  │
     │<────────────────┤                  │
     │                 │                  │
     │                 │ 5. Confirm       │
     │                 │    (agent PIN)   │
     │                 ├─────────────────>│
     │                 │                  │
     │ 6. Balance +    │ 7. Commission +  │
     │<────────────────┴──────────────────┤
     │                                    │

WITHDRAWAL FLOW (Digital → Cash)
┌──────────┐      ┌──────────┐      ┌──────────┐
│   User   │      │  Agent   │      │ Canister │
└────┬─────┘      └────┬─────┘      └────┬─────┘
     │                 │                  │
     │ 1. Request cash │                  │
     ├────────────────────────────────────>│
     │                 │                  │
     │ 2. Withdrawal code                 │
     │<────────────────────────────────────┤
     │                 │                  │
     │ 3. Show code    │                  │
     ├────────────────>│                  │
     │                 │                  │
     │                 │ 4. Confirm       │
     │                 │    (agent PIN)   │
     │                 ├─────────────────>│
     │                 │                  │
     │ 5. Receives cash│ 6. Commission +  │
     │<────────────────┴──────────────────┤
     │                                    │
```

---

## ✨ Features

### Core Operations

#### 1. Deposit Management
- **Create Deposit Request** - User brings cash to agent
- **Confirm Deposit** - Agent confirms cash received
- **Deposit Code Generation** - Unique DEP-{prefix}-{id}-{timestamp}
- **PIN Verification** - User and agent PIN validation
- **Balance Update** - Add to user's digital balance
- **Commission Tracking** - Agent earns 10% (keeps 90%)

#### 2. Withdrawal Management
- **Create Withdrawal Request** - User requests cash
- **Confirm Withdrawal** - Agent confirms cash given
- **Withdrawal Code Generation** - Unique WTH-{prefix}-{id}-{timestamp}
- **Fee Calculation** - 10% agent fee + 0.5% platform fee
- **Balance Deduction** - Deduct from user's balance
- **Commission Tracking** - Agent earns commission

#### 3. Agent Balance Tracking
- **Commission Earned** - Total commission from all transactions
- **Commission Pending** - Unpaid commission
- **Commission Paid** - Settled commission
- **Total Deposits** - Count of deposits processed
- **Total Withdrawals** - Count of withdrawals processed
- **Multi-Currency** - Separate tracking per currency

#### 4. Fraud Detection
- **Amount Limits** - Min/max per currency
- **Velocity Checks** - Rapid transaction detection
- **Volume Limits** - Daily/monthly caps
- **Pattern Detection** - Suspicious user-agent pairs

#### 5. Settlement Generation
- **Monthly Settlements** - Auto-generate for agents
- **Threshold Checks** - Minimum settlement amount
- **Payment Tracking** - Paid vs pending

---

## 📡 API Reference

### Deposit Endpoints

#### `create_deposit_request`
```rust
pub async fn create_deposit_request(
    request: CreateDepositRequest
) -> Result<CreateDepositResponse, String>
```

**Request:**
```rust
struct CreateDepositRequest {
    user_id: String,
    agent_id: String,
    amount: u64,
    currency: String,
    pin: String,  // User PIN
}
```

**Response:**
```rust
struct CreateDepositResponse {
    deposit_code: String,      // DEP-xxx-xxx-xxx
    amount: u64,
    currency: String,
    agent_commission: u64,     // 10% of amount
    net_to_user: u64,          // amount - commission
    expires_at: u64,           // 24 hours
}
```

#### `confirm_deposit`
```rust
pub async fn confirm_deposit(
    request: ConfirmDepositRequest
) -> Result<ConfirmDepositResponse, String>
```

**Request:**
```rust
struct ConfirmDepositRequest {
    deposit_code: String,
    agent_id: String,
    agent_pin: String,
}
```

### Withdrawal Endpoints

#### `create_withdrawal_request`
```rust
pub async fn create_withdrawal_request(
    request: CreateWithdrawalRequest
) -> Result<CreateWithdrawalResponse, String>
```

**Request:**
```rust
struct CreateWithdrawalRequest {
    user_id: String,
    agent_id: String,
    amount: u64,
    currency: String,
    pin: String,  // User PIN
}
```

**Response:**
```rust
struct CreateWithdrawalResponse {
    withdrawal_code: String,   // WTH-xxx-xxx-xxx
    amount: u64,
    currency: String,
    total_fees: u64,           // 10.5% of amount
    net_to_user: u64,          // amount - fees
    expires_at: u64,           // 24 hours
}
```

#### `confirm_withdrawal`
```rust
pub async fn confirm_withdrawal(
    request: ConfirmWithdrawalRequest
) -> Result<ConfirmWithdrawalResponse, String>
```

### Agent Balance Endpoints

#### `get_agent_balance`
```rust
pub async fn get_agent_balance(
    agent_id: String,
    currency: String
) -> Result<AgentBalanceResponse, String>
```

**Response:**
```rust
struct AgentBalanceResponse {
    agent_id: String,
    currency: String,
    total_deposits: u64,           // Count
    total_withdrawals: u64,        // Count
    commission_earned: u64,        // Total earned
    commission_paid: u64,          // Already paid
    commission_pending: u64,       // Awaiting settlement
}
```

---

## 💰 Commission Structure

### Deposit Fees (Per Whitepaper)

```
User brings 100,000 UGX cash to agent:

Agent Commission:     10% of 100,000 = 10,000 UGX
Platform Op Fee:      0.5% of 100,000 = 500 UGX
Platform Cut:         10% of 10,000 = 1,000 UGX

Agent Keeps:          10,000 - 1,000 = 9,000 UGX (90%)
Platform Revenue:     500 + 1,000 = 1,500 UGX
User Balance Added:   100,000 - 10,000 = 90,000 UGX
```

### Withdrawal Fees (Per Whitepaper)

```
User withdraws 100,000 UGX cash from agent:

Agent Fee:            10% of 100,000 = 10,000 UGX
Platform Op Fee:      0.5% of 100,000 = 500 UGX
Platform Cut:         10% of 10,000 = 1,000 UGX

Agent Keeps:          10,000 - 1,000 = 9,000 UGX (90%)
Platform Revenue:     500 + 1,000 = 1,500 UGX
Total Fees:           10,000 + 500 = 10,500 UGX
User Receives:        100,000 - 10,500 = 89,500 UGX cash
```

### Configuration

Fees are configured in `agent_config.toml`:

```toml
[fees.deposit]
agent_commission_basis_points = 1000  # 10%
platform_operation_fee_basis_points = 50  # 0.5%
platform_commission_cut_percentage = 10  # 10%

[fees.withdrawal]
agent_commission_basis_points = 1000  # 10%
platform_operation_fee_basis_points = 50  # 0.5%
platform_commission_cut_percentage = 10  # 10%
```

---

## 🔒 Security

### Multi-Layer Security

#### 1. PIN Verification
- **User PIN** - Required for deposit/withdrawal creation
- **Agent PIN** - Required for confirmation
- **Delegated to User Canister** - Centralized PIN management

#### 2. Authorization
- **Caller Verification** - Only authorized canisters can call
- **Agent Verification** - Agent must match transaction
- **User Verification** - User must match transaction

#### 3. Fraud Detection
- **Amount Limits** - Per currency min/max
- **Velocity Checks** - Rapid transaction detection
- **Volume Limits** - Daily caps
- **Pattern Detection** - Suspicious behavior

#### 4. Code Security
- **Unique Codes** - Timestamp-based generation
- **24-Hour Expiration** - Codes expire after 24 hours
- **Format Validation** - Strict format enforcement
- **One-Time Use** - Codes can only be confirmed once

#### 5. Audit Logging
- **100% Coverage** - All operations logged
- **Shared Audit Library** - Consistent logging
- **Caller Tracking** - Who called what
- **Timestamp Tracking** - When it happened

---

## 🛠️ Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install DFX
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Add wasm32 target
rustup target add wasm32-unknown-unknown
```

### Build

```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release -p agent_canister

# WASM will be at:
# target/wasm32-unknown-unknown/release/agent_canister.wasm
```

### Local Development

```bash
# Start local replica
dfx start --background

# Deploy locally
dfx deploy agent_canister

# Set canister IDs
dfx canister call agent_canister set_user_canister_id '(principal "xxxxx")'
dfx canister call agent_canister set_wallet_canister_id '(principal "xxxxx")'
dfx canister call agent_canister set_data_canister_id '(principal "xxxxx")'
```

---

## 🧪 Testing

### Test Suite Overview

```
Total Tests: 91
├── Unit Tests: 51 (100% pass)
│   ├── Config: 3 tests
│   ├── Deposit Logic: 15 tests
│   ├── Withdrawal Logic: 15 tests
│   └── Fraud Detection: 18 tests
│
└── Integration Tests: 40 (100% pass)
    ├── Core Operations: 7 tests
    ├── Settlement: 3 tests
    ├── Fraud Detection: 6 tests
    ├── Edge Cases: 5 tests
    ├── Multi-Currency: 8 tests
    ├── PIN Security: 5 tests
    ├── Code Validation: 4 tests
    └── Concurrent Ops: 4 tests
```

### Run Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test lib

# Run specific test
cargo test test_deposit_flow_end_to_end

# Run with output
cargo test -- --nocapture
```

### Test Coverage

See [COVERAGE_REPORT.md](./COVERAGE_REPORT.md) for detailed coverage analysis.

---

## 🚀 Deployment

### Production Deployment

```bash
# Build optimized WASM
./scripts/build.sh

# Deploy to IC mainnet
dfx deploy --network ic agent_canister

# Configure canister IDs
dfx canister --network ic call agent_canister set_user_canister_id '(principal "xxxxx")'
dfx canister --network ic call agent_canister set_wallet_canister_id '(principal "xxxxx")'
dfx canister --network ic call agent_canister set_data_canister_id '(principal "xxxxx")'
```

### Post-Deployment Checklist

- [ ] Verify canister IDs are set correctly
- [ ] Test deposit flow end-to-end
- [ ] Test withdrawal flow end-to-end
- [ ] Verify commission calculations
- [ ] Check audit logs
- [ ] Monitor error rates
- [ ] Verify authorization works

---

## 📊 Monitoring

### Key Metrics

```rust
// Agent Performance
- Total deposits processed
- Total withdrawals processed
- Average commission per transaction
- Settlement generation rate

// System Health
- Transaction success rate
- Average response time
- Error rate by endpoint
- Fraud detection triggers

// Business Metrics
- Total commission earned (all agents)
- Total commission paid
- Total commission pending
- Active agents count
```

### Audit Logs

All operations are logged with:
- Operation name
- Caller principal
- User ID (if applicable)
- Success/failure status
- Timestamp
- Additional context

Access logs via:
```bash
dfx canister logs agent_canister
```

---

## 📚 Additional Documentation

- [Security Audit](./SECURITY_AUDIT.md) - Comprehensive security analysis
- [Coverage Report](./COVERAGE_REPORT.md) - Detailed test coverage
- [Agent Config](./agent_config.toml) - Configuration reference

---

## 🤝 Contributing

This canister is part of the AfriTokeni platform. For contribution guidelines, see the main repository README.

---

## 📄 License

Copyright © 2025 AfriTokeni. All rights reserved.

---

**Last Updated:** November 13, 2025  
**Version:** 1.0.0  
**Status:** ✅ Production Ready
