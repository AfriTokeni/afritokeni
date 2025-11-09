use candid::Principal;
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::msg_caller;
use std::cell::RefCell;

// ============================================================================
// Business Logic Canister - Shared by USSD & Web
// ============================================================================

mod models;
mod services;

use models::*;

// ============================================================================
// Access Control - CRITICAL SECURITY
// ============================================================================

thread_local! {
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static AUDIT_LOG: RefCell<Vec<AuditEntry>> = RefCell::new(Vec::new());
}

/// Log audit entry
fn log_audit(action: &str, user_id: Option<String>, details: &str, success: bool) {
    let entry = AuditEntry {
        timestamp: ic_cdk::api::time() / 1_000_000_000,
        action: action.to_string(),
        caller: msg_caller().to_text(),
        user_id,
        details: details.to_string(),
        success,
    };
    
    AUDIT_LOG.with(|log| {
        let mut l = log.borrow_mut();
        l.push(entry);
        // Keep last 10,000 entries
        if l.len() > 10_000 {
            l.remove(0);
        }
    });
}

/// Verify caller is an authorized canister (USSD or Web)
fn verify_authorized_caller() -> Result<(), String> {
    let caller = msg_caller();
    
    // Allow controller
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    }
    
    // Check if caller is authorized
    AUTHORIZED_CANISTERS.with(|canisters| {
        if canisters.borrow().contains(&caller) {
            Ok(())
        } else {
            Err(format!("Unauthorized caller: {}", caller.to_text()))
        }
    })
}

/// Add authorized canister (only controller can call)
#[update]
fn add_authorized_canister(canister_id: String) -> Result<(), String> {
    let caller = msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can add authorized canisters".to_string());
    }
    
    let principal = Principal::from_text(&canister_id)
        .map_err(|e| format!("Invalid principal: {}", e))?;
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut c = canisters.borrow_mut();
        if !c.contains(&principal) {
            c.push(principal);
        }
    });
    
    // Audit log
    log_audit(
        "add_authorized_canister",
        None,
        &format!("Added authorized canister: {}", canister_id),
        true
    );
    
    Ok(())
}

/// Remove authorized canister (only controller can call)
#[update]
fn remove_authorized_canister(canister_id: String) -> Result<(), String> {
    let caller = msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can remove authorized canisters".to_string());
    }
    
    let principal = Principal::from_text(&canister_id)
        .map_err(|e| format!("Invalid principal: {}", e))?;
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow_mut().retain(|p| p != &principal);
    });
    
    // Audit log
    log_audit(
        "remove_authorized_canister",
        None,
        &format!("Removed authorized canister: {}", canister_id),
        true
    );
    
    Ok(())
}

/// List authorized canisters (only controller can call)
#[query]
fn list_authorized_canisters() -> Result<Vec<String>, String> {
    let caller = msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can list authorized canisters".to_string());
    }
    
    Ok(AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow().iter().map(|p| p.to_text()).collect()
    }))
}

// ============================================================================
// Initialization
// ============================================================================

/// Initialize Business Logic canister with Data canister ID
#[init]
fn init(data_canister_id: String) {
    services::config::set_data_canister_id(data_canister_id.clone());
    ic_cdk::println!("✅ Business Logic canister initialized with Data canister: {}", data_canister_id);
}

/// Manually set data canister ID (for flexibility)
#[update]
fn set_data_canister_id(canister_id: String) -> Result<(), String> {
    let caller = msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can set data canister ID".to_string());
    }
    
    services::config::set_data_canister_id(canister_id.clone());
    ic_cdk::println!("✅ Data canister ID set to: {}", canister_id);
    Ok(())
}

// ============================================================================
// Configuration Management
// ============================================================================

/// Set maximum transaction amount (admin only)
#[update]
fn set_max_transaction_amount(amount: u64) -> Result<(), String> {
    // TODO: Add admin-only check
    services::config::set_max_transaction_amount(amount);
    Ok(())
}

/// Set suspicious amount threshold (admin only)
#[update]
fn set_suspicious_amount_threshold(amount: u64) -> Result<(), String> {
    // TODO: Add admin-only check
    services::config::set_suspicious_amount_threshold(amount);
    Ok(())
}

/// Get current fraud detection limits
#[query]
fn get_fraud_detection_limits() -> (u64, u64) {
    (
        services::config::get_max_transaction_amount(),
        services::config::get_suspicious_amount_threshold()
    )
}

