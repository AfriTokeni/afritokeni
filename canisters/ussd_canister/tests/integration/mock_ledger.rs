// Mock ICRC-1 Ledger Canister for Testing
// Implements minimal ICRC-1 interface needed for tests

use candid::{CandidType, Deserialize, Nat, Principal};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

thread_local! {
    static BALANCES: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static TOTAL_SUPPLY: RefCell<u64> = RefCell::new(1_000_000_000_000); // 1 trillion base units
}

/// Mock ICRC-1 transfer function
pub fn mock_icrc1_transfer(arg: TransferArg) -> Result<Nat, TransferError> {
    let to_key = format!("{}", arg.to.owner);
    let amount_u64 = arg.amount.0.to_u64_digits()[0];
    
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let current = balances.get(&to_key).unwrap_or(&0);
        balances.insert(to_key, current + amount_u64);
    });
    
    // Return a mock transaction ID
    Ok(Nat::from(12345u64))
}

/// Mock ICRC-1 balance query
pub fn mock_icrc1_balance_of(account: Account) -> Nat {
    let key = format!("{}", account.owner);
    BALANCES.with(|balances| {
        let balances = balances.borrow();
        Nat::from(*balances.get(&key).unwrap_or(&0))
    })
}

/// Mock ICRC-1 metadata query
pub fn mock_icrc1_metadata() -> Vec<(String, candid::types::internal::MetadataValue)> {
    use candid::types::internal::MetadataValue;
    vec![
        ("icrc1:name".to_string(), MetadataValue::Text("Mock Token".to_string())),
        ("icrc1:symbol".to_string(), MetadataValue::Text("MOCK".to_string())),
        ("icrc1:decimals".to_string(), MetadataValue::Nat(Nat::from(8u64))),
    ]
}

/// Reset mock ledger state (for test isolation)
pub fn reset_mock_ledger() {
    BALANCES.with(|balances| {
        balances.borrow_mut().clear();
    });
}

/// Get WASM bytes for mock ledger canister
pub fn get_mock_ledger_wasm() -> Vec<u8> {
    // For now, we'll use the actual ledger WASM if available
    // In a real implementation, you'd compile a minimal mock canister
    vec![]
}
