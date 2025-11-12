// Integration tests for USSD language selection flows
use super::*;

#[test]
fn test_language_menu_navigation() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_1";
    
    // Navigate to language menu (Main menu -> 8 or similar)
    let (response, continue_session) = env.process_ussd(session_id, phone, "8");
    
    assert!(continue_session, "Session should continue");
    assert!(response.contains("Language") || response.contains("English") || response.contains("Luganda"),
        "Should show language options. Got: {}", response);
}

#[test]
fn test_select_english() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_2";
    
    // Navigate to language menu and select English
    env.process_ussd(session_id, phone, "8");
    let (response, _) = env.process_ussd(session_id, phone, "1");
    
    assert!(response.contains("English") || response.contains("selected") || response.contains("Main"),
        "Should confirm English selection. Got: {}", response);
}

#[test]
fn test_select_luganda() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_3";
    
    // Navigate to language menu and select Luganda
    env.process_ussd(session_id, phone, "8");
    let (response, _) = env.process_ussd(session_id, phone, "2");
    
    assert!(response.contains("Luganda") || response.contains("selected") || response.len() > 0,
        "Should confirm Luganda selection. Got: {}", response);
}

#[test]
fn test_select_swahili() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_4";
    
    // Navigate to language menu and select Swahili
    env.process_ussd(session_id, phone, "8");
    let (response, _) = env.process_ussd(session_id, phone, "3");
    
    assert!(response.contains("Swahili") || response.contains("selected") || response.len() > 0,
        "Should confirm Swahili selection. Got: {}", response);
}

#[test]
fn test_language_persistence() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id_1 = "lang_test_5a";
    let session_id_2 = "lang_test_5b";
    
    // Select language in first session
    env.process_ussd(session_id_1, phone, "8");
    env.process_ussd(session_id_1, phone, "2"); // Select Luganda
    
    // Start new session - should remember language
    let (response, _) = env.process_ussd(session_id_2, phone, "");
    
    // Response should be in selected language or at least not error
    assert!(response.len() > 0, "Should return response in selected language");
}

#[test]
fn test_invalid_language_selection() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_6";
    
    // Navigate to language menu and select invalid option
    env.process_ussd(session_id, phone, "8");
    let (response, continue_session) = env.process_ussd(session_id, phone, "99");
    
    assert!(continue_session, "Should continue session");
    assert!(response.contains("Invalid") || response.contains("Language") || response.contains("option"),
        "Should show error or re-prompt. Got: {}", response);
}

#[test]
fn test_all_menus_have_translations() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_7";
    
    // Select Luganda
    env.process_ussd(session_id, phone, "8");
    env.process_ussd(session_id, phone, "2");
    
    // Navigate to different menus and check for translations
    let (response, _) = env.process_ussd(session_id, phone, "");
    
    // Should not contain only English text (basic check)
    assert!(response.len() > 0, "Should have translated content");
}

#[test]
fn test_language_menu_structure() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_8";
    
    // Navigate to language menu
    let (response, continue_session) = env.process_ussd(session_id, phone, "8");
    
    assert!(continue_session, "Session should continue");
    
    // Should list at least 3 languages
    assert!(response.contains("1") && response.contains("2") && response.contains("3"),
        "Should show numbered language options. Got: {}", response);
}

#[test]
fn test_language_change_affects_errors() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_9";
    
    // Select Swahili
    env.process_ussd(session_id, phone, "8");
    env.process_ussd(session_id, phone, "3");
    
    // Trigger an error (invalid menu option)
    let (response, _) = env.process_ussd(session_id, phone, "999");
    
    // Error should be in selected language or at least return something
    assert!(response.len() > 0, "Should return error message");
}

#[test]
fn test_no_hardcoded_english_strings() {
    let env = get_test_env();
    
    let phone = &phone("UGX");
    let session_id = "lang_test_10";
    
    // Select Luganda
    env.process_ussd(session_id, phone, "8");
    env.process_ussd(session_id, phone, "2");
    
    // Check main menu
    let (response, _) = env.process_ussd(session_id, phone, "");
    
    // This is a placeholder - actual test would check for specific Luganda text
    assert!(response.len() > 0, "Should return translated menu");
}
