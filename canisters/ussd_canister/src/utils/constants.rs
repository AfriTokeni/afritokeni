/// Global constants for USSD canister
///
/// This module centralizes magic numbers and configuration values used throughout
/// the USSD canister to improve maintainability and reduce duplication.

// =============================================================================
// CRYPTOCURRENCY CONVERSION FACTORS
// =============================================================================

/// Number of satoshis in one Bitcoin (10^8)
pub const SATOSHIS_PER_BTC: f64 = 100_000_000.0;

/// Number of micro-USDC (e6 format) in one USDC (10^6)
pub const E6_PER_USDC: f64 = 1_000_000.0;

// =============================================================================
// WITHDRAWAL LIMITS (in local currency units)
// =============================================================================

/// Minimum withdrawal amount in UGX
pub const MIN_WITHDRAWAL_AMOUNT_UGX: u64 = 100_000;

/// Maximum withdrawal amount in UGX
pub const MAX_WITHDRAWAL_AMOUNT_UGX: u64 = 1_000_000;

// =============================================================================
// VALIDATION LIMITS
// =============================================================================

/// Maximum allowed amount for validation (in local currency)
pub const MAX_VALIDATION_AMOUNT: f64 = 100_000_000.0;

// =============================================================================
// SESSION MANAGEMENT
// =============================================================================

/// Session timeout in nanoseconds (5 minutes)
pub const SESSION_TIMEOUT_NANOS: u64 = 5 * 60 * 1_000_000_000;

/// Nanoseconds per second conversion factor
pub const NANOS_PER_SECOND: u64 = 1_000_000_000;
