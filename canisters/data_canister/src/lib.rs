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
    static AGENT_ACTIVITIES: RefCell<std::collections::BTreeMap<String, shared_types::AgentActivity>> = RefCell::new(std::collections::BTreeMap::new());
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct DataCanisterState {
    users: HashMap<String, User>,
    fiat_balances: HashMap<String, FiatBalance>,  // key: "user_id:currency"
    crypto_balances: HashMap<String, CryptoBalance>,  // key: user_id
    transactions: HashMap<String, Transaction>,
    user_pins: HashMap<String, UserPin>,
    escrows: HashMap<String, Escrow>,  // key: escrow_code
    settlements: Vec<MonthlySettlement>,
    // Agent-specific data (added for agent_canister)
    deposit_transactions: HashMap<String, DepositTransaction>,  // key: deposit_code
    withdrawal_transactions: HashMap<String, WithdrawalTransaction>,  // key: withdrawal_code
    agent_balances: HashMap<String, AgentBalance>,  // key: "agent_id:currency"
    // Agent reviews (added to replace Juno storage)
    agent_reviews: HashMap<String, shared_types::AgentReview>,  // key: review_id
}

impl DataCanisterState {
    pub fn new() -> Self {
        Self::default()
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
    
    // For testing only: Allow anonymous if no authorized canisters are set
    // This makes PocketIC tests work without explicit authorization while keeping production strict.
    #[cfg(test)]
    {
        let has_authorized = AUTHORIZED_CANISTERS.with(|canisters| {
            !canisters.borrow().is_empty()
        });
        if !has_authorized && caller == Principal::anonymous() {
            return AccessLevel::AuthorizedCanister;
        }
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
    ic_cdk::println!("🔄 Pre-upgrade: Serializing state to stable memory");

    // Extract state from thread-local storage (clone the inner value, not the Ref)
    let state = STATE.with(|s| (*s.borrow()).clone());
    let authorized = AUTHORIZED_CANISTERS.with(|c| (*c.borrow()).clone());
    let agent_activities = AGENT_ACTIVITIES.with(|a| (*a.borrow()).clone());

    // Serialize to stable memory
    ic_cdk::storage::stable_save((state, authorized, agent_activities))
        .expect("Failed to save state to stable memory");

    ic_cdk::println!("✅ Pre-upgrade: State serialized successfully");
}

#[post_upgrade]
fn post_upgrade(ussd_canister_id: Option<String>, web_canister_id: Option<String>) {
    ic_cdk::println!("🔄 Post-upgrade: Restoring state from stable memory");

    // Restore state from stable memory
    let (state, authorized, agent_activities): (
        DataCanisterState,
        Vec<Principal>,
        std::collections::BTreeMap<String, shared_types::AgentActivity>
    ) = ic_cdk::storage::stable_restore()
        .expect("Failed to restore state from stable memory");

    // Restore to thread-local storage
    STATE.with(|s| *s.borrow_mut() = state);
    AUTHORIZED_CANISTERS.with(|c| *c.borrow_mut() = authorized);
    AGENT_ACTIVITIES.with(|a| *a.borrow_mut() = agent_activities);

    ic_cdk::println!("✅ Post-upgrade: State restored successfully");

    // Re-initialize authorized canisters if provided (for manual override)
    if ussd_canister_id.is_some() || web_canister_id.is_some() {
        ic_cdk::println!("📝 Post-upgrade: Re-initializing authorized canisters from args");
        init(ussd_canister_id, web_canister_id);
    }
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
    
    shared_types::audit::log_success(
        "add_authorized_canister",
        None,
        format!("Added authorized canister: {}", canister_id)
    );
    
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
    
    shared_types::audit::log_success(
        "remove_authorized_canister",
        None,
        format!("Removed authorized canister: {}", canister_id)
    );
    
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
fn create_user(request: shared_types::CreateUserRequest) -> Result<User, String> {
    ic_cdk::println!("📥 Data canister received create_user request");
    
    verify_canister_access()?;
    
    // Convert strings to enums
    let user_type = match request.user_type_str.as_str() {
        "User" => UserType::User,
        "Admin" => UserType::Admin,
        "Agent" => UserType::Agent,
        _ => return Err(format!("Invalid user type: {}", request.user_type_str)),
    };
    
    let preferred_currency = FiatCurrency::from_string(&request.preferred_currency_str)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let user_data = CreateUserData {
        user_type,
        preferred_currency,
        email: request.email,
        first_name: request.first_name,
        last_name: request.last_name,
        principal_id: request.principal_id,
        phone_number: request.phone_number,
    };
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        operations::user_ops::create_user(&mut s, user_data)
    })
}

// Removed link_principal_to_user - principal is stored directly in User struct

/// Get user data (user can access their own, canisters can access any)
#[query]
fn get_user(user_id: String) -> Result<Option<User>, String> {
    verify_user_access(&user_id)?;
    
    STATE.with(|state| {
        Ok(state.borrow().users.get(&user_id).cloned())
    })
}

/// Get user by phone (canister only)
#[query]
fn get_user_by_phone(phone_number: String) -> Result<Option<User>, String> {
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
fn get_my_user_data() -> Result<Option<User>, String> {
    let caller_text = msg_caller().to_text();
    
    STATE.with(|state| {
        Ok(state.borrow().users.values()
            .find(|u| u.principal_id.as_ref() == Some(&caller_text))
            .cloned())
    })
}

/// Get user by principal (canister only)
#[query]
fn get_user_by_principal(principal_id: String) -> Result<Option<User>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        Ok(state.borrow().users.values()
            .find(|u| u.principal_id.as_ref() == Some(&principal_id))
            .cloned())
    })
}

