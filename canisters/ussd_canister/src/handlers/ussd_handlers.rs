use crate::juno_store::{self, Balance, Transaction};
use crate::pin;
use crate::session::UssdSession;
use crate::translations::{Language, TranslationService};

/// Check if user needs registration (first time user)
pub async fn check_user_registration(session: &mut UssdSession) -> Option<(String, bool)> {
    let lang = Language::from_code(&session.language);
    
    // Check if user has a PIN set
    match juno_store::get_user_pin(&session.phone_number).await {
        Ok(_) => None, // User registered
        Err(_) => {
            // First time user - need to set PIN
            session.current_menu = "register".to_string();
            session.step = 0;
            Some((format!("{}\n\n{}\n{}",
                TranslationService::translate("welcome", lang),
                TranslationService::translate("first_time_setup", lang),
                TranslationService::translate("enter_new_pin", lang)), true))
        }
    }
}

/// Main menu handler
pub async fn handle_main_menu(input: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    if input.is_empty() {
        // Check if user needs registration
        if let Some(response) = check_user_registration(session).await {
            return response;
        }
        return (TranslationService::get_main_menu(lang), true);
    }
    
    let parts: Vec<&str> = input.split('*').collect();
    let choice = parts.get(0).unwrap_or(&"");
    
    match *choice {
        "1" => {
            session.current_menu = "local_currency".to_string();
            session.step = 0;
            handle_local_currency_menu("", session).await
        }
        "2" => {
            session.current_menu = "bitcoin".to_string();
            session.step = 0;
            handle_bitcoin_menu("", session).await
        }
        "3" => {
            session.current_menu = "usdc".to_string();
            session.step = 0;
            handle_usdc_menu("", session).await
        }
        "4" => {
            session.current_menu = "dao".to_string();
            session.step = 0;
            handle_dao_menu("", session).await
        }
        "5" => {
            // Help menu
            (format!("{}\n\n{}: {}, {}\n{}: {}, {}\n{}: {}\n\n{}: {} +256-XXX-XXXX\n{}: {}\n\n{}", 
                TranslationService::translate("help", lang),
                TranslationService::translate("local_currency", lang),
                TranslationService::translate("send_money", lang),
                TranslationService::translate("withdraw", lang),
                TranslationService::translate("bitcoin", lang),
                TranslationService::translate("buy_bitcoin", lang),
                TranslationService::translate("sell_bitcoin", lang),
                TranslationService::translate("dao", lang),
                TranslationService::translate("vote_on_proposals", lang),
                TranslationService::translate("for_support", lang),
                TranslationService::translate("call", lang),
                TranslationService::translate("visit", lang),
                TranslationService::translate("website", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
        "6" => {
            session.current_menu = "language".to_string();
            session.step = 0;
            handle_language_menu("", session).await
        }
        "0" => {
            (TranslationService::translate("thank_you", lang).to_string(), false)
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_option", lang),
                TranslationService::get_main_menu(lang)), true)
        }
    }
}

/// Local currency menu
pub async fn handle_local_currency_menu(input: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    if input.is_empty() {
        return (format!("{}\n{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n6. {}\n0. {}",
            TranslationService::translate("local_currency_menu", lang),
            TranslationService::translate("please_select_option", lang),
            TranslationService::translate("send_money", lang),
            TranslationService::translate("check_balance", lang),
            TranslationService::translate("deposit", lang),
            TranslationService::translate("withdraw", lang),
            TranslationService::translate("transactions", lang),
            TranslationService::translate("find_agent", lang),
            TranslationService::translate("back_to_main_menu", lang)), true);
    }
    
    let parts: Vec<&str> = input.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "1" => handle_send_money(session).await,
        "2" => handle_check_balance(session).await,
        "3" => handle_deposit(session).await,
        "4" => handle_withdraw(session).await,
        "5" => handle_transaction_history(session).await,
        "6" => handle_find_agent(session).await,
        "0" => {
            // Reset to main menu
            session.current_menu = String::new();
            session.step = 0;
            let lang = Language::from_code(&session.language);
            (TranslationService::get_main_menu(lang), true)
        }
        _ => (TranslationService::translate("invalid_option", lang).to_string(), true),
    }
}

/// Check balance
pub async fn handle_check_balance(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    // Check if PIN verified
    if !pin::is_pin_verified(session) {
        return (pin::request_pin_verification(session, "check_balance"), true);
    }
    
    // Fetch balance
    match juno_store::get_user_balance(&session.phone_number).await {
        Ok(balance) => {
            (format!("{}:\nKES: {:.2}\nckBTC: {:.8}\nckUSDC: {:.2}",
                TranslationService::translate("your_balance", lang),
                balance.kes,
                balance.ckbtc,
                balance.ckusdc), false)
        }
        Err(e) => {
            ic_cdk::println!("❌ Balance fetch error: {}", e);
            (TranslationService::translate("error_occurred", lang).to_string(), false)
        }
    }
}

/// Send money flow
pub async fn handle_send_money(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            // Ask for recipient
            session.step = 1;
            (TranslationService::translate("enter_recipient_phone", lang).to_string(), true)
        }
        1 => {
            // Ask for amount
            session.step = 2;
            (format!("{} (KES):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Ask for PIN if not verified
            if !pin::is_pin_verified(session) {
                return (pin::request_pin_verification(session, "send_money"), true);
            }
            // PIN verified, move to confirmation
            session.step = 3;
            (TranslationService::translate("confirm_transaction", lang).to_string(), true)
        }
        3 => {
            // Confirm and execute
            let recipient = session.get_data("recipient").cloned().unwrap_or_default();
            let amount_str = session.get_data("amount").cloned().unwrap_or_default();
            let amount: f64 = amount_str.parse().unwrap_or(0.0);
            
            if amount <= 0.0 {
                return (TranslationService::translate("invalid_amount", lang).to_string(), false);
            }
            
            // Create transaction
            let tx = Transaction {
                from: session.phone_number.clone(),
                to: recipient.clone(),
                amount,
                currency: "KES".to_string(),
                tx_type: "send".to_string(),
                status: "completed".to_string(),
                timestamp: ic_cdk::api::time(),
                fee: 0.0,
            };
            
            match juno_store::create_transaction(tx).await {
                Ok(_) => {
                    (format!("{} {:.2} KES {} {}\n{}",
                        TranslationService::translate("send_money", lang),
                        amount,
                        TranslationService::translate("to", lang),
                        recipient,
                        TranslationService::translate("transaction_successful", lang)), false)
                }
                Err(e) => {
                    ic_cdk::println!("❌ Transaction error: {}", e);
                    (TranslationService::translate("error_occurred", lang).to_string(), false)
                }
            }
        }
        _ => (TranslationService::translate("invalid_selection", lang).to_string(), false),
    }
}

/// Bitcoin menu
pub async fn handle_bitcoin_menu(input: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    if input.is_empty() {
        return (format!("{} (ckBTC)\n{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n0. {}",
            TranslationService::translate("bitcoin", lang),
            TranslationService::translate("please_select_option", lang),
            TranslationService::translate("check_balance", lang),
            TranslationService::translate("buy_bitcoin", lang),
            TranslationService::translate("sell_bitcoin", lang),
            TranslationService::translate("send_bitcoin", lang),
            TranslationService::translate("view_rate", lang),
            TranslationService::translate("back_to_main_menu", lang)), true);
    }
    
    let parts: Vec<&str> = input.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "1" => handle_check_balance(session).await,
        "2" => handle_buy_bitcoin(session).await,
        "3" => handle_sell_bitcoin(session).await,
        "4" => handle_send_bitcoin(session).await,
        "5" => handle_bitcoin_rate(session).await,
        "0" => {
            // Reset to main menu
            session.current_menu = String::new();
            session.step = 0;
            let lang = Language::from_code(&session.language);
            (TranslationService::get_main_menu(lang), true)
        }
        _ => (TranslationService::translate("invalid_option", lang).to_string(), true),
    }
}

