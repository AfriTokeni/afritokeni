// THEN steps - Assertions

use cucumber::then;
use super::world::UssdWorld;

#[then(expr = "I should see {string} in USSD response")]
async fn should_see_in_response(world: &mut UssdWorld, expected: String) {
    assert!(
        world.last_response.contains(&expected),
        "Expected to see '{}' in response, but got:\n{}",
        expected,
        world.last_response
    );
}

#[then(expr = "the response should contain {string}")]
async fn response_should_contain(world: &mut UssdWorld, expected: String) {
    assert!(
        world.last_response.contains(&expected),
        "Expected response to contain '{}', but got:\n{}",
        expected,
        world.last_response
    );
}

#[then(expr = "the response should be {string}")]
async fn response_should_be(world: &mut UssdWorld, expected: String) {
    assert_eq!(
        world.last_response, expected,
        "Expected exact response: '{}', but got:\n{}",
        expected, world.last_response
    );
}

#[then(expr = "the session language should be {string}")]
async fn check_session_language(world: &mut UssdWorld, expected_lang: String) {
    let session = world.get_or_create_session();
    assert_eq!(
        session.language, expected_lang,
        "Expected language '{}', but got '{}'",
        expected_lang, session.language
    );
}

#[then(expr = "the session should end")]
async fn session_should_end(world: &mut UssdWorld) {
    assert!(
        !world.continue_session || world.last_response.contains("Thank you") || world.last_response.contains("Goodbye"),
        "Expected session to end, but got: {}",
        world.last_response
    );
}
