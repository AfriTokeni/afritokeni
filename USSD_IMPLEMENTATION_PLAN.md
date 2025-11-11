# USSD Implementation Plan - FINAL

## ✅ Confirmed Requirements

1. **Phase 1 Priority:** Deposits/Withdrawals with commission display ⚠️ CRITICAL
2. **Commission Display:** YES - Show fees before user confirms
3. **Agent Operations:** Web app only (skip USSD agent menu)
4. **DAO:** Ready - Business Logic must integrate with ICP SNS

---

## 🚀 Implementation Phases

### Phase 1: Deposit & Withdrawal Commission (Week 1) ⚠️ CRITICAL

#### 1.1 Add Business Logic Methods
**File:** `business_logic_canister/src/lib.rs`

```rust
/// Create deposit request (user shows code to agent)
#[update]
async fn create_deposit_request(
    user_phone: String,
    agent_id: String,
    amount: u64,
) -> Result<DepositRequestResult, String> {
    verify_authorized_caller()?;
    
    // Get user principal from phone
    let user = services::user_operations::get_user_by_phone(&user_phone).await?;
    
    // Call deposit canister
    let deposit_tx = services::commission_client::create_deposit_request(
        user.principal,
        Principal::from_text(&agent_id)?,
        amount
    ).await?;
    
    Ok(DepositRequestResult {
        deposit_code: deposit_tx.deposit_code,
        amount_ugx: deposit_tx.amount_ugx,
        commission_ugx: deposit_tx.commission_ugx,
        net_amount: deposit_tx.amount_ugx - deposit_tx.commission_ugx,
    })
}

/// Get withdrawal fees estimate (before creating withdrawal)
#[update]
async fn get_withdrawal_fees(
    amount: u64,
) -> Result<WithdrawalFeesResult, String> {
    verify_authorized_caller()?;
    
    // Call withdrawal canister to get fee split
    let (platform_fee_rate, agent_fee_rate) = services::commission_client::get_withdrawal_fees().await?;
    
    let platform_fee = (amount * platform_fee_rate) / 10_000;
    let agent_fee = (amount * agent_fee_rate) / 10_000;
    let total_fees = platform_fee + agent_fee;
    let net_amount = amount.saturating_sub(total_fees);
    
    Ok(WithdrawalFeesResult {
        amount,
        platform_fee,
        agent_fee,
        total_fees,
        net_amount,
    })
}

/// Create withdrawal request (with fees already shown to user)
#[update]
async fn create_withdrawal_request(
    user_phone: String,
    agent_id: String,
    amount: u64,
    pin: String,
) -> Result<WithdrawalRequestResult, String> {
    verify_authorized_caller()?;
    
    // Verify PIN first
    services::user_operations::verify_user_pin(&user_phone, &pin).await?;
    
    // Get user principal
    let user = services::user_operations::get_user_by_phone(&user_phone).await?;
    
    // Call withdrawal canister
    let withdrawal_tx = services::commission_client::create_withdrawal_request(
        user.principal,
        Principal::from_text(&agent_id)?,
        amount
    ).await?;
    
    Ok(WithdrawalRequestResult {
        withdrawal_code: withdrawal_tx.withdrawal_code,
        amount_ugx: withdrawal_tx.amount_ugx,
        platform_fee_ugx: withdrawal_tx.platform_fee_ugx,
        agent_fee_ugx: withdrawal_tx.agent_fee_ugx,
        net_amount: amount - withdrawal_tx.platform_fee_ugx - withdrawal_tx.agent_fee_ugx,
    })
}
```

#### 1.2 Update USSD Deposit Flow
**File:** `ussd_canister/src/flows/local_currency/deposit.rs`

