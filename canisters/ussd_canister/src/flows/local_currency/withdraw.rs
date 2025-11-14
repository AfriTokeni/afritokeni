// Withdraw flow with PIN verification and commission display
use crate::core::session::UssdSession;
use crate::logic::agent_logic::calculate_withdrawal_fees;
use crate::utils::constants::{MIN_WITHDRAWAL_AMOUNT_UGX, MAX_WITHDRAWAL_AMOUNT_UGX};
use crate::utils::translations::{Language, TranslationService};

/// Handle withdraw flow
/// Supports both interactive and shorthand inputs:
/// - Interactive: "1*4" → amount → agent → confirm → PIN
/// - Shorthand: "1*4*amount*agent*pin" (skips confirmation)
/// - Continued: When in session, just the current input (e.g., "50000")
pub async fn handle_withdraw(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // Check if this is a continued session or fresh start
    let is_fresh_start = parts.get(0) == Some(&"1") && parts.get(1) == Some(&"4");

    // Store session data in variables that live long enough
    let stored_amount = session.get_data("amount");
    let stored_agent = session.get_data("agent_id");

    // Extract params differently based on mode
    let (amount_opt, agent_id_opt, param_4, param_5) = if is_fresh_start {
        // Fresh start: "1*4*amount*agent*confirmation*pin"
        (
            parts.get(2).map(|s| s.trim()).filter(|s| !s.is_empty()),
            parts.get(3).map(|s| s.trim()).filter(|s| !s.is_empty()),
            parts.get(4).map(|s| s.trim()).filter(|s| !s.is_empty()),
            parts.get(5).map(|s| s.trim()).filter(|s| !s.is_empty()),
        )
    } else {
        // Continued session: user just sends the next value
        // Determine what the current input represents based on what's already stored
        if stored_amount.is_none() {
            // No amount yet, current input is amount
            (Some(text.trim()), None, None, None)
        } else if stored_agent.is_none() {
            // Have amount, need agent
            (stored_amount.as_ref().map(|s| s.as_str()), Some(text.trim()), None, None)
        } else {
            // Have amount and agent, current input is confirmation or PIN
            (stored_amount.as_ref().map(|s| s.as_str()), stored_agent.as_ref().map(|s| s.as_str()), Some(text.trim()), None)
        }
    };

    // Determine PIN location:
    // - If param_4 is "1" or "2", it's confirmation → PIN is in param_5
    // - Otherwise, param_4 is PIN (shorthand mode)
    let (confirmation, pin_opt) = match param_4 {
        Some("1") => (Some("1"), param_5), // Confirmed, PIN in param_5
        Some("2") => (Some("2"), None),     // Cancelled
        Some(p4) if p4.len() == 4 && p4.chars().all(|c| c.is_numeric()) => (None, Some(p4)), // Shorthand: param_4 is PIN
        Some(p4) => (None, Some(p4)),       // Treat as PIN attempt (will fail validation if wrong format)
        None => (None, None),
    };

    // SHORTHAND MODE: If we have amount + agent + PIN, validate and execute immediately
    ic_cdk::println!("🔍 [WITHDRAW] Checking shorthand: amount={:?}, agent={:?}, pin={:?}", amount_opt, agent_id_opt, pin_opt);
    if let (Some(amount_str), Some(agent_id), Some(pin)) = (amount_opt, agent_id_opt, pin_opt) {
        ic_cdk::println!("✅ [WITHDRAW] Shorthand mode - executing withdrawal");
        return execute_withdrawal(session, agent_id, amount_str, pin, lang).await;
    }
    ic_cdk::println!("➡️ [WITHDRAW] Not shorthand, entering interactive mode");

    // INTERACTIVE MODE: Guide user through missing steps

    // Step 0: Need amount
    if amount_opt.is_none() {
        return (format!("{}\n{}",
            TranslationService::translate("withdraw", lang),
            "Enter amount (UGX):"), true);
    }

    // Validate amount before proceeding
    let amount = match amount_opt.unwrap().parse::<u64>() {
        Ok(a) if a == 0 => {
            return (format!("❌ Invalid amount: must be a positive number greater than 0.\n\nTry again:"), true);
        }
        Ok(a) if a < MIN_WITHDRAWAL_AMOUNT_UGX => {
            return (format!("❌ Minimum withdrawal amount is {}.\n\nTry again:", MIN_WITHDRAWAL_AMOUNT_UGX), true);
        }
        Ok(a) if a > MAX_WITHDRAWAL_AMOUNT_UGX => {
            return (format!("❌ Maximum withdrawal amount is {}.\n\nTry again:", MAX_WITHDRAWAL_AMOUNT_UGX), true);
        }
        Ok(a) => a,
        Err(_) => {
            return (format!("❌ Invalid amount. Please enter a valid number.\n\nTry again:"), true);
        }
    };

    // Store validated amount
    session.set_data("amount", &amount.to_string());

    // Step 1: Have amount, need agent
    if agent_id_opt.is_none() {
        return (format!("Enter agent ID:"), true);
    }

    // Validate agent ID
    let agent_id = agent_id_opt.unwrap();
    if agent_id.is_empty() {
        return (format!("❌ Invalid agent ID. Agent ID cannot be empty.\n\nEnter agent ID:"), true);
    }

    // Store agent ID
    session.set_data("agent_id", agent_id);

    // Step 2: Have amount + agent, show fees and confirmation
    if confirmation.is_none() && pin_opt.is_none() {
        // Get withdrawal fees from Agent Canister
        match crate::services::agent_client::get_withdrawal_fees(amount).await {
            Ok(fees) => {
                (format!("💰 Withdrawal Details:\n\nAmount: {}\nPlatform fee (0.5%): {}\nAgent fee (10%): {}\nTotal fees: {}\nYou receive: {}\n\n1. Confirm\n2. Cancel",
                    fees.amount,
                    fees.platform_fee,
                    fees.agent_fee,
                    fees.total_fees,
                    fees.net_amount), true)
            }
            Err(e) => {
                (format!("❌ Error calculating fees: {}\n\nTry again", e), true)
            }
        }
    }
    // Step 3: Handle confirmation choice
    else if let Some(conf) = confirmation {
        if conf == "1" {
            // User confirmed, ask for PIN
            return (format!("Enter your PIN:"), true);
        } else {
            // User cancelled
            session.clear_data();
            return (format!("Withdrawal cancelled\n\n0. Main Menu"), false);
        }
    }
    // Step 4: Have amount + agent + confirmation + PIN, execute
    else if let (Some(_), Some(_), Some(pin)) = (amount_opt, agent_id_opt, pin_opt) {
        // Get values from session (stored in earlier steps)
        let amount_str = session.get_data("amount").unwrap_or_else(|| amount_opt.unwrap().to_string());
        let agent_id = session.get_data("agent_id").unwrap_or_else(|| agent_id_opt.unwrap().to_string());

        return execute_withdrawal(session, &agent_id, &amount_str, pin, lang).await;
    } else {
        // Shouldn't reach here, but handle gracefully
        session.clear_data();
        return (format!("Invalid input. Please try again.\n\n0. Main Menu"), false);
    }
}

