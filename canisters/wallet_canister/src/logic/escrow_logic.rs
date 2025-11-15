/// Pure business logic for escrow operations
/// No I/O, no async, fully testable
use shared_types::{EscrowStatus, CryptoType};

/// Generate escrow code from timestamp and user_id
pub fn generate_escrow_code(timestamp: u64, user_id: &str) -> String {
    // Create a simple but unique code: first 8 chars of user_id + timestamp
    let user_prefix: String = user_id.chars().take(8).collect();
    format!("ESC-{}-{}", user_prefix, timestamp)
}

/// Validate escrow is active
pub fn validate_escrow_active(status: EscrowStatus) -> Result<(), String> {
    if status != EscrowStatus::Active {
        return Err(format!("Escrow is not active: {:?}", status));
    }
    Ok(())
}

/// Validate escrow not expired
pub fn validate_escrow_not_expired(current_time: u64, expires_at: u64) -> Result<(), String> {
    if current_time > expires_at {
        return Err("Escrow has expired".to_string());
    }
    Ok(())
}

/// Validate agent matches escrow
pub fn validate_agent_authorized(escrow_agent_id: &str, claiming_agent_id: &str) -> Result<(), String> {
    if escrow_agent_id != claiming_agent_id {
        return Err("Agent not authorized to claim this escrow".to_string());
    }
    Ok(())
}

/// Validate user owns escrow
pub fn validate_user_owns_escrow(escrow_user_id: &str, user_id: &str) -> Result<(), String> {
    if escrow_user_id != user_id {
        return Err("User does not own this escrow".to_string());
    }
    Ok(())
}

/// Calculate escrow expiration time
pub fn calculate_expiration_time(current_time: u64, expiration_duration_ns: u64) -> Result<u64, String> {
    current_time
        .checked_add(expiration_duration_ns)
        .ok_or_else(|| "Expiration time calculation would overflow".to_string())
}

/// Validate escrow amount is positive
pub fn validate_escrow_amount(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Escrow amount must be greater than 0".to_string());
    }
    Ok(())
}

/// Calculate crypto balance delta for escrow creation (deduct from user)
pub fn calculate_escrow_creation_delta(amount: u64, crypto_type: CryptoType) -> (i64, i64) {
    let amount_i64 = -(amount as i64);
    match crypto_type {
        CryptoType::CkBTC => (amount_i64, 0),
        CryptoType::CkUSD => (0, amount_i64),
    }
}

/// Calculate crypto balance delta for escrow claim (add to agent)
pub fn calculate_escrow_claim_delta(amount: u64, crypto_type: CryptoType) -> (i64, i64) {
    let amount_i64 = amount as i64;
    match crypto_type {
        CryptoType::CkBTC => (amount_i64, 0),
        CryptoType::CkUSD => (0, amount_i64),
    }
}

