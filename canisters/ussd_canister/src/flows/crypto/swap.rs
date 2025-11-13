// Crypto swap flow - Swap BTC ↔ USDC
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::services::exchange;

/// Handle crypto swap flow
/// Steps: 0. Select from crypto → 1. Select to crypto → 2. Enter amount → 3. Show spread & confirm → 4. Enter PIN → 5. Execute swap
pub async fn handle_crypto_swap(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    // Determine step based on text input (stateless)
    // parts: ["4"] = step 0, ["4","1"] = step 0, ["4","1","2"] = step 1, etc.
    // Same pattern as other flows: parts.len() - 2
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    ic_cdk::println!("🔄 Swap flow: parts={:?}, len={}, step={}", parts, parts.len(), step);
    
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
            let from_choice = parts.get(1).unwrap_or(&"");
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
            // Step 2: Check if amount is provided
            let to_choice = parts.get(2).unwrap_or(&"");
            let from_choice = parts.get(1).unwrap_or(&""); // Read from parts, not session
            let amount_str = parts.get(3).unwrap_or(&"");
            
            // Validate not swapping same token
            if from_choice == to_choice {
                session.clear_data();
                return (format!("❌ {}\n\n0. {}",
                    TranslationService::translate("cannot_swap_same_token", lang),
                    TranslationService::translate("main_menu", lang)), false);
            }
            
            // If amount is provided, show confirmation (step 3 logic)
            if !amount_str.is_empty() {
                match amount_str.parse::<u64>() {
                    Ok(amount) => {
                        if amount == 0 {
                            return (format!("❌ {}\n\n0. {}",
                                TranslationService::translate("amount_must_be_greater_than_zero", lang),
                                TranslationService::translate("main_menu", lang)), false);
                        }
                        
                        // Fetch spread from exchange canister
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
                        
                        let spread = (amount * spread_basis_points) / 10_000;
                        let net_amount = amount.saturating_sub(spread);
                        
                        let from_name = match *from_choice {
                            "1" => "BTC",
                            "2" => "USDC",
                            _ => "?"
                        };
                        
                        let to_name = match *to_choice {
                            "1" => "BTC",
                            "2" => "USDC",
                            _ => "?"
                        };
                        
                        let spread_pct = spread_basis_points as f64 / 100.0;
                        return (format!("{}:\n\n{}: {} {}\n{} ({:.1}%): {} {}\n{}: ~{} {}\n\n1. {}\n2. {}",
                            TranslationService::translate("swap_details", lang),
                            TranslationService::translate("swap_from", lang), amount, from_name,
                            TranslationService::translate("spread", lang), spread_pct, spread, from_name,
                            TranslationService::translate("youll_receive_approx", lang), net_amount, to_name,
                            TranslationService::translate("confirm", lang),
                            TranslationService::translate("cancel", lang)), true);
                    }
                    Err(_) => {
                        session.clear_data();
                        return (format!("❌ {}\n\n0. {}",
                            TranslationService::translate("invalid_amount", lang),
                            TranslationService::translate("main_menu", lang)), false);
                    }
                }
            }
            
            // No amount provided, ask for it
            session.set_data("to_crypto", to_choice);
            
            let from_name = match *from_choice {
                "1" => "BTC",
                "2" => "USDC",
                _ => ""
            };
            
            (format!("{} ({}):",
                TranslationService::translate("enter_amount", lang),
                from_name), true)
        }
        3 => {
            // Step 3: Handle confirmation choice
            let choice = parts.get(4).unwrap_or(&"");
            
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
        4 => {
            // Step 4: Execute swap with PIN
            let pin = parts.get(5).unwrap_or(&"");
            let phone = session.phone_number.clone();
            let amount_str = parts.get(3).unwrap_or(&"");
            let amount: u64 = amount_str.parse().unwrap_or(0);
            let from_choice = parts.get(1).unwrap_or(&"");
            let to_choice = parts.get(2).unwrap_or(&"");
            
            // Map choices to crypto types
            let (from_crypto, to_crypto) = match (*from_choice, *to_choice) {
                ("1", "2") => ("CkBTC", "CkUSDC"),
                ("2", "1") => ("CkUSDC", "CkBTC"),
                _ => {
                    session.clear_data();
                    return (format!("❌ {}\n\n0. {}",
                        TranslationService::translate("invalid_crypto_selection", lang),
                        TranslationService::translate("main_menu", lang)), false);
                }
            };
            
            let from_type = match from_crypto {
                "CkBTC" => shared_types::CryptoType::CkBTC,
                "CkUSDC" => shared_types::CryptoType::CkUSDC,
                _ => return (format!("Invalid crypto type\n\n0. {}", TranslationService::translate("main_menu", lang)), false),
            };
            
            let to_type = match to_crypto {
                "CkBTC" => shared_types::CryptoType::CkBTC,
                "CkUSDC" => shared_types::CryptoType::CkUSDC,
                _ => return (format!("Invalid crypto type\n\n0. {}", TranslationService::translate("main_menu", lang)), false),
            };
            
            match crate::services::crypto_client::swap_crypto(
                phone.clone(),
                from_type,
                to_type,
                amount,
                pin.to_string()
            ).await {
                Ok(result) => {
                    session.clear_data();
                    (format!("✅ {}!\n\n{}: {} {}\n{}: {} {}\n{}: {} bps\n{}: {}\n\n0. {}",
                        TranslationService::translate("swap_successful", lang),
                        TranslationService::translate("swap_from", lang), result.from_amount, from_crypto,
                        TranslationService::translate("swap_to", lang), result.to_amount, to_crypto,
                        TranslationService::translate("spread", lang), result.spread_bps,
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