/// Buy Bitcoin
pub async fn handle_buy_bitcoin(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            session.step = 1;
            (format!("{} (KES) {} ckBTC:",
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            if !pin::is_pin_verified(session) {
                return (pin::request_pin_verification(session, "buy_bitcoin"), true);
            }
            session.step = 2;
            (TranslationService::translate("confirm_transaction", lang).to_string(), true)
        }
        2 => {
            let amount_str = session.get_data("amount").cloned().unwrap_or_default();
            let amount: f64 = amount_str.parse().unwrap_or(0.0);
            
            if amount <= 0.0 {
                return (TranslationService::translate("invalid_amount", lang).to_string(), false);
            }
            
            // Create buy transaction
            let tx = Transaction {
                from: session.phone_number.clone(),
                to: "system".to_string(),
                amount,
                currency: "ckBTC".to_string(),
                tx_type: "buy".to_string(),
                status: "completed".to_string(),
                timestamp: ic_cdk::api::time(),
                fee: 0.0,
            };
            
            match juno_store::create_transaction(tx).await {
                Ok(_) => {
                    (format!("{} ckBTC {} {:.2} KES\n{}",
                        TranslationService::translate("buy_bitcoin", lang),
                        TranslationService::translate("with", lang),
                        amount,
                        TranslationService::translate("transaction_successful", lang)), false)
                }
                Err(e) => {
                    ic_cdk::println!("❌ Buy BTC error: {}", e);
                    (TranslationService::translate("error_occurred", lang).to_string(), false)
                }
            }
        }
        _ => (TranslationService::translate("invalid_selection", lang).to_string(), false),
    }
}

