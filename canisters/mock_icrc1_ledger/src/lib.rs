// Mock ICRC-1 Ledger for Testing
// Implements minimal ICRC-1 standard for PocketIC tests
use candid::{CandidType, Deserialize, Principal, Nat};
use ic_cdk::api::msg_caller;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

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

pub type TransferResult = Result<Nat, TransferError>;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ApproveArg {
    pub from_subaccount: Option<Vec<u8>>,
    pub spender: Account,
    pub amount: Nat,
    pub expected_allowance: Option<Nat>,
    pub expires_at: Option<u64>,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum ApproveError {
    BadFee { expected_fee: Nat },
    InsufficientFunds { balance: Nat },
    AllowanceChanged { current_allowance: Nat },
    Expired { ledger_time: u64 },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

pub type ApproveResult = Result<Nat, ApproveError>;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferFromArg {
    pub spender_subaccount: Option<Vec<u8>>,
    pub from: Account,
    pub to: Account,
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

pub type TransferFromResult = Result<Nat, TransferError>;

// Storage
thread_local! {
    static BALANCES: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static ALLOWANCES: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static NEXT_TX_ID: RefCell<u64> = RefCell::new(1);
}

fn account_key(account: &Account) -> String {
    format!("{}:{:?}", account.owner.to_text(), account.subaccount)
}

fn allowance_key(owner: &Account, spender: &Account) -> String {
    format!("{}:{}", account_key(owner), account_key(spender))
}

#[init]
fn init() {
    ic_cdk::println!("🧪 Mock ICRC-1 Ledger initialized");
}

/// ICRC-1 transfer
#[update]
fn icrc1_transfer(arg: TransferArg) -> TransferResult {
    let caller = msg_caller();
    
    let from = Account {
        owner: caller,
        subaccount: arg.from_subaccount.clone(),
    };
    
    let from_key = account_key(&from);
    let to_key = account_key(&arg.to);
    let amount: u64 = arg.amount.0.try_into().unwrap_or(0);
    
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let from_balance = balances.get(&from_key).copied().unwrap_or(0);
        
        if from_balance < amount {
            return Err(TransferError::InsufficientFunds {
                balance: Nat::from(from_balance),
            });
        }
        
        balances.insert(from_key.clone(), from_balance - amount);
        let to_balance = balances.get(&to_key).copied().unwrap_or(0);
        balances.insert(to_key.clone(), to_balance + amount);

        let tx_id = NEXT_TX_ID.with(|id| {
            let current = *id.borrow();
            *id.borrow_mut() = current + 1;
            current
        });

        ic_cdk::println!("✅ Mock transfer: {} -> {} amount: {}", from_key, account_key(&arg.to), amount);
        ic_cdk::println!("💰 Balance after transfer: {} = {}", to_key, to_balance + amount);
        Ok(Nat::from(tx_id))
    })
}

/// ICRC-2 approve
#[update]
fn icrc2_approve(arg: ApproveArg) -> ApproveResult {
    let caller = msg_caller();
    
    let from = Account {
        owner: caller,
        subaccount: arg.from_subaccount.clone(),
    };
    
    let allowance_key_str = allowance_key(&from, &arg.spender);
    let amount: u64 = arg.amount.0.try_into().unwrap_or(0);
    
    ALLOWANCES.with(|a| {
        let mut allowances = a.borrow_mut();
        allowances.insert(allowance_key_str.clone(), amount);
        
        let tx_id = NEXT_TX_ID.with(|id| {
            let current = *id.borrow();
            *id.borrow_mut() = current + 1;
            current
        });
        
        ic_cdk::println!("✅ Mock approve: {} -> {} amount: {}", account_key(&from), account_key(&arg.spender), amount);
        Ok(Nat::from(tx_id))
    })
}

/// ICRC-2 transfer_from
#[update]
fn icrc2_transfer_from(arg: TransferFromArg) -> TransferFromResult {
    let caller = msg_caller();

    let spender = Account {
        owner: caller,
        subaccount: arg.spender_subaccount.clone(),
    };

    let allowance_key_str = allowance_key(&arg.from, &spender);
    let from_key = account_key(&arg.from);
    let to_key = account_key(&arg.to);
    let amount: u64 = arg.amount.0.try_into().unwrap_or(0);

    ic_cdk::println!("🔍 Mock transfer_from called: from_key='{}', to_key='{}', amount={}", from_key, to_key, amount);

    // Check allowance (but allow unlimited for testing if no explicit allowance set)
    let allowed = ALLOWANCES.with(|a| {
        a.borrow().get(&allowance_key_str).copied().unwrap_or(u64::MAX)  // Default to unlimited for testing
    });

    ic_cdk::println!("🔍 Mock transfer_from allowance check: allowance_key='{}', allowed={}, need={}",
        allowance_key_str, if allowed == u64::MAX { "unlimited".to_string() } else { allowed.to_string() }, amount);

    if allowed < amount {
        return Err(TransferError::InsufficientFunds {
            balance: Nat::from(allowed),
        });
    }

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let from_balance = balances.get(&from_key).copied().unwrap_or(0);

        ic_cdk::println!("🔍 Mock transfer_from balance check: from_key='{}', balance={}, need={}",
            from_key, from_balance, amount);

        if from_balance < amount {
            return Err(TransferError::InsufficientFunds {
                balance: Nat::from(from_balance),
            });
        }

        // Update balances
        balances.insert(from_key.clone(), from_balance - amount);
        let to_balance = balances.get(&to_key).copied().unwrap_or(0);
        balances.insert(to_key.clone(), to_balance + amount);

        // Decrease allowance (only if not unlimited)
        if allowed != u64::MAX {
            ALLOWANCES.with(|a| {
                let mut allowances = a.borrow_mut();
                allowances.insert(allowance_key_str.clone(), allowed - amount);
            });
        }

        let tx_id = NEXT_TX_ID.with(|id| {
            let current = *id.borrow();
            *id.borrow_mut() = current + 1;
            current
        });

        ic_cdk::println!("✅ Mock transfer_from: {} -> {} amount: {}", from_key, account_key(&arg.to), amount);
        ic_cdk::println!("💰 Balance after transfer_from: from={}, to={}", from_balance - amount, to_balance + amount);
        Ok(Nat::from(tx_id))
    })
}

/// ICRC-1 balance_of
#[query]
fn icrc1_balance_of(account: Account) -> Nat {
    let key = account_key(&account);
    BALANCES.with(|b| {
        let balance = b.borrow().get(&key).copied().unwrap_or(0);
        Nat::from(balance)
    })
}

/// Test helper: Set balance directly
#[update]
fn set_balance_for_testing(account: Account, balance: u64) {
    let key = account_key(&account);
    BALANCES.with(|b| {
        b.borrow_mut().insert(key.clone(), balance);
    });
    ic_cdk::println!("🧪 Mock ledger: Set balance {} = {}", key, balance);
}

/// Test helper: Set allowance directly (for testing approve+transferFrom patterns)
#[update]
fn set_allowance_for_testing(from: Account, spender: Account, amount: u64) {
    let key = allowance_key(&from, &spender);
    ALLOWANCES.with(|a| {
        a.borrow_mut().insert(key.clone(), amount);
    });
    ic_cdk::println!("🧪 Mock ledger: Set allowance {} -> {} = {}", account_key(&from), account_key(&spender), amount);
}

/// Test helper: Get all balances
#[query]
fn get_all_balances_for_testing() -> Vec<(String, u64)> {
    BALANCES.with(|b| {
        b.borrow().iter().map(|(k, v)| (k.clone(), *v)).collect()
    })
}
