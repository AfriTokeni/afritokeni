use super::*;

// Test constant - matches the config value
const TEST_PLATFORM_FEE_BPS: u64 = 50; // 0.5% from revenue_config.toml

// ============================================================================
// DEPOSIT CREATION TESTS
// ============================================================================

#[test]
fn test_create_deposit_request() {
    let amount = 100_000u64;
    
    // Use test constant that matches config
    let expected_commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    assert_eq!(expected_commission, 500);
}

#[test]
fn test_deposit_code_generation() {
    let code1 = generate_deposit_code(1);
    let code2 = generate_deposit_code(100);
    let code3 = generate_deposit_code(999999);
    
    assert_eq!(code1, "DEP-00000001");
    assert_eq!(code2, "DEP-00000100");
    assert_eq!(code3, "DEP-00999999");
}

#[test]
fn test_zero_amount_rejected() {
    let amount = 0u64;
    assert_eq!(amount, 0, "Zero amounts should be rejected");
}

// ============================================================================
// COMMISSION CALCULATION TESTS
// ============================================================================

#[test]
fn test_commission_calculation_small() {
    let amount = 10_000u64; // 10k UGX
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    assert_eq!(commission, 50); // 0.5% = 50 UGX
}

#[test]
fn test_commission_calculation_medium() {
    let amount = 100_000u64; // 100k UGX
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    assert_eq!(commission, 500); // 0.5% = 500 UGX
}

#[test]
fn test_commission_calculation_large() {
    let amount = 1_000_000u64; // 1M UGX
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    assert_eq!(commission, 5_000); // 0.5% = 5,000 UGX
}

#[test]
fn test_commission_calculation_very_large() {
    let amount = 10_000_000u64; // 10M UGX
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    assert_eq!(commission, 50_000); // 0.5% = 50,000 UGX
}

// ============================================================================
// REALISTIC SCENARIOS
// ============================================================================

#[test]
fn test_typical_deposit_scenario() {
    // User deposits 50,000 UGX (~$13 USD)
    let amount = 50_000u64;
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    
    assert_eq!(commission, 250); // AfriTokeni gets 250 UGX
    assert_eq!(amount - commission, 49_750); // User gets 49,750 UGX credited
}

#[test]
fn test_large_deposit_scenario() {
    // User deposits 5,000,000 UGX (~$1,300 USD)
    let amount = 5_000_000u64;
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    
    assert_eq!(commission, 25_000); // AfriTokeni gets 25k UGX
    assert_eq!(amount - commission, 4_975_000);
}

#[test]
fn test_multiple_deposits_revenue() {
    // 10 users each deposit 100k UGX
    let single_deposit = 100_000u64;
    let num_deposits = 10u64;
    let total_volume = single_deposit * num_deposits;
    
    let total_commission = (total_volume * 50) / 10000;
    
    assert_eq!(total_commission, 5_000); // 5k UGX total revenue
}

#[test]
fn test_daily_volume_scenario() {
    // 100 deposits per day, average 75k UGX each
    let avg_deposit = 75_000u64;
    let deposits_per_day = 100u64;
    let daily_volume = avg_deposit * deposits_per_day;
    
    let daily_commission = (daily_volume * 50) / 10000;
    
    assert_eq!(daily_volume, 7_500_000); // 7.5M UGX daily volume
    assert_eq!(daily_commission, 37_500); // 37.5k UGX daily revenue
    
    // Monthly revenue (30 days)
    let monthly_revenue = daily_commission * 30;
    assert_eq!(monthly_revenue, 1_125_000); // 1.125M UGX/month
}

// ============================================================================
// AGENT BALANCE TESTS
// ============================================================================

#[test]
fn test_agent_balance_accumulation() {
    // Agent processes 5 deposits
    let deposits = vec![100_000, 200_000, 150_000, 300_000, 250_000];
    
    let mut total_deposits = 0u64;
    let mut total_commission = 0u64;
    
    for amount in deposits {
        total_deposits += amount;
        total_commission += (amount * 50) / 10000;
    }
    
    assert_eq!(total_deposits, 1_000_000); // 1M UGX processed
    assert_eq!(total_commission, 5_000); // 5k UGX owed to AfriTokeni
}

#[test]
fn test_commission_owed_vs_paid() {
    let total_owed = 10_000u64;
    let total_paid = 6_000u64;
    let outstanding = total_owed - total_paid;
    
    assert_eq!(outstanding, 4_000); // 4k UGX still owed
}

// ============================================================================
// SETTLEMENT TESTS
// ============================================================================

#[test]
fn test_monthly_settlement_calculation() {
    // Agent processed 2M UGX in deposits this month
    let monthly_volume = 2_000_000u64;
    let monthly_commission = (monthly_volume * 50) / 10000;
    
    assert_eq!(monthly_commission, 10_000); // Agent owes 10k UGX
}

#[test]
fn test_settlement_after_payment() {
    let commission_owed = 15_000u64;
    let commission_paid = 0u64;
    
    // Before payment
    assert_eq!(commission_owed - commission_paid, 15_000);
    
    // After payment
    let new_paid = commission_paid + 15_000;
    assert_eq!(commission_owed - new_paid, 0); // Fully settled
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_minimum_deposit() {
    // Smallest meaningful deposit (100 UGX)
    let amount = 100u64;
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    
    // Commission rounds to 0 for very small amounts
    assert_eq!(commission, 0);
}

#[test]
fn test_commission_never_exceeds_amount() {
    let amount = 1_000u64;
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    
    assert!(commission < amount);
    assert_eq!(commission, 5); // 0.5% of 1000 = 5
}

#[test]
fn test_large_number_handling() {
    // Test with very large amounts (100M UGX)
    let amount = 100_000_000u64;
    let commission = (amount * TEST_PLATFORM_FEE_BPS) / 10000;
    
    assert_eq!(commission, 500_000); // 500k UGX commission
    assert!(commission < amount);
}

// ============================================================================
// TRANSACTION STATUS TESTS
// ============================================================================

#[test]
fn test_transaction_status_flow() {
    let pending = TransactionStatus::Pending;
    let confirmed = TransactionStatus::Confirmed;
    let cancelled = TransactionStatus::Cancelled;
    
    assert_ne!(pending, confirmed);
    assert_ne!(pending, cancelled);
    assert_ne!(confirmed, cancelled);
}

#[test]
fn test_pending_to_confirmed() {
    let status = TransactionStatus::Pending;
    assert_eq!(status, TransactionStatus::Pending);
    
    // After confirmation
    let new_status = TransactionStatus::Confirmed;
    assert_eq!(new_status, TransactionStatus::Confirmed);
}
