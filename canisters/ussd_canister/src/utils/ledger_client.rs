use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call::CallResult;

// ============================================================================
// ICRC-1 Ledger Types (ckBTC and ckUSDC use ICRC-1 standard)
// ============================================================================

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

// ============================================================================
// Ledger Client
// ============================================================================

pub struct LedgerClient {
    canister_id: Principal,
    decimals: u8,
}

impl LedgerClient {
    pub fn new(canister_id: Principal, decimals: u8) -> Self {
        Self { canister_id, decimals }
    }

    /// Get balance for an account
    pub async fn balance_of(&self, account: Account) -> Result<u64, String> {
        let result: CallResult<(Nat,)> = ic_cdk::call(
            self.canister_id,
            "icrc1_balance_of",
            (account,),
        ).await;

        match result {
            Ok((balance,)) => {
                // Convert Nat to u64
                let balance_u64 = balance.0.to_u64_digits();
                if balance_u64.is_empty() {
                    Ok(0)
                } else {
                    Ok(balance_u64[0])
                }
            }
            Err((code, msg)) => Err(format!("Balance query failed: {:?} - {}", code, msg)),
        }
    }

    /// Transfer tokens
    pub async fn transfer(
        &self,
        from_subaccount: Option<Vec<u8>>,
        to: Account,
        amount: u64,
        memo: Option<Vec<u8>>,
    ) -> Result<u64, String> {
        let transfer_arg = TransferArg {
            from_subaccount,
            to,
            amount: Nat::from(amount),
            fee: None, // Let ledger use default fee
            memo,
            created_at_time: Some(ic_cdk::api::time()),
        };

        let result: CallResult<(TransferResult,)> = ic_cdk::call(
            self.canister_id,
            "icrc1_transfer",
            (transfer_arg,),
        ).await;

        match result {
            Ok((Ok(block_index),)) => {
                let block_u64 = block_index.0.to_u64_digits();
                if block_u64.is_empty() {
                    Ok(0)
                } else {
                    Ok(block_u64[0])
                }
            }
            Ok((Err(e),)) => Err(format!("Transfer failed: {:?}", e)),
            Err((code, msg)) => Err(format!("Transfer call failed: {:?} - {}", code, msg)),
        }
    }

    /// Get fee
    pub async fn get_fee(&self) -> Result<u64, String> {
        let result: CallResult<(Nat,)> = ic_cdk::call(
            self.canister_id,
            "icrc1_fee",
            (),
        ).await;

        match result {
            Ok((fee,)) => {
                let fee_u64 = fee.0.to_u64_digits();
                if fee_u64.is_empty() {
                    Ok(0)
                } else {
                    Ok(fee_u64[0])
                }
            }
            Err((code, msg)) => Err(format!("Fee query failed: {:?} - {}", code, msg)),
        }
    }

    /// Convert amount to smallest unit (e.g., satoshis for BTC, micro-USDC for USDC)
    pub fn to_smallest_unit(&self, amount: f64) -> u64 {
        (amount * 10_f64.powi(self.decimals as i32)) as u64
    }

    /// Convert from smallest unit to decimal
    pub fn from_smallest_unit(&self, amount: u64) -> f64 {
        (amount as f64) / 10_f64.powi(self.decimals as i32)
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get ckBTC ledger canister ID
pub fn get_ckbtc_ledger_id() -> Result<Principal, String> {
    std::env::var("CKBTC_LEDGER_ID")
        .ok()
        .and_then(|id| Principal::from_text(&id).ok())
        .ok_or_else(|| "CKBTC_LEDGER_ID not set".to_string())
}

/// Get ckUSDC ledger canister ID
pub fn get_ckusdc_ledger_id() -> Result<Principal, String> {
    std::env::var("CKUSDC_LEDGER_ID")
        .ok()
        .and_then(|id| Principal::from_text(&id).ok())
        .ok_or_else(|| "CKUSDC_LEDGER_ID not set".to_string())
}

/// Create ckBTC ledger client
pub fn create_ckbtc_client() -> Result<LedgerClient, String> {
    let canister_id = get_ckbtc_ledger_id()?;
    Ok(LedgerClient::new(canister_id, 8)) // ckBTC has 8 decimals (satoshis)
}

/// Create ckUSDC ledger client
pub fn create_ckusdc_client() -> Result<LedgerClient, String> {
    let canister_id = get_ckusdc_ledger_id()?;
    Ok(LedgerClient::new(canister_id, 6)) // ckUSDC has 6 decimals (micro-USDC)
}

/// Get account for a user (using their principal or phone-derived principal)
pub fn get_user_account(user_principal: Principal) -> Account {
    Account {
        owner: user_principal,
        subaccount: None,
    }
}

/// Get account for USSD canister (for holding user funds)
pub fn get_canister_account(subaccount: Option<Vec<u8>>) -> Account {
    Account {
        owner: ic_cdk::id(),
        subaccount,
    }
}

/// Derive subaccount from phone number (for USSD users without principals)
pub fn derive_subaccount_from_phone(phone: &str) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(phone.as_bytes());
    let hash = hasher.finalize();
    
    // Take first 32 bytes for subaccount
    hash[..32].to_vec()
}
