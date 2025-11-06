// UssdWorld - shared test state across all step definitions

use cucumber::World;
use std::sync::Arc;
use crate::mocks::juno_mock::MockJunoStore;
use crate::mocks::canister_mock::{MockCkBtcCanister, MockCkUsdcCanister};

// Test session that mirrors satellite's UssdSession
#[derive(Debug, Clone)]
pub struct TestSession {
    pub session_id: String,
    pub phone_number: String,
    pub language: String,
    pub current_menu: String,
    pub step: u32,
    pub data: std::collections::HashMap<String, String>,
    pub last_activity: u64,
}

impl TestSession {
    pub fn new(phone: String) -> Self {
        Self {
            session_id: format!("test_{}", uuid::Uuid::new_v4()),
            phone_number: phone,
            language: "en".to_string(),
            current_menu: String::new(),
            step: 0,
            data: std::collections::HashMap::new(),
            last_activity: 0,
        }
    }
}

// World state shared across all step definitions
#[derive(Debug, World)]
pub struct UssdWorld {
    pub phone_number: String,
    pub pin: String,
    pub session: Option<TestSession>,
    pub last_response: String,
    pub continue_session: bool,
    
    // Mock infrastructure - ONLY these are mocked
    pub juno_store: Arc<MockJunoStore>,
    pub ckbtc_canister: Arc<MockCkBtcCanister>,
    pub ckusdc_canister: Arc<MockCkUsdcCanister>,
}

impl Default for UssdWorld {
    fn default() -> Self {
        Self {
            phone_number: String::new(),
            pin: String::new(),
            session: None,
            last_response: String::new(),
            continue_session: true,
            juno_store: Arc::new(MockJunoStore::new()),
            ckbtc_canister: Arc::new(MockCkBtcCanister::new()),
            ckusdc_canister: Arc::new(MockCkUsdcCanister::new()),
        }
    }
}

impl UssdWorld {
    pub fn get_or_create_session(&mut self) -> &mut TestSession {
        if self.session.is_none() {
            self.session = Some(TestSession::new(self.phone_number.clone()));
        }
        self.session.as_mut().unwrap()
    }
    
    // Main entry point - processes USSD input
    pub async fn process_ussd_input(&mut self, input: &str) {
        let session = self.get_or_create_session();
        
        // Determine which handler to call based on current state
        let (response, cont) = if input == "*229#" || session.current_menu.is_empty() {
            // Main menu
            super::handlers::handle_main_menu(self, input).await
        } else {
            // Submenu
            super::handlers::handle_submenu(self, input).await
        };
        
        self.last_response = response;
        self.continue_session = cont;
    }
}