/// Calculate crypto balance delta for escrow cancellation (refund to user)
pub fn calculate_escrow_refund_delta(amount: u64, crypto_type: CryptoType) -> (i64, i64) {
    calculate_escrow_claim_delta(amount, crypto_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Escrow Code Generation Tests
    // ============================================================================

    #[test]
    fn test_generate_escrow_code_format() {
        let code = generate_escrow_code(1234567890, "user12345");
        assert!(code.starts_with("ESC-"));
        assert!(code.contains("user1234"));
        assert!(code.contains("1234567890"));
    }

    #[test]
    fn test_generate_escrow_code_unique() {
        let code1 = generate_escrow_code(1000, "user1");
        let code2 = generate_escrow_code(2000, "user1");
        assert_ne!(code1, code2);
    }

    #[test]
    fn test_generate_escrow_code_different_users() {
        let code1 = generate_escrow_code(1000, "user1");
        let code2 = generate_escrow_code(1000, "user2");
        assert_ne!(code1, code2);
    }

    #[test]
    fn test_generate_escrow_code_long_user_id() {
        let code = generate_escrow_code(1000, "verylonguseridherebutonly8charsused");
        assert!(code.contains("verylong"));
        assert!(!code.contains("useridherebutonly8charsused"));
    }

    // ============================================================================
    // Escrow Status Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_escrow_active_ok() {
        assert!(validate_escrow_active(EscrowStatus::Active).is_ok());
    }

    #[test]
    fn test_validate_escrow_active_claimed() {
        let result = validate_escrow_active(EscrowStatus::Claimed);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not active"));
    }

    #[test]
    fn test_validate_escrow_active_expired() {
        let result = validate_escrow_active(EscrowStatus::Expired);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_escrow_active_cancelled() {
        let result = validate_escrow_active(EscrowStatus::Cancelled);
        assert!(result.is_err());
    }

    // ============================================================================
    // Expiration Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_escrow_not_expired_ok() {
        assert!(validate_escrow_not_expired(1000, 2000).is_ok());
        assert!(validate_escrow_not_expired(1000, 1000).is_ok());
    }

    #[test]
    fn test_validate_escrow_not_expired_fail() {
        let result = validate_escrow_not_expired(2000, 1000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Escrow has expired");
    }

    // ============================================================================
    // Agent Authorization Tests
    // ============================================================================

    #[test]
    fn test_validate_agent_authorized_ok() {
        assert!(validate_agent_authorized("agent1", "agent1").is_ok());
    }

    #[test]
    fn test_validate_agent_authorized_fail() {
        let result = validate_agent_authorized("agent1", "agent2");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not authorized"));
    }

    // ============================================================================
    // User Ownership Tests
    // ============================================================================

    #[test]
    fn test_validate_user_owns_escrow_ok() {
        assert!(validate_user_owns_escrow("user1", "user1").is_ok());
    }

    #[test]
    fn test_validate_user_owns_escrow_fail() {
        let result = validate_user_owns_escrow("user1", "user2");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not own"));
    }

    // ============================================================================
    // Expiration Time Calculation Tests
    // ============================================================================

    #[test]
    fn test_calculate_expiration_time_valid() {
        let current = 1000u64;
        let duration = 86400000000000u64; // 24 hours in ns
        let expiration = calculate_expiration_time(current, duration).unwrap();
        assert_eq!(expiration, current + duration);
    }

    #[test]
    fn test_calculate_expiration_time_overflow() {
        let result = calculate_expiration_time(u64::MAX, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    // ============================================================================
    // Amount Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_escrow_amount_valid() {
        assert!(validate_escrow_amount(1).is_ok());
        assert!(validate_escrow_amount(1000).is_ok());
        assert!(validate_escrow_amount(u64::MAX).is_ok());
    }

    #[test]
    fn test_validate_escrow_amount_zero() {
        let result = validate_escrow_amount(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Escrow amount must be greater than 0");
    }

    // ============================================================================
    // Balance Delta Calculation Tests
    // ============================================================================

    #[test]
    fn test_calculate_escrow_creation_delta_ckbtc() {
        let (btc_delta, usdc_delta) = calculate_escrow_creation_delta(1000, CryptoType::CkBTC);
        assert_eq!(btc_delta, -1000);
        assert_eq!(usdc_delta, 0);
    }

    #[test]
    fn test_calculate_escrow_creation_delta_ckusdc() {
        let (btc_delta, usdc_delta) = calculate_escrow_creation_delta(1000, CryptoType::CkUSD);
        assert_eq!(btc_delta, 0);
        assert_eq!(usdc_delta, -1000);
    }

    #[test]
    fn test_calculate_escrow_claim_delta_ckbtc() {
        let (btc_delta, usdc_delta) = calculate_escrow_claim_delta(1000, CryptoType::CkBTC);
        assert_eq!(btc_delta, 1000);
        assert_eq!(usdc_delta, 0);
    }

    #[test]
    fn test_calculate_escrow_claim_delta_ckusdc() {
        let (btc_delta, usdc_delta) = calculate_escrow_claim_delta(1000, CryptoType::CkUSD);
        assert_eq!(btc_delta, 0);
        assert_eq!(usdc_delta, 1000);
    }

    #[test]
    fn test_calculate_escrow_refund_delta() {
        // Refund should be same as claim (positive delta)
        let (btc_delta1, usdc_delta1) = calculate_escrow_refund_delta(1000, CryptoType::CkBTC);
        let (btc_delta2, usdc_delta2) = calculate_escrow_claim_delta(1000, CryptoType::CkBTC);
        assert_eq!(btc_delta1, btc_delta2);
        assert_eq!(usdc_delta1, usdc_delta2);
    }

    // ============================================================================
    // Combined Scenario Tests
    // ============================================================================

    #[test]
    fn test_escrow_lifecycle() {
        let user_id = "user123";
        let agent_id = "agent456";
        let amount = 1000u64;
        let current_time = 1000u64;
        let duration = 86400000000000u64;

        // 1. Generate code
        let code = generate_escrow_code(current_time, user_id);
        assert!(code.starts_with("ESC-"));

        // 2. Validate amount
        assert!(validate_escrow_amount(amount).is_ok());

        // 3. Calculate expiration
        let expires_at = calculate_expiration_time(current_time, duration).unwrap();
        assert_eq!(expires_at, current_time + duration);

        // 4. Create escrow (deduct from user)
        let (btc_delta, _) = calculate_escrow_creation_delta(amount, CryptoType::CkBTC);
        assert_eq!(btc_delta, -1000);

        // 5. Validate active and not expired
        assert!(validate_escrow_active(EscrowStatus::Active).is_ok());
        assert!(validate_escrow_not_expired(current_time + 1000, expires_at).is_ok());

        // 6. Validate agent
        assert!(validate_agent_authorized(agent_id, agent_id).is_ok());

        // 7. Claim escrow (add to agent)
        let (btc_delta, _) = calculate_escrow_claim_delta(amount, CryptoType::CkBTC);
        assert_eq!(btc_delta, 1000);
    }

    #[test]
    fn test_escrow_cancellation() {
        let user_id = "user123";
        let amount = 1000u64;

        // 1. Validate ownership
        assert!(validate_user_owns_escrow(user_id, user_id).is_ok());

        // 2. Validate active
        assert!(validate_escrow_active(EscrowStatus::Active).is_ok());

        // 3. Refund to user
        let (btc_delta, _) = calculate_escrow_refund_delta(amount, CryptoType::CkBTC);
        assert_eq!(btc_delta, 1000);
    }
}
