// Mock canister clients for E2E testing
// Mocks ckBTC and ckUSDC canister calls

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct CanisterBalance {
    pub amount: f64,
}

// Mock ckBTC canister
pub struct MockCkBtcCanister {
    balances: Arc<Mutex<HashMap<String, f64>>>,
    btc_to_kes_rate: f64,
}

impl MockCkBtcCanister {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(Mutex::new(HashMap::new())),
            btc_to_kes_rate: 150_000_000.0, // 1 BTC = 150M KES
        }
    }

    pub fn set_balance(&self, account: &str, amount: f64) {
        self.balances.lock().unwrap().insert(account.to_string(), amount);
    }

    pub fn get_balance(&self, account: &str) -> f64 {
        *self.balances.lock().unwrap().get(account).unwrap_or(&0.0)
    }

    pub fn transfer(&self, from: &str, to: &str, amount: f64) -> Result<(), String> {
        let mut balances = self.balances.lock().unwrap();
        
        let from_balance = balances.get(from).copied().unwrap_or(0.0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        balances.insert(from.to_string(), from_balance - amount);
        let to_balance = balances.get(to).copied().unwrap_or(0.0);
        balances.insert(to.to_string(), to_balance + amount);

        Ok(())
    }

    pub fn get_rate(&self) -> f64 {
        self.btc_to_kes_rate
    }
}

impl Default for MockCkBtcCanister {
    fn default() -> Self {
        Self::new()
    }
}

// Mock ckUSDC canister
pub struct MockCkUsdcCanister {
    balances: Arc<Mutex<HashMap<String, f64>>>,
    usdc_to_kes_rate: f64,
}

impl MockCkUsdcCanister {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(Mutex::new(HashMap::new())),
            usdc_to_kes_rate: 150.0, // 1 USDC = 150 KES
        }
    }

    pub fn set_balance(&self, account: &str, amount: f64) {
        self.balances.lock().unwrap().insert(account.to_string(), amount);
    }

    pub fn get_balance(&self, account: &str) -> f64 {
        *self.balances.lock().unwrap().get(account).unwrap_or(&0.0)
    }

    pub fn transfer(&self, from: &str, to: &str, amount: f64) -> Result<(), String> {
        let mut balances = self.balances.lock().unwrap();
        
        let from_balance = balances.get(from).copied().unwrap_or(0.0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        balances.insert(from.to_string(), from_balance - amount);
        let to_balance = balances.get(to).copied().unwrap_or(0.0);
        balances.insert(to.to_string(), to_balance + amount);

        Ok(())
    }

    pub fn get_rate(&self) -> f64 {
        self.usdc_to_kes_rate
    }
}

impl Default for MockCkUsdcCanister {
    fn default() -> Self {
        Self::new()
    }
}
