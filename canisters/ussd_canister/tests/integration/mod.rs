// USSD Canister Integration Tests
use candid::{encode_one, encode_args, decode_one, Principal, CandidType, Deserialize};
use pocket_ic::PocketIc;
use shared_types::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// User profile response from user_canister
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,
    pub kyc_status: String,
    pub created_at: u64,
    pub last_active: u64,
}

lazy_static! {
    /// Shared test environment - created once and reused across all tests
    /// This dramatically speeds up test execution by avoiding repeated canister deployments
    static ref SHARED_ENV: Mutex<TestEnv> = Mutex::new(TestEnv::new());
}

// Integration test modules - test real canister interactions
pub mod crypto_swap_integration_tests;
pub mod registration_flow_tests;
pub mod send_money_flow_tests;
// pub mod send_money_complete_tests; // ALL send money combinations (28 tests) - TEMPORARILY DISABLED
pub mod bitcoin_flow_tests;
pub mod bitcoin_complete_tests; // ALL Bitcoin combinations (30 tests)
pub mod usdc_flow_tests;
pub mod usdc_complete_tests; // ALL USDC combinations (30 tests)
pub mod crypto_swap_complete_tests; // ALL swap combinations (25 tests)
pub mod balance_check_tests;
pub mod balance_complete_tests; // ALL balance combinations (30 tests)
pub mod withdraw_flow_tests;
pub mod withdraw_complete_tests; // ALL withdrawal combinations (30 tests)
pub mod dao_flow_tests;
pub mod language_flow_tests;
pub mod main_menu_tests;
pub mod stateless_ussd_tests;
pub mod error_security_tests;

/// Get the shared test environment
/// This allows all tests to reuse the same canister deployments
/// Handles poisoned mutex by recovering the inner value
pub fn get_test_env() -> std::sync::MutexGuard<'static, TestEnv> {
    SHARED_ENV.lock().unwrap_or_else(|poisoned| {
        // If the mutex is poisoned (previous test panicked), recover it
        poisoned.into_inner()
    })
}

// ============================================================================
// Phone Number Generator - Ensures unique phones per test
// ============================================================================

/// Generate a unique phone number based on test name
/// Usage: let phone = &phone("UGX");
/// Returns "+256700XXXXXX" where X is unique per test based on test name hash
pub fn phone(country_or_currency: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Get test name from thread
    let test_name = std::thread::current()
        .name()
        .unwrap_or("unknown")
        .to_string();
    
    // Hash test name to get unique 6-digit suffix
    let mut hasher = DefaultHasher::new();
    test_name.hash(&mut hasher);
    let hash = hasher.finish();
    let suffix = format!("{:06}", hash % 1_000_000);
    
    // Map currency/country to country code
    let country_code = match country_or_currency {
        "UGX" | "Uganda" | "256" => "256",
        "KES" | "Kenya" | "254" => "254",
        "TZS" | "Tanzania" | "255" => "255",
        "RWF" | "Rwanda" | "250" => "250",
        "NGN" | "Nigeria" | "234" => "234",
        "GHS" | "Ghana" | "233" => "233",
        "ZAR" | "SouthAfrica" | "27" => "27",
        _ => "256", // Default to Uganda
    };
    
    format!("+{}7{}", country_code, suffix)
}

/// Generate a unique session ID based on test name
/// Usage: let session = &session();
/// Returns "session_XXXXXX" where X is unique per test
pub fn session() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let test_name = std::thread::current()
        .name()
        .unwrap_or("unknown")
        .to_string();
    
    let mut hasher = DefaultHasher::new();
    test_name.hash(&mut hasher);
    let hash = hasher.finish();
    
    format!("session_{:06}", hash % 1_000_000)
}

// ============================================================================
// Test Environment Setup
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
    pub ussd_canister_id: Principal,
    pub user_canister_id: Principal,
    pub wallet_canister_id: Principal,
    pub crypto_canister_id: Principal,
    pub agent_canister_id: Principal,
    pub data_canister_id: Principal,
    pub ckbtc_ledger_id: Principal,
    pub ckusdc_ledger_id: Principal,
    // Keep for backward compatibility during migration
    #[allow(dead_code)]
    pub business_logic_canister_id: Principal,
}

