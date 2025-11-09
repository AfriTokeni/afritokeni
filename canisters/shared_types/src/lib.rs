use candid::{CandidType, Deserialize};

// ============================================================================
// Shared Types - Used by ALL canisters
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserType {
    User,
    Admin,
    Agent,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
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
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum KYCStatus {
    NotStarted,
    Pending,
    Approved,
    Rejected,
}

/// CreateUserData - MUST match Data Canister's definition EXACTLY
/// Field order is critical for Candid encoding!
#[derive(CandidType, Deserialize)]
pub struct CreateUserData {
    pub user_type: UserType,
    pub preferred_currency: FiatCurrency,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub principal_id: Option<String>,
    pub phone_number: Option<String>,
}

/// User - MUST match Data Canister's definition EXACTLY
#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub phone_number: Option<String>,
    pub principal_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_currency: FiatCurrency,
    pub user_type: UserType,
    pub kyc_status: KYCStatus,
    pub is_verified: bool,
    pub created_at: u64,
    pub last_active: u64,
}
