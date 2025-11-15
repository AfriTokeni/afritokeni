# AfriTokeni Architecture

## System Overview

AfriTokeni uses a 3-tier architecture on the Internet Computer (IC), similar to traditional web applications but with blockchain-native security and authorization.

```
┌────────────────────────────────────────────────────────────────────────────┐
│                               External World                               │
│  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐                 │
│  │ USSD Gateway │     │  Web Clients │     │  Mobile Apps │ (roadmap)       │
│  └──────┬───────┘     └──────┬───────┘     └──────┬──────┘                 │
└─────────┼────────────────────┼────────────────────┼─────────────────────────┘
          │                    │                    │
┌─────────▼────────┐  ┌────────▼────────────┐  ┌────▼────────────────────┐
│  USSD Canister   │  │  Web App (Svelte +  │  │  Mobile App (Planned)   │
│  - Parse USSD    │  │  Juno HTTP Layer)   │  │  - Same domain APIs     │
│  - Format replies│  │  - UI / analytics   │  │  - Native experience    │
│  - Session mgmt  │  │  - Canister proxies │  │  - Coming milestone     │
└─────────┬────────┘  └────────┬────────────┘  └────────┬──────────────────┘
          │                    │                         │
          └──────────────┬─────┴──────────────┬──────────┘
                         ▼                    ▼
┌────────────────────────────────────────────────────────────────────────────┐
│                         BUSINESS LOGIC LAYER                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │    User      │  │   Wallet     │  │   Crypto     │              │
│  │  Canister    │  │  Canister    │  │  Canister    │              │
│  │ - User mgmt  │  │ - Transfers  │  │ - BTC/USDC   │              │
│  │ - KYC        │  │ - Balances   │  │ - Purchases  │              │
│  │ - Auth       │  │ - Escrow     │  │ - Sales      │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                    │
│  ┌──────────────┐                                                  │
│  │    Agent     │                                                  │
│  │  Canister    │                                                  │
│  │ - Deposits   │                                                  │
│  │ - Withdrawals│                                                  │
│  │ - Settlements│                                                  │
│  └──────────────┘                                                  │
└────────────────────────────────────────────────────────────────────────────┘
                         ↓
┌────────────────────────────────────────────────────────────────────────────┐
│                             STORAGE LAYER                                  │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │                        Data Canister                               │    │
│  │  - Pure CRUD operations                                             │    │
│  │  - Users, balances, transactions, escrows, settlements              │    │
│  │  - Can be called by: Domain canisters only                          │    │
│  └────────────────────────────────────────────────────────────────────┘    │
└────────────────────────────────────────────────────────────────────────────┘
```

## Authorization Model

### Access Control Matrix

| Canister | Can Call | Can Be Called By | Access Level |
|----------|----------|------------------|--------------|
| **Data Canister** | None | User, Wallet, Crypto, Agent | Domain + Admin |
| **User Canister** | Data | USSD | Presentation + Admin |
| **Wallet Canister** | Data, User | USSD | Presentation + Admin |
| **Crypto Canister** | Data | USSD | Presentation + Admin |
| **Agent Canister** | Data, Wallet | USSD | Presentation + Admin |
| **USSD Canister** | User, Wallet, Crypto, Agent | External Gateway | External + Admin |

### Authorization Levels

1. **Admin (Controller)**
   - Full access to all canisters
   - Can add/remove other admins
   - Emergency stop/start capabilities
   - Upgrade canisters

2. **Authorized Canister**
   - Can call specific canisters based on architecture
   - Cannot modify authorization
   - Cannot stop/start canisters

3. **User Self**
   - Users can access their own data
   - Cannot access other users' data
   - Cannot call canisters directly (must go through USSD/Web)

4. **Unauthorized**
   - No access
   - All calls rejected

## Data Flow Examples

### Example 1: User Registration (USSD)

```
1. USSD Gateway → USSD Canister
   handle_ussd_request("*123*1#", session_id, phone)

2. USSD Canister → User Canister
   register_user(phone, name, pin)

3. User Canister → Data Canister
   create_user(user_data)

4. Data Canister → User Canister
   Result<User, String>

5. User Canister → USSD Canister
   Result<User, String>

6. USSD Canister → USSD Gateway
   "Welcome to AfriTokeni! Your account is ready."
```

### Example 2: Send Money

```
1. USSD Gateway → USSD Canister
   handle_ussd_request("*123*2*500*254700000000#", session, phone)

2. USSD Canister → Wallet Canister
   transfer_fiat(from_user, to_phone, amount, currency)

3. Wallet Canister → User Canister
   get_user_by_phone(to_phone)

4. User Canister → Data Canister
   get_user_by_phone(to_phone)

5. Wallet Canister → Data Canister
   verify_balance(from_user, amount)
   deduct_balance(from_user, amount)
   add_balance(to_user, amount)
   store_transaction(tx)

6. Wallet Canister → USSD Canister
   Result<Transaction, String>

7. USSD Canister → USSD Gateway
   "Sent 500 KES to 254700000000. New balance: 1500 KES"
```

### Example 3: Buy Bitcoin