/// Update user phone number (canister only)
#[update]
async fn update_user_phone(request: shared_types::UpdateUserPhoneRequest) -> Result<(), String> {
    verify_canister_access()?;

    STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Check if user exists
        let user = state.users.get_mut(&request.user_id)
            .ok_or_else(|| format!("User not found: {}", request.user_id))?;

        // Update phone number
        user.phone_number = Some(request.phone_number);

        Ok(())
    })
}

/// Update KYC status (canister only - for compliance)
#[update]
async fn update_kyc_status(user_id: String, status: KYCStatus) -> Result<(), String> {
    verify_canister_access()?;

    STATE.with(|state| {
        operations::user_ops::update_kyc_status(&mut state.borrow_mut(), &user_id, status)
    })
}

// ============================================================================
// Balance Operations
// ============================================================================

/// Get fiat balance (user can access their own, canisters can access any)
#[query]
fn get_fiat_balance(user_id: String, currency: FiatCurrency) -> Result<u64, String> {
    verify_user_access(&user_id)?;
    
    STATE.with(|state| {
        let s = state.borrow();
        let balance_key = format!("{}:{}", user_id, currency.code());
        Ok(s.fiat_balances.get(&balance_key).map(|b| b.balance).unwrap_or(0))
    })
}

/// Get crypto balance (user can access their own, canisters can access any)
#[query]
fn get_crypto_balance(user_id: String) -> Result<CryptoBalance, String> {
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
fn get_my_balances() -> Result<(Vec<FiatBalance>, CryptoBalance), String> {
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
async fn setup_user_pin(request: shared_types::SetupPinRequest) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        security::pin_ops::setup_pin_with_salt(&mut s, request.user_id, &request.pin, request.salt)
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
fn is_pin_locked(user_id: String) -> Result<bool, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::is_pin_locked(&state.borrow(), user_id)
    })
}

/// Get failed PIN attempts (canister only)
#[query]
fn get_failed_attempts(user_id: String) -> Result<u32, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::get_failed_attempts(&state.borrow(), user_id)
    })
}

/// Get remaining lockout time in seconds (canister only - for UX)
#[query]
fn get_remaining_lockout_time(user_id: String) -> Result<u64, String> {
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

/// Store PIN hash (canister only - for Argon2 hashes from user_canister)
#[update]
async fn store_pin_hash(user_id: String, pin_hash: String) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::store_pin_hash(&mut state.borrow_mut(), user_id, pin_hash)
    })
}

