// Type definitions for Business Logic Canister API
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserBalances {
    pub fiat_balances: Vec<FiatBalance>,
    pub ckbtc_balance: u64,
    pub ckusdc_balance: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FiatBalance {
    pub currency: String,
    pub amount: u64,
}

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

#[derive(CandidType, Deserialize, Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum CryptoType {
    ckBTC,
    ckUSDC,
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
