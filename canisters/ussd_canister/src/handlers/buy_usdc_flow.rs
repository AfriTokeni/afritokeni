// Buy USDC flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle buy USDC flow
/// Steps: 1. Enter KES amount → 2. Enter PIN → 3. Execute
pub async fn handle_buy_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    match session.step {
        0 => {
            // Step 0: Ask for KES amount
            session.current_menu = "buy_usdc".to_string();
            session.step = 1;
            session.clear_data();
            (format!("{}\n{} (KES) {} ckUSDC:", 
                TranslationService::translate("buy_usdc", lang),
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            // Step 1: Validate amount and ask for PIN
            let amount_str = parts.last().unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    // TODO: Get actual USDC rate
                    let usdc_rate = 150.0; // 150 KES per USDC (example)
                    let usdc_amount = amount / usdc_rate;
                    
                    session.set_data("amount_kes", amount_str);
                    session.set_data("amount_usdc", &format!("{:.2}", usdc_amount));
                    session.step = 2;
                    
                    (format!("{}\n{}: {} KES\n{}: {:.2} ckUSDC\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("you_pay", lang),
                        amount,
                        TranslationService::translate("you_receive", lang),
                        usdc_amount,
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
            let amount_kes = session.get_data("amount_kes").cloned().unwrap_or_default();
            let amount_usdc = session.get_data("amount_usdc").cloned().unwrap_or_default();
            
            match crate::utils::pin::verify_user_pin(&phone, pin).await {
                Ok(true) => {
                    // Execute USDC purchase
                    let kes_f64 = amount_kes.parse::<f64>().unwrap_or(0.0);
                    let usdc_f64 = amount_usdc.parse::<f64>().unwrap_or(0.0);
                    
                    let kes_balance = crate::utils::datastore::get_user_data(&phone, "kes_balance")
                        .await.ok().flatten().and_then(|b| b.parse::<f64>().ok()).unwrap_or(0.0);
                    
                    if kes_balance < kes_f64 {
                        return (format!("{}\n{}: {} KES\n\n{}", 
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            kes_balance,
                            TranslationService::translate("try_again", lang)), true);
                    }
                    
                    let _ = crate::utils::datastore::set_user_data(&phone, "kes_balance", &(kes_balance - kes_f64).to_string()).await;
                    
                    let usdc_balance = crate::utils::datastore::get_user_data(&phone, "ckusdc_balance")
                        .await.ok().flatten().and_then(|b| b.parse::<f64>().ok()).unwrap_or(0.0);
                    let _ = crate::utils::datastore::set_user_data(&phone, "ckusdc_balance", &(usdc_balance + usdc_f64).to_string()).await;
                    
                    session.clear_data();
                    session.current_menu = String::new();
                    session.step = 0;
                    
                    (format!("{}\n{} {} KES\n{} {} ckUSDC\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("paid", lang),
                        amount_kes,
                        TranslationService::translate("received", lang),
                        amount_usdc,
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
