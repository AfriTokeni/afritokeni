// Configuration loader - reads from config.toml at compile time
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub rate_limiting: RateLimitingConfig,
    pub pin_security: PinSecurityConfig,
    pub transaction_limits: TransactionLimitsConfig,
    pub session: SessionConfig,
    pub security: SecurityConfig,
    pub africas_talking: AfricasTalkingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitingConfig {
    pub max_requests_per_minute: u32,
    pub rate_limit_window_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PinSecurityConfig {
    pub max_pin_attempts: u32,
    pub lockout_duration_minutes: u64,
    pub min_pin_length: usize,
    pub max_pin_length: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TransactionLimitsConfig {
    pub min_amount_kes: f64,
    pub max_amount_kes: f64,
    pub min_amount_btc: f64,
    pub max_amount_btc: f64,
    pub min_amount_usdc: f64,
    pub max_amount_usdc: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SessionConfig {
    pub timeout_minutes: u64,
    pub max_active_sessions: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub allowed_user_agents: String,
    pub verify_signature: bool,
    pub hmac_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AfricasTalkingConfig {
    pub username: String,
    pub api_key: String,
    pub api_url: String,
    pub is_sandbox: bool,
}

// Load config at compile time
static CONFIG_STR: &str = include_str!("../config.toml");

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = toml::from_str(CONFIG_STR)
        .expect("Failed to parse config.toml");
}

pub fn get_config() -> &'static Config {
    &CONFIG
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads() {
        let config = get_config();
        assert_eq!(config.rate_limiting.max_requests_per_minute, 10);
        assert_eq!(config.pin_security.max_pin_attempts, 3);
    }
}
