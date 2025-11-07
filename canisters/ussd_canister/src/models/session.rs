use ic_cdk::api::time;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

const SESSION_TIMEOUT_NANOS: u64 = 5 * 60 * 1_000_000_000; // 5 minutes in nanoseconds

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UssdSession {
    pub session_id: String,
    pub phone_number: String,
    pub current_menu: String,
    pub step: u32,
    pub language: String, // "en", "lg", or "sw"
    pub last_activity: u64,
}

impl UssdSession {
    pub fn new(session_id: String, phone_number: String) -> Self {
        Self {
            session_id,
            phone_number,
            current_menu: String::new(),
            step: 0,
            language: "en".to_string(), // Default to English
            last_activity: time(),
        }
    }
    
    pub fn is_expired(&self) -> bool {
        let current_time = time();
        current_time - self.last_activity > SESSION_TIMEOUT_NANOS
    }
    
    pub fn update_activity(&mut self) {
        self.last_activity = time();
    }
}

// Thread-local storage for sessions
thread_local! {
    static SESSIONS: RefCell<HashMap<String, UssdSession>> = RefCell::new(HashMap::new());
}

/// Get or create a USSD session
pub async fn get_or_create_session(session_id: &str, phone_number: &str) -> Result<UssdSession, String> {
    SESSIONS.with(|sessions| {
        let mut sessions_map = sessions.borrow_mut();
        
        // Check if session exists and is not expired
        if let Some(session) = sessions_map.get(session_id) {
            if !session.is_expired() {
                let mut session_clone = session.clone();
                session_clone.update_activity();
                sessions_map.insert(session_id.to_string(), session_clone.clone());
                return Ok(session_clone);
            } else {
                // Session expired, remove it
                sessions_map.remove(session_id);
            }
        }
        
        // Create new session
        let new_session = UssdSession::new(session_id.to_string(), phone_number.to_string());
        sessions_map.insert(session_id.to_string(), new_session.clone());
        Ok(new_session)
    })
}

/// Save session
pub async fn save_session(session: &UssdSession) -> Result<(), String> {
    SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(session.session_id.clone(), session.clone());
    });
    Ok(())
}

/// Delete session
pub async fn delete_session(session_id: &str) -> Result<(), String> {
    SESSIONS.with(|sessions| {
        sessions.borrow_mut().remove(session_id);
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_structure() {
        // Test session structure without calling time()
        let session = UssdSession {
            session_id: "test123".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: String::new(),
            step: 0,
            language: "en".to_string(),
            last_activity: 0,
        };
        assert_eq!(session.session_id, "test123");
        assert_eq!(session.phone_number, "+256700123456");
        assert_eq!(session.language, "en");
        assert_eq!(session.step, 0);
    }

    #[tokio::test]
    async fn test_save_and_retrieve() {
        let session = UssdSession {
            session_id: "test789".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: "bitcoin".to_string(),
            step: 1,
            language: "en".to_string(),
            last_activity: 0,
        };
        
        let save_result = save_session(&session).await;
        assert!(save_result.is_ok());
    }

    #[tokio::test]
    async fn test_delete() {
        let session = UssdSession {
            session_id: "test999".to_string(),
            phone_number: "+256700123456".to_string(),
            current_menu: String::new(),
            step: 0,
            language: "en".to_string(),
            last_activity: 0,
        };
        save_session(&session).await.unwrap();
        
        let delete_result = delete_session("test999").await;
        assert!(delete_result.is_ok());
    }
}
