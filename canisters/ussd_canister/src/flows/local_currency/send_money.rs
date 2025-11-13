// Send Money flow with PIN verification
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::utils::validation;

/// Handle send money flow
/// Steps: 0. Enter recipient → 1. Enter amount → 2. Enter PIN → 3. Execute
pub async fn handle_send_money(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    // Determine step based on number of parts after "1*1": "1*1" = step 0, "1*1*phone" = step 1, etc.
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    ic_cdk::println!("💸 Send Money flow: step={}, parts={:?}", step, parts);
    
    match step {
        0 => {
            // Step 0: Ask for recipient phone
            (format!("{}\n{}\n{}\n\n{}", 
                TranslationService::translate("send_money", lang),
                TranslationService::translate("enter_recipient_phone", lang),
                TranslationService::translate("phone_format_example", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
        1 => {
            // Step 1: Validate recipient phone and ask for amount
            let recipient_raw = parts.get(2).unwrap_or(&"");
            let recipient = validation::sanitize_input(recipient_raw);
            
            // Validate phone format
            if !validation::is_valid_phone(&recipient) {
                return (format!("{}\n{}\n{}\n\n{}", 
                    TranslationService::translate("invalid_phone", lang),
                    TranslationService::translate("enter_recipient_phone", lang),
                    TranslationService::translate("phone_format_example", lang),
                    TranslationService::translate("back_or_menu", lang)), true);
            }
            
            // Ask for amount
            (format!("{} ({}):", 
                TranslationService::translate("enter_amount", lang),
                currency), true)
        }
        2 => {
            // Step 2: Validate amount, check balance, ask for PIN
            let amount_str = parts.get(3).unwrap_or(&"");
            
            let amount_f64 = match validation::parse_amount(amount_str) {
                Ok(amt) => amt,
                Err(e) => {
                    return (format!("{}\n{} ({}):", 
                        e,
                        TranslationService::translate("enter_amount", lang),
                        currency), true);
                }
            };
            
            // Calculate fee (0.5% platform fee)
            let fee = (amount_f64 * 0.005).round();
            let total_required = amount_f64 + fee;
            
            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => {
                    return (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("back_or_menu", lang)), true);
                }
            };
            
            // Check user balance from wallet canister
            let currency_enum = shared_types::FiatCurrency::from_code(&currency)
                .ok_or_else(|| format!("Invalid currency: {}", currency));
            
            let currency_enum = match currency_enum {
                Ok(c) => c,
                Err(e) => {
                    return (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            match crate::services::wallet_client::get_fiat_balance(user_profile.id.clone(), currency_enum).await {
                Ok(balance_cents) => {
                    let fiat_balance = balance_cents as f64 / 100.0;
                    
                    if fiat_balance < total_required {
                        return (format!("{}!\n{}: {} {:.2}\n{}: {} {:.2} ({}: {:.2} + {}: {:.2})\n\n{}", 
                            TranslationService::translate("insufficient_balance", lang),
                            TranslationService::translate("your_balance", lang),
                            currency,
                            fiat_balance,
                            TranslationService::translate("required", lang),
                            currency,
                            total_required,
                            TranslationService::translate("amount", lang),
                            amount_f64,
                            TranslationService::translate("fee", lang),
                            fee,
                            TranslationService::translate("thank_you", lang)), false);
                    }
                    
                    // Balance sufficient, ask for PIN
                    let recipient = parts.get(2).unwrap_or(&"");
                    (format!("{}\n{}: {}\n{}: {} {:.2}\n{}: {} {:.2}\n{}: {} {:.2}\n\n{}", 
                        TranslationService::translate("confirm_transaction", lang),
                        TranslationService::translate("to", lang),
                        recipient,
                        TranslationService::translate("amount", lang),
                        currency,
                        amount_f64,
                        TranslationService::translate("fee", lang),
                        currency,
                        fee,
                        TranslationService::translate("total", lang),
                        currency,
                        total_required,
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
        3 => {
            // Step 3: Execute transaction with PIN
            let pin = parts.get(4).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let recipient_phone = parts.get(2).unwrap_or(&"").to_string();
            let amount_str = parts.get(3).unwrap_or(&"").to_string();

            // Parse and validate amount
            let amount_cents = match amount_str.parse::<f64>() {
                Ok(amt) if amt > 0.0 => (amt * 100.0) as u64,
                _ => {
                    return (format!("{}\n\n{}",
                        TranslationService::translate("invalid_amount", lang),
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            ic_cdk::println!("💸 Executing send_money: from={}, to={}, amount={} cents, currency={}", 
                phone, recipient_phone, amount_cents, currency);
            
            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(phone.clone()).await {
                Ok(profile) => profile,
                Err(e) => {
                    return (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            let recipient_profile = match crate::services::user_client::get_user_by_phone(recipient_phone.clone()).await {
                Ok(profile) => profile,
                Err(e) => {
                    return (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            let currency_enum = shared_types::FiatCurrency::from_code(&currency)
                .ok_or_else(|| format!("Invalid currency: {}", currency));
            
            let currency_enum = match currency_enum {
                Ok(c) => c,
                Err(e) => {
                    return (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("thank_you", lang)), false);
                }
            };
            
            // Call Wallet Canister to transfer money
            match crate::services::wallet_client::transfer_fiat(
                user_profile.id,
                recipient_profile.id,
                amount_cents,
                currency_enum,
                pin.to_string()
            ).await {
                Ok(tx_result) => {
                    let new_balance = (tx_result.sender_new_balance as f64) / 100.0;
                    let amount_f64 = amount_cents as f64 / 100.0;
                    (format!("{}!\n{} {} {:.2} {} {}\n{}: {} {:.2}\n\n{}",
                        TranslationService::translate("transaction_successful", lang),
                        TranslationService::translate("sent", lang),
                        currency,
                        amount_f64,
                        TranslationService::translate("to", lang),
                        recipient_phone,
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
            // Invalid state
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