/// Execute withdrawal request (helper function)
async fn execute_withdrawal(
    session: &mut UssdSession,
    agent_id: &str,
    amount_str: &str,
    pin: &str,
    lang: Language,
) -> (String, bool) {
    ic_cdk::println!("🔧 [WITHDRAW] execute_withdrawal called: agent='{}', amount='{}', pin len={}", agent_id, amount_str, pin.len());

    // Validate agent ID
    if agent_id.is_empty() {
        ic_cdk::println!("❌ [WITHDRAW] Empty agent ID");
        session.clear_data();
        return (format!("❌ Invalid agent ID.\n\n0. Main Menu"), false);
    }

    // Parse and validate amount
    let amount = match amount_str.parse::<u64>() {
        Ok(a) if a > 0 => {
            ic_cdk::println!("✅ [WITHDRAW] Amount parsed: {}", a);
            a
        }
        Ok(a) => {
            ic_cdk::println!("❌ [WITHDRAW] Amount is zero: {}", a);
            session.clear_data();
            return (format!("❌ Amount must be greater than 0.\n\n0. Main Menu"), false);
        }
        Err(e) => {
            ic_cdk::println!("❌ [WITHDRAW] Failed to parse amount '{}': {:?}", amount_str, e);
            session.clear_data();
            return (format!("❌ Invalid amount.\n\n0. Main Menu"), false);
        }
    };

    // Validate PIN format
    if pin.len() != 4 || !pin.chars().all(|c| c.is_numeric()) {
        session.clear_data();
        return (format!("❌ Invalid PIN format. PIN must be 4 digits.\n\n0. Main Menu"), false);
    }

    // Get user ID
    let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
        Ok(profile) => profile,
        Err(e) => {
            session.clear_data();
            return (format!("❌ Error: {}\n\n0. Main Menu", e), false);
        }
    };

    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
        Some(c) => c,
        None => {
            session.clear_data();
            return (format!("Invalid currency\n\n0. Main Menu"), false);
        }
    };

    // Create withdrawal request
    match crate::services::agent_client::create_withdrawal_request(
        user_profile.id.clone(),
        agent_id.to_string(),
        amount,
        currency_enum,
        pin.to_string()
    ).await {
        Ok(result) => {
            session.clear_data();
            // Calculate fees for display using shared logic
            let fees = calculate_withdrawal_fees(result.amount);
            let net_amount = result.amount - fees.platform_fee - fees.agent_fee;

            (format!("✅ Withdrawal Request Created!\n\n📋 CODE: {}\n\nShow this code to agent:\n{}\n\nAmount: {} {}\nPlatform fee: {} {}\nAgent fee: {} {}\nYou'll receive: {} {}\n\n0. Main Menu",
                result.withdrawal_code,
                result.withdrawal_code,
                result.amount,
                currency,
                fees.platform_fee,
                currency,
                fees.agent_fee,
                currency,
                net_amount,
                currency), false)
        }
        Err(e) => {
            session.clear_data();
            // Check for specific error messages
            let error_message = if e.contains("Invalid PIN") || e.contains("incorrect") || e.contains("PIN") {
                format!("❌ Invalid PIN. Please try again.\n\n0. Main Menu")
            } else if e.contains("Insufficient") || e.contains("balance") {
                format!("❌ Insufficient balance.\n\n0. Main Menu")
            } else {
                format!("❌ Error: {}\n\n0. Main Menu", e)
            };
            (error_message, false)
        }
    }
}
