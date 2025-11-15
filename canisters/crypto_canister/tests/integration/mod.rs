use pocket_ic::PocketIc;
use candid::{encode_args, decode_one, Principal};

// Test modules
mod buy_sell_tests;
mod transfer_tests;
mod swap_tests;
mod escrow_tests;
mod cleanup_tests;
mod fraud_detection_tests;
mod slippage_tests;
mod error_sanitization_tests;
mod buy_sell_flow_tests;

/// Helper to create PocketIC instance with all required canisters
/// Returns: (PocketIc, data_canister, user_canister, wallet_canister, crypto_canister, ckbtc_ledger, ckusdc_ledger)
pub fn setup_test_environment() -> (PocketIc, Principal, Principal, Principal, Principal, Principal, Principal) {
    let pic = PocketIc::new();

    // Get workspace root (tests run from canister directory, need to go up 2 levels)
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let workspace_root = current_dir.parent().and_then(|p| p.parent())
        .expect("Failed to find workspace root");
    let wasm_dir = workspace_root.join("target/wasm32-unknown-unknown/release");

    // Create all canisters first to get their principals
    let data_canister = pic.create_canister();
    let user_canister = pic.create_canister();
    let wallet_canister = pic.create_canister();
    let crypto_canister = pic.create_canister();
    let ckbtc_ledger = pic.create_canister();
    let ckusdc_ledger = pic.create_canister();

    // Add cycles
    pic.add_cycles(data_canister, 100_000_000_000_000);
    pic.add_cycles(user_canister, 100_000_000_000_000);
    pic.add_cycles(wallet_canister, 100_000_000_000_000);
    pic.add_cycles(crypto_canister, 100_000_000_000_000);
    pic.add_cycles(ckbtc_ledger, 100_000_000_000_000);
    pic.add_cycles(ckusdc_ledger, 100_000_000_000_000);

    // Deploy data canister with ALL authorized canisters
    let data_wasm = std::fs::read(wasm_dir.join("data_canister.wasm"))
        .expect("Data canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release -p data_canister");
    // Authorize user_canister (for user operations) and crypto_canister (for crypto/escrow operations)
    // Note: wallet_canister calls through these, doesn't need direct access
    let data_init_args = encode_args((Some(user_canister.to_text()), Some(crypto_canister.to_text()))).unwrap();
    pic.install_canister(data_canister, data_wasm, data_init_args, None);

    // Deploy user canister
    let user_wasm = std::fs::read(wasm_dir.join("user_canister.wasm"))
        .expect("User canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release -p user_canister");
    pic.install_canister(user_canister, user_wasm, vec![], None);

    // Deploy wallet canister
    let wallet_wasm = std::fs::read(wasm_dir.join("wallet_canister.wasm"))
        .expect("Wallet canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release -p wallet_canister");
    pic.install_canister(wallet_canister, wallet_wasm, vec![], None);

    // Deploy crypto canister
    let crypto_wasm = std::fs::read(wasm_dir.join("crypto_canister.wasm"))
        .expect("Crypto canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release -p crypto_canister");
    pic.install_canister(crypto_canister, crypto_wasm, vec![], None);

    // Deploy mock ICRC-1 ledgers for ckBTC and ckUSDC
    let ledger_wasm = std::fs::read(wasm_dir.join("mock_icrc1_ledger.wasm"))
        .expect("Mock ledger WASM not found. Run: cargo build --target wasm32-unknown-unknown --release -p mock_icrc1_ledger");
    pic.install_canister(ckbtc_ledger, ledger_wasm.clone(), vec![], None);
    pic.install_canister(ckusdc_ledger, ledger_wasm, vec![], None);

    // Configure canisters
    configure_canisters(&pic, data_canister, user_canister, wallet_canister, crypto_canister, ckbtc_ledger, ckusdc_ledger);

    (pic, data_canister, user_canister, wallet_canister, crypto_canister, ckbtc_ledger, ckusdc_ledger)
}

