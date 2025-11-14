/// Crypto Canister Client
/// Handles all cryptocurrency operations: buy, sell, send, swap, escrow
use candid::Principal;
use ic_cdk::call::Call;
use shared_types::{CryptoType, FiatCurrency};
use std::cell::RefCell;

thread_local! {
    static CRYPTO_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// ============================================================================
// TEST MOCKS
// ============================================================================

#[cfg(any(test, feature = "test-utils"))]
use std::sync::Mutex;

#[cfg(any(test, feature = "test-utils"))]
lazy_static::lazy_static! {
    static ref MOCK_BUY_CRYPTO: Mutex<Option<Box<dyn Fn(String, u64, FiatCurrency, CryptoType, String) -> Result<BuyCryptoResponse, String> + Send>>> = Mutex::new(None);
    static ref MOCK_SELL_CRYPTO: Mutex<Option<Box<dyn Fn(String, u64, CryptoType, FiatCurrency, String) -> Result<BuyCryptoResponse, String> + Send>>> = Mutex::new(None);
    static ref MOCK_SEND_CRYPTO: Mutex<Option<Box<dyn Fn(String, String, u64, CryptoType, String) -> Result<String, String> + Send>>> = Mutex::new(None);
    static ref MOCK_SWAP_CRYPTO: Mutex<Option<Box<dyn Fn(String, CryptoType, CryptoType, u64, String) -> Result<SwapCryptoResponse, String> + Send>>> = Mutex::new(None);
    static ref MOCK_CHECK_CRYPTO_BALANCE: Mutex<Option<Box<dyn Fn(String, CryptoType) -> Result<u64, String> + Send>>> = Mutex::new(None);
    static ref MOCK_APPROVE_CRYPTO_SPENDING: Mutex<Option<Box<dyn Fn(String, u64, CryptoType, String) -> Result<u64, String> + Send>>> = Mutex::new(None);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_buy_crypto<F>(mock: F)
where
    F: Fn(String, u64, FiatCurrency, CryptoType, String) -> Result<BuyCryptoResponse, String> + Send + 'static,
{
    *MOCK_BUY_CRYPTO.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_sell_crypto<F>(mock: F)
where
    F: Fn(String, u64, CryptoType, FiatCurrency, String) -> Result<BuyCryptoResponse, String> + Send + 'static,
{
    *MOCK_SELL_CRYPTO.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_send_crypto<F>(mock: F)
where
    F: Fn(String, String, u64, CryptoType, String) -> Result<String, String> + Send + 'static,
{
    *MOCK_SEND_CRYPTO.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_swap_crypto<F>(mock: F)
where
    F: Fn(String, CryptoType, CryptoType, u64, String) -> Result<SwapCryptoResponse, String> + Send + 'static,
{
    *MOCK_SWAP_CRYPTO.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_check_crypto_balance<F>(mock: F)
where
    F: Fn(String, CryptoType) -> Result<u64, String> + Send + 'static,
{
    *MOCK_CHECK_CRYPTO_BALANCE.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_approve_crypto_spending<F>(mock: F)
where
    F: Fn(String, u64, CryptoType, String) -> Result<u64, String> + Send + 'static,
{
    *MOCK_APPROVE_CRYPTO_SPENDING.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn clear_mocks() {
    *MOCK_BUY_CRYPTO.lock().unwrap() = None;
    *MOCK_SELL_CRYPTO.lock().unwrap() = None;
    *MOCK_SEND_CRYPTO.lock().unwrap() = None;
    *MOCK_SWAP_CRYPTO.lock().unwrap() = None;
    *MOCK_CHECK_CRYPTO_BALANCE.lock().unwrap() = None;
    *MOCK_APPROVE_CRYPTO_SPENDING.lock().unwrap() = None;
}

pub fn set_crypto_canister_id(canister_id: Principal) {
    CRYPTO_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(canister_id);
    });
}

pub fn get_crypto_canister_id() -> Result<Principal, String> {
    CRYPTO_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Crypto Canister ID not set".to_string())
    })
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct BuyCryptoRequest {
    pub user_identifier: String,
    pub fiat_amount: u64,
    pub currency: String,  // ACL pattern: accept string
    pub crypto_type: String,  // ACL pattern: accept string
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct BuyCryptoResponse {
    pub transaction_id: String,
    pub crypto_amount: u64,
    pub fiat_amount: u64,
    pub exchange_rate: f64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct SellCryptoRequest {
    pub user_identifier: String,
    pub crypto_amount: u64,
    pub crypto_type: String,  // ACL pattern
    pub currency: String,  // ACL pattern
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct SendCryptoRequest {
    pub user_identifier: String,
    pub to_address: String,
    pub amount: u64,
    pub crypto_type: String,  // ACL pattern
    pub pin: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct SwapCryptoRequest {
    pub user_identifier: String,
    pub from_crypto: String,  // ACL pattern
    pub to_crypto: String,  // ACL pattern
    pub amount: u64,
    pub pin: String,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct SwapCryptoResponse {
    pub transaction_id: String,
    pub from_amount: u64,
    pub to_amount: u64,
    pub exchange_rate: f64,
    pub spread_bps: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct ApproveCryptoRequest {
    pub user_identifier: String,
    pub crypto_amount: u64,
    pub crypto_type: String,
    pub pin: String,
}

// ============================================================================
// BUY/SELL OPERATIONS
// ============================================================================

/// Buy cryptocurrency with fiat
pub async fn buy_crypto(
    user_identifier: String,
    fiat_amount: u64,
    fiat_currency: FiatCurrency,
    crypto_type: CryptoType,
    pin: String,
) -> Result<BuyCryptoResponse, String> {
    #[cfg(any(test, feature = "test-utils"))]
    {
        if let Some(mock) = MOCK_BUY_CRYPTO.lock().unwrap().as_ref() {
            return mock(user_identifier, fiat_amount, fiat_currency, crypto_type, pin);
        }
    }
    
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling buy_crypto: user={}, amount={}, currency={:?}, type={:?}", 
        user_identifier, fiat_amount, fiat_currency, crypto_type);
    
    let request = BuyCryptoRequest {
        user_identifier: user_identifier.clone(),
        fiat_amount,
        currency: fiat_currency.code().to_string(),  // Convert enum to string
        crypto_type: format!("{:?}", crypto_type),  // Convert enum to string
        pin,
        device_fingerprint: None,
        geo_location: None,
    };
    
    let response = Call::unbounded_wait(canister_id, "buy_crypto")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<BuyCryptoResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Buy successful: tx_id={}, crypto_amount={}", 
            resp.transaction_id, resp.crypto_amount),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Buy failed: {}", e),
    }
    
    result
}

/// Sell cryptocurrency for fiat
pub async fn sell_crypto(
    user_identifier: String,
    crypto_amount: u64,
    crypto_type: CryptoType,
    fiat_currency: FiatCurrency,
    pin: String,
) -> Result<BuyCryptoResponse, String> {
    #[cfg(any(test, feature = "test-utils"))]
    {
        if let Some(mock) = MOCK_SELL_CRYPTO.lock().unwrap().as_ref() {
            return mock(user_identifier, crypto_amount, crypto_type, fiat_currency, pin);
        }
    }
    
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling sell_crypto: user={}, amount={}, type={:?}, currency={:?}", 
        user_identifier, crypto_amount, crypto_type, fiat_currency);
    
    let request = SellCryptoRequest {
        user_identifier: user_identifier.clone(),
        crypto_amount,
        crypto_type: format!("{:?}", crypto_type),
        currency: fiat_currency.code().to_string(),
        pin,
        device_fingerprint: None,
        geo_location: None,
    };
    
    let response = Call::unbounded_wait(canister_id, "sell_crypto")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<BuyCryptoResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Sell successful: tx_id={}, fiat_amount={}", 
            resp.transaction_id, resp.fiat_amount),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Sell failed: {}", e),
    }
    
    result
}

/// Approve platform to spend user's crypto (ICRC-2 step 1)
pub async fn approve_crypto_spending(
    user_identifier: String,
    crypto_amount: u64,
    crypto_type: CryptoType,
    pin: String,
) -> Result<u64, String> {
    #[cfg(any(test, feature = "test-utils"))]
    {
        if let Some(mock) = MOCK_APPROVE_CRYPTO_SPENDING.lock().unwrap().as_ref() {
            return mock(user_identifier, crypto_amount, crypto_type, pin);
        }
    }
    
    let canister_id = get_crypto_canister_id()?;
    
    shared_types::audit::log_inter_canister_call(
        "crypto_canister",
        "approve_crypto_spending",
        Some(user_identifier.clone())
    );
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling approve_crypto_spending: user={}, amount={}, type={:?}", 
        user_identifier, crypto_amount, crypto_type);
    
    let request = ApproveCryptoRequest {
        user_identifier: user_identifier.clone(),
        crypto_amount,
        crypto_type: format!("{:?}", crypto_type),
        pin,
    };
    
    let response = Call::unbounded_wait(canister_id, "approve_crypto_spending")
        .with_args(&(request,))
        .await
        .map_err(|e| {
            shared_types::audit::log_failure(
                "approve_crypto_spending",
                Some(user_identifier.clone()),
                format!("Call failed: {:?}", e)
            );
            format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e)
        })?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| {
            shared_types::audit::log_failure(
                "approve_crypto_spending",
                Some(user_identifier.clone()),
                format!("Decode failed: {}", e)
            );
            format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e)
        })?;
    
    match &result {
        Ok(approval_id) => {
            shared_types::audit::log_success(
                "approve_crypto_spending",
                Some(user_identifier.clone()),
                format!("Approved {} {:?}, approval_id={}", crypto_amount, crypto_type, approval_id)
            );
            ic_cdk::println!("✅ [CRYPTO_CLIENT] Approval successful: approval_id={}", approval_id);
        }
        Err(e) => {
            shared_types::audit::log_failure(
                "approve_crypto_spending",
                Some(user_identifier),
                format!("Approval failed: {}", e)
            );
            ic_cdk::println!("❌ [CRYPTO_CLIENT] Approval failed: {}", e);
        }
    }
    
    result
}

// ============================================================================
// SEND & BALANCE OPERATIONS
// ============================================================================

/// Send cryptocurrency to another user or external address
pub async fn send_crypto(
    user_identifier: String,
    recipient: String,
    amount: u64,
    crypto_type: CryptoType,
    pin: String,
) -> Result<String, String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling send_crypto: user={}, to={}, amount={}, type={:?}", 
        user_identifier, recipient, amount, crypto_type);
    
    let request = SendCryptoRequest {
        user_identifier: user_identifier.clone(),
        to_address: recipient.clone(),
        amount,
        crypto_type: format!("{:?}", crypto_type),
        pin,
        device_fingerprint: None,
        geo_location: None,
    };
    
    let response = Call::unbounded_wait(canister_id, "send_crypto")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<String, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(tx_id) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Send successful: tx_id={}", tx_id),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Send failed: {}", e),
    }
    
    result
}

