use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;

pub mod user_registration_tests;
pub mod money_transfer_tests;
pub mod deposit_withdrawal_tests;

// ============================================================================
// Test Environment Setup
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
    pub data_canister_id: Principal,
    pub business_canister_id: Principal,
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
        
        Self { pic, data_canister_id, business_canister_id }
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
}
