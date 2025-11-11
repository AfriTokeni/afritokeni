# Next Session: USSD Phase 2 - Crypto Swap & DAO Integration

## 🎯 Objective
Implement crypto swap (BTC ↔ USDC) and DAO integration for USSD flows.

---

## ✅ Current Status (Phase 1 Complete)

### What's Working
- ✅ 102/102 integration tests passing (100%)
- ✅ Business Logic Canister: All core operations
- ✅ Commission System: Deposit, Withdrawal, Exchange canisters
- ✅ USSD Flows: Deposit & Withdrawal with commission display
- ✅ Stateless USSD architecture

### Commission Rates Verified
- Deposit: 0.5% platform fee (90% company, 10% agent)
- Withdrawal: 0.5% platform + 10% agent fee
- Exchange: 0.5% spread (100% company)

---

## 🚀 Phase 2: Crypto Swap (Priority 1)

### Goal
Enable users to swap between BTC and USDC via USSD with 0.5% spread display.

### Tasks

#### 1. Add Business Logic Method
**File:** `business_logic_canister/src/lib.rs`

```rust
/// Swap between cryptocurrencies (BTC ↔ USDC)
#[update]
async fn swap_crypto(
    user_identifier: String,
    from_crypto: CryptoType,
    to_crypto: CryptoType,
    amount: u64,
    pin: String,
) -> Result<SwapResult, String> {
    verify_authorized_caller()?;
    
    // Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // Verify PIN
    if !services::data_client::verify_pin(&user.id, &pin).await? {
        return Err("Invalid PIN".to_string());
    }
    
    // Get user principal
    let user_principal = Principal::from_text(user.principal_id.as_ref()
        .ok_or("User has no principal ID")?)
        .map_err(|e| format!("Invalid principal ID: {:?}", e))?;
    
    // Call exchange canister
    let swap_result = services::commission_client::swap_tokens(
        from_crypto,
        to_crypto,
        amount,
        user_principal
    ).await?;
    
    Ok(SwapResult {
        from_amount: amount,
        to_amount: swap_result.output_amount,
        spread_amount: swap_result.spread_amount,
        exchange_rate: swap_result.exchange_rate,
    })
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct SwapResult {
    pub from_amount: u64,
    pub to_amount: u64,
    pub spread_amount: u64,
    pub exchange_rate: String,
}
```

#### 2. Add Commission Client Method
**File:** `business_logic_canister/src/services/commission_client.rs`

```rust
/// Swap tokens via exchange canister
pub async fn swap_tokens(
    from_token: CryptoType,
    to_token: CryptoType,
    amount: u64,
    user_principal: Principal,
) -> Result<ExchangeResult, String> {
    let exchange_canister = config::get_exchange_canister_id();
    
    let request = ExchangeRequest {
        from_token,
        to_token,
        amount,
        user_principal,
    };
    
    let response = Call::unbounded_wait(exchange_canister, "swap_tokens")
        .with_arg((request,))
        .await
        .map_err(|e| format!("Exchange canister call failed: {:?}", e))?;
    
    let (result,): (Result<ExchangeResult, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode exchange response: {:?}", e))?;
    
    result
}
```

#### 3. Add USSD Business Logic Client
**File:** `ussd_canister/src/services/business_logic/crypto.rs`

Add `swap_crypto` method similar to `buy_crypto` and `sell_crypto`.

#### 4. Create USSD Crypto Swap Flow
**File:** `ussd_canister/src/flows/crypto/swap.rs` (NEW)

```rust
/// Handle crypto swap flow
/// Steps: 0. Select from crypto → 1. Select to crypto → 2. Enter amount → 3. Show spread & confirm → 4. Enter PIN → 5. Execute swap
pub async fn handle_crypto_swap(text: &str, session: &mut UssdSession) -> (String, bool) {
    let parts: Vec<&str> = text.split('*').collect();
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Select from crypto (1=BTC, 2=USDC)
            (format!("Swap Crypto\n\nFrom:\n1. Bitcoin (BTC)\n2. USDC"), true)
        }
        1 => {
            // Select to crypto
            let from = parts.get(2).unwrap_or(&"");
            session.set_data("from_crypto", from);
            
            (format!("To:\n1. Bitcoin (BTC)\n2. USDC"), true)
        }
        2 => {
            // Enter amount
            (format!("Enter amount:"), true)
        }
        3 => {
            // Show spread and confirm
            let amount: u64 = parts.get(4).unwrap_or(&"").parse().unwrap_or(0);
            let spread = (amount * 50) / 10_000; // 0.5%
            
            (format!("Swap Details:\n\nAmount: {}\nSpread (0.5%): {}\n\n1. Confirm\n2. Cancel",
                amount, spread), true)
        }
        4 => {
            // Enter PIN
            (format!("Enter your PIN:"), true)
        }
        5 => {
            // Execute swap
            // Call business_logic::swap_crypto()
        }
        _ => (format!("Invalid selection"), false)
    }
}
```

#### 5. Wire to Main Menu
Update routing to include crypto swap option.

---

## 🏛️ Phase 3: DAO Integration (Priority 2)

### Goal
Enable users to check DAO token balance and vote on proposals via USSD.

### Tasks

#### 1. Add Business Logic DAO Methods
**File:** `business_logic_canister/src/lib.rs`

