// Deposit flow - Find agent and deposit cash
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle deposit flow
/// Supports both interactive and shorthand inputs:
/// - Interactive: "1*3" → agent → amount → confirm
/// - Shorthand: "1*3*AGENT001*50000*1" (auto-executes with confirmation)
pub async fn handle_deposit(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();

    // Extract params (after "1*3" prefix)
    // Flow order: 1*3 → AGENT_ID → AMOUNT → CONFIRMATION
    let agent_opt = parts.get(2).map(|s| s.trim()).filter(|s| !s.is_empty());
    let amount_opt = parts.get(3).map(|s| s.trim()).filter(|s| !s.is_empty());
    let confirm_opt = parts.get(4).map(|s| s.trim()).filter(|s| !s.is_empty());

    ic_cdk::println!("🔍 [DEPOSIT] Checking shorthand: agent={:?}, amount={:?}, confirm={:?}", agent_opt, amount_opt, confirm_opt);

    // SHORTHAND MODE: If we have all params and confirmation = "1", execute immediately
    if let (Some(agent), Some(amount_str), Some("1")) = (agent_opt, amount_opt, confirm_opt) {
        ic_cdk::println!("✅ [DEPOSIT] Shorthand mode - executing deposit");
        return execute_deposit(session, agent, amount_str, lang).await;
    }

    // Handle cancellation
    if confirm_opt == Some("2") {
        session.clear_data();
        return (format!("Deposit cancelled\n\n0. Main Menu"), false);
    }

    ic_cdk::println!("➡️ [DEPOSIT] Not shorthand, entering interactive mode");

    // INTERACTIVE MODE: Guide user through missing steps

    // Step 0: Need agent ID
    if agent_opt.is_none() {
        return (format!("{}\n{}",
            TranslationService::translate("deposit", lang),
            "Enter agent ID:"), true);
    }

    // Step 1: Have agent, need amount
    if amount_opt.is_none() {
        session.set_data("agent_id", agent_opt.unwrap());
        return (format!("Enter amount (UGX):"), true);
    }

    // Step 2: Have agent + amount, show fees and ask for confirmation
    if confirm_opt.is_none() {
        let amount_str = amount_opt.unwrap();

        match amount_str.parse::<u64>() {
            Ok(amount) => {
                if amount == 0 {
                    return (format!("❌ Amount must be greater than 0.\n\nTry again"), true);
                }

                // Calculate commission (0.5% platform fee)
                let commission = (amount * 50) / 10_000;
                let net_amount = amount.saturating_sub(commission);

                session.set_data("agent_id", agent_opt.unwrap());
                session.set_data("amount", &amount.to_string());

                (format!("💰 Deposit Details:\n\nAmount: {} UGX\nCommission (0.5%): {} UGX\nYou receive: {} UGX\n\n1. Confirm\n2. Cancel",
                    amount,
                    commission,
                    net_amount), true)
            }
            Err(_) => {
                (format!("Invalid amount. Please try again."), true)
            }
        }
    }
    // Step 3: Have all params + confirmation, execute deposit
    else if confirm_opt == Some("1") {
        // Get values from session (stored in step 2)
        let agent_id = session.get_data("agent_id").unwrap_or_else(|| agent_opt.unwrap().to_string());
        let amount_str = session.get_data("amount").unwrap_or_else(|| amount_opt.unwrap().to_string());

        return execute_deposit(session, &agent_id, &amount_str, lang).await;
    } else {
        // Invalid confirmation choice
        session.clear_data();
        return (format!("Invalid selection. Please try again.\n\n0. Main Menu"), false);
    }
}

/// Execute deposit request (helper function)
async fn execute_deposit(
    session: &mut UssdSession,
    agent_id: &str,
    amount_str: &str,
    _lang: Language,
) -> (String, bool) {
    ic_cdk::println!("🔧 [DEPOSIT] execute_deposit called: agent='{}', amount='{}'", agent_id, amount_str);

    // Validate agent ID
    if agent_id.is_empty() {
        ic_cdk::println!("❌ [DEPOSIT] Empty agent ID");
        session.clear_data();
        return (format!("❌ Invalid agent ID.\n\n0. Main Menu"), false);
    }

    // Parse and validate amount
    let amount = match amount_str.parse::<u64>() {
        Ok(a) if a > 0 => {
            ic_cdk::println!("✅ [DEPOSIT] Amount parsed: {}", a);
            a
        }
        Ok(a) => {
            ic_cdk::println!("❌ [DEPOSIT] Amount is zero: {}", a);
            session.clear_data();
            return (format!("❌ Amount must be greater than 0.\n\n0. Main Menu"), false);
        }
        Err(e) => {
            ic_cdk::println!("❌ [DEPOSIT] Failed to parse amount '{}': {:?}", amount_str, e);
            session.clear_data();
            return (format!("❌ Invalid amount.\n\n0. Main Menu"), false);
        }
    };

    // Get user identifier (phone number)
    let phone = session.phone_number.clone();

    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
        Some(c) => c,
        None => {
            session.clear_data();
            return (format!("Invalid currency\n\n0. Main Menu"), false);
        }
    };

    // Create deposit request
    match crate::services::agent_client::create_deposit_request(
        phone.clone(),
        agent_id.to_string(),
        amount,
        currency_enum
    ).await {
        Ok(result) => {
            session.clear_data();
            // Calculate commission for display (10% agent commission per the agent_canister tests)
            let agent_commission = (result.amount as f64 * 0.10).round() as u64;
            let net_to_user = result.amount - agent_commission;

            (format!("✅ Deposit Request Created!\n\n📋 CODE: {}\n\nShow this code to agent:\n{}\n\nAmount: {} {}\nAgent Commission: {} {}\nYou'll receive: {} {}\n\n0. Main Menu",
                result.deposit_code,
                result.deposit_code,
                result.amount,
                currency,
                agent_commission,
                currency,
                net_to_user,
                currency), false)
        }
        Err(e) => {
            session.clear_data();
            // Check for specific error messages
            let error_message = if e.contains("below minimum") {
                format!("❌ Amount below minimum. Please try again.\n\n0. Main Menu")
            } else if e.contains("exceeds maximum") {
                format!("❌ Amount exceeds maximum. Please try again.\n\n0. Main Menu")
            } else {
                format!("❌ Error: {}\n\n0. Main Menu", e)
            };
            (error_message, false)
        }
    }
}
