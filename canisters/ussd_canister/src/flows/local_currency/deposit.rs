// Deposit flow - Find agent and deposit cash
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle deposit flow
/// Steps: 0. Show agents → 1. Select agent → 2. Enter amount → 3. Confirm
pub async fn handle_deposit(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    // For now, show message that user needs to find an agent
    (format!("{}\n\n{}\n\n{}\n\n{}", 
        TranslationService::translate("deposit", lang),
        TranslationService::translate("find_agent_to_deposit", lang),
        format!("1. {} ({})", TranslationService::translate("find_agent", lang), currency),
        TranslationService::translate("back_or_menu", lang)), true)
}