/// Sell Bitcoin
pub async fn handle_sell_bitcoin(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    (format!("{} {}", 
        TranslationService::translate("sell_bitcoin", lang),
        TranslationService::translate("coming_soon", lang)), false)
}

/// Send Bitcoin
pub async fn handle_send_bitcoin(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    (format!("{} {}",
        TranslationService::translate("send_bitcoin", lang),
        TranslationService::translate("coming_soon", lang)), false)
}

/// Bitcoin rate
pub async fn handle_bitcoin_rate(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    // Mock rate - in production, fetch from oracle
    (format!("{}: 1 BTC = 150,000,000 KES", 
        TranslationService::translate("current_rate", lang)), false)
}

/// USDC menu
pub async fn handle_usdc_menu(input: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    if input.is_empty() {
        return (format!("{} (ckUSDC)\n{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n0. {}",
            TranslationService::translate("usdc", lang),
            TranslationService::translate("please_select_option", lang),
            TranslationService::translate("check_balance", lang),
            TranslationService::translate("buy_usdc", lang),
            TranslationService::translate("sell_usdc", lang),
            TranslationService::translate("send_usdc", lang),
            TranslationService::translate("view_rate", lang),
            TranslationService::translate("back_to_main_menu", lang)), true);
    }
    
    let parts: Vec<&str> = input.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "1" => handle_check_balance(session).await,
        "2" => handle_buy_usdc(session).await,
        "3" => (format!("{} {}", TranslationService::translate("sell_usdc", lang), TranslationService::translate("coming_soon", lang)), false),
        "4" => (format!("{} {}", TranslationService::translate("send_usdc", lang), TranslationService::translate("coming_soon", lang)), false),
        "5" => (format!("{}: 1 USDC = 3,800 KES", TranslationService::translate("current_rate", lang)), false),
        "0" => {
            // Reset to main menu
            session.current_menu = String::new();
            session.step = 0;
            let lang = Language::from_code(&session.language);
            (TranslationService::get_main_menu(lang), true)
        }
        _ => (TranslationService::translate("invalid_option", lang).to_string(), true),
    }
}

/// Buy USDC
pub async fn handle_buy_usdc(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            session.step = 1;
            (format!("{} (KES) {} ckUSDC:",
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            if !pin::is_pin_verified(session) {
                return (pin::request_pin_verification(session, "buy_usdc"), true);
            }
            session.step = 2;
            (TranslationService::translate("confirm_transaction", lang).to_string(), true)
        }
        2 => {
            let amount_str = session.get_data("amount").cloned().unwrap_or_default();
            let amount: f64 = amount_str.parse().unwrap_or(0.0);
            
            if amount <= 0.0 {
                return (TranslationService::translate("invalid_amount", lang).to_string(), false);
            }
            
            let tx = Transaction {
                from: session.phone_number.clone(),
                to: "system".to_string(),
                amount,
                currency: "ckUSDC".to_string(),
                tx_type: "buy".to_string(),
                status: "completed".to_string(),
                timestamp: ic_cdk::api::time(),
                fee: 0.0,
            };
            
            match juno_store::create_transaction(tx).await {
                Ok(_) => {
                    (format!("{} ckUSDC {} {:.2} KES\n{}",
                        TranslationService::translate("buy_usdc", lang),
                        TranslationService::translate("with", lang),
                        amount,
                        TranslationService::translate("transaction_successful", lang)), false)
                }
                Err(e) => {
                    ic_cdk::println!("❌ Buy USDC error: {}", e);
                    (TranslationService::translate("error_occurred", lang).to_string(), false)
                }
            }
        }
        _ => (TranslationService::translate("invalid_selection", lang).to_string(), false),
    }
}

