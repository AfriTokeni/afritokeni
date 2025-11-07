# E2E Tests for USSD Satellite

Real end-to-end tests that call the actual Rust satellite canister's USSD webhook endpoint.

## Architecture

```
Cucumber Feature Files
        ↓
TypeScript Step Definitions
        ↓
HTTP POST to /api/ussd
        ↓
Satellite Canister (Rust)
        ↓
Real USSD Handlers
        ↓
Response Assertions
```

## Setup

### 1. Start Juno Emulator (Docker)

```bash
# Start the Docker-based emulator with predictable satellite ID
pnpm juno:emulator:start
```

This starts the `junobuild/satellite` Docker image with satellite ID: `jx5yt-yyaaa-aaaal-abzbq-cai`

### 2. Build and Deploy Satellite

```bash
# Build the satellite WASM
cd src/satellite
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Deploy to emulator
pnpm juno:emulator:deploy
```

### 3. Run E2E Tests

```bash
# Run all E2E tests against emulator
pnpm test:e2e

# Or specify custom satellite URL
SATELLITE_URL=http://jx5yt-yyaaa-aaaal-abzbq-cai.localhost:5987 pnpm test:e2e
```

## Test Against Different Environments

### Local Development Satellite
```bash
SATELLITE_URL=http://atbka-rp777-77775-aaaaq-cai.localhost:5987 pnpm test:e2e
```

### Preview Satellite
```bash
SATELLITE_URL=https://64njw-oiaaa-aaaal-asppa-cai.icp0.io pnpm test:e2e
```

### Production Satellite
```bash
SATELLITE_URL=https://dkk74-oyaaa-aaaal-askxq-cai.icp0.io pnpm test:e2e
```

## Writing Tests

Feature files are in `tests/e2e/features/`. They use Gherkin syntax:

```gherkin
Feature: USSD Bitcoin Operations
  
  Scenario: Access Bitcoin menu
    Given I have a phone number "+256700123456"
    And I have set my PIN to "1234"
    When I dial "*229#"
    And I select "2" for Bitcoin
    Then I should see "Bitcoin (ckBTC)" in USSD response
```

Step definitions are in `tests/e2e/steps/`:
- `given.ts` - Setup steps
- `when.ts` - Action steps (calls satellite)
- `then.ts` - Assertion steps
- `world.ts` - Test context and HTTP client

## CI/CD

In GitHub Actions, use the Docker emulator:

```yaml
- name: Start Juno Emulator
  run: pnpm juno:emulator:start

- name: Deploy Satellite
  run: pnpm juno:emulator:deploy

- name: Run E2E Tests
  run: pnpm test:e2e
```

## Troubleshooting

### 400 Error: Canister ID not resolved
- Make sure the emulator is running: `pnpm juno:emulator:start`
- Check the satellite is deployed: `pnpm juno:emulator:deploy`
- Verify the satellite ID matches in `juno.config.ts` and `world.ts`

### Connection Refused
- Emulator not started
- Wrong port (should be 5987)
- Docker not running

### Tests Timing Out
- Satellite build failed
- USSD handler not deployed
- Check satellite logs: `docker logs <container-id>`