/// Get PIN hash (canister only - for verification in user_canister)
#[query]
fn get_pin_hash(user_id: String) -> Result<String, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::get_pin_hash(&state.borrow(), user_id)
    })
}

/// Increment failed PIN attempts (canister only)
#[update]
async fn increment_failed_attempts(user_id: String) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        security::pin_ops::increment_failed_attempts(&mut state.borrow_mut(), user_id)
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

// Note: Fraud detection and account takeover checks have been moved to the business_logic_canister.

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

/// Set crypto balance directly (canister only - for testing)
#[update]
async fn set_crypto_balance(
    user_id: String,
    ckbtc: u64,
    ckusdc: u64,
) -> Result<(), String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let now = ic_cdk::api::time() / 1_000_000_000;
        
        let balance = CryptoBalance {
            user_id: user_id.clone(),
            ckbtc,
            ckusdc,
            updated_at: now,
        };
        
        s.crypto_balances.insert(user_id, balance);
        Ok(())
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

// =========================================================================
// Settlement Storage (Canister-only writes; queries for admin/canisters)
// =========================================================================

/// Store settlements for a given month (idempotent: replaces existing entries for the month)
#[update]
async fn store_settlements(month: String, settlements: Vec<MonthlySettlement>) -> Result<(), String> {
    verify_canister_access()?;
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        // Remove existing for the month
        s.settlements.retain(|ms| ms.month != month);
        // Insert all provided
        for mut ms in settlements {
            // Ensure month matches for safety
            ms.month = month.clone();
            s.settlements.push(ms);
        }
        Ok(())
    })
}

/// Mark settlement paid for a given month and agent principal text
#[update]
async fn mark_settlement_paid_record(month: String, agent_principal: String) -> Result<(), String> {
    verify_canister_access()?;
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let now = ic_cdk::api::time();
        let mut found = false;
        for ms in s.settlements.iter_mut() {
            if ms.month == month && ms.agent_principal == agent_principal {
                ms.paid = true;
                ms.paid_date = Some(now);
                found = true;
            }
        }
        if !found { return Err("Settlement not found".to_string()); }
        Ok(())
    })
}

/// Get settlements for a month
#[query]
fn get_settlements_for_month(month: String) -> Result<Vec<MonthlySettlement>, String> {
    // Allow admin or authorized canister
    match get_access_level(None) {
        AccessLevel::Controller | AccessLevel::AuthorizedCanister => {}
        _ => return Err("Unauthorized".to_string()),
    }
    STATE.with(|state| {
        let s = state.borrow();
        Ok(s.settlements.iter().filter(|ms| ms.month == month).cloned().collect())
    })
}

/// Get settlements for a specific agent principal text
#[query]
fn get_agent_settlements(agent_principal: String) -> Result<Vec<MonthlySettlement>, String> {
    match get_access_level(None) {
        AccessLevel::Controller | AccessLevel::AuthorizedCanister => {}
        _ => return Err("Unauthorized".to_string()),
    }
    STATE.with(|state| {
        let s = state.borrow();
        Ok(s.settlements.iter().filter(|ms| ms.agent_principal == agent_principal).cloned().collect())
    })
}

/// Get user transactions (user can access their own, canisters can access any)
#[query]
fn get_user_transactions(
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
fn get_my_transactions(
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
    
    get_user_transactions(user_id, limit, offset)
}

// ============================================================================
// Escrow CRUD Operations (Pure Storage - NO Business Logic)
// ============================================================================

/// Store escrow (canister only - pure CRUD)
#[update]
async fn store_escrow(escrow: Escrow) -> Result<(), String> {
    verify_canister_access()?;
    
    let code = escrow.code.clone();
    let user_id = escrow.user_id.clone();
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.escrows.insert(escrow.code.clone(), escrow);
    });
    
    shared_types::audit::log_success(
        "store_escrow",
        Some(user_id),
        format!("Stored escrow: {}", code)
    );
    
    Ok(())
}

/// Get escrow by code (canister only)
#[query]
fn get_escrow(code: String) -> Result<Option<Escrow>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        Ok(state.borrow().escrows.get(&code).cloned())
    })
}