/// Deposit flow
pub async fn handle_deposit(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            session.step = 1;
            (format!("{} (KES):", TranslationService::translate("enter_amount", lang)), true)
        }
        1 => {
            let amount_str = session.get_data("amount").cloned().unwrap_or_default();
            let amount: f64 = amount_str.parse().unwrap_or(0.0);
            
            if amount <= 0.0 {
                return (TranslationService::translate("invalid_amount", lang).to_string(), false);
            }
            
            // Generate deposit code
            let code = format!("{:06}", (ic_cdk::api::time() % 1000000));
            
            // Get nearby agent
            match juno_store::find_agents_near("Kampala").await {
                Ok(agents) => {
                    if let Some(agent) = agents.first() {
                        (format!("{} {:.2} KES\n\n{}: {}\n\n{}: {}\n{}: {}\n\n{}",
                            TranslationService::translate("deposit", lang),
                            amount,
                            TranslationService::translate("code", lang),
                            code,
                            TranslationService::translate("agent", lang),
                            agent.name,
                            TranslationService::translate("phone", lang),
                            agent.phone,
                            TranslationService::translate("show_code_to_agent", lang)), false)
                    } else {
                        (TranslationService::translate("no_agents_found", lang).to_string(), false)
                    }
                }
                Err(e) => {
                    ic_cdk::println!("❌ Find agent error: {}", e);
                    (TranslationService::translate("error_occurred", lang).to_string(), false)
                }
            }
        }
        _ => (TranslationService::translate("invalid_selection", lang).to_string(), false),
    }
}

/// Withdraw flow
pub async fn handle_withdraw(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            session.step = 1;
            (format!("{} (KES):", TranslationService::translate("enter_amount", lang)), true)
        }
        1 => {
            let amount_str = session.get_data("amount").cloned().unwrap_or_default();
            let amount: f64 = amount_str.parse().unwrap_or(0.0);
            
            if amount <= 0.0 {
                return (TranslationService::translate("invalid_amount", lang).to_string(), false);
            }
            
            // Generate withdrawal code
            let code = format!("{:06}", (ic_cdk::api::time() % 1000000));
            
            (format!("{} {:.2} KES\n{}: {}\n{}",
                TranslationService::translate("withdraw", lang),
                amount,
                TranslationService::translate("code", lang),
                code,
                TranslationService::translate("receive_cash", lang)), false)
        }
        _ => (TranslationService::translate("invalid_selection", lang).to_string(), false),
    }
}

/// Transaction history
pub async fn handle_transaction_history(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match juno_store::get_transactions(&session.phone_number, 5).await {
        Ok(txs) => {
            if txs.is_empty() {
                return (TranslationService::translate("no_transactions", lang).to_string(), false);
            }
            
            let mut response = format!("{}:\n", TranslationService::translate("recent_transactions", lang));
            for (i, tx) in txs.iter().enumerate().take(5) {
                response.push_str(&format!("\n{}. {} {:.2} {} {}\n{}: {}",
                    i + 1,
                    tx.tx_type,
                    tx.amount,
                    tx.currency,
                    if tx.from == session.phone_number { 
                        format!("{} {}", TranslationService::translate("to", lang), tx.to)
                    } else {
                        format!("{} {}", TranslationService::translate("from", lang), tx.from)
                    },
                    TranslationService::translate("status", lang),
                    tx.status
                ));
            }
            (response, false)
        }
        Err(e) => {
            ic_cdk::println!("❌ Transaction history error: {}", e);
            (TranslationService::translate("error_occurred", lang).to_string(), false)
        }
    }
}

/// Find agent
pub async fn handle_find_agent(session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match juno_store::find_agents_near("Kampala").await {
        Ok(agents) => {
            if agents.is_empty() {
                return (TranslationService::translate("no_agents_found", lang).to_string(), false);
            }
            
            let mut response = format!("{}:\n", TranslationService::translate("nearby_agents", lang));
            for (i, agent) in agents.iter().enumerate().take(3) {
                response.push_str(&format!("\n{}. {}\n{}: {}\n{}: ⭐ {:.1}\n{}: {}",
                    i + 1,
                    agent.name,
                    TranslationService::translate("phone", lang),
                    agent.phone,
                    TranslationService::translate("rating", lang),
                    agent.rating,
                    TranslationService::translate("location", lang),
                    agent.location
                ));
            }
            (response, false)
        }
        Err(e) => {
            ic_cdk::println!("❌ Find agent error: {}", e);
            (TranslationService::translate("error_occurred", lang).to_string(), false)
        }
    }
}

