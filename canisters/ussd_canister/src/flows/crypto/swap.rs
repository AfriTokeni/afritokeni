// Crypto swap flow - Swap BTC ↔ USDC
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::services::crypto_client;

/// Handle crypto swap flow
/// Format: 4*FROM_CRYPTO*TO_CRYPTO*AMOUNT*CONFIRMATION*PIN
/// FROM/TO: 1=BTC, 2=USDC
/// CONFIRMATION: 1=Confirm, 2=Cancel
///
/// Steps:
/// 0. Select from crypto (1=BTC, 2=USDC)
/// 1. Select to crypto (1=BTC, 2=USDC)
/// 2. Enter amount (shows confirmation with spread)
/// 3. Confirm/Cancel (1=Confirm, 2=Cancel)
/// 4. Enter PIN and execute swap
pub async fn handle_crypto_swap(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // Parse all parameters upfront for both shorthand and interactive modes
    let from_choice = parts.get(1).unwrap_or(&"");
    let to_choice = parts.get(2).unwrap_or(&"");
    let amount_str = parts.get(3).unwrap_or(&"");
    let confirmation = parts.get(4).unwrap_or(&"");
    let pin = parts.get(5).unwrap_or(&"");

    // Determine step based on text input (stateless)
    // parts: ["4"] = step 0, ["4","1"] = step 1, ["4","1","2"] = step 2, etc.
    // Correct calculation: step = parts.len() - 1
    let step = parts.len() - 1;

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
            // Step 1: Select to crypto
            // Validate from_choice
            if !from_choice.is_empty() && from_choice != &"1" && from_choice != &"2" {
                session.clear_data();
                return (format!("❌ {}\n\n0. {}",
                    TranslationService::translate("invalid_selection", lang),
                    TranslationService::translate("main_menu", lang)), false);
            }

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
            // Step 2: To crypto selected, validate and ask for amount
            // Validate same-currency rejection early
            if from_choice == to_choice && !from_choice.is_empty() && !to_choice.is_empty() {
                session.clear_data();
                return (format!("❌ {}\n\n0. {}",
                    TranslationService::translate("cannot_swap_same_token", lang),
                    TranslationService::translate("main_menu", lang)), false);
            }

            // Interactive mode: ask for amount
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
            // Step 3: Amount entered, show confirmation with spread
            show_swap_confirmation(from_choice, to_choice, amount_str, lang).await
        }
        4 => {
            // Step 4: Handle confirmation choice
            if confirmation == &"1" {
                // User confirmed - ask for PIN
                (format!("{}:",
                    TranslationService::translate("enter_pin", lang)), true)
            } else if confirmation == &"2" {
                // User cancelled
                session.clear_data();
                (format!("{}\n\n0. {}",
                    TranslationService::translate("swap_cancelled", lang),
                    TranslationService::translate("main_menu", lang)), false)
            } else {
                // Invalid confirmation choice
                session.clear_data();
                (format!("❌ {}\n\n0. {}",
                    TranslationService::translate("invalid_selection", lang),
                    TranslationService::translate("main_menu", lang)), false)
            }
        }
        5 => {
            // Step 5: Execute swap with PIN
            let phone = session.phone_number.clone();
            execute_swap(phone, from_choice, to_choice, amount_str, pin, session, lang).await
        }
        _ => {
            session.clear_data();
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}

/// Show swap confirmation with spread details
async fn show_swap_confirmation(
    from_choice: &str,
    to_choice: &str,
    amount_str: &str,
    lang: Language,
) -> (String, bool) {
    // Validate amount
    let amount = match amount_str.parse::<u64>() {
        Ok(amt) => amt,
        Err(_) => {
            return (format!("❌ {}\n\n0. {}",
                TranslationService::translate("invalid_amount", lang),
                TranslationService::translate("main_menu", lang)), false);
        }
    };

    if amount == 0 {
        return (format!("❌ {}\n\n0. {}",
            TranslationService::translate("amount_must_be_greater_than_zero", lang),
            TranslationService::translate("main_menu", lang)), false);
    }

    // Fetch spread from crypto canister
    let spread_basis_points = match crypto_client::get_spread_basis_points().await {
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

    let from_name = match from_choice {
        "1" => "BTC",
        "2" => "USDC",
        _ => "?"
    };

    let to_name = match to_choice {
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

/// Execute the swap operation
async fn execute_swap(
    phone: String,
    from_choice: &str,
    to_choice: &str,
    amount_str: &str,
    pin: &str,
    session: &mut UssdSession,
    lang: Language,
) -> (String, bool) {
    // Parse amount
    let amount: u64 = match amount_str.parse() {
        Ok(amt) => amt,
        Err(_) => {
            session.clear_data();
            return (format!("❌ {}\n\n0. {}",
                TranslationService::translate("invalid_amount", lang),
                TranslationService::translate("main_menu", lang)), false);
        }
    };

    // Map choices to crypto types
    let (from_crypto, to_crypto) = match (from_choice, to_choice) {
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
        _ => {
            session.clear_data();
            return (format!("Invalid crypto type\n\n0. {}",
                TranslationService::translate("main_menu", lang)), false);
        }
    };

    let to_type = match to_crypto {
        "CkBTC" => shared_types::CryptoType::CkBTC,
        "CkUSDC" => shared_types::CryptoType::CkUSDC,
        _ => {
            session.clear_data();
            return (format!("Invalid crypto type\n\n0. {}",
                TranslationService::translate("main_menu", lang)), false);
        }
    };

    // Execute swap via crypto canister
    match crypto_client::swap_crypto(
        phone.clone(),
        from_type,
        to_type,
        amount,
        pin.to_string()
    ).await {
        Ok(result) => {
            session.clear_data();
            (format!("✅ {}!\n\n{}: {} {}\n{}: {} {}\n{}: {}\n{}: {}\n\n0. {}",
                TranslationService::translate("swap_successful", lang),
                TranslationService::translate("swap_from", lang), result.from_amount, from_crypto,
                TranslationService::translate("swap_to", lang), result.to_amount, to_crypto,
                TranslationService::translate("spread", lang), result.spread_amount,
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
