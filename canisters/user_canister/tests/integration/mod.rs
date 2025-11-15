use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;

pub mod user_registration_tests;
pub mod pin_security_tests;
pub mod access_control_tests;
pub mod user_enumeration_tests;

// ============================================================================
// Test Environment Setup - User Canister Only
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
    pub data_canister_id: Principal,
    pub user_canister_id: Principal,
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
        ).expect("data_canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release --package data_canister");
        
        let user_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/user_canister.wasm")
        ).expect("user_canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release --package user_canister");
        
        // Install data canister
        let data_canister_id = pic.create_canister();
        pic.add_cycles(data_canister_id, 2_000_000_000_000);
        let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);
        
        // Install user canister
        let user_canister_id = pic.create_canister();
        pic.add_cycles(user_canister_id, 2_000_000_000_000);
        pic.install_canister(user_canister_id, user_wasm, vec![], None);
        
        // Configure user canister
        let config_arg = encode_args((data_canister_id,)).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "set_data_canister_id",
            config_arg,
        ).expect("Failed to configure data canister ID");
        
        // Enable test mode
        let test_mode_arg = encode_one(()).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "enable_test_mode",
            test_mode_arg,
        ).expect("Failed to enable test mode");
        
        // Authorize user_canister to call data_canister
        let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize user canister");
        
        Self {
            pic,
            data_canister_id,
            user_canister_id,
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
            self.user_canister_id,
            Principal::anonymous(),
            "register_user",
            arg,
        ).expect("register_user call failed");
        
        decode_one(&response).expect("Failed to decode register_user response")
    }
    
    pub fn user_exists(&self, user_identifier: &str) -> Result<bool, String> {
        let arg = encode_one(user_identifier.to_string()).unwrap();
        let response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "user_exists",
            arg,
        ).expect("user_exists call failed");
        
        decode_one(&response).expect("Failed to decode user_exists response")
    }
    
    pub fn verify_pin(&self, user_identifier: &str, pin: &str) -> Result<bool, String> {
        let arg = encode_args((user_identifier, pin)).unwrap();
        let response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "verify_pin",
            arg,
        ).expect("verify_pin call failed");
        
        decode_one(&response).expect("Failed to decode verify_pin response")
    }
    
    pub fn change_pin(&self, user_identifier: &str, old_pin: &str, new_pin: &str) -> Result<(), String> {
        let arg = encode_args((user_identifier, old_pin, new_pin)).unwrap();
        let response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "change_pin",
            arg,
        ).expect("change_pin call failed");
        
        decode_one(&response).expect("Failed to decode change_pin response")
    }
    
    pub fn link_phone_to_account(&self, principal_id: &str, phone_number: &str) -> Result<(), String> {
        let arg = encode_args((principal_id, phone_number)).unwrap();
        let response = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "link_phone_to_account",
            arg,
        ).expect("link_phone_to_account call failed");
        
        decode_one(&response).expect("Failed to decode link_phone_to_account response")
    }
    
    // Helper to get user directly from data canister (for verification)
    pub fn get_user(&self, user_id: &str) -> Result<Option<User>, String> {
        let arg = encode_one(user_id.to_string()).unwrap();
        let response = self.pic.update_call(
            self.data_canister_id,
            Principal::anonymous(),
            "get_user",
            arg,
        ).expect("get_user call failed");
        
        decode_one(&response).expect("Failed to decode get_user response")
    }
}
