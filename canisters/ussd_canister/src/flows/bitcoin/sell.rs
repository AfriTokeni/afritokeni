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
            
            // Check BTC balance
            match crate::services::business_logic::get_balances(&session.phone_number).await {
                Ok(balances) => {
                    let btc_balance = balances.ckbtc_balance as f64 / 100_000_000.0;
                    
                    if btc_balance < amount_btc {
                        return (format!("{}!\n{}: {:.8} BTC\n{}: {:.8} BTC\n\n{}", 
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            btc_balance,
                            TranslationService::translate("required", lang),
                            amount_btc,
                            TranslationService::translate("thank_you", lang)), false);
                    }
                    
                    // Get current rate
                    match crate::services::business_logic::get_bitcoin_rate(&currency).await {
                        Ok(rate) => {
                            let fiat_amount = amount_btc * rate.rate_to_fiat;
                            (format!("{}\n{}: {:.8} BTC\n{}: {} {:.2}\n\n{}", 
                                TranslationService::translate("confirm_transaction", lang),
                                TranslationService::translate("amount", lang),
                                amount_btc,
                                TranslationService::translate("you_will_receive", lang),
                                currency,
                                fiat_amount,
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
            
            ic_cdk::println!("💰 Executing sell_bitcoin: amount={} sats", amount_sats);
            
            match crate::services::business_logic::sell_bitcoin(
                &session.phone_number,
                amount_sats,
                &currency,
                pin
            ).await {
                Ok(result) => {
                    let new_balance = result.new_balance as f64 / 100.0;
                    (format!("{}!\n{} {:.8} BTC\n{}: {} {:.2}\n\n{}", 
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sold", lang),
                        amount_btc,
                        TranslationService::translate("new_balance", lang),
                        currency,
                        new_balance,
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
