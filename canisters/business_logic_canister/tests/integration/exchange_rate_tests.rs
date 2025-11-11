use crate::integration::TestEnv;
use shared_types::*;

// ============================================================================
// EXCHANGE RATE TESTS
// ============================================================================
// These tests verify multi-currency exchange rate calculations for:
// - 39 African currencies
// - BTC and USDC crypto conversions
// - Real-time rate integration (with mock fallbacks for testing)
// ============================================================================

#[test]
fn test_btc_to_ugx_conversion() {
    let env = TestEnv::new();
    
    // Mock rates: 1 BTC = $50,000, 1 USD = 3700 UGX
    // So 1 BTC = 185,000,000 UGX
    // 0.001 BTC (100,000 satoshis) = 185,000 UGX
    
    let btc_amount = 100_000; // 0.001 BTC in satoshis
    let result = env.get_crypto_value_estimate(btc_amount, CryptoType::CkBTC, "UGX");
    
    assert!(result.is_ok(), "BTC to UGX conversion should succeed");
    let ugx_value = result.unwrap();
    
    // Should be approximately 185,000 UGX (allow 10% variance for rounding)
    assert!(ugx_value > 166_500 && ugx_value < 203_500, 
        "Expected ~185,000 UGX, got {}", ugx_value);
}

#[test]
fn test_usdc_to_kes_conversion() {
    let env = TestEnv::new();
    
    // Mock rates: 1 USDC = $1, 1 USD = 150 KES
    // So 1 USDC = 150 KES
    // 100 USDC (100,000,000 in smallest units) = 15,000 KES
    
    let usdc_amount = 100_000_000; // 100 USDC
    let result = env.get_crypto_value_estimate(usdc_amount, CryptoType::CkUSDC, "KES");
    
    assert!(result.is_ok(), "USDC to KES conversion should succeed");
    let kes_value = result.unwrap();
    
    // Should be approximately 15,000 KES
    assert!(kes_value > 13_500 && kes_value < 16_500,
        "Expected ~15,000 KES, got {}", kes_value);
}

#[test]
fn test_btc_to_ngn_conversion() {
    let env = TestEnv::new();
    
    // Mock rates: 1 BTC = $50,000, 1 USD = 1500 NGN
    // So 1 BTC = 75,000,000 NGN
    // 0.01 BTC (1,000,000 satoshis) = 750,000 NGN
    
    let btc_amount = 1_000_000; // 0.01 BTC
    let result = env.get_crypto_value_estimate(btc_amount, CryptoType::CkBTC, "NGN");
    
    assert!(result.is_ok(), "BTC to NGN conversion should succeed");
    let ngn_value = result.unwrap();
    
    // Should be approximately 750,000 NGN
    assert!(ngn_value > 675_000 && ngn_value < 825_000,
        "Expected ~750,000 NGN, got {}", ngn_value);
}

#[test]
fn test_usdc_to_tzs_conversion() {
    let env = TestEnv::new();
    
    // Mock rates: 1 USDC = $1, 1 USD = 2500 TZS
    // So 1 USDC = 2500 TZS
    // 50 USDC = 125,000 TZS
    
    let usdc_amount = 50_000_000; // 50 USDC
    let result = env.get_crypto_value_estimate(usdc_amount, CryptoType::CkUSDC, "TZS");
    
    assert!(result.is_ok(), "USDC to TZS conversion should succeed");
    let tzs_value = result.unwrap();
    
    // Should be approximately 125,000 TZS
    assert!(tzs_value > 112_500 && tzs_value < 137_500,
        "Expected ~125,000 TZS, got {}", tzs_value);
}

#[test]
fn test_small_btc_amount_conversion() {
    let env = TestEnv::new();
    
    // Test very small amount: 1000 satoshis (0.00001 BTC)
    // At $50,000/BTC: 0.00001 BTC = $0.50 = 1850 UGX
    
    let btc_amount = 1_000; // 0.00001 BTC
    let result = env.get_crypto_value_estimate(btc_amount, CryptoType::CkBTC, "UGX");
    
    assert!(result.is_ok(), "Small BTC amount conversion should succeed");
    let ugx_value = result.unwrap();
    
    // Should be approximately 1850 UGX
    assert!(ugx_value > 1_665 && ugx_value < 2_035,
        "Expected ~1850 UGX, got {}", ugx_value);
}

