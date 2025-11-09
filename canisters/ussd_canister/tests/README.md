# USSD Canister Tests

## Structure

```
tests/
├── common/                      # Shared test utilities
│   └── mod.rs                  # Test helpers, builders, constants
│
├── unit/                        # Unit tests (fast, isolated)
│   ├── session_tests.rs        # Session management
│   ├── validation_tests.rs     # Input validation (phone, amount, PIN)
│   ├── rate_limit_tests.rs     # Rate limiting logic
│   ├── integration_tests.rs    # Health checks, basic functionality
│   └── request_validation/     # HTTP request parsing tests
│       ├── main_menu_tests.rs
│       ├── local_currency_tests.rs
│       ├── bitcoin_tests.rs
│       ├── usdc_tests.rs
│       ├── dao_tests.rs
│       └── language_tests.rs
│
└── lib.rs                       # Test entry point
```

## Test Types

### Unit Tests (`tests/unit/`)
- **Fast**: Run in milliseconds
- **Isolated**: No external dependencies
- **Purpose**: Test individual functions and modules
- **Run**: `cargo test --package ussd_canister --lib`

### Integration Tests (NOT HERE!)
- **Location**: `/tests/integration/` (project root)
- **Framework**: Cucumber BDD
- **Purpose**: Test full USSD flows end-to-end
- **Run**: `npm run test:integration:ussd`

## Important Notes

⚠️ **The tests in `unit/request_validation/` are NOT integration tests!**
- They only test HTTP request parsing and structure
- They don't call the actual canister
- They were previously misnamed as "integration tests"
- Real integration tests use BDD and test actual canister calls

## Running Tests

```bash
# Run all unit tests
cargo test --package ussd_canister --lib

# Run specific test module
cargo test --package ussd_canister --lib validation

# Run with output
cargo test --package ussd_canister --lib -- --nocapture

# Run real integration tests (BDD)
npm run test:integration:ussd
```

## Writing New Tests

### Unit Test Example
```rust
#[test]
fn test_phone_validation() {
    use ussd_canister::utils::validation;
    assert!(validation::is_valid_phone("+254712345678"));
}
```

### Using Test Helpers
```rust
use crate::common::{create_test_session, test_phones};

#[test]
fn test_session_creation() {
    let session = create_test_session(test_phones::VALID_KENYA, "test_123");
    assert_eq!(session.phone_number, test_phones::VALID_KENYA);
}
```

## Test Coverage Goals

- ✅ Session management: 50+ tests
- ✅ Validation: 60+ tests  
- ✅ Rate limiting: 30+ tests
- ✅ Request parsing: 100+ tests
- 🎯 Flow logic: TODO
- 🎯 Error handling: TODO
