// Juno Satellite Client for USSD Canister
// Makes inter-canister calls to Juno satellite for user data

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

/// Get Juno satellite ID from environment variable
fn get_satellite_id() -> Result<Principal, String> {
    // Try to get from environment variable first
    // For Juno emulator, use development satellite ID
    let satellite_id_str = option_env!("JUNO_SATELLITE_ID")
        .unwrap_or("atbka-rp777-77775-aaaaq-cai"); // Juno development satellite on emulator
    
    Principal::from_text(satellite_id_str)
        .map_err(|e| format!("Invalid satellite ID '{}': {}", satellite_id_str, e))
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SetDoc {
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub version: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Doc {
    pub updated_at: u64,
    pub owner: Principal,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub created_at: u64,
    pub version: Option<u64>,
}

/// Get user language from Juno
pub async fn get_user_language(phone_number: &str) -> Result<Option<String>, String> {
    let satellite_id = get_satellite_id()?;
    
    let key = format!("user_language_{}", phone_number);
    
    // Call Juno's get_doc method: get_doc(collection: text, key: text) -> (opt Doc)
    let result: Result<(Option<Doc>,), _> = ic_cdk::call(
        satellite_id,
        "get_doc",
        ("users", &key),
    )
    .await;
    
    match result {
        Ok((Some(doc),)) => {
            // Decode the data
            let lang = String::from_utf8(doc.data)
                .map_err(|e| format!("Failed to decode language: {}", e))?;
            Ok(Some(lang))
        }
        Ok((None,)) => Ok(None),
        Err((code, msg)) => {
            ic_cdk::println!("❌ Juno get_doc failed: {:?} - {}", code, msg);
            Ok(None) // Return None instead of error for missing data
        }
    }
}

/// Set user language in Juno
pub async fn set_user_language(phone_number: &str, language: &str) -> Result<(), String> {
    let satellite_id = get_satellite_id()?;
    
    let key = format!("user_language_{}", phone_number);
    
    // Call Juno's set_doc method: set_doc(collection: text, key: text, doc: SetDoc) -> (Doc)
    let result: Result<(Doc,), _> = ic_cdk::call(
        satellite_id,
        "set_doc",
        (
            "users",
            &key,
            SetDoc {
                data: language.as_bytes().to_vec(),
                description: Some("User language preference".to_string()),
                version: None,
            },
        ),
    )
    .await;
    
    match result {
        Ok(_) => {
            ic_cdk::println!("✅ Saved language {} for {} to Juno", language, phone_number);
            Ok(())
        }
        Err((code, msg)) => {
            ic_cdk::println!("❌ Juno set_doc failed: {:?} - {}", code, msg);
            Err(format!("Failed to save language: {}", msg))
        }
    }
}

/// Get PIN attempts from Juno
pub async fn get_pin_attempts(phone_number: &str) -> Result<u32, String> {
    let satellite_id = get_satellite_id()?;
    
    let key = format!("pin_attempts_{}", phone_number);
    
    let result: Result<(Option<Doc>,), _> = ic_cdk::call(
        satellite_id,
        "get_doc",
        ("users", &key),
    )
    .await;
    
    match result {
        Ok((Some(doc),)) => {
            let attempts_str = String::from_utf8(doc.data)
                .map_err(|e| format!("Failed to decode attempts: {}", e))?;
            let attempts = attempts_str.parse::<u32>().unwrap_or(0);
            Ok(attempts)
        }
        Ok((None,)) => Ok(0),
        Err(_) => Ok(0),
    }
}

/// Set PIN attempts in Juno
pub async fn set_pin_attempts(phone_number: &str, attempts: u32) -> Result<(), String> {
    let satellite_id = get_satellite_id()?;
    
    let key = format!("pin_attempts_{}", phone_number);
    
    let result: Result<(Doc,), _> = ic_cdk::call(
        satellite_id,
        "set_doc",
        (
            "users",
            &key,
            SetDoc {
                data: attempts.to_string().as_bytes().to_vec(),
                description: Some("PIN attempt counter".to_string()),
                version: None,
            },
        ),
    )
    .await;
    
    match result {
        Ok(_) => Ok(()),
        Err((code, msg)) => {
            ic_cdk::println!("❌ Juno set_doc failed: {:?} - {}", code, msg);
            Err(format!("Failed to save PIN attempts: {}", msg))
        }
    }
}

/// Reset PIN attempts in Juno
pub async fn reset_pin_attempts(phone_number: &str) -> Result<(), String> {
    set_pin_attempts(phone_number, 0).await
}
