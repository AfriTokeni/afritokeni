use candid::{Encode, CandidType, Deserialize};

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

fn main() {
    let user_data = CreateUserData {
        user_type: UserType::User,
        preferred_currency: FiatCurrency::UGX,
        email: "test@example.com".to_string(),
        first_name: "John Doe".to_string(),
        last_name: "Doe".to_string(),
        principal_id: Some("2vxsx-fae".to_string()),
        phone_number: Some("+256700123456".to_string()),
    };
    
    let encoded = Encode!(&user_data).unwrap();
    println!("Encoded hex: {}", hex::encode(&encoded));
    println!("Encoded bytes: {:?}", encoded);
}
