 AfriTokeni Data Canister

 This canister is a pure data store for AfriTokeni. It persists user profiles, fiat and crypto balances, PIN security metadata, transactions, and escrow records. All fraud detection and business rules live in the business_logic_canister.

 Overview

 - Purpose: Persist and serve application data for other canisters (USSD/Web/business logic).
 - Design: Stateless API over an internal state map (HashMaps), with stable upgrade hooks and a compact audit log.
 - Access model:
   - Controllers: full admin API (authorize/deauthorize canisters, list authorized canisters).
   - Authorized canisters: perform CRUD on users, balances, PIN metadata, transactions, escrows.
   - End users: may query their own data via specific endpoints (self-access).

 Repository Layout

 - Source: canisters/data_canister/src
 - Candid: canisters/data_canister/data_canister.did
 - Tests:
   - Unit: canisters/data_canister/tests/unit/*.rs
   - Integration (PocketIC): canisters/data_canister/tests/integration.rs

 Prerequisites

 - rustup target add wasm32-unknown-unknown
 - Cargo (Rust)
 - dfx (optional, for local replica)

 Build

 From repository root:
 - cargo build -p data_canister --target wasm32-unknown-unknown --release

 WASM output: target/wasm32-unknown-unknown/release/data_canister.wasm

 Test

 - First build the WASM (required for the integration test):
   - cargo build -p data_canister --target wasm32-unknown-unknown --release
 - Run tests:
   - cargo test -p data_canister

 PocketIC is used to install the compiled WASM and exercise create_user/get_user flows end-to-end.

 Deploy (optional with dfx)

 - dfx start --background
 - dfx deploy data_canister
 Initialize authorized canisters either via init args or by calling admin methods as the controller.

 Access control

 - Controller only:
   - add_authorized_canister, remove_authorized_canister, list_authorized_canisters
 - Authorized canisters:
   - create_user, set_fiat_balance, deposit_fiat, withdraw_fiat, transfer_fiat
   - setup_user_pin, verify_user_pin, reset_pin_attempts, change_pin
   - update_crypto_balance, set_crypto_balance (testing), store_transaction
   - store_escrow, get_user_by_phone, etc.
 - End users (self-access queries only):
   - get_my_user_data, get_my_balances, get_my_transactions

 Anonymous callers are not allowed in production.

 Interface examples (dfx)

 - Create user:
   - dfx canister call data_canister create_user '(record { user_type_str="User"; preferred_currency_str="UGX"; email="user@example.com"; first_name="Jane"; last_name="Doe"; principal_id=null; phone_number=opt "+256700000111" })'
 - Get user:
   - dfx canister call data_canister get_user '("user_001")'
 - Set fiat balance:
   - dfx canister call data_canister set_fiat_balance '("user_001", "UGX", 100000)'

 See data_canister.did for the full interface.

 Security

 - Data store only; business rules and fraud checks are outside this canister
 - PIN stored as salted hashes; failed attempts and lockout tracked
 - Audit log is size-bounded to prevent unbounded growth

 License

 See LICENSE at the repository root.