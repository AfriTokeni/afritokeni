// Sell Bitcoin flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle sell Bitcoin flow with confirmation
/// Supports both interactive and shorthand modes:
/// - Interactive: 2*4 -> amount -> PIN -> confirm
/// - Shorthand: 2*4*AMOUNT*PIN*CONFIRMATION (or 2*4*AMOUNT*PIN if confirmation=1 is implicit)
pub async fn handle_sell_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());

    // Parse parameters upfront
    let amount_str = parts.get(2).unwrap_or(&"");
    let param_3 = parts.get(3).unwrap_or(&"");
    let param_4 = parts.get(4).unwrap_or(&"");

    ic_cdk::println!("💰 Sell Bitcoin flow: parts={:?}", parts);

    // Detect shorthand mode
    // Case 1: 2*4*AMOUNT*PIN*CONFIRMATION (5 parts)
    // Case 2: 2*4*AMOUNT*PIN (4 parts, implicit confirmation)
    let is_shorthand_with_confirm = parts.len() == 5 && !amount_str.is_empty() && !param_3.is_empty() && !param_4.is_empty();
    let is_shorthand_implicit = parts.len() == 4 && !amount_str.is_empty() && !param_3.is_empty() && param_3.len() == 4;

    if is_shorthand_with_confirm {
        // Shorthand with explicit confirmation: 2*4*AMOUNT*PIN*CONFIRMATION
        let pin = param_3;
        let confirmation = param_4;

        if *confirmation != "1" {
            return (format!("{}\n\n0. {}",
                TranslationService::translate("transaction_cancelled", lang),
                TranslationService::translate("main_menu", lang)), false);
        }

        return execute_sell_bitcoin(session, amount_str, pin, &currency, lang).await;
    } else if is_shorthand_implicit {
        // Shorthand without confirmation: 2*4*AMOUNT*PIN (implicit confirm=1)
        let pin = param_3;
        return execute_sell_bitcoin(session, amount_str, pin, &currency, lang).await;
    }

    // Interactive mode: step through the flow
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    match step {
        0 => {
            // Step 0: Ask for BTC amount to sell
            (format!("{}\n{} (BTC):\n\n{}",
                TranslationService::translate("sell_bitcoin", lang),
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
        1 => {
            // Step 1: Validate amount, check balance, ask for PIN
            let amount_btc = match validation::parse_amount(amount_str) {
                Ok(amt) => amt,
                Err(e) => {
                    return (format!("{}\n{} (BTC):", e, TranslationService::translate("enter_amount", lang)), true);
                }
            };

            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };

            // Check BTC balance from crypto canister
            match crate::services::crypto_client::check_crypto_balance(
                user_profile.id.clone(),
                shared_types::CryptoType::CkBTC
            ).await {
                Ok(balance_sats) => {
                    let btc_balance = balance_sats as f64 / 100_000_000.0;

                    if btc_balance < amount_btc {
                        return (format!("{}!\n{}: {:.8} BTC\n{}: {:.8} BTC\n\n{}",
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            btc_balance,
                            TranslationService::translate("required", lang),
                            amount_btc,
                            TranslationService::translate("thank_you", lang)), false);
                    }

                    // Validate currency
                    let _currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                        Some(c) => c,
                        None => {
                            return (format!("{}: Invalid currency\n\n{}",
                                TranslationService::translate("error", lang),
                                TranslationService::translate("back_or_menu", lang)), true);
                        }
                    };

                    // Store amount for next step
                    session.set_data("amount_sats", &((amount_btc * 100_000_000.0) as u64).to_string());

                    (format!("{} {:.8} BTC\n{}",
                        TranslationService::translate("selling", lang),
                        amount_btc,
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
        2 => {
            // Step 2: Show preview and ask for confirmation
            let pin = param_3;

            let amount_sats = match amount_str.parse::<f64>() {
                Ok(amt) if amt > 0.0 => (amt * 100_000_000.0) as u64,
                _ => 0,
            };
            let amount_btc = amount_sats as f64 / 100_000_000.0;

            // Store for next step
            session.set_data("amount_sats", &amount_sats.to_string());
            session.set_data("pin", pin);

            (format!("💰 {}:\n\n{}: {:.8} BTC\n{}: ~{} (est)\n\n1. {}\n2. {}",
                TranslationService::translate("confirm_transaction", lang),
                TranslationService::translate("selling", lang),
                amount_btc,
                TranslationService::translate("you_receive", lang),
                currency,
                TranslationService::translate("confirm", lang),
                TranslationService::translate("cancel", lang)), true)
        }
        3 => {
            // Step 3: Execute sell after confirmation
            let confirmation = param_4;

            if *confirmation != "1" {
                return (format!("{}\n\n0. {}",
                    TranslationService::translate("transaction_cancelled", lang),
                    TranslationService::translate("main_menu", lang)), false);
            }

            // Get amount and PIN from parts (stateless) or session (stateful)
            let amount_sats = if let Some(amt_str) = parts.get(2) {
                amt_str.parse::<f64>().ok()
                    .map(|amt| (amt * 100_000_000.0) as u64)
                    .unwrap_or(0)
            } else {
                session.get_data("amount_sats")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0)
            };

            let pin = parts.get(3)
                .map(|s| s.to_string())
                .or_else(|| session.get_data("pin"))
                .unwrap_or_default();

            execute_sell_bitcoin_raw(session, amount_sats, &pin, &currency, lang).await
        }
        _ => {
            (format!("{}\n\n{}",
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}

/// Execute sell Bitcoin transaction from amount string (helper function for shorthand)
async fn execute_sell_bitcoin(
    session: &UssdSession,
    amount_str: &str,
    pin: &str,
    currency: &str,
    lang: Language,
) -> (String, bool) {
    // Parse amount from BTC string
    let amount_sats = match amount_str.parse::<f64>() {
        Ok(amt) if amt > 0.0 => (amt * 100_000_000.0) as u64,
        _ => {
            return (format!("{}\n\n0. {}",
                TranslationService::translate("invalid_amount", lang),
                TranslationService::translate("main_menu", lang)), false);
        }
    };

    execute_sell_bitcoin_raw(session, amount_sats, pin, currency, lang).await
}

/// Execute sell Bitcoin transaction from satoshis (helper function for interactive)
async fn execute_sell_bitcoin_raw(
    session: &UssdSession,
    amount_sats: u64,
    pin: &str,
    currency: &str,
    lang: Language,
) -> (String, bool) {
    if amount_sats == 0 {
        return (format!("{}\n\n0. {}",
            TranslationService::translate("invalid_amount", lang),
            TranslationService::translate("main_menu", lang)), false);
    }

    let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
        Ok(profile) => profile,
        Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
    };

    let currency_enum = match shared_types::FiatCurrency::from_code(currency) {
        Some(c) => c,
        None => return (format!("Error: Invalid currency\n\n0. Main Menu"), false),
    };

    ic_cdk::println!("💰 Executing sell_bitcoin: amount={} sats", amount_sats);

    match crate::services::crypto_client::sell_crypto(
        user_profile.id.clone(),
        amount_sats,
        shared_types::CryptoType::CkBTC,
        currency_enum,
        pin.to_string()
    ).await {
        Ok(result) => {
            let fiat_received = result.fiat_amount as f64 / 100.0;
            let amount_btc = amount_sats as f64 / 100_000_000.0;
            (format!("{}!\n{} {:.8} BTC\n{}: {} {:.2}\n\n{}",
                TranslationService::translate("transaction_successful", lang),
                TranslationService::translate("sold", lang),
                amount_btc,
                TranslationService::translate("received", lang),
                currency,
                fiat_received,
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
