// Mock Juno datastore for E2E testing
// This replaces the real Juno calls with in-memory storage

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Balance {
    pub kes: f64,
    pub ckbtc: f64,
    pub ckusdc: f64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub currency: String,
    pub timestamp: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub phone: String,
    pub name: String,
    pub location: String,
    pub rating: f32,
}

#[derive(Debug, Clone)]
pub struct DaoProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
}

// Mock datastore - shared across all tests
pub struct MockJunoStore {
    pub users: Arc<Mutex<HashMap<String, String>>>, // phone -> language
    pub pins: Arc<Mutex<HashMap<String, String>>>,  // phone -> PIN hash
    pub balances: Arc<Mutex<HashMap<String, Balance>>>, // phone -> Balance
    pub transactions: Arc<Mutex<Vec<Transaction>>>,
    pub agents: Arc<Mutex<Vec<Agent>>>,
    pub proposals: Arc<Mutex<Vec<DaoProposal>>>,
}

impl MockJunoStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            pins: Arc::new(Mutex::new(HashMap::new())),
            balances: Arc::new(Mutex::new(HashMap::new())),
            transactions: Arc::new(Mutex::new(Vec::new())),
            agents: Arc::new(Mutex::new(Vec::new())),
            proposals: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // User operations
    pub fn set_user_language(&self, phone: &str, language: &str) {
        self.users.lock().unwrap().insert(phone.to_string(), language.to_string());
    }

    pub fn get_user_language(&self, phone: &str) -> Option<String> {
        self.users.lock().unwrap().get(phone).cloned()
    }

    // PIN operations
    pub fn set_user_pin(&self, phone: &str, pin_hash: &str) -> Result<(), String> {
        self.pins.lock().unwrap().insert(phone.to_string(), pin_hash.to_string());
        Ok(())
    }

    pub fn get_user_pin(&self, phone: &str) -> Result<String, String> {
        self.pins
            .lock()
            .unwrap()
            .get(phone)
            .cloned()
            .ok_or_else(|| "PIN not found".to_string())
    }

    // Balance operations
    pub fn set_balance(&self, phone: &str, balance: Balance) {
        self.balances.lock().unwrap().insert(phone.to_string(), balance);
    }

    pub fn get_balance(&self, phone: &str) -> Result<Balance, String> {
        self.balances
            .lock()
            .unwrap()
            .get(phone)
            .cloned()
            .ok_or_else(|| "Balance not found".to_string())
    }

    pub fn update_balance(&self, phone: &str, kes_delta: f64, btc_delta: f64, usdc_delta: f64) -> Result<(), String> {
        let mut balances = self.balances.lock().unwrap();
        if let Some(balance) = balances.get_mut(phone) {
            balance.kes += kes_delta;
            balance.ckbtc += btc_delta;
            balance.ckusdc += usdc_delta;
            Ok(())
        } else {
            Err("Balance not found".to_string())
        }
    }

    // Transaction operations
    pub fn add_transaction(&self, tx: Transaction) {
        self.transactions.lock().unwrap().push(tx);
    }

    pub fn get_transactions(&self, phone: &str, limit: usize) -> Vec<Transaction> {
        self.transactions
            .lock()
            .unwrap()
            .iter()
            .filter(|tx| tx.from == phone || tx.to == phone)
            .take(limit)
            .cloned()
            .collect()
    }

    // Agent operations
    pub fn add_agent(&self, agent: Agent) {
        self.agents.lock().unwrap().push(agent);
    }

    pub fn find_agents_near(&self, _location: &str) -> Vec<Agent> {
        self.agents.lock().unwrap().clone()
    }

    // DAO operations
    pub fn add_proposal(&self, proposal: DaoProposal) {
        self.proposals.lock().unwrap().push(proposal);
    }

    pub fn get_proposals(&self) -> Vec<DaoProposal> {
        self.proposals.lock().unwrap().clone()
    }

    pub fn vote_on_proposal(&self, proposal_id: &str, vote_for: bool) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();
        if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    // Clear all data (for test cleanup)
    pub fn clear(&self) {
        self.users.lock().unwrap().clear();
        self.pins.lock().unwrap().clear();
        self.balances.lock().unwrap().clear();
        self.transactions.lock().unwrap().clear();
        self.agents.lock().unwrap().clear();
        self.proposals.lock().unwrap().clear();
    }
}

impl Default for MockJunoStore {
    fn default() -> Self {
        Self::new()
    }
}