impl TestEnv {
    pub fn new() -> Self {
        let pic = PocketIc::new();
        
        let workspace_root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf();
        
        // Load WASMs
        let ussd_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/ussd_canister.wasm")
        ).expect("ussd_canister WASM not found");
        
        let user_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/user_canister.wasm")
        ).expect("user_canister WASM not found");
        
        let wallet_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/wallet_canister.wasm")
        ).expect("wallet_canister WASM not found");
        
        let crypto_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/crypto_canister.wasm")
        ).expect("crypto_canister WASM not found");
        
        let agent_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/agent_canister.wasm")
        ).expect("agent_canister WASM not found");
        
        let data_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
        ).expect("data_canister WASM not found");
        
        let mock_ledger_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/mock_icrc1_ledger.wasm")
        ).expect("mock_icrc1_ledger WASM not found");
        
        // Install data canister first
        let data_canister_id = pic.create_canister();
        pic.add_cycles(data_canister_id, 2_000_000_000_000);
        let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);
        
        // Install user canister
        let user_canister_id = pic.create_canister();
        pic.add_cycles(user_canister_id, 2_000_000_000_000);
        pic.install_canister(user_canister_id, user_wasm, vec![], None);
        
        // Install wallet canister
        let wallet_canister_id = pic.create_canister();
        pic.add_cycles(wallet_canister_id, 2_000_000_000_000);
        pic.install_canister(wallet_canister_id, wallet_wasm, vec![], None);
        
        // Install crypto canister
        let crypto_canister_id = pic.create_canister();
        pic.add_cycles(crypto_canister_id, 2_000_000_000_000);
        pic.install_canister(crypto_canister_id, crypto_wasm, vec![], None);
        
        // Install agent canister
        let agent_canister_id = pic.create_canister();
        pic.add_cycles(agent_canister_id, 2_000_000_000_000);
        pic.install_canister(agent_canister_id, agent_wasm, vec![], None);
        
        // Install mock ckBTC ledger
        let ckbtc_ledger_id = pic.create_canister();
        pic.add_cycles(ckbtc_ledger_id, 2_000_000_000_000);
        pic.install_canister(ckbtc_ledger_id, mock_ledger_wasm.clone(), vec![], None);
        
        // Install mock ckUSDC ledger
        let ckusdc_ledger_id = pic.create_canister();
        pic.add_cycles(ckusdc_ledger_id, 2_000_000_000_000);
        pic.install_canister(ckusdc_ledger_id, mock_ledger_wasm, vec![], None);
        
        // Configure domain canisters with data canister ID (as Principal)
        let data_id_arg = encode_one(data_canister_id).unwrap();
        pic.update_call(user_canister_id, Principal::anonymous(), "set_data_canister_id", data_id_arg.clone()).expect("Failed to set data canister ID on user_canister");
        pic.update_call(wallet_canister_id, Principal::anonymous(), "set_data_canister_id", data_id_arg.clone()).expect("Failed to set data canister ID on wallet_canister");
        pic.update_call(crypto_canister_id, Principal::anonymous(), "set_data_canister_id", data_id_arg.clone()).expect("Failed to set data canister ID on crypto_canister");
        pic.update_call(agent_canister_id, Principal::anonymous(), "set_data_canister_id", data_id_arg).expect("Failed to set data canister ID on agent_canister");
        
        // Configure inter-canister dependencies (as Principal)
        let user_id_arg = encode_one(user_canister_id).unwrap();
        pic.update_call(wallet_canister_id, Principal::anonymous(), "set_user_canister_id", user_id_arg.clone()).expect("Failed to set user canister ID on wallet_canister");
        pic.update_call(crypto_canister_id, Principal::anonymous(), "set_user_canister_id", user_id_arg.clone()).expect("Failed to set user canister ID on crypto_canister");
        pic.update_call(agent_canister_id, Principal::anonymous(), "set_user_canister_id", user_id_arg).expect("Failed to set user canister ID on agent_canister");
        
        let wallet_id_arg = encode_one(wallet_canister_id).unwrap();
        pic.update_call(crypto_canister_id, Principal::anonymous(), "set_wallet_canister_id", wallet_id_arg.clone()).expect("Failed to set wallet canister ID on crypto_canister");
        pic.update_call(agent_canister_id, Principal::anonymous(), "set_wallet_canister_id", wallet_id_arg).expect("Failed to set wallet canister ID on agent_canister");
        
        // Install USSD canister
        let ussd_canister_id = pic.create_canister();
        pic.add_cycles(ussd_canister_id, 2_000_000_000_000);
        pic.install_canister(ussd_canister_id, ussd_wasm, vec![], None);
        
        // Configure USSD canister with domain canister IDs (as Principal)
        pic.update_call(ussd_canister_id, Principal::anonymous(), "set_user_canister_id", encode_one(user_canister_id).unwrap()).expect("Failed to set user canister ID on USSD");
        pic.update_call(ussd_canister_id, Principal::anonymous(), "set_wallet_canister_id", encode_one(wallet_canister_id).unwrap()).expect("Failed to set wallet canister ID on USSD");
        pic.update_call(ussd_canister_id, Principal::anonymous(), "set_crypto_canister_id", encode_one(crypto_canister_id).unwrap()).expect("Failed to set crypto canister ID on USSD");
        pic.update_call(ussd_canister_id, Principal::anonymous(), "set_agent_canister_id", encode_one(agent_canister_id).unwrap()).expect("Failed to set agent canister ID on USSD");
        
        // Enable test mode on all domain canisters (bypasses authorization checks)
        pic.update_call(user_canister_id, Principal::anonymous(), "enable_test_mode", encode_args(()).unwrap()).expect("Failed to enable test mode on user_canister");
        pic.update_call(wallet_canister_id, Principal::anonymous(), "enable_test_mode", encode_args(()).unwrap()).expect("Failed to enable test mode on wallet_canister");
        pic.update_call(crypto_canister_id, Principal::anonymous(), "enable_test_mode", encode_args(()).unwrap()).expect("Failed to enable test mode on crypto_canister");
        pic.update_call(agent_canister_id, Principal::anonymous(), "enable_test_mode", encode_args(()).unwrap()).expect("Failed to enable test mode on agent_canister");
        
        // Configure crypto canister with mock ledger IDs
        pic.update_call(crypto_canister_id, Principal::anonymous(), "set_ckbtc_ledger_id", encode_one(ckbtc_ledger_id).unwrap()).expect("Failed to set ckBTC ledger ID");
        pic.update_call(crypto_canister_id, Principal::anonymous(), "set_ckusdc_ledger_id", encode_one(ckusdc_ledger_id).unwrap()).expect("Failed to set ckUSDC ledger ID");
        
        // Fund platform reserve accounts in mock ledgers (100 BTC and 10M USDC)
        // Platform reserve uses subaccount [1, 0, 0, ..., 0]
        #[derive(CandidType, Deserialize, Clone)]
        struct Account { owner: Principal, subaccount: Option<Vec<u8>> }
        let mut reserve_subaccount = vec![0u8; 32];
        reserve_subaccount[0] = 1;
        let reserve_account = Account {
            owner: crypto_canister_id,
            subaccount: Some(reserve_subaccount),
        };
        let btc_reserve_balance = 10_000_000_000u64; // 100 BTC in satoshis
        let usdc_reserve_balance = 1_000_000_000_000u64; // 10M USDC in cents
        pic.update_call(ckbtc_ledger_id, Principal::anonymous(), "set_balance_for_testing", encode_args((reserve_account.clone(), btc_reserve_balance)).unwrap()).expect("Failed to fund ckBTC reserve");
        pic.update_call(ckusdc_ledger_id, Principal::anonymous(), "set_balance_for_testing", encode_args((reserve_account, usdc_reserve_balance)).unwrap()).expect("Failed to fund ckUSDC reserve");
        
        // Authorize domain canisters to call data_canister
        pic.update_call(data_canister_id, Principal::anonymous(), "add_authorized_canister", encode_one(user_canister_id.to_text()).unwrap()).ok();
        pic.update_call(data_canister_id, Principal::anonymous(), "add_authorized_canister", encode_one(wallet_canister_id.to_text()).unwrap()).ok();
        pic.update_call(data_canister_id, Principal::anonymous(), "add_authorized_canister", encode_one(crypto_canister_id.to_text()).unwrap()).ok();
        pic.update_call(data_canister_id, Principal::anonymous(), "add_authorized_canister", encode_one(agent_canister_id.to_text()).unwrap()).ok();
        
        Self {
            pic,
            ussd_canister_id,
            user_canister_id,
            wallet_canister_id,
            crypto_canister_id,
            agent_canister_id,
            data_canister_id,
            ckbtc_ledger_id,
            ckusdc_ledger_id,
            business_logic_canister_id: user_canister_id, // Backward compatibility
        }
    }
    
    /// Process USSD request
    pub fn process_ussd(&self, session_id: &str, phone_number: &str, text: &str) -> (String, bool) {
        let arg = encode_args((session_id, phone_number, text)).unwrap();
        
        let response = self.pic.update_call(
            self.ussd_canister_id,
            Principal::anonymous(),
            "ussd",
            arg,
        ).expect("ussd call failed");
        
        candid::decode_args(&response).expect("Failed to decode")
    }
    
    /// Get exchange spread (stub - exchange canister doesn't exist)
    /// Returns a default spread for testing
    pub fn get_exchange_spread(&self) -> u64 {
        50 // 0.5% default spread
    }
    
    /// Register user via user canister (for test setup)
    /// Idempotent - returns existing user_id if user already registered
    pub fn register_user_direct(
        &self,
        phone_number: &str,
        first_name: &str,
        last_name: &str,
        email: &str,
        preferred_currency: &str,
        pin: &str,
    ) -> Result<String, String> {
        // Check if user already exists (for shared test environment)
        if let Ok(Some(existing_user)) = self.get_user(phone_number) {
            // User exists, return their ID
            return Ok(existing_user.id);
        }
        
        // Generate a unique principal ID for each test user
        // Use a deterministic principal based on phone number for consistency
        let mut bytes = [0u8; 29];
        let phone_string = format!("test-user-{}", phone_number);
        let phone_bytes = phone_string.as_bytes();
        let len = phone_bytes.len().min(29);
        bytes[..len].copy_from_slice(&phone_bytes[..len]);
        let principal_id = Principal::from_slice(&bytes);
        
        let request = RegisterUserRequest {
            phone_number: Some(phone_number.to_string()),
            principal_id: Some(principal_id.to_text()),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            preferred_currency: preferred_currency.to_string(),
            pin: pin.to_string(),
        };
        
        let arg = encode_one(request).unwrap();
        let response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "register_user",
            arg,
        ).expect("register_user call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Set fiat balance for testing (accepts phone number or user_id)
    /// Amount is in CENTS (e.g., 150_000 cents = 1,500 UGX)
    pub fn set_fiat_balance(&self, user_identifier: &str, currency: &str, amount_in_cents: u64) -> Result<(), String> {
        // If it's a phone number, look up the user_id first
        let user_id = if user_identifier.starts_with('+') {
            match self.get_user(user_identifier) {
                Ok(Some(user)) => user.id,
                Ok(None) => return Err(format!("User not found: {}", user_identifier)),
                Err(e) => return Err(e),
            }
        } else {
            user_identifier.to_string()
        };
        
        // Convert currency string to FiatCurrency enum
        let currency_enum = FiatCurrency::from_code(currency)
            .ok_or_else(|| format!("Invalid currency code: {}", currency))?;
        
        // Call wallet_canister (not data_canister directly - respect architecture!)
        let arg = encode_args((user_id, currency_enum, amount_in_cents)).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "set_fiat_balance",
            arg,
        ).expect("set_fiat_balance call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Set crypto balance for testing (ckBTC and ckUSDC)
    /// Uses the crypto_canister test-only endpoint set_crypto_balance_for_testing
    /// Accepts phone number or user_id
    pub fn set_crypto_balance(&self, user_identifier: &str, ckbtc: u64, ckusdc: u64) -> Result<(), String> {
        // If it's a phone number, look up the user_id first
        let user_id = if user_identifier.starts_with('+') {
            match self.get_user(user_identifier) {
                Ok(Some(user)) => user.id,
                Ok(None) => return Err(format!("User not found: {}", user_identifier)),
                Err(e) => return Err(format!("Failed to look up user: {}", e)),
            }
        } else {
            user_identifier.to_string()
        };

        let arg = encode_args((user_id, ckbtc, ckusdc)).unwrap();
        let response = self.pic.update_call(
            self.crypto_canister_id,
            Principal::anonymous(),
            "set_crypto_balance_for_testing",
            arg,
        ).map_err(|e| format!("set_crypto_balance_for_testing call failed: {:?}", e))?;

        decode_one(&response).expect("Failed to decode set_crypto_balance_for_testing response")
    }
    
    /// Get user by ID or phone
    pub fn get_user(&self, user_id: &str) -> Result<Option<User>, String> {
        let arg = encode_one(user_id.to_string()).unwrap();
        let response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "get_user_profile_update",
            arg,
        ).expect("get_user_profile_update call failed");
        
        // get_user_profile_update returns Result<UserProfile, String>
        // We need to convert UserProfile to Option<User>
        let profile_result: Result<UserProfile, String> = decode_one(&response).expect("Failed to decode");
        match profile_result {
            Ok(profile) => {
                // Convert UserProfile to User
                Ok(Some(User {
                    id: profile.id,
                    user_type: UserType::User, // Default
                    preferred_currency: FiatCurrency::from_code(&profile.preferred_currency).unwrap_or(FiatCurrency::UGX),
                    created_at: profile.created_at,
                    last_active: profile.last_active,
                    email: profile.email,
                    is_verified: false, // Default
                    kyc_status: match profile.kyc_status.as_str() {
                        "approved" => KYCStatus::Approved,
                        "pending" => KYCStatus::Pending,
                        _ => KYCStatus::NotStarted,
                    },
                    first_name: profile.first_name,
                    last_name: profile.last_name,
                    principal_id: profile.principal_id,
                    phone_number: profile.phone_number,
                }))
            },
            Err(_) => Ok(None),
        }
    }
    
    /// Check fiat balance via business logic (accepts phone number or user_id)
    /// Returns balance in CENTS (e.g., 100000 cents = 1000 UGX)
    pub fn check_fiat_balance(&self, user_identifier: &str, currency: &str) -> Result<u64, String> {
        // If it's a phone number, look up the user_id first
        let user_id = if user_identifier.starts_with('+') {
            match self.get_user(user_identifier) {
                Ok(Some(user)) => user.id,
                Ok(None) => return Err(format!("User not found: {}", user_identifier)),
                Err(e) => return Err(e),
            }
        } else {
            user_identifier.to_string()
        };

        // Convert currency string to FiatCurrency enum
        let currency_enum = FiatCurrency::from_code(currency)
            .ok_or_else(|| format!("Invalid currency code: {}", currency))?;

        let arg = encode_args((user_id, currency_enum)).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "get_fiat_balance",
            arg,
        ).expect("check_fiat_balance call failed");

        let balance_in_cents: Result<u64, String> = decode_one(&response).expect("Failed to decode");
        balance_in_cents
    }
    
    /// Get crypto balance from data canister (accepts phone number or user_id)
    pub fn get_crypto_balance(&self, user_identifier: &str) -> Result<(u64, u64), String> {
        // If it's a phone number, look up the user_id first
        let user_id = if user_identifier.starts_with('+') {
            match self.get_user(user_identifier) {
                Ok(Some(user)) => user.id,
                Ok(None) => return Err(format!("User not found: {}", user_identifier)),
                Err(e) => return Err(e),
            }
        } else {
            user_identifier.to_string()
        };

        // Get BTC balance
        let btc_arg = encode_args((user_id.clone(), "CkBTC".to_string())).unwrap();
        let btc_response = self.pic.update_call(
            self.crypto_canister_id,
            Principal::anonymous(),
            "check_crypto_balance",
            btc_arg,
        ).expect("check_crypto_balance (BTC) call failed");

        let btc_balance: Result<u64, String> = decode_one(&btc_response).expect("Failed to decode BTC balance");

        // Get USDC balance
        let usdc_arg = encode_args((user_id.clone(), "CkUSDC".to_string())).unwrap();
        let usdc_response = self.pic.update_call(
            self.crypto_canister_id,
            Principal::anonymous(),
            "check_crypto_balance",
            usdc_arg,
        ).expect("check_crypto_balance (USDC) call failed");

        let usdc_balance: Result<u64, String> = decode_one(&usdc_response).expect("Failed to decode USDC balance");

        Ok((btc_balance?, usdc_balance?))
    }

    /// Fund user ledger balances in mock ledgers (for sell/transfer operations)
    /// This sets the actual ledger balance to match the data_canister balance
    pub fn fund_user_ledger_balances(&self, user_id: &str, btc_amount: u64, usdc_amount: u64) -> Result<(), String> {
        // Get user's principal from user_canister
        let user_arg = encode_one(user_id.to_string()).unwrap();
        let user_response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "get_user_profile_update",
            user_arg,
        ).map_err(|e| format!("Failed to get user profile: {:?}", e))?;

        let profile_result: Result<UserProfile, String> = decode_one(&user_response)
            .map_err(|e| format!("Failed to decode user profile: {:?}", e))?;
        let profile = profile_result?;

        let user_principal = Principal::from_text(&profile.principal_id.ok_or("User has no principal")?)
            .map_err(|e| format!("Invalid principal: {:?}", e))?;

        // Fund ckBTC ledger balance if needed
        if btc_amount > 0 {
            #[derive(CandidType, Deserialize, Clone)]
            struct Account { owner: Principal, subaccount: Option<Vec<u8>> }
            let btc_account = Account {
                owner: user_principal,
                subaccount: None,
            };
            let arg = encode_args((btc_account, btc_amount)).unwrap();
            self.pic.update_call(
                self.ckbtc_ledger_id,
                Principal::anonymous(),
                "set_balance_for_testing",
                arg,
            ).map_err(|e| format!("Failed to fund ckBTC ledger: {:?}", e))?;
        }

        // Fund ckUSDC ledger balance if needed
        if usdc_amount > 0 {
            #[derive(CandidType, Deserialize, Clone)]
            struct Account { owner: Principal, subaccount: Option<Vec<u8>> }
            let usdc_account = Account {
                owner: user_principal,
                subaccount: None,
            };
            let arg = encode_args((usdc_account, usdc_amount)).unwrap();
            self.pic.update_call(
                self.ckusdc_ledger_id,
                Principal::anonymous(),
                "set_balance_for_testing",
                arg,
            ).map_err(|e| format!("Failed to fund ckUSDC ledger: {:?}", e))?;
        }

        Ok(())
    }
    
    /// Setup test user with initial balances - ONE FUNCTION TO RULE THEM ALL
    /// This is the ONLY method tests should use - completely idempotent
    pub fn setup_test_user_with_balances(
        &self,
        phone: &str,
        first_name: &str,
        last_name: &str,
        email: &str,
        currency: &str,
        pin: &str,
        fiat_balance: u64,
        btc_balance: u64,
        usdc_balance: u64,
    ) -> Result<String, String> {
        // Step 1: Register user (idempotent)
        let user_id = self.register_user_direct(phone, first_name, last_name, email, currency, pin)?;

        // Step 2: Reset ALL currencies EXCEPT the one we're about to set
        let all_test_currencies = vec!["UGX", "KES", "TZS", "RWF", "NGN", "GHS", "ZAR"];
        for curr in all_test_currencies {
            if curr != currency {
                let _ = self.set_fiat_balance(phone, curr, 0);
            }
        }

        // Step 3: Set the currency we care about (use user_id, not phone!)
        self.set_fiat_balance(&user_id, currency, fiat_balance)?;

        // Step 4: Set crypto balances (use user_id, not phone!)
        self.set_crypto_balance(&user_id, btc_balance, usdc_balance)?;

        // Step 5: Fund user ledger balances (so they can sell/transfer crypto)
        // This ensures the mock ledgers have actual balances that match data_canister
        if btc_balance > 0 || usdc_balance > 0 {
            self.fund_user_ledger_balances(&user_id, btc_balance, usdc_balance)?;
        }

        Ok(user_id)
    }

    /// Register agent for withdrawal tests
    /// Agents are just users - returns the user_id which should be used as agent_id
    /// Idempotent - returns existing user_id if agent already registered
    pub fn register_agent(&self, agent_name: &str) -> Result<String, String> {
        // Create unique phone number for agent based on name hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        agent_name.hash(&mut hasher);
        let hash = hasher.finish();
        let phone_suffix = format!("{:06}", hash % 1_000_000);
        let agent_phone = format!("+256788{}", phone_suffix);

        // Register agent as a user (idempotent)
        let user_id = self.register_user_direct(
            &agent_phone,
            "Agent",
            agent_name,
            &format!("{}@agent.test", agent_name.to_lowercase()),
            "UGX",
            "1111" // Default agent PIN
        )?;

        Ok(user_id)
    }
}
