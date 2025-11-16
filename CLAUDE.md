# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AfriTokeni is an SMS-accessible crypto banking platform for Africa, enabling instant Bitcoin (ckBTC) and stablecoin (ckUSDC) transfers via USSD (*229#) on any phone. Built 100% on Internet Computer Protocol (ICP) with SvelteKit frontend and Rust backend canisters.

**Key Features:**
- USSD banking interface (works on feature phones)
- Multi-currency support (39 African currencies)
- ckBTC and ckUSDC integration
- Agent network for cash deposits/withdrawals
- DAO governance with USSD voting
- Escrow system for secure transactions

## Development Commands

### Building & Running

```bash
# Install dependencies
pnpm install

# Start development server
pnpm run dev

# Build for production
pnpm run build

# Preview production build
pnpm run preview
```

### Rust Canister Development

```bash
# Build all Rust canisters to WASM
pnpm run canisters:build

# Generate Candid interfaces from WASM
pnpm run canisters:generate-candid

# Generate TypeScript bindings from Candid
pnpm run canisters:generate-ts

# Full pipeline: build → Candid → TS bindings
pnpm run canisters:generate

# Deploy canisters to local replica
pnpm run canisters:deploy
```

**IMPORTANT:** Always run `pnpm run canisters:generate` after modifying canister Rust code to keep TypeScript bindings synchronized.

### Testing

```bash
# Run all tests (unit + integration)
pnpm run test

# Unit tests only (fast, no blockchain)
pnpm run test:unit

# Integration tests (requires dfx replica)
pnpm run test:integration

# Test specific canister
pnpm run test:integration:business-logic
pnpm run test:integration:ussd
pnpm run test:integration:data

# Test coverage report
pnpm run test:coverage
```

**Rust canister tests:**
```bash
# Test a specific canister
cd canisters/<canister_name> && cargo test

# Test with release optimizations
cargo test --release
```

### DFX (Internet Computer CLI)

```bash
# Start local ICP replica
dfx start --clean --background

# Stop local replica
dfx stop

# Deploy all canisters
dfx deploy

# Deploy specific canister
dfx deploy data_canister

# Call canister function
dfx canister call <canister_name> <method_name> '(args)'

# Check canister status
dfx canister status <canister_name>

# Get canister ID
dfx canister id <canister_name>
```

### Code Quality

```bash
# Lint code
pnpm run lint

# Fix linting issues automatically
pnpm run lint:fix

# Format code with Prettier
pnpm run format

# Check formatting without modifying
pnpm run format:check

# Full validation (format + lint + typecheck)
pnpm run validate

# Pre-commit checks (format + lint + typecheck)
pnpm run precommit
```

## Architecture

### Current State (In Migration)

AfriTokeni is currently migrating from a monolithic architecture to a 4-domain canister system:

**OLD Architecture (Being Replaced):**
```
business_logic_canister (1.9M - 95% full) ⚠️
├── User management
├── Wallet operations
├── Agent operations
└── Crypto operations
```

**NEW Architecture (Target State):**
```
user_canister (~400KB)
├── User registration & authentication
├── PIN management
├── Profile management
└── Phone/Principal linking

wallet_canister (~600KB)
├── P2P fiat transfers
├── Balance queries
├── Transaction history
└── Fraud detection

agent_canister (~700KB)
├── Deposit operations (absorbed from deposit_canister)
├── Withdrawal operations (absorbed from withdrawal_canister)
├── Agent commission tracking
└── Monthly settlements

crypto_canister (~1.0M)
├── Buy/Sell ckBTC & ckUSDC
├── Crypto transfers
├── Token swaps (absorbed from exchange_canister)
├── Escrow management
└── DEX integration (Sonic)
```

**Unchanged Canisters:**
- `data_canister` - Pure storage layer (users, balances, transactions)
- `ussd_canister` - USSD interface & session management

### Migration Status

Check `CANISTER_MIGRATION_PLAN.md` for the complete migration guide. Key documents:
- `BUSINESS_LOGIC_ANALYSIS.md` - Endpoint breakdown
- `REVISED_ARCHITECTURE.md` - Architecture rationale
- `NEXT_SESSION_DETAILED_PROMPT.md` - Implementation guide

### Frontend Architecture

**Tech Stack:**
- SvelteKit 2.x (Static SPA)
- Svelte 5 with Runes
- TailwindCSS 4
- Vite 7
- TypeScript

