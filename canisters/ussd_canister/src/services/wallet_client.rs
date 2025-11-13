/// Wallet Canister Client
/// Handles fiat transfers, balances, escrow, and transaction history
use candid::Principal;
use ic_cdk::call::Call;
use shared_types::{FiatCurrency, Transaction};
use std::cell::RefCell;

thread_local! {
    static WALLET_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// ============================================================================
// TEST MOCKS
// ============================================================================

#[cfg(any(test, feature = "test-utils"))]
use std::sync::Mutex;

#[cfg(any(test, feature = "test-utils"))]
lazy_static::lazy_static! {
    static ref MOCK_TRANSFER_FIAT: Mutex<Option<Box<dyn Fn(String, String, u64, FiatCurrency, String) -> Result<TransferResponse, String> + Send>>> = Mutex::new(None);
    static ref MOCK_GET_FIAT_BALANCE: Mutex<Option<Box<dyn Fn(String, FiatCurrency) -> Result<u64, String> + Send>>> = Mutex::new(None);
    static ref MOCK_GET_TRANSACTION_HISTORY: Mutex<Option<Box<dyn Fn(String, u32) -> Result<Vec<Transaction>, String> + Send>>> = Mutex::new(None);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_transfer_fiat<F>(mock: F)
where
    F: Fn(String, String, u64, FiatCurrency, String) -> Result<TransferResponse, String> + Send + 'static,
{
    *MOCK_TRANSFER_FIAT.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_get_fiat_balance<F>(mock: F)
where
    F: Fn(String, FiatCurrency) -> Result<u64, String> + Send + 'static,
{
    *MOCK_GET_FIAT_BALANCE.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_get_transaction_history<F>(mock: F)
where
    F: Fn(String, u32) -> Result<Vec<Transaction>, String> + Send + 'static,
{
    *MOCK_GET_TRANSACTION_HISTORY.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn clear_mocks() {
    *MOCK_TRANSFER_FIAT.lock().unwrap() = None;
    *MOCK_GET_FIAT_BALANCE.lock().unwrap() = None;
    *MOCK_GET_TRANSACTION_HISTORY.lock().unwrap() = None;
}

pub fn set_wallet_canister_id(canister_id: Principal) {
    WALLET_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(canister_id);
    });
}

pub fn get_wallet_canister_id() -> Result<Principal, String> {
    WALLET_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Wallet Canister ID not set".to_string())
    })
}

// ============================================================================
// TRANSFER REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct TransferRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: u64,
    pub currency: FiatCurrency,
    pub pin: String,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct TransferResponse {
    pub transaction_id: String,
    pub from_balance: u64,
    pub to_balance: u64,
}

// ============================================================================
// FIAT TRANSFERS
// ============================================================================

/// Transfer fiat currency between users
pub async fn transfer_fiat(
    from_user_id: String,
    to_user_id: String,
    amount: u64,
    currency: FiatCurrency,
    pin: String,
) -> Result<TransferResponse, String> {
    #[cfg(test)]
    {
        if let Some(mock) = MOCK_TRANSFER_FIAT.lock().unwrap().as_ref() {
            return mock(from_user_id, to_user_id, amount, currency, pin);
        }
    }
    
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling transfer_fiat: from={}, to={}, amount={}, currency={:?}", 
        from_user_id, to_user_id, amount, currency);
    
    let request = TransferRequest {
        from_user_id: from_user_id.clone(),
        to_user_id: to_user_id.clone(),
        amount,
        currency,
        pin,
    };
    
    let response = Call::unbounded_wait(canister_id, "transfer_fiat")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<TransferResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [WALLET_CLIENT] Transfer successful: tx_id={}", resp.transaction_id),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Transfer failed: {}", e),
    }
    
    result
}

// ============================================================================
// BALANCE OPERATIONS
// ============================================================================

/// Get fiat balance for a user
pub async fn get_fiat_balance(user_id: String, currency: FiatCurrency) -> Result<u64, String> {
    #[cfg(test)]
    {
        if let Some(mock) = MOCK_GET_FIAT_BALANCE.lock().unwrap().as_ref() {
            return mock(user_id, currency);
        }
    }
    
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling get_fiat_balance: user={}, currency={:?}", 
        user_id, currency);
    
    let response = Call::unbounded_wait(canister_id, "get_fiat_balance")
        .with_args(&(user_id.clone(), currency))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(balance) => ic_cdk::println!("✅ [WALLET_CLIENT] Balance: {} = {}", user_id, balance),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Get balance failed: {}", e),
    }
    
    result
}

/// Add to fiat balance (for deposits)
pub async fn add_fiat_balance(user_id: String, amount: u64, currency: FiatCurrency) -> Result<u64, String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling add_fiat_balance: user={}, amount={}, currency={:?}", 
        user_id, amount, currency);
    
    let response = Call::unbounded_wait(canister_id, "add_fiat_balance")
        .with_args(&(user_id.clone(), amount, currency))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(new_balance) => ic_cdk::println!("✅ [WALLET_CLIENT] Added balance: {} -> {}", user_id, new_balance),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Add balance failed: {}", e),
    }
    
    result
}

