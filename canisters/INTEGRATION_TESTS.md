# Integration Tests Guide

This document explains how to run integration tests for AfriTokeni canisters using PocketIC.

## Overview

AfriTokeni uses [PocketIC](https://github.com/dfinity/pocketic) for integration testing. PocketIC is a lightweight, deterministic testing framework for Internet Computer canisters that runs entirely in-process without requiring a local replica.

## Prerequisites

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target installed
- Node.js and pnpm (for npm scripts)

```bash
# Install Rust target
rustup target add wasm32-unknown-unknown
```

## Running Tests

### All Integration Tests

Run all integration tests across all canisters:

```bash
npm run test:integration
# or
npm run test:integration:all
```

### Individual Canister Tests

Run tests for specific canisters:

```bash
# Business Logic Canister tests
npm run test:integration:business-logic

# USSD Canister tests
npm run test:integration:ussd

# Data Canister tests
npm run test:integration:data
```

### Direct Cargo Commands

You can also run tests directly with cargo:

```bash
# Business Logic Canister
cargo test --test lib --manifest-path canisters/business_logic_canister/Cargo.toml -- --test-threads=1

# USSD Canister
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml -- --test-threads=1

# Data Canister
cargo test --test lib --manifest-path canisters/data_canister/Cargo.toml -- --test-threads=1
```

### Running Specific Tests

To run a specific test:

```bash
# Example: Run only Bitcoin buy tests
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml integration::bitcoin_complete_tests::test_buy_bitcoin_with_ugx -- --nocapture

# Example: Run all registration tests
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml integration::registration -- --nocapture
```

## Test Structure

### Business Logic Canister Tests

Location: `canisters/business_logic_canister/tests/integration/`

Test suites:
- User registration and management
- Money transfers (fiat)
- Crypto operations (buy, sell, send)
- Deposits and withdrawals
- Balance integrity
- PIN security
- Error handling
- Escrow system
- Exchange rates
- Fraud detection

### USSD Canister Tests

Location: `canisters/ussd_canister/tests/integration/`

Test suites:
- Registration flows
- Bitcoin operations (buy, sell, send, balance, rate)
- USDC operations (buy, sell, send, balance, rate)
- Crypto swap operations
- Send money flows
- Withdraw flows
- Language selection
- Stateless USSD interactions

### Data Canister Tests

Location: `canisters/data_canister/tests/integration/`

Test suites:
- User CRUD operations
- Balance management
- Transaction storage
- Escrow operations

## CI/CD Integration

Integration tests run automatically in GitHub Actions on:
- Pull requests to `main`
- Pushes to `feature/**`, `fix/**`, `hotfix/**` branches

### GitHub Actions Workflow

The CI pipeline:
1. Builds all canister WASMs
2. Runs integration tests for each canister
3. Generates test summary
4. Continues even if some tests fail (business logic under development)

See `.github/workflows/ci.yml` for the full configuration.

## Test Mode Features

### PocketIC Advantages

- **Fast**: Runs in-process, no network overhead
- **Deterministic**: Consistent results across runs
- **Isolated**: Each test gets a fresh environment
- **No External Dependencies**: No need for local replica or ledger canisters

### Test Mode Implementation

The test infrastructure includes:

1. **Test Mode Flag**: Business logic canister has a test mode that skips external ledger calls
2. **Mock Responses**: Returns mock success responses for ledger operations
3. **Principal ID Generation**: Auto-generates principal IDs for test users
4. **Complete USSD Paths**: Tests can simulate full user flows in a single call

## Current Test Status

As of the latest update:

- **Business Logic Canister**: 80/80 tests passing (100%)
- **USSD Canister**: 42/239 tests passing (18%)
  - Infrastructure: 100% complete ✅
  - Remaining failures are business logic bugs, not test infrastructure issues
- **Data Canister**: Tests available

### Known Issues

1. **USSD Sell Operations**: Some sell tests fail due to business logic bugs
2. **USSD Send Operations**: Address validation needs improvement
3. **Multi-step Flows**: State management issues in complex workflows
4. **Edge Cases**: Some edge cases not fully implemented

See `canisters/ussd_canister/COMPREHENSIVE_FINAL_REPORT.md` for detailed analysis.

## Debugging Tests

### View Test Output

Use `--nocapture` to see println! output:

```bash
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml test_name -- --nocapture
```

### Run Tests Sequentially

Use `--test-threads=1` to avoid race conditions:

```bash
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml -- --test-threads=1
```

### Filter Tests

Run tests matching a pattern:

```bash
# Run all Bitcoin tests
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml bitcoin

# Run all buy tests
cargo test --test lib --manifest-path canisters/ussd_canister/Cargo.toml buy
```

## Writing New Tests

### Test Template

```rust
#[test]
fn test_my_feature() {
    let env = get_test_env();
    let phone = "+256700123456";
    
    // Setup
    env.register_user_direct(phone, "First", "Last", "email@test.com", "UGX", "1234")
        .expect("Registration");
    env.set_fiat_balance(phone, "UGX", 100000).expect("Set balance");
    
    // Execute
    let (response, _) = env.process_ussd("session", phone, "menu*option*input*pin");
    
    // Assert
    assert!(response.contains("success"), "Should succeed. Got: {}", response);
    
    // Verify state
    let balance = env.check_fiat_balance(phone, "UGX").expect("Get balance");
    assert_eq!(balance, expected_balance);
}
```

### Best Practices

1. **Use Unique Phone Numbers**: Avoid conflicts between tests
2. **Clean State**: Each test should be independent
3. **Descriptive Names**: Test names should clearly describe what they test
4. **Complete Paths**: Use complete USSD paths for realistic testing
5. **Error Messages**: Include helpful assertion messages
6. **Test One Thing**: Each test should verify one specific behavior

## Troubleshooting

### PocketIC Server Issues

If tests hang or fail to start:

```bash
# Kill any hanging PocketIC servers
pkill -f pocket-ic

# Clean and rebuild
cargo clean
cargo build --target wasm32-unknown-unknown --release
```

### WASM Build Issues

If WASMs fail to build:

```bash
# Ensure target is installed
rustup target add wasm32-unknown-unknown

# Clean build
cargo clean
cargo build --target wasm32-unknown-unknown --release --package canister_name
```

### Test Timeout Issues

If tests timeout:

```bash
# Increase timeout in CI or run with more time locally
cargo test --test lib -- --test-threads=1 --nocapture
```

## Resources

- [PocketIC Documentation](https://github.com/dfinity/pocketic)
- [IC Testing Best Practices](https://internetcomputer.org/docs/current/developer-docs/backend/rust/testing)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

## Contributing

When adding new tests:

1. Follow the existing test structure
2. Add tests to the appropriate canister's `tests/integration/` directory
3. Update this README if adding new test suites
4. Ensure tests pass locally before pushing
5. Document any known issues or expected failures

## Support

For issues or questions about integration tests:

1. Check existing test documentation in each canister
2. Review `COMPREHENSIVE_FINAL_REPORT.md` for USSD test details
3. Open an issue on GitHub with test output and reproduction steps
