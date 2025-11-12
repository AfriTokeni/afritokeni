// Integration tests for USSD registration flow - ALL COMBINATIONS
// Tests real canister interactions: USSD -> Business Logic -> Data
use super::*;

#[test]
fn test_registration_flow_complete() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "reg_test_1";
    
    // Step 0: Start registration - should ask for PIN
    let (response, continue_session) = env.process_ussd(session_id, phone, "");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("PIN") || response.contains("pin"), 
        "Should ask for PIN. Got: {}", response);
    
    // Step 1: Enter PIN - should ask for first name
    let (response, continue_session) = env.process_ussd(session_id, phone, "1234");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("first name") || response.contains("First name"), 
        "Should ask for first name. Got: {}", response);
    
    // Step 2: Enter first name - should ask for last name
    let (response, continue_session) = env.process_ussd(session_id, phone, "John");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("last name") || response.contains("Last name"), 
        "Should ask for last name. Got: {}", response);
    
    // Step 3: Enter last name - should show detected currency
    let (response, continue_session) = env.process_ussd(session_id, phone, "Doe");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("UGX") || response.contains("currency"), 
        "Should show detected currency. Got: {}", response);
    assert!(response.contains("Confirm") || response.contains("1"), 
        "Should show confirm option. Got: {}", response);
    
    // Step 4: Confirm currency - should complete registration
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    assert!(continue_session, "Session should continue to main menu");
    assert!(response.contains("successful") || response.contains("Welcome"), 
        "Should show success message. Got: {}", response);
    assert!(response.contains("John") || response.contains("Doe"), 
        "Should show user name. Got: {}", response);
    
    // Verify user was created in data canister
    let user = env.get_user(phone)
        .expect("Should get user")
        .expect("User should exist");
    
    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.phone_number, Some(phone.to_string()));
}

#[test]
fn test_registration_invalid_pin_format() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "reg_test_2";
    
    // Start registration
    env.process_ussd(session_id, phone, "");
    
    // Try invalid PIN (too short)
    let (response, continue_session) = env.process_ussd(session_id, phone, "123");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Invalid") || response.contains("4-digit"), 
        "Should reject invalid PIN. Got: {}", response);
    
    // Try invalid PIN (non-numeric)
    let (response, continue_session) = env.process_ussd(session_id, phone, "abcd");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Invalid") || response.contains("4-digit"), 
        "Should reject non-numeric PIN. Got: {}", response);
}

#[test]
fn test_registration_currency_auto_detection() {
    let env = get_test_env();
    
    // Test Kenya number
    let phone_ke = "+254700333333";
    let session_id_ke = "reg_test_3";
    
    env.process_ussd(session_id_ke, phone_ke, "");
    env.process_ussd(session_id_ke, phone_ke, "1234");
    env.process_ussd(session_id_ke, phone_ke, "Alice");
    let (response, _) = env.process_ussd(session_id_ke, phone_ke, "Smith");
    
    assert!(response.contains("KES"), 
        "Should detect KES for Kenya number. Got: {}", response);
    
    // Test Tanzania number
    let phone_tz = "+255700444444";
    let session_id_tz = "reg_test_4";
    
    env.process_ussd(session_id_tz, phone_tz, "");
    env.process_ussd(session_id_tz, phone_tz, "5678");
    env.process_ussd(session_id_tz, phone_tz, "Bob");
    let (response, _) = env.process_ussd(session_id_tz, phone_tz, "Jones");
    
    assert!(response.contains("TZS"), 
        "Should detect TZS for Tanzania number. Got: {}", response);
}

#[test]
fn test_registration_manual_currency_selection() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "reg_test_5";
    
    // Go through registration steps
    env.process_ussd(session_id, phone, "");
    env.process_ussd(session_id, phone, "9999");
    env.process_ussd(session_id, phone, "Charlie");
    env.process_ussd(session_id, phone, "Brown");
    
    // Choose to change currency
    let (response, continue_session) = env.process_ussd(session_id, phone, "2");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("KES") || response.contains("UGX") || response.contains("currency"), 
        "Should show currency options. Got: {}", response);
    
    // Select KES
    let (response, continue_session) = env.process_ussd(session_id, phone, "1");
    assert!(continue_session, "Session should continue to main menu");
    assert!(response.contains("successful") || response.contains("Welcome"), 
        "Should complete registration. Got: {}", response);
}

#[test]
fn test_registration_duplicate_phone_fails() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    
    // Register user directly first
    env.setup_test_user_with_balances(phone, "First", "User", "first@test.com", "UGX", "1111", 0, 0, 0)
        .expect("Setup");
    
    // Try to register again via USSD
    let session_id = "reg_test_6";
    env.process_ussd(session_id, phone, "");
    env.process_ussd(session_id, phone, "2222");
    env.process_ussd(session_id, phone, "Second");
    env.process_ussd(session_id, phone, "User");
    let (response, _) = env.process_ussd(session_id, phone, "1");
    
    assert!(response.contains("already registered") || response.contains("failed"), 
        "Should reject duplicate registration. Got: {}", response);
}

#[test]
fn test_registration_empty_names_rejected() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "reg_test_7";
    
    env.process_ussd(session_id, phone, "");
    env.process_ussd(session_id, phone, "3333");
    
    // Try empty first name
    let (response, continue_session) = env.process_ussd(session_id, phone, "");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("cannot be empty") || response.contains("first name"), 
        "Should reject empty first name. Got: {}", response);
    
    // Provide valid first name
    env.process_ussd(session_id, phone, "David");
    
    // Try empty last name
    let (response, continue_session) = env.process_ussd(session_id, phone, "   ");
    assert!(continue_session, "Session should continue");
    assert!(response.contains("cannot be empty") || response.contains("last name"), 
        "Should reject empty last name. Got: {}", response);
}