#[test]
fn test_large_usdc_amount_conversion() {
    let env = TestEnv::new();
    
    // Test large amount: 10,000 USDC
    // At 1 USD = 150 KES: 10,000 USDC = 1,500,000 KES
    
    let usdc_amount = 10_000_000_000; // 10,000 USDC
    let result = env.get_crypto_value_estimate(usdc_amount, CryptoType::CkUSDC, "KES");
    
    assert!(result.is_ok(), "Large USDC amount conversion should succeed");
    let kes_value = result.unwrap();
    
    // Should be approximately 1,500,000 KES
    assert!(kes_value > 1_350_000 && kes_value < 1_650_000,
        "Expected ~1,500,000 KES, got {}", kes_value);
}

#[test]
fn test_multiple_currency_conversions() {
    let env = TestEnv::new();
    
    // Test same crypto amount to different currencies
    let btc_amount = 500_000; // 0.005 BTC
    
    // UGX conversion
    let ugx_result = env.get_crypto_value_estimate(btc_amount, CryptoType::CkBTC, "UGX");
    assert!(ugx_result.is_ok(), "UGX conversion should succeed");
    
    // KES conversion
    let kes_result = env.get_crypto_value_estimate(btc_amount, CryptoType::CkBTC, "KES");
    assert!(kes_result.is_ok(), "KES conversion should succeed");
    
    // NGN conversion
    let ngn_result = env.get_crypto_value_estimate(btc_amount, CryptoType::CkBTC, "NGN");
    assert!(ngn_result.is_ok(), "NGN conversion should succeed");
    
    // All should have different values due to different exchange rates
    let ugx_val = ugx_result.unwrap();
    let kes_val = kes_result.unwrap();
    let ngn_val = ngn_result.unwrap();
    
    assert_ne!(ugx_val, kes_val, "UGX and KES values should differ");
    assert_ne!(kes_val, ngn_val, "KES and NGN values should differ");
    assert_ne!(ugx_val, ngn_val, "UGX and NGN values should differ");
}

#[test]
fn test_zero_amount_conversion() {
    let env = TestEnv::new();
    
    // Zero crypto should convert to zero fiat
    let result = env.get_crypto_value_estimate(0, CryptoType::CkBTC, "UGX");
    
    assert!(result.is_ok(), "Zero amount conversion should succeed");
    assert_eq!(result.unwrap(), 0, "Zero crypto should equal zero fiat");
}

#[test]
fn test_unsupported_currency_handling() {
    let env = TestEnv::new();
    
    // Try to convert to an unsupported currency
    let result = env.get_crypto_value_estimate(100_000, CryptoType::CkBTC, "XXX");
    
    // Should fail gracefully with error message
    assert!(result.is_err(), "Unsupported currency should return error");
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("Unsupported") || err_msg.contains("not found"),
        "Error should mention unsupported currency, got: {}", err_msg);
}

// ============================================================================
// INTEGRATION WITH BUY_CRYPTO TESTS
// ============================================================================
// These tests verify that exchange rates work correctly in actual buy_crypto flows

#[test]
fn test_buy_crypto_uses_exchange_rates() {
    let env = TestEnv::new();
    
    // Register user with principal ID (required for crypto operations)
    let user_id = env.register_user(
        Some("+254700000001".to_string()),
        Some("aaaaa-aa".to_string()), // Add principal ID
        "Alice",
        "User",
        "alice@test.com",
        "UGX",
        "1234",
    ).unwrap();
    
    // Give user fiat balance
    env.set_fiat_balance(&user_id, "UGX", 1_000_000).unwrap();
    
    // Buy BTC with UGX
    // 1,000,000 UGX / 3700 = ~270 USD
    // 270 USD / 50,000 = ~0.0054 BTC = ~540,000 satoshis
    let result = env.buy_crypto(
        "+254700000001",
        1_000_000,
        "UGX",
        CryptoType::CkBTC,
        "1234",
    );
    
    // Should fail at ledger level in test environment (no real ledgers)
    // But the exchange rate calculation should have happened
    assert!(result.is_err(), "Should fail at ledger level in test env");
    let err = result.unwrap_err();
    
    // Should fail at ledger transfer, not at validation or calculation
    assert!(err.contains("ledger") || err.contains("transfer") || err.contains("Call failed"),
        "Should fail at ledger level, not calculation: {}", err);
}