/// Update escrow status (canister only - pure CRUD)
#[update]
async fn update_escrow_status(code: String, status: EscrowStatus) -> Result<(), String> {
    verify_canister_access()?;
    
    let user_id = STATE.with(|state| {
        let mut s = state.borrow_mut();
        if let Some(escrow) = s.escrows.get_mut(&code) {
            escrow.status = status;
            if status == EscrowStatus::Claimed {
                escrow.claimed_at = Some(ic_cdk::api::time());
            }
            Ok(escrow.user_id.clone())
        } else {
            Err(format!("Escrow not found: {}", code))
        }
    })?;
    
    shared_types::audit::log_success(
        "update_escrow_status",
        Some(user_id),
        format!("Updated escrow {} to status: {:?}", code, status)
    );
    
    Ok(())
}

/// Delete escrow (canister only - pure CRUD)
#[update]
async fn delete_escrow(code: String) -> Result<(), String> {
    verify_canister_access()?;
    
    let user_id = STATE.with(|state| {
        let s = state.borrow();
        s.escrows.get(&code).map(|e| e.user_id.clone())
    });
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.escrows.remove(&code);
    });
    
    shared_types::audit::log_success(
        "delete_escrow",
        user_id,
        format!("Deleted escrow: {}", code)
    );
    
    Ok(())
}

/// Get all active escrows (canister only - for cleanup jobs)
#[query]
fn get_active_escrows() -> Result<Vec<Escrow>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let s = state.borrow();
        Ok(s.escrows.values()
            .filter(|e| e.status == EscrowStatus::Active)
            .cloned()
            .collect())
    })
}

// ============================================================================
// System Stats
// ============================================================================

#[query]
fn get_system_stats() -> Result<SystemStats, String> {
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
// Audit Log Queries (Using Shared Audit Library)
// ============================================================================

/// Get audit log (only admin/controller can access)
#[query]
fn get_audit_log(limit: Option<usize>) -> Result<Vec<AuditEntry>, String> {
    verify_admin_access()?;
    Ok(shared_types::audit::get_audit_log(limit))
}

/// Get audit log count
#[query]
fn get_audit_log_count() -> Result<usize, String> {
    verify_admin_access()?;
    let stats = shared_types::audit::get_audit_stats();
    Ok(stats.total_entries)
}

/// Get audit log statistics (admin only)
#[query]
fn get_audit_stats() -> Result<shared_types::audit::AuditStats, String> {
    verify_admin_access()?;
    Ok(shared_types::audit::get_audit_stats())
}

/// Get failed operations (admin only - for debugging)
#[query]
fn get_failed_operations(limit: Option<usize>) -> Result<Vec<AuditEntry>, String> {
    verify_admin_access()?;
    Ok(shared_types::audit::get_failed_operations(limit))
}

// ============================================================================
// Agent Operations - Deposit & Withdrawal CRUD (Canister Only)
// ============================================================================

/// Store deposit transaction (canister only - pure CRUD)
#[update]
async fn store_deposit_transaction(deposit: DepositTransaction) -> Result<DepositTransaction, String> {
    verify_canister_access()?;
    
    let code = deposit.deposit_code.clone();
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.deposit_transactions.insert(code.clone(), deposit.clone());
    });
    
    shared_types::audit::log_success(
        "store_deposit_transaction",
        Some(deposit.user_id.clone()),
        format!("Stored deposit: {}", code)
    );
    
    Ok(deposit)
}

/// Get deposit by code (canister only)
#[query]
fn get_deposit_by_code(code: String) -> Result<Option<DepositTransaction>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        Ok(state.borrow().deposit_transactions.get(&code).cloned())
    })
}

/// Update deposit status (canister only)
#[update]
async fn update_deposit_status(code: String, status: AgentTransactionStatus) -> Result<DepositTransaction, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let deposit = s.deposit_transactions.get_mut(&code)
            .ok_or_else(|| "Deposit not found".to_string())?;
        
        deposit.status = status.clone();
        if status == AgentTransactionStatus::Confirmed {
            deposit.confirmed_at = Some(ic_cdk::api::time());
        }
        
        Ok(deposit.clone())
    })
}

