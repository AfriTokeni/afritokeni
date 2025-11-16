# USSD Canister

The USSD Canister is the presentation layer and session management hub for AfriTokeni's SMS-based banking interface. It handles all Africa's Talking USSD webhook requests, manages user sessions, routes users through different flows, and coordinates with domain canisters to provide banking services via USSD (*229#).

## Purpose

- **USSD Interface**: Receives and responds to Africa's Talking USSD webhook requests
- **Session Management**: Maintains user session state across multiple menu interactions (5-minute timeout)
- **Multi-language Support**: Supports English, Luganda, and Swahili translations
- **Flow Routing**: Directs users through registration, send money, deposits, withdrawals, and crypto operations
- **Rate Limiting**: Protects against abuse with configurable rate limiting (1000 req/min for development)
- **Input Sanitization**: Cleans all user inputs to prevent injection attacks
- **Playground Mode**: Frontend testing mode with auto-registration and demo sessions

## Key Features

### Security Improvements
- **Expanded Currency Detection**: Supports 54 African countries and 39 currencies (auto-detected from phone number)
- **Input Sanitization**: All USSD inputs are sanitized against common injection attacks
- **Session Cleanup**: Automatic timeout and memory cleanup of expired sessions (5-minute timeout)
- **Rate Limiting**: Configurable per-minute request limits with lazy cleanup (10,000 max active sessions)
- **Playground Mode**: Frontend testing mode with prefix-based session detection
- **Navigation Priority Fix**: Active flows checked before navigation commands (prevents "0" from overriding active flows)

## USSD Flow Structure

The canister organizes USSD flows into modular, feature-specific directories:

### Local Currency Flows (`flows/local_currency/`)
- **Send Money** (`send_money.rs`): Transfer fiat between users via USSD
- **Deposit** (`deposit.rs`): Receive cash from agents, credit user's balance
- **Withdraw** (`withdraw.rs`): Withdraw cash via agents, debit user's balance

### Bitcoin Flows (`flows/bitcoin/`)
- **Buy ckBTC** (`buy.rs`): Purchase Bitcoin with local currency
- **Sell ckBTC** (`sell.rs`): Sell Bitcoin for local currency with approval flow
- **Send ckBTC** (`send.rs`): Transfer Bitcoin between users

### USDC Flows (`flows/usd/`)
- **Buy ckUSDC** (`buy.rs`): Purchase USDC stablecoin with local currency
- **Sell ckUSDC** (`sell.rs`): Sell USDC for local currency with approval flow
- **Send ckUSDC** (`send.rs`): Transfer USDC between users

### DAO Governance Flows (`flows/dao/`)
- **View Proposals** (`proposals.rs`): List active DAO proposals
- **Vote on Proposals** (`vote.rs`): Cast votes using USSD shorthand

### Crypto Swap Flow (`flows/crypto/`)
- **Token Swap** (`swap.rs`): Exchange between ckBTC and ckUSDC via Sonic DEX

### Common Utilities (`flows/common/`)
- **Bitcoin Rate** (`bitcoin_rate.rs`): Fetch current BTC conversion rates
- **USD Rate** (`usd_rate.rs`): Fetch current USD conversion rates
- **Transaction History** (`transactions.rs`): Retrieve user transaction records
- **Find Agent** (`find_agent.rs`): Locate nearest cash agents for deposits/withdrawals

## Playground Mode Configuration

Playground mode enables frontend testing without requiring live Africa's Talking integration:

### Configuration (`config.toml`)
```toml
[playground]
enabled = true
session_id_prefix = "playground_"
default_pin = "1234"           # Testing PIN (NEVER use in production)
default_currency = "UGX"       # Default currency for demo users
```

### Features
- **Auto-Registration**: Demo users registered automatically with prefix phone numbers
- **Session Detection**: Session IDs starting with `playground_` skip rate limiting
- **Fixed Credentials**: Default PIN `1234` works for all playground sessions
- **JSON Support**: Accepts JSON payloads in addition to form-urlencoded (Africa's Talking format)

### Usage
Send JSON requests to test flows without real USSD provider:
```json
{
  "sessionId": "playground_test_1",
  "phoneNumber": "+256700000001",
  "text": "1"  // Menu selections
}
```

## Multi-Language Support

The canister supports three languages with complete translation coverage:

- **English** (code: `en`) - Default
- **Luganda** (code: `lg`) - Uganda
- **Swahili** (code: `sw`) - East Africa

Language selection is persistent per session. All menus, error messages, and confirmations are translated via the `TranslationService` in `src/utils/translations.rs`.

## Session Management

### Session Lifecycle
1. **Creation**: Sessions created on first request with auto-detected language and currency
2. **Persistence**: State maintained across USSD interactions (phone number, menu, step, temp data)
3. **Timeout**: Sessions expire after 5 minutes of inactivity
4. **Cleanup**: Expired sessions automatically cleaned up (lazy cleanup during normal request processing)

### Configuration (`config.toml`)
```toml
[session]
timeout_minutes = 5
max_active_sessions = 10000
```

### Session Data
- `session_id`: Unique session identifier from Africa's Talking
- `phone_number`: User's phone number (with country code)
- `current_menu`: Current location in USSD flow (e.g., "main", "send_money", "bitcoin_buy")
- `step`: Current step within flow (0-based, increments as user progresses)
- `language`: User's selected language code
- `data`: Temporary storage for flow inputs (recipient, amount, PIN, etc.)

## Rate Limiting

Rate limiting protects the canister from abuse while allowing legitimate testing:

### Configuration (`config.toml`)
```toml
[rate_limiting]
max_requests_per_minute = 1000    # Development: 1000, Production: 100-200
rate_limit_window_seconds = 60
```

### Bypass Rules
- **Test Phone Numbers**: Numbers starting with `+254700` always allowed (integration tests)
- **JSON Requests**: Playground mode requests skip rate limiting
- **Lazy Cleanup**: Inactive entries cleaned up deterministically during requests

### Response
When rate limited: HTTP 429 with message "Rate limit exceeded"

## Inter-Canister Dependencies

The USSD canister coordinates with four domain canisters:

### User Canister (`user_canister`)
- **Register users**: Store phone, PIN, first name, last name, currency
- **Verify PIN**: Authenticate operations requiring PIN confirmation
- **Resolve identifiers**: Map phone number → user ID → principal

### Wallet Canister (`wallet_canister`)
- **Transfer fiat**: Send money between users in local currency
- **Check balance**: Query user's fiat balance
- **Transaction history**: Retrieve past transactions

### Crypto Canister (`crypto_canister`)
- **Buy/sell crypto**: Exchange fiat for ckBTC or ckUSDC
- **Crypto transfers**: Send ckBTC or ckUSDC between users
- **Token swaps**: Exchange ckBTC ↔ ckUSDC via Sonic DEX
- **Escrow operations**: Manage escrow for secure transactions
- **Approval flows**: Handle ICRC-2 token approvals for selling

### Agent Canister (`agent_canister`)
- **Deposit operations**: Credit user balance from agent cash deposits
- **Withdrawal operations**: Debit user balance for agent cash withdrawals
- **Agent commission**: Track and calculate agent fees
- **Monthly settlements**: Handle settlement transactions

## Configuration

All USSD behavior is configured via `src/config.toml`:

```toml
# Rate limiting (prevent abuse)
[rate_limiting]
max_requests_per_minute = 1000
rate_limit_window_seconds = 60

# PIN security (USSD users)
[pin_security]
max_pin_attempts = 3
lockout_duration_minutes = 30
min_pin_length = 4
max_pin_length = 4

# Transaction limits (flow-specific)
[transaction_limits]
min_amount_kes = 10.0
max_amount_kes = 1_000_000.0
min_amount_btc = 0.00001
max_amount_btc = 1.0
min_amount_usdc = 1.0
max_amount_usdc = 100_000.0

# Session management
[session]
timeout_minutes = 5
max_active_sessions = 10000

# Africa's Talking integration
[africas_talking]
username = "sandbox"
api_key = "test_key"
api_url = "https://api.sandbox.africastalking.com"
is_sandbox = true

# USSD defaults (generated emails for USSD users)
[ussd_defaults]
default_email_domain = "ussd.afritokeni.com"
default_currency = "UGX"

# Feature flags
[features]
enable_dao_voting = false
enable_rate_checking = false
enable_find_agent = false
enable_transaction_history = false
enable_language_switching = false

# Bitcoin address validation
[validation]
btc_address_min_length = 26
btc_address_max_length = 62
btc_strict_checksum_validation = false

# Frontend testing mode
[playground]
enabled = true
session_id_prefix = "playground_"
default_pin = "1234"
default_currency = "UGX"
```

## Deployment

### Local Development
```bash
# Start local ICP replica
dfx start --clean --background

# Deploy all canisters
dfx deploy

# Configure USSD canister with domain canister IDs
dfx canister call ussd_canister configure_domain_canisters \
  "(principal \"$USER_CANISTER_ID\", \
    principal \"$WALLET_CANISTER_ID\", \
    principal \"$CRYPTO_CANISTER_ID\", \
    principal \"$AGENT_CANISTER_ID\")"

# Start development server (frontend)
pnpm run dev
```

### Production Deployment
1. Merge changes to `main` branch
2. Create GitHub release with semantic version tag (e.g., `v1.0.0`)
3. CD pipeline automatically builds and deploys to mainnet ICP

**Production Canister IDs:**
- Retrieve from GitHub Actions deployment logs
- Update Africa's Talking webhook URL to production canister

## Testing

Comprehensive test suite with 250+ tests:

```bash
# Unit tests (fast, no blockchain)
cargo test --package ussd_canister --lib

# Integration tests (full flows with canister interactions)
cargo test --package ussd_canister --test lib --features test-utils -- --test-threads=1

# Test specific module
cargo test --package ussd_canister --lib validation

# Run with output for debugging
cargo test --package ussd_canister --lib -- --nocapture
```

**Test Coverage** (see `TEST_COVERAGE.md` for full details):
- Unit tests: Input validation, session management, rate limiting
- Integration tests: Complete flow testing (send money, buy/sell crypto, deposits/withdrawals)
- Critical paths: Language selection, registration, all menu options, error handling

## API Endpoints

### HTTP Endpoints (Africa's Talking Integration)

**POST /api/ussd** - USSD webhook handler
```
Input:
  sessionId: string          # Unique session from Africa's Talking
  phoneNumber: string        # User's phone number with country code
  text: string              # User's menu selections (USSD text input)

Output (USSD format):
  CON <message>             # Continue session (show menu)
  END <message>             # End session (final response)
```

### Canister Methods

**Update Methods** (require authorization):
- `set_user_canister_id(Principal)` - Configure user_canister ID
- `set_wallet_canister_id(Principal)` - Configure wallet_canister ID
- `set_crypto_canister_id(Principal)` - Configure crypto_canister ID
- `set_agent_canister_id(Principal)` - Configure agent_canister ID
- `configure_domain_canisters(...)` - Configure all four at once
- `http_request_update(HttpRequest)` - Handle POST requests (USSD, updates)

**Query Methods** (read-only):
- `http_request(HttpRequest)` - Handle GET requests
- `ussd(session_id, phone_number, text)` - Direct USSD call (for integration tests)

## Code Structure

```
canisters/ussd_canister/
├── Cargo.toml                    # Rust dependencies
├── src/
│   ├── lib.rs                   # Main API endpoints
│   ├── config_loader.rs         # Configuration from config.toml
│   ├── api/
│   │   ├── ussd.rs             # USSD webhook handler (Africa's Talking integration)
│   │   ├── http.rs             # HTTP request/response types and routing
│   │   └── mod.rs
│   ├── core/
│   │   ├── session.rs          # Session management (create, retrieve, cleanup)
│   │   ├── routing.rs          # Menu routing and navigation logic
│   │   └── mod.rs
│   ├── flows/                   # USSD flow handlers (organized by feature)
│   │   ├── local_currency/     # Send money, deposit, withdraw
│   │   ├── bitcoin/            # Buy, sell, send ckBTC
│   │   ├── usd/                # Buy, sell, send ckUSDC
│   │   ├── dao/                # DAO governance voting
│   │   ├── crypto/             # Token swaps
│   │   ├── common/             # Shared utilities (rates, agents, transactions)
│   │   └── mod.rs
│   ├── logic/                   # Pure business logic (testable, no async/IC calls)
│   │   ├── validation.rs       # Input validation
│   │   ├── menu_logic.rs       # Menu display and selection
│   │   ├── send_money_logic.rs # Send money parsing and validation
│   │   ├── crypto_logic.rs     # Crypto operation parsing
│   │   ├── agent_logic.rs      # Deposit/withdrawal parsing
│   │   └── mod.rs
│   ├── services/                # Domain canister client libraries (mockable)
│   │   ├── user_client.rs      # User canister calls
│   │   ├── wallet_client.rs    # Wallet canister calls
│   │   ├── crypto_client.rs    # Crypto canister calls
│   │   ├── agent_client.rs     # Agent canister calls
│   │   └── mod.rs
│   ├── utils/
│   │   ├── validation.rs       # Input sanitization and format validation
│   │   ├── rate_limit.rs       # Rate limiting logic
│   │   ├── translations.rs     # Multi-language translation service
│   │   ├── constants.rs        # Shared constants
│   │   ├── config.rs           # Africa's Talking credentials
│   │   └── mod.rs
│   └── config.toml             # Configuration file
│
├── tests/
│   ├── unit/                    # Unit tests (fast, isolated)
│   │   ├── session_tests.rs
│   │   ├── rate_limit_tests.rs
│   │   ├── validation_tests.rs
│   │   ├── request_validation/  # HTTP request parsing tests
│   │   └── mod.rs
│   └── integration/             # Integration tests (full flows with canister interactions)
│       ├── language_flow_tests.rs
│       ├── send_money_complete_tests.rs
│       ├── bitcoin_complete_tests.rs
│       ├── usdc_complete_tests.rs
│       ├── withdraw_complete_tests.rs
│       ├── dao_flow_tests.rs
│       └── ... (20 test files total)
│
├── README.md                    # This file
└── TEST_COVERAGE.md            # Detailed test documentation
```

## Security Considerations

### Input Security
- All USSD inputs sanitized before processing
- Phone numbers validated against 54 African country codes
- Amounts validated against min/max transaction limits
- Bitcoin addresses validated for format and checksum
- PINs validated for length and numeric format

### Session Security
- Session state stored in canister memory (not user-controlled)
- Expired sessions automatically cleaned up (5-minute timeout)
- PIN never stored in session (only verified via user_canister)
- Sensitive operations always require PIN verification

### Rate Limiting
- Per-phone-number rate limiting (configurable)
- Lazy cleanup prevents memory leaks
- Integration tests bypass limits via phone number prefix
- Playground mode bypasses limits for frontend testing

### Canister Coordination
- All domain canister calls routed through typed client libraries
- Pin verification required before sensitive operations
- User identifier resolution prevents impersonation
- Escrow system ensures atomic transactions

## Monitoring & Debugging

### Logging
The canister logs key events via `ic_cdk::println!`:
- Session creation/expiry
- Rate limiting events
- User registration
- Flow transitions
- Transaction success/failure
- Canister call results

```bash
# View canister logs
dfx canister logs ussd_canister

# View logs with filtering (requires deployed canister)
dfx canister logs ussd_canister | grep "🚫"  # Rate limit events
```

### Testing USSD Flows
Use the built-in `ussd()` method for direct testing:
```bash
# Test send money flow
dfx canister call ussd_canister ussd \
  "(\"session1\", \"+256700000001\", \"1*1*+256700000002*10000*1234\")"
```

## Troubleshooting

### Session Issues
- **Sessions expiring too quickly**: Check `session.timeout_minutes` in config.toml
- **Too many sessions in memory**: Check cleanup is running (every 10th request)
- **User losing data between requests**: Verify session_id is consistent from Africa's Talking

### Rate Limiting
- **Requests blocked for valid users**: Check `max_requests_per_minute` config
- **Test phone numbers still limited**: Ensure they start with `+254700`
- **Playground mode not working**: Check `playground.enabled = true` in config.toml

### Flow Issues
- **User stuck in flow**: Clear session via session timeout or restart session
- **Menu not appearing**: Check language translations are complete
- **Currency detection failing**: Verify phone number format (must have country code)
- **Canister calls failing**: Check domain canister IDs are configured via `configure_domain_canisters`

## Related Documentation

- **TESTING_STRATEGY.md** - Comprehensive testing architecture
- **TEST_COVERAGE.md** - Detailed test coverage and execution guide
- **tests/README.md** - Test structure and organization
- **tests/QUICK_REFERENCE.md** - Quick reference for common test patterns

## References

- **Internet Computer Docs**: https://internetcomputer.org/docs
- **Candid Reference**: https://internetcomputer.org/docs/current/references/candid-ref
- **IC CERES**: https://github.com/dfinity/ceres
- **Africa's Talking USSD**: https://africastalking.com/ussd
