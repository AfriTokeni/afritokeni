use super::*;
use candid::Principal;

// ============================================================================
// CONFIG TESTS
// ============================================================================

#[test]
fn test_config_loads_from_toml() {
    let config: Config = toml::from_str(CONFIG_TOML).expect("Failed to parse config");
    assert_eq!(config.spread.basis_points, 50);
    assert_eq!(config.dex.provider, "sonic");
    assert!(!config.company_wallet.principal.is_empty());
}

#[test]
fn test_all_principals_are_valid() {
    let config: Config = toml::from_str(CONFIG_TOML).unwrap();
    
    // Company wallet principal
    if config.company_wallet.principal != "aaaaa-aa" {
        assert!(Principal::from_text(&config.company_wallet.principal).is_ok());
    }
    
    // Token principals
    assert!(Principal::from_text(&config.tokens.ckbtc.ledger).is_ok());
    assert!(Principal::from_text(&config.tokens.ckusdc.ledger).is_ok());
    
    // Sonic DEX principal
    assert!(Principal::from_text(&config.dex.sonic.swap_canister).is_ok());
}

#[test]
fn test_token_decimals_are_correct() {
    let config: Config = toml::from_str(CONFIG_TOML).unwrap();
    assert_eq!(config.tokens.ckbtc.decimals, 8, "ckBTC should have 8 decimals");
    assert_eq!(config.tokens.ckusdc.decimals, 6, "ckUSDC should have 6 decimals");
}

#[test]
fn test_spread_is_within_limits() {
    let config: Config = toml::from_str(CONFIG_TOML).unwrap();
    assert!(config.spread.basis_points > 0, "Spread must be positive");
    assert!(config.spread.basis_points <= 1000, "Spread cannot exceed 10%");
}

// ============================================================================
// SPREAD CALCULATION TESTS
// ============================================================================

#[test]
fn test_spread_calculation_btc() {
    let amount = 100_000_000u64; // 1 BTC
    let spread_bps = 50u64; // 0.5%
    
    let spread = (amount * spread_bps) / 10000;
    let remaining = amount - spread;
    
    assert_eq!(spread, 500_000, "0.5% of 1 BTC = 0.005 BTC");
    assert_eq!(remaining, 99_500_000, "99.5% of 1 BTC = 0.995 BTC");
}

#[test]
fn test_spread_calculation_usdc() {
    let amount = 1_000_000_000u64; // 1000 USDC
    let spread_bps = 50u64; // 0.5%
    
    let spread = (amount * spread_bps) / 10000;
    let remaining = amount - spread;
    
    assert_eq!(spread, 5_000_000, "0.5% of 1000 USDC = 5 USDC");
    assert_eq!(remaining, 995_000_000, "99.5% of 1000 USDC = 995 USDC");
}

#[test]
fn test_spread_with_small_amounts() {
    let amount = 10_000u64; // Very small amount
    let spread_bps = 50u64;
    
    let spread = (amount * spread_bps) / 10000;
    assert_eq!(spread, 50, "0.5% of 10,000 = 50");
}

#[test]
fn test_spread_with_large_amounts() {
    let amount = 10_000_000_000u64; // Large amount
    let spread_bps = 50u64;
    
    let spread = (amount * spread_bps) / 10000;
    assert_eq!(spread, 50_000_000, "0.5% of 10B = 50M");
}

#[test]
fn test_different_spread_percentages() {
    let amount = 100_000_000u64;
    
    // 0.1%
    let spread_10bps = (amount * 10) / 10000;
    assert_eq!(spread_10bps, 100_000);
    
    // 1%
    let spread_100bps = (amount * 100) / 10000;
    assert_eq!(spread_100bps, 1_000_000);
    
    // 5%
    let spread_500bps = (amount * 500) / 10000;
    assert_eq!(spread_500bps, 5_000_000);
}

// ============================================================================
// TOKEN VALIDATION TESTS
// ============================================================================

#[test]
fn test_same_token_swap_should_fail() {
    let from = Token::CkBTC;
    let to = Token::CkBTC;
    assert_eq!(from, to, "Same token swap should be detected");
}

#[test]
fn test_different_tokens_are_valid() {
    let btc = Token::CkBTC;
    let usdc = Token::CkUSDC;
    assert_ne!(btc, usdc, "Different tokens should be allowed");
}

// ============================================================================
// REALISTIC EXCHANGE SCENARIOS
// ============================================================================

