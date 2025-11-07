use ic_cdk::api::time;
use junobuild_satellite::{get_doc_store, set_doc_store, SetDoc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SESSION_COLLECTION: &str = "ussd_sessions";
const SESSION_TIMEOUT_NANOS: u64 = 5 * 60 * 1_000_000_000; // 5 minutes in nanoseconds

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UssdSession {
    pub session_id: String,
    pub phone_number: String,
    pub current_menu: String,
    pub step: u32,
    pub data: HashMap<String, String>,
    pub last_activity: u64,
    pub language: String, // "en", "lg", or "sw"
}

impl UssdSession {
    pub fn new(session_id: String, phone_number: String) -> Self {
        Self {
            session_id,
            phone_number,
            current_menu: "main".to_string(),
            step: 0,
            data: HashMap::new(),
            last_activity: time(),
            language: "en".to_string(), // Default to English
        }
    }
    
    pub fn is_expired(&self) -> bool {
        let current_time = time();
        current_time - self.last_activity > SESSION_TIMEOUT_NANOS
    }
    
    pub fn update_activity(&mut self) {
        self.last_activity = time();
    }
    
    pub fn set_data(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

/// Get or create USSD session
pub async fn get_or_create_session(
    session_id: &str,
    phone_number: &str,
) -> Result<UssdSession, String> {
    // Try to get existing session
    match get_doc_store(
        ic_cdk::api::caller(),
        SESSION_COLLECTION.to_string(),
        session_id.to_string(),
    ) {
        Ok(Some(doc)) => {
            // Decode existing session
            match junobuild_utils::decode_doc_data::<UssdSession>(&doc.data) {
                Ok(mut session) => {
                    // Check if expired
                    if session.is_expired() {
                        ic_cdk::println!("⏰ Session expired, creating new one");
                        // Create new session
                        Ok(UssdSession::new(session_id.to_string(), phone_number.to_string()))
                    } else {
                        // Update activity timestamp
                        session.update_activity();
                        Ok(session)
                    }
                }
                Err(e) => {
                    ic_cdk::println!("❌ Failed to decode session: {}", e);
                    // Create new session on decode error
                    Ok(UssdSession::new(session_id.to_string(), phone_number.to_string()))
                }
            }
        }
        Ok(None) => {
            // No existing session, create new one
            ic_cdk::println!("✨ Creating new USSD session for {}", phone_number);
            Ok(UssdSession::new(session_id.to_string(), phone_number.to_string()))
        }
        Err(e) => {
            ic_cdk::println!("❌ Error getting session: {}", e);
            // Create new session on error
            Ok(UssdSession::new(session_id.to_string(), phone_number.to_string()))
        }
    }
}

/// Save USSD session to Juno datastore
pub async fn save_session(session: &UssdSession) -> Result<(), String> {
    let encoded = junobuild_utils::encode_doc_data(session)
        .map_err(|e| format!("Failed to encode session: {}", e))?;
    
    let doc = SetDoc {
        data: encoded,
        description: Some(format!("USSD session for {}", session.phone_number)),
        version: None,
    };
    
    junobuild_satellite::set_doc_store(
        ic_cdk::api::caller(),
        SESSION_COLLECTION.to_string(),
        session.session_id.clone(),
        doc,
    )?;
    
    ic_cdk::println!("💾 Saved session {} for {}", session.session_id, session.phone_number);
    Ok(())
}

/// Delete USSD session (when user exits)
pub async fn delete_session(session_id: &str) -> Result<(), String> {
    let del_doc = junobuild_satellite::DelDoc {
        version: None,
    };
    
    match junobuild_satellite::delete_doc_store(
        ic_cdk::api::caller(),
        SESSION_COLLECTION.to_string(),
        session_id.to_string(),
        del_doc,
    ) {
        Ok(_) => {
            ic_cdk::println!("🗑️ Deleted session {}", session_id);
            Ok(())
        }
        Err(e) => {
            ic_cdk::println!("⚠️ Failed to delete session: {}", e);
            Err(format!("Failed to delete session: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let session = UssdSession::new("test123".to_string(), "+254700000000".to_string());
        assert_eq!(session.session_id, "test123");
        assert_eq!(session.phone_number, "+254700000000");
        assert_eq!(session.current_menu, "main");
        assert_eq!(session.step, 0);
        assert!(session.data.is_empty());
    }
    
    #[test]
    fn test_session_data() {
        let mut session = UssdSession::new("test123".to_string(), "+254700000000".to_string());
        session.set_data("recipient", "+254711111111");
        session.set_data("amount", "1000");
        
        assert_eq!(session.get_data("recipient"), Some(&"+ 254711111111".to_string()));
        assert_eq!(session.get_data("amount"), Some(&"1000".to_string()));
        assert_eq!(session.get_data("nonexistent"), None);
    }
    
    #[test]
    fn test_session_serialization() {
        let mut session = UssdSession::new("test123".to_string(), "+254700000000".to_string());
        session.set_data("test", "value");
        
        let json = serde_json::to_string(&session).unwrap();
        let decoded: UssdSession = serde_json::from_str(&json).unwrap();
        
        assert_eq!(decoded.session_id, session.session_id);
        assert_eq!(decoded.phone_number, session.phone_number);
        assert_eq!(decoded.get_data("test"), Some(&"value".to_string()));
    }
}
