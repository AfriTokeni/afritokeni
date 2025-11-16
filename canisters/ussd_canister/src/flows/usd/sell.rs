// Sell USDC flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle sell USDC flow with confirmation
/// Supports both interactive and shorthand modes:
/// - Interactive: 3*4 -> amount -> PIN -> confirmation
/// - Shorthand: 3*4*AMOUNT*PIN*CONFIRMATION (or 3*4*AMOUNT*PIN if confirmation=1 is implicit)
pub async fn handle_sell_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());

    // Parse parameters upfront
    let amount_str = parts.get(2).unwrap_or(&"");
    let param_3 = parts.get(3).unwrap_or(&"");
    let param_4 = parts.get(4).unwrap_or(&"");

    ic_cdk::println!("💵 Sell USDC flow: parts={:?}", parts);

    // Detect shorthand mode
    // Case 1: 3*4*AMOUNT*PIN*CONFIRMATION (5 parts)
    // Case 2: 3*4*AMOUNT*PIN (4 parts, implicit confirmation)
    let is_shorthand_with_confirm = parts.len() == 5 && !amount_str.is_empty() && !param_3.is_empty() && !param_4.is_empty();
    let is_shorthand_implicit = parts.len() == 4 && !amount_str.is_empty() && !param_3.is_empty() && param_3.len() == 4;

    if is_shorthand_with_confirm {
        // Shorthand with explicit confirmation: 3*4*AMOUNT*PIN*CONFIRMATION
        let pin = param_3;
        let confirmation = param_4;

        if *confirmation != "1" {
            return (format!("{}\n\n0. {}",
                TranslationService::translate("transaction_cancelled", lang),
                TranslationService::translate("main_menu", lang)), false);
        }

        return execute_sell_usdc(session, amount_str, pin, &currency, lang).await;
    } else if is_shorthand_implicit {
        // Shorthand without confirmation: 3*4*AMOUNT*PIN (implicit confirm=1)
        let pin = param_3;
        return execute_sell_usdc(session, amount_str, pin, &currency, lang).await;
    }

    // Interactive mode: step through the flow
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    match step {
        0 => {
            // Step 0: Ask for USDC amount to sell
            (format!("{}\n{} (USDC):\n\n{}",
                TranslationService::translate("sell_usdc", lang),
                TranslationService::translate("enter_amount", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
        1 => {
            // Step 1: Validate amount and check balance
            // parts = [3, 4, amount]
            let amount_str = parts.get(2).unwrap_or(&"");

            let amount_usdc = match validation::parse_amount(amount_str) {
                Ok(amt) => amt,
                Err(e) => {
                    return (format!("{}\n{} (USDC):", e, TranslationService::translate("enter_amount", lang)), true);
                }
            };

            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };

            // Check USDC balance from crypto canister
            match crate::services::crypto_client::check_crypto_balance(
                user_profile.id.clone(),
                shared_types::CryptoType::CkUSD
            ).await {
                Ok(balance_e6) => {
                    let usdc_balance = balance_e6 as f64 / 1_000_000.0;

                    if usdc_balance < amount_usdc {
                        return (format!("{}!\n{}: {:.2} USDC\n{}: {:.2} USDC\n\n{}",
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            usdc_balance,
                            TranslationService::translate("required", lang),
                            amount_usdc,
                            TranslationService::translate("thank_you", lang)), false);
                    }

                    // Validate currency (validation will be done again at execution)
                    let _currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                        Some(c) => c,
                        None => {
                            return (format!("{}: Invalid currency\n\n{}",
                                TranslationService::translate("error", lang),
                                TranslationService::translate("back_or_menu", lang)), true);
                        }
                    };

                    // Store amount for next step
                    session.set_data("amount_e6", &((amount_usdc * 1_000_000.0) as u64).to_string());

                    (format!("{} {:.2} USDC\n{}",
                        TranslationService::translate("selling", lang),
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
        2 => {
            // Step 2: Verify PIN first
            // parts = [3, 4, amount, PIN]
            let amount_str = parts.get(2).unwrap_or(&"");
            let pin = parts.get(3).unwrap_or(&"");

            let amount_e6 = match amount_str.parse::<f64>() {
                Ok(amt) if amt > 0.0 => (amt * 1_000_000.0) as u64,
                _ => {
                    return (format!("{}\n\n{}",
                        TranslationService::translate("invalid_amount", lang),
                        TranslationService::translate("back_or_menu", lang)), true);
                }
            };
            let amount_usdc = amount_e6 as f64 / 1_000_000.0;

            // Get user profile to verify PIN
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };

            // Verify PIN BEFORE showing confirmation
            match crate::services::user_client::verify_pin(user_profile.id.clone(), pin.to_string()).await {
                Ok(true) => {
                    // PIN is valid, store for next step and show confirmation
                    session.set_data("amount_e6", &amount_e6.to_string());
                    session.set_data("pin", pin);

                    // TODO: Get actual exchange rate from crypto canister
                    // For now, show estimated values
                    (format!("💰 {}:

{}: {:.2} USDC
{}: ~{} (est)

1. {}
2. {}",
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("selling", lang),
                        amount_usdc,
                        TranslationService::translate("you_receive", lang),
                        currency,
                        TranslationService::translate("confirm", lang),
                        TranslationService::translate("cancel", lang)), true)
                }
                Ok(false) | Err(_) => {
                    (format!("{}\n\n{}",
                        TranslationService::translate("incorrect_pin", lang),
                        TranslationService::translate("back_or_menu", lang)), false)
                }
            }
        }
        3 => {
            // Step 3: Execute sell
            // parts = [3, 4, amount, PIN, confirmation]
            let confirmation = parts.get(4).unwrap_or(&"");

            if confirmation != &"1" {
                return (format!("{}

0. {}",
                    TranslationService::translate("transaction_cancelled", lang),
                    TranslationService::translate("main_menu", lang)), false);
            }

            // Get amount and PIN from parts (stateless USSD) or session (stateful)
            let amount_e6 = if let Some(amt_str) = parts.get(2) {
                amt_str.parse::<f64>().ok()
                    .map(|amt| (amt * 1_000_000.0) as u64)
                    .unwrap_or(0)
            } else {
                session.get_data("amount_e6")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0)
            };

            let pin = parts.get(3)
                .map(|s| s.to_string())
                .or_else(|| session.get_data("pin"))
                .unwrap_or_default();

            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}

0. Main Menu", e), false),
            };

            let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                Some(c) => c,
                None => return (format!("Error: Invalid currency

0. Main Menu"), false),
            };

            ic_cdk::println!("💵 Executing sell_usdc: amount={} e6", amount_e6);

            match crate::services::crypto_client::sell_crypto(
                user_profile.id.clone(),
                amount_e6,
                shared_types::CryptoType::CkUSD,
                currency_enum,
                pin
            ).await {
                Ok(result) => {
                    let fiat_received = result.fiat_amount as f64 / 100.0;
                    let amount_usdc = amount_e6 as f64 / 1_000_000.0;
                    (format!("{}!
{} {:.2} USDC
{}: {} {:.2}

{}",
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sold", lang),
                        amount_usdc,
                        TranslationService::translate("received", lang),
                        currency,
                        fiat_received,
                        TranslationService::translate("thank_you", lang)), false)
                }
                Err(e) => {
                    (format!("{}: {}

{}",
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

/// Execute sell USDC transaction from amount string (helper function for shorthand)
async fn execute_sell_usdc(
    session: &UssdSession,
    amount_str: &str,
    pin: &str,
    currency: &str,
    lang: Language,
) -> (String, bool) {
    // Parse amount from USDC string
    let amount_e6 = match amount_str.parse::<f64>() {
        Ok(amt) if amt > 0.0 => (amt * 1_000_000.0) as u64,
        _ => {
            return (format!("{}\n\n0. {}",
                TranslationService::translate("invalid_amount", lang),
                TranslationService::translate("main_menu", lang)), false);
        }
    };

    execute_sell_usdc_raw(session, amount_e6, pin, currency, lang).await
}

/// Execute sell USDC transaction from e6 amount (helper function for interactive)
async fn execute_sell_usdc_raw(
    session: &UssdSession,
    amount_e6: u64,
    pin: &str,
    currency: &str,
    lang: Language,
) -> (String, bool) {
    if amount_e6 == 0 {
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

    ic_cdk::println!("💵 Executing sell_usdc: amount={} e6", amount_e6);

    match crate::services::crypto_client::sell_crypto(
        user_profile.id.clone(),
        amount_e6,
        shared_types::CryptoType::CkUSD,
        currency_enum,
        pin.to_string()
    ).await {
        Ok(result) => {
            let fiat_received = result.fiat_amount as f64 / 100.0;
            let amount_usdc = amount_e6 as f64 / 1_000_000.0;
            (format!("{}!\n{} {:.2} USDC\n{}: {} {:.2}\n\n{}",
                TranslationService::translate("transaction_successful", lang),
                TranslationService::translate("sold", lang),
                amount_usdc,
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