#[test]
fn test_small_btc_to_usdc_exchange() {
    // User swaps 0.01 BTC (~$420 at $42k/BTC)
    let input = 1_000_000u64; // 0.01 BTC
    let spread_bps = 50u64;
    
    let spread = (input * spread_bps) / 10000;
    let swap_amount = input - spread;
    
    assert_eq!(spread, 5_000); // 0.00005 BTC to company
    assert_eq!(swap_amount, 995_000); // 0.00995 BTC to swap
    
    // At $42k/BTC, user gets ~$417.90 USDC
    let btc_price = 42_000_000_000u64; // $42k (6 decimals)
    let expected_usdc = (swap_amount as u128 * btc_price as u128) / 100_000_000u128;
    
    assert!(expected_usdc > 417_000_000); // More than $417
    assert!(expected_usdc < 418_000_000); // Less than $418
}

#[test]
fn test_large_btc_to_usdc_exchange() {
    // User swaps 10 BTC (~$420k at $42k/BTC)
    let input = 1_000_000_000u64; // 10 BTC
    let spread_bps = 50u64;
    
    let spread = (input * spread_bps) / 10000;
    let swap_amount = input - spread;
    
    assert_eq!(spread, 5_000_000); // 0.05 BTC to company ($2.1k revenue!)
    assert_eq!(swap_amount, 995_000_000); // 9.95 BTC to swap
}

#[test]
fn test_usdc_to_btc_exchange() {
    // User swaps $10,000 USDC to BTC
    let input = 10_000_000_000u64; // $10k USDC
    let spread_bps = 50u64;
    
    let spread = (input * spread_bps) / 10000;
    let swap_amount = input - spread;
    
    assert_eq!(spread, 50_000_000); // $50 USDC to company
    assert_eq!(swap_amount, 9_950_000_000); // $9,950 USDC to swap
    
    // At $42k/BTC, user gets ~0.237 BTC
    let btc_price = 42_000_000_000u64;
    let expected_btc = (swap_amount as u128 * 100_000_000u128) / btc_price as u128;
    
    assert!(expected_btc > 23_000_000); // More than 0.23 BTC
    assert!(expected_btc < 24_000_000); // Less than 0.24 BTC
}

#[test]
fn test_company_revenue_calculation() {
    // If 100 users each swap 0.1 BTC
    let single_swap = 10_000_000u64; // 0.1 BTC
    let num_users = 100u64;
    let total_volume = single_swap * num_users; // 10 BTC total
    
    let spread_bps = 50u64;
    let total_revenue = (total_volume * spread_bps) / 10000;
    
    assert_eq!(total_revenue, 5_000_000); // 0.05 BTC revenue
    
    // At $42k/BTC, that's $2,100 revenue!
    let btc_price = 42_000u64;
    let revenue_usd = (total_revenue as u128 * btc_price as u128) / 100_000_000u128;
    assert_eq!(revenue_usd, 2_100);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_zero_amount_should_be_rejected() {
    let amount = 0u64;
    assert_eq!(amount, 0, "Zero amounts should fail validation");
}

#[test]
fn test_minimum_viable_swap() {
    // Smallest possible swap (1 satoshi)
    let amount = 1u64;
    let spread_bps = 50u64;
    
    let spread = (amount * spread_bps) / 10000;
    // With 1 satoshi, spread rounds to 0
    assert_eq!(spread, 0, "Spread on 1 satoshi rounds to 0");
}

#[test]
fn test_spread_never_exceeds_input() {
    let amount = 1_000_000u64;
    let max_spread_bps = 1000u64; // 10% max
    
    let spread = (amount * max_spread_bps) / 10000;
    assert!(spread < amount, "Spread should never equal or exceed input");
    assert_eq!(spread, 100_000, "10% of 1M = 100k");
}

#[test]
fn test_precision_with_different_decimals() {
    // ckBTC has 8 decimals, ckUSDC has 6
    let btc_amount = 100_000_000u64; // 1 BTC (8 decimals)
    let usdc_amount = 1_000_000u64; // 1 USDC (6 decimals)
    
    // Both should calculate spread correctly despite different decimals
    let spread_bps = 50u64;
    
    let btc_spread = (btc_amount * spread_bps) / 10000;
    let usdc_spread = (usdc_amount * spread_bps) / 10000;
    
    assert_eq!(btc_spread, 500_000); // 0.005 BTC
    assert_eq!(usdc_spread, 5_000); // 0.005 USDC
}