```
1. USSD Gateway → USSD Canister
   handle_ussd_request("*123*3*100#", session, phone)

2. USSD Canister → Crypto Canister
   purchase_crypto(user_id, amount, currency, crypto_type)

3. Crypto Canister → Data Canister
   verify_balance(user_id, amount)
   deduct_balance(user_id, amount)
   add_crypto_balance(user_id, btc_amount)
   store_transaction(tx)

4. Crypto Canister → ckBTC Ledger (external)
   icrc1_transfer(to: user_subaccount, amount: btc_amount)

5. Crypto Canister → USSD Canister
   Result<Transaction, String>

6. USSD Canister → USSD Gateway
   "Bought 0.0015 BTC for 100 USD. New BTC balance: 0.0015"
```

## Security Architecture

### 1. Non-Custodial Design

- **Users control their crypto**: Each user has a unique subaccount on ckBTC/ckUSDC ledgers
- **Platform cannot access user crypto**: Only users can initiate transfers
- **Fiat is custodial**: Fiat balances are managed by the platform (required for USSD)

### 2. Inter-Canister Security

```rust
// Every canister checks caller authorization
fn verify_authorized_caller() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    
    // 1. Check if controller (admin)
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    }
    
    // 2. Check if authorized canister
    AUTHORIZED_CANISTERS.with(|canisters| {
        if canisters.borrow().contains(&caller) {
            Ok(())
        } else {
            Err("Unauthorized caller".to_string())
        }
    })
}
```

### 3. Data Access Control

```rust
// Data canister enforces 3-tier access
enum AccessLevel {
    Controller,           // Platform admin - full access
    AuthorizedCanister,   // Domain canisters - full access
    UserSelf(String),     // Users - own data only
    Unauthorized,         // Rejected
}
```

### 4. Audit Trail

All operations are logged:
- Who called (principal)
- What function
- When (timestamp)
- Result (success/failure)

## Deployment Architecture

### Development Flow

```
┌─────────────┐
│   Local     │  dfx start --clean
│  Network    │  bash scripts/deploy.sh local
└─────────────┘
      ↓
┌─────────────┐
│ Playground  │  bash scripts/deploy.sh playground
│  (Testing)  │  bash scripts/test-deployment.sh playground
└─────────────┘
      ↓
┌─────────────┐
│ IC Mainnet  │  bash scripts/deploy.sh ic
│(Production) │  bash scripts/admin.sh add-admin ic <principal>
└─────────────┘
```

### Deployment Order

1. **Data Canister** (no dependencies)
2. **Domain Canisters** (depend on Data)
   - User Canister
   - Wallet Canister
   - Crypto Canister
   - Agent Canister
3. **USSD Canister** (depends on Domain)

### Authorization Setup

After deployment, authorization is configured:

```bash
# Data Canister authorizes domain canisters
data_canister.add_authorized_canister(user_canister)
data_canister.add_authorized_canister(wallet_canister)
data_canister.add_authorized_canister(crypto_canister)
data_canister.add_authorized_canister(agent_canister)

# Domain canisters authorize USSD
user_canister.add_authorized_canister(ussd_canister)
wallet_canister.add_authorized_canister(ussd_canister)
crypto_canister.add_authorized_canister(ussd_canister)
agent_canister.add_authorized_canister(ussd_canister)

# Set canister IDs for inter-canister calls
user_canister.set_data_canister_id(data_canister)
wallet_canister.set_data_canister_id(data_canister)
wallet_canister.set_user_canister_id(user_canister)
crypto_canister.set_data_canister_id(data_canister)
agent_canister.set_data_canister_id(data_canister)
agent_canister.set_wallet_canister_id(wallet_canister)
ussd_canister.set_all_domain_canister_ids(...)
```

## Comparison with Ethereum

| Aspect | Ethereum | Internet Computer (AfriTokeni) |
|--------|----------|-------------------------------|
| **Deployment** | Hardhat Ignition | `deploy.sh` script |
| **Authorization** | `onlyOwner` modifier | Principal-based access control |
| **Inter-contract calls** | Direct calls | Inter-canister calls with authorization |
| **Upgrades** | Proxy pattern | Native canister upgrades |
| **Storage** | State variables | Stable memory + heap |
| **Gas/Fees** | Gas fees per transaction | Cycles (pre-paid compute) |
| **Identity** | Ethereum addresses | IC Principals |
| **Admin** | Contract owner | Canister controller |

## Scalability

### Horizontal Scaling

Each canister can be scaled independently:
- **Data Canister**: Can be sharded by user ID
- **Domain Canisters**: Can have multiple instances
- **USSD Canister**: Can have multiple instances for load balancing

### Vertical Scaling

Canisters can be upgraded with:
- More memory (up to 4GB stable memory)
- More compute (cycles)
- Better algorithms

## Monitoring & Maintenance

### Health Checks

```bash
# Check all canisters
bash scripts/admin.sh status playground

# Check specific canister
dfx canister status data_canister --network playground
```

### Logs

```bash
# View canister logs
dfx canister logs data_canister --network playground
```

### Metrics

Monitor:
- Cycles balance (cost)
- Memory usage
- Call count
- Error rate

## Emergency Procedures

### 1. Stop All Canisters

```bash
bash scripts/admin.sh emergency-stop playground
```

### 2. Investigate Issue

```bash
# Check logs
dfx canister logs <canister-id> --network playground

# Check status
bash scripts/admin.sh status playground
```

### 3. Fix and Upgrade

```bash
# Fix code
# Build and upgrade
bash scripts/admin.sh upgrade playground
```

### 4. Restart

```bash
bash scripts/admin.sh emergency-start playground
```

---

**Last Updated**: November 2025
**Version**: 1.0.0
