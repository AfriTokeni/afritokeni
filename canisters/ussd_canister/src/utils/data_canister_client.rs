use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call::Call;

// ============================================================================
// Data Canister Types (matching data_canister/src/models.rs)
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum FiatCurrency {
    UGX,  // Uganda Shilling (LAUNCH CURRENCY)
    KES,  // Kenyan Shilling
    NGN,  // Nigerian Naira
    GHS,  // Ghanaian Cedi
    ZAR,  // South African Rand
    TZS,  // Tanzanian Shilling
    // Add more as needed
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserType {
    User,
    Agent,
    Admin,
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum KYCStatus {
    NotStarted,
    Pending,
    Approved,
    Rejected,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency,
    pub kyc_status: KYCStatus,
    pub is_verified: bool,
    pub created_at: u64,
    pub last_active: u64,
}

#[derive(CandidType, Deserialize)]
pub struct CreateUserData {
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
}

// ============================================================================
// Data Canister Client
// ============================================================================

pub struct DataCanisterClient {
    canister_id: Principal,
}

impl DataCanisterClient {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }

    /// Create a new user
    pub async fn create_user(&self, user_data: CreateUserData) -> Result<User, String> {
        let response = Call::unbounded_wait(self.canister_id, "create_user")
            .with_arg((user_data,))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<User, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Get user by phone number
    pub async fn get_user_by_phone(&self, phone_number: &str) -> Result<Option<User>, String> {
        let response = Call::unbounded_wait(self.canister_id, "get_user_by_phone")
            .with_arg((phone_number.to_string(),))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<Option<User>, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Get fiat balance
    pub async fn get_fiat_balance(&self, user_id: &str, currency: FiatCurrency) -> Result<u64, String> {
        let response = Call::unbounded_wait(self.canister_id, "get_fiat_balance")
            .with_arg((user_id.to_string(), currency))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<u64, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Deposit fiat
    pub async fn deposit_fiat(
        &self,
        user_id: &str,
        amount: u64,
        currency: FiatCurrency,
        description: Option<String>,
    ) -> Result<Transaction, String> {
        let response = Call::unbounded_wait(self.canister_id, "deposit_fiat")
            .with_arg((user_id.to_string(), amount, currency, description))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<Transaction, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Transfer fiat between users
    pub async fn transfer_fiat(
        &self,
        from_user: &str,
        to_user: &str,
        amount: u64,
        currency: FiatCurrency,
        description: Option<String>,
    ) -> Result<Transaction, String> {
        let response = Call::unbounded_wait(self.canister_id, "transfer_fiat")
            .with_arg((from_user.to_string(), to_user.to_string(), amount, currency, description))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<Transaction, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Setup user PIN
    pub async fn setup_user_pin(&self, user_id: &str, pin: &str) -> Result<(), String> {
        // Generate salt
        let salt = self.generate_salt().await?;
        
        let response = Call::unbounded_wait(self.canister_id, "setup_user_pin")
            .with_arg((user_id.to_string(), pin.to_string(), salt))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<(), String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Verify user PIN
    pub async fn verify_user_pin(&self, user_id: &str, pin: &str) -> Result<bool, String> {
        let response = Call::unbounded_wait(self.canister_id, "verify_user_pin")
            .with_arg((user_id.to_string(), pin.to_string()))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<bool, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Generate random salt for PIN hashing
    async fn generate_salt(&self) -> Result<String, String> {
        let random_bytes = ic_cdk::api::management_canister::main::raw_rand()
            .await
            .map_err(|e| format!("Failed to generate random bytes: {:?}", e))?
            .0;
        
        let mut salt = [0u8; 16];
        random_bytes.iter().take(16).enumerate().for_each(|(i, &b)| salt[i] = b);
        
        Ok(hex::encode(salt))
    }

    /// Update crypto balance (after ledger operations)
    pub async fn update_crypto_balance(
        &self,
        user_id: &str,
        ckbtc_delta: i64,
        ckusdc_delta: i64,
    ) -> Result<(), String> {
        let response = Call::unbounded_wait(self.canister_id, "update_crypto_balance")
            .with_arg((user_id.to_string(), ckbtc_delta, ckusdc_delta))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<(), String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        result
    }

    /// Get crypto balance
    pub async fn get_crypto_balance(&self, user_id: &str) -> Result<(u64, u64), String> {
        #[derive(CandidType, Deserialize)]
        struct CryptoBalance {
            ckbtc: u64,
            ckusdc: u64,
        }

        let response = Call::unbounded_wait(self.canister_id, "get_crypto_balance")
            .with_arg((user_id.to_string(),))
            .await
            .map_err(|e| format!("Call failed: {:?}", e))?;
        
        let (result,): (Result<CryptoBalance, String>,) = response
            .candid_tuple()
            .map_err(|e| format!("Decode failed: {}", e))?;
        
        let balance = result?;
        Ok((balance.ckbtc, balance.ckusdc))
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get data canister ID from environment or config
pub fn get_data_canister_id() -> Result<Principal, String> {
    // In production, this should come from dfx canister ID or environment variable
    // For now, we'll use a placeholder that will be set during deployment
    std::env::var("DATA_CANISTER_ID")
        .ok()
        .and_then(|id| Principal::from_text(&id).ok())
        .ok_or_else(|| "DATA_CANISTER_ID not set".to_string())
}

/// Create a data canister client instance
pub fn create_client() -> Result<DataCanisterClient, String> {
    let canister_id = get_data_canister_id()?;
    Ok(DataCanisterClient::new(canister_id))
}
