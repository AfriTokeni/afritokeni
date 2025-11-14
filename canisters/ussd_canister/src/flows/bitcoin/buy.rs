// Buy Bitcoin flow with PIN verification
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle buy Bitcoin flow
/// Supports both interactive and shorthand modes:
/// - Interactive: 2*3 -> amount -> PIN
/// - Shorthand: 2*3*AMOUNT*PIN
pub async fn handle_buy_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // Parse parameters upfront
    let amount_str = parts.get(2).unwrap_or(&"");
    let pin = parts.get(3).unwrap_or(&"");

    // Detect mode: shorthand if both amount and PIN are provided
    let is_shorthand = !amount_str.is_empty() && !pin.is_empty();

    if is_shorthand {
        // Shorthand mode: execute directly
        return execute_buy_bitcoin(session, amount_str, pin, lang).await;
    }

    // Interactive mode: step through the flow
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    match step {
        0 => {
            // Step 0: Ask for amount
            (format!("{}\n{} (KES) {} ckBTC:",
                TranslationService::translate("buy_bitcoin", lang),
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("to", lang)), true)
        }
        1 => {
            // Step 1: Validate amount and ask for PIN
            match validation::parse_amount(amount_str) {
                Ok(amount) => {
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
            // Step 2: Execute the buy
            execute_buy_bitcoin(session, amount_str, pin, lang).await
        }
        _ => {
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}

/// Execute buy Bitcoin transaction (helper function)
async fn execute_buy_bitcoin(
    session: &UssdSession,
    amount_str: &str,
    pin: &str,
    lang: Language,
) -> (String, bool) {
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
