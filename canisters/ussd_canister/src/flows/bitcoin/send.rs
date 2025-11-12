// Send Bitcoin flow with PIN verification
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

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
            // Amount is already in satoshis
            let amount_sats = amount_str.parse::<u64>().unwrap_or(0);
            
            // Call Business Logic to send Bitcoin
            match crate::services::business_logic::send_crypto(
                &phone,
                &btc_address,
                amount_sats,
                crate::services::business_logic::CryptoType::CkBTC,
                pin
            ).await {
                Ok(result) => {
                    let btc_sent = result.amount as f64 / 100_000_000.0;
                    session.clear_data();
                    (format!("{}\nSent {:.8} ckBTC to {}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        btc_sent,
                        btc_address,
                        TranslationService::translate("main_menu", lang)), false)
                }
                Err(e) if e.contains("PIN") => {
                    session.clear_data();
                    (format!("{}\n{}", 
                        TranslationService::translate("incorrect_pin", lang),
                        TranslationService::translate("try_again", lang)), true)
                }
                Err(e) => {
                    session.clear_data();
                    (format!("Transfer failed: {}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false)
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
