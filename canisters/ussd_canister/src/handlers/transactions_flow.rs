// Transactions history flow
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle transactions history
pub async fn handle_transactions(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    ic_cdk::println!("📜 Fetching transaction history for: {}", session.phone_number);
    
    // Get transaction history from Business Logic Canister
    match crate::utils::business_logic_helper::get_transaction_history(&session.phone_number, 5).await {
        Ok(transactions) => {
            if transactions.is_empty() {
                return (format!("{}\n\n{}\n\n{}", 
                    TranslationService::translate("transactions", lang),
                    TranslationService::translate("no_transactions", lang),
                    TranslationService::translate("back_or_menu", lang)), true);
            }
            
            let mut response = format!("{} ({}):\n\n", 
                TranslationService::translate("last_transactions", lang),
                transactions.len());
            
            for (i, tx) in transactions.iter().enumerate() {
                let amount = tx.amount as f64 / 100.0;
                response.push_str(&format!("{}. {} {:.2} - {}\n", 
                    i + 1,
                    tx.tx_type,
                    amount,
                    tx.timestamp));
            }
            
            response.push_str(&format!("\n{}", TranslationService::translate("back_or_menu", lang)));
            (response, true)
        }
        Err(e) => {
            (format!("{}: {}\n\n{}", 
                TranslationService::translate("error", lang),
                e,
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
