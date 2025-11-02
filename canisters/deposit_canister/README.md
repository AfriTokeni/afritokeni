# AfriTokeni Deposit Canister

Production-grade canister for managing cash deposit transactions, agent commissions, and monthly settlements.

## Overview

This canister handles the **Cash → Digital Balance** flow:
1. User creates deposit request with deposit code
2. User brings cash to agent
3. Agent confirms deposit using code
4. User's balance is credited
5. Agent owes 0.5% commission to AfriTokeni
6. Monthly settlement enforced on-chain

## Features

✅ **Immutable audit trail** - All deposits recorded on-chain  
✅ **Automatic commission** - 0.5% calculated automatically  
✅ **Monthly settlements** - Track and enforce agent payments  
✅ **Deposit codes** - Secure verification system  
✅ **Real-time balances** - Agent commission tracking  
✅ **Revenue tracking** - Total company revenue visible  

## Data Structures

### DepositTransaction
```rust
{
  id: u64,
  user_principal: Principal,
  agent_principal: Principal,
  amount_ugx: u64,
  commission_ugx: u64,  // 0.5% automatic
  deposit_code: String,  // "DEP-00000001"
  timestamp: u64,
  status: Pending | Confirmed | Cancelled
}
```

### AgentBalance
```rust
{
  principal: Principal,
  total_deposits: u64,
  total_commission_owed: u64,
  total_commission_paid: u64,
  last_settlement_date: Option<u64>
}
```

### MonthlySettlement
```rust
{
  month: String,  // "2024-11"
  agent_principal: Principal,
  total_commission: u64,
  paid: bool,
  paid_date: Option<u64>
}
```

## API

### User Functions

#### `create_deposit_request(request: CreateDepositRequest) -> Result<DepositTransaction, String>`

User creates a deposit request.

**Request:**
```rust
{
  user_principal: Principal,  // Must match caller
  agent_principal: Principal,
  amount_ugx: 100000  // 100,000 UGX
}
```

**Response:**
```rust
{
  id: 1,
  deposit_code: "DEP-00000001",  // Show this to agent!
  commission_ugx: 500,  // 0.5% = 500 UGX
  status: Pending
}
```

### Agent Functions

#### `confirm_deposit(request: ConfirmDepositRequest) -> Result<DepositTransaction, String>`

Agent confirms deposit after receiving cash.

**Request:**
```rust
{
  deposit_code: "DEP-00000001",
  agent_principal: Principal  // Must match caller
}
```

**Response:**
```rust
{
  status: Confirmed,
  // User's balance is now credited
  // Agent now owes 500 UGX commission
}
```

#### `get_pending_deposits(agent: Principal) -> Vec<DepositTransaction>`

Get all pending deposits for an agent.

#### `get_agent_balance(agent: Principal) -> Option<AgentBalance>`

Check agent's commission balance.

### Company Functions

#### `create_monthly_settlement(month: String) -> Result<Vec<MonthlySettlement>, String>`

Generate settlement report for all agents (company wallet only).

**Example:**
```rust
create_monthly_settlement("2024-11")
// Returns list of all agents with outstanding commissions
```

#### `mark_settlement_paid(month: String, agent: Principal) -> Result<(), String>`

Mark agent's settlement as paid (company wallet only).

#### `get_total_revenue() -> u64`

Get total revenue (all commissions owed).

## Deployment

### 1. Build
```bash
dfx build deposit_canister
```

### 2. Deploy
```bash
# Replace with your company wallet principal
COMPANY_WALLET="icjnv-sun2n-xwa4k-pagqz-5xhjn-ul2uf-njoee-gr5v2-espqh-bwmfd-zqe"

dfx deploy deposit_canister --argument "(principal \"$COMPANY_WALLET\")"
```

### 3. Add to frontend
```bash
# In sveltekit-app/.env
VITE_DEPOSIT_CANISTER_ID="<canister-id-from-deploy>"
```

## Usage Flow

### User Side:
```typescript
// 1. Create deposit request
const result = await depositCanister.create_deposit_request({
  user_principal: userPrincipal,
  agent_principal: selectedAgent,
  amount_ugx: 100000
});

// 2. Show deposit code to user
alert(`Your deposit code: ${result.deposit_code}`);
// User shows this code to agent
```

### Agent Side:
```typescript
// 1. Get pending deposits
const pending = await depositCanister.get_pending_deposits(agentPrincipal);

// 2. Agent receives cash from user
// 3. Agent enters deposit code
const confirmed = await depositCanister.confirm_deposit({
  deposit_code: "DEP-00000001",
  agent_principal: agentPrincipal
});

// 4. User's balance is now credited!
```

### Company Side (Monthly):
```typescript
// 1. Generate settlement report
const settlements = await depositCanister.create_monthly_settlement("2024-11");

// 2. Review and pay agents
for (const settlement of settlements) {
  console.log(`Agent ${settlement.agent_principal} owes ${settlement.total_commission} UGX`);
  
  // After payment via mobile money/bank:
  await depositCanister.mark_settlement_paid("2024-11", settlement.agent_principal);
}
```

## Commission Calculation

```rust
// 0.5% commission on all deposits
let commission = (amount * 50) / 10000;

// Example:
// 100,000 UGX deposit = 500 UGX commission
// 1,000,000 UGX deposit = 5,000 UGX commission
```

## Security

- ✅ Only user can create deposit for themselves
- ✅ Only assigned agent can confirm deposit
- ✅ Only company wallet can create/mark settlements
- ✅ Deposit codes are unique and sequential
- ✅ All transactions immutable on-chain

## Audit Trail

Every deposit is permanently recorded with:
- User principal
- Agent principal
- Amount
- Commission
- Timestamp
- Status

Perfect for regulatory compliance and audits!
