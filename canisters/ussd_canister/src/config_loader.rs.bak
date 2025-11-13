// Configuration loader - reads from config.toml at compile time
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub rate_limiting: RateLimitingConfig,
    pub pin_security: PinSecurityConfig,
    pub transaction_limits: TransactionLimitsConfig,
    pub session: SessionConfig,
    pub security: SecurityConfig,
    pub africas_talking: AfricasTalkingConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct RateLimitingConfig {
    pub max_requests_per_minute: u32,
    pub rate_limit_window_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PinSecurityConfig {
    pub max_pin_attempts: u32,
    pub lockout_duration_minutes: u64,
    pub min_pin_length: usize,
    pub max_pin_length: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct TransactionLimitsConfig {
    pub min_amount_kes: f64,
    pub max_amount_kes: f64,
    pub min_amount_btc: f64,
    pub max_amount_btc: f64,
    pub min_amount_usdc: f64,
    pub max_amount_usdc: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SessionConfig {
    pub timeout_minutes: u64,
    pub max_active_sessions: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SecurityConfig {
    pub allowed_user_agents: String,
    pub verify_signature: bool,
    pub hmac_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct AfricasTalkingConfig {
    pub username: String,
    pub api_key: String,
    pub api_url: String,
    pub is_sandbox: bool,
}

// Default config for when TOML parsing fails (e.g., in tests)
fn get_default_config() -> Config {
    Config {
        rate_limiting: RateLimitingConfig {
            max_requests_per_minute: 1000,
            rate_limit_window_seconds: 60,
        },
        pin_security: PinSecurityConfig {
            max_pin_attempts: 3,
            lockout_duration_minutes: 30,
            min_pin_length: 4,
            max_pin_length: 4,
        },
        transaction_limits: TransactionLimitsConfig {
            min_amount_kes: 10.0,
            max_amount_kes: 1_000_000.0,
            min_amount_btc: 0.00001,
            max_amount_btc: 1.0,
            min_amount_usdc: 1.0,
            max_amount_usdc: 100_000.0,
        },
        session: SessionConfig {
            timeout_minutes: 5,
            max_active_sessions: 1000,
        },
        security: SecurityConfig {
            allowed_user_agents: "AfricasTalking".to_string(),
            verify_signature: false,
            hmac_secret: "test_secret".to_string(),
        },
        africas_talking: AfricasTalkingConfig {
            username: "sandbox".to_string(),
            api_key: "test_key".to_string(),
            api_url: "https://api.sandbox.africastalking.com".to_string(),
            is_sandbox: true,
        },
    }
}

// Load config at compile time
static CONFIG_STR: &str = include_str!("../config.toml");

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = {
        match toml::from_str(CONFIG_STR) {
            Ok(config) => config,
            Err(_) => get_default_config(),
        }
    };
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
