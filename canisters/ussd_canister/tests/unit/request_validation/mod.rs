// Request validation unit tests
// These test HTTP request parsing and structure validation
// NOT actual integration tests (those are in /tests/integration/ with BDD)

#[path = "request_validation_main_menu_tests.rs"]
mod main_menu_tests;

#[path = "request_validation_local_currency_tests.rs"]
mod local_currency_tests;

#[path = "request_validation_bitcoin_tests.rs"]
mod bitcoin_tests;

#[path = "request_validation_usdc_tests.rs"]
mod usdc_tests;

#[path = "request_validation_dao_tests.rs"]
mod dao_tests;

#[path = "request_validation_language_tests.rs"]
mod language_tests;
