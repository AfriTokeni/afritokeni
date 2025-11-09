// DAO Proposals viewing flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle view proposals
pub async fn handle_view_proposals(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    
    ic_cdk::println!("🗳️ Fetching DAO proposals");
    
    // Get active proposals from Business Logic Canister
    match crate::services::business_logic::get_dao_proposals().await {
        Ok(proposals) => {
            if proposals.is_empty() {
                return (format!("{}\n\n{}\n\n{}", 
                    TranslationService::translate("view_proposals", lang),
                    TranslationService::translate("no_active_proposals", lang),
                    TranslationService::translate("back_or_menu", lang)), true);
            }
            
            let mut response = format!("{} ({}):\n\n", 
                TranslationService::translate("active_proposals", lang),
                proposals.len());
            
            for (i, proposal) in proposals.iter().take(5).enumerate() {
                response.push_str(&format!("{}. {}\n{}: {}\n{}: {} / {}\n\n", 
                    i + 1,
                    proposal.title,
                    TranslationService::translate("status", lang),
                    proposal.status,
                    TranslationService::translate("votes", lang),
                    proposal.yes_votes,
                    proposal.total_votes));
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
