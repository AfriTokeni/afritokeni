// Data Canister integration test harness using PocketIC
// Mirrors the style used by other canisters' integration tests (TestEnv with helpers)

use candid::{encode_one, decode_one, Principal};
use pocket_ic::PocketIc;
use shared_types::*;
use std::sync::{OnceLock, Mutex, MutexGuard};

static ENV: OnceLock<Mutex<TestEnv>> = OnceLock::new();

pub fn get_test_env() -> MutexGuard<'static, TestEnv> {
    ENV.get_or_init(|| Mutex::new(TestEnv::new())).lock().unwrap()
}

pub struct TestEnv {
    pub pic: PocketIc,
    pub canister_id: Principal,
}

impl TestEnv {
    pub fn new() -> Self {
        // Load WASM built for data_canister. Use workspace target path like other canisters' tests.
        const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/data_canister.wasm";
        let wasm = std::fs::read(WASM_PATH)
            .expect("data_canister WASM not found. Build it with `cargo build --target wasm32-unknown-unknown --release` from workspace root");

        // Spin up PocketIC and install the canister
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 2_000_000_000_000);

        // data_canister init takes (Option<String>, Option<String>) for authorized canisters
        let init_arg = encode_one((None::<String>, None::<String>)).unwrap();
        pic.install_canister(canister_id, wasm, init_arg, None);

        let env = Self { pic, canister_id };

        // Authorize the anonymous principal so tests can call update methods deterministically
        env.authorize_caller(Principal::anonymous());

        env
    }

    pub fn authorize_caller(&self, principal: Principal) {
        let arg = encode_one(principal.to_text()).unwrap();
        self.pic.update_call(
            self.canister_id,
            Principal::anonymous(),
            "add_authorized_canister",
            arg,
        ).expect("Failed to add authorized canister");
    }

    pub fn create_user_direct(&self, req: CreateUserRequest) -> Result<User, String> {
        let arg = encode_one(req).unwrap();
        let bytes = self.pic.update_call(
            self.canister_id,
            Principal::anonymous(),
            "create_user",
            arg,
        ).expect("create_user call failed");
        decode_one::<Result<User, String>>(&bytes).unwrap()
    }

    pub fn get_user(&self, user_id: &str) -> Result<Option<User>, String> {
        let arg = encode_one(user_id.to_string()).unwrap();
        let bytes = self.pic.query_call(
            self.canister_id,
            Principal::anonymous(),
            "get_user",
            arg,
        );
        match bytes {
            Ok(b) => decode_one::<Result<Option<User>, String>>(&b).unwrap(),
            Err(e) => Err(format!("query get_user failed: {e:?}")),
        }
    }

    pub fn set_fiat_balance(&self, user_id: &str, currency: &str, amount: u64) -> Result<(), String> {
        let arg = encode_one((user_id.to_string(), currency.to_string(), amount)).unwrap();
        let bytes = self.pic.update_call(
            self.canister_id,
            Principal::anonymous(),
            "set_fiat_balance",
            arg,
        ).expect("set_fiat_balance call failed");
        decode_one::<Result<(), String>>(&bytes).unwrap()
    }

    pub fn get_fiat_balance(&self, user_id: &str, currency: FiatCurrency) -> Result<u64, String> {
        let arg = encode_one((user_id.to_string(), currency)).unwrap();
        let bytes = self.pic.query_call(
            self.canister_id,
            Principal::anonymous(),
            "get_fiat_balance",
            arg,
        ).expect("get_fiat_balance query failed");
        decode_one::<Result<u64, String>>(&bytes).unwrap()
    }
}

// A basic happy-path integration test to ensure harness works
#[test]
fn integration_create_user_and_balances() {
    let env = get_test_env();

    let req = CreateUserRequest {
        user_type_str: "User".to_string(),
        preferred_currency_str: "UGX".to_string(),
        email: "int@example.com".to_string(),
        first_name: "Int".to_string(),
        last_name: "Test".to_string(),
        principal_id: None,
        phone_number: Some("+256700000111".to_string()),
    };

    let user = env.create_user_direct(req).expect("user creation should succeed");
    assert_eq!(user.email, "int@example.com");

    // Verify retrieval works through query
    let fetched = env.get_user(&user.id).expect("get_user should succeed").expect("user should exist");
    assert_eq!(fetched.id, user.id);
}
