// Find Agent flow
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle find agent
pub async fn handle_find_agent(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    ic_cdk::println!("🔍 Finding agents for currency: {}", currency);
    
    // Get user's location from their profile, then find nearby agents
    // Business Logic Canister will use the user's registered location
    match crate::utils::business_logic_helper::get_nearby_agents(&session.phone_number, &currency).await {
        Ok(agents) => {
            if agents.is_empty() {
                return (format!("{}\n\n{}\n\n{}", 
                    TranslationService::translate("find_agent", lang),
                    TranslationService::translate("no_agents_found", lang),
                    TranslationService::translate("back_or_menu", lang)), true);
            }
            
            let mut response = format!("{} ({}):\n\n", 
                TranslationService::translate("nearby_agents", lang),
                agents.len());
            
            for (i, agent) in agents.iter().take(5).enumerate() {
                response.push_str(&format!("{}. {}\n{}: {}\n{}: {}\n\n", 
                    i + 1,
                    agent.name,
                    TranslationService::translate("phone", lang),
                    agent.phone,
                    TranslationService::translate("location", lang),
                    agent.location));
            }
            
            response.push_str(&TranslationService::translate("back_or_menu", lang));
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
