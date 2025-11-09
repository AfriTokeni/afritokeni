// Helper functions to call Business Logic Canister
// USSD is presentation layer only - all business logic goes through Business Logic Canister

use candid::Principal;
use ic_cdk::call::Call;

/// Get Business Logic Canister ID
fn get_business_logic_canister_id() -> Result<Principal, String> {
    crate::get_business_logic_canister_id()
}

/// Verify user PIN
pub async fn verify_pin(phone_number: &str, pin: &str) -> Result<bool, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "verify_pin")
        .with_arg((phone_number.to_string(), pin.to_string()))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Get user balances (fiat and crypto)
pub async fn get_balances(phone_number: &str) -> Result<UserBalances, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "get_balances")
        .with_arg((phone_number.to_string(),))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<UserBalances, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Send money to another user
pub async fn send_money(
    from_phone: &str,
    to_phone: &str,
    amount: u64,
    currency: &str,
    pin: &str,
) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "send_money_to_phone")
        .with_arg((
            from_phone.to_string(),
            to_phone.to_string(),
            amount,
            currency.to_string(),
            pin.to_string(),
        ))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Buy cryptocurrency
pub async fn buy_crypto(
    phone_number: &str,
    fiat_amount: u64,
    fiat_currency: &str,
    crypto_type: CryptoType,
    pin: &str,
) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "buy_crypto")
        .with_arg((
            phone_number.to_string(),
            fiat_amount,
            fiat_currency.to_string(),
            crypto_type,
            pin.to_string(),
        ))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Send cryptocurrency
pub async fn send_crypto(
    phone_number: &str,
    to_address: &str,
    amount: u64,
    crypto_type: CryptoType,
    pin: &str,
) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "send_crypto")
        .with_arg((
            phone_number.to_string(),
            to_address.to_string(),
            amount,
            crypto_type,
            pin.to_string(),
        ))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Check if user exists
pub async fn user_exists(phone_number: &str) -> Result<bool, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    let response = Call::unbounded_wait(canister_id, "user_exists")
        .with_arg(phone_number.to_string())
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<bool, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Register new user
pub async fn register_user(
    phone_number: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
    pin: &str,
    preferred_currency: &str,
) -> Result<String, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    // Generate Principal ID from phone number (deterministic, self-authenticating)
    use candid::Principal;
    use sha2::{Sha256, Digest};
    
    // Use phone number as seed for deterministic principal generation
    let mut hasher = Sha256::new();
    hasher.update(phone_number.as_bytes());
    let hash_result = hasher.finalize();
    let hash: [u8; 32] = hash_result.into();
    
    // Create self-authenticating principal from hash
    let principal = Principal::self_authenticating(&hash);
    let principal_id = principal.to_text();
    
    ic_cdk::println!("📤 Calling register_user with:");
    ic_cdk::println!("  phone_number: {:?}", Some(phone_number));
    ic_cdk::println!("  principal_id: {:?}", Some(&principal_id));
    ic_cdk::println!("  first_name: {:?}", first_name);
    ic_cdk::println!("  last_name: {:?}", last_name);
    ic_cdk::println!("  email: {:?}", email);
    ic_cdk::println!("  preferred_currency: {:?}", preferred_currency);
    ic_cdk::println!("  pin: {:?}", pin);
    
    let response = Call::unbounded_wait(canister_id, "register_user")
        .with_args(&(
            Some(phone_number.to_string()), // phone_number: Option<String>
            Some(principal_id),              // principal_id: Option<String>
            first_name.to_string(),          // first_name: String
            last_name.to_string(),           // last_name: String
            email.to_string(),               // email: String
            preferred_currency.to_string(),  // preferred_currency: String
            pin.to_string(),                 // pin: String
        ))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<String, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Update user's language preference
pub async fn update_user_language(phone_number: &str, language_code: &str) -> Result<(), String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling update_user_language: phone={}, lang={}", phone_number, language_code);
    
    let response = Call::unbounded_wait(canister_id, "update_user_language")
        .with_args(&(phone_number, language_code))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

// ============================================================================
// Types (matching Business Logic Canister)
// ============================================================================

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserBalances {
    pub fiat_balances: Vec<FiatBalance>,
    pub ckbtc_balance: u64,
    pub ckusdc_balance: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FiatBalance {
    pub currency: String,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransactionResult {
    pub transaction_id: String,
    pub from_user: String,
    pub to_user: String,
    pub amount: u64,
    pub currency: String,
    pub new_balance: u64,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum CryptoType {
    ckBTC,
    ckUSDC,
}
