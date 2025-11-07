# USSD Canister

Standalone canister that handles USSD webhook requests from Africa's Talking.

## Architecture

```
HTTP Request (Africa's Talking)
        ↓
USSD Canister (this canister)
        ↓
Juno Satellite (datastore via inter-canister calls)
        ↓
Response back to Africa's Talking
```

## Structure

```
src/
├── handlers/           # Request handlers
│   ├── http_handlers.rs   # HTTP routing
│   ├── ussd.rs            # USSD webhook handler
│   └── ussd_handlers.rs   # USSD menu logic
├── models/             # Data models
│   └── session.rs         # USSD session model
├── utils/              # Utilities
│   ├── config.rs          # Configuration
│   ├── datastore.rs       # Juno datastore client
│   ├── pin.rs             # PIN validation/hashing
│   └── translations.rs    # Multi-language support
└── lib.rs              # Canister entry point

tests/
├── pin_tests.rs        # PIN validation tests
└── session_tests.rs    # Session management tests
```

## Building

```bash
cargo build --target wasm32-unknown-unknown --release --package ussd_canister
```

## Testing

```bash
# Run unit tests
cargo test --package ussd_canister

# Run with output
cargo test --package ussd_canister -- --nocapture
```

## Deployment

```bash
# Deploy to local dfx
dfx deploy ussd_canister

# Deploy to IC mainnet
dfx deploy --network ic ussd_canister
```

## HTTP Endpoint

Once deployed, the canister exposes:
- `POST /api/ussd` - USSD webhook handler

Example request:
```json
{
  "sessionId": "session123",
  "serviceCode": "*229#",
  "phoneNumber": "+256700123456",
  "text": ""
}
```

## TODO

- [ ] Implement inter-canister calls to Juno satellite datastore
- [ ] Add comprehensive integration tests
- [ ] Add error handling and logging
- [ ] Implement session persistence
- [ ] Add rate limiting
- [ ] Add monitoring/metrics
