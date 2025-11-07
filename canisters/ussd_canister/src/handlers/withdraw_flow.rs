// Withdraw flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle withdraw flow
/// Steps: 1. Enter amount → 2. Enter PIN → 3. Execute
pub async fn handle_withdraw(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    match session.step {
        0 => {
            // Step 0: Ask for amount
            session.current_menu = "withdraw".to_string();
            session.step = 1;
            session.clear_data();
            (format!("{}\n{} (KES):", 
                TranslationService::translate("withdraw", lang),
                TranslationService::translate("enter_amount", lang)), true)
        }
        1 => {
            // Step 1: Validate amount and ask for PIN
            let amount_str = parts.last().unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    session.set_data("amount", amount_str);
                    session.step = 2;
                    (format!("{}\n{}: {} KES\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("withdraw", lang),
                        amount,
                        TranslationService::translate("enter_pin_confirm", lang)), true)
                }
                Err(e) => {
                    (format!("{}\n{}", e, TranslationService::translate("try_again", lang)), true)
                }
            }
        }
        2 => {
            // Step 2: Verify PIN and execute
            let pin = parts.last().unwrap_or(&"");
            let phone = session.phone_number.clone();
            let amount = session.get_data("amount").cloned().unwrap_or_default();
            
            match crate::utils::pin::verify_user_pin(&phone, pin).await {
                Ok(true) => {
                    // TODO: Actually process withdrawal
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    
                    (format!("{}\n{} {} KES\n{}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("withdraw", lang),
                        amount,
                        TranslationService::translate("receive_cash", lang),
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
