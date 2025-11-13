// Buy Bitcoin flow with PIN verification
use crate::core::session::UssdSession;
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
            // Step 2: Call Business Logic to buy crypto
            // parts: [0]=2, [1]=3, [2]=amount, [3]=pin
            let pin = parts.get(3).unwrap_or(&"");
            let amount_str = parts.get(2).unwrap_or(&"");
            
            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("{}: {}\n\n0. {}", 
                    TranslationService::translate("error", lang),
                    e,
                    TranslationService::translate("main_menu", lang)), false),
            };
            
            // Get user's currency from session
            let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
            
            // Parse amount
            let amount_f64 = amount_str.parse::<f64>().unwrap_or(0.0);
            let amount_cents = (amount_f64 * 100.0) as u64;
            
            // Convert currency string to enum
            let currency_enum = shared_types::FiatCurrency::from_code(&currency)
                .ok_or_else(|| format!("Invalid currency: {}", currency));
            
            let currency_enum = match currency_enum {
                Ok(c) => c,
                Err(e) => {
                    return (format!("{}: {}\n\n0. {}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("main_menu", lang)), false);
                }
            };
            
            // Call Crypto Canister to buy Bitcoin
            match crate::services::crypto_client::buy_crypto(
                user_profile.id.clone(),
                amount_cents,
                currency_enum,
                shared_types::CryptoType::CkBTC,
                pin.to_string()
            ).await {
                Ok(tx_result) => {
                    let btc_amount = (tx_result.crypto_amount as f64) / 100_000_000.0; // satoshis to BTC
                    let fiat_spent = (tx_result.fiat_amount as f64) / 100.0;
                    
                    (format!("{}\n{} {} {} {} {:.8} BTC\n{}: {:.2} {}\n\n0. {}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("bought", lang),
                        fiat_spent,
                        currency,
                        TranslationService::translate("worth_of", lang),
                        btc_amount,
                        TranslationService::translate("exchange_rate", lang),
                        tx_result.exchange_rate,
                        currency,
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
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
