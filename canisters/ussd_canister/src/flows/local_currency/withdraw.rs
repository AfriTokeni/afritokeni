// Withdraw flow with PIN verification and commission display
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle withdraw flow
/// Steps: 0. Enter agent ID → 1. Enter amount → 2. Show fees & confirm → 3. Enter PIN → 4. Create request
pub async fn handle_withdraw(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    match step {
        0 => {
            // Step 0: Ask for agent ID
            (format!("{}\n{}", 
                TranslationService::translate("withdraw", lang),
                "Enter agent ID:"), true)
        }
        1 => {
            // Step 1: Store agent ID, ask for amount
            let agent_id = parts.get(2).unwrap_or(&"");
            session.set_data("agent_id", agent_id);
            
            (format!("Enter amount (UGX):"), true)
        }
        2 => {
            // Step 2: Get fees and show confirmation
            let amount_str = parts.get(3).unwrap_or(&"");
            
            match amount_str.parse::<u64>() {
                Ok(amount) => {
                    // Get withdrawal fees from Agent Canister
                    match crate::services::agent_client::get_withdrawal_fees(amount).await {
                        Ok(fees) => {
                            session.set_data("amount", &amount.to_string());
                            
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
                Err(_) => {
                    (format!("Invalid amount. Please try again."), true)
                }
            }
        }
        3 => {
            // Step 3: User confirmed, ask for PIN
            let choice = parts.get(4).unwrap_or(&"");
            
            if choice == &"1" {
                (format!("Enter your PIN:"), true)
            } else {
                session.clear_data();
                (format!("Withdrawal cancelled\n\n0. Main Menu"), false)
            }
        }
        4 => {
            // Step 4: Create withdrawal request with PIN
            let pin = parts.get(5).unwrap_or(&"");
            let agent_id = session.get_data("agent_id").unwrap_or_default();
            let amount: u64 = session.get_data("amount").unwrap_or_default().parse().unwrap_or(0);
            
            // Get user ID first
            let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
                Ok(profile) => profile,
                Err(e) => return (format!("❌ Error: {}\n\n0. Main Menu", e), false),
            };
            
            let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
            let currency_enum = match shared_types::FiatCurrency::from_code(&currency) {
                Some(c) => c,
                None => return (format!("Invalid currency\n\n0. Main Menu"), false),
            };
            
            match crate::services::agent_client::create_withdrawal_request(
                user_profile.id.clone(),
                agent_id.clone(),
                amount,
                currency_enum,
                pin.to_string()
            ).await {
                Ok(result) => {
                    session.clear_data();
                    // Calculate fees for display
                    let platform_fee = (result.amount as f64 * 0.005).round() as u64;
                    let agent_fee = (result.amount as f64 * 0.10).round() as u64;
                    let net_amount = result.amount - platform_fee - agent_fee;
                    
                    (format!("✅ Withdrawal Request Created!\n\n📋 CODE: {}\n\nShow this code to agent:\n{}\n\nAmount: {} {}\nPlatform fee: {} {}\nAgent fee: {} {}\nYou'll receive: {} {}\n\n0. Main Menu",
                        result.withdrawal_code,
                        result.withdrawal_code,
                        result.amount,
                        currency,
                        platform_fee,
                        currency,
                        agent_fee,
                        currency,
                        net_amount,
                        currency), false)
                }
                Err(e) => {
                    session.clear_data();
                    (format!("❌ Error: {}\n\n0. Main Menu", e), false)
                }
            }
        }
        _ => {
            (TranslationService::translate("invalid_selection", lang).to_string(), false)
        }
    }
}
