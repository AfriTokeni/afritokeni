# AfriTokeni Revenue Configuration

**SINGLE SOURCE OF TRUTH** for all revenue model constants across all canisters.

## Revenue Streams

### 1. Platform Fees (0.5%)
Charged on ALL transactions (deposits, withdrawals, transfers).

**Constant**: `DEFAULT_PLATFORM_FEE_BPS = 50` (50 basis points = 0.5%)

**Location**:
- `deposit_canister/src/lib.rs` - Not used (agents owe commission instead)
- `withdrawal_canister/src/lib.rs` - `DEFAULT_PLATFORM_FEE_BPS`
- `exchange_canister/src/lib.rs` - TODO: Add exchange spread constant

### 2. Agent Commission Cut (10% of agent earnings)
Platform takes 10% of what agents earn, agents keep 90%.

**Constant**: `PLATFORM_CUT_OF_AGENT_FEE_PERCENT = 10` (10%)

**Location**:
- `withdrawal_canister/src/lib.rs` - `PLATFORM_CUT_OF_AGENT_FEE_PERCENT`

### 3. Agent Commission on Deposits (0.5%)
What agents owe AfriTokeni when they process deposits.

**Constant**: `DEFAULT_COMMISSION_RATE_BPS = 50` (50 basis points = 0.5%)

**Location**:
- `deposit_canister/src/lib.rs` - `DEFAULT_COMMISSION_RATE_BPS`

### 4. Agent Fee (Dynamic 2-12%)
What agents charge users for withdrawal services (location-based).

**Default**: `DEFAULT_AGENT_FEE_BPS = 300` (300 basis points = 3%)
**Range**: 200-1200 bps (2-12%)

**Location**:
- `withdrawal_canister/src/lib.rs` - `DEFAULT_AGENT_FEE_BPS`

### 5. Exchange Spread (0.5%)
Charged on ckBTC ↔ ckUSD swaps.

**Constant**: TODO - Add to exchange canister

## How to Change Fees

### ⚠️ IMPORTANT: Change in ONE place only

1. **Locate the constant** in the appropriate canister (see locations above)
2. **Update the const value** at the top of `lib.rs`
3. **Rebuild and test**: `cargo test --package <canister_name>`
4. **Regenerate bindings**: `npm run canisters:generate`
5. **Update this file** to reflect the change

### Example: Change platform fee from 0.5% to 0.75%

```rust
// In withdrawal_canister/src/lib.rs
const DEFAULT_PLATFORM_FEE_BPS: u64 = 75; // Changed from 50
```

Then:
```bash
cargo test --package withdrawal_canister
npm run canisters:generate
```

## Revenue Calculation Examples

### Withdrawal (100,000 UGX)
```
Platform base fee: 100,000 * 50 / 10,000 = 500 UGX (0.5%)
Agent total fee: 100,000 * 300 / 10,000 = 3,000 UGX (3%)
Platform cut: 3,000 * 10 / 100 = 300 UGX (10% of agent fee)

Total platform revenue: 500 + 300 = 800 UGX
Agent keeps: 3,000 - 300 = 2,700 UGX
```

### Deposit (100,000 UGX)
```
Agent commission: 100,000 * 50 / 10,000 = 500 UGX (0.5%)

Agent owes AfriTokeni: 500 UGX
```

## Basis Points Reference

| Percentage | Basis Points (bps) |
|------------|-------------------|
| 0.5%       | 50                |
| 1%         | 100               |
| 2%         | 200               |
| 3%         | 300               |
| 5%         | 500               |
| 10%        | 1000              |

**Formula**: `percentage * 100 = basis points`
**Usage**: `(amount * bps) / 10000`
