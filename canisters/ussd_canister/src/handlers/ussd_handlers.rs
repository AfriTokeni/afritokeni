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
pub async fn handle_local_currency_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    // Parse the text to determine which submenu we're in
    let parts: Vec<&str> = text.split('*').collect();
    ic_cdk::println!("🔍 Local currency menu: parts={:?}, len={}", parts, parts.len());
    
    // If we have more than 2 parts (e.g., "1*2*phone"), we're in a flow
    if parts.len() > 2 {
        ic_cdk::println!("🔍 Checking flow routing, parts[1]={:?}", parts.get(1));
        // Determine which flow based on second part
        match parts.get(1) {
            Some(&"2") => {
                // Send money flow
                ic_cdk::println!("✅ Routing to send_money flow");
                session.current_menu = "send_money".to_string();
                return crate::handlers::send_money_flow::handle_send_money(text, session).await;
            }
            Some(&"4") => {
                // Withdraw flow
                session.current_menu = "withdraw".to_string();
                return crate::handlers::withdraw_flow::handle_withdraw(text, session).await;
            }
            _ => {}
        }
    }
    
    let last_input = parts.last().unwrap_or(&"");
    
    match *last_input {
        "1" => {
            // Check balance
            let balance = crate::utils::datastore::get_balance(&session.phone_number).await
                .unwrap_or_default();
            session.current_menu = String::new();
            (format!("{}:\nKES: {:.2}\nckBTC: {:.8}\nckUSDC: {:.2}\n\n0. {}", 
                TranslationService::translate("your_balance", lang),
                balance.kes,
                balance.ckbtc,
                balance.ckusdc,
                TranslationService::translate("main_menu", lang)), true)
        }
        "2" => {
            // Send money - start the flow
            session.step = 0;
            crate::handlers::send_money_flow::handle_send_money(text, session).await
        }
        "4" => {
            // Withdraw - start the flow
            session.step = 0;
            crate::handlers::withdraw_flow::handle_withdraw(text, session).await
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

/// Handle Bitcoin menu
pub async fn handle_bitcoin_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    let parts: Vec<&str> = text.split('*').collect();
    
    // If we have more than 2 parts, we're in a flow
    if parts.len() > 2 {
        match parts.get(1) {
            Some(&"2") => {
                // Buy Bitcoin flow (option 2 in bitcoin menu)
                return crate::handlers::buy_bitcoin_flow::handle_buy_bitcoin(text, session).await;
            }
            Some(&"4") => {
                // Send Bitcoin flow (option 4 in bitcoin menu)
                return crate::handlers::send_bitcoin_flow::handle_send_bitcoin(text, session).await;
            }
            _ => {}
        }
    }
    
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "2" => {
            // Show Bitcoin menu (when text is just "2")
            let menu = format!("{}\n1. {}\n2. {}\n3. {}\n4. {}\n0. {}",
                TranslationService::translate("bitcoin_menu", lang),
                TranslationService::translate("check_balance", lang),
                TranslationService::translate("buy_bitcoin", lang),
                TranslationService::translate("sell_bitcoin", lang),
                TranslationService::translate("send_bitcoin", lang),
                TranslationService::translate("back", lang));
            (menu, true)
        }
        "1" => {
            // Check balance (when text is "2*1")
            let balance = crate::utils::datastore::get_balance(&session.phone_number).await
                .unwrap_or_default();
            (format!("{}:\nckBTC: {:.8}\n\n0. {}", 
                TranslationService::translate("bitcoin_balance", lang),
                balance.ckbtc,
                TranslationService::translate("main_menu", lang)), false)
        }
        "2" => {
            // Buy Bitcoin - start the flow (when text is "2*2")
            crate::handlers::buy_bitcoin_flow::handle_buy_bitcoin(text, session).await
        }
        "4" => {
            // Send Bitcoin - start the flow (when text is "2*4")
            crate::handlers::send_bitcoin_flow::handle_send_bitcoin(text, session).await
        }
        _ => {
            (format!("{}\n0. {}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back", lang)), true)
        }
    }
}

/// Handle USDC menu
pub async fn handle_usdc_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    let parts: Vec<&str> = text.split('*').collect();
    
    // If we have more than 2 parts, we're in buy flow
    if parts.len() > 2 {
        if let Some(&"2") = parts.get(1) {
            return crate::handlers::buy_usdc_flow::handle_buy_usdc(text, session).await;
        }
    }
    
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "3" => {
            // Show USDC menu (when text is just "3")
            let menu = format!("{}\n1. {}\n2. {}\n3. {}\n0. {}",
                TranslationService::translate("usdc_menu", lang),
                TranslationService::translate("check_balance", lang),
                TranslationService::translate("buy_usdc", lang),
                TranslationService::translate("send_usdc", lang),
                TranslationService::translate("back", lang));
            (menu, true)
        }
        "1" => {
            // Check balance (when text is "3*1")
            let balance = crate::utils::datastore::get_balance(&session.phone_number).await
                .unwrap_or_default();
            (format!("{}:\nckUSDC: {:.2}\n\n0. {}", 
                TranslationService::translate("usdc_balance", lang),
                balance.ckusdc,
                TranslationService::translate("main_menu", lang)), false)
        }
        "2" => {
            // Buy USDC - start the flow (when text is "3*2")
            crate::handlers::buy_usdc_flow::handle_buy_usdc(text, session).await
        }
        _ => {
            (format!("{}\n0. {}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back", lang)), true)
        }
    }
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
    
    let parts: Vec<&str> = text.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "5" => {
            // Show language menu (when text is just "5")
            let menu = format!("{}\n1. English\n2. Luganda\n3. Swahili\n0. {}",
                TranslationService::translate("select_language", lang),
                TranslationService::translate("back", lang));
            (menu, true)
        }
        "1" => {
            let new_lang = Language::English;
            session.language = new_lang.to_code().to_string();
            let phone = session.phone_number.clone();
            let lang_code = new_lang.to_code();
            // Save language preference to datastore
            let _ = crate::utils::datastore::set_user_language(&phone, lang_code).await;
            (format!("{}\n0. {}", 
                TranslationService::translate("language_set", new_lang),
                TranslationService::translate("main_menu", new_lang)), false)
        }
        "2" => {
            let new_lang = Language::Luganda;
            session.language = new_lang.to_code().to_string();
            let phone = session.phone_number.clone();
            let lang_code = new_lang.to_code();
            let _ = crate::utils::datastore::set_user_language(&phone, lang_code).await;
            (format!("{}\n0. {}", 
                TranslationService::translate("language_set", new_lang),
                TranslationService::translate("main_menu", new_lang)), false)
        }
        "3" => {
            let new_lang = Language::Swahili;
            session.language = new_lang.to_code().to_string();
            let phone = session.phone_number.clone();
            let lang_code = new_lang.to_code();
            let _ = crate::utils::datastore::set_user_language(&phone, lang_code).await;
            (format!("{}\n0. {}", 
                TranslationService::translate("language_set", new_lang),
                TranslationService::translate("main_menu", new_lang)), false)
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
