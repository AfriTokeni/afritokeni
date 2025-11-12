# Next Session: Canister Migration Implementation

## Context

We're splitting the business_logic_canister (1.9M - 95% full!) into 4 domain-driven canisters to solve the size limit issue and improve architecture.

## Current State

**Committed & Pushed to `feature/bdd-tests` branch:**
- ✅ Architecture analysis complete
- ✅ Migration plan documented
- ✅ All documentation in place
- ✅ dfx.json updated with new canister definitions

**Key Documents:**
- `BUSINESS_LOGIC_ANALYSIS.md` - Endpoint breakdown
- `CANISTER_MIGRATION_PLAN.md` - Complete implementation guide
- `REVISED_ARCHITECTURE.md` - Architecture overview

## Your Task

Implement the 4-domain canister architecture following the migration plan.

## Architecture Overview

```
OLD (1 fat canister):
business_logic_canister (1.9M) ⚠️

NEW (4 domain canisters):
├── user_canister (400KB) - Identity & auth
├── wallet_canister (600KB) - P2P fiat transfers
├── agent_canister (700KB) - Deposits + withdrawals + commissions
└── crypto_canister (1.0M) - All crypto + DEX integration
```

## Implementation Order

### Phase 1: user_canister (Start Here)
**Priority:** HIGHEST - Other canisters depend on this

**What to do:**
1. Create `canisters/user_canister/` directory structure
2. Create `Cargo.toml` with dependencies
3. Create `user_canister.did` (Candid interface)
4. Implement `src/lib.rs` with these endpoints:
   - `register_user()`
   - `user_exists()`
   - `verify_pin()`
   - `change_pin()`
   - `link_phone_to_account()`
   - `get_user_profile()`
   - `update_user_profile()`
   - `get_user_by_phone()`
   - `get_user_by_principal()`

**Code to move:**
- From `business_logic_canister/src/logic/user_logic.rs` (238 lines)
- User functions from `business_logic_canister/src/services/data_client.rs`

**Tests to create:**
- `tests/integration/registration_tests.rs`
- `tests/integration/authentication_tests.rs`
- `tests/integration/profile_tests.rs`

**Success criteria:**
- ✅ Compiles without errors
- ✅ All user tests pass
- ✅ WASM size < 500KB
- ✅ Can be called by other canisters

### Phase 2: wallet_canister
**Priority:** HIGH - Core money operations

**What to do:**
1. Create `canisters/wallet_canister/` directory
2. Implement P2P fiat transfer logic
3. Include fraud detection
4. Add balance queries

**Code to move:**
- `business_logic_canister/src/logic/transfer_logic.rs` (320 lines)
- `business_logic_canister/src/logic/fraud_logic.rs` (243 lines)
- `business_logic_canister/src/services/fraud_detection.rs` (221 lines)

**Dependencies:**
- Calls: data_canister, user_canister

### Phase 3: agent_canister
**Priority:** HIGH - Cash in/out operations

**What to do:**
1. Create `canisters/agent_canister/` directory
2. Merge deposit_canister code
3. Merge withdrawal_canister code
4. Create unified agent management

**Code to move:**
- **ENTIRE** `deposit_canister/src/` (~1,258 lines)
- **ENTIRE** `withdrawal_canister/src/` (~838 lines)
- Commission functions from `business_logic_canister/src/services/commission_client.rs`

**Key feature:** Unified agent balance tracking (no more duplicates!)

### Phase 4: crypto_canister
**Priority:** MEDIUM - Crypto operations

**What to do:**
1. Create `canisters/crypto_canister/` directory
2. Implement buy/sell logic
3. Merge exchange_canister for swaps
4. Add escrow operations

**Code to move:**
- `business_logic_canister/src/logic/crypto_logic.rs` (288 lines)
- `business_logic_canister/src/services/crypto_operations.rs` (440 lines)
- **ENTIRE** `exchange_canister/src/` (~1,164 lines)

**Dependencies:**
- Calls: data_canister, user_canister, wallet_canister, Sonic DEX

### Phase 5: Update Clients
**Priority:** HIGH - Make everything work together

**What to do:**
1. Update USSD canister to call new canisters
2. Update Web frontend services
3. Update all integration tests

**Files to update:**
- `canisters/ussd_canister/src/services/` - Create new client modules
- `src/lib/services/` - Update web services
- `canisters/ussd_canister/tests/integration/` - Update test setup