/// Get agent deposits (canister only)
#[query]
fn get_agent_deposits(agent_id: String) -> Result<Vec<DepositTransaction>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let deposits: Vec<DepositTransaction> = state.borrow()
            .deposit_transactions
            .values()
            .filter(|d| d.agent_id == agent_id)
            .cloned()
            .collect();
        Ok(deposits)
    })
}

/// Store withdrawal transaction (canister only - pure CRUD)
#[update]
async fn store_withdrawal_transaction(withdrawal: WithdrawalTransaction) -> Result<WithdrawalTransaction, String> {
    verify_canister_access()?;
    
    let code = withdrawal.withdrawal_code.clone();
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.withdrawal_transactions.insert(code.clone(), withdrawal.clone());
    });
    
    shared_types::audit::log_success(
        "store_withdrawal_transaction",
        Some(withdrawal.user_id.clone()),
        format!("Stored withdrawal: {}", code)
    );
    
    Ok(withdrawal)
}

/// Get withdrawal by code (canister only)
#[query]
fn get_withdrawal_by_code(code: String) -> Result<Option<WithdrawalTransaction>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        Ok(state.borrow().withdrawal_transactions.get(&code).cloned())
    })
}

/// Update withdrawal status (canister only)
#[update]
async fn update_withdrawal_status(code: String, status: AgentTransactionStatus) -> Result<WithdrawalTransaction, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let withdrawal = s.withdrawal_transactions.get_mut(&code)
            .ok_or_else(|| "Withdrawal not found".to_string())?;
        
        withdrawal.status = status.clone();
        if status == AgentTransactionStatus::Confirmed {
            withdrawal.confirmed_at = Some(ic_cdk::api::time());
        }
        
        Ok(withdrawal.clone())
    })
}

/// Get agent withdrawals (canister only)
#[query]
fn get_agent_withdrawals(agent_id: String) -> Result<Vec<WithdrawalTransaction>, String> {
    verify_canister_access()?;
    
    STATE.with(|state| {
        let withdrawals: Vec<WithdrawalTransaction> = state.borrow()
            .withdrawal_transactions
            .values()
            .filter(|w| w.agent_id == agent_id)
            .cloned()
            .collect();
        Ok(withdrawals)
    })
}

/// Get agent balance (canister only)
#[query]
fn get_agent_balance(agent_id: String, currency: String) -> Result<Option<AgentBalance>, String> {
    verify_canister_access()?;
    
    let key = format!("{}:{}", agent_id, currency);
    
    STATE.with(|state| {
        Ok(state.borrow().agent_balances.get(&key).cloned())
    })
}

/// Update agent balance (canister only - pure CRUD)
#[update]
async fn update_agent_balance(balance: AgentBalance) -> Result<AgentBalance, String> {
    verify_canister_access()?;
    
    let key = format!("{}:{}", balance.agent_id, balance.currency);
    
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.agent_balances.insert(key, balance.clone());
    });
    
    shared_types::audit::log_success(
        "update_agent_balance",
        Some(balance.agent_id.clone()),
        format!("Updated balance for currency: {}", balance.currency)
    );
    
    Ok(balance)
}

/// Get all agent balances (canister only)
#[query]
fn get_all_agent_balances() -> Result<Vec<AgentBalance>, String> {
    verify_canister_access()?;

    STATE.with(|state| {
        Ok(state.borrow().agent_balances.values().cloned().collect())
    })
}

// ============================================================================
// Agent Activity Operations - Fraud Detection (Canister Only)
// ============================================================================

/// Get agent activity for fraud detection analysis
///
/// Returns activity metrics for a specific agent and currency, including:
/// - Daily deposit/withdrawal counts and volumes
/// - Hourly and 24h operation velocity
/// - User-agent pair frequency (for detecting coordination)
///
/// # Arguments
/// * `agent_id` - The agent's unique identifier
/// * `currency` - The currency code (e.g., "UGX", "NGN")
///
/// # Access Control
/// Canister-only endpoint. Must be called by authorized canisters.
#[query]
fn get_agent_activity(agent_id: String, currency: String) -> Result<Option<shared_types::AgentActivity>, String> {
    verify_canister_access()?;

    AGENT_ACTIVITIES.with(|activities| {
        let activities = activities.borrow();
        Ok(operations::agent_activity_ops::get_agent_activity(&activities, agent_id, currency))
    })
}