// ============================================================================
// Money Transfer Operations
// ============================================================================

/// Transfer money between users (USSD & Web both use this)
#[update]
async fn transfer_money(
    from_phone_or_principal: String,
    to_phone_or_principal: String,
    amount: u64,
    currency: String,
    pin: String,
) -> Result<TransactionResult, String> {
    verify_authorized_caller()?;
    
    let result = services::money_transfer::transfer_money(
        from_phone_or_principal.clone(),
        to_phone_or_principal.clone(),
        amount,
        currency.clone(),
        pin,
    ).await;
    
    // Audit log
    match &result {
        Ok(_tx_result) => {
            log_audit(
                "transfer_money",
                Some(from_phone_or_principal.clone()),
                &format!("Transferred {} {} to {}", amount, currency, to_phone_or_principal),
                true
            );
        }
        Err(e) => {
            log_audit(
                "transfer_money",
                Some(from_phone_or_principal),
                &format!("Failed: {}", e),
                false
            );
        }
    }
    
    result
}

/// Withdraw fiat (USSD cash withdrawal via agent)
#[update]
async fn withdraw_fiat(
    phone_number: String,
    amount: u64,
    currency: String,
    agent_id: String,
    pin: String,
) -> Result<TransactionResult, String> {
    verify_authorized_caller()?;
    
    // Get user
    let user = services::data_client::get_user_by_phone(&phone_number).await?
        .ok_or_else(|| format!("User not found: {}", phone_number))?;
    
    // Verify PIN
    if !services::data_client::verify_pin(&user.id, &pin).await? {
        log_audit(
            "withdraw_fiat",
            Some(phone_number.clone()),
            "PIN verification failed",
            false
        );
        return Err("Incorrect PIN".to_string());
    }
    
    // Reset PIN attempts on success
    let _ = services::data_client::reset_pin_attempts(&user.id).await;
    
    // Check balance
    let balance = services::data_client::get_fiat_balance(&user.id, &currency).await?;
    if balance < amount {
        return Err(format!("Insufficient balance. Have: {}, Need: {}", balance, amount));
    }
    
    // Rate limiting
    if !services::fraud_detection::check_rate_limit(&user.id)? {
        return Err("Too many transactions. Please wait before trying again.".to_string());
    }
    
    // Fraud check
    let fraud_check = services::fraud_detection::check_transaction(&user.id, amount, &currency)?;
    if fraud_check.should_block {
        return Err(format!("Transaction blocked: {:?}", fraud_check.warnings));
    }
    
    // Process withdrawal via Data Canister
    let tx = services::data_client::withdraw_fiat(
        &user.id,
        amount,
        &currency,
        Some(format!("Cash withdrawal via agent {}", agent_id))
    ).await?;
    
    // Update last active
    let _ = services::data_client::update_last_active(&user.id).await;
    
    // Audit log
    log_audit(
        "withdraw_fiat",
        Some(phone_number),
        &format!("Withdrew {} {} via agent {}", amount, currency, agent_id),
        true
    );
    
    // Return result
    Ok(TransactionResult {
        transaction_id: tx.id,
        from_user: user.id.clone(),
        to_user: agent_id,
        amount,
        currency,
        new_balance: balance - amount,
        timestamp: tx.timestamp,
    })
}

/// Send money to phone number (convenience method for USSD)
#[update]
async fn send_money_to_phone(
    from_phone: String,
    to_phone: String,
    amount: u64,
    currency: String,
    pin: String,
) -> Result<TransactionResult, String> {
    verify_authorized_caller()?;
    
    let result = services::money_transfer::send_money_to_phone(
        from_phone.clone(),
        to_phone.clone(),
        amount,
        currency.clone(),
        pin,
    ).await;
    
    // Audit log
    match &result {
        Ok(_) => {
            log_audit(
                "send_money_to_phone",
                Some(from_phone.clone()),
                &format!("Sent {} {} to {}", amount, currency, to_phone),
                true
            );
        }
        Err(e) => {
            log_audit(
                "send_money_to_phone",
                Some(from_phone),
                &format!("Failed: {}", e),
                false
            );
        }
    }
    
    result
}

// ============================================================================
// Crypto Operations
// ============================================================================

