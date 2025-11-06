use ic_cdk::api::time;
use junobuild_satellite::{get_doc_store, set_doc_store, SetDoc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const VERIFICATION_COLLECTION: &str = "verification_codes";
const CODE_EXPIRY_NANOS: u64 = 10 * 60 * 1_000_000_000; // 10 minutes in nanoseconds

#[derive(Serialize, Deserialize, Clone)]
pub struct VerificationData {
    pub code: String,
    pub user_id: String,
    pub timestamp: u64,
}

/// Store verification code in Juno datastore
pub async fn store_verification_code(
    phone_number: &str,
    code: &str,
    user_id: &str,
) -> Result<(), String> {
    let data = VerificationData {
        code: code.to_string(),
        user_id: user_id.to_string(),
        timestamp: time(),
    };
    
    let encoded_data = junobuild_utils::encode_doc_data(&data)
        .map_err(|e| format!("Failed to encode data: {}", e))?;
    
    let doc = SetDoc {
        data: encoded_data,
        description: Some("SMS verification code".to_string()),
        version: None,
    };
    
    // Store with phone number as key
    set_doc_store(
        ic_cdk::api::caller(),
        VERIFICATION_COLLECTION.to_string(),
        phone_number.to_string(),
        doc,
    )?;
    
    Ok(())
}

/// Verify code and return user_id if valid
pub async fn verify_code(phone_number: &str, code: &str) -> Result<String, String> {
    // Get verification data from Juno
    let doc = match get_doc_store(
        ic_cdk::api::caller(),
        VERIFICATION_COLLECTION.to_string(),
        phone_number.to_string(),
    ) {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            let lang = crate::translations::Language::English;
            return Err(crate::translations::TranslationService::translate("user_not_found", lang).to_string());
        }
        Err(e) => return Err(format!("Failed to get verification data: {}", e)),
    };
    
    // Decode data
    let data: VerificationData = junobuild_utils::decode_doc_data(&doc.data)
        .map_err(|e| format!("Failed to decode data: {}", e))?;
    
    // Check expiry
    let current_time = time();
    if current_time - data.timestamp > CODE_EXPIRY_NANOS {
        let lang = crate::translations::Language::English;
        return Err(crate::translations::TranslationService::translate("session_expired", lang).to_string());
    }
    
    // Verify code
    if data.code != code {
        let lang = crate::translations::Language::English;
        return Err(crate::translations::TranslationService::translate("incorrect_pin", lang).to_string());
    }
    
    // Code is valid, return user_id
    Ok(data.user_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verification_data_serialization() {
        let data = VerificationData {
            code: "123456".to_string(),
            user_id: "user123".to_string(),
            timestamp: 1234567890,
        };
        
        let json = serde_json::to_string(&data).unwrap();
        let decoded: VerificationData = serde_json::from_str(&json).unwrap();
        
        assert_eq!(decoded.code, "123456");
        assert_eq!(decoded.user_id, "user123");
        assert_eq!(decoded.timestamp, 1234567890);
    }
    
    #[test]
    fn test_code_expiry_calculation() {
        let current_time = 1_000_000_000_000_000_000u64; // Some timestamp
        let old_time = current_time - (11 * 60 * 1_000_000_000); // 11 minutes ago
        
        let is_expired = current_time - old_time > CODE_EXPIRY_NANOS;
        assert!(is_expired, "Code should be expired after 11 minutes");
        
        let recent_time = current_time - (5 * 60 * 1_000_000_000); // 5 minutes ago
        let is_valid = current_time - recent_time <= CODE_EXPIRY_NANOS;
        assert!(is_valid, "Code should be valid within 10 minutes");
    }
}
