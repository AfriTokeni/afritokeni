use candid::{CandidType, Deserialize};

// Re-export CryptoType from shared_types instead of defining our own
pub use shared_types::CryptoType;

#[derive(CandidType, Deserialize, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: String,
    pub caller: String,
    pub user_id: Option<String>,
    pub details: String,
    pub success: bool,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransactionResult {
    pub transaction_id: String,
    pub from_user: String,
    pub to_user: String,
    pub amount: u64,
    pub currency: String,
    pub new_balance: u64,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserBalances {
    pub fiat_balances: Vec<FiatBalance>,
    pub ckbtc_balance: u64,
    pub ckusdc_balance: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct FiatBalance {
    pub currency: String,
    pub balance: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransactionRecord {
    pub id: String,
    pub transaction_type: String,
    pub amount: u64,
    pub currency: String,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub timestamp: u64,
    pub status: String,
}