```rust
// Step 1: Enter agent ID
// Step 2: Enter amount
// Step 3: Show commission and confirm
// Step 4: Create deposit request and show code

pub async fn handle_deposit(session: &mut UssdSession, input: &str) -> (String, bool) {
    match session.step {
        0 => {
            // Step 0: Enter agent ID
            session.step = 1;
            ("Enter agent ID:", true)
        }
        1 => {
            // Step 1: Validate agent ID and ask for amount
            session.set_data("agent_id", input);
            session.step = 2;
            ("Enter amount (UGX):", true)
        }
        2 => {
            // Step 2: Parse amount, calculate commission, show confirmation
            let amount: u64 = input.parse().map_err(|_| "Invalid amount")?;
            session.set_data("amount", &amount.to_string());
            
            // Calculate commission (0.5%)
            let commission = (amount * 50) / 10_000;
            let net_amount = amount - commission;
            
            session.step = 3;
            (format!(
                "Deposit: {} UGX\nCommission: {} UGX\nYou receive: {} UGX\n\n1. Confirm\n2. Cancel",
                amount, commission, net_amount
            ), true)
        }
        3 => {
            // Step 3: Confirm and create deposit request
            if input == "1" {
                let agent_id = session.get_data("agent_id").unwrap();
                let amount: u64 = session.get_data("amount").unwrap().parse().unwrap();
                
                match business_logic::create_deposit_request(
                    &session.phone_number,
                    &agent_id,
                    amount
                ).await {
                    Ok(result) => {
                        session.clear_data();
                        session.step = 0;
                        (format!(
                            "✅ Deposit request created!\n\nShow this code to agent:\n{}\n\nAmount: {} UGX\nCommission: {} UGX\nYou'll receive: {} UGX",
                            result.deposit_code,
                            result.amount_ugx,
                            result.commission_ugx,
                            result.net_amount
                        ), false) // END session
                    }
                    Err(e) => {
                        session.clear_data();
                        (format!("❌ Error: {}", e), false)
                    }
                }
            } else {
                session.clear_data();
                ("Deposit cancelled", false)
            }
        }
        _ => ("Invalid step", false)
    }
}
```

#### 1.3 Update USSD Withdrawal Flow
**File:** `ussd_canister/src/flows/local_currency/withdraw.rs`

```rust
// Step 1: Enter agent ID
// Step 2: Enter amount
// Step 3: Show ALL fees and confirm
// Step 4: Enter PIN
// Step 5: Create withdrawal request and show code

pub async fn handle_withdraw(session: &mut UssdSession, input: &str) -> (String, bool) {
    match session.step {
        0 => {
            session.step = 1;
            ("Enter agent ID:", true)
        }
        1 => {
            session.set_data("agent_id", input);
            session.step = 2;
            ("Enter amount (UGX):", true)
        }
        2 => {
            // Get withdrawal fees estimate
            let amount: u64 = input.parse().map_err(|_| "Invalid amount")?;
            session.set_data("amount", &amount.to_string());
            
            match business_logic::get_withdrawal_fees(amount).await {
                Ok(fees) => {
                    session.step = 3;
                    (format!(
                        "Withdrawal: {} UGX\nPlatform fee: {} UGX\nAgent fee: {} UGX\nTotal fees: {} UGX\nYou receive: {} UGX\n\n1. Confirm\n2. Cancel",
                        fees.amount,
                        fees.platform_fee,
                        fees.agent_fee,
                        fees.total_fees,
                        fees.net_amount
                    ), true)
                }
                Err(e) => {
                    session.clear_data();
                    (format!("❌ Error: {}", e), false)
                }
            }
        }
        3 => {
            if input == "1" {
                session.step = 4;
                ("Enter your PIN:", true)
            } else {
                session.clear_data();
                ("Withdrawal cancelled", false)
            }
        }
        4 => {
            // Create withdrawal request
            let agent_id = session.get_data("agent_id").unwrap();
            let amount: u64 = session.get_data("amount").unwrap().parse().unwrap();
            let pin = input;
            
            match business_logic::create_withdrawal_request(
                &session.phone_number,
                &agent_id,
                amount,
                pin
            ).await {
                Ok(result) => {
                    session.clear_data();
                    (format!(
                        "✅ Withdrawal request created!\n\nShow this code to agent:\n{}\n\nAmount: {} UGX\nFees: {} UGX\nYou'll receive: {} UGX",
                        result.withdrawal_code,
                        result.amount_ugx,
                        result.platform_fee_ugx + result.agent_fee_ugx,
                        result.net_amount
                    ), false)
                }
                Err(e) => {
                    session.clear_data();
                    (format!("❌ Error: {}", e), false)
                }
            }
        }
        _ => ("Invalid step", false)
    }
}
```

---

### Phase 2: Crypto Swap (Week 2)

