use candid::{CandidType, Deserialize};

// ============================================================================
// Currency Enums - Import from shared_types for consistency
// ============================================================================

pub use shared_types::FiatCurrency;

// OLD LOCAL DEFINITION - REMOVED TO USE SHARED_TYPES
/*
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
*/

// OLD IMPL BLOCK - REMOVED, NOW IN SHARED_TYPES
/*
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
*/

// ============================================================================
// ALL MODELS NOW IN SHARED_TYPES - SINGLE SOURCE OF TRUTH
// ============================================================================

pub use shared_types::{
    // User types
    UserType, KYCStatus, User, CreateUserData,
    // Balance types
    FiatBalance, CryptoBalance,
    // Transaction types
    Transaction, TransactionType, TransactionStatus, CurrencyType,
    // Note: CryptoType is re-exported via CurrencyType enum variant, not used directly here
    // Security types
    UserPin,
    // Audit types
    AuditEntry, MonthlySettlement,
    // Escrow types
    Escrow, EscrowStatus,
    // Agent types
    DepositTransaction, WithdrawalTransaction, AgentBalance, AgentTransactionStatus,
    // Agent Profile types (composition pattern - separate from User)
    AgentProfile, AgentStatus, LocationData, CreateAgentProfileRequest, UpdateAgentProfileRequest,
};

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
// User helper methods will be added here when needed
