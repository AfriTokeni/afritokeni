use ussd_canister::core::session::UssdSession;
use ussd_canister::services::{wallet_client, user_client};
use shared_types::FiatCurrency;

// Helper to setup test environment
fn setup() {
    wallet_client::clear_mocks();
    user_client::clear_mocks();
}

#[tokio::test]
async fn test_send_money_step_0_ask_for_recipient() {
    setup();
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1"; // Main menu -> Send money
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(continues, "Should continue session");
    assert!(response.contains("phone") || response.contains("recipient"), "Should ask for phone number");
}

#[tokio::test]
async fn test_send_money_step_1_invalid_phone() {
    setup();
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*invalid"; // Invalid phone
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(continues, "Should continue session for retry");
    assert!(response.contains("Invalid") || response.contains("invalid"), "Should show error");
}

#[tokio::test]
async fn test_send_money_step_1_valid_phone() {
    setup();
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*+256700000002"; // Valid phone
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(continues, "Should continue session");
    assert!(response.contains("amount") || response.contains("Amount"), "Should ask for amount");
}

#[tokio::test]
async fn test_send_money_step_2_invalid_amount() {
    setup();
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*+256700000002*abc"; // Invalid amount
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(continues, "Should continue session for retry");
    assert!(response.contains("Invalid") || response.contains("invalid"), "Should show error");
}

#[tokio::test]
async fn test_send_money_step_2_insufficient_balance() {
    setup();
    
    // Mock: User has 5000 UGX, tries to send 10000 UGX
    wallet_client::set_mock_get_fiat_balance(|_user_id, _currency| {
        Ok(500_000) // 5000 UGX in cents
    });
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*+256700000002*10000"; // Try to send 10000 UGX
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(!continues, "Should end session");
    assert!(response.contains("Insufficient") || response.contains("insufficient"), "Should show insufficient balance error");
}

#[tokio::test]
async fn test_send_money_step_2_sufficient_balance() {
    setup();
    
    // Mock: User has 20000 UGX, tries to send 10000 UGX
    wallet_client::set_mock_get_fiat_balance(|_user_id, _currency| {
        Ok(2_000_000) // 20000 UGX in cents
    });
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*+256700000002*10000"; // Send 10000 UGX
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(continues, "Should continue to PIN step");
    assert!(response.contains("PIN") || response.contains("pin"), "Should ask for PIN");
    assert!(response.contains("10000") || response.contains("10,000"), "Should show amount");
}

#[tokio::test]
async fn test_send_money_step_3_successful_transfer() {
    setup();
    
    // Mock user lookup
    user_client::set_mock_get_user_by_phone(|phone| {
        Ok(user_client::UserProfile {
            id: if phone == "+256700000001" { "user1".to_string() } else { "user2".to_string() },
            phone_number: Some(phone),
            principal_id: None,
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: "test@test.com".to_string(),
            preferred_currency: "UGX".to_string(),
            kyc_status: "NotStarted".to_string(),
            created_at: 0,
            last_active: 0,
        })
    });
    
    // Mock successful transfer
    wallet_client::set_mock_transfer_fiat(|from, to, amount, currency, _pin| {
        assert_eq!(from, "user1", "Should transfer from correct user");
        assert_eq!(to, "user2", "Should transfer to correct user");
        assert_eq!(amount, 1_000_000, "Should transfer 10000 UGX (in cents)");
        assert_eq!(currency, FiatCurrency::UGX, "Should use correct currency");
        
        Ok(wallet_client::TransferResponse {
            transaction_id: "tx123".to_string(),
            from_user_id: "user1".to_string(),
            to_user_id: "user2".to_string(),
            amount: 1_000_000,
            fee: 5_000,
            currency: "UGX".to_string(),
            sender_new_balance: 1_000_000,
            recipient_new_balance: 1_000_000,
            timestamp: 0,
        })
    });
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*+256700000002*10000*1234"; // Send with PIN
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(!continues, "Should end session");
    assert!(response.contains("successful") || response.contains("Successful"), "Should show success");
    assert!(response.contains("10000") || response.contains("10,000"), "Should show amount sent");
}

#[tokio::test]
async fn test_send_money_step_3_wrong_pin() {
    setup();
    
    // Mock user lookup
    user_client::set_mock_get_user_by_phone(|phone| {
        Ok(user_client::UserProfile {
            id: if phone == "+256700000001" { "user1".to_string() } else { "user2".to_string() },
            phone_number: Some(phone),
            principal_id: None,
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: "test@test.com".to_string(),
            preferred_currency: "UGX".to_string(),
            kyc_status: "NotStarted".to_string(),
            created_at: 0,
            last_active: 0,
        })
    });
    
    // Mock failed transfer (wrong PIN)
    wallet_client::set_mock_transfer_fiat(|_from, _to, _amount, _currency, _pin| {
        Err("Incorrect PIN".to_string())
    });
    
    let mut session = UssdSession::new("test_session".to_string(), "+256700000001".to_string());
    session.set_data("currency", "UGX");
    
    let text = "1*1*+256700000002*10000*9999"; // Wrong PIN
    let (response, continues) = ussd_canister::flows::local_currency::send_money::handle_send_money(text, &mut session).await;
    
    assert!(!continues, "Should end session");
    assert!(response.contains("failed") || response.contains("Failed") || response.contains("Incorrect"), "Should show error");
}
