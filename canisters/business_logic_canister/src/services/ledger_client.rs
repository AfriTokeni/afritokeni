// ============================================================================
// ICRC-1/ICRC-2 Ledger Client - Production Ready
// ============================================================================
// Handles all interactions with ckBTC and ckUSDC ledgers
// ============================================================================

use candid::{CandidType, Deserialize, Principal, Nat};
use ic_cdk::call::Call;
use super::config;

#[derive(CandidType, Deserialize, Clone, Copy, Debug)]
pub enum CryptoToken {
    CkBTC,
    CkUSDC,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Debug)]
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

/// Transfer crypto from this canister to a user
pub async fn transfer_crypto_to_user(
    token: CryptoToken,
    to_principal: Principal,
    amount: u64,
) -> Result<u64, String> {
    let ledger_id = get_ledger_principal(token)?;
    
    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: to_principal,
            subaccount: None,
        },
        amount: Nat::from(amount),
        fee: None, // Use default fee
        memo: None,
        created_at_time: Some(ic_cdk::api::time()),
    };
    
    let response = Call::unbounded_wait(ledger_id, "icrc1_transfer")
        .with_arg((transfer_arg,))
        .await
        .map_err(|e| format!("ICRC-1 transfer call failed: {:?}", e))?;
    
    let result: (Result<Nat, TransferError>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode transfer response: {:?}", e))?;
    
    match result.0 {
        Ok(block_index) => {
            let block_u64: u64 = block_index.0.try_into()
                .map_err(|_| "Block index too large".to_string())?;
            Ok(block_u64)
        }
        Err(e) => Err(format!("Transfer failed: {:?}", e)),
    }
}

/// Get balance of a principal
#[allow(dead_code)]
pub async fn get_crypto_balance(
    token: CryptoToken,
    principal: Principal,
) -> Result<u64, String> {
    let ledger_id = get_ledger_principal(token)?;
    
    let account = Account {
        owner: principal,
        subaccount: None,
    };
    
    let response = Call::unbounded_wait(ledger_id, "icrc1_balance_of")
        .with_arg((account,))
        .await
        .map_err(|e| format!("Balance query failed: {:?}", e))?;
    
    let (balance,): (Nat,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode balance: {:?}", e))?;
    
    let balance_u64: u64 = balance.0.try_into()
        .map_err(|_| "Balance too large".to_string())?;
    
    Ok(balance_u64)
}

/// Get ledger fee
#[allow(dead_code)]
pub async fn get_transfer_fee(token: CryptoToken) -> Result<u64, String> {
    let ledger_id = get_ledger_principal(token)?;
    
    let response = Call::unbounded_wait(ledger_id, "icrc1_fee")
        .with_arg(())
        .await
        .map_err(|e| format!("Fee query failed: {:?}", e))?;
    
    let (fee,): (Nat,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode fee: {:?}", e))?;
    
    let fee_u64: u64 = fee.0.try_into()
        .map_err(|_| "Fee too large".to_string())?;
    
    Ok(fee_u64)
}

/// Approve spending (ICRC-2)
#[allow(dead_code)]
pub async fn approve_spending(
    token: CryptoToken,
    spender: Principal,
    amount: u64,
) -> Result<u64, String> {
    let ledger_id = get_ledger_principal(token)?;
    
    #[derive(CandidType)]
    struct ApproveArg {
        from_subaccount: Option<Vec<u8>>,
        spender: Account,
        amount: Nat,
        expected_allowance: Option<Nat>,
        expires_at: Option<u64>,
        fee: Option<Nat>,
        memo: Option<Vec<u8>>,
        created_at_time: Option<u64>,
    }
    
    let approve_arg = ApproveArg {
        from_subaccount: None,
        spender: Account {
            owner: spender,
            subaccount: None,
        },
        amount: Nat::from(amount),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: Some(ic_cdk::api::time()),
    };
    
    let response = Call::unbounded_wait(ledger_id, "icrc2_approve")
        .with_arg((approve_arg,))
        .await
        .map_err(|e| format!("ICRC-2 approve call failed: {:?}", e))?;
    
    let result: (Result<Nat, TransferError>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Failed to decode approve response: {:?}", e))?;
    
    match result.0 {
        Ok(block_index) => {
            let block_u64: u64 = block_index.0.try_into()
                .map_err(|_| "Block index too large".to_string())?;
            Ok(block_u64)
        }
        Err(e) => Err(format!("Approve failed: {:?}", e)),
    }
}

fn get_ledger_principal(token: CryptoToken) -> Result<Principal, String> {
    Ok(match token {
        CryptoToken::CkBTC => config::get_ckbtc_ledger_id(),
        CryptoToken::CkUSDC => config::get_ckusdc_ledger_id(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ledger_principal_ckbtc() {
        let principal = get_ledger_principal(CryptoToken::CkBTC).unwrap();
        assert_eq!(principal, config::get_ckbtc_ledger_id());
    }

    #[test]
    fn test_get_ledger_principal_ckusdc() {
        let principal = get_ledger_principal(CryptoToken::CkUSDC).unwrap();
        assert_eq!(principal, config::get_ckusdc_ledger_id());
    }
}
