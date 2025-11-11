// Minimal USSD handlers - to be expanded
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle main menu - just show the menu, routing is handled in ussd.rs
pub async fn handle_main_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    // Default to UGX if no currency set
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    let menu = TranslationService::get_main_menu(lang, &currency);
    (menu, true)
}

/// Handle multi-step registration
/// Step 0: PIN, Step 1: First name, Step 2: Last name, Step 3: Currency
pub async fn handle_registration(session: &mut UssdSession, input: &str) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            // Step 0: Collect PIN
            if input.len() != 4 || !input.chars().all(|c| c.is_numeric()) {
                return (format!("{}\n\n{}", 
                    TranslationService::translate("invalid_pin_format", lang),
                    "Enter 4-digit PIN:"), true);
            }
            session.set_data("pin", input);
            session.step = 1;
            (String::from("Enter your first name:"), true)
        }
        1 => {
            // Step 1: Collect first name
            if input.trim().is_empty() {
                return (String::from("First name cannot be empty.\n\nEnter your first name:"), true);
            }
            session.set_data("first_name", input.trim());
            session.step = 2;
            (String::from("Enter your last name:"), true)
        }
        2 => {
            // Step 2: Collect last name and auto-detect currency
            if input.trim().is_empty() {
                return (String::from("Last name cannot be empty.\n\nEnter your last name:"), true);
            }
            session.set_data("last_name", input.trim());
            
            // Auto-detect currency from phone number
            let detected_currency = detect_currency_from_phone(&session.phone_number);
            session.set_data("currency", &detected_currency);
            session.step = 3;
            
            (format!("Detected currency: {}\n\n1. Confirm\n2. Change currency", detected_currency), true)
        }
        3 => {
            // Step 3: Confirm or change currency
            let currency = if input == "1" {
                // Confirm detected currency
                session.get_data("currency").unwrap_or_else(|| "KES".to_string())
            } else if input == "2" {
                // Show currency selection
                session.step = 4; // Go to currency selection step
                return (String::from("Select your currency:\n1. KES (Kenya)\n2. UGX (Uganda)\n3. TZS (Tanzania)\n4. RWF (Rwanda)\n5. NGN (Nigeria)\n6. GHS (Ghana)\n7. ZAR (South Africa)"), true);
            } else {
                return (String::from("Invalid choice.\n\n1. Confirm\n2. Change currency"), true);
            };
            
            // Get collected data
            let pin = session.get_data("pin").unwrap_or_default();
            let first_name = session.get_data("first_name").unwrap_or_default();
            let last_name = session.get_data("last_name").unwrap_or_default();
            
            // Register user (USSD users don't have email, Business Logic will handle)
            match crate::services::business_logic::register_user(
                &session.phone_number,
                &first_name,
                &last_name,
                "ussd@afritokeni.com",
                &pin,
                &currency
            ).await {
                Ok(_user_id) => {
                    ic_cdk::println!("✅ User registered: {} {} ({})", first_name, last_name, session.phone_number);
                    session.current_menu = "main".to_string();
                    session.step = 0;
                    session.clear_data();
                    
                    // Show main menu with proper translations
                    let menu = format!("✅ Registration successful!\n\nWelcome {} {}!\n\n{}", 
                        first_name, last_name,
                        TranslationService::get_main_menu(lang, &currency));
                    (menu, true)
                }
                Err(e) => {
                    // Log detailed error for debugging and monitoring
                    ic_cdk::println!("❌ CRITICAL: Registration failed for {} - Error: {}", session.phone_number, e);
                    ic_cdk::println!("   Details: name={} {}, currency={}", first_name, last_name, currency);
                    
                    session.clear_data();
                    
                    // User-friendly error message (hide technical details)
                    let user_message = if e.contains("already registered") {
                        "This phone number is already registered.\n\nPlease contact support if you need help.".to_string()
                    } else {
                        "We're sorry, registration failed due to a technical issue.\n\nPlease try again later or contact support.".to_string()
                    };
                    
                    (user_message, true)
                }
            }
        }
        4 => {
            // Step 4: Manual currency selection (if user chose to change)
            let currency = match input {
                "1" => "KES",
                "2" => "UGX",
                "3" => "TZS",
                "4" => "RWF",
                "5" => "NGN",
                "6" => "GHS",
                "7" => "ZAR",
                _ => {
                    return (String::from("Invalid choice.\n\nSelect your currency:\n1. KES\n2. UGX\n3. TZS\n4. RWF\n5. NGN\n6. GHS\n7. ZAR"), true);
                }
            };
            
            session.set_data("currency", currency);
            
            // Get collected data
            let pin = session.get_data("pin").unwrap_or_default();
            let first_name = session.get_data("first_name").unwrap_or_default();
            let last_name = session.get_data("last_name").unwrap_or_default();
            
            // Register user (USSD users don't have email, Business Logic will handle)
            match crate::services::business_logic::register_user(
                &session.phone_number,
                &first_name,
                &last_name,
                "ussd@afritokeni.com",
                &pin,
                &currency
            ).await {
                Ok(_user_id) => {
                    ic_cdk::println!("✅ User registered: {} {} ({})", first_name, last_name, session.phone_number);
                    session.current_menu = "main".to_string();
                    session.step = 0;
                    session.clear_data();
                    
                    // Show main menu with proper translations
                    let menu = format!("✅ Registration successful!\n\nWelcome {} {}!\n\n{}", 
                        first_name, last_name,
                        TranslationService::get_main_menu(lang, &currency));
                    (menu, true)
                }
                Err(e) => {
                    // Log detailed error for debugging and monitoring
                    ic_cdk::println!("❌ CRITICAL: Registration failed for {} - Error: {}", session.phone_number, e);
                    ic_cdk::println!("   Details: name={} {}, currency={}", first_name, last_name, currency);
                    
                    session.clear_data();
                    
                    // User-friendly error message (hide technical details)
                    let user_message = if e.contains("already registered") {
                        "This phone number is already registered.\n\nPlease contact support if you need help.".to_string()
                    } else {
                        "We're sorry, registration failed due to a technical issue.\n\nPlease try again later or contact support.".to_string()
                    };
                    
                    (user_message, true)
                }
            }
        }
        _ => {
            // Reset if in invalid state
            session.step = 0;
            session.clear_data();
            (String::from("Welcome to AfriTokeni!\n\nTo get started, please set your 4-digit PIN:\n\nEnter PIN"), true)
        }
    }
}

