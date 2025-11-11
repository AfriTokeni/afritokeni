// Crypto swap flow - Swap BTC ↔ USDC
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::services::{business_logic, exchange};

/// Handle crypto swap flow
/// Steps: 0. Select from crypto → 1. Select to crypto → 2. Enter amount → 3. Show spread & confirm → 4. Enter PIN → 5. Execute swap
pub async fn handle_crypto_swap(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    // Determine step based on text input (stateless)
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Select from crypto (1=BTC, 2=USDC)
            (format!("{}\n\n{}:\n1. {} (BTC)\n2. USDC",
                TranslationService::translate("swap_crypto", lang),
                TranslationService::translate("swap_from", lang),
                TranslationService::translate("bitcoin", lang)), true)
        }
        1 => {
            // Step 1: Store from crypto, select to crypto
            let from_choice = parts.get(2).unwrap_or(&"");
            session.set_data("from_crypto", from_choice);
            
            let from_name = match from_choice {
                &"1" => TranslationService::translate("bitcoin", lang),
                &"2" => "USDC",
                _ => ""
            };
            
            (format!("{}:\n1. {} (BTC)\n2. USDC\n\n({} {})",
                TranslationService::translate("swap_to", lang),
                TranslationService::translate("bitcoin", lang),
                TranslationService::translate("swapping_from", lang),
                from_name), true)
        }
        2 => {
            // Step 2: Store to crypto, ask for amount
            let to_choice = parts.get(3).unwrap_or(&"");
            let from_choice = session.get_data("from_crypto").unwrap_or_default();
            
            // Validate not swapping same token
            if from_choice == *to_choice {
                session.clear_data();
                return (format!("❌ {}\n\n0. {}",
                    TranslationService::translate("cannot_swap_same_token", lang),
                    TranslationService::translate("main_menu", lang)), false);
            }
            
            session.set_data("to_crypto", to_choice);
            
            let from_name = match from_choice.as_str() {
                "1" => "BTC",
                "2" => "USDC",
                _ => ""
            };
            
            (format!("{} ({}):",
                TranslationService::translate("enter_amount", lang),
                from_name), true)
        }
        3 => {
            // Step 3: Calculate spread and show confirmation
            let amount_str = parts.get(4).unwrap_or(&"");
            
            match amount_str.parse::<u64>() {
                Ok(amount) => {
                    if amount == 0 {
                        return (format!("❌ {}\n\n0. {}",
                            TranslationService::translate("amount_must_be_greater_than_zero", lang),
                            TranslationService::translate("main_menu", lang)), false);
                    }
                    
                    // Fetch spread from exchange canister (NO HARDCODED VALUES!)
                    let spread_basis_points = match exchange::get_spread_basis_points().await {
                        Ok(bp) => bp,
                        Err(e) => {
                            ic_cdk::println!("❌ Failed to fetch spread: {}", e);
                            return (format!("❌ {}: {}\n\n0. {}",
                                TranslationService::translate("error", lang),
                                "Could not fetch exchange rate",
                                TranslationService::translate("main_menu", lang)), false);
                        }
                    };
                    
                    // Calculate spread using fetched value
                    let spread = (amount * spread_basis_points) / 10_000;
                    let net_amount = amount.saturating_sub(spread);
                    
                    session.set_data("amount", &amount.to_string());
                    
                    let from_choice = session.get_data("from_crypto").unwrap_or_default();
                    let to_choice = session.get_data("to_crypto").unwrap_or_default();
                    
                    let from_name = match from_choice.as_str() {
                        "1" => "BTC",
                        "2" => "USDC",
                        _ => "?"
                    };
                    
                    let to_name = match to_choice.as_str() {
                        "1" => "BTC",
                        "2" => "USDC",
                        _ => "?"
                    };
                    
                    let spread_pct = spread_basis_points as f64 / 100.0;
                    (format!("{}:\n\n{}: {} {}\n{} ({:.1}%): {} {}\n{}: ~{} {}\n\n1. {}\n2. {}",
                        TranslationService::translate("swap_details", lang),
                        TranslationService::translate("swap_from", lang), amount, from_name,
                        TranslationService::translate("spread", lang), spread_pct, spread, from_name,
                        TranslationService::translate("youll_receive_approx", lang), net_amount, to_name,
                        TranslationService::translate("confirm", lang),
                        TranslationService::translate("cancel", lang)), true)
                }
                Err(_) => {
                    session.clear_data();
                    (format!("❌ {}\n\n0. {}",
                        TranslationService::translate("invalid_amount", lang),
                        TranslationService::translate("main_menu", lang)), false)
                }
            }
        }
        4 => {
            // Step 4: Confirm and ask for PIN
            let choice = parts.get(5).unwrap_or(&"");
            
            if choice == &"1" {
                // User confirmed - ask for PIN
                (format!("{}:",
                    TranslationService::translate("enter_pin", lang)), true)
            } else {
                // User cancelled
                session.clear_data();
                (format!("{}\n\n0. {}",
                    TranslationService::translate("swap_cancelled", lang),
                    TranslationService::translate("main_menu", lang)), false)
            }
        }
        5 => {
            // Step 5: Execute swap
            let pin = parts.get(6).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let amount: u64 = session.get_data("amount").unwrap_or_default().parse().unwrap_or(0);
            let from_choice = session.get_data("from_crypto").unwrap_or_default();
            let to_choice = session.get_data("to_crypto").unwrap_or_default();
            
            // Map choices to crypto types
            let (from_crypto, to_crypto) = match (from_choice.as_str(), to_choice.as_str()) {
                ("1", "2") => ("CkBTC", "CkUSDC"),
                ("2", "1") => ("CkUSDC", "CkBTC"),
                _ => {
                    session.clear_data();
                    return (format!("❌ {}\n\n0. {}",
                        TranslationService::translate("invalid_crypto_selection", lang),
                        TranslationService::translate("main_menu", lang)), false);
                }
            };
            
            match business_logic::swap_crypto(&phone, from_crypto, to_crypto, amount, pin).await {
                Ok(result) => {
                    session.clear_data();
                    (format!("✅ {}!\n\n{}: {} {}\n{}: {} {}\n{}: {} {}\n{}: {}\n\n0. {}",
                        TranslationService::translate("swap_successful", lang),
                        TranslationService::translate("swap_from", lang), result.from_amount, from_crypto,
                        TranslationService::translate("swap_to", lang), result.to_amount, to_crypto,
                        TranslationService::translate("spread", lang), result.spread_amount, from_crypto,
                        TranslationService::translate("rate", lang), result.exchange_rate,
                        TranslationService::translate("main_menu", lang)), false)
                }
                Err(e) => {
                    session.clear_data();
                    (format!("❌ {}: {}\n\n0. {}",
                        TranslationService::translate("swap_failed", lang),
                        e,
                        TranslationService::translate("main_menu", lang)), false)
                }
            }
        }
        _ => {
            session.clear_data();
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