### Phase 6: Deploy & Test
**Priority:** CRITICAL - Verify everything works

**What to do:**
1. Deploy all canisters locally
2. Configure canister IDs
3. Run integration tests
4. Verify WASM sizes
5. Test critical user flows

## Detailed Instructions

### Step 1: Create user_canister

```bash
# Create directory structure
mkdir -p canisters/user_canister/src/logic
mkdir -p canisters/user_canister/src/services
mkdir -p canisters/user_canister/tests/integration
```

**Create `canisters/user_canister/Cargo.toml`:**
```toml
[package]
name = "user_canister"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.10"
ic-cdk = "0.13"
ic-cdk-macros = "0.13"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
shared_types = { path = "../shared_types" }

[dev-dependencies]
pocket-ic = "4.0"
```

**Create `canisters/user_canister/user_canister.did`:**
```candid
type RegisterUserRequest = record {
    phone_number : opt text;
    principal_id : opt text;
    first_name : text;
    last_name : text;
    email : text;
    pin : text;
    preferred_currency : text;
};

type UserProfile = record {
    phone_number : text;
    principal_id : text;
    first_name : text;
    last_name : text;
    email : text;
    preferred_currency : text;
    kyc_status : text;
    created_at : nat64;
};

service : {
    // Registration & Management
    register_user : (RegisterUserRequest) -> (variant { Ok : text; Err : text });
    user_exists : (text) -> (variant { Ok : bool; Err : text });
    
    // Authentication
    verify_pin : (text, text) -> (variant { Ok : bool; Err : text });
    change_pin : (text, text, text) -> (variant { Ok : null; Err : text });
    
    // Profile
    get_user_profile : (text) -> (variant { Ok : UserProfile; Err : text }) query;
    
    // Linking
    link_phone_to_account : (text, text) -> (variant { Ok : null; Err : text });
    
    // Configuration
    set_data_canister_id : (text) -> (variant { Ok : null; Err : text });
    add_authorized_canister : (text) -> (variant { Ok : null; Err : text });
    enable_test_mode : () -> (variant { Ok : null; Err : text });
}
```

**Create `canisters/user_canister/src/lib.rs`:**
```rust
use candid::Principal;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;

// Import shared types
use shared_types::RegisterUserRequest;

// State management
thread_local! {
    static DATA_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static TEST_MODE: RefCell<bool> = RefCell::new(false);
}

// Access control
fn verify_authorized_caller() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    }
    
    let has_authorized = AUTHORIZED_CANISTERS.with(|c| !c.borrow().is_empty());
    
    if !has_authorized {
        return Ok(());
    }
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        if canisters.borrow().contains(&caller) {
            Ok(())
        } else {
            Err("Unauthorized caller".to_string())
        }
    })
}

// Configuration
#[update]
fn set_data_canister_id(canister_id: String) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can set canister IDs".to_string());
    }
    
    let principal = Principal::from_text(canister_id)
        .map_err(|e| format!("Invalid principal: {}", e))?;
    
    DATA_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(principal);
    });
    
    Ok(())
}

#[update]
fn add_authorized_canister(canister_id: String) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can authorize canisters".to_string());
    }
    
    let principal = Principal::from_text(canister_id)
        .map_err(|e| format!("Invalid principal: {}", e))?;
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow_mut().push(principal);
    });
    
    Ok(())
}

#[update]
fn enable_test_mode() -> Result<(), String> {
    TEST_MODE.with(|mode| {
        *mode.borrow_mut() = true;
    });
    Ok(())
}

// User Management Endpoints
#[update]
async fn register_user(request: RegisterUserRequest) -> Result<String, String> {
    verify_authorized_caller()?;
    
    // TODO: Implement registration logic
    // 1. Validate input
    // 2. Call data_canister to create user
    // 3. Return user ID
    
    Err("Not implemented yet".to_string())
}

#[update]
async fn user_exists(user_identifier: String) -> Result<bool, String> {
    verify_authorized_caller()?;
    
    // TODO: Check if user exists in data_canister
    
    Err("Not implemented yet".to_string())
}

#[update]
async fn verify_pin(user_identifier: String, pin: String) -> Result<bool, String> {
    verify_authorized_caller()?;
    
    // TODO: Verify PIN via data_canister
    
    Err("Not implemented yet".to_string())
}

// Add remaining endpoints...
```

