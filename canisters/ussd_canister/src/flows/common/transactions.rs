// Transactions history flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle transactions history
pub async fn handle_transactions(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    ic_cdk::println!("📜 Fetching transaction history for: {}", session.phone_number);
    
    // TODO: Use wallet_client::get_transaction_history
    (format!("{}\n\n{}\n\n{}", 
        TranslationService::translate("transactions", lang),
        "Transaction history coming soon",
        TranslationService::translate("back_or_menu", lang)), true)
}
