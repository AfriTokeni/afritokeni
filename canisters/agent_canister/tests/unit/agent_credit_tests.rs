// ============================================================================
// Agent Credit System Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use shared_types::{AgentTier, AgentBalance};
    
    // ============================================================================
    // Agent Tier Tests
    // ============================================================================
    
    #[test]
    fn test_agent_tier_credit_limits() {
        assert_eq!(AgentTier::New.default_credit_limit(), 1_000_000);
        assert_eq!(AgentTier::Trusted.default_credit_limit(), 5_000_000);
        assert_eq!(AgentTier::Premium.default_credit_limit(), 10_000_000);
    }
    
    #[test]
    fn test_agent_tier_progression() {
        let new_tier = AgentTier::New;
        let trusted_tier = AgentTier::Trusted;
        let premium_tier = AgentTier::Premium;
        
        assert!(new_tier.default_credit_limit() < trusted_tier.default_credit_limit());
        assert!(trusted_tier.default_credit_limit() < premium_tier.default_credit_limit());
    }
    
    // ============================================================================
    // Outstanding Balance Tests
    // ============================================================================
    
    #[test]
    fn test_outstanding_balance_deposit_scenario() {
        // Agent credits user 100,000 UGX
        // Agent owes platform 100,000 UGX
        // outstanding_balance = -100,000
        
        let mut balance = create_test_balance();
        let deposit_amount = 100_000u64;
        
        balance.outstanding_balance -= deposit_amount as i64;
        
        assert_eq!(balance.outstanding_balance, -100_000);
        assert!(balance.outstanding_balance < 0); // Agent owes platform
    }
    
    #[test]
    fn test_outstanding_balance_withdrawal_scenario() {
        // User withdraws 50,000 UGX from agent
        // Platform owes agent 50,000 UGX
        // outstanding_balance = +50,000
        
        let mut balance = create_test_balance();
        let withdrawal_amount = 50_000u64;
        
        balance.outstanding_balance += withdrawal_amount as i64;
        
        assert_eq!(balance.outstanding_balance, 50_000);
        assert!(balance.outstanding_balance > 0); // Platform owes agent
    }
    
    #[test]
    fn test_outstanding_balance_mixed_scenario() {
        // Agent credits user 200,000 UGX (deposit)
        // User withdraws 80,000 UGX (withdrawal)
        // Net: Agent owes platform 120,000 UGX
        
        let mut balance = create_test_balance();
        
        balance.outstanding_balance -= 200_000; // Deposit
        balance.outstanding_balance += 80_000;  // Withdrawal
        
        assert_eq!(balance.outstanding_balance, -120_000);
        assert!(balance.outstanding_balance < 0); // Agent still owes platform
    }
    
    #[test]
    fn test_outstanding_balance_settlement() {
        let mut balance = create_test_balance();
        
        // Agent does deposits totaling 500,000
        balance.outstanding_balance -= 500_000;
        assert_eq!(balance.outstanding_balance, -500_000);
        
        // Agent settles by paying platform
        balance.outstanding_balance = 0;
        assert_eq!(balance.outstanding_balance, 0);
    }
    
    // ============================================================================
    // Credit Limit Tests
    // ============================================================================
    
    #[test]
    fn test_credit_limit_not_exceeded() {
        let balance = create_test_balance_with_tier(AgentTier::New);
        let deposit_amount = 500_000u64;
        
        let new_outstanding = balance.outstanding_balance - (deposit_amount as i64);
        let would_exceed = new_outstanding.unsigned_abs() > balance.credit_limit;
        
        assert!(!would_exceed);
    }
    
    #[test]
    fn test_credit_limit_exceeded() {
        let balance = create_test_balance_with_tier(AgentTier::New);
        let deposit_amount = 1_500_000u64; // Exceeds 1M limit
        
        let new_outstanding = balance.outstanding_balance - (deposit_amount as i64);
        let would_exceed = new_outstanding.unsigned_abs() > balance.credit_limit;
        
        assert!(would_exceed);
    }
    
    #[test]
    fn test_credit_limit_exact_limit() {
        let balance = create_test_balance_with_tier(AgentTier::New);
        let deposit_amount = 1_000_000u64; // Exactly at limit
        
        let new_outstanding = balance.outstanding_balance - (deposit_amount as i64);
        let would_exceed = new_outstanding.unsigned_abs() > balance.credit_limit;
        
        assert!(!would_exceed);
    }
    
    #[test]
    fn test_available_credit_calculation() {
        let mut balance = create_test_balance_with_tier(AgentTier::Trusted); // 5M limit
        
        // Agent has used 2M of credit
        balance.outstanding_balance = -2_000_000;
        
        let available_credit = if balance.outstanding_balance < 0 {
            balance.credit_limit.saturating_sub(balance.outstanding_balance.unsigned_abs())
        } else {
            balance.credit_limit
        };
        
        assert_eq!(available_credit, 3_000_000);
    }
    
    #[test]
    fn test_credit_utilization_calculation() {
        let mut balance = create_test_balance_with_tier(AgentTier::Premium); // 10M limit
        
        // Agent has used 7M of credit
        balance.outstanding_balance = -7_000_000;
        
        let utilization = (balance.outstanding_balance.unsigned_abs() as f64 
            / balance.credit_limit as f64) * 100.0;
        
        assert_eq!(utilization, 70.0);
    }
    
    // ============================================================================
    // Tier Upgrade Tests
    // ============================================================================
    
    #[test]
    fn test_tier_upgrade_increases_limit() {
        let mut balance = create_test_balance_with_tier(AgentTier::New);
        assert_eq!(balance.credit_limit, 1_000_000);
        
        // Upgrade to Trusted
        balance.credit_limit = AgentTier::Trusted.default_credit_limit();
        assert_eq!(balance.credit_limit, 5_000_000);
        
        // Upgrade to Premium
        balance.credit_limit = AgentTier::Premium.default_credit_limit();
        assert_eq!(balance.credit_limit, 10_000_000);
    }
    
    #[test]
    fn test_tier_upgrade_with_existing_balance() {
        let mut balance = create_test_balance_with_tier(AgentTier::New);
        
        // Agent has used 800K of 1M limit
        balance.outstanding_balance = -800_000;
        
        let old_available = balance.credit_limit.saturating_sub(
            balance.outstanding_balance.unsigned_abs()
        );
        assert_eq!(old_available, 200_000);
        
        // Upgrade to Trusted (5M limit)
        balance.credit_limit = AgentTier::Trusted.default_credit_limit();
        
        let new_available = balance.credit_limit.saturating_sub(
            balance.outstanding_balance.unsigned_abs()
        );
        assert_eq!(new_available, 4_200_000);
    }
    
    // ============================================================================
    // Edge Cases
    // ============================================================================
    
    #[test]
    fn test_zero_outstanding_balance() {
        let balance = create_test_balance();
        assert_eq!(balance.outstanding_balance, 0);
        
        let available_credit = if balance.outstanding_balance < 0 {
            balance.credit_limit.saturating_sub(balance.outstanding_balance.unsigned_abs())
        } else {
            balance.credit_limit
        };
        
        assert_eq!(available_credit, balance.credit_limit);
    }
    
    #[test]
    fn test_positive_outstanding_balance_available_credit() {
        let mut balance = create_test_balance_with_tier(AgentTier::New);
        
        // Platform owes agent 300K (from withdrawals)
        balance.outstanding_balance = 300_000;
        
        // Available credit should still be full limit
        let available_credit = if balance.outstanding_balance < 0 {
            balance.credit_limit.saturating_sub(balance.outstanding_balance.unsigned_abs())
        } else {
            balance.credit_limit
        };
        
        assert_eq!(available_credit, balance.credit_limit);
    }
    
    #[test]
    fn test_saturating_sub_prevents_overflow() {
        let mut balance = create_test_balance_with_tier(AgentTier::New);
        
        // Agent owes more than limit (shouldn't happen in production)
        balance.outstanding_balance = -2_000_000;
        
        let available_credit = balance.credit_limit.saturating_sub(
            balance.outstanding_balance.unsigned_abs()
        );
        
        // Should saturate to 0, not underflow
        assert_eq!(available_credit, 0);
    }
    
    // ============================================================================
    // Helper Functions
    // ============================================================================
    
    fn create_test_balance() -> AgentBalance {
        AgentBalance {
            agent_id: "test_agent".to_string(),
            currency: "UGX".to_string(),
            total_deposits: 0,
            total_withdrawals: 0,
            commission_earned: 0,
            commission_paid: 0,
            outstanding_balance: 0,
            credit_limit: AgentTier::New.default_credit_limit(),
            last_settlement_date: None,
            last_updated: 0,
        }
    }
    
    fn create_test_balance_with_tier(tier: AgentTier) -> AgentBalance {
        AgentBalance {
            agent_id: "test_agent".to_string(),
            currency: "UGX".to_string(),
            total_deposits: 0,
            total_withdrawals: 0,
            commission_earned: 0,
            commission_paid: 0,
            outstanding_balance: 0,
            credit_limit: tier.default_credit_limit(),
            last_settlement_date: None,
            last_updated: 0,
        }
    }
}
