// ============================================================================
// Services Module - Inter-Canister Communication
// ============================================================================
// All I/O operations go through service clients
// ============================================================================

pub mod data_client;
pub mod user_client;
pub mod wallet_client;

pub use data_client::*;
pub use user_client::*;
pub use wallet_client::*;