/// Detect currency from phone number (simple country code detection)
fn detect_currency_from_phone(phone: &str) -> String {
    let phone = phone.trim_start_matches('+');
    
    // African country codes and their currencies
    if phone.starts_with("256") { "UGX".to_string() } // Uganda
    else if phone.starts_with("254") { "KES".to_string() } // Kenya
    else if phone.starts_with("255") { "TZS".to_string() } // Tanzania
    else if phone.starts_with("250") { "RWF".to_string() } // Rwanda
    else if phone.starts_with("234") { "NGN".to_string() } // Nigeria
    else if phone.starts_with("233") { "GHS".to_string() } // Ghana
    else if phone.starts_with("27") { "ZAR".to_string() } // South Africa
    else { "UGX".to_string() } // Default to UGX
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
                return crate::flows::local_currency::send_money::handle_send_money(text, session).await;
            }
            Some(&"4") => {
                // Withdraw flow
                session.current_menu = "withdraw".to_string();
                return crate::flows::local_currency::withdraw::handle_withdraw(text, session).await;
            }
            _ => {}
        }
    }
    
    let last_input = parts.last().unwrap_or(&"");
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    match *last_input {
        "1" if parts.len() == 1 => {
            // Show Local Currency menu (when text is just "1")
            let menu = format!("{} ({})\n{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n6. {}\n\n{}",
                TranslationService::translate("local_currency_menu", lang),
                currency,
                TranslationService::translate("please_select_option", lang),
                TranslationService::translate("send_money", lang),
                TranslationService::translate("check_balance", lang),
                TranslationService::translate("deposit", lang),
                TranslationService::translate("withdraw", lang),
                TranslationService::translate("transactions", lang),
                TranslationService::translate("find_agent", lang),
                TranslationService::translate("back_or_menu", lang));
            (menu, true)
        }
        "2" if parts.len() == 2 => {
            // Check balance (when text is "1*2")
            match crate::services::business_logic::get_balances(&session.phone_number).await {
                Ok(balances) => {
                    let fiat_amount = match balances.fiat_balances.iter()
                        .find(|b| format!("{:?}", b.currency) == currency) {
                        Some(balance) => balance.balance as f64 / 100.0,
                        None => {
                            return (format!("{}: {} {}\n\n{}", 
                                TranslationService::translate("error", lang),
                                TranslationService::translate("currency_not_found", lang),
                                currency,
                                TranslationService::translate("back_or_menu", lang)), true);
                        }
                    };
                    let ckbtc = balances.ckbtc_balance as f64 / 100_000_000.0;
                    let ckusdc = balances.ckusdc_balance as f64 / 1_000_000.0;
                    
                    (format!("{}:\n{}: {:.2}\nckBTC: {:.8}\nckUSDC: {:.2}\n\n{}", 
                        TranslationService::translate("your_balance", lang),
                        currency,
                        fiat_amount, ckbtc, ckusdc,
                        TranslationService::translate("back_or_menu", lang)), true)
                }
                Err(e) => {
                    (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("back_or_menu", lang)), true)
                }
            }
        }
        "1" if parts.len() == 2 => {
            // Send money - start the flow (when text is "1*1")
            session.step = 0;
            session.current_menu = "send_money".to_string();
            crate::flows::local_currency::send_money::handle_send_money(text, session).await
        }
        "3" if parts.len() == 2 => {
            // Deposit - start the flow (when text is "1*3")
            session.step = 0;
            session.current_menu = "deposit".to_string();
            crate::flows::local_currency::deposit::handle_deposit(text, session).await
        }
        "4" if parts.len() == 2 => {
            // Withdraw - start the flow (when text is "1*4")
            session.step = 0;
            session.current_menu = "withdraw".to_string();
            crate::flows::local_currency::withdraw::handle_withdraw(text, session).await
        }
        "5" if parts.len() == 2 => {
            // Transactions history (when text is "1*5")
            crate::flows::common::transactions::handle_transactions(text, session).await
        }
        "6" if parts.len() == 2 => {
            // Find agent (when text is "1*6")
            crate::flows::common::find_agent::handle_find_agent(text, session).await
        }
        _ => {
            (format!("{}\n0. {}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back_or_menu", lang)), true)
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
            Some(&"3") => {
                // Buy Bitcoin flow
                return crate::flows::bitcoin::buy::handle_buy_bitcoin(text, session).await;
            }
            Some(&"5") => {
                // Send Bitcoin flow
                return crate::flows::bitcoin::send::handle_send_bitcoin(text, session).await;
            }
            _ => {}
        }
    }
    
    let last_input = parts.last().unwrap_or(&"");
    
    match *last_input {
        "2" if parts.len() == 1 => {
            // Show Bitcoin menu (when text is just "2")
            let menu = format!("{}\n{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n\n{}",
                TranslationService::translate("bitcoin_menu_title", lang),
                TranslationService::translate("please_select_option", lang),
                TranslationService::translate("check_balance", lang),
                TranslationService::translate("bitcoin_rate", lang),
                TranslationService::translate("buy_bitcoin", lang),
                TranslationService::translate("sell_bitcoin", lang),
                TranslationService::translate("send_bitcoin", lang),
                TranslationService::translate("back_or_menu", lang));
            (menu, true)
        }
        "1" if parts.len() == 2 => {
            // Check balance (when text is "2*1")
            match crate::services::business_logic::get_balances(&session.phone_number).await {
                Ok(balances) => {
                    let ckbtc = balances.ckbtc_balance as f64 / 100_000_000.0;
                    (format!("{}:\nckBTC: {:.8}\n\n{}", 
                        TranslationService::translate("bitcoin_balance", lang),
                        ckbtc,
                        TranslationService::translate("back_or_menu", lang)), true)
                }
                Err(_) => {
                    (format!("{}:\nckBTC: 0.00000000\n\n{}", 
                        TranslationService::translate("bitcoin_balance", lang),
                        TranslationService::translate("back_or_menu", lang)), true)
                }
            }
        }
        "2" if parts.len() == 2 => {
            // Bitcoin rate (when text is "2*2")
            crate::flows::common::bitcoin_rate::handle_bitcoin_rate(text, session).await
        }
        "3" if parts.len() == 2 => {
            // Buy Bitcoin - start the flow (when text is "2*3")
            session.step = 0;
            session.current_menu = "buy_bitcoin".to_string();
            crate::flows::bitcoin::buy::handle_buy_bitcoin(text, session).await
        }
        "4" if parts.len() == 2 => {
            // Sell Bitcoin (when text is "2*4")
            session.step = 0;
            session.current_menu = "sell_bitcoin".to_string();
            crate::flows::bitcoin::sell::handle_sell_bitcoin(text, session).await
        }
        "5" if parts.len() == 2 => {
            // Send Bitcoin - start the flow (when text is "2*5")
            session.step = 0;
            session.current_menu = "send_bitcoin".to_string();
            crate::flows::bitcoin::send::handle_send_bitcoin(text, session).await
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}

/// Handle USDC menu
pub async fn handle_usdc_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    let parts: Vec<&str> = text.split('*').collect();
    let last_input = parts.last().unwrap_or(&"");
    
    // If we have more than 2 parts, we're in buy flow
    if parts.len() > 2 {
        if let Some(&"2") = parts.get(1) {
            return crate::flows::usdc::buy::handle_buy_usdc(text, session).await;
        }
    }
    
    match *last_input {
        "3" if parts.len() == 1 => {
            // Show USDC menu (when text is just "3")
            let menu = format!("{}\n{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n\n{}",
                TranslationService::translate("usdc_menu_title", lang),
                TranslationService::translate("please_select_option", lang),
                TranslationService::translate("check_balance", lang),
                TranslationService::translate("usdc_rate", lang),
                TranslationService::translate("buy_usdc", lang),
                TranslationService::translate("sell_usdc", lang),
                TranslationService::translate("send_usdc", lang),
                TranslationService::translate("back_or_menu", lang));
            (menu, true)
        }
        "1" if parts.len() == 2 => {
            // Check balance (when text is "3*1")
            match crate::services::business_logic::get_balances(&session.phone_number).await {
                Ok(balances) => {
                    let ckusdc = balances.ckusdc_balance as f64 / 1_000_000.0;
                    (format!("{}:\nckUSDC: {:.2}\n\n{}", 
                        TranslationService::translate("usdc_balance", lang),
                        ckusdc,
                        TranslationService::translate("back_or_menu", lang)), true)
                }
                Err(_) => {
                    (format!("{}:\nckUSDC: 0.00\n\n{}", 
                        TranslationService::translate("usdc_balance", lang),
                        TranslationService::translate("back_or_menu", lang)), true)
                }
            }
        }
        "2" if parts.len() == 2 => {
            // USDC rate (when text is "3*2")
            crate::flows::common::usdc_rate::handle_usdc_rate(text, session).await
        }
        "3" if parts.len() == 2 => {
            // Buy USDC - start the flow (when text is "3*3")
            session.step = 0;
            session.current_menu = "buy_usdc".to_string();
            crate::flows::usdc::buy::handle_buy_usdc(text, session).await
        }
        "4" if parts.len() == 2 => {
            // Sell USDC (when text is "3*4")
            session.step = 0;
            session.current_menu = "sell_usdc".to_string();
            crate::flows::usdc::sell::handle_sell_usdc(text, session).await
        }
        "5" if parts.len() == 2 => {
            // Send USDC (when text is "3*5")
            session.step = 0;
            session.current_menu = "send_usdc".to_string();
            crate::flows::usdc::send::handle_send_usdc(text, session).await
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}

/// Handle DAO menu
pub async fn handle_dao_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    let last_input = parts.last().unwrap_or(&"");
    
    match *last_input {
        "5" if parts.len() == 1 => {
            // Show DAO menu (when text is just "5")
            session.current_menu = "dao".to_string();
            let menu = format!("{}\n{}\n1. {}\n2. {}\n\n{}",
                TranslationService::translate("dao_governance", lang),
                TranslationService::translate("please_select_option", lang),
                TranslationService::translate("view_proposals", lang),
                TranslationService::translate("vote_on_proposals", lang),
                TranslationService::translate("back_or_menu", lang));
            (menu, true)
        }
        "1" if parts.len() == 2 => {
            // View proposals (when text is "5*1")
            crate::flows::dao::proposals::handle_view_proposals(text, session).await
        }
        "2" if parts.len() == 2 => {
            // Vote on proposals (when text is "5*2")
            crate::flows::dao::vote::handle_vote(text, session).await
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}

/// Handle language menu
pub async fn handle_language_menu(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    let parts: Vec<&str> = text.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    // Helper to show language menu
    let show_menu = || -> (String, bool) {
        let menu = format!("{}\n1. {}\n2. {}\n3. {}\n\n{}",
            TranslationService::translate("select_language", lang),
            TranslationService::translate("english", lang),
            TranslationService::translate("luganda", lang),
            TranslationService::translate("swahili", lang),
            TranslationService::translate("back_or_menu", lang));
        (menu, true)
    };
    
    match *choice {
        "6" | "9" => {
            // Show language menu (when text is "6" or user presses "9" to repeat menu)
            show_menu()
        }
        "1" => {
            // Set English
            let new_lang = Language::English;
            session.language = new_lang.to_code().to_string();
            
            // Save language preference to Data Canister
            match crate::services::business_logic::update_user_language(&session.phone_number, "en").await {
                Ok(_) => ic_cdk::println!("✅ Language preference saved: en"),
                Err(e) => ic_cdk::println!("⚠️ Failed to save language preference: {}", e),
            }
            
            (format!("{}\n\n{}", 
                TranslationService::translate("language_set", new_lang),
                TranslationService::translate("back_or_menu", new_lang)), true)
        }
        "2" => {
            // Set Luganda
            let new_lang = Language::Luganda;
            session.language = new_lang.to_code().to_string();
            
            // Save language preference to Data Canister
            match crate::services::business_logic::update_user_language(&session.phone_number, "lg").await {
                Ok(_) => ic_cdk::println!("✅ Language preference saved: lg"),
                Err(e) => ic_cdk::println!("⚠️ Failed to save language preference: {}", e),
            }
            
            (format!("{}\n\n{}", 
                TranslationService::translate("language_set", new_lang),
                TranslationService::translate("back_or_menu", new_lang)), true)
        }
        "3" => {
            // Set Swahili
            let new_lang = Language::Swahili;
            session.language = new_lang.to_code().to_string();
            
            // Save language preference to Data Canister
            match crate::services::business_logic::update_user_language(&session.phone_number, "sw").await {
                Ok(_) => ic_cdk::println!("✅ Language preference saved: sw"),
                Err(e) => ic_cdk::println!("⚠️ Failed to save language preference: {}", e),
            }
            
            (format!("{}\n\n{}", 
                TranslationService::translate("language_set", new_lang),
                TranslationService::translate("back_or_menu", new_lang)), true)
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::translate("back_or_menu", lang)), true)
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
            data: std::collections::HashMap::new(),
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
            data: std::collections::HashMap::new(),
            last_activity: 0,
        };
        
        let (response, cont) = handle_registration(&mut session, "123").await;
        assert!(cont);
        assert!(response.contains("Invalid PIN"));
    }
}
