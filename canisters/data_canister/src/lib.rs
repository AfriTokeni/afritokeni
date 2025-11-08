use ic_cdk_macros::{init, update, query, pre_upgrade, post_upgrade};
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::msg_caller;
use std::collections::HashMap;
use std::cell::RefCell;

mod models;
mod operations;
mod security;

use models::*;

// ============================================================================
// State Management
// ============================================================================

thread_local! {
    static STATE: RefCell<DataCanisterState> = RefCell::new(DataCanisterState::new());
    static AUTHORIZED_CANISTERS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
}

#[derive(CandidType, Deserialize, Default)]
pub struct DataCanisterState {
    users: HashMap<String, User>,
    fiat_balances: HashMap<String, FiatBalance>,  // key: "user_id:currency"
    crypto_balances: HashMap<String, CryptoBalance>,  // key: user_id
    transactions: HashMap<String, Transaction>,
    user_pins: HashMap<String, UserPin>,
    audit_log: Vec<AuditEntry>,
}

impl DataCanisterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log_audit(&mut self, entry: AuditEntry) {
        self.audit_log.push(entry);
        if self.audit_log.len() > 10000 {
            self.audit_log.remove(0);
        }
    }
}

// ============================================================================
// Access Control - NON-CUSTODIAL
// ============================================================================

#[derive(Debug)]
enum AccessLevel {
    Controller,           // Platform admin
    AuthorizedCanister,   // USSD/Web canister
    #[allow(dead_code)]
    UserSelf(String),     // User accessing their own data
    Unauthorized,
}

/// Check caller's access level
fn get_access_level(user_id: Option<&str>) -> AccessLevel {
    let caller = msg_caller();
    
    // 1. Check if controller (admin)
    if ic_cdk::api::is_controller(&caller) {
        return AccessLevel::Controller;
    }
    
    // 2. Check if authorized canister (USSD/Web)
    let is_authorized_canister = AUTHORIZED_CANISTERS.with(|canisters| {
        canisters.borrow().contains(&caller)
    });
    
    if is_authorized_canister {
        return AccessLevel::AuthorizedCanister;
    }
    
    // 3. Check if user accessing their own data
    if let Some(uid) = user_id {
        let caller_text = caller.to_text();
        let is_own_data = STATE.with(|state| {
            state.borrow().users.get(uid)
                .and_then(|u| u.principal_id.as_ref())
                .map(|pid| pid == &caller_text)
                .unwrap_or(false)
        });
        
        if is_own_data {
            return AccessLevel::UserSelf(uid.to_string());
        }
    }
    
    AccessLevel::Unauthorized
}

/// Verify caller can access user's data
fn verify_user_access(user_id: &str) -> Result<(), String> {
    match get_access_level(Some(user_id)) {
        AccessLevel::Controller => Ok(()),
        AccessLevel::AuthorizedCanister => Ok(()),
        AccessLevel::UserSelf(_) => Ok(()),
        AccessLevel::Unauthorized => {
            Err(format!("Unauthorized: Cannot access user {}", user_id))
        }
    }
}

/// Verify caller can perform admin operations
fn verify_admin_access() -> Result<(), String> {
    match get_access_level(None) {
        AccessLevel::Controller => Ok(()),
        _ => Err("Unauthorized: Admin access required".to_string())
    }
}

/// Verify caller can perform canister operations
fn verify_canister_access() -> Result<(), String> {
    match get_access_level(None) {
        AccessLevel::Controller => Ok(()),
        AccessLevel::AuthorizedCanister => Ok(()),
        _ => Err("Unauthorized: Only authorized canisters can call this".to_string())
    }
}

// ============================================================================
// Initialization & Lifecycle
// ============================================================================

