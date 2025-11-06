use serde::{Deserialize, Serialize};

/// User balance structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub kes: f64,
    pub ckbtc: f64,
    pub ckusdc: f64,
}

/// User structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub phone: String,
    pub language: String,
    pub pin_hash: Option<String>,
    pub created_at: u64,
}

/// Transaction structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub currency: String,
    pub tx_type: String, // "send", "buy", "sell", "deposit", "withdraw"
    pub status: String,  // "pending", "completed", "failed"
    pub timestamp: u64,
    pub fee: f64,
}

/// Agent structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub location: String,
    pub rating: f64,
    pub cash_balance: f64,
}

/// DAO Proposal structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: String, // "active", "passed", "rejected"
    pub created_at: u64,
}

/// Get user balance from Juno datastore
pub async fn get_user_balance(phone: &str) -> Result<Balance, String> {
    let caller = ic_cdk::api::caller();
    
    match junobuild_satellite::get_doc_store(
        caller,
        "balances".to_string(),
        phone.to_string(),
    ) {
        Ok(Some(doc)) => {
            match junobuild_utils::decode_doc_data::<Balance>(&doc.data) {
                Ok(balance) => Ok(balance),
                Err(e) => Err(format!("Failed to decode balance: {}", e)),
            }
        }
        Ok(None) => {
            // Return zero balance if not found
            Ok(Balance {
                kes: 0.0,
                ckbtc: 0.0,
                ckusdc: 0.0,
            })
        }
        Err(e) => Err(format!("Failed to get balance: {}", e)),
    }
}

/// Get user from Juno datastore
pub async fn get_user(phone: &str) -> Result<User, String> {
    let caller = ic_cdk::api::caller();
    
    match junobuild_satellite::get_doc_store(
        caller,
        "users".to_string(),
        phone.to_string(),
    ) {
        Ok(Some(doc)) => {
            match junobuild_utils::decode_doc_data::<User>(&doc.data) {
                Ok(user) => Ok(user),
                Err(e) => Err(format!("Failed to decode user: {}", e)),
            }
        }
        Ok(None) => Err("User not found".to_string()),
        Err(e) => Err(format!("Failed to get user: {}", e)),
    }
}

/// Create or update user balance
pub async fn set_user_balance(phone: &str, balance: &Balance) -> Result<(), String> {
    let caller = ic_cdk::api::caller();
    
    let encoded = junobuild_utils::encode_doc_data(balance)
        .map_err(|e| format!("Failed to encode balance: {}", e))?;
    
    let doc = junobuild_satellite::SetDoc {
        data: encoded,
        description: Some("User balance".to_string()),
        version: None,
    };
    
    junobuild_satellite::set_doc_store(
        caller,
        "balances".to_string(),
        phone.to_string(),
        doc,
    ).map_err(|e| format!("Failed to save balance: {}", e))?;
    
    Ok(())
}

/// Create transaction
pub async fn create_transaction(tx: Transaction) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let tx_id = format!("tx_{}", ic_cdk::api::time());
    
    let encoded = junobuild_utils::encode_doc_data(&tx)
        .map_err(|e| format!("Failed to encode transaction: {}", e))?;
    
    let doc = junobuild_satellite::SetDoc {
        data: encoded,
        description: Some(format!("{} transaction", tx.tx_type)),
        version: None,
    };
    
    junobuild_satellite::set_doc_store(
        caller,
        "transactions".to_string(),
        tx_id.clone(),
        doc,
    ).map_err(|e| format!("Failed to save transaction: {}", e))?;
    
    Ok(tx_id)
}