**Copy logic from business_logic_canister:**
```bash
# Copy user logic
cp canisters/business_logic_canister/src/logic/user_logic.rs \
   canisters/user_canister/src/logic/

# Extract user-related functions from data_client.rs
# (You'll need to manually extract these)
```

### Step 2: Test user_canister

**Create `canisters/user_canister/tests/integration/mod.rs`:**
```rust
use pocket_ic::PocketIc;
use candid::Principal;

pub struct TestEnv {
    pic: PocketIc,
    user_canister: Principal,
    data_canister: Principal,
}

impl TestEnv {
    pub fn new() -> Self {
        let pic = PocketIc::new();
        
        // Deploy data_canister
        let data_canister = pic.create_canister();
        pic.add_cycles(data_canister, 2_000_000_000_000);
        let data_wasm = std::fs::read("../data_canister/target/wasm32-unknown-unknown/release/data_canister.wasm")
            .expect("Data canister WASM not found");
        pic.install_canister(data_canister, data_wasm, vec![], None);
        
        // Deploy user_canister
        let user_canister = pic.create_canister();
        pic.add_cycles(user_canister, 2_000_000_000_000);
        let user_wasm = std::fs::read("target/wasm32-unknown-unknown/release/user_canister.wasm")
            .expect("User canister WASM not found");
        pic.install_canister(user_canister, user_wasm, vec![], None);
        
        // Configure
        pic.update_call(
            user_canister,
            Principal::anonymous(),
            "set_data_canister_id",
            candid::encode_one(data_canister.to_text()).unwrap(),
        ).expect("Failed to set data canister ID");
        
        pic.update_call(
            user_canister,
            Principal::anonymous(),
            "enable_test_mode",
            candid::encode_one(()).unwrap(),
        ).expect("Failed to enable test mode");
        
        Self { pic, user_canister, data_canister }
    }
    
    pub fn register_user(&self, phone: &str, first_name: &str, last_name: &str, email: &str, pin: &str) -> Result<String, String> {
        // TODO: Implement
        Ok("user_id".to_string())
    }
}

#[test]
fn test_user_registration() {
    let env = TestEnv::new();
    
    let result = env.register_user(
        "+256700123456",
        "John",
        "Doe",
        "john@test.com",
        "1234"
    );
    
    assert!(result.is_ok(), "Registration should succeed");
}
```

### Step 3: Build & Test

```bash
# Build user_canister
cd canisters/user_canister
cargo build --target wasm32-unknown-unknown --release

# Check size
ls -lh target/wasm32-unknown-unknown/release/user_canister.wasm
# Should be < 500KB

# Run tests
cargo test

# If tests pass, move to next canister
```

## Important Notes

### Code Movement Guidelines
1. **Don't copy-paste blindly** - Understand what each function does
2. **Update imports** - Change `crate::services::data_client` to local paths
3. **Remove dependencies** - Each canister should only depend on what it needs
4. **Test incrementally** - Build and test after each major change

### Common Pitfalls
1. **Circular dependencies** - wallet_canister shouldn't call crypto_canister
2. **Missing authorization** - Always verify caller in update methods
3. **Hardcoded canister IDs** - Use configuration, not hardcoded values
4. **Duplicate code** - Extract common logic to shared_types

### Testing Strategy
1. **Unit tests** - Test each canister independently
2. **Integration tests** - Test inter-canister communication
3. **End-to-end tests** - Test full user flows via USSD

## Success Criteria

### Per Canister
- ✅ Compiles without errors
- ✅ All tests pass
- ✅ WASM size within limits
- ✅ Candid interface correct
- ✅ Can be called by authorized canisters

### Overall
- ✅ All 4 canisters deployed
- ✅ USSD tests pass (312 tests)
- ✅ Critical flows work end-to-end
- ✅ No data loss
- ✅ Performance acceptable

## Timeline

**Estimated: 3-4 days**
- Day 1: user_canister + wallet_canister
- Day 2: agent_canister + crypto_canister
- Day 3: Update clients + integration
- Day 4: Testing + deployment

## Questions?

If you get stuck:
1. Check `CANISTER_MIGRATION_PLAN.md` for details
2. Look at existing canister code for patterns
3. Test incrementally - don't try to do everything at once
4. Ask for help if needed!

## Ready?

Start with **Phase 1: user_canister**. Good luck! 🚀
