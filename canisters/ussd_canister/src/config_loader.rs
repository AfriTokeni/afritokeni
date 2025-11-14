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
    pub contact_info: ContactInfoConfig,
    pub ussd_defaults: UssdDefaultsConfig,
    pub features: FeaturesConfig,
    pub validation: ValidationConfig,
    pub playground: PlaygroundConfig,
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

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ContactInfoConfig {
    pub support_phone: String,
    pub support_email: String,
    pub website: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UssdDefaultsConfig {
    pub default_email_domain: String,
    pub default_currency: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct FeaturesConfig {
    pub enable_dao_voting: bool,
    pub enable_rate_checking: bool,
    pub enable_find_agent: bool,
    pub enable_transaction_history: bool,
    pub enable_language_switching: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ValidationConfig {
    pub btc_address_min_length: usize,
    pub btc_address_max_length: usize,
    pub btc_strict_checksum_validation: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PlaygroundConfig {
    pub enabled: bool,
    pub session_id_prefix: String,
    pub default_pin: String,
    pub default_currency: String,
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
        contact_info: ContactInfoConfig {
            support_phone: "+256700000000".to_string(),
            support_email: "support@afritokeni.com".to_string(),
            website: "afritokeni.com".to_string(),
        },
        ussd_defaults: UssdDefaultsConfig {
            default_email_domain: "ussd.afritokeni.com".to_string(),
            default_currency: "UGX".to_string(),
        },
        features: FeaturesConfig {
            enable_dao_voting: false,
            enable_rate_checking: false,
            enable_find_agent: false,
            enable_transaction_history: false,
            enable_language_switching: false,
        },
        validation: ValidationConfig {
            btc_address_min_length: 26,
            btc_address_max_length: 62,
            btc_strict_checksum_validation: false, // Default to false for tests
        },
        playground: PlaygroundConfig {
            enabled: true,
            session_id_prefix: "playground_".to_string(),
            default_pin: "1234".to_string(),
            default_currency: "UGX".to_string(),
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
        assert!(config.rate_limiting.max_requests_per_minute > 0);
        assert_eq!(config.pin_security.max_pin_attempts, 3);
        assert!(!config.contact_info.support_phone.is_empty());
    }
}
