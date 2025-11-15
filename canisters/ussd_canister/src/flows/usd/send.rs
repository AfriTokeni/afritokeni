// Send USDC flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send USDC flow
/// Supports both interactive and shorthand modes:
/// - Interactive: 3*5 -> address -> amount -> PIN
/// - Shorthand: 3*5*ADDRESS*AMOUNT*PIN
pub async fn handle_send_usdc(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };

    ic_cdk::println!("💵 Send USDC flow: step={}, parts={:?}", step, parts);

    match step {
        0 => {
            // Step 0: Ask for USDC address
            (format!("{}\n{}",
                TranslationService::translate("send_usdc", lang),
                TranslationService::translate("enter_usdc_address", lang)), true)
        }
        1 => {
            // Step 1: Validate USDC address (parts[2])
            // Text: "3*5*address" -> parts[0]=3, parts[1]=5, parts[2]=address
            let address_raw = parts.get(2).unwrap_or(&"");
            let address = validation::sanitize_input(address_raw);

            // Validate address format (IC Principal for ckUSDC)
            if !validation::is_valid_usdc_address(&address) {
                return (format!("Invalid USDC address format\n{}",
                    TranslationService::translate("try_again", lang)), true);
            }

            (format!("{} (ckUSDC):", TranslationService::translate("enter_amount", lang)), true)
        }
        2 => {
            // Step 2: Validate amount and check balance (parts[3])
            // Text: "3*5*address*amount" -> parts[3]=amount
            let amount_str = parts.get(3).unwrap_or(&"");

            // Parse amount in e6 units (micro-USDC), consistent with Bitcoin flow (satoshis)
            let amount_e6 = match amount_str.parse::<u64>() {
                Ok(amt) if amt > 0 => amt,
                Ok(_) | Err(_) => {
                    return (format!("{}\n{}",
                        TranslationService::translate("invalid_amount", lang),
                        TranslationService::translate("try_again", lang)), true);
                }
            };

            // Get user profile to check balance
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };

            // Check USDC balance
            match crate::services::crypto_client::check_crypto_balance(
                user_profile.id.clone(),
                shared_types::CryptoType::CkUSD
            ).await {
                Ok(balance_e6) => {
                    if balance_e6 < amount_e6 {
                        return (format!("{}!\n{}: {:.2} USDC\n{}: {:.2} USDC\n\n{}",
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            balance_e6 as f64 / 1_000_000.0,
                            TranslationService::translate("required", lang),
                            amount_e6 as f64 / 1_000_000.0,
                            TranslationService::translate("back_or_menu", lang)), true);
                    }

                    let address = parts.get(2).unwrap_or(&"");
                    (format!("{}\n{}: {}\n{}: {} ckUSDC\n\n{}",
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("to", lang),
                        address,
                        TranslationService::translate("amount", lang),
                        amount_e6 as f64 / 1_000_000.0,
                        TranslationService::translate("enter_pin_confirm", lang)), true)
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
            // Step 3: Verify PIN and execute real ckUSDC transfer
            // parts: [0]=3, [1]=5, [2]=address, [3]=amount, [4]=PIN
            let pin = parts.get(4).unwrap_or(&"");
            let usdc_address = parts.get(2).unwrap_or(&"").to_string();
            let amount_str = parts.get(3).unwrap_or(&"").to_string();

            // Parse amount in e6 units (micro-USDC), consistent with Bitcoin flow (satoshis)
            let amount_e6 = amount_str.parse::<u64>().unwrap_or(0);

            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };

            // Call Crypto Canister to send USDC
            match crate::services::crypto_client::send_crypto(
                user_profile.id.clone(),
                usdc_address.clone(),
                amount_e6,
                shared_types::CryptoType::CkUSD,
                pin.to_string()
            ).await {
                Ok(_tx_id) => {
                    let usdc_sent = amount_e6 as f64 / 1_000_000.0;
                    session.clear_data();
                    (format!("{}\nSent {:.2} ckUSDC to {}\n\n0. {}",
                        TranslationService::translate("transaction_successful", lang),
                        usdc_sent,
                        usdc_address,
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