#### 2.1 Add Business Logic Method
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
    
    // Verify PIN
    services::user_operations::verify_user_pin(&user_identifier, &pin).await?;
    
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
```

#### 2.2 Create USSD Crypto Swap Flow
**File:** `ussd_canister/src/flows/crypto/swap.rs` (NEW)

---

### Phase 3: DAO Integration (Week 3)

#### 3.1 Add Business Logic DAO Methods
```rust
/// Get user's DAO token balance
#[update]
async fn get_dao_token_balance(user_identifier: String) -> Result<u64, String> {
    // Call ICP SNS canister
}

/// Vote on DAO proposal
#[update]
async fn vote_on_proposal(
    user_identifier: String,
    proposal_id: u64,
    vote: bool, // true = yes, false = no
    pin: String,
) -> Result<(), String> {
    // Call ICP SNS canister
}

/// Get active proposals
#[update]
async fn get_active_proposals() -> Result<Vec<Proposal>, String> {
    // Call ICP SNS canister
}
```

#### 3.2 Wire USSD DAO Flows
- `flows/dao/check_tokens.rs` → `get_dao_token_balance`
- `flows/dao/vote.rs` → `vote_on_proposal`
- `flows/dao/create_proposal.rs` → ICP SNS (if allowed)

---

### Phase 4: Balance & History (Week 4)

#### 4.1 Wire Balance Flows
- `flows/common/balance.rs` → `get_balances`
- `flows/bitcoin/check_balance.rs` → `check_crypto_balance`
- `flows/usdc/check_balance.rs` → `check_crypto_balance`

#### 4.2 Wire Transaction History
- `flows/common/history.rs` → `get_transaction_history`
- Format for USSD (show last 5 transactions)
- Paginate if needed

#### 4.3 Wire PIN Change
- `flows/common/pin.rs` → `change_pin`

---

## 📋 Data Structures Needed

```rust
// business_logic_canister/src/lib.rs

#[derive(CandidType, Deserialize)]
pub struct DepositRequestResult {
    pub deposit_code: String,
    pub amount_ugx: u64,
    pub commission_ugx: u64,
    pub net_amount: u64,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawalFeesResult {
    pub amount: u64,
    pub platform_fee: u64,
    pub agent_fee: u64,
    pub total_fees: u64,
    pub net_amount: u64,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawalRequestResult {
    pub withdrawal_code: String,
    pub amount_ugx: u64,
    pub platform_fee_ugx: u64,
    pub agent_fee_ugx: u64,
    pub net_amount: u64,
}

#[derive(CandidType, Deserialize)]
pub struct SwapResult {
    pub from_amount: u64,
    pub to_amount: u64,
    pub spread_amount: u64,
    pub exchange_rate: String,
}
```

---

## ✅ Implementation Checklist

### Week 1: Deposits & Withdrawals
- [ ] Add `create_deposit_request` to Business Logic
- [ ] Add `get_withdrawal_fees` to Business Logic
- [ ] Add `create_withdrawal_request` to Business Logic
- [ ] Update USSD deposit flow to show commission
- [ ] Update USSD withdrawal flow to show all fees
- [ ] Test deposit flow end-to-end
- [ ] Test withdrawal flow end-to-end

### Week 2: Crypto Swap
- [ ] Add `swap_crypto` to Business Logic
- [ ] Create USSD crypto swap flow
- [ ] Show 0.5% spread to user
- [ ] Test BTC → USDC swap
- [ ] Test USDC → BTC swap

### Week 3: DAO Integration
- [ ] Add DAO methods to Business Logic
- [ ] Wire USSD DAO flows
- [ ] Test token balance check
- [ ] Test voting flow
- [ ] Test proposal creation (if allowed)

### Week 4: Balance & History
- [ ] Wire all balance check flows
- [ ] Wire transaction history flow
- [ ] Wire PIN change flow
- [ ] Format all responses for USSD
- [ ] Test all flows end-to-end

---

## 🎯 Success Criteria

1. ✅ User sees commission BEFORE confirming deposit
2. ✅ User sees ALL fees BEFORE confirming withdrawal
3. ✅ Deposit/withdrawal codes displayed clearly
4. ✅ Crypto swap shows spread percentage
5. ✅ DAO integration works with ICP SNS
6. ✅ All flows are stateless (parse from text parameter)
7. ✅ All responses fit USSD screen limits
8. ✅ Multi-language support works

---

**Ready to start Phase 1: Deposit & Withdrawal Commission!** 🚀