/// DAO menu
pub async fn handle_dao_menu(input: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    if input.is_empty() {
        return (format!("{}\n{}\n1. {}\n2. {}\n0. {}",
            TranslationService::translate("dao_governance", lang),
            TranslationService::translate("please_select_option", lang),
            TranslationService::translate("view_proposals", lang),
            TranslationService::translate("my_votes", lang),
            TranslationService::translate("back_to_main_menu", lang)), true);
    }
    
    let parts: Vec<&str> = input.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "1" => {
            // View proposals
            match juno_store::get_proposals().await {
                Ok(proposals) => {
                    if proposals.is_empty() {
                        return (TranslationService::translate("no_proposals", lang).to_string(), false);
                    }
                    
                    let mut response = format!("{}:\n", TranslationService::translate("active_proposals", lang));
                    for (i, prop) in proposals.iter().enumerate().take(3) {
                        let total_votes = prop.votes_for + prop.votes_against;
                        let approval = if total_votes > 0 {
                            (prop.votes_for as f64 / total_votes as f64 * 100.0) as u32
                        } else {
                            0
                        };
                        
                        response.push_str(&format!("\n{}. {}\n{}: {}%\n{}: {}\n",
                            i + 1,
                            prop.title,
                            TranslationService::translate("approval", lang),
                            approval,
                            TranslationService::translate("status", lang),
                            prop.status
                        ));
                    }
                    (response, false)
                }
                Err(e) => {
                    ic_cdk::println!("❌ Get proposals error: {}", e);
                    (TranslationService::translate("error_occurred", lang).to_string(), false)
                }
            }
        }
        "2" => {
            // My votes
            (format!("{}\n{}", 
                TranslationService::translate("my_votes", lang),
                TranslationService::translate("no_votes_yet", lang)), false)
        }
        "0" => {
            // Reset to main menu
            session.current_menu = String::new();
            session.step = 0;
            let lang = Language::from_code(&session.language);
            (TranslationService::get_main_menu(lang), true)
        }
        _ => (TranslationService::translate("invalid_option", lang).to_string(), true),
    }
}

/// Language selection
pub async fn handle_language_menu(input: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    if input.is_empty() {
        return (format!("{}\n1. English\n2. Luganda\n3. Swahili\n0. {}",
            TranslationService::translate("select_language", lang),
            TranslationService::translate("back_to_main_menu", lang)), true);
    }
    
    let parts: Vec<&str> = input.split('*').collect();
    let choice = parts.last().unwrap_or(&"");
    
    match *choice {
        "1" => {
            session.language = "en".to_string();
            let _ = juno_store::set_language(&session.phone_number, "en").await;
            (TranslationService::translate("language_changed", Language::English).to_string(), false)
        }
        "2" => {
            session.language = "lg".to_string();
            let _ = juno_store::set_language(&session.phone_number, "lg").await;
            (TranslationService::translate("language_changed", Language::Luganda).to_string(), false)
        }
        "3" => {
            session.language = "sw".to_string();
            let _ = juno_store::set_language(&session.phone_number, "sw").await;
            (TranslationService::translate("language_changed", Language::Swahili).to_string(), false)
        }
        "0" => {
            // Reset to main menu
            session.current_menu = String::new();
            session.step = 0;
            let lang = Language::from_code(&session.language);
            (TranslationService::get_main_menu(lang), true)
        }
        _ => (TranslationService::translate("invalid_option", lang).to_string(), true),
    }
}

/// User registration handler (first-time PIN setup)
pub async fn handle_registration(session: &mut UssdSession, pin_input: &str) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    match session.step {
        0 => {
            // First PIN entry
            if !pin::is_valid_pin(pin_input) {
                return (format!("{}\n{}", 
                    TranslationService::translate("invalid_pin", lang),
                    TranslationService::translate("pin_4_6_digits", lang)), true);
            }
            
            // Store PIN temporarily for confirmation
            session.set_data("new_pin", pin_input);
            session.step = 1;
            (TranslationService::translate("confirm_pin", lang).to_string(), true)
        }
        1 => {
            // Confirm PIN
            let stored_pin = session.get_data("new_pin").cloned().unwrap_or_default();
            
            if pin_input != stored_pin {
                session.step = 0;
                session.data.remove("new_pin");
                return (format!("{}\n{}", 
                    TranslationService::translate("pins_dont_match", lang),
                    TranslationService::translate("enter_new_pin", lang)), true);
            }
            
            // Save PIN
            match pin::setup_pin(&session.phone_number, pin_input).await {
                Ok(_) => {
                    session.current_menu = String::new();
                    session.step = 0;
                    session.data.remove("new_pin");
                    pin::mark_pin_verified(session);
                    (format!("{}\n\n{}", 
                        TranslationService::translate("pin_setup_success", lang),
                        TranslationService::get_main_menu(lang)), true)
                }
                Err(e) => {
                    ic_cdk::println!("❌ PIN setup error: {}", e);
                    (TranslationService::translate("error_occurred", lang).to_string(), false)
                }
            }
        }
        _ => (TranslationService::translate("invalid_selection", lang).to_string(), false),
    }
}
