/// User Canister Client
/// Handles all user-related operations: registration, authentication, profile management
use candid::Principal;
use ic_cdk::call::Call;
use shared_types::{RegisterUserRequest, AuditEntry};
use std::cell::RefCell;

// ============================================================================
// TYPES (matching user_canister API)
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,
    pub kyc_status: String,
    pub created_at: u64,
    pub last_active: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct ProfileUpdates {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub preferred_currency: Option<String>,
}

thread_local! {
    static USER_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// ============================================================================
// TEST MOCKS
// ============================================================================

#[cfg(any(test, feature = "test-utils"))]
use std::sync::Mutex;

#[cfg(any(test, feature = "test-utils"))]
lazy_static::lazy_static! {
    static ref MOCK_REGISTER_USER: Mutex<Option<Box<dyn Fn(Option<String>, Option<String>, String, String, String, String, String) -> Result<String, String> + Send>>> = Mutex::new(None);
    static ref MOCK_GET_USER_BY_PHONE: Mutex<Option<Box<dyn Fn(String) -> Result<UserProfile, String> + Send>>> = Mutex::new(None);
    static ref MOCK_VERIFY_PIN: Mutex<Option<Box<dyn Fn(String, String) -> Result<bool, String> + Send>>> = Mutex::new(None);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_register_user<F>(mock: F)
where
    F: Fn(Option<String>, Option<String>, String, String, String, String, String) -> Result<String, String> + Send + 'static,
{
    *MOCK_REGISTER_USER.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_get_user_by_phone<F>(mock: F)
where
    F: Fn(String) -> Result<UserProfile, String> + Send + 'static,
{
    *MOCK_GET_USER_BY_PHONE.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_verify_pin<F>(mock: F)
where
    F: Fn(String, String) -> Result<bool, String> + Send + 'static,
{
    *MOCK_VERIFY_PIN.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn clear_mocks() {
    *MOCK_REGISTER_USER.lock().unwrap() = None;
    *MOCK_GET_USER_BY_PHONE.lock().unwrap() = None;
    *MOCK_VERIFY_PIN.lock().unwrap() = None;
}

pub fn set_user_canister_id(canister_id: Principal) {
    USER_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(canister_id);
    });
}

pub fn get_user_canister_id() -> Result<Principal, String> {
    USER_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "User Canister ID not set".to_string())
    })
}

// ============================================================================
// USER REGISTRATION & EXISTENCE
// ============================================================================

/// Register a new user
/// Uses shared_types::RegisterUserRequest for type safety
pub async fn register_user(
    phone_number: Option<String>,
    principal_id: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
    pin: String,
    preferred_currency: String,
) -> Result<String, String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling register_user: phone={:?}, principal={:?}", 
        phone_number, principal_id);
    
    let request = RegisterUserRequest {
        phone_number,
        principal_id,
        first_name,
        last_name,
        email,
        pin,
        preferred_currency,
    };
    
    let response = Call::unbounded_wait(canister_id, "register_user")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<String, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(user_id) => ic_cdk::println!("✅ [USER_CLIENT] User registered: {}", user_id),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] Registration failed: {}", e),
    }
    
    result
}

/// Check if user exists (by phone or principal)
pub async fn user_exists(user_identifier: String) -> Result<bool, String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling user_exists: {}", user_identifier);
    
    let response = Call::unbounded_wait(canister_id, "user_exists")
        .with_args(&(user_identifier.clone(),))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(exists) => ic_cdk::println!("✅ [USER_CLIENT] User exists check: {} = {}", user_identifier, exists),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] User exists check failed: {}", e),
    }
    
    result
}

// ============================================================================
// PIN MANAGEMENT
// ============================================================================

/// Verify user PIN
pub async fn verify_pin(user_identifier: String, pin: String) -> Result<bool, String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling verify_pin: {}", user_identifier);
    
    let response = Call::unbounded_wait(canister_id, "verify_pin")
        .with_args(&(user_identifier.clone(), pin))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(valid) => ic_cdk::println!("✅ [USER_CLIENT] PIN verification: {} = {}", user_identifier, valid),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] PIN verification failed: {}", e),
    }
    
    result
}

/// Change user PIN
pub async fn change_pin(user_identifier: String, old_pin: String, new_pin: String) -> Result<(), String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling change_pin: {}", user_identifier);
    
    let response = Call::unbounded_wait(canister_id, "change_pin")
        .with_args(&(user_identifier.clone(), old_pin, new_pin))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(_) => ic_cdk::println!("✅ [USER_CLIENT] PIN changed: {}", user_identifier),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] PIN change failed: {}", e),
    }
    
    result
}

// ============================================================================
// PROFILE MANAGEMENT
// ============================================================================

/// Get user profile
pub async fn get_user_profile(user_identifier: String) -> Result<UserProfile, String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling get_user_profile_update: {}", user_identifier);
    
    let response = Call::unbounded_wait(canister_id, "get_user_profile_update")
        .with_args(&(user_identifier.clone(),))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<UserProfile, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(profile) => ic_cdk::println!("✅ [USER_CLIENT] Got profile: {} ({})", user_identifier, profile.email),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] Get profile failed: {}", e),
    }
    
    result
}

