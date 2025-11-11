// Crypto swap flow - Swap BTC ↔ USDC
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::services::business_logic;

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
            (format!("🔄 Swap Crypto\n\nFrom:\n1. Bitcoin (BTC)\n2. USDC"), true)
        }
        1 => {
            // Step 1: Store from crypto, select to crypto
            let from_choice = parts.get(2).unwrap_or(&"");
            session.set_data("from_crypto", from_choice);
            
            let from_name = match from_choice {
                &"1" => "Bitcoin (BTC)",
                &"2" => "USDC",
                _ => "Unknown"
            };
            
            (format!("To:\n1. Bitcoin (BTC)\n2. USDC\n\n(Swapping from {})", from_name), true)
        }
        2 => {
            // Step 2: Store to crypto, ask for amount
            let to_choice = parts.get(3).unwrap_or(&"");
            let from_choice = session.get_data("from_crypto").unwrap_or_default();
            
            // Validate not swapping same token
            if from_choice == to_choice {
                session.clear_data();
                return (format!("❌ Cannot swap same token\n\n0. Main Menu"), false);
            }
            
            session.set_data("to_crypto", to_choice);
            
            let from_name = match from_choice.as_str() {
                "1" => "BTC",
                "2" => "USDC",
                _ => "Unknown"
            };
            
            (format!("Enter amount ({}):", from_name), true)
        }
        3 => {
            // Step 3: Calculate spread and show confirmation
            let amount_str = parts.get(4).unwrap_or(&"");
            
            match amount_str.parse::<u64>() {
                Ok(amount) => {
                    if amount == 0 {
                        return (format!("❌ Amount must be greater than 0\n\n0. Main Menu"), false);
                    }
                    
                    // Calculate spread (0.5%)
                    let spread = (amount * 50) / 10_000;
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
                    
                    (format!("🔄 Swap Details:\n\nFrom: {} {}\nSpread (0.5%): {} {}\nYou'll receive: ~{} {}\n\n1. Confirm\n2. Cancel",
                        amount, from_name,
                        spread, from_name,
                        net_amount, to_name), true)
                }
                Err(_) => {
                    session.clear_data();
                    (format!("❌ Invalid amount\n\n0. Main Menu"), false)
                }
            }
        }
        4 => {
            // Step 4: Confirm and ask for PIN
            let choice = parts.get(5).unwrap_or(&"");
            
            if choice == &"1" {
                // User confirmed - ask for PIN
                (format!("🔐 Enter your PIN:"), true)
            } else {
                // User cancelled
                session.clear_data();
                (format!("Swap cancelled\n\n0. Main Menu"), false)
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
                    return (format!("❌ Invalid crypto selection\n\n0. Main Menu"), false);
                }
            };
            
            match business_logic::swap_crypto(&phone, from_crypto, to_crypto, amount, pin).await {
                Ok(result) => {
                    session.clear_data();
                    (format!("✅ Swap Successful!\n\n📊 Details:\nFrom: {} {}\nTo: {} {}\nSpread: {} {}\nRate: {}\n\n0. Main Menu",
                        result.from_amount, from_crypto,
                        result.to_amount, to_crypto,
                        result.spread_amount, from_crypto,
                        result.exchange_rate), false)
                }
                Err(e) => {
                    session.clear_data();
                    (format!("❌ Swap failed: {}\n\n0. Main Menu", e), false)
                }
            }
        }
        _ => {
            session.clear_data();
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
