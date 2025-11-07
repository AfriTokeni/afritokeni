// Send Money flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send money flow
/// Steps: 1. Enter recipient → 2. Enter amount → 3. Enter PIN → 4. Confirm
pub async fn handle_send_money(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    match session.step {
        0 => {
            // Step 0: Show send money menu, ask for recipient
            session.current_menu = "send_money".to_string();
            session.step = 1;
            session.clear_data();
            (format!("{}\n{}", 
                TranslationService::translate("send_money", lang),
                TranslationService::translate("enter_recipient_phone", lang)), true)
        }
        1 => {
            // Step 1: Validate and save recipient phone
            let recipient_raw = parts.last().unwrap_or(&"");
            let recipient = validation::sanitize_input(recipient_raw);
            
            if !validation::is_valid_phone(&recipient) {
                return (format!("{}\n{}", 
                    TranslationService::translate("invalid_phone", lang),
                    TranslationService::translate("try_again", lang)), true);
            }
            
            session.set_data("recipient", &recipient);
            session.step = 2;
            (format!("{} (KES):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate and save amount
            let amount_str = parts.last().unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    session.set_data("amount", amount_str);
                    session.step = 3;
                    
                    let recipient = session.get_data("recipient").cloned().unwrap_or_default();
                    (format!("{}\n{}: {}\n{}: {} KES\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("to", lang),
                        recipient,
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
            // Step 3: Verify PIN and execute transaction
            let pin = parts.last().unwrap_or(&"");
            let phone = session.phone_number.clone();
            let recipient = session.get_data("recipient").cloned().unwrap_or_default();
            let amount = session.get_data("amount").cloned().unwrap_or_default();
            
            // Verify PIN
            match crate::utils::pin::verify_user_pin(&phone, pin).await {
                Ok(true) => {
                    // PIN correct - execute transaction
                    // TODO: Actually send money via inter-canister call
                    
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    
                    (format!("{}\n{} {} KES {} {}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sent", lang),
                        amount,
                        TranslationService::translate("to", lang),
                        recipient,
                        TranslationService::translate("main_menu", lang)), true)
                }
                Ok(false) => {
                    // PIN incorrect
                    (format!("{}\n{}", 
                        TranslationService::translate("incorrect_pin", lang),
                        TranslationService::translate("try_again", lang)), true)
                }
                Err(e) => {
                    // Locked out or error
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    (format!("{}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), true)
                }
            }
        }
        _ => {
            // Invalid state, reset
            session.clear_data();
            session.current_menu = String::new();
            session.step = 0;
            (TranslationService::translate("invalid_selection", lang).to_string(), true)
        }
    }
}