**Project Structure:**
```
src/
├── lib/
│   ├── components/        # Reusable UI components
│   ├── services/          # Backend API clients
│   │   ├── userService.ts
│   │   ├── cryptoService.ts
│   │   └── agentService.ts
│   ├── stores/            # Svelte stores (state)
│   └── utils/             # Helper functions
├── routes/                # SvelteKit pages
└── satellite/             # Juno satellite functions (Rust)
```

**Data Flow:**
```
Juno Datastore → Service Layer → Svelte Stores → Components → UI
```

### Canister Structure

Each Rust canister follows this pattern:

```
canisters/<canister_name>/
├── Cargo.toml
├── <canister_name>.did      # Candid interface (auto-generated)
├── src/
│   ├── lib.rs               # Main API endpoints
│   ├── models/              # Data structures
│   ├── logic/               # Business logic
│   └── services/            # External canister clients
└── tests/
    ├── unit/                # Unit tests
    └── integration/         # Integration tests
```

## Key Development Patterns

### Canister Access Control

All domain canisters use a 3-tier access control system:

```rust
enum AccessLevel {
    Controller,           // Platform admin (dfx controller)
    AuthorizedCanister,   // USSD/Web canisters
    UserSelf(String),     // User accessing own data
    Unauthorized,
}
```

**Setup authorized canisters:**
```bash
dfx canister call user_canister add_authorized_canister "(\"$USSD_CANISTER_ID\")"
```

### Inter-Canister Communication

Canisters communicate using typed Candid interfaces:

```rust
// Example: wallet_canister calling user_canister
use ic_cdk::call;

let (user,): (Result<User, String>,) = call(
    user_canister_id,
    "get_user_by_phone",
    (phone_number,)
).await?;
```

### PIN Security

All sensitive operations require PIN verification:

```rust
// Always verify PIN before executing
let pin_valid = call(
    user_canister_id,
    "verify_pin",
    (user_identifier.clone(), pin.clone())
).await?;

if !pin_valid {
    return Err("Invalid PIN".to_string());
}
```

PINs are hashed with Argon2 before storage in data_canister.

### Data Storage

**Juno Datastore (Web Frontend):**
- Used for: User preferences, UI state, KYC documents
- Collections defined in `juno.config.ts`
- Access via `@junobuild/core` SDK

**Data Canister (Backend):**
- Used for: Balances, transactions, PINs, escrows
- Single source of truth for financial data
- Called by all domain canisters

### Testing Patterns

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test without external dependencies
    }
}
```

**Integration Tests:**
```rust
// tests/integration/feature_tests.rs
use pocket_ic::PocketIc;

#[test]
fn test_end_to_end_flow() {
    let pic = PocketIc::new();
    // Deploy canisters
    // Call canister methods
    // Assert outcomes
}
```

Run integration tests sequentially to avoid state conflicts:
```bash
cargo test --test lib -- --test-threads=1
```

## Common Development Tasks

### Adding a New Canister Endpoint

1. Define in Candid interface (`<canister>.did`)
2. Implement in `src/lib.rs` with `#[update]` or `#[query]` macro
3. Add business logic in `src/logic/` modules
4. Write unit tests in `src/lib.rs` or `tests/unit/`
5. Write integration tests in `tests/integration/`
6. Regenerate bindings: `pnpm run canisters:generate`
7. Update frontend service to call new endpoint

### Modifying USSD Flow

USSD flows are in `canisters/ussd_canister/src/flows/`:

```
flows/
├── local_currency/
│   ├── send_money.rs
│   ├── deposit.rs
│   └── withdraw.rs
├── bitcoin/
│   ├── buy.rs
│   ├── sell.rs
│   └── send.rs
└── usdc/
    ├── buy.rs
    ├── sell.rs
    └── send.rs
```

Each flow manages session state and calls domain canisters for business logic.

### Adding Multi-Language Support

USSD supports English, Luganda, and Swahili. Translations are in:
```
canisters/ussd_canister/src/translations/
├── mod.rs
├── english.rs
├── luganda.rs
└── swahili.rs
```

Add translations to all three files when adding new messages.

### Debugging Canister Issues

```bash
# View canister logs
dfx canister logs <canister_name>

# Get detailed canister info
dfx canister status <canister_name>

# Check cycles balance
dfx canister status <canister_name> | grep Balance

# Reinstall canister (WARNING: loses all data)
dfx canister install <canister_name> --mode reinstall
```

