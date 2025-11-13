// Find Agent flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle find agent
pub async fn handle_find_agent(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    ic_cdk::println!("🔍 Finding agents for currency: {}", currency);
    
    // TODO: Use agent_client::get_nearby_agents
    let _ = currency; // Suppress unused warning
    (format!("{}\n\n{}\n\n{}", 
        TranslationService::translate("find_agent", lang),
        "Agent search coming soon",
        TranslationService::translate("back_or_menu", lang)), true)
}