#[init]
fn init(ussd_canister_id: Option<String>, web_canister_id: Option<String>) {
    let mut authorized = Vec::new();
    
    if let Some(ussd_id) = ussd_canister_id {
        if let Ok(principal) = Principal::from_text(&ussd_id) {
            authorized.push(principal);
            ic_cdk::println!("✅ Authorized USSD canister: {}", ussd_id);
        }
    }
    
    if let Some(web_id) = web_canister_id {
        if let Ok(principal) = Principal::from_text(&web_id) {
            authorized.push(principal);
            ic_cdk::println!("✅ Authorized Web canister: {}", web_id);
        }
    }
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        *canisters.borrow_mut() = authorized;
    });
    
    ic_cdk::println!("🔐 Data canister initialized - NON-CUSTODIAL mode");
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("🔄 Pre-upgrade: State will be preserved");
}

#[post_upgrade]
fn post_upgrade(ussd_canister_id: Option<String>, web_canister_id: Option<String>) {
    init(ussd_canister_id, web_canister_id);
    ic_cdk::println!("✅ Post-upgrade: Canister restored");
}

// ============================================================================
// Admin Functions (Controller only)
// ============================================================================

#[update]
fn add_authorized_canister(canister_id: String) -> Result<(), String> {
    verify_admin_access()?;
    
    let principal = Principal::from_text(&canister_id)
        .map_err(|_| "Invalid principal ID".to_string())?;
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut list = canisters.borrow_mut();
        if !list.contains(&principal) {
            list.push(principal);
            ic_cdk::println!("✅ Added authorized canister: {}", canister_id);
        }
    });
    
    Ok(())
}

#[update]
fn remove_authorized_canister(canister_id: String) -> Result<(), String> {
    verify_admin_access()?;
    
    let principal = Principal::from_text(&canister_id)
        .map_err(|_| "Invalid principal ID".to_string())?;
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        let mut list = canisters.borrow_mut();
        list.retain(|p| p != &principal);
        ic_cdk::println!("❌ Removed authorized canister: {}", canister_id);
    });
    
    Ok(())
}

#[query]
fn list_authorized_canisters() -> Result<Vec<String>, String> {
    verify_admin_access()?;
    
    AUTHORIZED_CANISTERS.with(|canisters| {
        Ok(canisters.borrow().iter().map(|p| p.to_text()).collect())
    })
}

// ============================================================================
// User Management
// ============================================================================

/// Create user (canister only - called during registration)
#[update]
async fn create_user(user_data: CreateUserData) -> Result<User, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        operations::user_ops::create_user(&mut s, user_data)
    })
}

// Removed link_principal_to_user - principal is stored directly in User struct

/// Get user data (user can access their own, canisters can access any)
#[query]
async fn get_user(user_id: String) -> Result<Option<User>, String> {
    verify_user_access(&user_id)?;
    
    STATE.with(|state| {
        Ok(state.borrow().users.get(&user_id).cloned())
    })
}

/// Get user by phone (canister only)
#[query]
async fn get_user_by_phone(phone_number: String) -> Result<Option<User>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        Ok(state.borrow().users.values()
            .find(|u| u.phone_number.as_ref() == Some(&phone_number))
            .cloned())
    })
}

/// Update last active timestamp (canister only - called on every transaction)
#[update]
async fn update_last_active(user_id: String) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        operations::user_ops::update_last_active(&mut state.borrow_mut(), &user_id)
    })
}

/// Get user by principal (user accessing their own data)
#[query]
async fn get_my_user_data() -> Result<Option<User>, String> {
    let caller_text = msg_caller().to_text();
    
    STATE.with(|state| {
        Ok(state.borrow().users.values()
            .find(|u| u.principal_id.as_ref() == Some(&caller_text))
            .cloned())
    })
}

// ============================================================================
// Balance Operations
// ============================================================================

/// Get fiat balance (user can access their own, canisters can access any)
#[query]
async fn get_fiat_balance(user_id: String, currency: FiatCurrency) -> Result<u64, String> {
    verify_user_access(&user_id)?;
    
    STATE.with(|state| {
        let s = state.borrow();
        let balance_key = format!("{}:{}", user_id, currency.code());
        Ok(s.fiat_balances.get(&balance_key).map(|b| b.balance).unwrap_or(0))
    })
}