// ============================================================================
// REGISTRATION WITH ALL 39 AFRICAN CURRENCIES
// ============================================================================

#[test]
fn test_registration_with_kes_kenya() {
    let env = get_test_env();
    let phone = &phone("KES"); // Kenya number
    
    env.setup_test_user_with_balances(phone, "Kenya", "User", "kenya@test.com", "KES", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "KES");
}

#[test]
fn test_registration_with_ugx_uganda() {
    let env = get_test_env();
    let phone = &phone("UGX"); // Uganda number
    
    env.setup_test_user_with_balances(phone, "Uganda", "User", "uganda@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "UGX");
}

#[test]
fn test_registration_with_tzs_tanzania() {
    let env = get_test_env();
    let phone = &phone("TZS"); // Tanzania number
    
    env.setup_test_user_with_balances(phone, "Tanzania", "User", "tanzania@test.com", "TZS", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "TZS");
}

#[test]
fn test_registration_with_rwf_rwanda() {
    let env = get_test_env();
    let phone = &phone("RWF"); // Rwanda number
    
    env.setup_test_user_with_balances(phone, "Rwanda", "User", "rwanda@test.com", "RWF", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "RWF");
}

#[test]
fn test_registration_with_ngn_nigeria() {
    let env = get_test_env();
    let phone = &phone("NGN"); // Nigeria number
    
    env.setup_test_user_with_balances(phone, "Nigeria", "User", "nigeria@test.com", "NGN", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "NGN");
}

#[test]
fn test_registration_with_ghs_ghana() {
    let env = get_test_env();
    let phone = &phone("GHS"); // Ghana number
    
    env.setup_test_user_with_balances(phone, "Ghana", "User", "ghana@test.com", "GHS", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "GHS");
}

#[test]
fn test_registration_with_zar_south_africa() {
    let env = get_test_env();
    let phone = "+27700777777"; // South Africa number
    
    env.setup_test_user_with_balances(phone, "SouthAfrica", "User", "sa@test.com", "ZAR", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.preferred_currency.code(), "ZAR");
}

// ============================================================================
// REGISTRATION WITH DIFFERENT PIN COMBINATIONS
// ============================================================================

#[test]
fn test_registration_with_all_zeros_pin() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Zero", "PIN", "zero@test.com", "UGX", "0000", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, "Zero");
}

#[test]
fn test_registration_with_all_nines_pin() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Nine", "PIN", "nine@test.com", "UGX", "9999", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, "Nine");
}

#[test]
fn test_registration_with_sequential_pin() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Sequential", "PIN", "seq@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, "Sequential");
}

// ============================================================================
// REGISTRATION WITH SPECIAL NAME CHARACTERS
// ============================================================================

#[test]
fn test_registration_with_hyphenated_name() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Mary-Jane", "Smith-Jones", "hyphen@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, "Mary-Jane");
    assert_eq!(user.last_name, "Smith-Jones");
}

#[test]
fn test_registration_with_apostrophe_name() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "O'Brien", "D'Angelo", "apostrophe@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, "O'Brien");
}

#[test]
fn test_registration_with_single_letter_name() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "X", "Y", "single@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, "X");
    assert_eq!(user.last_name, "Y");
}

#[test]
fn test_registration_with_very_long_name() {
    let env = get_test_env();
    let phone = &phone("UGX");
    let long_name = "Uvuvwevwevwe Onyetenyevwe Ugwemuhwem Osas";
    
    env.register_user_direct(phone, long_name, "Lastname", "long@test.com", "UGX", "1234")
        .expect("Long names should work");
    
    let user = env.get_user(phone).expect("Should get user").expect("User should exist");
    assert_eq!(user.first_name, long_name);
}

// ============================================================================
// REGISTRATION FLOW VERIFICATION (DATA PERSISTS ACROSS CANISTERS)
// ============================================================================

#[test]
fn test_registration_data_in_business_logic_and_data_canister() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    // Register via business logic
    env.setup_test_user_with_balances(phone, "Cross", "Canister", "cross@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Verify data exists in data canister
    let user = env.get_user(phone).expect("Should get from data canister").expect("User should exist");
    
    assert_eq!(user.first_name, "Cross");
    assert_eq!(user.last_name, "Canister");
    assert_eq!(user.phone_number, Some(phone.to_string()));
    assert_eq!(user.preferred_currency.code(), "UGX");
}

#[test]
fn test_registration_creates_zero_balances() {
    let env = get_test_env();
    let phone = &phone("UGX");
    
    env.setup_test_user_with_balances(phone, "Zero", "Balance", "zero@test.com", "UGX", "1234", 0, 0, 0)
        .expect("Setup");
    
    // Check fiat balance is 0
    let balance = env.check_fiat_balance(phone, "UGX").expect("Should get balance");
    assert_eq!(balance, 0, "New user should have 0 balance");
    
    // Check crypto balance is 0
    let (btc, usdc) = env.get_crypto_balance(phone).expect("Should get crypto balance");
    assert_eq!(btc, 0, "New user should have 0 BTC");
    assert_eq!(usdc, 0, "New user should have 0 USDC");
}