```rust
/// Get user's DAO token balance
#[update]
async fn get_dao_token_balance(user_identifier: String) -> Result<u64, String> {
    verify_authorized_caller()?;
    
    // Call ICP SNS canister
    // TODO: Implement SNS integration
    Ok(0)
}

/// Vote on DAO proposal
#[update]
async fn vote_on_proposal(
    user_identifier: String,
    proposal_id: u64,
    vote: bool, // true = yes, false = no
    pin: String,
) -> Result<(), String> {
    verify_authorized_caller()?;
    
    // Verify PIN
    // Call ICP SNS canister
    // TODO: Implement SNS integration
    Ok(())
}

/// Get active proposals
#[update]
async fn get_active_proposals() -> Result<Vec<Proposal>, String> {
    verify_authorized_caller()?;
    
    // Call ICP SNS canister
    // TODO: Implement SNS integration
    Ok(vec![])
}
```

#### 2. Wire USSD DAO Flows
**Files:**
- `ussd_canister/src/flows/dao/check_tokens.rs` - Check token balance
- `ussd_canister/src/flows/dao/vote.rs` - Vote on proposals

---

## 📋 Implementation Checklist

### Phase 2: Crypto Swap
- [ ] Add `swap_crypto` to Business Logic Canister
- [ ] Add `swap_tokens` to commission_client
- [ ] Add `SwapResult` type
- [ ] Create USSD crypto swap flow
- [ ] Wire to main menu
- [ ] Test BTC → USDC swap
- [ ] Test USDC → BTC swap
- [ ] Verify 0.5% spread calculation
- [ ] Test with invalid amounts
- [ ] Test with wrong PIN

### Phase 3: DAO Integration
- [ ] Research ICP SNS integration
- [ ] Add DAO methods to Business Logic
- [ ] Wire USSD token balance check
- [ ] Wire USSD voting flow
- [ ] Test token balance query
- [ ] Test voting flow
- [ ] Verify proposal display

### Phase 4: Balance & History (Optional)
- [ ] Wire balance check flows
- [ ] Wire transaction history
- [ ] Wire PIN change
- [ ] Format for USSD display

---

## 🧪 Testing Strategy

### Integration Tests
Create `crypto_swap_tests.rs`:
```rust
#[test]
fn test_swap_btc_to_usdc() {
    // Create swap request
    // Verify spread calculation
    // Verify balances updated
}

#[test]
fn test_swap_shows_spread_before_confirm() {
    // Verify user sees 0.5% spread
}
```

### USSD Flow Tests
Test complete swap flow:
1. Select BTC → USDC
2. Enter amount
3. See spread (0.5%)
4. Confirm
5. Enter PIN
6. Verify swap executed

---

## 📊 Success Criteria

### Phase 2: Crypto Swap
- ✅ User can swap BTC ↔ USDC via USSD
- ✅ User sees 0.5% spread BEFORE confirming
- ✅ Swap executes correctly
- ✅ Balances update properly
- ✅ Integration tests pass

### Phase 3: DAO Integration
- ✅ User can check DAO token balance
- ✅ User can view active proposals
- ✅ User can vote on proposals
- ✅ Votes are recorded on ICP SNS

---

## 🔗 Resources

### Exchange Canister
- Location: `canisters/exchange_canister/`
- Config: `exchange_config.toml`
- Spread: 50 basis points (0.5%)
- DEX: Sonic (`3xwpq-ziaaa-aaaah-qcn4a-cai`)

### ICP SNS Documentation
- [SNS Overview](https://internetcomputer.org/docs/current/developer-docs/integrations/sns/)
- [SNS Governance](https://internetcomputer.org/docs/current/developer-docs/integrations/sns/managing/sns-governance)

### Existing Code
- `business_logic_canister/src/services/crypto_operations.rs` - Crypto buy/sell logic
- `ussd_canister/src/flows/bitcoin/` - BTC flows
- `ussd_canister/src/flows/usdc/` - USDC flows
- `ussd_canister/src/flows/dao/` - DAO flows (to be wired)

---

## 💡 Key Considerations

### Crypto Swap
1. **Exchange Rate** - Get from exchange canister
2. **Slippage** - Consider adding slippage tolerance
3. **Minimum Amount** - Set minimum swap amount
4. **Gas Fees** - Consider transaction fees

### DAO Integration
1. **SNS Canister ID** - Get from mainnet/playground
2. **Voting Power** - Based on token balance
3. **Proposal Display** - Format for USSD (limited space)
4. **Vote Confirmation** - Show vote summary before confirming

---

## 🚀 Getting Started

### Step 1: Review Current Code
```bash
# Check exchange canister
cat canisters/exchange_canister/src/lib.rs

# Check existing crypto flows
cat canisters/ussd_canister/src/flows/bitcoin/buy.rs
cat canisters/ussd_canister/src/flows/usdc/buy.rs
```

### Step 2: Start with Crypto Swap
1. Add Business Logic method
2. Add commission client method
3. Create USSD flow
4. Test end-to-end

### Step 3: Move to DAO
1. Research SNS integration
2. Add Business Logic methods
3. Wire USSD flows
4. Test with playground

---

## 📝 Questions to Answer

1. **Exchange Canister** - Is `swap_tokens` method already implemented?
2. **SNS Integration** - Which SNS canister ID to use (mainnet/playground)?
3. **Token Balance** - How to query user's DAO token balance?
4. **Voting** - How to submit votes to SNS?
5. **Proposals** - How to fetch active proposals from SNS?

---

## 🎯 Expected Timeline

- **Week 1:** Crypto swap implementation & testing
- **Week 2:** DAO integration research & implementation
- **Week 3:** Testing & polish
- **Week 4:** Documentation & deployment

---

## ✅ Ready to Start!

All Phase 1 work is complete and committed. The architecture is solid, tests are passing, and USSD flows are working.

**Start with Phase 2: Crypto Swap!** 🚀
