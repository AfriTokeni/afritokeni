use candid::{CandidType, Deserialize};

// ============================================================================
// Public Modules - Shared functionality across all canisters
// ============================================================================

pub mod audit;

// ============================================================================
// Shared Types - Used by ALL canisters
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserType {
    User,
    Admin,
    Agent,
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FiatCurrency {
    AOA, BIF, BWP, CDF, CVE, DJF, DZD, EGP, ERN, ETB,
    GHS, GMD, KES, KMF, LRD, LSL, LYD, MAD, MGA, MRU,
    MUR, MWK, NAD, NGN, RWF, SCR, SDG, SLL, SOS, SSP,
    STN, SZL, TND, TZS, UGX, XAF, XOF, ZAR, ZMW
}

impl FiatCurrency {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "AOA" => Ok(FiatCurrency::AOA),
            "BIF" => Ok(FiatCurrency::BIF),
            "BWP" => Ok(FiatCurrency::BWP),
            "CDF" => Ok(FiatCurrency::CDF),
            "CVE" => Ok(FiatCurrency::CVE),
            "DJF" => Ok(FiatCurrency::DJF),
            "DZD" => Ok(FiatCurrency::DZD),
            "EGP" => Ok(FiatCurrency::EGP),
            "ERN" => Ok(FiatCurrency::ERN),
            "ETB" => Ok(FiatCurrency::ETB),
            "GHS" => Ok(FiatCurrency::GHS),
            "GMD" => Ok(FiatCurrency::GMD),
            "KES" => Ok(FiatCurrency::KES),
            "KMF" => Ok(FiatCurrency::KMF),
            "LRD" => Ok(FiatCurrency::LRD),
            "LSL" => Ok(FiatCurrency::LSL),
            "LYD" => Ok(FiatCurrency::LYD),
            "MAD" => Ok(FiatCurrency::MAD),
            "MGA" => Ok(FiatCurrency::MGA),
            "MRU" => Ok(FiatCurrency::MRU),
            "MUR" => Ok(FiatCurrency::MUR),
            "MWK" => Ok(FiatCurrency::MWK),
            "NAD" => Ok(FiatCurrency::NAD),
            "NGN" => Ok(FiatCurrency::NGN),
            "RWF" => Ok(FiatCurrency::RWF),
            "SCR" => Ok(FiatCurrency::SCR),
            "SDG" => Ok(FiatCurrency::SDG),
            "SLL" => Ok(FiatCurrency::SLL),
            "SOS" => Ok(FiatCurrency::SOS),
            "SSP" => Ok(FiatCurrency::SSP),
            "STN" => Ok(FiatCurrency::STN),
            "SZL" => Ok(FiatCurrency::SZL),
            "TND" => Ok(FiatCurrency::TND),
            "TZS" => Ok(FiatCurrency::TZS),
            "UGX" => Ok(FiatCurrency::UGX),
            "XAF" => Ok(FiatCurrency::XAF),
            "XOF" => Ok(FiatCurrency::XOF),
            "ZAR" => Ok(FiatCurrency::ZAR),
            "ZMW" => Ok(FiatCurrency::ZMW),
            _ => Err(format!("Unsupported currency: {}", s)),
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    
    pub fn code(&self) -> &'static str {
        match self {
            FiatCurrency::AOA => "AOA",
            FiatCurrency::BIF => "BIF",
            FiatCurrency::BWP => "BWP",
            FiatCurrency::CDF => "CDF",
            FiatCurrency::CVE => "CVE",
            FiatCurrency::DJF => "DJF",
            FiatCurrency::DZD => "DZD",
            FiatCurrency::EGP => "EGP",
            FiatCurrency::ERN => "ERN",
            FiatCurrency::ETB => "ETB",
            FiatCurrency::GHS => "GHS",
            FiatCurrency::GMD => "GMD",
            FiatCurrency::KES => "KES",
            FiatCurrency::KMF => "KMF",
            FiatCurrency::LRD => "LRD",
            FiatCurrency::LSL => "LSL",
            FiatCurrency::LYD => "LYD",
            FiatCurrency::MAD => "MAD",
            FiatCurrency::MGA => "MGA",
            FiatCurrency::MRU => "MRU",
            FiatCurrency::MUR => "MUR",
            FiatCurrency::MWK => "MWK",
            FiatCurrency::NAD => "NAD",
            FiatCurrency::NGN => "NGN",
            FiatCurrency::RWF => "RWF",
            FiatCurrency::SCR => "SCR",
            FiatCurrency::SDG => "SDG",
            FiatCurrency::SLL => "SLL",
            FiatCurrency::SOS => "SOS",
            FiatCurrency::SSP => "SSP",
            FiatCurrency::STN => "STN",
            FiatCurrency::SZL => "SZL",
            FiatCurrency::TND => "TND",
            FiatCurrency::TZS => "TZS",
            FiatCurrency::UGX => "UGX",
            FiatCurrency::XAF => "XAF",
            FiatCurrency::XOF => "XOF",
            FiatCurrency::ZAR => "ZAR",
            FiatCurrency::ZMW => "ZMW",
        }
    }
    
    pub fn from_code(code: &str) -> Option<Self> {
        Self::from_string(code).ok()
    }
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum KYCStatus {
    NotStarted,
    Pending,
    Approved,
    Rejected,
}

/// Crypto currency types
#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptoType {
    CkBTC,
    CkUSDC,
}

