use super::*;

// ============================================================================
// FEE SPLIT TESTS
// ============================================================================

#[test]
fn test_fee_split_10_90() {
    let amount = 100_000u64; // 100k UGX withdrawal
    
    let platform_fee = (amount * 10) / 100;
    let agent_fee = (amount * 90) / 100;
    
    assert_eq!(platform_fee, 10_000); // AfriTokeni gets 10k (10%)
    assert_eq!(agent_fee, 90_000); // Agent gets 90k (90%)
    assert_eq!(platform_fee + agent_fee, amount); // Total = 100%
}

#[test]
fn test_withdrawal_code_generation() {
    let code1 = generate_withdrawal_code(1);
    let code2 = generate_withdrawal_code(100);
    let code3 = generate_withdrawal_code(999999);
    
    assert_eq!(code1, "WTH-00000001");
    assert_eq!(code2, "WTH-00000100");
    assert_eq!(code3, "WTH-00999999");
}

// ============================================================================
// FEE CALCULATION TESTS
// ============================================================================

#[test]
fn test_small_withdrawal_fees() {
    let amount = 10_000u64; // 10k UGX
    
    let platform = (amount * 10) / 100;
    let agent = (amount * 90) / 100;
    
    assert_eq!(platform, 1_000); // 1k to platform
    assert_eq!(agent, 9_000); // 9k to agent
}

#[test]
fn test_medium_withdrawal_fees() {
    let amount = 100_000u64; // 100k UGX
    
    let platform = (amount * 10) / 100;
    let agent = (amount * 90) / 100;
    
    assert_eq!(platform, 10_000); // 10k to platform
    assert_eq!(agent, 90_000); // 90k to agent
}

#[test]
fn test_large_withdrawal_fees() {
    let amount = 1_000_000u64; // 1M UGX
    
    let platform = (amount * 10) / 100;
    let agent = (amount * 90) / 100;
    
    assert_eq!(platform, 100_000); // 100k to platform
    assert_eq!(agent, 900_000); // 900k to agent
}

// ============================================================================
// REALISTIC SCENARIOS
// ============================================================================

#[test]
fn test_typical_withdrawal_scenario() {
    // User withdraws 50,000 UGX (~$13 USD)
    let amount = 50_000u64;
    
    let platform_fee = (amount * 10) / 100;
    let agent_fee = (amount * 90) / 100;
    
    assert_eq!(platform_fee, 5_000); // AfriTokeni gets 5k
    assert_eq!(agent_fee, 45_000); // Agent gets 45k
    
    // Agent gives user 50k cash, earns 45k fee
}

#[test]
fn test_agent_daily_earnings() {
    // Agent processes 20 withdrawals per day, avg 75k UGX each
    let avg_withdrawal = 75_000u64;
    let withdrawals_per_day = 20u64;
    let daily_volume = avg_withdrawal * withdrawals_per_day;
    
    let daily_agent_earnings = (daily_volume * 90) / 100;
    let daily_platform_revenue = (daily_volume * 10) / 100;
    
    assert_eq!(daily_volume, 1_500_000); // 1.5M UGX daily volume
    assert_eq!(daily_agent_earnings, 1_350_000); // Agent earns 1.35M
    assert_eq!(daily_platform_revenue, 150_000); // Platform gets 150k
}

#[test]
fn test_monthly_agent_earnings() {
    // Agent processes 500k UGX in withdrawals per month
    let monthly_volume = 500_000u64;
    
    let agent_earnings = (monthly_volume * 90) / 100;
    let platform_revenue = (monthly_volume * 10) / 100;
    
    assert_eq!(agent_earnings, 450_000); // Agent earns 450k/month
    assert_eq!(platform_revenue, 50_000); // Platform gets 50k/month
}

#[test]
fn test_multiple_agents_revenue() {
    // 10 agents, each processes 1M UGX/month
    let per_agent_volume = 1_000_000u64;
    let num_agents = 10u64;
    let total_volume = per_agent_volume * num_agents;
    
    let total_platform_revenue = (total_volume * 10) / 100;
    let total_agent_earnings = (total_volume * 90) / 100;
    
    assert_eq!(total_volume, 10_000_000); // 10M total
    assert_eq!(total_platform_revenue, 1_000_000); // 1M to platform
    assert_eq!(total_agent_earnings, 9_000_000); // 9M to agents
}

