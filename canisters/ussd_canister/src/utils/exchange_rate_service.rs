use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse, TransformContext,
};
use serde::{Deserialize, Serialize};

// ============================================================================
// Exchange Rate Service - Uses HTTP Outcalls to External APIs
// ============================================================================

use std::collections::HashMap;

/// Exchange rates for supported currencies
#[derive(Debug, Clone)]
pub struct ExchangeRates {
    pub btc_to_usd: f64,
    pub usdc_to_usd: f64,  // Always 1.0 (stablecoin)
    pub fiat_to_usd: HashMap<String, f64>,  // All fiat currencies to USD
}

/// CoinGecko API response for BTC price
#[derive(Deserialize, Serialize)]
struct CoinGeckoResponse {
    bitcoin: CoinGeckoPrices,
}

#[derive(Deserialize, Serialize)]
struct CoinGeckoPrices {
    usd: f64,
}

/// ExchangeRate-API response for fiat currencies
#[derive(Deserialize, Serialize)]
struct ExchangeRateApiResponse {
    conversion_rates: HashMap<String, f64>,
}

/// Get current exchange rates from external APIs
pub async fn get_exchange_rates() -> Result<ExchangeRates, String> {
    // Get BTC/USD rate from CoinGecko (free, no API key needed)
    let btc_to_usd = get_btc_price().await?;
    
    // USDC is pegged 1:1 to USD
    let usdc_to_usd = 1.0;
    
    // Get all African fiat currency rates from ExchangeRate-API
    let fiat_to_usd = get_fiat_rates().await?;
    
    Ok(ExchangeRates {
        btc_to_usd,
        usdc_to_usd,
        fiat_to_usd,
    })
}

/// Get BTC price in USD from CoinGecko
async fn get_btc_price() -> Result<f64, String> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: Some(1000),
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        transform: Some(TransformContext::from_name(
            "transform_http_response".to_string(),
            vec![],
        )),
    };
    
    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            let body = String::from_utf8(response.body)
                .map_err(|_| "Failed to parse response body")?;
            
            let data: CoinGeckoResponse = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;
            
            Ok(data.bitcoin.usd)
        }
        Err((code, msg)) => {
            Err(format!("HTTP request failed: {:?} - {}", code, msg))
        }
    }
}

/// Get fiat currency rates from ExchangeRate-API (free tier, no API key)
async fn get_fiat_rates() -> Result<HashMap<String, f64>, String> {
    // Using ExchangeRate-API free tier - base currency USD
    let url = "https://api.exchangerate-api.com/v4/latest/USD";
    
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: Some(10000),
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        transform: Some(TransformContext::from_name(
            "transform_http_response".to_string(),
            vec![],
        )),
    };
    
    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            let body = String::from_utf8(response.body)
                .map_err(|_| "Failed to parse response body")?;
            
            let data: ExchangeRateApiResponse = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;
            
            // Convert to USD base (API gives us rates FROM USD, we need TO USD)
            let mut rates = HashMap::new();
            for (currency, rate) in data.conversion_rates {
                rates.insert(currency, 1.0 / rate);
            }
            
            Ok(rates)
        }
        Err((code, msg)) => {
            Err(format!("HTTP request failed: {:?} - {}", code, msg))
        }
    }
}

