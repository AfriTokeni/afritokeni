// Send Bitcoin flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send Bitcoin flow
/// Steps: 1. Enter BTC address → 2. Enter amount → 3. Enter PIN → 4. Execute
pub async fn handle_send_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    match session.step {
        0 => {
            // Step 0: Ask for recipient BTC address
            session.current_menu = "send_bitcoin".to_string();
            session.step = 1;
            session.clear_data();
            (format!("{}\n{}", 
                TranslationService::translate("send_bitcoin", lang),
                TranslationService::translate("enter_btc_address", lang)), true)
        }
        1 => {
            // Step 1: Validate and save BTC address
            let address_raw = parts.last().unwrap_or(&"");
            let address = validation::sanitize_input(address_raw);
            
            if !validation::is_valid_btc_address(&address) {
                return (format!("{}\n{}", 
                    TranslationService::translate("invalid_btc_address", lang),
                    TranslationService::translate("try_again", lang)), true);
            }
            
            session.set_data("btc_address", &address);
            session.step = 2;
            (format!("{} (ckBTC):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate and save amount
            let amount_str = parts.last().unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    session.set_data("amount", amount_str);
                    session.step = 3;
                    
                    let address = session.get_data("btc_address").cloned().unwrap_or_default();
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
            // Step 3: Verify PIN and execute
            let pin = parts.last().unwrap_or(&"");
            let phone = session.phone_number.clone();
            let address = session.get_data("btc_address").cloned().unwrap_or_default();
            let amount = session.get_data("amount").cloned().unwrap_or_default();
            
            match crate::utils::pin::verify_user_pin(&phone, pin).await {
                Ok(true) => {
                    // TODO: Actually send Bitcoin
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    
                    (format!("{}\n{} {} ckBTC {} {}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sent", lang),
                        amount,
                        TranslationService::translate("to", lang),
                        address,
                        TranslationService::translate("main_menu", lang)), true)
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
                    (format!("{}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), true)
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
