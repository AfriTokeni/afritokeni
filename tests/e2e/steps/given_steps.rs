// GIVEN steps - Setup test state

use cucumber::given;
use super::world::UssdWorld;
use crate::mocks::juno_mock::{Balance, Agent, DaoProposal};

#[given(expr = "I have a valid phone number {string}")]
async fn have_phone_number(world: &mut UssdWorld, phone: String) {
    world.phone_number = phone.clone();
    world.juno_store.set_balance(&phone, Balance { kes: 10000.0, ckbtc: 0.001, ckusdc: 100.0 });
}

#[given(expr = "I have set my PIN to {string}")]
async fn have_pin(world: &mut UssdWorld, pin: String) {
    world.pin = pin.clone();
    use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};
    let salt = SaltString::encode_b64(world.phone_number.as_bytes()).unwrap();
    let hash = Argon2::default().hash_password(pin.as_bytes(), &salt).unwrap().to_string();
    world.juno_store.set_user_pin(&world.phone_number, &hash).ok();
}

#[given("I am a registered USSD user")]
async fn registered_user(world: &mut UssdWorld) {
    if world.phone_number.is_empty() {
        world.phone_number = "+256700123456".to_string();
    }
    world.juno_store.set_balance(&world.phone_number, Balance { kes: 10000.0, ckbtc: 0.001, ckusdc: 100.0 });
}

#[given(expr = "the session language should be {string}")]
async fn session_language_is(world: &mut UssdWorld, lang: String) {
    let session = world.get_or_create_session();
    session.language = lang;
}

#[given(expr = "I have a balance of {float} KES")]
async fn have_kes_balance(world: &mut UssdWorld, amount: f64) {
    let mut balance = world.juno_store.get_balance(&world.phone_number)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    balance.kes = amount;
    world.juno_store.set_balance(&world.phone_number, balance);
}

#[given(expr = "I have {float} ckBTC")]
async fn have_btc_balance(world: &mut UssdWorld, amount: f64) {
    let mut balance = world.juno_store.get_balance(&world.phone_number)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    balance.ckbtc = amount;
    world.juno_store.set_balance(&world.phone_number, balance);
}

#[given(expr = "I have {float} ckUSDC")]
async fn have_usdc_balance(world: &mut UssdWorld, amount: f64) {
    let mut balance = world.juno_store.get_balance(&world.phone_number)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    balance.ckusdc = amount;
    world.juno_store.set_balance(&world.phone_number, balance);
}

#[given("there are nearby agents")]
async fn have_nearby_agents(world: &mut UssdWorld) {
    world.juno_store.add_agent(Agent {
        phone: "+256700111111".to_string(),
        name: "John Agent".to_string(),
        location: "Kampala Central".to_string(),
        rating: 4.5,
    });
    world.juno_store.add_agent(Agent {
        phone: "+256700222222".to_string(),
        name: "Mary Agent".to_string(),
        location: "Kampala North".to_string(),
        rating: 4.8,
    });
}

#[given("there are active DAO proposals")]
async fn have_dao_proposals(world: &mut UssdWorld) {
    world.juno_store.add_proposal(DaoProposal {
        id: "prop1".to_string(),
        title: "Increase agent commission".to_string(),
        description: "Proposal to increase commission from 1% to 2%".to_string(),
        votes_for: 150,
        votes_against: 50,
    });
    world.juno_store.add_proposal(DaoProposal {
        id: "prop2".to_string(),
        title: "Add new currency support".to_string(),
        description: "Support for TZS (Tanzanian Shilling)".to_string(),
        votes_for: 200,
        votes_against: 30,
    });
}
