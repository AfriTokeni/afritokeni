// Common test utilities for Withdrawal Canister

pub mod test_data {
    #[allow(dead_code)]
    pub const USER_ID: &str = "user_001";
    #[allow(dead_code)]
    pub const AGENT_ID: &str = "agent_001";
    #[allow(dead_code)]
    pub const PHONE: &str = "+254712345678";
}

pub mod test_amounts {
    pub const MIN_WITHDRAWAL: u64 = 10;
    #[allow(dead_code)]
    pub const NORMAL_WITHDRAWAL: u64 = 1000;
    #[allow(dead_code)]
    pub const LARGE_WITHDRAWAL: u64 = 100000;
    pub const MAX_WITHDRAWAL: u64 = 1000000;
}
