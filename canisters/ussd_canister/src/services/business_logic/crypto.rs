// Cryptocurrency functions (Bitcoin and USDC)
use super::{get_business_logic_canister_id, TransactionResult, ExchangeRate, SwapResult};
use ic_cdk::call::Call;

// ============================================================================
// Bitcoin Functions
// ============================================================================

/// Get Bitcoin rate
pub async fn get_bitcoin_rate(currency: &str) -> Result<ExchangeRate, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_bitcoin_rate: currency={}", currency);
    
    let response = Call::unbounded_wait(canister_id, "get_bitcoin_rate")
        .with_args(&(currency,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<ExchangeRate, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Sell Bitcoin
pub async fn sell_bitcoin(phone_number: &str, amount_sats: u64, currency: &str, pin: &str) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling sell_bitcoin: phone={}, amount={} sats", phone_number, amount_sats);
    
    let response = Call::unbounded_wait(canister_id, "sell_bitcoin")
        .with_args(&(phone_number, amount_sats, currency, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

// ============================================================================
// USDC Functions
// ============================================================================

/// Get USDC rate
pub async fn get_usdc_rate(currency: &str) -> Result<ExchangeRate, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_usdc_rate: currency={}", currency);
    
    let response = Call::unbounded_wait(canister_id, "get_usdc_rate")
        .with_args(&(currency,))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<ExchangeRate, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Sell USDC
pub async fn sell_usdc(phone_number: &str, amount_e6: u64, currency: &str, pin: &str) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling sell_usdc: phone={}, amount={} e6", phone_number, amount_e6);
    
    let response = Call::unbounded_wait(canister_id, "sell_usdc")
        .with_args(&(phone_number, amount_e6, currency, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Send USDC
pub async fn send_usdc(phone_number: &str, recipient: &str, amount_e6: u64, pin: &str) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling send_usdc: from={}, to={}, amount={} e6", phone_number, recipient, amount_e6);
    
    let response = Call::unbounded_wait(canister_id, "send_usdc")
        .with_args(&(phone_number, recipient, amount_e6, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Buy cryptocurrency (Bitcoin or USDC)
pub async fn buy_crypto(
    phone_number: &str,
    amount_cents: u64,
    currency: &str,
    crypto_type: super::CryptoType,
    pin: &str,
) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling buy_crypto: phone={}, amount={} cents, type={:?}", phone_number, amount_cents, crypto_type);
    
    let response = Call::unbounded_wait(canister_id, "buy_crypto")
        .with_args(&(phone_number, amount_cents, currency, crypto_type, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Send cryptocurrency (Bitcoin or USDC)
pub async fn send_crypto(
    phone_number: &str,
    recipient: &str,
    amount: u64,
    crypto_type: super::CryptoType,
    pin: &str,
) -> Result<TransactionResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling send_crypto: from={}, to={}, amount={}, type={:?}", phone_number, recipient, amount, crypto_type);
    
    let response = Call::unbounded_wait(canister_id, "send_crypto")
        .with_args(&(phone_number, recipient, amount, crypto_type, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<TransactionResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Swap between cryptocurrencies (BTC ↔ USDC)
pub async fn swap_crypto(
    phone_number: &str,
    from_crypto: &str,
    to_crypto: &str,
    amount: u64,
    pin: &str,
) -> Result<SwapResult, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    // Map string to CryptoType enum
    let from_type = match from_crypto {
        "CkBTC" => super::CryptoType::CkBTC,
        "CkUSDC" => super::CryptoType::CkUSDC,
        _ => return Err(format!("Invalid from_crypto: {}", from_crypto)),
    };
    
    let to_type = match to_crypto {
        "CkBTC" => super::CryptoType::CkBTC,
        "CkUSDC" => super::CryptoType::CkUSDC,
        _ => return Err(format!("Invalid to_crypto: {}", to_crypto)),
    };
    
    ic_cdk::println!("📤 Calling swap_crypto: phone={}, from={:?}, to={:?}, amount={}", 
        phone_number, from_type, to_type, amount);
    
    let response = Call::unbounded_wait(canister_id, "swap_crypto")
        .with_args(&(phone_number, from_type, to_type, amount, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<SwapResult, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