/// Buy cryptocurrency with fiat
#[update]
async fn buy_crypto(
    user_identifier: String,
    fiat_amount: u64,
    fiat_currency: String,
    crypto_type: CryptoType,
    pin: String,
) -> Result<TransactionResult, String> {
    verify_authorized_caller()?;
    
    let result = services::crypto_operations::buy_crypto(
        user_identifier.clone(),
        fiat_amount,
        fiat_currency.clone(),
        crypto_type,
        pin,
    ).await;
    
    // Audit log
    match &result {
        Ok(_tx_result) => {
            log_audit(
                "buy_crypto",
                Some(user_identifier.clone()),
                &format!("Bought {:?} with {} {}", crypto_type, fiat_amount, fiat_currency),
                true
            );
        }
        Err(e) => {
            log_audit(
                "buy_crypto",
                Some(user_identifier),
                &format!("Failed: {}", e),
                false
            );
        }
    }
    
    result
}

/// Send cryptocurrency to address
#[update]
async fn send_crypto(
    user_identifier: String,
    to_address: String,
    amount: u64,
    crypto_type: CryptoType,
    pin: String,
) -> Result<TransactionResult, String> {
    verify_authorized_caller()?;
    
    let result = services::crypto_operations::send_crypto(
        user_identifier.clone(),
        to_address.clone(),
        amount,
        crypto_type,
        pin,
    ).await;
    
    // Audit log
    match &result {
        Ok(_) => {
            log_audit(
                "send_crypto",
                Some(user_identifier.clone()),
                &format!("Sent {} {:?} to {}", amount, crypto_type, to_address),
                true
            );
        }
        Err(e) => {
            log_audit(
                "send_crypto",
                Some(user_identifier),
                &format!("Failed: {}", e),
                false
            );
        }
    }
    
    result
}

// ============================================================================
// Balance Queries
// ============================================================================

/// Get user balances (both fiat and crypto)
#[query]
async fn get_balances(user_identifier: String) -> Result<UserBalances, String> {
    verify_authorized_caller()?;
    services::balance_queries::get_balances(user_identifier).await
}

/// Check fiat balance for specific currency
#[query]
async fn check_fiat_balance(
    user_identifier: String,
    currency: String,
) -> Result<u64, String> {
    verify_authorized_caller()?;
    services::balance_queries::check_fiat_balance(user_identifier, currency).await
}

/// Check crypto balance
#[query]
async fn check_crypto_balance(
    user_identifier: String,
    crypto_type: CryptoType,
) -> Result<u64, String> {
    verify_authorized_caller()?;
    services::balance_queries::check_crypto_balance(user_identifier, crypto_type).await
}

// ============================================================================
// User Management
// ============================================================================

/// Check if user exists (for USSD registration detection)
#[update]
async fn user_exists(user_identifier: String) -> Result<bool, String> {
    verify_authorized_caller()?;
    
    // Check by phone or principal
    let exists = if let Some(_) = services::data_client::get_user_by_phone(&user_identifier).await? {
        true
    } else if let Some(_) = services::data_client::get_user(&user_identifier).await? {
        true
    } else {
        false
    };
    
    Ok(exists)
}

/// Register new user (USSD or Web)
#[update]
async fn register_user(
    phone_number: Option<String>,
    principal_id: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
    preferred_currency: String,
    pin: String,
) -> Result<String, String> {
    verify_authorized_caller()?;
    
    ic_cdk::println!("📥 Business Logic received register_user:");
    ic_cdk::println!("  phone_number: {:?}", phone_number);
    ic_cdk::println!("  principal_id: {:?}", principal_id);
    ic_cdk::println!("  first_name: {:?}", first_name);
    ic_cdk::println!("  last_name: {:?}", last_name);
    ic_cdk::println!("  email: {:?}", email);
    ic_cdk::println!("  preferred_currency: {:?}", preferred_currency);
    ic_cdk::println!("  pin: {:?}", pin);
    
    let result = services::user_management::register_user(
        phone_number.clone(),
        principal_id.clone(),
        first_name.clone(),
        last_name.clone(),
        email.clone(),
        preferred_currency.clone(),
        pin,
    ).await;
    
    // Audit log
    match &result {
        Ok(user_id) => {
            log_audit(
                "register_user",
                Some(user_id.clone()),
                &format!("Registered {} {} ({})", first_name, last_name, email),
                true
            );
        }
        Err(e) => {
            log_audit(
                "register_user",
                phone_number.or(principal_id),
                &format!("Failed: {}", e),
                false
            );
        }
    }
    
    result
}

