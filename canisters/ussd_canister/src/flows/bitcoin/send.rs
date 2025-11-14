// Send Bitcoin flow with PIN verification
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send Bitcoin flow
/// Supports both interactive and shorthand modes:
/// - Interactive: 2*5 -> address -> amount -> PIN
/// - Shorthand: 2*5*ADDRESS*AMOUNT*PIN
pub async fn handle_send_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // Parse parameters upfront
    let address_str = parts.get(2).unwrap_or(&"");
    let amount_str = parts.get(3).unwrap_or(&"");
    let pin = parts.get(4).unwrap_or(&"");

    // Detect mode: shorthand if all params are provided
    let is_shorthand = !address_str.is_empty() && !amount_str.is_empty() && !pin.is_empty();

    if is_shorthand {
        // Shorthand mode: execute directly
        return execute_send_bitcoin(session, address_str, amount_str, pin, lang).await;
    }

    // Interactive mode: step through the flow
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    match step {
        0 => {
            // Step 0: Ask for BTC address
            (format!("{}\n{}",
                TranslationService::translate("send_bitcoin", lang),
                TranslationService::translate("enter_btc_address", lang)), true)
        }
        1 => {
            // Step 1: Validate BTC address
            let address = validation::sanitize_input(address_str);

            if !validation::is_valid_btc_address(&address) {
                return (format!("Invalid BTC address\n{}",
                    TranslationService::translate("try_again", lang)), true);
            }

            (format!("{} (ckBTC):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate amount and ask for PIN
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
            // Step 3: Execute the send
            execute_send_bitcoin(session, address_str, amount_str, pin, lang).await
        }
        _ => {
            session.clear_data();
            session.current_menu = String::new();
            session.step = 0;
            (TranslationService::translate("invalid_selection", lang).to_string(), true)
        }
    }
}

/// Execute send Bitcoin transaction (helper function)
async fn execute_send_bitcoin(
    session: &UssdSession,
    address_str: &str,
    amount_str: &str,
    pin: &str,
    lang: Language,
) -> (String, bool) {
    let btc_address = address_str.to_string();
    // Amount is in satoshis
    let amount_sats = amount_str.parse::<u64>().unwrap_or(0);

    if amount_sats == 0 {
        return (format!("{}\n\n0. {}",
            TranslationService::translate("invalid_amount", lang),
            TranslationService::translate("main_menu", lang)), false);
    }

    // Get user ID first
    let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
        Ok(profile) => profile,
        Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
    };

    // Call Crypto Canister to send Bitcoin
    match crate::services::crypto_client::send_crypto(
        user_profile.id.clone(),
        btc_address.clone(),
        amount_sats,
        shared_types::CryptoType::CkBTC,
        pin.to_string()
    ).await {
        Ok(_tx_id) => {
            let btc_sent = amount_sats as f64 / 100_000_000.0;
            (format!("{}\nSent {:.8} ckBTC to {}\n\n0. {}",
                TranslationService::translate("transaction_successful", lang),
                btc_sent,
                btc_address,
                TranslationService::translate("main_menu", lang)), false)
        }
        Err(e) if e.contains("PIN") => {
            (format!("{}\n{}",
                TranslationService::translate("incorrect_pin", lang),
                TranslationService::translate("try_again", lang)), true)
        }
        Err(e) => {
            (format!("Transfer failed: {}\n\n0. {}", e, TranslationService::translate("main_menu", lang)), false)
        }
    }
}
