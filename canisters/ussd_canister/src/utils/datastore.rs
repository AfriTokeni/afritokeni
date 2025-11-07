// In-memory datastore for USSD canister
// Uses thread-local storage to persist data across calls

use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Balance {
    pub kes: f64,
    pub ckbtc: f64,
    pub ckusdc: f64,
}

impl Default for Balance {
    fn default() -> Self {
        Balance {
            kes: 0.0,
            ckbtc: 0.0,
            ckusdc: 0.0,
        }
    }
}

// Thread-local storage for user data
thread_local! {
    static USER_PINS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static USER_BALANCES: RefCell<HashMap<String, Balance>> = RefCell::new(HashMap::new());
    static USER_LANGUAGES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Get user's PIN hash from datastore
#[allow(dead_code)]
pub async fn get_user_pin(phone_number: &str) -> Result<String, String> {
    USER_PINS.with(|pins| {
        pins.borrow()
            .get(phone_number)
            .cloned()
            .ok_or_else(|| "PIN not found".to_string())
    })
}

/// Set user's PIN hash in datastore
#[allow(dead_code)]
pub async fn set_user_pin(phone_number: &str, pin_hash: &str) -> Result<(), String> {
    USER_PINS.with(|pins| {
        pins.borrow_mut().insert(phone_number.to_string(), pin_hash.to_string());
    });
    Ok(())
}

/// Get user's balance from datastore
#[allow(dead_code)]
pub async fn get_balance(phone_number: &str) -> Result<Balance, String> {
    USER_BALANCES.with(|balances| {
        Ok(balances.borrow()
            .get(phone_number)
            .cloned()
            .unwrap_or_default())
    })
}

/// Set user's balance in datastore
#[allow(dead_code)]
pub async fn set_balance(phone_number: &str, balance: &Balance) -> Result<(), String> {
    USER_BALANCES.with(|balances| {
        balances.borrow_mut().insert(phone_number.to_string(), balance.clone());
    });
    Ok(())
}

/// Get user's language preference
#[allow(dead_code)]
pub async fn get_user_language(phone_number: &str) -> Result<String, String> {
    USER_LANGUAGES.with(|languages| {
        Ok(languages.borrow()
            .get(phone_number)
            .cloned()
            .unwrap_or_else(|| "en".to_string()))
    })
}

/// Set user's language preference
#[allow(dead_code)]
pub async fn set_user_language(phone_number: &str, language: &str) -> Result<(), String> {
    USER_LANGUAGES.with(|languages| {
        languages.borrow_mut().insert(phone_number.to_string(), language.to_string());
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_creation() {
        let balance = Balance {
            kes: 1000.0,
            ckbtc: 0.001,
            ckusdc: 100.0,
        };
        assert_eq!(balance.kes, 1000.0);
        assert_eq!(balance.ckbtc, 0.001);
        assert_eq!(balance.ckusdc, 100.0);
    }

    #[test]
    fn test_balance_default() {
        let balance = Balance::default();
        assert_eq!(balance.kes, 0.0);
        assert_eq!(balance.ckbtc, 0.0);
        assert_eq!(balance.ckusdc, 0.0);
    }

    #[tokio::test]
    async fn test_pin_storage() {
        let phone = "+256700123456";
        let pin_hash = "hashed_pin_123";
        
        // Set PIN
        let result = set_user_pin(phone, pin_hash).await;
        assert!(result.is_ok());
        
        // Get PIN
        let retrieved = get_user_pin(phone).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), pin_hash);
    }

    #[tokio::test]
    async fn test_balance_storage() {
        let phone = "+256700123456";
        let balance = Balance {
            kes: 5000.0,
            ckbtc: 0.005,
            ckusdc: 500.0,
        };
        
        // Set balance
        let result = set_balance(phone, &balance).await;
        assert!(result.is_ok());
        
        // Get balance
        let retrieved = get_balance(phone).await;
        assert!(retrieved.is_ok());
        let retrieved_balance = retrieved.unwrap();
        assert_eq!(retrieved_balance.kes, 5000.0);
        assert_eq!(retrieved_balance.ckbtc, 0.005);
    }

    #[tokio::test]
    async fn test_language_storage() {
        let phone = "+256700123456";
        
        // Set language
        let result = set_user_language(phone, "sw").await;
        assert!(result.is_ok());
        
        // Get language
        let retrieved = get_user_language(phone).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), "sw");
    }

    #[tokio::test]
    async fn test_default_language() {
        let phone = "+256700999999";
        
        // Get language for user without set language
        let retrieved = get_user_language(phone).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), "en"); // Default
    }
}