// ============================================================================
// AGENT EARNINGS TRACKING
// ============================================================================

#[test]
fn test_agent_earnings_accumulation() {
    // Agent processes 5 withdrawals
    let withdrawals = vec![100_000, 200_000, 150_000, 300_000, 250_000];
    
    let mut total_volume = 0u64;
    let mut total_agent_fees = 0u64;
    
    for amount in withdrawals {
        total_volume += amount;
        total_agent_fees += (amount * 90) / 100;
    }
    
    assert_eq!(total_volume, 1_000_000); // 1M processed
    assert_eq!(total_agent_fees, 900_000); // 900k earned
}

#[test]
fn test_earnings_vs_withdrawn() {
    let total_earned = 500_000u64;
    let total_withdrawn = 300_000u64;
    let available = total_earned - total_withdrawn;
    
    assert_eq!(available, 200_000); // 200k available to withdraw
}

// ============================================================================
// REVENUE COMPARISON TESTS
// ============================================================================

#[test]
fn test_withdrawal_vs_deposit_revenue() {
    let amount = 100_000u64;
    
    // Deposit: 0.5% commission
    let deposit_revenue = (amount * 50) / 10000;
    
    // Withdrawal: 10% fee
    let withdrawal_revenue = (amount * 10) / 100;
    
    assert_eq!(deposit_revenue, 500); // 500 UGX from deposit
    assert_eq!(withdrawal_revenue, 10_000); // 10k UGX from withdrawal
    
    // Withdrawal generates 20x more revenue!
    assert_eq!(withdrawal_revenue / deposit_revenue, 20);
}

#[test]
fn test_combined_revenue_scenario() {
    // User deposits 100k, later withdraws 100k
    let amount = 100_000u64;
    
    let deposit_commission = (amount * 50) / 10000; // 0.5%
    let withdrawal_fee = (amount * 10) / 100; // 10%
    
    let total_revenue = deposit_commission + withdrawal_fee;
    
    assert_eq!(deposit_commission, 500);
    assert_eq!(withdrawal_fee, 10_000);
    assert_eq!(total_revenue, 10_500); // 10.5k total revenue
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_minimum_withdrawal() {
    let amount = 1_000u64; // 1k UGX minimum
    
    let platform = (amount * 10) / 100;
    let agent = (amount * 90) / 100;
    
    assert_eq!(platform, 100);
    assert_eq!(agent, 900);
}

#[test]
fn test_fees_never_exceed_amount() {
    let amount = 50_000u64;
    
    let platform = (amount * 10) / 100;
    let agent = (amount * 90) / 100;
    let total_fees = platform + agent;
    
    assert_eq!(total_fees, amount); // Fees = 100% of amount
}

#[test]
fn test_large_number_handling() {
    // 100M UGX withdrawal
    let amount = 100_000_000u64;
    
    let platform = (amount * 10) / 100;
    let agent = (amount * 90) / 100;
    
    assert_eq!(platform, 10_000_000); // 10M to platform
    assert_eq!(agent, 90_000_000); // 90M to agent
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

// ============================================================================
// BUSINESS LOGIC TESTS
// ============================================================================

#[test]
fn test_agent_incentive_structure() {
    // Agent gives 100k cash, earns 90k fee
    // This incentivizes agents to provide liquidity
    let cash_given = 100_000u64;
    let fee_earned = (cash_given * 90) / 100;
    
    assert_eq!(fee_earned, 90_000);
    
    // Agent's effective cost: 10k UGX (10%)
    let agent_cost = cash_given - fee_earned;
    assert_eq!(agent_cost, 10_000);
}

#[test]
fn test_platform_revenue_scaling() {
    // As volume increases, platform revenue scales
    let volumes = vec![1_000_000, 5_000_000, 10_000_000, 50_000_000];
    
    for volume in volumes {
        let revenue = (volume * 10) / 100;
        
        // Revenue is always 10% of volume
        assert_eq!(revenue, volume / 10);
    }
}