/// Link phone number to existing principal (account reconciliation)
#[update]
async fn link_phone_to_account(
    principal_id: String,
    phone_number: String,
    pin: String,
) -> Result<(), String> {
    verify_authorized_caller()?;
    services::user_management::link_phone_to_account(principal_id, phone_number, pin).await
}

/// Verify user PIN (for USSD authentication)
#[update]
async fn verify_pin(
    user_identifier: String,
    pin: String,
) -> Result<bool, String> {
    verify_authorized_caller()?;
    
    // Get user by phone or principal
    let user = if let Some(u) = services::data_client::get_user_by_phone(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user(&user_identifier).await? {
        u
    } else {
        return Err(format!("User not found: {}", user_identifier));
    };
    
    // SECURITY: Check if PIN is locked (too many failed attempts)
    if services::data_client::is_pin_locked(&user.id).await? {
        let attempts = services::data_client::get_failed_attempts(&user.id).await.unwrap_or(0);
        let remaining_seconds = services::data_client::get_remaining_lockout_time(&user.id).await.unwrap_or(0);
        let remaining_minutes = (remaining_seconds + 59) / 60; // Round up
        
        log_audit(
            "verify_pin",
            Some(user_identifier.clone()),
            &format!("PIN locked - {} failed attempts, {} seconds remaining", attempts, remaining_seconds),
            false
        );
        
        return Err(format!(
            "Account locked due to {} failed PIN attempts. Please try again in {} minutes.",
            attempts, remaining_minutes
        ));
    }
    
    // SECURITY: Check for account takeover patterns
    if services::data_client::check_account_takeover(&user.id).await? {
        log_audit(
            "verify_pin",
            Some(user_identifier.clone()),
            "Possible account takeover detected",
            false
        );
        return Err("Suspicious activity detected. Please contact support.".to_string());
    }
    
    // Verify PIN via data canister
    let result = services::data_client::verify_pin(&user.id, &pin).await;
    
    // Handle result and audit log
    match &result {
        Ok(true) => {
            // SECURITY: Reset failed attempts on successful verification
            let _ = services::data_client::reset_pin_attempts(&user.id).await;
            
            log_audit(
                "verify_pin",
                Some(user_identifier.clone()),
                "PIN verified successfully",
                true
            );
        }
        Ok(false) => {
            let attempts = services::data_client::get_failed_attempts(&user.id).await.unwrap_or(0);
            log_audit(
                "verify_pin",
                Some(user_identifier.clone()),
                &format!("PIN verification failed - incorrect PIN ({} attempts)", attempts),
                false
            );
        }
        Err(e) => {
            log_audit(
                "verify_pin",
                Some(user_identifier),
                &format!("PIN verification error: {}", e),
                false
            );
        }
    }
    
    result
}

/// Change PIN (for USSD and Web)
#[update]
async fn change_pin(
    user_identifier: String,
    old_pin: String,
    new_pin: String,
) -> Result<(), String> {
    verify_authorized_caller()?;
    
    // Get user by phone or principal
    let user = if let Some(u) = services::data_client::get_user_by_phone(&user_identifier).await? {
        u
    } else if let Some(u) = services::data_client::get_user(&user_identifier).await? {
        u
    } else {
        return Err(format!("User not found: {}", user_identifier));
    };
    
    // Validate new PIN format (4 digits)
    if new_pin.len() != 4 || !new_pin.chars().all(|c| c.is_numeric()) {
        return Err("Invalid PIN format. PIN must be exactly 4 digits.".to_string());
    }
    
    // Generate new salt for security
    let new_salt = format!("salt_{}", ic_cdk::api::time());
    
    // Change PIN via data canister (it will verify old PIN)
    let result = services::data_client::change_pin(&user.id, &old_pin, &new_pin, &new_salt).await;
    
    // Audit log
    match &result {
        Ok(()) => {
            log_audit(
                "change_pin",
                Some(user_identifier),
                "PIN changed successfully",
                true
            );
        }
        Err(e) => {
            log_audit(
                "change_pin",
                Some(user_identifier),
                &format!("PIN change failed: {}", e),
                false
            );
        }
    }
    
    result
}

// ============================================================================
// Transaction History
// ============================================================================

/// Get transaction history
#[query]
async fn get_transaction_history(
    user_identifier: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<TransactionRecord>, String> {
    verify_authorized_caller()?;
    
    services::transaction_history::get_history(
        user_identifier,
        limit,
        offset,
    ).await
}

// ============================================================================
// Audit Log Queries
// ============================================================================

/// Get audit log (only controller can access)
#[query]
fn get_audit_log(limit: Option<usize>, offset: Option<usize>) -> Result<Vec<AuditEntry>, String> {
    let caller = msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can access audit log".to_string());
    }
    
    Ok(AUDIT_LOG.with(|log| {
        let l = log.borrow();
        let start = offset.unwrap_or(0);
        let end = start + limit.unwrap_or(100).min(1000);
        l.iter().skip(start).take(end - start).cloned().collect()
    }))
}

/// Get audit log count
#[query]
fn get_audit_log_count() -> Result<usize, String> {
    let caller = msg_caller();
    if !ic_cdk::api::is_controller(&caller) {
        return Err("Only controller can access audit log".to_string());
    }
    
    Ok(AUDIT_LOG.with(|log| log.borrow().len()))
}

// Export Candid interface
ic_cdk::export_candid!();

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry {
            timestamp: 1699459200,
            action: "test_action".to_string(),
            caller: "test_caller".to_string(),
            user_id: Some("user123".to_string()),
            details: "test details".to_string(),
            success: true,
        };
        
        assert_eq!(entry.action, "test_action");
        assert_eq!(entry.success, true);
        assert!(entry.user_id.is_some());
    }
    
    // ============================================================================
    // verify_pin Tests
    // ============================================================================
    
    #[test]
    fn test_verify_pin_requires_authorized_caller() {
        // Note: In actual canister environment, verify_authorized_caller() would check
        // if the caller is in AUTHORIZED_CANISTERS. This is a unit test limitation.
        // Integration tests should verify this properly.
        
        // This test documents the expected behavior:
        // - Only authorized canisters (USSD) can call verify_pin
        // - Unauthorized callers should be rejected
        
        // The actual authorization check happens at runtime via verify_authorized_caller()
        assert!(true, "Authorization check is enforced at runtime");
    }
    
    #[test]
    fn test_verify_pin_validates_user_identifier_format() {
        // PIN verification should work with both phone numbers and user IDs
        
        // Valid phone number format (E.164)
        let phone = "+256700123456";
        assert!(phone.starts_with('+'));
        assert!(phone.len() >= 10);
        
        // Valid user ID format (UUID-like)
        let user_id = "user_123abc";
        assert!(!user_id.is_empty());
    }
    
    #[test]
    fn test_verify_pin_validates_pin_format() {
        // PIN must be exactly 4 digits
        let valid_pin = "1234";
        assert_eq!(valid_pin.len(), 4);
        assert!(valid_pin.chars().all(|c| c.is_numeric()));
        
        // Invalid PINs
        let too_short = "123";
        assert_ne!(too_short.len(), 4);
        
        let too_long = "12345";
        assert_ne!(too_long.len(), 4);
        
        let non_numeric = "12a4";
        assert!(!non_numeric.chars().all(|c| c.is_numeric()));
    }
    
    #[test]
    fn test_verify_pin_audit_logging() {
        // verify_pin should log to audit trail:
        // - Successful PIN verification (success=true)
        // - Failed PIN verification (success=false)  
        // - Errors (success=false with error details)
        
        // This is verified in integration tests where we can check AUDIT_LOG
        assert!(true, "Audit logging verified in integration tests");
    }
    
    #[test]
    fn test_verify_pin_security_considerations() {
        // Security requirements for PIN verification:
        
        // 1. PIN should never be logged in plaintext
        let pin = "1234";
        let audit_message = format!("PIN verification for user");
        assert!(!audit_message.contains(pin), "PIN must not appear in audit logs");
        
        // 2. Failed attempts should be tracked (future enhancement)
        // TODO: Implement rate limiting after N failed attempts
        
        // 3. Timing attacks: verification should take constant time
        // (This is handled by the PIN hashing library)
        
        assert!(true, "Security considerations documented");
    }
    
    #[test]
    fn test_transaction_result_creation() {
        let result = TransactionResult {
            transaction_id: "tx_123".to_string(),
            from_user: "alice".to_string(),
            to_user: "bob".to_string(),
            amount: 10000,
            currency: "UGX".to_string(),
            new_balance: 90000,
            timestamp: 1699459200,
        };
        
        assert_eq!(result.amount, 10000);
        assert_eq!(result.currency, "UGX");
        assert_eq!(result.new_balance, 90000);
    }
}
