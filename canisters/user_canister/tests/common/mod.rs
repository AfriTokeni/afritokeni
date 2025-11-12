// Common test utilities for Business Logic Canister

/// Test user data
pub mod test_users {
    pub const PHONE_KENYA: &str = "+254712345678";
    pub const PHONE_UGANDA: &str = "+256700123456";
    pub const PHONE_TANZANIA: &str = "+255712345678";
    pub const TEST_PIN: &str = "1234";
    pub const TEST_EMAIL: &str = "test@afritokeni.com";
}

/// Test amounts
#[allow(dead_code)]
pub mod test_amounts {
    pub const SMALL: u64 = 100;
    pub const MEDIUM: u64 = 1000;
    pub const LARGE: u64 = 10000;
    pub const VERY_LARGE: u64 = 1000000;
}

/// Test currencies
pub mod test_currencies {
    pub const KES: &str = "KES";
    pub const UGX: &str = "UGX";
    pub const TZS: &str = "TZS";
    pub const NGN: &str = "NGN";
}