/// Get user's transaction history
/// Note: This is a simplified version. In production, you'd want to:
/// 1. Use list_docs with proper filtering
/// 2. Implement pagination
/// 3. Index transactions by user for efficient queries
pub async fn get_transactions(phone: &str, _limit: u32) -> Result<Vec<Transaction>, String> {
    let caller = ic_cdk::api::caller();
    
    // For now, we'll return a mock recent transaction
    // In production, implement proper list_docs filtering
    let mock_tx = Transaction {
        from: phone.to_string(),
        to: "+256700000000".to_string(),
        amount: 10000.0,
        currency: "KES".to_string(),
        tx_type: "send".to_string(),
        status: "completed".to_string(),
        timestamp: ic_cdk::api::time(),
        fee: 0.0,
    };
    
    Ok(vec![mock_tx])
}

/// Get user PIN hash
pub async fn get_user_pin(phone: &str) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    
    match junobuild_satellite::get_doc_store(
        caller,
        "user_pins".to_string(),
        phone.to_string(),
    ) {
        Ok(Some(doc)) => {
            #[derive(Deserialize)]
            struct PinData {
                pin_hash: String,
            }
            
            match junobuild_utils::decode_doc_data::<PinData>(&doc.data) {
                Ok(data) => Ok(data.pin_hash),
                Err(e) => Err(format!("Failed to decode PIN: {}", e)),
            }
        }
        Ok(None) => Err("PIN not found".to_string()),
        Err(e) => Err(format!("Failed to get PIN: {}", e)),
    }
}

/// Set user PIN (hashed)
pub async fn set_user_pin(phone: &str, pin_hash: &str) -> Result<(), String> {
    let caller = ic_cdk::api::caller();
    
    #[derive(Serialize)]
    struct PinData {
        pin_hash: String,
    }
    
    let pin_data = PinData {
        pin_hash: pin_hash.to_string(),
    };
    
    let encoded = junobuild_utils::encode_doc_data(&pin_data)
        .map_err(|e| format!("Failed to encode PIN: {}", e))?;
    
    let doc = junobuild_satellite::SetDoc {
        data: encoded,
        description: Some("User PIN".to_string()),
        version: None,
    };
    
    junobuild_satellite::set_doc_store(
        caller,
        "user_pins".to_string(),
        phone.to_string(),
        doc,
    ).map_err(|e| format!("Failed to save PIN: {}", e))?;
    
    Ok(())
}

/// Get user's preferred language
pub async fn get_language(phone: &str) -> Result<String, String> {
    match get_user(phone).await {
        Ok(user) => Ok(user.language),
        Err(_) => Ok("en".to_string()), // Default to English
    }
}

/// Set user language preference
pub async fn set_language(phone: &str, lang: &str) -> Result<(), String> {
    let caller = ic_cdk::api::caller();
    
    // Get existing user or create new one
    let mut user = get_user(phone).await.unwrap_or(User {
        phone: phone.to_string(),
        language: "en".to_string(),
        pin_hash: None,
        created_at: ic_cdk::api::time(),
    });
    
    user.language = lang.to_string();
    
    let encoded = junobuild_utils::encode_doc_data(&user)
        .map_err(|e| format!("Failed to encode user: {}", e))?;
    
    let doc = junobuild_satellite::SetDoc {
        data: encoded,
        description: Some("User profile".to_string()),
        version: None,
    };
    
    junobuild_satellite::set_doc_store(
        caller,
        "users".to_string(),
        phone.to_string(),
        doc,
    ).map_err(|e| format!("Failed to save user: {}", e))?;
    
    Ok(())
}

/// Find agents near a location
/// Returns agents sorted by rating
/// In production, implement geospatial queries or location-based filtering
pub async fn find_agents_near(_location: &str) -> Result<Vec<Agent>, String> {
    // Return top-rated agents
    // In production, filter by actual location proximity
    Ok(vec![
        Agent {
            id: "agent_kampala_001".to_string(),
            name: "John's Mobile Money".to_string(),
            phone: "+256700111222".to_string(),
            location: "Kampala Central".to_string(),
            rating: 4.8,
            cash_balance: 1000000.0,
        },
        Agent {
            id: "agent_kampala_002".to_string(),
            name: "Mary's Shop".to_string(),
            phone: "+256700222333".to_string(),
            location: "Kampala".to_string(),
            rating: 4.6,
            cash_balance: 750000.0,
        },
        Agent {
            id: "agent_kampala_003".to_string(),
            name: "AfriTokeni Hub".to_string(),
            phone: "+256700333444".to_string(),
            location: "Kampala".to_string(),
            rating: 4.9,
            cash_balance: 2000000.0,
        }
    ])
}

