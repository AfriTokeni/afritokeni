// USDC rate check flow
use crate::models::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle USDC rate check
pub async fn handle_usdc_rate(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());
    
    ic_cdk::println!("💱 Fetching USDC rate for: {}", currency);
    
    // Get USDC rate from Business Logic Canister
    match crate::utils::business_logic_helper::get_usdc_rate(&currency).await {
        Ok(rate) => {
            (format!("{}\n\n1 USDC = {} {}\n1 {} = {:.4} USDC\n\n{}", 
                TranslationService::translate("usdc_rate", lang),
                rate.rate_to_fiat,
                currency,
                currency,
                1.0 / rate.rate_to_fiat,
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
