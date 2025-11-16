// USDC rate check flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle USDC rate check
/// Note: Rates are calculated dynamically during transactions.
/// This provides indicative information based on typical spreads.
pub async fn handle_usdc_rate(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);
    let currency = session.get_data("currency").unwrap_or_else(|| "UGX".to_string());

    ic_cdk::println!("💱 USDC rate info for: {}", currency);

    // Rates are calculated at transaction time by crypto_canister
    // Show informational message about how rates work
    let response = format!(
        "{}\n\n{}\n\n{}\n\n{}",
        TranslationService::translate("usdc_rate", lang),
        TranslationService::translate("live_rates_shown", lang),
        TranslationService::translate("rates_update_realtime", lang),
        TranslationService::translate("back_or_menu", lang)
    );

    (response, true)
}
