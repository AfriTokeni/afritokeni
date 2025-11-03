# AfriTokeni Test Suite

Comprehensive test coverage across all layers of the AfriTokeni platform.

## Test Structure

```
tests/
├── unit/           # USSD service unit tests (15 features, 162 scenarios)
├── integration/    # ICP canister integration tests (2 features, 19 scenarios)
├── e2e/            # End-to-end tests (5 features, 36 scenarios)
├── helpers/        # Shared test utilities
└── mocks/          # Mock implementations
```

## Running Tests

### All Tests
```bash
npm run test:all
```

### Unit Tests Only
```bash
npm run test:unit
```

### Integration Tests Only
```bash
npm run test:integration
```

### E2E Tests Only
```bash
npm run test:e2e
```

## Test Coverage

### Unit Tests (tests/unit/)
- USSD menu navigation
- Bitcoin buy/sell/send flows
- USDC buy/sell/send flows
- Local currency operations
- DAO voting
- Language support (English, Luganda, Swahili)
- Error handling
- Session management
- Cancel/back functionality

### Integration Tests (tests/integration/)
- ckBTC ledger integration
- ckUSDC ledger integration
- ICP canister interactions

### E2E Tests (tests/e2e/)
- **Deposit Flow** - Complete USSD deposit with canister integration
- **Withdrawal Flow** - Complete USSD withdrawal with agent confirmation
- **Exchange Flow** - ckBTC ↔ ckUSD exchanges with spread tracking
- **API Routes** - USSD/SMS webhook handling
- **Revenue Tracking** - Platform fees, agent commissions, on-chain verification

## Revenue Verification

Every E2E test verifies the revenue model:

### Deposits
- Platform earns **0.5%** on all deposits
- Example: 50,000 UGX deposit → 250 UGX platform fee

### Withdrawals
- Platform earns **0.5%** withdrawal fee
- Platform earns **10%** of agent commission
- Agent keeps **90%** of their commission
- Example: 100,000 UGX withdrawal → 800 UGX total platform revenue

### Exchanges
- Platform earns **0.5%** spread on all exchanges
- Tracked separately by token type (ckBTC, ckUSD)

## Test Results

```
✅ 58 Rust canister tests (100%)
✅ 162 USSD unit test scenarios
✅ 19 ICP integration test scenarios
✅ 36 E2E test scenarios
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Total: 275 tests across all layers
```

## Writing New Tests

### Unit Tests
Place in `tests/unit/` with step definitions in the same folder.

### Integration Tests
Place in `tests/integration/` - requires local ICP replica.

### E2E Tests
Place in `tests/e2e/` with step definitions in `tests/e2e/step-definitions/`.

## Notes

- Tests use BDD (Behavior-Driven Development) format with Cucumber
- TypeScript errors in SvelteKit files are expected (types generated at runtime)
- Unit tests use mocks for external dependencies
- Integration/E2E tests require running canisters
