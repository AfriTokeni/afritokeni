use candid::{CandidType, Deserialize};

// ============================================================================
// Currency Enums - Type-safe, extensible for 39 African currencies
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FiatCurrency {
    // East Africa
    UGX,  // Uganda Shilling (LAUNCH CURRENCY)
    KES,  // Kenyan Shilling
    TZS,  // Tanzanian Shilling
    RWF,  // Rwandan Franc
    BIF,  // Burundian Franc
    
    // West Africa
    NGN,  // Nigerian Naira
    GHS,  // Ghanaian Cedi
    XOF,  // West African CFA Franc
    GMD,  // Gambian Dalasi
    SLL,  // Sierra Leonean Leone
    LRD,  // Liberian Dollar
    
    // Southern Africa
    ZAR,  // South African Rand
    BWP,  // Botswana Pula
    LSL,  // Lesotho Loti
    SZL,  // Swazi Lilangeni
    NAD,  // Namibian Dollar
    ZMW,  // Zambian Kwacha
    MWK,  // Malawian Kwacha
    
    // North Africa
    EGP,  // Egyptian Pound
    MAD,  // Moroccan Dirham
    TND,  // Tunisian Dinar
    DZD,  // Algerian Dinar
    LYD,  // Libyan Dinar
    
    // Central Africa
    XAF,  // Central African CFA Franc
    CDF,  // Congolese Franc
    AOA,  // Angolan Kwanza
    
    // Other African Currencies
    ETB,  // Ethiopian Birr
    SOS,  // Somali Shilling
    SDG,  // Sudanese Pound
    SSP,  // South Sudanese Pound
    DJF,  // Djiboutian Franc
    ERN,  // Eritrean Nakfa
    MUR,  // Mauritian Rupee
    SCR,  // Seychellois Rupee
    MGA,  // Malagasy Ariary
    KMF,  // Comorian Franc
    CVE,  // Cape Verdean Escudo
    STN,  // São Tomé and Príncipe Dobra
    MRU,  // Mauritanian Ouguiya
}

impl FiatCurrency {
    pub fn code(&self) -> &'static str {
        match self {
            FiatCurrency::UGX => "UGX",
            FiatCurrency::KES => "KES",
            FiatCurrency::TZS => "TZS",
            FiatCurrency::RWF => "RWF",
            FiatCurrency::BIF => "BIF",
            FiatCurrency::NGN => "NGN",
            FiatCurrency::GHS => "GHS",
            FiatCurrency::XOF => "XOF",
            FiatCurrency::GMD => "GMD",
            FiatCurrency::SLL => "SLL",
            FiatCurrency::LRD => "LRD",
            FiatCurrency::ZAR => "ZAR",
            FiatCurrency::BWP => "BWP",
            FiatCurrency::LSL => "LSL",
            FiatCurrency::SZL => "SZL",
            FiatCurrency::NAD => "NAD",
            FiatCurrency::ZMW => "ZMW",
            FiatCurrency::MWK => "MWK",
            FiatCurrency::EGP => "EGP",
            FiatCurrency::MAD => "MAD",
            FiatCurrency::TND => "TND",
            FiatCurrency::DZD => "DZD",
            FiatCurrency::LYD => "LYD",
            FiatCurrency::XAF => "XAF",
            FiatCurrency::CDF => "CDF",
            FiatCurrency::AOA => "AOA",
            FiatCurrency::ETB => "ETB",
            FiatCurrency::SOS => "SOS",
            FiatCurrency::SDG => "SDG",
            FiatCurrency::SSP => "SSP",
            FiatCurrency::DJF => "DJF",
            FiatCurrency::ERN => "ERN",
            FiatCurrency::MUR => "MUR",
            FiatCurrency::SCR => "SCR",
            FiatCurrency::MGA => "MGA",
            FiatCurrency::KMF => "KMF",
            FiatCurrency::CVE => "CVE",
            FiatCurrency::STN => "STN",
            FiatCurrency::MRU => "MRU",
        }
    }
    
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "UGX" => Some(FiatCurrency::UGX),
            "KES" => Some(FiatCurrency::KES),
            "TZS" => Some(FiatCurrency::TZS),
            "RWF" => Some(FiatCurrency::RWF),
            "BIF" => Some(FiatCurrency::BIF),
            "NGN" => Some(FiatCurrency::NGN),
            "GHS" => Some(FiatCurrency::GHS),
            "XOF" => Some(FiatCurrency::XOF),
            "GMD" => Some(FiatCurrency::GMD),
            "SLL" => Some(FiatCurrency::SLL),
            "LRD" => Some(FiatCurrency::LRD),
            "ZAR" => Some(FiatCurrency::ZAR),
            "BWP" => Some(FiatCurrency::BWP),
            "LSL" => Some(FiatCurrency::LSL),
            "SZL" => Some(FiatCurrency::SZL),
            "NAD" => Some(FiatCurrency::NAD),
            "ZMW" => Some(FiatCurrency::ZMW),
            "MWK" => Some(FiatCurrency::MWK),
            "EGP" => Some(FiatCurrency::EGP),
            "MAD" => Some(FiatCurrency::MAD),
            "TND" => Some(FiatCurrency::TND),
            "DZD" => Some(FiatCurrency::DZD),
            "LYD" => Some(FiatCurrency::LYD),
            "XAF" => Some(FiatCurrency::XAF),
            "CDF" => Some(FiatCurrency::CDF),
            "AOA" => Some(FiatCurrency::AOA),
            "ETB" => Some(FiatCurrency::ETB),
            "SOS" => Some(FiatCurrency::SOS),
            "SDG" => Some(FiatCurrency::SDG),
            "SSP" => Some(FiatCurrency::SSP),
            "DJF" => Some(FiatCurrency::DJF),
            "ERN" => Some(FiatCurrency::ERN),
            "MUR" => Some(FiatCurrency::MUR),
            "SCR" => Some(FiatCurrency::SCR),
            "MGA" => Some(FiatCurrency::MGA),
            "KMF" => Some(FiatCurrency::KMF),
            "CVE" => Some(FiatCurrency::CVE),
            "STN" => Some(FiatCurrency::STN),
            "MRU" => Some(FiatCurrency::MRU),
            _ => None,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CryptoCurrency {
    CkBTC,   // ICP-native Bitcoin
    CkUSDC,  // ICP-native USDC
}

