// Sell USDC flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle sell USDC flow with confirmation
/// Supports both interactive and shorthand modes:
/// - Interactive: 3*4 -> amount -> PIN -> confirmation
/// - Shorthand: 3*4*AMOUNT*PIN*CONFIRMATION
pub async fn handle_sell_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());

    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    ic_cdk::println!("💵 Sell USDC flow: step={}, parts={:?}", step, parts);

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
                shared_types::CryptoType::CkUSDC
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
            // Step 2: Show preview and ask for confirmation
            // parts = [3, 4, amount, PIN]
            let amount_str = parts.get(2).unwrap_or(&"");
            let pin = parts.get(3).unwrap_or(&"");

            let amount_e6 = match amount_str.parse::<f64>() {
                Ok(amt) if amt > 0.0 => (amt * 1_000_000.0) as u64,
                _ => 0,
            };
            let amount_usdc = amount_e6 as f64 / 1_000_000.0;

            // Store for next step
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
                shared_types::CryptoType::CkUSDC,
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