/// Get agent by ID
pub async fn get_agent(agent_id: &str) -> Result<Agent, String> {
    let caller = ic_cdk::api::caller();
    
    match junobuild_satellite::get_doc_store(
        caller,
        "agents".to_string(),
        agent_id.to_string(),
    ) {
        Ok(Some(doc)) => {
            match junobuild_utils::decode_doc_data::<Agent>(&doc.data) {
                Ok(agent) => Ok(agent),
                Err(e) => Err(format!("Failed to decode agent: {}", e)),
            }
        }
        Ok(None) => Err("Agent not found".to_string()),
        Err(e) => Err(format!("Failed to get agent: {}", e)),
    }
}

/// Get DAO proposals
/// Returns active and recent proposals
pub async fn get_proposals() -> Result<Vec<Proposal>, String> {
    // Return active proposals
    // In production, fetch from dao_proposals collection
    Ok(vec![
        Proposal {
            id: "prop_001".to_string(),
            title: "Increase Agent Commission to 2%".to_string(),
            description: "Proposal to increase agent commission from 1.5% to 2%".to_string(),
            votes_for: 1250,
            votes_against: 340,
            status: "active".to_string(),
            created_at: ic_cdk::api::time() - (7 * 24 * 60 * 60 * 1_000_000_000), // 7 days ago
        },
        Proposal {
            id: "prop_002".to_string(),
            title: "Add Support for ckETH".to_string(),
            description: "Proposal to add Ethereum (ckETH) to AfriTokeni platform".to_string(),
            votes_for: 2100,
            votes_against: 150,
            status: "active".to_string(),
            created_at: ic_cdk::api::time() - (3 * 24 * 60 * 60 * 1_000_000_000), // 3 days ago
        },
        Proposal {
            id: "prop_003".to_string(),
            title: "Reduce Transaction Fees".to_string(),
            description: "Proposal to reduce platform fees from 0.5% to 0.3%".to_string(),
            votes_for: 3400,
            votes_against: 890,
            status: "passed".to_string(),
            created_at: ic_cdk::api::time() - (14 * 24 * 60 * 60 * 1_000_000_000), // 14 days ago
        }
    ])
}

/// Vote on a proposal
pub async fn vote_on_proposal(user: &str, proposal_id: &str, vote: bool) -> Result<(), String> {
    let caller = ic_cdk::api::caller();
    
    #[derive(Serialize)]
    struct Vote {
        user: String,
        proposal_id: String,
        vote: bool,
        timestamp: u64,
    }
    
    let vote_data = Vote {
        user: user.to_string(),
        proposal_id: proposal_id.to_string(),
        vote,
        timestamp: ic_cdk::api::time(),
    };
    
    let encoded = junobuild_utils::encode_doc_data(&vote_data)
        .map_err(|e| format!("Failed to encode vote: {}", e))?;
    
    let vote_id = format!("vote_{}_{}", user, proposal_id);
    let doc = junobuild_satellite::SetDoc {
        data: encoded,
        description: Some("DAO vote".to_string()),
        version: None,
    };
    
    // Store vote in transactions collection with special type
    // In production, create a dedicated dao_votes collection in juno.config.ts
    junobuild_satellite::set_doc_store(
        caller,
        "transactions".to_string(),
        vote_id,
        doc,
    ).map_err(|e| format!("Failed to save vote: {}", e))?;
    
    ic_cdk::println!("✅ Vote recorded: user={}, proposal={}, vote={}", user, proposal_id, vote);
    Ok(())
}