/// Check crypto balance
pub async fn check_crypto_balance(user_identifier: String, crypto_type: CryptoType) -> Result<u64, String> {
    #[cfg(any(test, feature = "test-utils"))]
    {
        if let Some(mock) = MOCK_CHECK_CRYPTO_BALANCE.lock().unwrap().as_ref() {
            return mock(user_identifier, crypto_type);
        }
    }
    
    let canister_id = get_crypto_canister_id()?;
    
    // Convert CryptoType to string for the API
    let crypto_type_str = match crypto_type {
        CryptoType::CkBTC => "CkBTC",
        CryptoType::CkUSDC => "CkUSDC",
    };
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling check_crypto_balance: user={}, type={}", 
        user_identifier, crypto_type_str);
    
    let response = Call::unbounded_wait(canister_id, "check_crypto_balance")
        .with_args(&(user_identifier.clone(), crypto_type_str.to_string()))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<u64, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(balance) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Balance: {} = {}", user_identifier, balance),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Check balance failed: {}", e),
    }
    
    result
}

// ============================================================================
// SWAP OPERATIONS
// ============================================================================

/// Swap between cryptocurrencies (BTC ↔ USDC)
pub async fn swap_crypto(
    user_identifier: String,
    from_crypto: CryptoType,
    to_crypto: CryptoType,
    from_amount: u64,
    pin: String,
) -> Result<SwapCryptoResponse, String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling swap_crypto: user={}, from={:?}, to={:?}, amount={}", 
        user_identifier, from_crypto, to_crypto, from_amount);
    
    let request = SwapCryptoRequest {
        user_identifier: user_identifier.clone(),
        from_crypto: format!("{:?}", from_crypto),
        to_crypto: format!("{:?}", to_crypto),
        amount: from_amount,
        pin,
    };
    
    let response = Call::unbounded_wait(canister_id, "swap_crypto")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<SwapCryptoResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Swap successful: tx_id={}, to_amount={}", 
            resp.transaction_id, resp.to_amount),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Swap failed: {}", e),
    }
    
    result
}