/// Update user profile
pub async fn update_user_profile(user_identifier: String, updates: ProfileUpdates) -> Result<(), String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling update_user_profile: {}", user_identifier);
    
    let response = Call::unbounded_wait(canister_id, "update_user_profile")
        .with_args(&(user_identifier.clone(), updates))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(_) => ic_cdk::println!("✅ [USER_CLIENT] Profile updated: {}", user_identifier),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] Profile update failed: {}", e),
    }
    
    result
}

/// Get user by phone number
pub async fn get_user_by_phone(phone: String) -> Result<UserProfile, String> {
    #[cfg(test)]
    {
        if let Some(mock) = MOCK_GET_USER_BY_PHONE.lock().unwrap().as_ref() {
            return mock(phone);
        }
    }
    
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling get_user_by_phone_update: {}", phone);
    
    let response = Call::unbounded_wait(canister_id, "get_user_by_phone_update")
        .with_args(&(phone.clone(),))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<UserProfile, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(profile) => ic_cdk::println!("✅ [USER_CLIENT] Got user by phone: {} -> {}", phone, profile.id),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] Get user by phone failed: {}", e),
    }
    
    result
}

/// Get user by principal
pub async fn get_user_by_principal(principal: String) -> Result<UserProfile, String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling get_user_by_principal_update: {}", principal);
    
    let response = Call::unbounded_wait(canister_id, "get_user_by_principal_update")
        .with_args(&(principal.clone(),))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<UserProfile, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(profile) => ic_cdk::println!("✅ [USER_CLIENT] Got user by principal: {} -> {}", principal, profile.id),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] Get user by principal failed: {}", e),
    }
    
    result
}

/// Link phone number to existing principal account
pub async fn link_phone_to_account(principal_id: String, phone_number: String) -> Result<(), String> {
    let canister_id = get_user_canister_id()?;
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling link_phone_to_account: principal={}, phone={}", 
        principal_id, phone_number);
    
    let response = Call::unbounded_wait(canister_id, "link_phone_to_account")
        .with_args(&(principal_id.clone(), phone_number.clone()))
        .await
        .map_err(|e| format!("❌ [USER_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [USER_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(_) => ic_cdk::println!("✅ [USER_CLIENT] Linked phone to account: {} -> {}", principal_id, phone_number),
        Err(e) => ic_cdk::println!("❌ [USER_CLIENT] Link phone failed: {}", e),
    }
    
    result
}

// ============================================================================
// AUDIT LOGGING (for debugging/monitoring)
// ============================================================================

/// Get audit log entries
pub async fn get_audit_log(limit: Option<u64>) -> Vec<AuditEntry> {
    let canister_id = match get_user_canister_id() {
        Ok(id) => id,
        Err(e) => {
            ic_cdk::println!("❌ [USER_CLIENT] Cannot get audit log: {}", e);
            return Vec::new();
        }
    };
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling get_audit_log: limit={:?}", limit);
    
    let response = match Call::unbounded_wait(canister_id, "get_audit_log")
        .with_args(&(limit,))
        .await
    {
        Ok(r) => r,
        Err(e) => {
            ic_cdk::println!("❌ [USER_CLIENT] Audit log call failed: {:?}", e);
            return Vec::new();
        }
    };
    
    let (entries,): (Vec<AuditEntry>,) = match response.candid_tuple() {
        Ok(e) => e,
        Err(e) => {
            ic_cdk::println!("❌ [USER_CLIENT] Audit log decode failed: {}", e);
            return Vec::new();
        }
    };
    
    ic_cdk::println!("✅ [USER_CLIENT] Got {} audit entries", entries.len());
    entries
}

/// Get audit log for specific user
pub async fn get_user_audit_log(user_id: String, limit: Option<u64>) -> Vec<AuditEntry> {
    let canister_id = match get_user_canister_id() {
        Ok(id) => id,
        Err(e) => {
            ic_cdk::println!("❌ [USER_CLIENT] Cannot get user audit log: {}", e);
            return Vec::new();
        }
    };
    
    ic_cdk::println!("📤 [USER_CLIENT] Calling get_user_audit_log: user={}, limit={:?}", user_id, limit);
    
    let response = match Call::unbounded_wait(canister_id, "get_user_audit_log")
        .with_args(&(user_id.clone(), limit))
        .await
    {
        Ok(r) => r,
        Err(e) => {
            ic_cdk::println!("❌ [USER_CLIENT] User audit log call failed: {:?}", e);
            return Vec::new();
        }
    };
    
    let (entries,): (Vec<AuditEntry>,) = match response.candid_tuple() {
        Ok(e) => e,
        Err(e) => {
            ic_cdk::println!("❌ [USER_CLIENT] User audit log decode failed: {}", e);
            return Vec::new();
        }
    };
    
    ic_cdk::println!("✅ [USER_CLIENT] Got {} audit entries for user {}", entries.len(), user_id);
    entries
}