/// CreateUserData - MUST match Data Canister's definition EXACTLY
/// Field order is critical for Candid encoding!
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateUserData {
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency, 
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub principal_id: Option<String>,
    pub phone_number: Option<String>,
}

/// User - MUST match Data Canister's Candid definition EXACTLY (including field order!)
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency,
    pub created_at: u64,
    pub last_active: u64,
    pub email: String,
    pub is_verified: bool,
    pub kyc_status: KYCStatus,
    pub first_name: String,
    pub last_name: String,
    pub principal_id: Option<String>,
    pub phone_number: Option<String>,
}

// Helper to deserialize numbers from strings (Candid JSON format)
#[allow(dead_code)]
fn deserialize_number_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)?
        .parse()
        .map_err(D::Error::custom)
}

// ============================================================================
// Business Logic API Types - Shared between USSD and Business Logic canisters
// ============================================================================

/// Register user request - SINGLE SOURCE OF TRUTH
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RegisterUserRequest {
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: String,  // String instead of enum to avoid Candid issues
    pub pin: String,
}

/// Simplified fiat balance info for API responses
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FiatBalanceInfo {
    pub currency: String,
    pub balance: u64,
}

/// User balances response
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserBalances {
    pub fiat_balances: Vec<FiatBalanceInfo>,
    pub ckbtc_balance: u64,
    pub ckusdc_balance: u64,
}

/// Transaction result returned from money transfer operations
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

/// Swap result returned from crypto swap operations
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