### Working with ckBTC/ckUSDC Ledgers

**Local Development:**
```bash
# Deploy local ledgers
dfx deploy ckbtc_ledger
dfx deploy ckusdc_ledger

# Get ledger canister IDs
dfx canister id ckbtc_ledger
dfx canister id ckusdc_ledger
```

**Mainnet Canister IDs:**
- ckBTC: `mxzaz-hqaaa-aaaar-qaada-cai`
- ckUSDC: `xevnm-gaaaa-aaaar-qafnq-cai`

Ledgers follow ICRC-1 standard. See `@dfinity/ledger-icrc` for SDK.

## Configuration Files

### `dfx.json`
Defines all ICP canisters, build settings, and network configurations.

### `juno.config.ts`
Juno satellite configuration for web hosting, datastore collections, and storage.

### `Cargo.toml` (workspace root)
Rust workspace configuration. Lists all canister members.

### `package.json`
Node.js dependencies and npm scripts for building, testing, and deployment.

### `svelte.config.js`
SvelteKit adapter configuration (static site generation).

### Revenue & Commission Config

Configuration files in `canisters/`:
- `revenue_config.toml` - Platform fees and commission rates
- `business_logic_config.toml` - Exchange rates and limits

## Deployment

### Local Development
```bash
dfx start --clean --background
dfx deploy
pnpm run dev
```

### Production Deployment

Production deployments are **release-based only**:

1. Merge changes to `main` branch
2. Create GitHub release with semantic version tag (e.g., `v1.0.0`)
3. CD pipeline automatically builds and deploys to production

**Production URLs:**
- Frontend: https://dkk74-oyaaa-aaaal-askxq-cai.icp0.io
- Custom domain: https://afritokeni.com
- Satellite ID: `dkk74-oyaaa-aaaal-askxq-cai`

**Manual Deploy (Emergency):**
Go to GitHub Actions → CD - Deploy to Production → Run workflow

## Important Notes

### Canister Size Limits
- Maximum WASM size: 2MB
- Always check after changes: `ls -lh target/wasm32-unknown-unknown/release/*.wasm`
- If approaching limit, consider splitting functionality

### Candid Interface Changes
When modifying canister interfaces:
1. Update the Rust code
2. Run `pnpm run canisters:generate` to regenerate `.did` files and TypeScript bindings
3. Update frontend services to match new interface
4. Update tests

### Git Pre-Commit Hooks
Automatically run on every commit:
- Gitleaks (secret detection)
- Prettier (formatting)
- ESLint (linting)
- svelte-check (type checking)

Install with: `pnpm install` (automatically runs `husky install`)

### Security Best Practices
- Never hardcode credentials (use Juno config collection)
- Always verify PINs before sensitive operations
- Use authorized canister lists for access control
- Hash PINs with Argon2 before storage
- Validate all inputs from USSD/Web

### SNS & DAO Governance

The project includes SNS (Service Nervous System) canisters for decentralized governance:
- `sns_governance` - Proposal voting
- `sns_ledger` - AFRI token ledger
- `sns_root` - SNS management
- `sns_swap` - Token distribution
- `sns_index` - Transaction indexing

Users can vote via USSD (*229*4#) or web dashboard.

## Troubleshooting

### Canister Won't Deploy
- Check WASM size: `ls -lh target/wasm32-unknown-unknown/release/<canister>.wasm`
- Verify Candid syntax: `didc check <canister>.did`
- Check dfx version: `dfx --version` (should be ≥0.14.0)

### Tests Failing
- Run integration tests sequentially: `-- --test-threads=1`
- Ensure dfx replica is running: `dfx ping`
- Clear state: `dfx start --clean --background`

### TypeScript Binding Errors
- Regenerate: `pnpm run canisters:generate`
- Check Candid is valid
- Ensure canister compiled successfully

### USSD Session Issues
- Sessions expire after 5 minutes
- Rate limit: 10 requests/minute per phone number
- Check session state in `ussd_canister` logs

## Resources

- **ICP Documentation:** https://internetcomputer.org/docs
- **Svelte Documentation:** https://svelte.dev/docs
- **Juno Documentation:** https://juno.build/docs
- **Candid Reference:** https://internetcomputer.org/docs/current/references/candid-ref
- **Project README:** See `README.md` for project overview and whitepaper link
