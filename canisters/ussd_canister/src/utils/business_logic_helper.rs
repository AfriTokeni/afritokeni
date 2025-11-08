// Helper functions to call Business Logic Canister
// USSD is presentation layer only - all business logic goes through Business Logic Canister

use candid::Principal;
use ic_cdk::call::Call;

/// Get Business Logic Canister ID from environment
fn get_business_logic_canister_id() -> Result<Principal, String> {
    // TODO: Set this during deployment
    std::env::var("BUSINESS_LOGIC_CANISTER_ID")
        .ok()
        .and_then(|id| Principal::from_text(&id).ok())
        .ok_or_else(|| "BUSINESS_LOGIC_CANISTER_ID not set".to_string())
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
pub enum CryptoType {
    ckBTC,
    ckUSDC,
}
