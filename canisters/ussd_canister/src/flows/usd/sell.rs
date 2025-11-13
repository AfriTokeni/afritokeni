// Sell USDC flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle sell USDC flow
/// Steps: 0. Enter amount → 1. Enter PIN → 2. Execute
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
            // Step 1: Validate amount, check balance, ask for PIN
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
            
            // Check USDC balance
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
                    
                    // Show confirmation (rate determined at execution)
                    (format!("{}\n{}: {:.2} USDC\n{}\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("amount", lang),
                        amount_usdc,
                        TranslationService::translate("you_will_receive", lang),
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
            // Step 2: Execute sell
            let pin = parts.get(3).unwrap_or(&"");
            let amount_str = parts.get(2).unwrap_or(&"").to_string();

            // Parse and validate amount
            let amount_e6 = match amount_str.parse::<f64>() {
                Ok(amt) if amt > 0.0 => (amt * 1_000_000.0) as u64,
                _ => {
                    return (format!("{}\n\n{}",
                        TranslationService::translate("invalid_amount", lang),
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            ic_cdk::println!("💵 Executing sell_usdc: amount={} e6", amount_e6);
            
            let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                Some(c) => c,
                None => return (format!("Invalid currency\n\n0. {}", TranslationService::translate("main_menu", lang)), false),
            };
            
            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };
            
            match crate::services::crypto_client::sell_crypto(
                user_profile.id.clone(),
                amount_e6,
                shared_types::CryptoType::CkUSDC,
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
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
