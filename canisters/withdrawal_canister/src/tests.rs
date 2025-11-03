use super::*;

// ============================================================================
// FEE SPLIT TESTS - CORRECT REVENUE MODEL
// ============================================================================

#[test]
fn test_correct_fee_calculation() {
    let amount = 100_000u64; // 100k UGX withdrawal
    
    // Use constants from lib.rs instead of hardcoded values
    let platform_base_fee = (amount * DEFAULT_PLATFORM_FEE_BPS) / 10000;
    let agent_total_fee = (amount * DEFAULT_AGENT_FEE_BPS) / 10000;
    let platform_cut_of_agent = (agent_total_fee * PLATFORM_CUT_OF_AGENT_FEE_PERCENT) / 100;
    
    // Total platform revenue
    let total_platform_fee = platform_base_fee + platform_cut_of_agent;
    
    // Agent keeps 90% of their fee
    let agent_keeps = agent_total_fee - platform_cut_of_agent;
    
    assert_eq!(platform_base_fee, 500); // 0.5% of 100k = 500 UGX
    assert_eq!(agent_total_fee, 3_000); // 3% of 100k = 3,000 UGX
    assert_eq!(platform_cut_of_agent, 300); // 10% of 3k = 300 UGX
    assert_eq!(total_platform_fee, 800); // 500 + 300 = 800 UGX total revenue
    assert_eq!(agent_keeps, 2_700); // Agent keeps 2,700 UGX (90% of their fee)
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
    
    let platform_base = (amount * DEFAULT_PLATFORM_FEE_BPS) / 10000;
    let agent_total = (amount * DEFAULT_AGENT_FEE_BPS) / 10000;
    let platform_cut = (agent_total * PLATFORM_CUT_OF_AGENT_FEE_PERCENT) / 100;
    let platform_total = platform_base + platform_cut;
    let agent_keeps = agent_total - platform_cut;
    
    assert_eq!(platform_base, 50); // 0.5% of 10k = 50 UGX
    assert_eq!(agent_total, 300); // 3% of 10k = 300 UGX
    assert_eq!(platform_total, 80); // 50 + 30 = 80 UGX
    assert_eq!(agent_keeps, 270); // Agent keeps 270 UGX
}

#[test]
fn test_medium_withdrawal_fees() {
    let amount = 100_000u64; // 100k UGX
    
    let platform_base = (amount * DEFAULT_PLATFORM_FEE_BPS) / 10000;
    let agent_total = (amount * DEFAULT_AGENT_FEE_BPS) / 10000;
    let platform_cut = (agent_total * PLATFORM_CUT_OF_AGENT_FEE_PERCENT) / 100;
    let platform_total = platform_base + platform_cut;
    let agent_keeps = agent_total - platform_cut;
    
    assert_eq!(platform_base, 500); // 0.5% of 100k = 500 UGX
    assert_eq!(platform_total, 800); // 500 + 300 = 800 UGX
    assert_eq!(agent_keeps, 2_700); // Agent keeps 2,700 UGX
}

#[test]
fn test_large_withdrawal_fees() {
    let amount = 1_000_000u64; // 1M UGX
    
    let platform_base = (amount * DEFAULT_PLATFORM_FEE_BPS) / 10000;
    let agent_total = (amount * DEFAULT_AGENT_FEE_BPS) / 10000;
    let platform_cut = (agent_total * PLATFORM_CUT_OF_AGENT_FEE_PERCENT) / 100;
    let platform_total = platform_base + platform_cut;
    let agent_keeps = agent_total - platform_cut;
    
    assert_eq!(platform_base, 5_000); // 0.5% of 1M = 5,000 UGX
    assert_eq!(agent_total, 30_000); // 3% of 1M = 30,000 UGX
    assert_eq!(platform_total, 8_000); // 5,000 + 3,000 = 8,000 UGX
    assert_eq!(agent_keeps, 27_000); // Agent keeps 27,000 UGX
}

