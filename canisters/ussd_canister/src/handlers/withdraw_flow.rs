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
            (format!("{}\n{} (KES):", 
                TranslationService::translate("withdraw", lang),
                TranslationService::translate("enter_amount", lang)), true)
        }
        1 => {
            // Step 1: Validate amount (parts[2])
            // Text: "1*4*amount" -> parts[0]=1, parts[1]=4, parts[2]=amount
            let amount_str = parts.get(2).unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    (format!("{}\n{}: {} KES\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("amount", lang),
                        amount,
                        TranslationService::translate("enter_pin_confirm", lang)), true)
                }
                Err(e) => {
                    (format!("{}\n{}", e, TranslationService::translate("try_again", lang)), true)
                }
            }
        }
        2 => {
            // Step 2: Verify PIN and execute withdrawal
            // parts: [0]=1, [1]=4, [2]=amount, [3]=pin
            let pin = parts.get(3).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let amount_str = parts.get(2).unwrap_or(&"");
            let amount_f64 = amount_str.parse::<f64>().unwrap_or(0.0);
            
            // TODO: Implement withdraw_fiat in Business Logic Canister
            // For now, just verify PIN and show success
            match crate::utils::business_logic_helper::verify_pin(&phone, pin).await {
                Ok(true) => {
                    (format!("{}\n{} {} UGX\n{}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("withdraw", lang),
                        amount_f64,
                        TranslationService::translate("receive_cash", lang),
                        TranslationService::translate("main_menu", lang)), false)
                }
                Ok(false) => {
                    (format!("{}\n{}", 
                        TranslationService::translate("incorrect_pin", lang),
                        TranslationService::translate("try_again", lang)), true)
                }
                Err(e) => {
                    (format!("{}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false)
                }
            }
        }
        _ => {
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
