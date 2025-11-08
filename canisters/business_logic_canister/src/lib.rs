use candid::Principal;
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::caller as msg_caller;
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

#[init]
fn init(data_canister_id: String) {
    services::config::set_data_canister_id(data_canister_id);
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
    
    let result = services::user_management::link_phone_to_account(
        principal_id.clone(),
        phone_number.clone(),
        pin,
    ).await;
    
    // Audit log
    match &result {
        Ok(_) => {
            log_audit(
                "link_phone_to_account",
                Some(principal_id.clone()),
                &format!("Linked phone {} to principal {}", phone_number, principal_id),
                true
            );
        }
        Err(e) => {
            log_audit(
                "link_phone_to_account",
                Some(principal_id),
                &format!("Failed: {}", e),
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