/// Deduct from fiat balance (for withdrawals)
pub async fn deduct_fiat_balance(user_id: String, amount: u64, currency: FiatCurrency) -> Result<u64, String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling deduct_fiat_balance: user={}, amount={}, currency={:?}", 
        user_id, amount, currency);
    
    let response = Call::unbounded_wait(canister_id, "deduct_fiat_balance")
        .with_args(&(user_id.clone(), amount, currency))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(new_balance) => ic_cdk::println!("✅ [WALLET_CLIENT] Deducted balance: {} -> {}", user_id, new_balance),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Deduct balance failed: {}", e),
    }
    
    result
}

// ============================================================================
// ESCROW OPERATIONS (for crypto sales to agents)
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateEscrowRequest {
    pub seller_user_id: String,
    pub buyer_user_id: String,
    pub amount: u64,
    pub currency: FiatCurrency,
    pub crypto_type: shared_types::CryptoType,
    pub crypto_amount: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateEscrowResponse {
    pub code: String,
    pub expires_at: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct Escrow {
    pub code: String,
    pub seller_user_id: String,
    pub buyer_user_id: String,
    pub amount: u64,
    pub currency: FiatCurrency,
    pub crypto_type: shared_types::CryptoType,
    pub crypto_amount: u64,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: EscrowStatus,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug, PartialEq)]
pub enum EscrowStatus {
    Active,
    Claimed,
    Cancelled,
    Expired,
}

/// Create escrow for crypto sale
pub async fn create_escrow(
    seller_user_id: String,
    buyer_user_id: String,
    amount: u64,
    currency: FiatCurrency,
    crypto_type: shared_types::CryptoType,
    crypto_amount: u64,
) -> Result<CreateEscrowResponse, String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling create_escrow: seller={}, buyer={}, amount={}", 
        seller_user_id, buyer_user_id, amount);
    
    let request = CreateEscrowRequest {
        seller_user_id: seller_user_id.clone(),
        buyer_user_id: buyer_user_id.clone(),
        amount,
        currency,
        crypto_type,
        crypto_amount,
    };
    
    let response = Call::unbounded_wait(canister_id, "create_escrow")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<CreateEscrowResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [WALLET_CLIENT] Escrow created: code={}", resp.code),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Create escrow failed: {}", e),
    }
    
    result
}

/// Claim escrow (agent verifies code)
pub async fn claim_escrow(code: String, agent_id: String) -> Result<(), String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling claim_escrow: code={}, agent={}", code, agent_id);
    
    let response = Call::unbounded_wait(canister_id, "claim_escrow")
        .with_args(&(code.clone(), agent_id))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(_) => ic_cdk::println!("✅ [WALLET_CLIENT] Escrow claimed: {}", code),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Claim escrow failed: {}", e),
    }
    
    result
}

/// Cancel escrow and refund
pub async fn cancel_escrow(code: String, user_id: String, pin: String) -> Result<(), String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling cancel_escrow: code={}, user={}", code, user_id);
    
    let response = Call::unbounded_wait(canister_id, "cancel_escrow")
        .with_args(&(code.clone(), user_id, pin))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(_) => ic_cdk::println!("✅ [WALLET_CLIENT] Escrow cancelled: {}", code),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Cancel escrow failed: {}", e),
    }
    
    result
}

/// Get escrow status
pub async fn get_escrow(code: String) -> Result<Escrow, String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling get_escrow: code={}", code);
    
    let response = Call::unbounded_wait(canister_id, "get_escrow")
        .with_args(&(code.clone(),))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<Escrow, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(escrow) => ic_cdk::println!("✅ [WALLET_CLIENT] Got escrow: {} (status={:?})", code, escrow.status),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Get escrow failed: {}", e),
    }
    
    result
}

// ============================================================================
// TRANSACTION HISTORY
// ============================================================================

/// Get user transaction history
pub async fn get_transaction_history(user_id: String, limit: Option<usize>) -> Result<Vec<Transaction>, String> {
    let canister_id = get_wallet_canister_id()?;
    
    ic_cdk::println!("📤 [WALLET_CLIENT] Calling get_transaction_history: user={}, limit={:?}", 
        user_id, limit);
    
    let response = Call::unbounded_wait(canister_id, "get_transaction_history")
        .with_args(&(user_id.clone(), limit))
        .await
        .map_err(|e| format!("❌ [WALLET_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<Transaction>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [WALLET_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(txs) => ic_cdk::println!("✅ [WALLET_CLIENT] Got {} transactions for {}", txs.len(), user_id),
        Err(e) => ic_cdk::println!("❌ [WALLET_CLIENT] Get transaction history failed: {}", e),
    }
    
    result
}
