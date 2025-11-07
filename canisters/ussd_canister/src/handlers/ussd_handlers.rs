// Minimal USSD handlers - to be expanded
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle main menu
pub async fn handle_main_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let menu = TranslationService::get_main_menu(lang);
    (menu, true)
}

/// Handle registration
pub async fn handle_registration(session: &mut UssdSession, pin: &str) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    // Validate PIN format
    if !crate::utils::pin::is_valid_pin(pin) {
        return (format!("{}\n0. {}", 
            TranslationService::translate("invalid_pin_format", lang),
            TranslationService::translate("back", lang)), true);
    }
    
    // Hash and store PIN
    let phone = session.phone_number.clone();
    let pin_str = pin.to_string();
    
    ic_cdk::futures::spawn(async move {
        match crate::utils::pin::setup_pin(&phone, &pin_str).await {
            Ok(_) => ic_cdk::println!("✅ PIN set successfully for {}", phone),
            Err(e) => ic_cdk::println!("❌ Failed to set PIN: {}", e),
        }
    });
    
    session.current_menu = "main".to_string();
    (format!("{}\n0. {}", 
        TranslationService::translate("pin_set_success", lang),
        TranslationService::translate("main_menu", lang)), true)
}

/// Handle local currency menu
pub async fn handle_local_currency_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    session.current_menu = "local_currency".to_string();
    
    let menu = format!("{}\n1. {}\n2. {}\n3. {}\n4. {}\n0. {}",
        TranslationService::translate("local_currency_menu", lang),
        TranslationService::translate("check_balance", lang),
        TranslationService::translate("send_money", lang),
        TranslationService::translate("deposit", lang),
        TranslationService::translate("withdraw", lang),
        TranslationService::translate("back", lang));
    
    (menu, true)
}

/// Handle Bitcoin menu
pub async fn handle_bitcoin_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    session.current_menu = "bitcoin".to_string();
    
    let menu = format!("{}\n1. {}\n2. {}\n3. {}\n4. {}\n0. {}",
        TranslationService::translate("bitcoin_menu", lang),
        TranslationService::translate("check_balance", lang),
        TranslationService::translate("buy_bitcoin", lang),
        TranslationService::translate("sell_bitcoin", lang),
        TranslationService::translate("send_bitcoin", lang),
        TranslationService::translate("back", lang));
    
    (menu, true)
}

/// Handle USDC menu
pub async fn handle_usdc_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    session.current_menu = "usdc".to_string();
    
    let menu = format!("{}\n1. {}\n2. {}\n3. {}\n0. {}",
        TranslationService::translate("usdc_menu", lang),
        TranslationService::translate("check_balance", lang),
        TranslationService::translate("send_usdc", lang),
        TranslationService::translate("swap_to_kes", lang),
        TranslationService::translate("back", lang));
    
    (menu, true)
}

/// Handle DAO menu
pub async fn handle_dao_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    session.current_menu = "dao".to_string();
    
    let menu = format!("{}\n1. {}\n2. {}\n3. {}\n0. {}",
        TranslationService::translate("dao_menu", lang),
        TranslationService::translate("view_proposals", lang),
        TranslationService::translate("vote", lang),
        TranslationService::translate("create_proposal", lang),
        TranslationService::translate("back", lang));
    
    (menu, true)
}

/// Handle language menu
pub async fn handle_language_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    // If no input yet, show language menu
    if text.is_empty() || session.current_menu != "language" {
        session.current_menu = "language".to_string();
        let menu = format!("{}\n1. English\n2. Luganda\n3. Swahili\n0. {}",
            TranslationService::translate("select_language", lang),
            TranslationService::translate("back", lang));
        return (menu, true);
    }
    
    // Process language selection
    let parts: Vec<&str> = text.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "1" => {
            session.language = "en".to_string();
            let phone = session.phone_number.clone();
            // Save language preference to datastore
            ic_cdk::futures::spawn(async move {
                let _ = crate::utils::datastore::set_user_language(&phone, "en").await;
            });
            session.current_menu = String::new();
            (format!("{}\n0. {}", 
                TranslationService::translate("language_set", Language::English),
                TranslationService::translate("main_menu", Language::English)), true)
        }
        "2" => {
            session.language = "lg".to_string();
            let phone = session.phone_number.clone();
            ic_cdk::futures::spawn(async move {
                let _ = crate::utils::datastore::set_user_language(&phone, "lg").await;
            });
            session.current_menu = String::new();
            (format!("{}\n0. {}", 
                TranslationService::translate("language_set", Language::Luganda),
                TranslationService::translate("main_menu", Language::Luganda)), true)
        }
        "3" => {
            session.language = "sw".to_string();
            let phone = session.phone_number.clone();
            ic_cdk::futures::spawn(async move {
                let _ = crate::utils::datastore::set_user_language(&phone, "sw").await;
            });
            session.current_menu = String::new();
            (format!("{}\n0. {}", 
                TranslationService::translate("language_set", Language::Swahili),
                TranslationService::translate("main_menu", Language::Swahili)), true)
        }
        "0" => {
            session.current_menu = String::new();
            handle_main_menu("", session).await
        }
        _ => {
            (format!("{}\n0. {}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back", lang)), true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_menu() {
        let mut session = UssdSession {
            session_id: "test".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: "".to_string(),
            step: 0,
            language: "en".to_string(),
            last_activity: 0,
        };
        
        let (response, cont) = handle_main_menu("", &mut session).await;
        assert!(cont);
        assert!(response.contains("Welcome"));
        assert!(response.contains("Bitcoin"));
    }

    #[tokio::test]
    async fn test_registration_invalid_pin() {
        let mut session = UssdSession {
            session_id: "test".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: "register".to_string(),
            step: 0,
            language: "en".to_string(),
            last_activity: 0,
        };
        
        let (response, cont) = handle_registration(&mut session, "123").await;
        assert!(cont);
        assert!(response.contains("Invalid PIN"));
    }
}