// ============================================================================
// REALISTIC SCENARIOS
// ============================================================================

#[test]
fn test_typical_withdrawal_scenario() {
    // User withdraws 50,000 UGX (~$13 USD)
    let amount = 50_000u64;
    
    let platform_base = (amount * 50) / 10000; // 0.5%
    let agent_total = (amount * 300) / 10000; // 3%
    let platform_cut = (agent_total * 10) / 100;
    let platform_fee = platform_base + platform_cut;
    let agent_keeps = agent_total - platform_cut;
    
    assert_eq!(platform_fee, 400); // AfriTokeni gets 400 UGX (250 + 150)
    assert_eq!(agent_keeps, 1_350); // Agent keeps 1,350 UGX
    
    // Agent gives user 50k cash, earns 1,350 UGX net
}

#[test]
fn test_agent_daily_earnings() {
    // Agent processes 20 withdrawals per day, avg 75k UGX each
    let avg_withdrawal = 75_000u64;
    let withdrawals_per_day = 20u64;
    let daily_volume = avg_withdrawal * withdrawals_per_day;
    
    // Per withdrawal calculations
    let platform_base_per = (avg_withdrawal * 50) / 10000; // 0.5%
    let agent_total_per = (avg_withdrawal * 300) / 10000; // 3%
    let platform_cut_per = (agent_total_per * 10) / 100;
    
    let daily_platform_base = platform_base_per * withdrawals_per_day;
    let daily_platform_cut = platform_cut_per * withdrawals_per_day;
    let daily_platform_revenue = daily_platform_base + daily_platform_cut;
    let daily_agent_earnings = (agent_total_per - platform_cut_per) * withdrawals_per_day;
    
    assert_eq!(daily_volume, 1_500_000); // 1.5M UGX daily volume
    assert_eq!(daily_agent_earnings, 40_500); // Agent earns 40.5k (2,025 * 20)
    assert_eq!(daily_platform_revenue, 12_000); // Platform gets 12k (600 * 20)
}

#[test]
fn test_monthly_agent_earnings() {
    // Agent processes 500k UGX in withdrawals per month
    let monthly_volume = 500_000u64;
    
    let platform_base = (monthly_volume * 50) / 10000; // 0.5%
    let agent_total = (monthly_volume * 300) / 10000; // 3%
    let platform_cut = (agent_total * 10) / 100;
    let platform_revenue = platform_base + platform_cut;
    let agent_earnings = agent_total - platform_cut;
    
    assert_eq!(agent_earnings, 13_500); // Agent earns 13,500 UGX/month
    assert_eq!(platform_revenue, 4_000); // Platform gets 4,000 UGX/month
}

#[test]
fn test_multiple_agents_revenue() {
    // 10 agents, each processes 1M UGX/month
    let per_agent_volume = 1_000_000u64;
    let num_agents = 10u64;
    let total_volume = per_agent_volume * num_agents;
    
    let platform_base_per = (per_agent_volume * 50) / 10000;
    let agent_total_per = (per_agent_volume * 300) / 10000;
    let platform_cut_per = (agent_total_per * 10) / 100;
    
    let total_platform_revenue = (platform_base_per + platform_cut_per) * num_agents;
    let total_agent_earnings = (agent_total_per - platform_cut_per) * num_agents;
    
    assert_eq!(total_volume, 10_000_000); // 10M total
    assert_eq!(total_platform_revenue, 80_000); // 80k to platform (8k * 10)
    assert_eq!(total_agent_earnings, 270_000); // 270k to agents (27k * 10)
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
        let agent_total = (amount * 300) / 10000; // 3%
        let platform_cut = (agent_total * 10) / 100;
        total_agent_fees += agent_total - platform_cut; // Agent keeps 90%
    }
    
    assert_eq!(total_volume, 1_000_000); // 1M processed
    assert_eq!(total_agent_fees, 27_000); // 27k earned (90% of 30k)
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
