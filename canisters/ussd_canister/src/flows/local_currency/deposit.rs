// Deposit flow - Find agent and deposit cash
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle deposit flow
/// Steps: 0. Enter agent ID → 1. Enter amount → 2. Show commission & confirm → 3. Create request
pub async fn handle_deposit(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    // Determine step based on text input
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Ask for agent ID
            (format!("{}\n{}", 
                TranslationService::translate("deposit", lang),
                "Enter agent ID:"), true)
        }
        1 => {
            // Step 1: Store agent ID, ask for amount
            let agent_id = parts.get(2).unwrap_or(&"");
            session.set_data("agent_id", agent_id);
            
            (format!("Enter amount (UGX):"), true)
        }
        2 => {
            // Step 2: Calculate commission and show confirmation
            let amount_str = parts.get(3).unwrap_or(&"");
            
            match amount_str.parse::<u64>() {
                Ok(amount) => {
                    // Calculate commission (0.5%)
                    let commission = (amount * 50) / 10_000;
                    let net_amount = amount.saturating_sub(commission);
                    
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
        3 => {
            // Step 3: Confirm and create deposit request
            let choice = parts.get(4).unwrap_or(&"");
            
            if choice == &"1" {
                // User confirmed - create deposit request
                let agent_id = session.get_data("agent_id").unwrap_or_default();
                let amount: u64 = session.get_data("amount").unwrap_or_default().parse().unwrap_or(0);
                let phone = session.phone_number.clone();
                
                let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
                let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                    Some(c) => c,
                    None => return (format!("Invalid currency\n\n0. Main Menu"), false),
                };
                
                match crate::services::agent_client::create_deposit_request(
                    phone.clone(),
                    agent_id.clone(),
                    amount,
                    currency_enum
                ).await {
                    Ok(result) => {
                        session.clear_data();
                        (format!("✅ Deposit Request Created!\n\n📋 CODE: {}\n\nShow this code to agent:\n{}\n\nAmount: {} {}\nCommission: {} {}\nYou'll receive: {} {}\n\n0. Main Menu",
                            result.deposit_code,
                            result.deposit_code,
                            result.amount,
                            currency,
                            result.commission,
                            currency,
                            result.net_amount,
                            currency), false)
                    }
                    Err(e) => {
                        session.clear_data();
                        (format!("❌ Error: {}\n\n0. Main Menu", e), false)
                    }
                }
            } else {
                // User cancelled
                session.clear_data();
                (format!("Deposit cancelled\n\n0. Main Menu"), false)
            }
        }
        _ => {
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