/// Transform function for HTTP outcall (required by IC)
#[ic_cdk::query(hidden = true)]
fn transform_http_response(raw: ic_cdk::api::management_canister::http_request::TransformArgs) -> HttpResponse {
    HttpResponse {
        status: raw.response.status,
        headers: vec![],
        body: raw.response.body,
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

impl ExchangeRates {
    /// Convert fiat to BTC (works for any supported currency)
    pub fn fiat_to_btc(&self, fiat_amount: f64, currency_code: &str) -> Result<f64, String> {
        let fiat_to_usd_rate = self.fiat_to_usd.get(currency_code)
            .ok_or(format!("Currency {} not supported", currency_code))?;
        
        let usd_amount = fiat_amount * fiat_to_usd_rate;
        Ok(usd_amount / self.btc_to_usd)
    }
    
    /// Convert BTC to fiat (works for any supported currency)
    pub fn btc_to_fiat(&self, btc_amount: f64, currency_code: &str) -> Result<f64, String> {
        let fiat_to_usd_rate = self.fiat_to_usd.get(currency_code)
            .ok_or(format!("Currency {} not supported", currency_code))?;
        
        let usd_amount = btc_amount * self.btc_to_usd;
        Ok(usd_amount / fiat_to_usd_rate)
    }
    
    /// Convert fiat to USDC (works for any supported currency)
    pub fn fiat_to_usdc(&self, fiat_amount: f64, currency_code: &str) -> Result<f64, String> {
        let fiat_to_usd_rate = self.fiat_to_usd.get(currency_code)
            .ok_or(format!("Currency {} not supported", currency_code))?;
        
        let usd_amount = fiat_amount * fiat_to_usd_rate;
        Ok(usd_amount / self.usdc_to_usd)
    }
    
    /// Convert USDC to fiat (works for any supported currency)
    pub fn usdc_to_fiat(&self, usdc_amount: f64, currency_code: &str) -> Result<f64, String> {
        let fiat_to_usd_rate = self.fiat_to_usd.get(currency_code)
            .ok_or(format!("Currency {} not supported", currency_code))?;
        
        let usd_amount = usdc_amount * self.usdc_to_usd;
        Ok(usd_amount / fiat_to_usd_rate)
    }
    
    /// Get BTC price in any fiat currency
    pub fn btc_price_in_fiat(&self, currency_code: &str) -> Result<f64, String> {
        self.btc_to_fiat(1.0, currency_code)
    }
    
    /// Get USDC price in any fiat currency
    pub fn usdc_price_in_fiat(&self, currency_code: &str) -> Result<f64, String> {
        self.usdc_to_fiat(1.0, currency_code)
    }
    
    /// Convert between two fiat currencies
    pub fn convert_fiat(&self, amount: f64, from_currency: &str, to_currency: &str) -> Result<f64, String> {
        let from_rate = self.fiat_to_usd.get(from_currency)
            .ok_or(format!("Currency {} not supported", from_currency))?;
        let to_rate = self.fiat_to_usd.get(to_currency)
            .ok_or(format!("Currency {} not supported", to_currency))?;
        
        let usd_amount = amount * from_rate;
        Ok(usd_amount / to_rate)
    }
}

// ============================================================================
// Cached Rates (to avoid too many HTTP calls)
// ============================================================================

use std::cell::RefCell;

thread_local! {
    static CACHED_RATES: RefCell<Option<(ExchangeRates, u64)>> = RefCell::new(None);
}

const CACHE_DURATION_SECONDS: u64 = 60; // Cache for 1 minute

/// Get exchange rates with caching
pub async fn get_cached_rates() -> Result<ExchangeRates, String> {
    let now = ic_cdk::api::time() / 1_000_000_000;
    
    // Check cache
    let cached = CACHED_RATES.with(|cache| {
        cache.borrow().clone()
    });
    
    if let Some((rates, timestamp)) = cached {
        if now - timestamp < CACHE_DURATION_SECONDS {
            return Ok(rates);
        }
    }
    
    // Fetch new rates
    let rates = get_exchange_rates().await?;
    
    // Update cache
    CACHED_RATES.with(|cache| {
        *cache.borrow_mut() = Some((rates.clone(), now));
    });
    
    Ok(rates)
}

/// Get rate for a specific currency (query endpoint)
#[ic_cdk::query(hidden = true)]
fn get_currency_rate(currency_code: String) -> Result<f64, String> {
    CACHED_RATES.with(|cache| {
        cache.borrow().as_ref()
            .and_then(|(rates, _)| rates.fiat_to_usd.get(&currency_code).copied())
            .ok_or(format!("No rate available for {}", currency_code))
    })
}
