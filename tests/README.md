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

## Test Coverage

### Unit Tests (tests/unit/)
- **162 scenarios** - USSD flows with mocks
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
- **19 scenarios** - Real blockchain integration
- ckBTC ledger queries (balance, metadata, transfers)
- ckUSDC ledger queries (balance, metadata, transfers)
- Real ICP local replica
- Initial balances from dfx.json
- Transfer simulations

## Test Results

```
✅ 58 Rust canister tests (100%)
✅ 162 USSD unit test scenarios (100%)
✅ 19 ICP integration scenarios (100%)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Total: 239 tests - 100% passing
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
