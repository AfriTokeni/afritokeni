// USDC rate check flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle USDC rate check
pub async fn handle_usdc_rate(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    ic_cdk::println!("💱 Fetching USDC rate for: {}", currency);
    
    // TODO: Use exchange service for rates
    let _ = currency; // Suppress unused warning
    (format!("{}\n\n{}\n\n{}", 
        TranslationService::translate("usdc_rate", lang),
        "Rate service coming soon",
        TranslationService::translate("back_or_menu", lang)), true)
}