/// Get spread basis points (for displaying swap fees)
pub async fn get_spread_basis_points() -> Result<u64, String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling get_spread_basis_points");
    
    let response = Call::unbounded_wait(canister_id, "get_spread_basis_points")
        .with_args(&())
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (spread_bps,): (u64,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    ic_cdk::println!("✅ [CRYPTO_CLIENT] Spread BPS: {}", spread_bps);
    Ok(spread_bps)
}

// ============================================================================
// ESCROW OPERATIONS (for crypto-to-cash with agents)
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateEscrowRequest {
    pub user_identifier: String,
    pub agent_id: String,
    pub crypto_amount: u64,
    pub crypto_type: CryptoType,
    pub fiat_amount: u64,
    pub fiat_currency: FiatCurrency,
    pub pin: String,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateEscrowResponse {
    pub code: String,
    pub expires_at: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct VerifyEscrowRequest {
    pub code: String,
    pub agent_id: String,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct Escrow {
    pub code: String,
    pub user_id: String,
    pub agent_id: String,
    pub crypto_amount: u64,
    pub crypto_type: CryptoType,
    pub fiat_amount: u64,
    pub fiat_currency: FiatCurrency,
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

/// Create escrow for crypto-to-cash transaction
pub async fn create_escrow(
    user_identifier: String,
    agent_id: String,
    crypto_amount: u64,
    crypto_type: CryptoType,
    fiat_amount: u64,
    fiat_currency: FiatCurrency,
    pin: String,
) -> Result<CreateEscrowResponse, String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling create_escrow: user={}, agent={}, crypto_amount={}", 
        user_identifier, agent_id, crypto_amount);
    
    let request = CreateEscrowRequest {
        user_identifier: user_identifier.clone(),
        agent_id: agent_id.clone(),
        crypto_amount,
        crypto_type,
        fiat_amount,
        fiat_currency,
        pin,
    };
    
    let response = Call::unbounded_wait(canister_id, "create_escrow")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<CreateEscrowResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Escrow created: code={}", resp.code),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Create escrow failed: {}", e),
    }
    
    result
}

