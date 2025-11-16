// DAO Voting flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle DAO voting
/// Steps: 0. Show proposals → 1. Select proposal → 2. Vote Yes/No → 3. Enter PIN → 4. Execute
pub async fn handle_vote(text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let parts: Vec<&str> = text.split('*').collect();
    
    let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
    
    ic_cdk::println!("🗳️ DAO Vote flow: step={}, parts={:?}", step, parts);
    
    match step {
        0 => {
            // Step 0: Show active proposals
            // TODO: Implement DAO canister integration
            (format!("{}\n\n{}\n\n{}", 
                TranslationService::translate("vote_on_proposals", lang),
                "DAO functionality coming soon",
                TranslationService::translate("back_or_menu", lang)), true)
        }
        1 => {
            // Step 1: Show selected proposal and ask for vote
            let proposal_idx = parts.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            
            // TODO: Implement DAO canister integration
            let _ = proposal_idx; // Suppress unused warning
            (format!("{}\n\n{}", 
                "DAO functionality coming soon",
                TranslationService::translate("back_or_menu", lang)), true)
        }
        2 => {
            // Step 2: Ask for PIN to confirm vote
            let vote_choice = parts.get(3).unwrap_or(&"");
            let vote_text = if *vote_choice == "1" {
                TranslationService::translate("yes", lang)
            } else {
                TranslationService::translate("no", lang)
            };
            
            (format!("{}: {}\n\n{}", 
                TranslationService::translate("your_vote", lang),
                vote_text,
                TranslationService::translate("enter_pin_4digit", lang)), true)
        }
        3 => {
            // Step 3: Execute vote
            let pin = parts.get(4).unwrap_or(&"");
            let proposal_idx = parts.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            let vote_choice = parts.get(3).unwrap_or(&"");
            let vote_yes = *vote_choice == "1";
            
            ic_cdk::println!("🗳️ Executing vote: proposal={}, vote={}", proposal_idx, vote_yes);
            
            // TODO: Implement DAO canister integration
            let _ = (pin, proposal_idx, vote_yes); // Suppress unused warnings
            (format!("{}!\n\n{}", 
                TranslationService::translate("vote_recorded", lang),
                TranslationService::translate("thank_you", lang)), false)
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
