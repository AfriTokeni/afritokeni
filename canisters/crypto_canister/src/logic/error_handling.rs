/// Error handling utilities to prevent information leakage
/// This module sanitizes error messages to avoid exposing internal state

/// Sanitizes inter-canister call errors to prevent information leakage
#[allow(dead_code)]
pub fn sanitize_canister_error(error: String, operation: &str) -> String {
    // Don't expose internal canister details or specific error codes
    // that could reveal system architecture to attackers

    if error.contains("insufficient") {
        // Preserve balance-related errors as they're user-facing
        error
    } else if error.contains("not found") || error.contains("does not exist") {
        // Generic "not found" errors are safe
        error
    } else if error.contains("unauthorized") || error.contains("not authorized") {
        "Unauthorized operation".to_string()
    } else if error.contains("rate limit") || error.contains("too many") {
        "Rate limit exceeded. Please try again later".to_string()
    } else if error.contains("timeout") || error.contains("deadline") {
        "Operation timeout. Please try again".to_string()
    } else {
        // Generic error for unexpected failures
        format!("{} failed. Please try again or contact support", operation)
    }
}

/// Sanitizes ledger errors to prevent information leakage
#[allow(dead_code)]
pub fn sanitize_ledger_error(error: String) -> String {
    if error.contains("InsufficientFunds") || error.contains("insufficient") {
        "Insufficient balance for this operation".to_string()
    } else if error.contains("InsufficientAllowance") || error.contains("allowance") {
        "Insufficient approval. Please approve spending in your wallet first".to_string()
    } else if error.contains("BadFee") || error.contains("fee") {
        "Transaction fee error. Please try again".to_string()
    } else if error.contains("Duplicate") || error.contains("duplicate") {
        "Duplicate transaction detected".to_string()
    } else if error.contains("TooOld") || error.contains("expired") {
        "Transaction expired. Please retry".to_string()
    } else {
        "Ledger operation failed. Please try again".to_string()
    }
}

/// Sanitizes exchange rate errors
#[allow(dead_code)]
pub fn sanitize_exchange_rate_error(error: String) -> String {
    if error.contains("unavailable") || error.contains("not available") {
        "Exchange rate temporarily unavailable. Please try again".to_string()
    } else if error.contains("timeout") {
        "Exchange rate service timeout. Please try again".to_string()
    } else {
        "Unable to fetch exchange rate. Please try again later".to_string()
    }
}

/// Wraps a Result with sanitized error messages
#[allow(dead_code)]
pub fn with_sanitized_error<T>(
    result: Result<T, String>,
    operation: &str,
) -> Result<T, String> {
    result.map_err(|e| sanitize_canister_error(e, operation))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_canister_error_insufficient() {
        let error = "Insufficient fiat balance. Have: 100, Need: 200".to_string();
        let sanitized = sanitize_canister_error(error.clone(), "Buy crypto");
        assert_eq!(sanitized, error); // Should preserve balance errors
    }

    #[test]
    fn test_sanitize_canister_error_unauthorized() {
        let error = "Caller xyz is not authorized to call this method".to_string();
        let sanitized = sanitize_canister_error(error, "Update balance");
        assert_eq!(sanitized, "Unauthorized operation");
        assert!(!sanitized.contains("xyz")); // Should not leak caller ID
    }

    #[test]
    fn test_sanitize_canister_error_rate_limit() {
        let error = "Too many requests from user abc123".to_string();
        let sanitized = sanitize_canister_error(error, "Send crypto");
        assert_eq!(sanitized, "Rate limit exceeded. Please try again later");
        assert!(!sanitized.contains("abc123")); // Should not leak user ID
    }

    #[test]
    fn test_sanitize_canister_error_timeout() {
        let error = "Call deadline exceeded for canister rxyz-cai".to_string();
        let sanitized = sanitize_canister_error(error, "Transfer");
        assert_eq!(sanitized, "Operation timeout. Please try again");
        assert!(!sanitized.contains("rxyz")); // Should not leak canister ID
    }

    #[test]
    fn test_sanitize_canister_error_generic() {
        let error = "Internal panic at line 123 in module xyz".to_string();
        let sanitized = sanitize_canister_error(error, "Process payment");
        assert_eq!(sanitized, "Process payment failed. Please try again or contact support");
        assert!(!sanitized.contains("line 123")); // Should not leak internal details
    }

    #[test]
    fn test_sanitize_ledger_error_insufficient_funds() {
        let error = "InsufficientFunds: balance=100, required=200".to_string();
        let sanitized = sanitize_ledger_error(error);
        assert_eq!(sanitized, "Insufficient balance for this operation");
        assert!(!sanitized.contains("balance=")); // Should not leak exact amounts
    }

    #[test]
    fn test_sanitize_ledger_error_allowance() {
        let error = "InsufficientAllowance: current=50, required=100".to_string();
        let sanitized = sanitize_ledger_error(error);
        assert_eq!(sanitized, "Insufficient approval. Please approve spending in your wallet first");
        assert!(!sanitized.contains("current=")); // Should not leak amounts
    }

    #[test]
    fn test_sanitize_ledger_error_bad_fee() {
        let error = "BadFee: expected=10000, provided=5000".to_string();
        let sanitized = sanitize_ledger_error(error);
        assert_eq!(sanitized, "Transaction fee error. Please try again");
        assert!(!sanitized.contains("expected=")); // Should not leak fee details
    }

    #[test]
    fn test_sanitize_exchange_rate_error_unavailable() {
        let error = "Exchange rate service unavailable at endpoint xyz".to_string();
        let sanitized = sanitize_exchange_rate_error(error);
        assert_eq!(sanitized, "Exchange rate temporarily unavailable. Please try again");
        assert!(!sanitized.contains("endpoint")); // Should not leak service details
    }

    #[test]
    fn test_sanitize_exchange_rate_error_timeout() {
        let error = "HTTP timeout after 5000ms to https://api.example.com/rates".to_string();
        let sanitized = sanitize_exchange_rate_error(error);
        assert_eq!(sanitized, "Exchange rate service timeout. Please try again");
        assert!(!sanitized.contains("api.example.com")); // Should not leak API endpoint
    }
}