/// Verify and claim escrow (agent side)
pub async fn verify_escrow(code: String, agent_id: String) -> Result<String, String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling verify_escrow: code={}, agent={}", code, agent_id);
    
    let request = VerifyEscrowRequest {
        code: code.clone(),
        agent_id,
    };
    
    let response = Call::unbounded_wait(canister_id, "verify_escrow")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<String, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(msg) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Escrow verified: {}", msg),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Verify escrow failed: {}", e),
    }
    
    result
}

/// Cancel escrow and refund crypto
pub async fn cancel_escrow(code: String, user_id: String, pin: String) -> Result<(), String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling cancel_escrow: code={}, user={}", code, user_id);
    
    let response = Call::unbounded_wait(canister_id, "cancel_escrow")
        .with_args(&(code.clone(), user_id, pin))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(_) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Escrow cancelled: {}", code),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Cancel escrow failed: {}", e),
    }
    
    result
}

/// Get escrow status
pub async fn get_escrow_status(code: String) -> Result<Escrow, String> {
    let canister_id = get_crypto_canister_id()?;
    
    ic_cdk::println!("📤 [CRYPTO_CLIENT] Calling get_escrow_status: code={}", code);
    
    let response = Call::unbounded_wait(canister_id, "get_escrow_status")
        .with_args(&(code.clone(),))
        .await
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<Escrow, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [CRYPTO_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(escrow) => ic_cdk::println!("✅ [CRYPTO_CLIENT] Got escrow: {} (status={:?})", code, escrow.status),
        Err(e) => ic_cdk::println!("❌ [CRYPTO_CLIENT] Get escrow status failed: {}", e),
    }
    
    result
}
