use super::*;
use candid::{encode_args, decode_one, Principal};

// ============================================================================
// EXCHANGE SPREAD TESTS
// ============================================================================

#[test]
fn test_exchange_canister_deployed() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    // Test basic query to verify canister is responsive
    let response = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_spread_percentage",
        vec![],
    );
    
    match response {
        Ok(_) => println!("✅ Exchange canister responds to queries"),
        Err(e) => panic!("❌ Exchange canister query failed: {:?}", e),
    }
}

#[test]
fn test_get_spread_percentage() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    println!("Testing exchange canister configuration...");
    let query_result = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_spread_percentage",
        vec![],
    );
    
    match query_result {
        Ok(response) => {
            let spread: u64 = decode_one(&response).expect("Failed to decode spread percentage");
            println!("✅ Spread percentage: {} basis points ({}%)", spread, spread as f64 / 100.0);
            assert_eq!(spread, 50, "Spread should be 50 basis points (0.5%)");
        }
        Err(e) => {
            panic!("❌ Failed to query spread percentage: {:?}", e);
        }
    }
}

#[test]
fn test_get_company_wallet() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    let query_result = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_company_wallet",
        vec![],
    );
    
    match query_result {
        Ok(response) => {
            let wallet: String = decode_one(&response).expect("Failed to decode company wallet");
            println!("✅ Company wallet: {}", wallet);
            
            // Verify it's a valid principal
            let principal = Principal::from_text(&wallet);
            assert!(principal.is_ok(), "Company wallet should be a valid principal");
            
            // Should match the configured company wallet from exchange_config.toml
            // Note: exchange_config.toml has a placeholder value "aaaaa-aa"
            assert_eq!(wallet, "aaaaa-aa", "Should match exchange_config.toml placeholder");
        }
        Err(e) => {
            panic!("❌ Failed to query company wallet: {:?}", e);
        }
    }
}

#[test]
fn test_get_dex_provider() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    let query_result = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_dex_provider",
        vec![],
    );
    
    match query_result {
        Ok(response) => {
            let provider: String = decode_one(&response).expect("Failed to decode DEX provider");
            println!("✅ DEX provider: {}", provider);
            assert_eq!(provider, "sonic", "DEX provider should be Sonic");
        }
        Err(e) => {
            panic!("❌ Failed to query DEX provider: {:?}", e);
        }
    }
}

#[test]
fn test_get_sonic_canister() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    let query_result = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_sonic_canister",
        vec![],
    );
    
    match query_result {
        Ok(response) => {
            let sonic_canister: String = decode_one(&response).expect("Failed to decode Sonic canister");
            println!("✅ Sonic canister: {}", sonic_canister);
            
            // Verify it's a valid principal
            let principal = Principal::from_text(&sonic_canister);
            assert!(principal.is_ok(), "Sonic canister should be a valid principal");
        }
        Err(e) => {
            panic!("❌ Failed to query Sonic canister: {:?}", e);
        }
    }
}

#[test]
fn test_spread_calculation_verification() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    // Get spread percentage
    let spread_response = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_spread_percentage",
        vec![],
    ).expect("Should get spread percentage");
    
    let spread_bp: u64 = decode_one(&spread_response).expect("Failed to decode spread");
    
    // Test spread calculation for various amounts
    let test_cases = vec![
        (100_000, 500),      // 0.5% of 100,000 = 500
        (1_000_000, 5_000),  // 0.5% of 1,000,000 = 5,000
        (50_000, 250),       // 0.5% of 50,000 = 250
        (200_000, 1_000),    // 0.5% of 200,000 = 1,000
    ];
    
    for (amount, expected_spread) in test_cases {
        let calculated_spread = (amount * spread_bp) / 10_000;
        assert_eq!(
            calculated_spread, 
            expected_spread,
            "Spread for {} should be {}",
            amount,
            expected_spread
        );
        println!("✅ Spread calculation verified: {} UGX → {} UGX spread", amount, calculated_spread);
    }
}

#[test]
fn test_exchange_configuration_consistency() {
    let env = TestEnv::new_with_commission_canisters();
    let exchange_canister_id = env.exchange_canister_id.unwrap();
    
    // Verify all configuration values are consistent
    let spread_response = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_spread_percentage",
        vec![],
    ).expect("Should get spread percentage");
    let spread: u64 = decode_one(&spread_response).unwrap();
    
    let wallet_response = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_company_wallet",
        vec![],
    ).expect("Should get company wallet");
    let wallet: String = decode_one(&wallet_response).unwrap();
    
    let provider_response = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_dex_provider",
        vec![],
    ).expect("Should get DEX provider");
    let provider: String = decode_one(&provider_response).unwrap();
    
    let sonic_response = env.pic.query_call(
        exchange_canister_id,
        Principal::anonymous(),
        "get_sonic_canister",
        vec![],
    ).expect("Should get Sonic canister");
    let sonic: String = decode_one(&sonic_response).unwrap();
    
    // Verify all values are set
    assert_eq!(spread, 50, "Spread should be 0.5%");
    assert!(!wallet.is_empty(), "Company wallet should be set");
    assert_eq!(provider, "sonic", "Provider should be Sonic");
    assert!(!sonic.is_empty(), "Sonic canister should be set");
    
    println!("✅ Exchange configuration is consistent:");
    println!("   - Spread: {} basis points", spread);
    println!("   - Company wallet: {}", wallet);
    println!("   - DEX provider: {}", provider);
    println!("   - Sonic canister: {}", sonic);
}
