// Send USDC flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send USDC flow
/// Steps: 0. Enter recipient → 1. Enter amount → 2. Enter PIN → 3. Execute
pub async fn handle_send_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    ic_cdk::println!("💵 Send USDC flow: step={}, parts={:?}", step, parts);
    
    match step {
        0 => {
            // Step 0: Ask for recipient phone
            (format!("{}\n{}\n{}\n\n{}", 
                TranslationService::translate("send_usdc", lang),
                TranslationService::translate("enter_recipient_phone", lang),
                TranslationService::translate("phone_format_example", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
        1 => {
            // Step 1: Validate recipient and ask for amount
            let recipient_raw = parts.get(2).unwrap_or(&"");
            let recipient = validation::sanitize_input(recipient_raw);
            
            if !validation::is_valid_phone(&recipient) {
                return (format!("{}\n{}\n{}\n\n{}", 
                    TranslationService::translate("invalid_phone", lang),
                    TranslationService::translate("enter_recipient_phone", lang),
                    TranslationService::translate("phone_format_example", lang),
                    TranslationService::translate("back_or_menu", lang)), true);
            }
            
            (format!("{} (USDC):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate amount, check balance, ask for PIN
            let amount_str = parts.get(3).unwrap_or(&"");
            
            let amount_usdc = match validation::parse_amount(amount_str) {
                Ok(amt) => amt,
                Err(e) => {
                    return (format!("{}\n{} (USDC):", e, TranslationService::translate("enter_amount", lang)), true);
                }
            };
            
            // Check USDC balance
            match crate::services::business_logic::get_balances(&session.phone_number).await {
                Ok(balances) => {
                    let usdc_balance = balances.ckusdc_balance as f64 / 1_000_000.0;
                    
                    if usdc_balance < amount_usdc {
                        return (format!("{}!\n{}: {:.2} USDC\n{}: {:.2} USDC\n\n{}", 
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            usdc_balance,
                            TranslationService::translate("required", lang),
                            amount_usdc,
                            TranslationService::translate("thank_you", lang)), false);
                    }
                    
                    let recipient = parts.get(2).unwrap_or(&"");
                    (format!("{}\n{}: {}\n{}: {:.2} USDC\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("to", lang),
                        recipient,
                        TranslationService::translate("amount", lang),
                        amount_usdc,
                        TranslationService::translate("enter_pin_4digit", lang)), true)
                }
                Err(e) => {
                    (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("back_or_menu", lang)), true)
                }
            }
        }
        3 => {
            // Step 3: Execute send
            let pin = parts.get(4).unwrap_or(&"");
            let recipient = parts.get(2).unwrap_or(&"").to_string();
            let amount_str = parts.get(3).unwrap_or(&"").to_string();
            // Amount is already in e6 (micro-USDC)
            let amount_e6 = amount_str.parse::<u64>().unwrap_or(0);
            let amount_usdc = amount_e6 as f64 / 1_000_000.0;
            
            ic_cdk::println!("💵 Executing send_usdc: to={}, amount={} e6", recipient, amount_e6);
            
            match crate::services::business_logic::send_usdc(
                &session.phone_number,
                &recipient,
                amount_e6,
                pin
            ).await {
                Ok(result) => {
                    let new_balance = result.new_balance as f64 / 1_000_000.0;
                    (format!("{}!\n{} {:.2} USDC {} {}\n{}: {:.2} USDC\n\n{}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sent", lang),
                        amount_usdc,
                        TranslationService::translate("to", lang),
                        recipient,
                        TranslationService::translate("new_balance", lang),
                        new_balance,
                        TranslationService::translate("thank_you", lang)), false)
                }
                Err(e) => {
                    (format!("{}: {}\n\n{}", 
                        TranslationService::translate("transaction_failed", lang),
                        e,
                        TranslationService::translate("thank_you", lang)), false)
                }
            }
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
