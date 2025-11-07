// Minimal USSD handlers - to be expanded
use crate::models::session::UssdSession;

/// Handle main menu
pub async fn handle_main_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    session.current_menu = "main".to_string();
    
    let menu = "Welcome to AfriTokeni\n\
                1. Local Currency (KES)\n\
                2. Bitcoin (ckBTC)\n\
                3. USDC (ckUSDC)\n\
                4. DAO Governance\n\
                5. Language\n\
                0. Exit";
    
    (menu.to_string(), true)
}

/// Handle registration
pub async fn handle_registration(session: &mut UssdSession, pin: &str) -> (String, bool) {
    if pin.len() < 4 || pin.len() > 6 {
        return ("Invalid PIN. Must be 4-6 digits.\n0. Back".to_string(), true);
    }
    
    // TODO: Hash and store PIN
    session.current_menu = "main".to_string();
    ("PIN set successfully!\n0. Main Menu".to_string(), true)
}

/// Handle local currency menu
pub async fn handle_local_currency_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    session.current_menu = "local_currency".to_string();
    
    let menu = "Local Currency (KES)\n\
                1. Check Balance\n\
                2. Send Money\n\
                3. Deposit\n\
                4. Withdraw\n\
                0. Back";
    
    (menu.to_string(), true)
}

/// Handle Bitcoin menu
pub async fn handle_bitcoin_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    session.current_menu = "bitcoin".to_string();
    
    let menu = "Bitcoin (ckBTC)\n\
                1. Check Balance\n\
                2. Buy Bitcoin\n\
                3. Sell Bitcoin\n\
                4. Send Bitcoin\n\
                0. Back";
    
    (menu.to_string(), true)
}

/// Handle USDC menu
pub async fn handle_usdc_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    session.current_menu = "usdc".to_string();
    
    let menu = "USDC (ckUSDC)\n\
                1. Check Balance\n\
                2. Send USDC\n\
                3. Swap to KES\n\
                0. Back";
    
    (menu.to_string(), true)
}

/// Handle DAO menu
pub async fn handle_dao_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    session.current_menu = "dao".to_string();
    
    let menu = "DAO Governance\n\
                1. View Proposals\n\
                2. Vote\n\
                3. Create Proposal\n\
                0. Back";
    
    (menu.to_string(), true)
}

/// Handle language menu
pub async fn handle_language_menu(_text: &str, session: &mut UssdSession) -> (String, bool) {
    session.current_menu = "language".to_string();
    
    let menu = "Select Language\n\
                1. English\n\
                2. Swahili\n\
                3. Luganda\n\
                0. Back";
    
    (menu.to_string(), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_menu() {
        let mut session = UssdSession {
            session_id: "test".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: "".to_string(),
            step: 0,
            language: "en".to_string(),
            last_activity: 0,
        };
        
        let (response, cont) = handle_main_menu("", &mut session).await;
        assert!(cont);
        assert!(response.contains("Welcome"));
        assert!(response.contains("Bitcoin"));
    }

    #[tokio::test]
    async fn test_registration_invalid_pin() {
        let mut session = UssdSession {
            session_id: "test".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: "register".to_string(),
            step: 0,
            language: "en".to_string(),
            last_activity: 0,
        };
        
        let (response, cont) = handle_registration(&mut session, "123").await;
        assert!(cont);
        assert!(response.contains("Invalid PIN"));
    }
}
