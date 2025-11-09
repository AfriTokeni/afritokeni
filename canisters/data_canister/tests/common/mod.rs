// Common test utilities for Data Canister

pub mod test_data {
    pub const USER_ID_1: &str = "user_001";
    pub const USER_ID_2: &str = "user_002";
    pub const PHONE_1: &str = "+254712345678";
    pub const PHONE_2: &str = "+256700123456";
}

pub mod test_balances {
    pub const INITIAL_BALANCE: u64 = 10000;
    pub const ZERO_BALANCE: u64 = 0;
    pub const LARGE_BALANCE: u64 = 1000000;
}