/// Configure all canisters with their dependencies
fn configure_canisters(
    pic: &PocketIc,
    data_canister: Principal,
    user_canister: Principal,
    wallet_canister: Principal,
    crypto_canister: Principal,
    ckbtc_ledger: Principal,
    ckusdc_ledger: Principal,
) {
    // Configure user canister (takes Principal)
    let args = encode_args((data_canister,)).unwrap();
    pic.update_call(
        user_canister,
        Principal::anonymous(),
        "set_data_canister_id",
        args,
    ).expect("Failed to set data canister ID in user canister");
    
    // Configure wallet canister (takes Principal)
    let args = encode_args((data_canister,)).unwrap();
    pic.update_call(
        wallet_canister,
        Principal::anonymous(),
        "set_data_canister_id",
        args,
    ).expect("Failed to set data canister ID in wallet canister");
    
    let args = encode_args((user_canister,)).unwrap();
    pic.update_call(
        wallet_canister,
        Principal::anonymous(),
        "set_user_canister_id",
        args,
    ).expect("Failed to set user canister ID in wallet canister");
    
    // Configure crypto canister (takes Principal)
    let args = encode_args((data_canister,)).unwrap();
    pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "set_data_canister_id",
        args,
    ).expect("Failed to set data canister ID in crypto canister");
    
    let args = encode_args((user_canister,)).unwrap();
    pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "set_user_canister_id",
        args,
    ).expect("Failed to set user canister ID in crypto canister");
    
    let args = encode_args((wallet_canister,)).unwrap();
    pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "set_wallet_canister_id",
        args,
    ).expect("Failed to set wallet canister ID in crypto canister");
    
    // Authorize crypto_canister to call wallet_canister
    let args = encode_args((crypto_canister,)).unwrap();
    pic.update_call(
        wallet_canister,
        Principal::anonymous(),
        "add_authorized_canister",
        args,
    ).expect("Failed to authorize crypto canister in wallet canister");
    
    // Also authorize wallet_canister to call data_canister (for balance operations)
    let args = encode_args((wallet_canister.to_text(),)).unwrap();
    pic.update_call(
        data_canister,
        Principal::anonymous(),
        "add_authorized_canister",
        args,
    ).expect("Failed to authorize wallet canister in data canister");
    
    // Enable test mode on canisters that support it (bypasses authorization)
    pic.update_call(
        user_canister,
        Principal::anonymous(),
        "enable_test_mode",
        encode_args(()).unwrap(),
    ).expect("Failed to enable test mode on user canister");
    
    pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "enable_test_mode",
        encode_args(()).unwrap(),
    ).expect("Failed to enable test mode on crypto canister");

    // Configure ledger canister IDs in crypto canister
    let args = encode_args((ckbtc_ledger,)).unwrap();
    pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "set_ckbtc_ledger_id",
        args,
    ).expect("Failed to set ckBTC ledger ID in crypto canister");

    let args = encode_args((ckusdc_ledger,)).unwrap();
    pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "set_ckusdc_ledger_id",
        args,
    ).expect("Failed to set ckUSDC ledger ID in crypto canister");

    // Fund the crypto canister with tokens for testing
    // Crypto canister needs tokens to fulfill buy orders
    fund_ledger_account(pic, ckbtc_ledger, crypto_canister, 1_000_000_000_000); // 10,000 ckBTC (8 decimals)
    fund_ledger_account(pic, ckusdc_ledger, crypto_canister, 1_000_000_000_000); // 10,000 ckUSDC (6 decimals)
}

/// Fund a ledger account for testing
fn fund_ledger_account(
    pic: &PocketIc,
    ledger: Principal,
    account_owner: Principal,
    amount: u64,
) {
    #[derive(candid::CandidType)]
    struct Account {
        owner: Principal,
        subaccount: Option<Vec<u8>>,
    }

    // Fund the platform reserve subaccount [1, 0, 0, ..., 0]
    // This matches get_platform_reserve_subaccount() in ledger_client.rs
    let mut subaccount = vec![0u8; 32];
    subaccount[0] = 1;

    let account = Account {
        owner: account_owner,
        subaccount: Some(subaccount),
    };

    let args = encode_args((account, amount)).unwrap();
    pic.update_call(
        ledger,
        Principal::anonymous(),
        "set_balance_for_testing",
        args,
    ).expect("Failed to fund ledger account");
}

/// Helper to register a test user
/// Note: In test mode, we can call from anonymous
pub fn register_test_user(
    pic: &PocketIc,
    user_canister: Principal,
    phone: &str,
    pin: &str,
) -> String {
    use candid::CandidType;
    use serde::Deserialize;
    
    #[derive(CandidType, Deserialize)]
    struct RegisterUserRequest {
        phone_number: Option<String>,
        principal_id: Option<String>,
        pin: String,
        first_name: String,
        last_name: String,
        email: String,
        preferred_currency: String,
    }
    
    let request = RegisterUserRequest {
        phone_number: Some(phone.to_string()),
        principal_id: None,
        pin: pin.to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        preferred_currency: "KES".to_string(),
    };
    
    let args = encode_args((request,)).unwrap();
    let response = pic.update_call(
        user_canister,
        Principal::anonymous(),
        "register_user",
        args,
    ).expect("Failed to register user");
    
    let result: Result<String, String> = decode_one(&response).unwrap();
    result.expect("User registration failed")
}

/// Helper to set fiat balance for testing (calls data_canister directly)
pub fn set_fiat_balance(
    pic: &PocketIc,
    data_canister: Principal,
    user_id: &str,
    currency: &str,
    amount: u64,
) {
    let args = encode_args((user_id.to_string(), currency.to_string(), amount)).unwrap();
    let response = pic.update_call(
        data_canister,
        Principal::anonymous(),
        "set_fiat_balance",
        args,
    ).expect("Failed to set fiat balance");
    
    let result: Result<(), String> = decode_one(&response).unwrap();
    result.expect("Set fiat balance failed");
}

/// Helper to get crypto balance via crypto_canister
pub fn get_crypto_balance(
    pic: &PocketIc,
    crypto_canister: Principal,
    user_id: &str,
    crypto_type: &str,
) -> u64 {
    let args = encode_args((user_id.to_string(), crypto_type.to_string())).unwrap();
    let response = pic.update_call(
        crypto_canister,
        Principal::anonymous(),
        "check_crypto_balance",
        args,
    ).expect("Failed to check crypto balance");

    let (result,): (Result<u64, String>,) = candid::decode_args(&response)
        .expect("Failed to decode check_crypto_balance response");
    result.expect("Check crypto balance failed")
}

/// Helper to fund a user's ledger account for testing sell operations
/// After buying crypto, tests need to fund the user's actual ledger account
/// so they can transfer tokens back when selling
pub fn fund_user_ledger_account(
    pic: &PocketIc,
    ledger: Principal,
    user_principal: Principal,
    amount: u64,
) {
    #[derive(candid::CandidType)]
    struct Account {
        owner: Principal,
        subaccount: Option<Vec<u8>>,
    }

    let account = Account {
        owner: user_principal,
        subaccount: None,
    };

    let args = encode_args((account, amount)).unwrap();
    pic.update_call(
        ledger,
        Principal::anonymous(),
        "set_balance_for_testing",
        args,
    ).expect("Failed to fund user ledger account");
}
