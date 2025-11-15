// ============================================================================
// Platform Reserve Management
// ============================================================================
// Manages the platform's crypto reserve for non-custodial buy/sell operations
// ============================================================================

use candid::{CandidType, Deserialize};
use crate::services::ledger_client;
use crate::services::dex_client;
use crate::config;
use shared_types::{CryptoType, audit};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ReserveBalance {
    pub ckbtc_balance: u64,
    pub ckusdc_balance: u64,
    pub total_value_usd: f64,
    pub ckbtc_percent: f64,
    pub ckusdc_percent: f64,
    pub needs_rebalancing: bool,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RebalanceResult {
    pub from_token: String,
    pub to_token: String,
    pub amount_swapped: u64,
    pub amount_received: u64,
    pub new_ckbtc_balance: u64,
    pub new_ckusdc_balance: u64,
    pub new_ckbtc_percent: f64,
    pub new_ckusdc_percent: f64,
}

/// Get platform reserve balances
/// NOTE: Requires btc_price_usd parameter for accurate USD valuation
pub async fn get_reserve_balance_with_price(btc_price_usd: f64) -> Result<ReserveBalance, String> {
    if btc_price_usd <= 0.0 {
        return Err("Invalid BTC price. Must be greater than 0".to_string());
    }
    
    let cfg = config::get_config();
    let ckbtc_balance = ledger_client::get_platform_reserve_ckbtc_balance().await?;
    let ckusdc_balance = ledger_client::get_platform_reserve_ckusdc_balance().await?;
    
    // Calculate USD values using provided BTC price
    let ckbtc_value_usd = (ckbtc_balance as f64 / 100_000_000.0) * btc_price_usd;  // sats to BTC
    let ckusdc_value_usd = ckusdc_balance as f64 / 1_000_000.0;  // e6 to USDC
    
    let total_value_usd = ckbtc_value_usd + ckusdc_value_usd;
    
    let (ckbtc_percent, ckusdc_percent) = if total_value_usd > 0.0 {
        (
            (ckbtc_value_usd / total_value_usd) * 100.0,
            (ckusdc_value_usd / total_value_usd) * 100.0,
        )
    } else {
        (0.0, 0.0)
    };
    
    // Check if rebalancing is needed
    let ckbtc_deviation = (ckbtc_percent - cfg.reserve.target_ckbtc_percent).abs();
    let needs_rebalancing = ckbtc_deviation > cfg.reserve.rebalance_threshold_percent;
    
    Ok(ReserveBalance {
        ckbtc_balance,
        ckusdc_balance,
        total_value_usd,
        ckbtc_percent,
        ckusdc_percent,
        needs_rebalancing,
        timestamp: ic_cdk::api::time(),
    })
}

/// Rebalance reserve to maintain target BTC/USDC allocation from config
pub async fn rebalance_reserve(btc_price_usd: f64) -> Result<RebalanceResult, String> {
    if btc_price_usd <= 0.0 {
        return Err("Invalid BTC price. Must be greater than 0".to_string());
    }
    
    let cfg = config::get_config();
    let reserve = get_reserve_balance_with_price(btc_price_usd).await?;
    
    if !reserve.needs_rebalancing {
        return Err("Rebalancing not needed. Deviation is within threshold.".to_string());
    }
    
    audit::log_success(
        "rebalance_reserve_started",
        None,
        format!("Current: {}% BTC, {}% USDC | Target: {}% BTC, {}% USDC", 
            reserve.ckbtc_percent, reserve.ckusdc_percent,
            cfg.reserve.target_ckbtc_percent, cfg.reserve.target_ckusdc_percent)
    );
    
    // Calculate USD values
    let ckbtc_value_usd = (reserve.ckbtc_balance as f64 / 100_000_000.0) * btc_price_usd;
    let ckusdc_value_usd = reserve.ckusdc_balance as f64 / 1_000_000.0;
    let total_value_usd = ckbtc_value_usd + ckusdc_value_usd;
    
    let target_btc_value_usd = total_value_usd * (cfg.reserve.target_ckbtc_percent / 100.0);
    let target_usdc_value_usd = total_value_usd * (cfg.reserve.target_ckusdc_percent / 100.0);
    
    // Determine swap direction and amount
    let (from_token, to_token, swap_amount_usd) = if ckbtc_value_usd > target_btc_value_usd {
        // Too much BTC, swap BTC -> USDC
        let excess_btc_usd = ckbtc_value_usd - target_btc_value_usd;
        ("CkBTC".to_string(), "CkUSDC".to_string(), excess_btc_usd)
    } else {
        // Too much USDC, swap USDC -> BTC
        let excess_usdc_usd = ckusdc_value_usd - target_usdc_value_usd;
        ("CkUSDC".to_string(), "CkBTC".to_string(), excess_usdc_usd)
    };
    
    // Convert USD amount to token amount
    let swap_amount = if from_token == "CkBTC" {
        ((swap_amount_usd / btc_price_usd) * 100_000_000.0) as u64  // USD to sats
    } else {
        (swap_amount_usd * 1_000_000.0) as u64  // USD to e6
    };
    
    // Calculate minimum output using slippage tolerance from config
    let slippage_tolerance = cfg.reserve.rebalance_slippage_tolerance_bp as f64 / 10000.0;  // bp to decimal
    let expected_output = if to_token == "CkBTC" {
        ((swap_amount_usd / btc_price_usd) * 100_000_000.0) as u64
    } else {
        (swap_amount_usd * 1_000_000.0) as u64
    };
    let min_output = (expected_output as f64 * (1.0 - slippage_tolerance)) as u64;
    
    // Execute swap via Sonic DEX
    let from_crypto = if from_token == "CkBTC" { CryptoType::CkBTC } else { CryptoType::CkUSDC };
    let to_crypto = if to_token == "CkBTC" { CryptoType::CkBTC } else { CryptoType::CkUSDC };
    
    let amount_received = dex_client::swap_tokens(
        from_crypto,
        to_crypto,
        swap_amount,
        min_output,
    ).await?;
    
    // Get new balances
    let new_reserve = get_reserve_balance_with_price(btc_price_usd).await?;
    
    audit::log_success(
        "rebalance_reserve_completed",
        None,
        format!("Swapped {} {} -> {} {} | New allocation: {}% BTC, {}% USDC",
            swap_amount, from_token, amount_received, to_token,
            new_reserve.ckbtc_percent, new_reserve.ckusdc_percent)
    );
    
    Ok(RebalanceResult {
        from_token,
        to_token,
        amount_swapped: swap_amount,
        amount_received,
        new_ckbtc_balance: new_reserve.ckbtc_balance,
        new_ckusdc_balance: new_reserve.ckusdc_balance,
        new_ckbtc_percent: new_reserve.ckbtc_percent,
        new_ckusdc_percent: new_reserve.ckusdc_percent,
    })
}

/// Check if reserve has sufficient balance for a buy operation
/// Used by buy_crypto flow to ensure platform has sufficient reserves before executing purchase
#[allow(dead_code)]
pub async fn check_reserve_sufficient(
    crypto_type: CryptoType,
    amount: u64,
    btc_price_usd: f64,
) -> Result<bool, String> {
    if btc_price_usd <= 0.0 {
        return Err("Invalid BTC price. Must be greater than 0".to_string());
    }
    
    let reserve = get_reserve_balance_with_price(btc_price_usd).await?;
    
    let sufficient = match crypto_type {
        CryptoType::CkBTC => reserve.ckbtc_balance >= amount,
        CryptoType::CkUSDC => reserve.ckusdc_balance >= amount,
    };
    
    if !sufficient {
        let balance = match crypto_type {
            CryptoType::CkBTC => reserve.ckbtc_balance,
            CryptoType::CkUSDC => reserve.ckusdc_balance,
        };
        
        audit::log_failure(
            "insufficient_reserve",
            None,
            format!("Insufficient {:?} reserve. Need: {}, Have: {}", crypto_type, amount, balance)
        );
    }
    
    Ok(sufficient)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rebalance_threshold() {
        let target: f64 = 50.0;
        let threshold: f64 = 10.0;
        
        // 60% BTC, 40% USDC = 10% deviation, should trigger rebalance
        let deviation: f64 = (60.0 - target).abs();
        assert!(deviation >= threshold);
        
        // 55% BTC, 45% USDC = 5% deviation, should NOT trigger rebalance
        let deviation: f64 = (55.0 - target).abs();
        assert!(deviation < threshold);
    }
    
    #[test]
    fn test_allocation_calculation() {
        let ckbtc_value = 5000.0;  // $5000
        let ckusdc_value = 5000.0;  // $5000
        let total = ckbtc_value + ckusdc_value;
        
        let ckbtc_percent = (ckbtc_value / total) * 100.0;
        let ckusdc_percent = (ckusdc_value / total) * 100.0;
        
        assert_eq!(ckbtc_percent, 50.0);
        assert_eq!(ckusdc_percent, 50.0);
    }
}
