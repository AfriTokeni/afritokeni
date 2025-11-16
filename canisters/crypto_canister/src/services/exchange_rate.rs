// ============================================================================
// Exchange Rate Service - Multi-Currency Support
// ============================================================================
// 
// Integrates with:
// - CoinGecko API for BTC/USD and USDC/USD prices
// - ExchangeRate-API for fiat currency conversions
//
// Supports 39 African currencies:
// UGX, KES, TZS, RWF, BIF, NGN, GHS, XOF, GMD, SLL, LRD, ZAR, BWP, LSL, SZL,
// NAD, ZMW, MWK, EGP, MAD, TND, DZD, LYD, XAF, CDF, AOA, ETB, SOS, SDG, SSP,
// DJF, ERN, MUR, SCR, MGA, KMF, CVE, STN, MRU
// ============================================================================

use ic_cdk::call::Call;
use candid::{CandidType, Deserialize, Principal};
use crate::config;
use std::cell::RefCell;

// API rate limiting state
thread_local! {
    static API_CALL_TRACKER: RefCell<Vec<u64>> = RefCell::new(Vec::new());
}

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: HttpMethod,
    headers: Vec<HttpHeader>,
    body: Option<Vec<u8>>,
    max_response_bytes: Option<u64>,
}

#[derive(CandidType, Deserialize)]
struct HttpHeader {
    name: String,
    value: String,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
enum HttpMethod {
    GET,
}

#[derive(CandidType, Deserialize)]
struct HttpResponse {
    status: u64,
    headers: Vec<HttpHeader>,
    body: Vec<u8>,
}

/// Check API rate limit and enforce max calls per minute
fn check_api_rate_limit() -> Result<(), String> {
    let cfg = config::get_config();
    let max_calls = cfg.external_apis.max_api_calls_per_minute;

    API_CALL_TRACKER.with(|tracker| {
        let mut calls = tracker.borrow_mut();
        let now = ic_cdk::api::time();
        let minute_ago = now.saturating_sub(60_000_000_000); // 60 seconds in nanoseconds

        // Remove calls older than 1 minute
        calls.retain(|&t| t > minute_ago);

        // Check if we've exceeded the limit
        if calls.len() >= max_calls as usize {
            return Err(format!(
                "API rate limit exceeded. Max {} calls per minute. Try again later.",
                max_calls
            ));
        }

        // Record this call
        calls.push(now);
        Ok(())
    })
}

/// Get BTC price in USD from CoinGecko
pub async fn get_btc_usd_price() -> Result<f64, String> {
    // In test mode, return mock value (PocketIC can't make HTTP outcalls)
    if config::is_test_mode() {
        return Ok(50_000.0); // Mock: $50k per BTC for tests
    }

    // Check API rate limit before making the call
    check_api_rate_limit()?;

    let cfg = config::get_config();
    let timeout_seconds = cfg.external_apis.api_timeout_seconds;
    let cycles = 20_000_000_000u128; // 20 billion cycles for HTTP outcall

    let api_url = config::get_coingecko_api_url();
    let url = format!("{}?ids=bitcoin&vs_currencies=usd", api_url);

    let request = HttpRequest {
        url,
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        max_response_bytes: Some(1000),
    };

    let management_canister = Principal::management_canister();

    let response_bytes = Call::bounded_wait(management_canister, "http_request")
        .with_arg(request)
        .with_cycles(cycles)
        .change_timeout(timeout_seconds as u32)
        .await
        .map_err(|e| format!("HTTP request failed or timed out after {}s: {:?}", timeout_seconds, e))?;

    let (response,): (HttpResponse,) = candid::decode_args(&response_bytes.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    parse_coingecko_response(&response, "bitcoin")
}

/// Get USDC price in USD from CoinGecko
pub async fn get_usdc_usd_price() -> Result<f64, String> {
    // In test mode, return mock value (PocketIC can't make HTTP outcalls)
    if config::is_test_mode() {
        return Ok(1.0); // Mock: $1 per USDC for tests
    }

    // Check API rate limit before making the call
    check_api_rate_limit()?;

    let cfg = config::get_config();
    let timeout_seconds = cfg.external_apis.api_timeout_seconds;
    let cycles = 20_000_000_000u128; // 20 billion cycles for HTTP outcall

    let api_url = config::get_coingecko_api_url();
    let url = format!("{}?ids=usd-coin&vs_currencies=usd", api_url);

    let request = HttpRequest {
        url,
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        max_response_bytes: Some(1000),
    };

    let management_canister = Principal::management_canister();

    let response_bytes = Call::bounded_wait(management_canister, "http_request")
        .with_arg(request)
        .with_cycles(cycles)
        .change_timeout(timeout_seconds as u32)
        .await
        .map_err(|e| format!("HTTP request failed or timed out after {}s: {:?}", timeout_seconds, e))?;

    let (response,): (HttpResponse,) = candid::decode_args(&response_bytes.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    parse_coingecko_response(&response, "usd-coin")
}

/// Get fiat currency exchange rate to USD
pub async fn get_fiat_to_usd_rate(currency_code: &str) -> Result<f64, String> {
    // In test mode, return mock value (PocketIC can't make HTTP outcalls)
    if config::is_test_mode() {
        return get_mock_fiat_rate(currency_code);
    }

    // Check API rate limit before making the call
    check_api_rate_limit()?;

    let cfg = config::get_config();
    let timeout_seconds = cfg.external_apis.api_timeout_seconds;
    let cycles = 20_000_000_000u128; // 20 billion cycles for HTTP outcall

    let api_url = config::get_exchangerate_api_url();
    let request = HttpRequest {
        url: api_url,
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        max_response_bytes: Some(10000),
    };

    let management_canister = Principal::management_canister();

    let response_bytes = Call::bounded_wait(management_canister, "http_request")
        .with_arg(request)
        .with_cycles(cycles)
        .change_timeout(timeout_seconds as u32)
        .await
        .map_err(|e| format!("HTTP request failed or timed out after {}s: {:?}", timeout_seconds, e))?;

    let (response,): (HttpResponse,) = candid::decode_args(&response_bytes.into_bytes())
        .map_err(|e| format!("Failed to decode response: {:?}", e))?;

    parse_exchangerate_response(&response, currency_code)
}

/// Mock fiat rates for testing (ONLY used in test mode)
fn get_mock_fiat_rate(currency_code: &str) -> Result<f64, String> {
    let rate = match currency_code {
        "UGX" => 1.0 / 3700.0,   // 1 USD = 3700 UGX
        "KES" => 1.0 / 150.0,    // 1 USD = 150 KES
        "TZS" => 1.0 / 2500.0,   // 1 USD = 2500 TZS
        "RWF" => 1.0 / 1300.0,   // 1 USD = 1300 RWF
        "NGN" => 1.0 / 1500.0,   // 1 USD = 1500 NGN
        "GHS" => 1.0 / 12.0,     // 1 USD = 12 GHS
        "ZAR" => 1.0 / 18.0,     // 1 USD = 18 ZAR
        _ => return Err(format!("Unsupported currency for mock: {}", currency_code)),
    };
    Ok(rate)
}

/// Calculate crypto amount from fiat with real exchange rates
pub async fn calculate_crypto_from_fiat(
    fiat_amount: u64,
    fiat_currency: &str,
    crypto_type: &str,
) -> Result<u64, String> {
    // Get fiat to USD rate
    let fiat_to_usd = get_fiat_to_usd_rate(fiat_currency).await?;
    
    // Convert fiat to USD
    let usd_amount = (fiat_amount as f64) * fiat_to_usd;
    
    // Get crypto price in USD
    let crypto_usd_price = match crypto_type {
        "CkBTC" | "BTC" => get_btc_usd_price().await?,
        "CkUSD" | "USDC" => get_usdc_usd_price().await?,
        _ => return Err(format!("Unsupported crypto type: {}", crypto_type)),
    };
    
    // Calculate crypto amount
    let crypto_amount = usd_amount / crypto_usd_price;
    
    // Convert to smallest unit (satoshis for BTC, cents for USDC)
    let multiplier = match crypto_type {
        "CkBTC" | "BTC" => 100_000_000.0, // 8 decimals
        "CkUSD" | "USDC" => 1_000_000.0,  // 6 decimals
        _ => return Err("Invalid crypto type".to_string()),
    };
    
    Ok((crypto_amount * multiplier) as u64)
}

/// Calculate fiat amount from crypto with real exchange rates
pub async fn calculate_fiat_from_crypto(
    crypto_amount: u64,
    crypto_type: &str,
    fiat_currency: &str,
) -> Result<u64, String> {
    // Convert from smallest unit to whole units
    let divisor = match crypto_type {
        "CkBTC" | "BTC" => 100_000_000.0,
        "CkUSD" | "USDC" => 1_000_000.0,
        _ => return Err(format!("Unsupported crypto type: {}", crypto_type)),
    };
    
    let crypto_whole = (crypto_amount as f64) / divisor;
    
    // Get crypto price in USD
    let crypto_usd_price = match crypto_type {
        "CkBTC" | "BTC" => get_btc_usd_price().await?,
        "CkUSD" | "USDC" => get_usdc_usd_price().await?,
        _ => return Err("Invalid crypto type".to_string()),
    };
    
    // Calculate USD amount
    let usd_amount = crypto_whole * crypto_usd_price;
    
    // Get USD to fiat rate
    let usd_to_fiat = 1.0 / get_fiat_to_usd_rate(fiat_currency).await?;
    
    // Convert to fiat
    let fiat_amount = usd_amount * usd_to_fiat;
    
    Ok(fiat_amount as u64)
}

// ============================================================================
// HTTP Response Parsing
// ============================================================================

fn parse_coingecko_response(response: &HttpResponse, coin_id: &str) -> Result<f64, String> {
    let body = String::from_utf8(response.body.clone())
        .map_err(|_| "Invalid UTF-8 response".to_string())?;
    
    // Parse JSON manually (simple case)
    // Expected format: {"bitcoin":{"usd":45000.0}}
    let price_str = body
        .split(&format!("\"{}\":{{\"usd\":", coin_id))
        .nth(1)
        .and_then(|s| s.split('}').next())
        .ok_or("Failed to parse CoinGecko response")?;
    
    price_str.parse::<f64>()
        .map_err(|_| "Failed to parse price as number".to_string())
}

fn parse_exchangerate_response(response: &HttpResponse, currency_code: &str) -> Result<f64, String> {
    let body = String::from_utf8(response.body.clone())
        .map_err(|_| "Invalid UTF-8 response".to_string())?;
    
    // Parse JSON manually
    // Expected format: {"rates":{"UGX":3700.0,...}}
    let rate_str = body
        .split(&format!("\"{}\":", currency_code))
        .nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.split('}').next())
        .ok_or(format!("Currency {} not found in response", currency_code))?;
    
    rate_str.trim().parse::<f64>()
        .map_err(|_| "Failed to parse rate as number".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coingecko_response() {
        let response = HttpResponse {
            status: 200,
            headers: vec![],
            body: r#"{"bitcoin":{"usd":45000.5}}"#.as_bytes().to_vec(),
        };
        
        let price = parse_coingecko_response(&response, "bitcoin").unwrap();
        assert_eq!(price, 45000.5);
    }

    #[test]
    fn test_parse_exchangerate_response() {
        let response = HttpResponse {
            status: 200,
            headers: vec![],
            body: r#"{"rates":{"UGX":3700.5,"KES":150.2}}"#.as_bytes().to_vec(),
        };
        
        let rate = parse_exchangerate_response(&response, "UGX").unwrap();
        assert_eq!(rate, 3700.5);
        
        let rate = parse_exchangerate_response(&response, "KES").unwrap();
        assert_eq!(rate, 150.2);
    }
}