/// Get crypto balance (user can access their own, canisters can access any)
#[query]
async fn get_crypto_balance(user_id: String) -> Result<CryptoBalance, String> {
    verify_user_access(&user_id)?;
    
    STATE.with(|state| {
        let s = state.borrow();
        Ok(s.crypto_balances.get(&user_id).cloned().unwrap_or(CryptoBalance {
            user_id: user_id.clone(),
            ckbtc: 0,
            ckusdc: 0,
            updated_at: ic_cdk::api::time() / 1_000_000_000,
        }))
    })
}

/// Get my balances (user accessing their own data)
#[query]
async fn get_my_balances() -> Result<(Vec<FiatBalance>, CryptoBalance), String> {
    let caller_text = msg_caller().to_text();
    
    // Find user by principal
    let user_id = STATE.with(|state| {
        state.borrow().users.values()
            .find(|u| u.principal_id.as_ref() == Some(&caller_text))
            .map(|u| u.id.clone())
    }).ok_or("User not found for this principal".to_string())?;
    
    STATE.with(|state| {
        let s = state.borrow();
        
        // Get all fiat balances for this user
        let fiat_balances: Vec<FiatBalance> = s.fiat_balances.values()
            .filter(|b| b.user_id == user_id)
            .cloned()
            .collect();
        
        // Get crypto balance
        let crypto_balance = s.crypto_balances.get(&user_id).cloned().unwrap_or(CryptoBalance {
            user_id: user_id.clone(),
            ckbtc: 0,
            ckusdc: 0,
            updated_at: ic_cdk::api::time() / 1_000_000_000,
        });
        
        Ok((fiat_balances, crypto_balance))
    })
}

/// Set fiat balance (canister only - pure CRUD, no validation)
#[update]
async fn set_fiat_balance(
    user_id: String,
    currency: String,
    amount: u64,
) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let balance_key = format!("{}:{}", user_id, currency);
        
        let currency_enum = FiatCurrency::from_code(&currency)
            .ok_or(format!("Invalid currency code: {}", currency))?;
        
        let balance = FiatBalance {
            user_id: user_id.clone(),
            currency: currency_enum,
            balance: amount,
            updated_at: ic_cdk::api::time() / 1_000_000_000,
        };
        
        s.fiat_balances.insert(balance_key, balance);
        Ok(())
    })
}

/// Deposit fiat (canister only - called by USSD/Web after agent confirms)
#[update]
async fn deposit_fiat(
    user_id: String,
    amount: u64,
    currency: FiatCurrency,
    description: Option<String>
) -> Result<Transaction, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        operations::balance_ops::deposit_fiat(&mut s, user_id, amount, currency, description)
    })
}

/// Withdraw fiat (canister only - called by USSD after PIN verification and agent booking)
#[update]
async fn withdraw_fiat(
    user_id: String,
    amount: u64,
    currency: FiatCurrency,
    description: Option<String>
) -> Result<Transaction, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        operations::balance_ops::withdraw_fiat(&mut s, user_id, amount, currency, description)
    })
}

/// Transfer fiat (canister only - called by USSD/Web after PIN verification)
#[update]
async fn transfer_fiat(
    from_user: String,
    to_user: String,
    amount: u64,
    currency: FiatCurrency,
    description: Option<String>
) -> Result<Transaction, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        operations::balance_ops::transfer_fiat(&mut s, from_user, to_user, amount, currency, description)
    })
}

// ============================================================================
// PIN Security
// ============================================================================

/// Setup PIN (canister only - called during registration)
#[update]
async fn setup_user_pin(user_id: String, pin: String, salt: String) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        security::pin_ops::setup_pin_with_salt(&mut s, user_id, &pin, salt)
    })
}

/// Verify PIN (canister only - called during transactions)
#[update]
async fn verify_user_pin(user_id: String, pin: String) -> Result<bool, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        security::pin_ops::verify_pin(&mut s, user_id, &pin)
    })
}

