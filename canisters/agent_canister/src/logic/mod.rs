// ============================================================================
// Logic Module - Pure Business Logic
// ============================================================================
// All business logic is pure (no I/O) and 100% unit tested
// ============================================================================

pub mod deposit_logic;
pub mod withdrawal_logic;
pub mod fraud_detection;

pub use deposit_logic::*;
pub use withdrawal_logic::*;
pub use fraud_detection::*;
