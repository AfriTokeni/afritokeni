use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;

pub mod user_registration_tests;
pub mod money_transfer_tests;
pub mod deposit_withdrawal_tests;
pub mod balance_integrity_tests;
pub mod pin_security_tests;
pub mod error_handling_tests;
pub mod crypto_operations_tests;
pub mod escrow_tests;
pub mod exchange_rate_tests;
pub mod fraud_detection_tests;
pub mod deposit_commission_tests;
pub mod withdrawal_commission_tests;
pub mod exchange_spread_tests;
pub mod commission_end_to_end_tests;
pub mod crypto_swap_tests;

// ============================================================================
// Test Environment Setup
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
    pub data_canister_id: Principal,
    pub business_canister_id: Principal,
    pub deposit_canister_id: Option<Principal>,
    pub withdrawal_canister_id: Option<Principal>,
    pub exchange_canister_id: Option<Principal>,
}

impl TestEnv {
    pub fn new() -> Self {
        let pic = PocketIc::new();
        
        let workspace_root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf();
        
        let data_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
        ).expect("data_canister WASM not found");
        
        let business_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/business_logic_canister.wasm")
        ).expect("business_logic_canister WASM not found");
        
        // Install data canister
        let data_canister_id = pic.create_canister();
        pic.add_cycles(data_canister_id, 2_000_000_000_000);
        let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);
        
        // Install business logic canister
        let business_canister_id = pic.create_canister();
        pic.add_cycles(business_canister_id, 2_000_000_000_000);
        let init_arg = encode_one(data_canister_id.to_text()).unwrap();
        pic.install_canister(business_canister_id, business_wasm, init_arg, None);
        
        // Authorize business_logic to call data_canister
        let auth_arg = encode_one(business_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize");
        
        Self {
            pic,
            data_canister_id,
            business_canister_id,
            deposit_canister_id: None,
            withdrawal_canister_id: None,
            exchange_canister_id: None,
        }
    }
    
    /// Create test environment with commission canisters deployed
    pub fn new_with_commission_canisters() -> Self {
        let pic = PocketIc::new();
        
        let workspace_root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf();
        
        // Load all WASMs
        let data_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
        ).expect("data_canister WASM not found");
        
        let business_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/business_logic_canister.wasm")
        ).expect("business_logic_canister WASM not found");
        
        let deposit_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/deposit_canister.wasm")
        ).expect("deposit_canister WASM not found");
        
        let withdrawal_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/withdrawal_canister.wasm")
        ).expect("withdrawal_canister WASM not found");
        
        let exchange_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/exchange_canister.wasm")
        ).expect("exchange_canister WASM not found");
        
        // Install data canister
        let data_canister_id = pic.create_canister();
        pic.add_cycles(data_canister_id, 2_000_000_000_000);
        let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);
        
        // Install deposit canister
        let deposit_canister_id = pic.create_canister();
        pic.add_cycles(deposit_canister_id, 2_000_000_000_000);
        pic.install_canister(deposit_canister_id, deposit_wasm, vec![], None);
        
        // Install withdrawal canister
        let withdrawal_canister_id = pic.create_canister();
        pic.add_cycles(withdrawal_canister_id, 2_000_000_000_000);
        pic.install_canister(withdrawal_canister_id, withdrawal_wasm, vec![], None);
        
        // Install exchange canister
        let exchange_canister_id = pic.create_canister();
        pic.add_cycles(exchange_canister_id, 2_000_000_000_000);
        pic.install_canister(exchange_canister_id, exchange_wasm, vec![], None);
        
        // Install business logic canister with all canister IDs
        let business_canister_id = pic.create_canister();
        pic.add_cycles(business_canister_id, 2_000_000_000_000);
        let init_arg = encode_one(data_canister_id.to_text()).unwrap();
        pic.install_canister(business_canister_id, business_wasm, init_arg, None);
        
        // Configure commission canister IDs in business logic
        let config_deposit = encode_one(deposit_canister_id.to_text()).unwrap();
        pic.update_call(
            business_canister_id,
            Principal::anonymous(),
            "set_deposit_canister_id",
            config_deposit,
        ).ok(); // Ignore if method doesn't exist yet
        
        let config_withdrawal = encode_one(withdrawal_canister_id.to_text()).unwrap();
        pic.update_call(
            business_canister_id,
            Principal::anonymous(),
            "set_withdrawal_canister_id",
            config_withdrawal,
        ).ok();
        
        let config_exchange = encode_one(exchange_canister_id.to_text()).unwrap();
        pic.update_call(
            business_canister_id,
            Principal::anonymous(),
            "set_exchange_canister_id",
            config_exchange,
        ).ok();
        
        // Authorize business_logic to call data_canister
        let auth_arg = encode_one(business_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize");
        
        Self {
            pic,
            data_canister_id,
            business_canister_id,
            deposit_canister_id: Some(deposit_canister_id),
            withdrawal_canister_id: Some(withdrawal_canister_id),
            exchange_canister_id: Some(exchange_canister_id),
        }
    }
    
    pub fn register_user(
        &self,
        phone_number: Option<String>,
        principal_id: Option<String>,
        first_name: &str,
        last_name: &str,
        email: &str,
        preferred_currency: &str,
        pin: &str,
    ) -> Result<String, String> {
        let request = RegisterUserRequest {
            phone_number,
            principal_id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            preferred_currency: preferred_currency.to_string(),
            pin: pin.to_string(),
        };
        
        let arg = encode_one(request).unwrap();
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "register_user",
            arg,
        ).expect("register_user call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn get_user(&self, user_id: &str) -> Result<Option<User>, String> {
        let arg = encode_one(user_id.to_string()).unwrap();
        let response = self.pic.query_call(
            self.data_canister_id,
            Principal::anonymous(),
            "get_user",
            arg,
        ).expect("get_user call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn check_fiat_balance(&self, user_id: &str, currency: &str) -> Result<u64, String> {
        let arg = encode_args((user_id, currency)).unwrap();
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "check_fiat_balance",
            arg,
        ).expect("check_fiat_balance call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn set_fiat_balance(&self, user_id: &str, currency: &str, amount: u64) -> Result<(), String> {
        // set_fiat_balance takes (text, text, nat64) - use encode_args for multiple arguments!
        let arg = encode_args((user_id, currency, amount)).unwrap();
        let response = self.pic.update_call(
            self.data_canister_id,
            self.business_canister_id, // Call as business_logic to pass authorization
            "set_fiat_balance",
            arg,
        ).expect("set_fiat_balance call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn send_money_to_phone(
        &self,
        from_phone: &str,
        to_phone: &str,
        amount: u64,
        currency: &str,
        pin: &str,
    ) -> Result<TransactionResult, String> {
        let arg = encode_args((from_phone, to_phone, amount, currency, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "send_money_to_phone",
            arg,
        ).expect("send_money_to_phone call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn withdraw_fiat(
        &self,
        phone_number: &str,
        amount: u64,
        currency: &str,
        agent_id: &str,
        pin: &str,
    ) -> Result<TransactionResult, String> {
        let arg = encode_args((phone_number, amount, currency, agent_id, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "withdraw_fiat",
            arg,
        ).expect("withdraw_fiat call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn get_transaction_history(
        &self,
        user_id: &str,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<Transaction>, String> {
        let arg = encode_args((user_id, offset, limit)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "get_transaction_history",
            arg,
        ).expect("get_transaction_history call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn buy_crypto(
        &self,
        user_identifier: &str,
        fiat_amount: u64,
        fiat_currency: &str,
        crypto_type: CryptoType,
        pin: &str,
    ) -> Result<TransactionResult, String> {
        let arg = encode_args((user_identifier, fiat_amount, fiat_currency, crypto_type, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "buy_crypto",
            arg,
        ).expect("buy_crypto call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn send_crypto(
        &self,
        user_identifier: &str,
        to_address: &str,
        amount: u64,
        crypto_type: CryptoType,
        pin: &str,
    ) -> Result<TransactionResult, String> {
        let arg = encode_args((user_identifier, to_address, amount, crypto_type, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "send_crypto",
            arg,
        ).expect("send_crypto call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn sell_crypto_to_agent(
        &self,
        user_identifier: &str,
        crypto_amount: u64,
        crypto_type: CryptoType,
        agent_id: &str,
        pin: &str,
    ) -> Result<TransactionResult, String> {
        let arg = encode_args((user_identifier, crypto_amount, crypto_type, agent_id, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "sell_crypto_to_agent",
            arg,
        ).expect("sell_crypto_to_agent call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn get_crypto_balance(&self, user_id: &str) -> Result<(u64, u64), String> {
        let arg = encode_one(user_id.to_string()).unwrap();
        let response = self.pic.query_call(
            self.data_canister_id,
            Principal::anonymous(),
            "get_crypto_balance",
            arg,
        ).expect("get_crypto_balance call failed");
        
        let balance: Result<CryptoBalance, String> = decode_one(&response).expect("Failed to decode");
        balance.map(|b| (b.ckbtc, b.ckusdc))
    }
    
    pub fn set_crypto_balance(&self, user_id: &str, ckbtc: u64, ckusdc: u64) -> Result<(), String> {
        let arg = encode_args((user_id, ckbtc, ckusdc)).unwrap();
        let response = self.pic.update_call(
            self.data_canister_id,
            self.business_canister_id,
            "set_crypto_balance",
            arg,
        ).expect("set_crypto_balance call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn create_escrow(
        &self,
        user_identifier: &str,
        crypto_amount: u64,
        crypto_type: CryptoType,
        agent_id: &str,
        pin: &str,
    ) -> Result<String, String> {
        // Use sell_crypto_to_agent which creates escrow
        let result = self.sell_crypto_to_agent(user_identifier, crypto_amount, crypto_type, agent_id, pin)?;
        Ok(result.transaction_id) // Returns escrow code
    }
    
    pub fn verify_escrow_code(
        &self,
        code: &str,
        agent_id: &str,
    ) -> Result<TransactionResult, String> {
        let arg = encode_args((code, agent_id)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "verify_escrow_code",
            arg,
        ).expect("verify_escrow_code call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn get_escrow_status(&self, code: &str) -> Result<Escrow, String> {
        let arg = encode_one(code.to_string()).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "get_escrow_status",
            arg,
        ).expect("get_escrow_status call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn cancel_escrow(
        &self,
        code: &str,
        user_id: &str,
        pin: &str,
    ) -> Result<(), String> {
        let arg = encode_args((code, user_id, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "cancel_escrow",
            arg,
        ).expect("cancel_escrow call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn get_crypto_value_estimate(
        &self,
        crypto_amount: u64,
        crypto_type: CryptoType,
        fiat_currency: &str,
    ) -> Result<u64, String> {
        let arg = encode_args((crypto_amount, crypto_type, fiat_currency)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "get_crypto_value_estimate",
            arg,
        ).expect("get_crypto_value_estimate call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
    
    pub fn swap_crypto(
        &self,
        user_identifier: &str,
        from_crypto: CryptoType,
        to_crypto: CryptoType,
        amount: u64,
        pin: &str,
    ) -> Result<SwapResult, String> {
        let arg = encode_args((user_identifier, from_crypto, to_crypto, amount, pin)).unwrap();
        
        let response = self.pic.update_call(
            self.business_canister_id,
            Principal::anonymous(),
            "swap_crypto",
            arg,
        ).expect("swap_crypto call failed");
        
        decode_one(&response).expect("Failed to decode")
    }
}