/// Check if PIN is locked (canister only)
#[query]
async fn is_pin_locked(user_id: String) -> Result<bool, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::is_pin_locked(&state.borrow(), user_id)
    })
}

/// Get failed PIN attempts (canister only)
#[query]
async fn get_failed_attempts(user_id: String) -> Result<u32, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::get_failed_attempts(&state.borrow(), user_id)
    })
}

/// Get remaining lockout time in seconds (canister only - for UX)
#[query]
async fn get_remaining_lockout_time(user_id: String) -> Result<u64, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::get_remaining_lockout_time(&state.borrow(), user_id)
    })
}

/// Reset PIN attempts (canister only - called after successful verification)
#[update]
async fn reset_pin_attempts(user_id: String) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::reset_attempts(&mut state.borrow_mut(), user_id)
    })
}

/// Change PIN (canister only - requires old PIN verification)
#[update]
async fn change_pin(user_id: String, old_pin: String, new_pin: String, new_salt: String) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::change_pin(&mut state.borrow_mut(), user_id, &old_pin, &new_pin, new_salt)
    })
}

/// Check for account takeover (canister only - security check)
#[query]
async fn check_account_takeover(user_id: String) -> Result<bool, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::fraud_detection::check_account_takeover(&state.borrow(), &user_id)
    })
}

// ============================================================================
// Crypto Balance Operations
// ============================================================================

/// Update crypto balance (canister only - called after ledger operations)
#[update]
async fn update_crypto_balance(
    user_id: String,
    ckbtc_delta: i64,
    ckusdc_delta: i64,
) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        operations::balance_ops::update_crypto_balance(&mut s, user_id, ckbtc_delta, ckusdc_delta)
    })
}

// ============================================================================
// Transaction History
// ============================================================================

/// Store transaction (canister only - pure CRUD)
#[update]
async fn store_transaction(tx: Transaction) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.transactions.insert(tx.id.clone(), tx);
        Ok(())
    })
}

/// Get user transactions (user can access their own, canisters can access any)
#[query]
async fn get_user_transactions(
    user_id: String,
    limit: Option<usize>,
    offset: Option<usize>
) -> Result<Vec<Transaction>, String> {
    verify_user_access(&user_id)?;
    
    STATE.with(|state| {
        let s = state.borrow();
        let mut transactions: Vec<Transaction> = s.transactions.values()
            .filter(|tx| tx.from_user.as_ref() == Some(&user_id) || tx.to_user.as_ref() == Some(&user_id))
            .cloned()
            .collect();
        
        transactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(50);
        
        Ok(transactions.into_iter().skip(offset).take(limit).collect())
    })
}

/// Get my transactions (user accessing their own data)
#[query]
async fn get_my_transactions(
    limit: Option<usize>,
    offset: Option<usize>
) -> Result<Vec<Transaction>, String> {
    let caller_text = msg_caller().to_text();
    
    // Find user by principal
    let user_id = STATE.with(|state| {
        state.borrow().users.values()
            .find(|u| u.principal_id.as_ref() == Some(&caller_text))
            .map(|u| u.id.clone())
    }).ok_or("User not found for this principal".to_string())?;
    
    get_user_transactions(user_id, limit, offset).await
}

// ============================================================================
// System Stats
// ============================================================================

#[query]
async fn get_system_stats() -> Result<SystemStats, String> {
    verify_admin_access()?;
    
    STATE.with(|state| {
        let s = state.borrow();
        Ok(SystemStats {
            total_users: s.users.len(),
            total_transactions: s.transactions.len(),
            total_fiat_balances: s.fiat_balances.len(),
            total_crypto_balances: s.crypto_balances.len(),
        })
    })
}

// ============================================================================
// Audit Log Queries
// ============================================================================

