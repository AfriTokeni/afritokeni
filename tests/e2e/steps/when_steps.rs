// WHEN steps - User actions

use cucumber::when;
use super::world::UssdWorld;

#[when(expr = "I dial {string}")]
async fn dial_ussd(world: &mut UssdWorld, code: String) {
    world.process_ussd_input(&code).await;
}

#[when(expr = "I select {string} for {word}")]
async fn select_option_for(world: &mut UssdWorld, option: String, _description: String) {
    world.process_ussd_input(&option).await;
}

#[when(expr = "I select {string} to go back")]
async fn select_to_go_back(world: &mut UssdWorld, option: String) {
    world.process_ussd_input(&option).await;
}

#[when(expr = "I select {string} to show current menu")]
async fn select_to_show_menu(world: &mut UssdWorld, option: String) {
    world.process_ussd_input(&option).await;
}

#[when(expr = "I select {string} for invalid option")]
async fn select_invalid_option(world: &mut UssdWorld, option: String) {
    world.process_ussd_input(&option).await;
}

#[when(expr = "I enter {string}")]
async fn enter_input(world: &mut UssdWorld, input: String) {
    world.process_ussd_input(&input).await;
}