// ============================================================================
// User Models
// ============================================================================

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
    pub id: String,                          // UUID
    pub phone_number: Option<String>,        // For USSD users
    pub principal_id: Option<String>,        // For Web users (Internet Identity)
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency,    // User's local currency
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

// ============================================================================
// Balance Models
// ============================================================================

#[derive(CandidType, Deserialize, Clone)]
pub struct FiatBalance {
    pub user_id: String,
    pub currency: FiatCurrency,
    pub balance: u64,  // Amount in smallest unit (cents)
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CryptoBalance {
    pub user_id: String,
    pub ckbtc: u64,   // Satoshis (1 BTC = 100,000,000 satoshis)
    pub ckusdc: u64,  // Micro-USDC (1 USDC = 1,000,000 micro-USDC)
    pub updated_at: u64,
}

// ============================================================================
// Transaction Models
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransactionType {
    // Fiat operations
    DepositFiat,
    WithdrawFiat,
    TransferFiat,
    
    // Crypto operations
    BuyCrypto,    // Fiat → Crypto (via agent)
    SellCrypto,   // Crypto → Fiat (via agent)
    TransferCrypto,
    
    // Agent operations
    AgentCommission,
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub from_user: Option<String>,
    pub to_user: Option<String>,
    pub amount: u64,
    pub currency_type: CurrencyType,  // Fiat or Crypto
    pub status: TransactionStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub description: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CurrencyType {
    Fiat(FiatCurrency),
    Crypto(CryptoCurrency),
}

// ============================================================================
// PIN Security Models
// ============================================================================

#[derive(CandidType, Deserialize, Clone)]
pub struct UserPin {
    pub user_id: String,
    pub pin_hash: String,      // HMAC-SHA256 hash
    pub salt: String,          // Hex-encoded salt
    pub failed_attempts: u32,
    pub locked_until: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

// ============================================================================
// Audit Models
// ============================================================================

#[derive(CandidType, Deserialize, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: String,
    pub user_id: Option<String>,
    pub details: String,
}

// ============================================================================
// System Stats
// ============================================================================

#[derive(CandidType, Deserialize)]
pub struct SystemStats {
    pub total_users: usize,
    pub total_transactions: usize,
    pub total_fiat_balances: usize,
    pub total_crypto_balances: usize,
}

// ============================================================================
// Helper Implementations
// ============================================================================

impl User {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
