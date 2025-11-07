// Withdraw flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle withdraw flow
/// Steps: 1. Enter amount → 2. Enter PIN → 3. Execute
pub async fn handle_withdraw(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
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
                    // Check balance and process withdrawal
                    let amount_f64 = amount.parse::<f64>().unwrap_or(0.0);
                    
                    let current_balance = crate::utils::datastore::get_user_data(&phone, "kes_balance")
                        .await
                        .ok()
                        .flatten()
                        .and_then(|b| b.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    
                    if current_balance < amount_f64 {
                        return (format!("{}\n{}: {} KES\n\n{}", 
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            current_balance,
                            TranslationService::translate("try_again", lang)), true);
                    }
                    
                    let new_balance = current_balance - amount_f64;
                    let _ = crate::utils::datastore::set_user_data(&phone, "kes_balance", &new_balance.to_string()).await;
                    
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    
                    (format!("{}\n{} {} KES\n{}\n{}: {} KES\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("withdraw", lang),
                        amount,
                        TranslationService::translate("receive_cash", lang),
                        TranslationService::translate("new_balance", lang),
                        new_balance,
                        TranslationService::translate("main_menu", lang)), false)
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