/// Store or update agent activity for fraud detection
///
/// Persists agent activity metrics used for fraud detection and risk analysis.
/// Called by agent_canister after each deposit/withdrawal operation.
///
/// # Arguments
/// * `activity` - The agent activity record to store
///
/// # Returns
/// The stored activity record on success
///
/// # Access Control
/// Canister-only endpoint. Must be called by authorized canisters.
#[update]
async fn store_agent_activity(activity: shared_types::AgentActivity) -> Result<shared_types::AgentActivity, String> {
    verify_canister_access()?;

    AGENT_ACTIVITIES.with(|activities| {
        let mut activities = activities.borrow_mut();
        operations::agent_activity_ops::store_agent_activity(&mut activities, activity)
    })
}

// ============================================================================
// Agent Review Operations
// ============================================================================

/// Create agent review
///
/// # Arguments
/// * `request` - Review creation request with agent_id, user_id, rating, comment
///
/// # Returns
/// The created review with generated ID and timestamp
///
/// # Access Control
/// Canister-only endpoint (called by agent_canister or user_canister)
#[update]
async fn create_review(request: shared_types::CreateReviewRequest) -> Result<shared_types::AgentReview, String> {
    verify_canister_access()?;

    // Validate rating (1-5)
    if request.rating < 1 || request.rating > 5 {
        return Err("Rating must be between 1 and 5".to_string());
    }

    // Validate comment length
    if request.comment.is_empty() {
        return Err("Comment cannot be empty".to_string());
    }
    if request.comment.len() > 500 {
        return Err("Comment too long (max 500 characters)".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Generate review ID
        let review_id = format!("review_{}", ic_cdk::api::time());

        let review = shared_types::AgentReview {
            id: review_id.clone(),
            agent_id: request.agent_id,
            user_id: request.user_id,
            user_name: request.user_name,
            rating: request.rating,
            comment: request.comment,
            created_at: ic_cdk::api::time() / 1_000_000_000, // Convert to seconds
            verified_transaction: request.verified_transaction,
        };

        state.agent_reviews.insert(review_id, review.clone());

        Ok(review)
    })
}

/// Get all reviews for an agent
///
/// # Arguments
/// * `agent_id` - Agent ID to get reviews for
///
/// # Returns
/// List of reviews for the agent, sorted by most recent first
///
/// # Access Control
/// Public query (anyone can read reviews)
#[query]
fn get_agent_reviews(agent_id: String) -> Result<Vec<shared_types::AgentReview>, String> {
    STATE.with(|state| {
        let state = state.borrow();

        let mut reviews: Vec<shared_types::AgentReview> = state.agent_reviews
            .values()
            .filter(|r| r.agent_id == agent_id)
            .cloned()
            .collect();

        // Sort by most recent first
        reviews.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(reviews)
    })
}

/// Get reviews by rating for an agent
///
/// # Arguments
/// * `agent_id` - Agent ID to get reviews for
/// * `rating` - Filter by specific rating (1-5)
///
/// # Returns
/// List of reviews matching the rating, sorted by most recent first
///
/// # Access Control
/// Public query (anyone can read reviews)
#[query]
fn get_agent_reviews_by_rating(agent_id: String, rating: u8) -> Result<Vec<shared_types::AgentReview>, String> {
    if rating < 1 || rating > 5 {
        return Err("Rating must be between 1 and 5".to_string());
    }

    STATE.with(|state| {
        let state = state.borrow();

        let mut reviews: Vec<shared_types::AgentReview> = state.agent_reviews
            .values()
            .filter(|r| r.agent_id == agent_id && r.rating == rating)
            .cloned()
            .collect();

        // Sort by most recent first
        reviews.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(reviews)
    })
}

