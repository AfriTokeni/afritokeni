// Send Bitcoin flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;
use crate::utils::data_canister_client;
use crate::utils::ledger_client;
use candid::Principal;

/// Handle send Bitcoin flow
/// Steps: 1. Enter BTC address → 2. Enter amount → 3. Enter PIN → 4. Execute
pub async fn handle_send_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Ask for BTC address
            (format!("{}\n{}", 
                TranslationService::translate("send_bitcoin", lang),
                TranslationService::translate("enter_btc_address", lang)), true)
        }
        1 => {
            // Step 1: Validate BTC address (parts[2])
            // Text: "2*4*address" -> parts[0]=2, parts[1]=4, parts[2]=address
            let address_raw = parts.get(2).unwrap_or(&"");
            let address = validation::sanitize_input(address_raw);
            
            if !validation::is_valid_btc_address(&address) {
                return (format!("Invalid BTC address\n{}", 
                    TranslationService::translate("try_again", lang)), true);
            }
            
            (format!("{} (ckBTC):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate amount (parts[3])
            // Text: "2*4*address*amount" -> parts[3]=amount
            let amount_str = parts.get(3).unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    let address = parts.get(2).unwrap_or(&"");
                    (format!("{}\n{}: {}\n{}: {} ckBTC\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("to", lang),
                        address,
                        TranslationService::translate("amount", lang),
                        amount,
                        TranslationService::translate("enter_pin_confirm", lang)), true)
                }
                Err(e) => {
                    (format!("{}\n{}", e, TranslationService::translate("try_again", lang)), true)
                }
            }
        }
        3 => {
            // Step 3: Verify PIN and execute real ckBTC transfer
            // parts: [0]=2, [1]=4, [2]=address, [3]=amount, [4]=pin
            let pin = parts.get(4).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let btc_address = parts.get(2).unwrap_or(&"").to_string();
            let amount_str = parts.get(3).unwrap_or(&"").to_string();
            
            // Get clients
            let data_client = match data_canister_client::create_client() {
                Ok(c) => c,
                Err(e) => {
                    session.clear_data();
                    return (format!("Error: {}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false);
                }
            };
            
            let ledger = match ledger_client::create_ckbtc_client() {
                Ok(l) => l,
                Err(e) => {
                    session.clear_data();
                    return (format!("Error: {}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false);
                }
            };
            
            // Get user
            let user = match data_client.get_user_by_phone(&phone).await {
                Ok(Some(u)) => u,
                Ok(None) => {
                    session.clear_data();
                    return (format!("{}\n\n0. {}", TranslationService::translate("user_not_found", lang), TranslationService::translate("main_menu", lang)), false);
                }
                Err(e) => {
                    session.clear_data();
                    return (format!("Error: {}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false);
                }
            };
            
            // Verify PIN
            match data_client.verify_user_pin(&user.id, pin).await {
                Ok(true) => {
                    // PIN correct - execute transfer
                    let amount_btc = amount_str.parse::<f64>().unwrap_or(0.0);
                    let amount_sats = ledger.to_smallest_unit(amount_btc);
                    
                    // Parse recipient principal from BTC address (simplified - in production use proper address parsing)
                    let recipient_principal = match Principal::from_text(&btc_address) {
                        Ok(p) => p,
                        Err(_) => {
                            session.clear_data();
                            return (format!("Invalid address\n\n0. {}", TranslationService::translate("main_menu", lang)), false);
                        }
                    };
                    
                    // Get user's ckBTC account
                    let user_subaccount = ledger_client::derive_subaccount_from_phone(&phone);
                    let recipient_account = ledger_client::get_user_account(recipient_principal);
                    
                    // Execute transfer on ckBTC ledger
                    match ledger.transfer(Some(user_subaccount), recipient_account, amount_sats, None).await {
                        Ok(_block_index) => {
                            // Update balance in data canister
                            let _ = data_client.update_crypto_balance(&user.id, -(amount_sats as i64), 0).await;
                            
                            // Get new balance
                            let (new_balance_sats, _) = data_client.get_crypto_balance(&user.id).await.unwrap_or((0, 0));
                            let new_balance_btc = ledger.from_smallest_unit(new_balance_sats);
                            
                            session.clear_data();
                            session.current_menu = String::new();
                            session.step = 0;
                            
                            (format!("{}\n{} {} ckBTC {} {}\n{}: {} ckBTC\n\n0. {}", 
                                TranslationService::translate("transaction_successful", lang),
                                TranslationService::translate("sent", lang),
                                amount_btc,
                                TranslationService::translate("to", lang),
                                btc_address,
                                TranslationService::translate("new_balance", lang),
                                new_balance_btc,
                                TranslationService::translate("main_menu", lang)), false)
                        }
                        Err(e) => {
                            session.clear_data();
                            (format!("Transfer failed: {}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false)
                        }
                    }
                }
                Ok(false) => {
                    (format!("{}\n{}", 
                        TranslationService::translate("incorrect_pin", lang),
                        TranslationService::translate("try_again", lang)), true)
                }
                Err(e) => {
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    (format!("{}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false)
                }
            }
        }
        _ => {
            session.clear_data();
            session.current_menu = String::new();
            session.step = 0;
            (TranslationService::translate("invalid_selection", lang).to_string(), true)
        }
    }
}
