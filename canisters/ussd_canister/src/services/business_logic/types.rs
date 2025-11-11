// Type definitions for Business Logic Canister API
// Re-export shared types to avoid duplication
use candid::{CandidType, Deserialize};

// Use shared types - NO MORE DUPLICATION!
pub use shared_types::UserBalances;
pub use shared_types::CryptoType;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransactionResult {
    pub transaction_id: String,
    pub from_user: String,
    pub to_user: String,
    pub amount: u64,
    pub currency: String,
    pub new_balance: u64,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub tx_type: String,
    pub amount: u64,
    pub timestamp: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Agent {
    pub name: String,
    pub phone: String,
    pub location: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ExchangeRate {
    pub rate_to_fiat: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DaoProposal {
    pub title: String,
    pub status: String,
    pub yes_votes: u64,
    pub total_votes: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FeeConfig {
    pub fee_percentage: f64,
    pub min_fee: f64,
    pub max_fee: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SwapResult {
    pub from_crypto: CryptoType,
    pub to_crypto: CryptoType,
    pub from_amount: u64,
    pub to_amount: u64,
    pub spread_amount: u64,
    pub exchange_rate: String,
    pub tx_id: String,
    pub timestamp: u64,
}
