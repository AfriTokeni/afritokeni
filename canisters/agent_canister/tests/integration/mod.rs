use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;

mod deposit_tests;
mod withdrawal_tests;
mod agent_balance_tests;
mod settlement_tests;
mod fraud_detection_tests;
mod edge_case_tests;
mod multi_currency_tests;
mod pin_security_tests;
mod code_expiration_tests;
mod concurrent_operations_tests;

// ============================================================================
// Shared Request/Response Types for Tests
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct CreateDepositRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub pin: String,  // Changed from user_pin to match Candid interface
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct CreateDepositResponse {
    pub deposit_code: String,
    pub amount: u64,
    pub currency: String,
    pub agent_commission: u64,  // Changed from commission to match actual response
    pub net_to_user: u64,
    pub expires_at: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ConfirmDepositRequest {
    pub deposit_code: String,
    pub agent_id: String,
    pub agent_pin: String,
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct ConfirmDepositResponse {
    pub user_id: String,
    pub amount: u64,
    pub currency: String,
    pub commission: u64,
    pub user_balance_added: u64,
    pub confirmed_at: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct CreateWithdrawalRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub pin: String,  // Changed from user_pin to match Candid interface
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct CreateWithdrawalResponse {
    pub withdrawal_code: String,
    pub amount: u64,
    pub currency: String,
    pub total_fees: u64,
    pub net_to_user: u64,
    pub expires_at: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ConfirmWithdrawalRequest {
    pub withdrawal_code: String,
    pub agent_id: String,
    pub agent_pin: String,
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct ConfirmWithdrawalResponse {
    pub user_id: String,
    pub amount: u64,
    pub currency: String,
    pub total_fees: u64,
    pub confirmed_at: u64,
}

// ============================================================================
// Test Environment Setup
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
    pub agent_canister_id: Principal,
    pub data_canister_id: Principal,
    pub user_canister_id: Principal,
    pub wallet_canister_id: Principal,
}

impl TestEnv {
    pub fn new() -> Self {
        let pic = PocketIc::new();
        
        let workspace_root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf();
        
        // Load WASMs
        let agent_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/agent_canister.wasm")
        ).expect("agent_canister WASM not found - run 'cargo build --target wasm32-unknown-unknown --release' first");
        
        let data_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
        ).expect("data_canister WASM not found");
        
        let user_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/user_canister.wasm")
        ).expect("user_canister WASM not found");
        
        let wallet_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/wallet_canister.wasm")
        ).expect("wallet_canister WASM not found");
        
        // Install data canister
        let data_canister_id = pic.create_canister();
        pic.add_cycles(data_canister_id, 2_000_000_000_000);
        let data_init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(data_canister_id, data_wasm, data_init_arg, None);
        
        // Install user canister
        let user_canister_id = pic.create_canister();
        pic.add_cycles(user_canister_id, 2_000_000_000_000);
        let user_init_arg = encode_args((data_canister_id,)).unwrap();
        pic.install_canister(user_canister_id, user_wasm, user_init_arg, None);
        
        // Install wallet canister
        let wallet_canister_id = pic.create_canister();
        pic.add_cycles(wallet_canister_id, 2_000_000_000_000);
        let wallet_init_arg = encode_args((data_canister_id,)).unwrap();
        pic.install_canister(wallet_canister_id, wallet_wasm, wallet_init_arg, None);
        
        // Install agent canister
        let agent_canister_id = pic.create_canister();
        pic.add_cycles(agent_canister_id, 2_000_000_000_000);
        pic.install_canister(agent_canister_id, agent_wasm, vec![], None);
        
        // Configure user canister
        let config_data_user = encode_args((data_canister_id,)).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "set_data_canister_id",
            config_data_user,
        ).expect("Failed to set data canister ID in user canister");
        
        // Configure wallet canister
        let config_data_wallet = encode_args((data_canister_id,)).unwrap();
        pic.update_call(
            wallet_canister_id,
            Principal::anonymous(),
            "set_data_canister_id",
            config_data_wallet,
        ).expect("Failed to set data canister ID in wallet canister");
        
        let config_user_wallet = encode_args((user_canister_id,)).unwrap();
        pic.update_call(
            wallet_canister_id,
            Principal::anonymous(),
            "set_user_canister_id",
            config_user_wallet,
        ).expect("Failed to set user canister ID in wallet canister");
        
        // Configure agent canister with other canister IDs
        let config_data = encode_args((data_canister_id,)).unwrap();
        pic.update_call(
            agent_canister_id,
            Principal::anonymous(),
            "set_data_canister_id",
            config_data,
        ).expect("Failed to set data canister ID in agent canister");

        let config_user = encode_args((user_canister_id,)).unwrap();
        pic.update_call(
            agent_canister_id,
            Principal::anonymous(),
            "set_user_canister_id",
            config_user,
        ).expect("Failed to set user canister ID in agent canister");

        let config_wallet = encode_args((wallet_canister_id,)).unwrap();
        pic.update_call(
            agent_canister_id,
            Principal::anonymous(),
            "set_wallet_canister_id",
            config_wallet,
        ).expect("Failed to set wallet canister ID in agent canister");
        
        // Authorize canisters to call each other
        let auth_agent = encode_one(agent_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_agent,
        ).expect("Failed to authorize agent canister");

        let auth_user = encode_one(user_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_user,
        ).expect("Failed to authorize user canister");

        let auth_wallet = encode_one(wallet_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_wallet,
        ).expect("Failed to authorize wallet canister");

        // Authorize agent canister to call user and wallet canisters
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            encode_args((agent_canister_id,)).unwrap(),
        ).expect("Failed to authorize agent canister in user canister");

        pic.update_call(
            wallet_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            encode_args((agent_canister_id,)).unwrap(),
        ).expect("Failed to authorize agent canister in wallet canister");
        
        Self {
            pic,
            agent_canister_id,
            data_canister_id,
            user_canister_id,
            wallet_canister_id,
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
        
        let args = encode_one(request).unwrap();
        let result = self.pic.update_call(
            self.user_canister_id,
            Principal::anonymous(),
            "register_user",
            args,
        ).expect("register_user call failed");
        
        decode_one(&result).expect("Failed to decode register_user response")
    }
    
    pub fn set_fiat_balance(
        &self,
        user_id: &str,
        currency: &str,
        amount: u64,
    ) -> Result<(), String> {
        let currency_enum = FiatCurrency::from_code(currency)
            .ok_or_else(|| format!("Invalid currency: {}", currency))?;
        let args = encode_args((user_id.to_string(), currency_enum, amount)).unwrap();
        let result = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "set_fiat_balance",
            args,
        ).expect("set_fiat_balance call failed");
        
        decode_one(&result).expect("Failed to decode set_fiat_balance response")
    }
    
    pub fn get_fiat_balance(
        &self,
        user_id: &str,
        currency: &str,
    ) -> Result<u64, String> {
        let currency_enum = FiatCurrency::from_code(currency)
            .ok_or_else(|| format!("Invalid currency: {}", currency))?;
        let args = encode_args((user_id.to_string(), currency_enum)).unwrap();
        let result = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "get_fiat_balance",
            args,
        ).expect("get_fiat_balance call failed");
        
        decode_one(&result).expect("Failed to decode get_fiat_balance response")
    }
}