/// Get verified reviews for an agent (linked to actual transactions)
///
/// # Arguments
/// * `agent_id` - Agent ID to get verified reviews for
///
/// # Returns
/// List of verified reviews (with transaction IDs), sorted by most recent first
///
/// # Access Control
/// Public query (anyone can read reviews)
#[query]
fn get_verified_reviews(agent_id: String) -> Result<Vec<shared_types::AgentReview>, String> {
    STATE.with(|state| {
        let state = state.borrow();

        let mut reviews: Vec<shared_types::AgentReview> = state.agent_reviews
            .values()
            .filter(|r| r.agent_id == agent_id && r.verified_transaction.is_some())
            .cloned()
            .collect();

        // Sort by most recent first
        reviews.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(reviews)
    })
}

/// Get average rating for an agent
///
/// # Arguments
/// * `agent_id` - Agent ID to calculate rating for
///
/// # Returns
/// Average rating (0.0 if no reviews) and total review count
///
/// # Access Control
/// Public query (anyone can read reviews)
#[query]
fn get_agent_rating(agent_id: String) -> Result<(f32, usize), String> {
    STATE.with(|state| {
        let state = state.borrow();

        let reviews: Vec<&shared_types::AgentReview> = state.agent_reviews
            .values()
            .filter(|r| r.agent_id == agent_id)
            .collect();

        let count = reviews.len();

        if count == 0 {
            return Ok((0.0, 0));
        }

        let sum: u32 = reviews.iter().map(|r| r.rating as u32).sum();
        let average = sum as f32 / count as f32;

        Ok((average, count))
    })
}

/// Delete review (admin only - for moderation)
///
/// # Arguments
/// * `review_id` - ID of review to delete
///
/// # Returns
/// Success or error
///
/// # Access Control
/// Controller only (for content moderation)
#[update]
async fn delete_review(review_id: String) -> Result<(), String> {
    // Only controller can delete reviews
    if !ic_cdk::api::is_controller(&msg_caller()) {
        return Err("Only controller can delete reviews".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();

        if state.agent_reviews.remove(&review_id).is_some() {
            Ok(())
        } else {
            Err("Review not found".to_string())
        }
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
            caller: "aaaa-bbbb-cccc-dddd".to_string(),
            user_id: Some("user123".to_string()),
            details: "test details".to_string(),
            success: true,
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
            caller: "aaaa-bbbb-cccc-dddd".to_string(),
            user_id: None,
            details: "automated task".to_string(),
            success: false,
        };
        
        assert_eq!(entry.action, "system_action");
        assert!(entry.user_id.is_none());
    }

    #[test]
    fn test_audit_entry_with_empty_details() {
        let entry = AuditEntry {
            timestamp: 0,
            action: "action".to_string(),
            caller: "aaaa-bbbb-cccc-dddd".to_string(),
            user_id: None,
            details: "".to_string(),
            success: true,
        };
        
        assert_eq!(entry.details, "");
    }

    #[test]
    fn test_audit_entry_with_large_details() {
        let large_details = "x".repeat(10000);
        let entry = AuditEntry {
            timestamp: 1699459200,
            action: "large_action".to_string(),
            caller: "aaaa-bbbb-cccc-dddd".to_string(),
            user_id: Some("user123".to_string()),
            details: large_details.clone(),
            success: true,
        };
        
        assert_eq!(entry.details.len(), 10000);
    }

    #[test]
    fn test_audit_entry_timestamp_boundaries() {
        let entry_min = AuditEntry {
            timestamp: 0,
            action: "min".to_string(),
            caller: "aaaa-bbbb-cccc-dddd".to_string(),
            user_id: None,
            details: "".to_string(),
            success: true,
        };
        
        let entry_max = AuditEntry {
            timestamp: u64::MAX,
            action: "max".to_string(),
            caller: "aaaa-bbbb-cccc-dddd".to_string(),
            user_id: None,
            details: "".to_string(),
            success: true,
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
        assert_eq!(state.escrows.len(), 0);
        assert_eq!(state.settlements.len(), 0);
    }

}
