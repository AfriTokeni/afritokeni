# AfriTokeni Exchange Canister

Rust canister that acts as an intermediary for ckBTC ↔ ckUSDC exchanges, automatically collecting a 0.5% spread and sending it to the DAO treasury.

## How It Works

1. **User initiates exchange** via frontend
2. **Canister receives tokens** from user (using ICRC-2 `transferFrom`)
3. **Canister deducts 0.5% spread** and sends to DAO treasury
4. **Canister swaps remaining 99.5%** (currently placeholder, needs DEX integration)
5. **Canister sends output tokens** to user

## Features

✅ **Automatic spread collection** - 0.5% goes to DAO treasury  
✅ **Slippage protection** - User sets `min_output` to prevent bad trades  
✅ **Configurable spread** - Can be adjusted (max 10%)  
✅ **ICRC-1/ICRC-2 compatible** - Works with ckBTC and ckUSDC  
✅ **Trustless** - All logic on-chain, no manual intervention  

## Deployment

### 1. Build the canister
```bash
dfx build exchange_canister
```

### 2. Deploy with treasury principal
```bash
# Replace with your DAO treasury principal
TREASURY_PRINCIPAL="xxxxx-xxxxx-xxxxx-xxxxx-cai"

dfx deploy exchange_canister --argument "(principal \"$TREASURY_PRINCIPAL\")"
```

### 3. Add canister ID to frontend env
```bash
# In sveltekit-app/.env
VITE_EXCHANGE_CANISTER_ID="<canister-id-from-deploy>"
VITE_DAO_TREASURY_PRINCIPAL="<treasury-principal>"
```

## TODO: DEX Integration

The `perform_swap` function is currently a placeholder. You need to integrate with a real DEX:

### Option 1: ICPSwap
```rust
// Call ICPSwap router canister
let result: CallResult<(u64,)> = ic_cdk::call(
    ICPSWAP_ROUTER,
    "swap",
    (from_token_principal, to_token_principal, amount, min_output),
).await;
```

### Option 2: Sonic
```rust
// Call Sonic swap canister
let result: CallResult<(u64,)> = ic_cdk::call(
    SONIC_SWAP,
    "swap",
    (from_token, to_token, amount),
).await;
```

### Option 3: Internal Liquidity Pool
- Implement your own AMM (Automated Market Maker)
- Manage liquidity pools for ckBTC/ckUSDC
- Calculate output using constant product formula: `x * y = k`

## API

### `swap_tokens(request: ExchangeRequest) -> Result<ExchangeResult, String>`

Swap tokens with automatic spread collection.

**Request:**
```rust
{
  from_token: Token::CkBTC,
  to_token: Token::CkUSDC,
  amount: 100000000, // 1 ckBTC (8 decimals)
  min_output: 40000000000 // Min 40,000 ckUSDC (6 decimals)
}
```

**Response:**
```rust
{
  output_amount: 41500000000, // 41,500 ckUSDC received
  spread_amount: 500000, // 0.005 ckBTC sent to treasury
  tx_id: "1234567890-xxxxx-xxxxx"
}
```

### `get_treasury() -> Option<Principal>`

Get the configured DAO treasury principal.

### `get_spread_percentage() -> u64`

Get current spread in basis points (50 = 0.5%).

### `set_spread_percentage(new_spread: u64) -> Result<(), String>`

Update spread percentage (max 1000 = 10%).

## Security Considerations

1. **Approval Required**: Users must approve the exchange canister to spend their tokens (ICRC-2 `approve`)
2. **Slippage Protection**: Always set `min_output` to prevent sandwich attacks
3. **Spread Limit**: Maximum spread is capped at 10%
4. **Treasury Immutable**: Treasury principal is set at init and cannot be changed (upgrade required)

## Testing

```bash
# Local testing
dfx start --clean
dfx deploy

# Test swap
dfx canister call exchange_canister swap_tokens '(record {
  from_token = variant { CkBTC };
  to_token = variant { CkUSDC };
  amount = 100000000;
  min_output = 40000000000;
})'
```

## Mainnet Canister IDs

- **ckBTC Ledger**: `mxzaz-hqaaa-aaaar-qaada-cai`
- **ckUSDC Ledger**: `xevnm-gaaaa-aaaar-qafnq-cai`
- **Exchange Canister**: TBD (after deployment)
- **DAO Treasury**: TBD (configure in env)
