// Buy Bitcoin flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle buy Bitcoin flow
/// Steps: 1. Enter KES amount → 2. Enter PIN → 3. Execute
pub async fn handle_buy_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Ask for KES amount
            (format!("{}\n{} (KES) {} ckBTC:", 
                TranslationService::translate("buy_bitcoin", lang),
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            // Step 1: Validate amount (parts[2])
            // Text: "2*3*amount" -> parts[0]=2, parts[1]=3, parts[2]=amount
            let amount_str = parts.get(2).unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    // TODO: Get actual BTC rate
                    let btc_rate = 50_000_000.0; // 50M KES per BTC (example)
                    let btc_amount = amount / btc_rate;
                    
                    (format!("{}\n{}: {} KES\n{}: {:.8} ckBTC\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("you_pay", lang),
                        amount,
                        TranslationService::translate("you_receive", lang),
                        btc_amount,
                        TranslationService::translate("enter_pin_confirm", lang)), true)
                }
                Err(e) => {
                    (format!("{}\n{}", e, TranslationService::translate("try_again", lang)), true)
                }
            }
        }
        2 => {
            // Step 2: Verify PIN and execute
            // parts: [0]=2, [1]=3, [2]=amount_kes, [3]=pin
            let pin = parts.get(3).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let amount_kes_str = parts.get(2).unwrap_or(&"");
            
            let amount_kes = amount_kes_str.to_string();
            let btc_rate = 50_000_000.0;
            let amount_f64 = amount_kes_str.parse::<f64>().unwrap_or(0.0);
            let btc_amount_f64 = amount_f64 / btc_rate;
            let amount_btc = format!("{:.8}", btc_amount_f64);
            
            match crate::utils::pin::verify_user_pin(&phone, pin).await {
                Ok(true) => {
                    // Execute BTC purchase
                    let kes_f64 = amount_kes.parse::<f64>().unwrap_or(0.0);
                    let btc_f64 = amount_btc.parse::<f64>().unwrap_or(0.0);
                    
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
                    
                    let btc_balance = crate::utils::datastore::get_user_data(&phone, "ckbtc_balance")
                        .await.ok().flatten().and_then(|b| b.parse::<f64>().ok()).unwrap_or(0.0);
                    let _ = crate::utils::datastore::set_user_data(&phone, "ckbtc_balance", &(btc_balance + btc_f64).to_string()).await;
                    
                    (format!("{}\nBought {} ckBTC for {} KES\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        amount_btc,
                        amount_kes,
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
