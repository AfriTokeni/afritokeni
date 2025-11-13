// Sell Bitcoin flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle sell Bitcoin flow
/// Steps: 0. Enter amount → 1. Enter PIN → 2. Execute
pub async fn handle_sell_bitcoin(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    ic_cdk::println!("💰 Sell Bitcoin flow: step={}, parts={:?}", step, parts);
    
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
            let amount_str = parts.get(2).unwrap_or(&"");
            
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
                    
                    // Calculate fiat amount (rate will be determined by crypto canister)
                    let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                        Some(c) => c,
                        None => {
                            return (format!("{}: Invalid currency\n\n{}",
                                TranslationService::translate("error", lang),
                                TranslationService::translate("back_or_menu", lang)), true);
                        }
                    };
                    
                    // Show confirmation (actual rate determined at execution)
                    (format!("{}\n{}: {:.8} BTC\n{}\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("amount", lang),
                        amount_btc,
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
            let amount_btc = amount_str.parse::<f64>().unwrap_or(0.0);
            let amount_sats = (amount_btc * 100_000_000.0) as u64;
            
            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("Error: {}\n\n0. Main Menu", e), false),
            };
            
            ic_cdk::println!("💰 Executing sell_bitcoin: amount={} sats", amount_sats);
            
            let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                Some(c) => c,
                None => {
                    return (format!("{}: Invalid currency\n\n{}",
                        TranslationService::translate("error", lang),
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            match crate::services::crypto_client::sell_crypto(
                user_profile.id.clone(),
                amount_sats,
                shared_types::CryptoType::CkBTC,
                currency_enum,
                pin.to_string()
            ).await {
                Ok(result) => {
                    let fiat_received = result.fiat_amount as f64 / 100.0;
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
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
