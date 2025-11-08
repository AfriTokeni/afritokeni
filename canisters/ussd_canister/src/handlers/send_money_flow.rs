// Send Money flow with PIN verification
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send money flow
/// Steps: 1. Enter recipient → 2. Enter amount → 3. Enter PIN → 4. Confirm
pub async fn handle_send_money(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    // Determine step based on number of parts: "1*2" = step 0, "1*2*phone" = step 1, etc.
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Show send money menu, ask for recipient
            (format!("{}\n{}", 
                TranslationService::translate("send_money", lang),
                TranslationService::translate("enter_recipient_phone", lang)), true)
        }
        1 => {
            // Step 1: Validate recipient phone (parts[2])
            let recipient_raw = parts.get(2).unwrap_or(&"");
            let recipient = validation::sanitize_input(recipient_raw);
            
            if !validation::is_valid_phone(&recipient) {
                return (format!("{}\n{}", 
                    TranslationService::translate("invalid_phone", lang),
                    TranslationService::translate("try_again", lang)), true);
            }
            
            (format!("{} (KES):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate amount (parts[3])
            let amount_str = parts.get(3).unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    let recipient = parts.get(2).unwrap_or(&"");
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
            // Step 3: Call Business Logic Canister directly
            // parts: [0]=1, [1]=2, [2]=phone, [3]=amount, [4]=pin
            let pin = parts.get(4).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let recipient_phone = parts.get(2).unwrap_or(&"").to_string();
            let amount_str = parts.get(3).unwrap_or(&"").to_string();
            
            // Parse amount
            let amount_f64 = amount_str.parse::<f64>().unwrap_or(0.0);
            let amount_cents = (amount_f64 * 100.0) as u64;
            
            // Call Business Logic to send money
            match crate::utils::business_logic_helper::send_money(
                &phone,
                &recipient_phone,
                amount_cents,
                "KES",
                pin
            ).await {
                Ok(tx_result) => {
                    let new_balance = (tx_result.new_balance as f64) / 100.0;
                    (format!("{}\n{} {} UGX {} {}\n{}: {} UGX\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sent", lang),
                        amount_f64,
                        TranslationService::translate("to", lang),
                        recipient_phone,
                        TranslationService::translate("new_balance", lang),
                        new_balance,
                        TranslationService::translate("main_menu", lang)), false)
                }
                Err(e) => {
                    (format!("{}: {}\n\n0. {}", 
                        TranslationService::translate("transaction_failed", lang),
                        e,
                        TranslationService::translate("main_menu", lang)), false)
                }
            }
        }
        _ => {
            // Invalid state
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
