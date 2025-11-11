// USSD Canister Integration Tests
use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

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
pub mod debug_balance_test;

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
// Test Environment Setup
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
    pub ussd_canister_id: Principal,
    pub business_logic_canister_id: Principal,
    pub data_canister_id: Principal,
    pub exchange_canister_id: Principal,
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
        
        let business_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/business_logic_canister.wasm")
        ).expect("business_logic_canister WASM not found");
        
        let exchange_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/exchange_canister.wasm")
        ).expect("exchange_canister WASM not found");
        
        let data_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
        ).expect("data_canister WASM not found");
        
        // Install data canister first
        let data_canister_id = pic.create_canister();
        pic.add_cycles(data_canister_id, 2_000_000_000_000);
        let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);
        
        // Install business logic canister with data canister ID
        let business_logic_canister_id = pic.create_canister();
        pic.add_cycles(business_logic_canister_id, 2_000_000_000_000);
        let init_arg = encode_one(data_canister_id.to_text()).unwrap();
        pic.install_canister(business_logic_canister_id, business_wasm, init_arg, None);
        
        // Set business logic canister as its own controller (for add_authorized_canister)
        pic.set_controllers(
            business_logic_canister_id,
            None,
            vec![business_logic_canister_id],
        ).expect("Failed to set controllers");
        
        // Authorize business_logic to call data_canister
        let auth_arg = encode_one(business_logic_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize business logic canister");
        
        // Install exchange canister
        let exchange_canister_id = pic.create_canister();
        pic.add_cycles(exchange_canister_id, 2_000_000_000_000);
        pic.install_canister(exchange_canister_id, exchange_wasm, vec![], None);
        
        // Install USSD canister with optional business logic canister ID
        let ussd_canister_id = pic.create_canister();
        pic.add_cycles(ussd_canister_id, 2_000_000_000_000);
        let ussd_init_arg = encode_one(None::<String>).unwrap();
        pic.install_canister(ussd_canister_id, ussd_wasm, ussd_init_arg, None);
        
        // Configure USSD canister with business logic and exchange canister IDs
        let config_business = encode_one(business_logic_canister_id.to_text()).unwrap();
        pic.update_call(
            ussd_canister_id,
            Principal::anonymous(),
            "set_business_logic_canister_id",
            config_business,
        ).ok();
        
        let config_exchange = encode_one(exchange_canister_id).unwrap();
        pic.update_call(
            ussd_canister_id,
            Principal::anonymous(),
            "set_exchange_canister_id",
            config_exchange,
        ).ok();
        
        // Authorize USSD canister to call business logic canister
        // Call as business logic canister (which is now its own controller)
        let auth_ussd = encode_one(ussd_canister_id.to_text()).unwrap();
        pic.update_call(
            business_logic_canister_id,
            business_logic_canister_id,
            "add_authorized_canister",
            auth_ussd,
        ).expect("Failed to authorize USSD canister");
        
        Self {
            pic,
            ussd_canister_id,
            business_logic_canister_id,
            data_canister_id,
            exchange_canister_id,
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
    
    /// Get spread from exchange canister
    pub fn get_exchange_spread(&self) -> u64 {
        let response = self.pic.query_call(
            self.exchange_canister_id,
            Principal::anonymous(),
            "get_spread_basis_points",
            vec![],
        ).expect("get_spread_basis_points call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Register user via business logic canister (for test setup)
    pub fn register_user_direct(
        &self,
        phone_number: &str,
        first_name: &str,
        last_name: &str,
        email: &str,
        preferred_currency: &str,
        pin: &str,
    ) -> Result<String, String> {
        let request = RegisterUserRequest {
            phone_number: Some(phone_number.to_string()),
            principal_id: None,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            preferred_currency: preferred_currency.to_string(),
            pin: pin.to_string(),
        };
        
        let arg = encode_one(request).unwrap();
        let response = self.pic.update_call(
            self.business_logic_canister_id,
            self.ussd_canister_id, // Call as USSD canister (authorized)
            "register_user",
            arg,
        ).expect("register_user call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Set fiat balance for testing (accepts phone number or user_id)
    /// Amount is in base currency units (e.g., UGX), will be converted to cents internally
    pub fn set_fiat_balance(&self, user_identifier: &str, currency: &str, amount_in_currency: u64) -> Result<(), String> {
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
        
        // Convert to cents (multiply by 100)
        let amount_in_cents = amount_in_currency * 100;
        
        let arg = encode_args((user_id, currency, amount_in_cents)).unwrap();
        let response = self.pic.update_call(
            self.data_canister_id,
            self.business_logic_canister_id,
            "set_fiat_balance",
            arg,
        ).expect("set_fiat_balance call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Set crypto balance for testing (accepts phone number or user_id)
    pub fn set_crypto_balance(&self, user_identifier: &str, ckbtc: u64, ckusdc: u64) -> Result<(), String> {
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
        
        let arg = encode_args((user_id, ckbtc, ckusdc)).unwrap();
        let response = self.pic.update_call(
            self.business_logic_canister_id,
            self.ussd_canister_id,
            "set_crypto_balance",
            arg,
        ).expect("set_crypto_balance call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Get user by ID or phone
    pub fn get_user(&self, user_id: &str) -> Result<Option<User>, String> {
        // If it looks like a phone number, use get_user_by_phone
        let method = if user_id.starts_with('+') {
            "get_user_by_phone"
        } else {
            "get_user"
        };
        
        let arg = encode_one(user_id.to_string()).unwrap();
        let response = self.pic.query_call(
            self.data_canister_id,
            self.business_logic_canister_id, // Business logic is authorized on data canister
            method,
            arg,
        ).expect(&format!("{} call failed", method));
        
        decode_one(&response).expect("Failed to decode")
    }
    
    /// Check fiat balance via business logic
    /// Returns balance in base currency units (e.g., UGX), converted from cents
    pub fn check_fiat_balance(&self, user_id: &str, currency: &str) -> Result<u64, String> {
        let arg = encode_args((user_id, currency)).unwrap();
        let response = self.pic.update_call(
            self.business_logic_canister_id,
            self.ussd_canister_id,
            "check_fiat_balance",
            arg,
        ).expect("check_fiat_balance call failed");
        
        let balance_in_cents: Result<u64, String> = decode_one(&response).expect("Failed to decode");
        // Convert from cents to currency units (divide by 100)
        balance_in_cents.map(|cents| cents / 100)
    }
    
    /// Get crypto balance from data canister
    pub fn get_crypto_balance(&self, user_id: &str) -> Result<(u64, u64), String> {
        let arg = encode_one(user_id.to_string()).unwrap();
        let response = self.pic.query_call(
            self.data_canister_id,
            self.business_logic_canister_id,
            "get_crypto_balance",
            arg,
        ).expect("get_crypto_balance call failed");
        
        let balance: Result<CryptoBalance, String> = decode_one(&response).expect("Failed to decode");
        balance.map(|b| (b.ckbtc, b.ckusdc))
    }
}
