use cucumber::{given, when, then, World};
use crate::session::UssdSession;
use crate::ussd_handlers;

#[derive(Debug, Default, World)]
pub struct UssdWorld {
    phone_number: String,
    pin: String,
    session: Option<UssdSession>,
    last_response: String,
    continue_session: bool,
}

impl UssdWorld {
    fn get_or_create_session(&mut self) -> &mut UssdSession {
        if self.session.is_none() {
            self.session = Some(UssdSession::new(self.phone_number.clone()));
        }
        self.session.as_mut().unwrap()
    }
    
    async fn process_ussd_input(&mut self, input: &str) {
        let session = self.get_or_create_session();
        
        // Call REAL satellite handlers
        let (response, cont) = if input == "*229#" || session.current_menu.is_empty() {
            ussd_handlers::handle_main_menu(input, session).await
        } else {
            // Route to appropriate handler based on current menu
            match session.current_menu.as_str() {
                "bitcoin" => ussd_handlers::handle_bitcoin_menu(input, session).await,
                "usdc" => ussd_handlers::handle_usdc_menu(input, session).await,
                "local_currency" => ussd_handlers::handle_local_currency_menu(input, session).await,
                _ => ("Processing...".to_string(), true),
            }
        };
        
        self.last_response = response;
        self.continue_session = cont;
    }
}

#[given(expr = "I have a phone number {string}")]
async fn have_phone_number(world: &mut UssdWorld, phone: String) {
    world.phone_number = phone;
}

#[given(expr = "I have set my PIN to {string}")]
async fn have_pin(world: &mut UssdWorld, pin: String) {
    world.pin = pin;
    // TODO: Set PIN in mock Juno store
}

#[when(expr = "I dial {string}")]
async fn dial_ussd(world: &mut UssdWorld, code: String) {
    world.process_ussd_input(&code).await;
}

#[when(expr = "I select {string} for {word}")]
async fn select_option(world: &mut UssdWorld, option: String, _description: String) {
    world.process_ussd_input(&option).await;
}

#[when(expr = "I select {string} for {word} {word}")]
async fn select_option_two_words(world: &mut UssdWorld, option: String, _word1: String, _word2: String) {
    world.process_ussd_input(&option).await;
}

#[then(expr = "I should see {string} in USSD response")]
async fn should_see_in_response(world: &mut UssdWorld, expected: String) {
    assert!(
        world.last_response.contains(&expected),
        "Expected to see '{}' in response, but got:\n{}",
        expected,
        world.last_response
    );
}

#[tokio::main]
async fn main() {
    UssdWorld::cucumber()
        .run("tests/features/")
        .await;
}
