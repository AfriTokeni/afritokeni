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
            match crate::services::business_logic::get_dao_proposals().await {
                Ok(proposals) => {
                    if proposals.is_empty() {
                        return (format!("{}\n\n{}\n\n{}", 
                            TranslationService::translate("vote_on_proposals", lang),
                            TranslationService::translate("no_active_proposals", lang),
                            TranslationService::translate("back_or_menu", lang)), true);
                    }
                    
                    let mut response = format!("{}\n{}\n\n", 
                        TranslationService::translate("vote_on_proposals", lang),
                        TranslationService::translate("select_proposal", lang));
                    
                    for (i, proposal) in proposals.iter().take(5).enumerate() {
                        response.push_str(&format!("{}. {}\n", i + 1, proposal.title));
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
        1 => {
            // Step 1: Show selected proposal and ask for vote
            let proposal_idx = parts.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            
            match crate::services::business_logic::get_dao_proposals().await {
                Ok(proposals) => {
                    if proposal_idx == 0 || proposal_idx > proposals.len() {
                        return (format!("{}\n\n{}", 
                            TranslationService::translate("invalid_selection", lang),
                            TranslationService::translate("back_or_menu", lang)), true);
                    }
                    
                    let proposal = &proposals[proposal_idx - 1];
                    (format!("{}\n\n{}\n1. {}\n2. {}\n\n{}", 
                        proposal.title,
                        TranslationService::translate("cast_your_vote", lang),
                        TranslationService::translate("yes", lang),
                        TranslationService::translate("no", lang),
                        TranslationService::translate("back_or_menu", lang)), true)
                }
                Err(e) => {
                    (format!("{}: {}\n\n{}", 
                        TranslationService::translate("error", lang),
                        e,
                        TranslationService::translate("back_or_menu", lang)), true)
                }
            }
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
            
            match crate::services::business_logic::cast_dao_vote(
                &session.phone_number,
                proposal_idx as u64,
                vote_yes,
                pin
            ).await {
                Ok(_) => {
                    (format!("{}!\n\n{}", 
                        TranslationService::translate("vote_recorded", lang),
                        TranslationService::translate("thank_you", lang)), false)
                }
                Err(e) => {
                    (format!("{}: {}\n\n{}", 
                        TranslationService::translate("vote_failed", lang),
                        e,
                        TranslationService::translate("thank_you", lang)), false)
                }
            }
        }
        _ => {
            (format!("{}\n\n{}", 
                TranslationService::translate("invalid_selection", lang),
                TranslationService::translate("back_or_menu", lang)), true)
        }
    }
}