/// Get audit log (only admin/controller can access)
#[query]
async fn get_audit_log(limit: Option<usize>, offset: Option<usize>) -> Result<Vec<AuditEntry>, String> {
    verify_admin_access()?;
    
    STATE.with(|state| {
        let s = state.borrow();
        let start = offset.unwrap_or(0);
        let end = start + limit.unwrap_or(100).min(1000);
        Ok(s.audit_log.iter().skip(start).take(end - start).cloned().collect())
    })
}

/// Get audit log count
#[query]
async fn get_audit_log_count() -> Result<usize, String> {
    verify_admin_access()?;
    
    STATE.with(|state| {
        Ok(state.borrow().audit_log.len())
    })
}

// Export Candid interface
ic_cdk::export_candid!();

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use models::*;

    // ============================================================================
    // Fiat Currency Tests - Normal Cases
    // ============================================================================

    #[test]
    fn test_fiat_currency_code() {
        assert_eq!(FiatCurrency::UGX.code(), "UGX");
        assert_eq!(FiatCurrency::KES.code(), "KES");
        assert_eq!(FiatCurrency::NGN.code(), "NGN");
        assert_eq!(FiatCurrency::ZAR.code(), "ZAR");
    }

    #[test]
    fn test_fiat_currency_from_code_valid() {
        assert!(FiatCurrency::from_code("UGX").is_some());
        assert!(FiatCurrency::from_code("KES").is_some());
        assert!(FiatCurrency::from_code("NGN").is_some());
        assert_eq!(FiatCurrency::from_code("UGX").unwrap(), FiatCurrency::UGX);
    }

    // ============================================================================
    // Fiat Currency Tests - Edge Cases
    // ============================================================================

    #[test]
    fn test_fiat_currency_from_code_invalid() {
        assert!(FiatCurrency::from_code("INVALID").is_none());
        assert!(FiatCurrency::from_code("USD").is_none()); // Not African currency
        assert!(FiatCurrency::from_code("EUR").is_none());
    }

    #[test]
    fn test_fiat_currency_from_code_empty() {
        assert!(FiatCurrency::from_code("").is_none());
    }

    #[test]
    fn test_fiat_currency_from_code_lowercase() {
        assert!(FiatCurrency::from_code("ugx").is_none()); // Case sensitive
        assert!(FiatCurrency::from_code("kes").is_none());
    }

    #[test]
    fn test_fiat_currency_from_code_with_spaces() {
        assert!(FiatCurrency::from_code(" UGX").is_none());
        assert!(FiatCurrency::from_code("UGX ").is_none());
        assert!(FiatCurrency::from_code("U GX").is_none());
    }

    #[test]
    fn test_all_39_african_currencies() {
        // Test all 39 currencies are accessible
        let currencies = vec![
            "UGX", "KES", "TZS", "RWF", "BIF", "NGN", "GHS", "XOF", "GMD", "SLL",
            "LRD", "ZAR", "BWP", "LSL", "SZL", "NAD", "ZMW", "MWK", "EGP", "MAD",
            "TND", "DZD", "LYD", "XAF", "CDF", "AOA", "ETB", "SOS", "SDG", "SSP",
            "DJF", "ERN", "MUR", "SCR", "MGA", "KMF", "CVE", "STN", "MRU"
        ];
        
        for code in currencies {
            assert!(FiatCurrency::from_code(code).is_some(), "Currency {} should be valid", code);
        }
    }

    // ============================================================================
    // User Type Tests
    // ============================================================================

    #[test]
    fn test_user_type_variants() {
        let user = UserType::User;
        let agent = UserType::Agent;
        let admin = UserType::Admin;
        
        assert!(matches!(user, UserType::User));
        assert!(matches!(agent, UserType::Agent));
        assert!(matches!(admin, UserType::Admin));
    }

    #[test]
    fn test_user_type_equality() {
        assert_eq!(UserType::User, UserType::User);
        assert_ne!(UserType::User, UserType::Agent);
        assert_ne!(UserType::Agent, UserType::Admin);
    }

    // ============================================================================
    // KYC Status Tests
    // ============================================================================

    #[test]
    fn test_kyc_status_variants() {
        let not_started = KYCStatus::NotStarted;
        let pending = KYCStatus::Pending;
        let approved = KYCStatus::Approved;
        let rejected = KYCStatus::Rejected;
        
        assert!(matches!(not_started, KYCStatus::NotStarted));
        assert!(matches!(pending, KYCStatus::Pending));
        assert!(matches!(approved, KYCStatus::Approved));
        assert!(matches!(rejected, KYCStatus::Rejected));
    }

    #[test]
    fn test_kyc_status_equality() {
        assert_eq!(KYCStatus::Pending, KYCStatus::Pending);
        assert_ne!(KYCStatus::Pending, KYCStatus::Approved);
        assert_ne!(KYCStatus::Approved, KYCStatus::Rejected);
    }

    // ============================================================================
    // Audit Entry Tests
    // ============================================================================

    #[test]
    fn test_audit_entry_creation_with_user() {
        let entry = AuditEntry {
            timestamp: 1699459200,
            action: "test_action".to_string(),
            user_id: Some("user123".to_string()),
            details: "test details".to_string(),
        };
        
        assert_eq!(entry.action, "test_action");
        assert_eq!(entry.timestamp, 1699459200);
        assert!(entry.user_id.is_some());
        assert_eq!(entry.user_id.unwrap(), "user123");
        assert_eq!(entry.details, "test details");
    }

    #[test]
    fn test_audit_entry_creation_without_user() {
        let entry = AuditEntry {
            timestamp: 1699459200,
            action: "system_action".to_string(),
            user_id: None,
            details: "automated task".to_string(),
        };
        
        assert_eq!(entry.action, "system_action");
        assert!(entry.user_id.is_none());
    }

    #[test]
    fn test_audit_entry_with_empty_details() {
        let entry = AuditEntry {
            timestamp: 0,
            action: "action".to_string(),
            user_id: None,
            details: "".to_string(),
        };
        
        assert_eq!(entry.details, "");
    }

    #[test]
    fn test_audit_entry_with_large_details() {
        let large_details = "x".repeat(10000);
        let entry = AuditEntry {
            timestamp: 1699459200,
            action: "large_action".to_string(),
            user_id: Some("user123".to_string()),
            details: large_details.clone(),
        };
        
        assert_eq!(entry.details.len(), 10000);
    }

    #[test]
    fn test_audit_entry_timestamp_boundaries() {
        let entry_min = AuditEntry {
            timestamp: 0,
            action: "min".to_string(),
            user_id: None,
            details: "".to_string(),
        };
        
        let entry_max = AuditEntry {
            timestamp: u64::MAX,
            action: "max".to_string(),
            user_id: None,
            details: "".to_string(),
        };
        
        assert_eq!(entry_min.timestamp, 0);
        assert_eq!(entry_max.timestamp, u64::MAX);
    }

    // ============================================================================
    // Data Canister State Tests
    // ============================================================================

    #[test]
    fn test_data_canister_state_initialization() {
        let state = DataCanisterState::new();
        assert_eq!(state.users.len(), 0);
        assert_eq!(state.fiat_balances.len(), 0);
        assert_eq!(state.crypto_balances.len(), 0);
        assert_eq!(state.transactions.len(), 0);
        assert_eq!(state.user_pins.len(), 0);
        assert_eq!(state.audit_log.len(), 0);
    }

    #[test]
    fn test_audit_log_retention_limit() {
        let mut state = DataCanisterState::new();
        
        // Add 10,001 entries
        for i in 0..10_001 {
            let entry = AuditEntry {
                timestamp: i as u64,
                action: format!("action_{}", i),
                user_id: Some(format!("user_{}", i)),
                details: format!("details_{}", i),
            };
            state.log_audit(entry);
        }
        
        // Should only keep last 10,000
        assert_eq!(state.audit_log.len(), 10_000);
        // First entry should be removed
        assert_eq!(state.audit_log[0].action, "action_1");
    }
}
