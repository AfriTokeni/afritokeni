// Buy USDC flow with PIN verification
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle buy USDC flow
/// Supports both interactive and shorthand modes:
/// - Interactive: 3*3 -> amount -> PIN
/// - Shorthand: 3*3*AMOUNT*PIN
pub async fn handle_buy_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // Determine step based on parts length
    // parts[0]=3 (USDC menu), parts[1]=3 (Buy option)
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    match step {
        0 => {
            // Step 0: Ask for fiat amount
            let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
            (format!("{}\n{} ({}) {} ckUSDC:",
                TranslationService::translate("buy_usdc", lang),
                TranslationService::translate("enter_amount", lang),
                currency,
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            // Step 1: Validate amount and ask for PIN
            // Interactive: parts = [3, 3, amount]
            // Shorthand check: parts = [3, 3, amount, PIN]
            let amount_str = parts.get(2).unwrap_or(&"");

            match validation::parse_amount(amount_str) {
                Ok(amount) => {
                    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
                    (format!("{}\n{}: {} {}\n\n{}",
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("you_pay", lang),
                        amount,
                        currency,
                        TranslationService::translate("enter_pin_confirm", lang)), true)
                }
                Err(e) => {
                    (format!("{}\n{}", e, TranslationService::translate("try_again", lang)), true)
                }
            }
        }
        2 => {
            // Step 2: Execute buy with PIN
            // parts = [3, 3, amount, PIN]
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

            // Parse and validate amount
            let amount_cents = match amount_str.parse::<f64>() {
                Ok(amt) if amt > 0.0 => (amt * 100.0) as u64,
                _ => {
                    return (format!("{}\n\n0. {}",
                        TranslationService::translate("invalid_amount", lang),
                        TranslationService::translate("main_menu", lang)), false);
                }
            };

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

            // Call Crypto Canister to buy USDC
            match crate::services::crypto_client::buy_crypto(
                user_profile.id.clone(),
                amount_cents,
                currency_enum,
                shared_types::CryptoType::CkUSDC,
                pin.to_string()
            ).await {
                Ok(tx_result) => {
                    let usdc_amount = (tx_result.crypto_amount as f64) / 1_000_000.0; // e6 to USDC
                    let fiat_spent = (tx_result.fiat_amount as f64) / 100.0;

                    (format!("{}\n{} {} {} {} {:.2} USDC\n{}: {:.2} {}\n\n0. {}",
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("bought", lang),
                        fiat_spent,
                        currency,
                        TranslationService::translate("worth_of", lang),
                        usdc_amount,
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