// ============================================================================
// Balance Models - SHARED BETWEEN ALL CANISTERS
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FiatBalance {
    pub user_id: String,
    pub currency: FiatCurrency,
    pub balance: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CryptoBalance {
    pub user_id: String,
    pub ckbtc: u64,
    pub ckusdc: u64,
    pub updated_at: u64,
}

// ============================================================================
// Transaction Models - SHARED BETWEEN ALL CANISTERS
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransactionType {
    DepositFiat,
    WithdrawFiat,
    TransferFiat,
    BuyCrypto,
    SellCrypto,
    TransferCrypto,
    SwapCrypto,
    EscrowCreate,
    EscrowClaim,
    EscrowCancel,
    AgentCommission,
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub amount: u64,
    pub currency_type: CurrencyType,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub description: Option<String>,
}

/// TransactionRecord - simplified version for business logic layer
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransactionRecord {
    pub id: String,
    pub transaction_type: String,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub amount: u64,
    pub currency: String,
    pub timestamp: u64,
    pub status: String,
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CurrencyType {
    Fiat(FiatCurrency),
    Crypto(CryptoType),
}

// ============================================================================
// PIN Security Models - SHARED BETWEEN ALL CANISTERS
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserPin {
    pub user_id: String,
    pub pin_hash: String,
    pub salt: String,
    pub failed_attempts: u32,
    pub locked_until: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

// ============================================================================
// Audit Models - SHARED BETWEEN ALL CANISTERS
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: String,
    // Principal text of the caller that triggered the event (canister or user)
    pub caller: String,
    pub user_id: Option<String>,
    pub details: String,
    // Whether the operation succeeded (true) or failed/was denied (false)
    pub success: bool,
}


// ============================================================================
// Settlement Models - SHARED
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MonthlySettlement {
    pub month: String,             // e.g. "2025-11"
    pub agent_principal: String,   // principal text
    pub total_commission: u64,
    pub paid: bool,
    pub paid_date: Option<u64>,    // seconds since epoch
}

// ============================================================================
// Agent Operations Types (Deposits & Withdrawals)
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AgentTransactionStatus {
    Pending,
    Confirmed,
    Cancelled,
    Expired,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DepositTransaction {
    pub id: String,
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub agent_commission: u64,
    pub agent_keeps: u64,
    pub platform_revenue: u64,
    pub deposit_code: String,
    pub status: AgentTransactionStatus,
    pub timestamp: u64,
    pub confirmed_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WithdrawalTransaction {
    pub id: String,
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub agent_fee: u64,
    pub agent_keeps: u64,
    pub platform_revenue: u64,
    pub withdrawal_code: String,
    pub status: AgentTransactionStatus,
    pub timestamp: u64,
    pub confirmed_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AgentBalance {
    pub agent_id: String,
    pub currency: String,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub commission_earned: u64,
    pub commission_paid: u64,
    pub outstanding_balance: i64,  // NEW: Can be negative (owes platform) or positive (platform owes agent)
    pub credit_limit: u64,          // NEW: Maximum negative outstanding balance allowed
    pub last_settlement_date: Option<u64>,
    pub last_updated: u64,
}

/// Agent tier determines credit limit
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AgentTier {
    New,       // 1M credit limit
    Trusted,   // 5M credit limit
    Premium,   // 10M credit limit
}

impl AgentTier {
    /// Get default credit limit for this tier
    /// NOTE: Actual limits should be loaded from agent_config.toml
    /// These are fallback values only
    pub fn default_credit_limit(&self) -> u64 {
        match self {
            AgentTier::New => 1_000_000,      // 1M - matches agent_config.toml
            AgentTier::Trusted => 5_000_000,  // 5M - matches agent_config.toml
            AgentTier::Premium => 10_000_000, // 10M - matches agent_config.toml
        }
    }
}

/// Agent credit configuration per currency
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AgentCreditConfig {
    pub agent_id: String,
    pub currency: String,
    pub tier: AgentTier,
    pub credit_limit: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

// ============================================================================
// Data Canister Request Types - For proper non-deprecated API usage
// =========================================================================

/// Request to create a user (internal - string types for inter-canister calls)
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateUserRequest {
    pub user_type_str: String,
    pub preferred_currency_str: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub principal_id: Option<String>,
    pub phone_number: Option<String>,
}

/// Request to setup user PIN
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SetupPinRequest {
    pub user_id: String,
    pub pin: String,
    pub salt: String,
}

/// Request to get fiat balance
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetFiatBalanceRequest {
    pub user_id: String,
    pub currency: String,
}

/// Request to withdraw fiat
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WithdrawFiatRequest {
    pub user_id: String,
    pub amount: u64,
    pub currency: String,
    pub description: Option<String>,
}

/// Request to set fiat balance
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct SetFiatBalanceRequest {
    pub user_id: String,
    pub currency: String,
    pub amount: u64,
}

/// Request to verify PIN
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct VerifyPinRequest {
    pub user_id: String,
    pub pin: String,
}

/// Request to update crypto balance
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdateCryptoBalanceRequest {
    pub user_id: String,
    pub ckbtc_delta: i64,
    pub ckusdc_delta: i64,
}

/// Request to get user transactions
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetUserTransactionsRequest {
    pub user_id: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Request to change PIN
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ChangePinRequest {
    pub user_id: String,
    pub old_pin: String,
    pub new_pin: String,
    pub new_salt: String,
}

/// Request to update user phone
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdateUserPhoneRequest {
    pub user_id: String,
    pub phone_number: String,
}

// ============================================================================
// Escrow Models - SHARED BETWEEN ALL CANISTERS
// ============================================================================

/// Escrow status
#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EscrowStatus {
    Active,
    Claimed,
    Expired,
    Cancelled,
}

/// Escrow record
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Escrow {
    pub code: String,
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub crypto_type: CryptoType,
    pub status: EscrowStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub claimed_at: Option<u64>,
}

/// Request to create escrow
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateEscrowRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub crypto_type: CryptoType,
}

/// Request to verify and claim escrow
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ClaimEscrowRequest {
    pub code: String,
    pub agent_id: String,
}
