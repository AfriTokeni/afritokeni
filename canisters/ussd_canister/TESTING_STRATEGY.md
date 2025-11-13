# USSD Canister Testing Strategy

## Architecture Overview

**USSD Canister = Presentation Layer Only**
- Parses user input from Africa's Talking
- Formats responses with translations
- Manages session state (language, currency, current step)
- Calls domain canisters for business logic
- **Does NOT contain business logic**

## Testing Layers

### 1. Unit Tests (Pure Logic) ✅
**Location**: `src/logic/`
**What**: Pure functions with no I/O, no async, no IC calls
**Coverage**: 23/23 tests passing

**Modules**:
- `validation.rs` - Input validation (phone, amount, PIN, BTC address)
- `send_money_logic.rs` - Send money flow parsing and validation
- More to be added for other flows

**Example Tests**:
```rust
#[test]
fn test_validate_phone_format_valid() {
    assert!(validate_phone_format("+256700000001").is_ok());
}

#[test]
fn test_calculate_transfer_fee() {
    assert_eq!(calculate_transfer_fee(10000.0), 50.0); // 0.5%
}
```

**Run**: `cargo test --package ussd_canister --lib logic::`

### 2. Integration Tests with Mocks
**Location**: `tests/unit/`
**What**: Test flows with mocked domain canister calls
**Purpose**: Verify USSD calls correct canisters with correct parameters

**Mock Infrastructure**:
- Feature flag: `test-utils`
- Mocked services: `wallet_client`, `user_client`, `crypto_client`, `agent_client`
- Mock functions: `set_mock_*`, `clear_mocks()`

**Example Test**:
```rust
#[tokio::test]
async fn test_send_money_calls_correct_canister() {
    // Mock wallet_client
    wallet_client::set_mock_transfer_fiat(|from, to, amount, currency, pin| {
        assert_eq!(amount, 1_000_000); // 10000 UGX in cents
        assert_eq!(currency, FiatCurrency::UGX);
        Ok(TransferResponse { ... })
    });
    
    // Run flow
    let (response, _) = handle_send_money("1*1*+256700000002*10000*1234", &mut session).await;
    
    // Verify correct response
    assert!(response.contains("successful"));
}
```

**Run**: `cargo test --package ussd_canister --test lib --features test-utils`

### 3. PocketIC Integration Tests
**Location**: `tests/integration/`
**What**: End-to-end tests with real canisters
**Purpose**: Verify actual inter-canister communication works

**Example Test**:
```rust
#[test]
fn test_send_money_e2e() {
    let pic = PocketIc::new();
    
    // Deploy all canisters
    let user_canister = deploy_user_canister(&pic);
    let wallet_canister = deploy_wallet_canister(&pic);
    let ussd_canister = deploy_ussd_canister(&pic);
    
    // Configure canister IDs
    configure_canisters(&pic, &ussd_canister, &user_canister, &wallet_canister);
    
    // Register users
    register_user(&pic, &user_canister, "+256700000001", "1234");
    register_user(&pic, &user_canister, "+256700000002", "5678");
    
    // Fund sender
    fund_wallet(&pic, &wallet_canister, "+256700000001", 50000);
    
    // Execute send money flow
    let response = process_ussd(&pic, &ussd_canister, "session1", "+256700000001", "1*1*+256700000002*10000*1234");
    
    // Verify success
    assert!(response.contains("successful"));
    
    // Verify balances updated
    let sender_balance = get_balance(&pic, &wallet_canister, "+256700000001");
    let recipient_balance = get_balance(&pic, &wallet_canister, "+256700000002");
    
    assert_eq!(sender_balance, 39950); // 50000 - 10000 - 50 (fee)
    assert_eq!(recipient_balance, 10000);
}
```

**Run**: `cargo test --package ussd_canister --test lib integration::`

## Test Coverage Goals

### Unit Tests (Pure Logic)
- [x] Phone number validation
- [x] Amount parsing and validation
- [x] PIN format validation
- [x] BTC address validation
- [x] Send money input parsing
- [x] Fee calculation
- [x] Balance checking logic
- [ ] Buy/Sell crypto input parsing
- [ ] Deposit/Withdraw input parsing
- [ ] Swap input parsing
- [ ] Menu navigation logic
- [ ] Translation verification

### Integration Tests (with Mocks)
- [ ] Send money flow (all steps)
- [ ] Buy Bitcoin flow
- [ ] Sell Bitcoin flow
- [ ] Send Bitcoin flow
- [ ] Buy USDC flow
- [ ] Sell USDC flow
- [ ] Send USDC flow
- [ ] Deposit flow
- [ ] Withdraw flow
- [ ] Crypto swap flow
- [ ] Balance check flow
- [ ] Registration flow
- [ ] Language switching
- [ ] Error handling

### PocketIC Tests (E2E)
- [ ] Complete send money journey
- [ ] Complete buy Bitcoin journey
- [ ] Complete deposit/withdraw journey
- [ ] Multi-user scenarios
- [ ] Error scenarios (insufficient balance, wrong PIN, etc.)
- [ ] Session persistence across steps

## Running Tests

```bash
# Unit tests only (pure logic, fast)
cargo test --package ussd_canister --lib logic::

# Integration tests with mocks
cargo test --package ussd_canister --test lib --features test-utils

# PocketIC tests (slow, requires all canisters)
cargo test --package ussd_canister --test lib integration::

# All tests
cargo test --package ussd_canister --features test-utils
```

## Test File Structure

```
canisters/ussd_canister/
├── src/
│   ├── logic/                    # Pure, testable logic
│   │   ├── mod.rs
│   │   ├── validation.rs         # ✅ 12 tests
│   │   ├── send_money_logic.rs   # ✅ 11 tests
│   │   └── ...                   # TODO: Add more flows
│   ├── flows/                    # Flow handlers (call logic + services)
│   └── services/                 # Domain canister clients (mockable)
└── tests/
    ├── unit/                     # Unit tests with mocks
    │   ├── send_money_tests.rs   # TODO: Fix to use pure logic
    │   └── ...
    └── integration/              # PocketIC E2E tests
        ├── send_money_flow_tests.rs
        └── ...
```

## Next Steps

1. ✅ Create pure logic modules for all flows
2. ✅ Add comprehensive unit tests for pure logic
3. ⏳ Update integration tests to use pure logic + mocks
4. ⏳ Add PocketIC tests for critical E2E flows
5. ⏳ Achieve 100% coverage of all menu paths

## Key Principles

1. **USSD is stateless** - Africa's Talking sends full USSD string each time
2. **Session state is minimal** - Only language, currency, current step, temp data
3. **No business logic in USSD** - All business logic in domain canisters
4. **Pure functions are testable** - Extract logic that doesn't need IC calls
5. **Mock for integration** - Test that correct canisters are called with correct params
6. **PocketIC for E2E** - Test real inter-canister communication
