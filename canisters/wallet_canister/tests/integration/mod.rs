use candid::{encode_one, encode_args, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;

pub mod transfer_tests;
pub mod escrow_tests;
pub mod fraud_detection_tests;
pub mod balance_integrity_tests;
pub mod security_tests;

// ============================================================================
// Test Environment Setup - Wallet Canister Integration
// ============================================================================

pub struct TestEnv {
    pub pic: PocketIc,
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
        let data_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/data_canister.wasm")
        ).expect("data_canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release --package data_canister");
        
        let user_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/user_canister.wasm")
        ).expect("user_canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release --package user_canister");
        
        let wallet_wasm = std::fs::read(
            workspace_root.join("target/wasm32-unknown-unknown/release/wallet_canister.wasm")
        ).expect("wallet_canister WASM not found. Run: cargo build --target wasm32-unknown-unknown --release --package wallet_canister");
        
        // Install data canister
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
        
        // Configure user canister
        let config_arg = encode_args((data_canister_id,)).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "set_data_canister_id",
            config_arg,
        ).expect("Failed to configure user canister data_canister_id");
        
        // Enable test mode on user_canister (allows anonymous calls)
        let test_mode_arg = encode_one(()).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "enable_test_mode",
            test_mode_arg,
        ).expect("Failed to enable test mode on user_canister");
        
        // Configure wallet canister
        let config_arg = encode_args((data_canister_id,)).unwrap();
        pic.update_call(
            wallet_canister_id,
            Principal::anonymous(),
            "set_data_canister_id",
            config_arg,
        ).expect("Failed to configure wallet canister data_canister_id");

        let config_arg = encode_args((user_canister_id,)).unwrap();
        pic.update_call(
            wallet_canister_id,
            Principal::anonymous(),
            "set_user_canister_id",
            config_arg,
        ).expect("Failed to configure wallet canister user_canister_id");

        // Authorize canisters
        let auth_arg = encode_one(user_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize user canister");

        let auth_arg = encode_one(wallet_canister_id.to_text()).unwrap();
        pic.update_call(
            data_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize wallet canister");

        let auth_arg = encode_args((wallet_canister_id,)).unwrap();
        pic.update_call(
            user_canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            auth_arg,
        ).expect("Failed to authorize wallet canister on user canister");
        
        Self {
            pic,
            data_canister_id,
            user_canister_id,
            wallet_canister_id,
        }
    }
    
    // ========================================================================
    // User Management Helpers
    // ========================================================================
    
    pub fn register_user(
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
            self.user_canister_id,
            Principal::anonymous(),
            "register_user",
            arg,
        ).expect("register_user call failed");
        
        decode_one(&response).expect("Failed to decode register_user response")
    }
    
    // ========================================================================
    // Balance Management Helpers
    // ========================================================================
    
    pub fn set_fiat_balance(&self, user_id: &str, currency: &str, amount: u64) -> Result<(), String> {
        // Use encode_args with 3 parameters, call AS wallet_canister to pass authorization
        let arg = encode_args((user_id, currency, amount)).unwrap();
        let response = self.pic.update_call(
            self.data_canister_id,
            self.wallet_canister_id, // Call as wallet_canister to pass authorization
            "set_fiat_balance",
            arg,
        ).expect("set_fiat_balance call failed");
        
        decode_one(&response).expect("Failed to decode set_fiat_balance response")
    }
    
    pub fn get_fiat_balance(&self, user_id: &str, currency: &str) -> Result<u64, String> {
        // data_canister expects (String, FiatCurrency) and it's a QUERY
        let fiat_currency = FiatCurrency::from_string(currency).expect("Invalid currency");
        let arg = encode_args((user_id.to_string(), fiat_currency)).unwrap();
        let response = self.pic.query_call(
            self.data_canister_id,
            self.wallet_canister_id, // Call as wallet_canister
            "get_fiat_balance",
            arg,
        ).expect("get_fiat_balance call failed");
        
        decode_one(&response).expect("Failed to decode get_fiat_balance response")
    }
    
    pub fn update_crypto_balance(&self, user_id: &str, ckbtc_delta: i64, ckusdc_delta: i64) -> Result<(), String> {
        // Use encode_args with 3 parameters
        let arg = encode_args((user_id, ckbtc_delta, ckusdc_delta)).unwrap();
        let response = self.pic.update_call(
            self.data_canister_id,
            self.wallet_canister_id, // Call as wallet_canister
            "update_crypto_balance",
            arg,
        ).expect("update_crypto_balance call failed");
        
        decode_one(&response).expect("Failed to decode update_crypto_balance response")
    }
    
    // ========================================================================
    // Wallet Operations
    // ========================================================================
    
    pub fn transfer_fiat(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        amount: u64,
        currency: &str,
        pin: &str,
        description: Option<String>,
    ) -> Result<TransferResponse, String> {
        #[derive(candid::CandidType, candid::Deserialize)]
        struct TransferRequest {
            from_user_id: String,
            to_user_id: String,
            amount: u64,
            currency: String,
            pin: String,
            description: Option<String>,
        }
        
        let request = TransferRequest {
            from_user_id: from_user_id.to_string(),
            to_user_id: to_user_id.to_string(),
            amount,
            currency: currency.to_string(),
            pin: pin.to_string(),
            description,
        };
        
        let arg = encode_one(request).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "transfer_fiat",
            arg,
        ).expect("transfer_fiat call failed");
        
        decode_one(&response).expect("Failed to decode transfer_fiat response")
    }
    
    pub fn create_escrow(
        &self,
        user_id: &str,
        agent_id: &str,
        amount: u64,
        crypto_type: &str,
        pin: &str,
    ) -> Result<CreateEscrowResponse, String> {
        #[derive(candid::CandidType, candid::Deserialize)]
        struct CreateEscrowRequest {
            user_id: String,
            agent_id: String,
            amount: u64,
            crypto_type: String,
            pin: String,
        }
        
        let request = CreateEscrowRequest {
            user_id: user_id.to_string(),
            agent_id: agent_id.to_string(),
            amount,
            crypto_type: crypto_type.to_string(),
            pin: pin.to_string(),
        };
        
        let arg = encode_one(request).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "create_escrow",
            arg,
        ).expect("create_escrow call failed");
        
        decode_one(&response).expect("Failed to decode create_escrow response")
    }
    
    pub fn claim_escrow(&self, code: &str, agent_id: &str) -> Result<(), String> {
        let arg = encode_args((code, agent_id)).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "claim_escrow",
            arg,
        ).expect("claim_escrow call failed");
        
        decode_one(&response).expect("Failed to decode claim_escrow response")
    }
    
    pub fn cancel_escrow(&self, code: &str, user_id: &str, pin: &str) -> Result<(), String> {
        let arg = encode_args((code, user_id, pin)).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "cancel_escrow",
            arg,
        ).expect("cancel_escrow call failed");
        
        decode_one(&response).expect("Failed to decode cancel_escrow response")
    }
    
    pub fn get_escrow(&self, code: &str) -> Result<Escrow, String> {
        let arg = encode_one(code.to_string()).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "get_escrow",
            arg,
        ).expect("get_escrow call failed");
        
        decode_one(&response).expect("Failed to decode get_escrow response")
    }
    
    pub fn get_transaction_history(
        &self,
        user_id: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Transaction>, String> {
        let arg = encode_args((user_id, limit, offset)).unwrap();
        let response = self.pic.update_call(
            self.wallet_canister_id,
            Principal::anonymous(),
            "get_transaction_history",
            arg,
        ).expect("get_transaction_history call failed");
        
        decode_one(&response).expect("Failed to decode get_transaction_history response")
    }
}

// Response types
#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct TransferResponse {
    pub transaction_id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: u64,
    pub fee: u64,
    pub currency: String,
    pub sender_new_balance: u64,
    pub recipient_new_balance: u64,
    pub timestamp: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateEscrowResponse {
    pub code: String,
    pub amount: u64,
    pub crypto_type: String,
    pub expires_at: u64,
}
