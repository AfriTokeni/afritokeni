// Buy USDC flow with PIN verification
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle buy USDC flow
/// Steps: 1. Enter KES amount → 2. Enter PIN → 3. Execute
pub async fn handle_buy_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Ask for KES amount
            (format!("{}\n{} (KES) {} ckUSDC:", 
                TranslationService::translate("buy_usdc", lang),
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            // Step 1: Validate amount (parts[2])
            // Text: "3*2*amount" -> parts[0]=3, parts[1]=2, parts[2]=amount
            let amount_str = parts.get(2).unwrap_or(&"");
            
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    // Business Logic will handle exchange rates
                    (format!("{}\n{}: {} UGX\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("you_pay", lang),
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
            let pin = parts.get(3).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let amount_str = parts.get(2).unwrap_or(&"");
            
            // Get user's currency from session
            let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
            
            // Call Crypto Canister to buy USDC
            let amount_cents = (amount_str.parse::<f64>().unwrap_or(0.0) * 100.0) as u64;
            
            let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                Some(c) => c,
                None => return (format!("Invalid currency\n\n0. {}", TranslationService::translate("main_menu", lang)), false),
            };
            
            match crate::services::crypto_client::buy_crypto(
                phone.clone(),
                amount_cents,
                currency_enum,
                shared_types::CryptoType::CkUSDC,
                pin.to_string()
            ).await {
                Ok(result) => {
                    let usdc_amount = result.crypto_amount as f64 / 1_000_000.0;
                    let fiat_amount = amount_cents as f64 / 100.0;
                    
                    (format!("{}\nBought {:.2} ckUSDC for {:.2} {}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        usdc_amount,
                        fiat_amount,
                        currency,
                        TranslationService::translate("main_menu", lang)), false)
                }
                Err(e) if e.contains("Insufficient") => {
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
