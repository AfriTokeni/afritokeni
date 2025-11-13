// Transactions history flow
use crate::core::session::UssdSession;
use crate::utils::translations::{Language, TranslationService};

/// Handle transactions history
pub async fn handle_transactions(_text: &str, session: &mut UssdSession) -> (String, bool) {
    let lang = Language::from_code(&session.language);

    ic_cdk::println!("📜 Fetching transaction history for: {}", session.phone_number);

    // Get user ID first
    let user_profile = match crate::services::user_client::get_user_by_phone(session.phone_number.clone()).await {
        Ok(profile) => profile,
        Err(e) => {
            return (format!("{}: {}\n\n{}",
                TranslationService::translate("error", lang),
                e,
                TranslationService::translate("back_or_menu", lang)), true);
        }
    };

    // Get last 5 transactions from wallet canister
    match crate::services::wallet_client::get_transaction_history(user_profile.id, Some(5)).await {
        Ok(transactions) => {
            if transactions.is_empty() {
                return (format!("{}\n\n{}\n\n{}",
                    TranslationService::translate("transactions", lang),
                    TranslationService::translate("no_transactions", lang),
                    TranslationService::translate("back_or_menu", lang)), true);
            }

            let mut response = format!("{}\n\n", TranslationService::translate("transactions", lang));

            for (i, tx) in transactions.iter().enumerate().take(5) {
                let amount = tx.amount as f64 / 100.0;
                let tx_type = format!("{:?}", tx.transaction_type);
                let currency = format!("{:?}", tx.currency_type);
                let status = format!("{:?}", tx.status);
                response.push_str(&format!("{}. {} {:.2} {} - {}\n",
                    i + 1,
                    tx_type,
                    amount,
                    currency,
                    if status.contains("Completed") { "✓" } else { "..." }
                ));
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
