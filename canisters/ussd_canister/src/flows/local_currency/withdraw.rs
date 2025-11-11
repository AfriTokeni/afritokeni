// Withdraw flow with PIN verification and commission display
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};
use crate::services::business_logic;

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
                    // Get withdrawal fees from Business Logic
                    match business_logic::get_withdrawal_fees(amount).await {
                        Ok(fees) => {
                            session.set_data("amount", &amount.to_string());
                            
                            (format!("💰 Withdrawal Details:\n\nAmount: {} UGX\nPlatform fee (0.5%): {} UGX\nAgent fee (10%): {} UGX\nTotal fees: {} UGX\nYou receive: {} UGX\n\n1. Confirm\n2. Cancel",
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
            let phone = session.phone_number.clone();
            
            match business_logic::create_withdrawal_request(&phone, &agent_id, amount, pin).await {
                Ok(result) => {
                    session.clear_data();
                    (format!("✅ Withdrawal Request Created!\n\n📋 CODE: {}\n\nShow this code to agent:\n{}\n\nAmount: {} UGX\nPlatform fee: {} UGX\nAgent fee: {} UGX\nYou'll receive: {} UGX\n\n0. Main Menu",
                        result.withdrawal_code,
                        result.withdrawal_code,
                        result.amount_ugx,
                        result.platform_fee_ugx,
                        result.agent_fee_ugx,
                        result.net_amount), false)
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
