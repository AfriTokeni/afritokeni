use junobuild_satellite::get_doc_store;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigData {
    at_username: Option<String>,
    at_api_key: Option<String>,
    playground_mode: Option<bool>,
}

/// Get configuration from Juno datastore
/// Config is stored in a special "config" collection with key "afritalking"
async fn get_config_data() -> Option<ConfigData> {
    match get_doc_store(
        ic_cdk::api::caller(),
        "config".to_string(),
        "afritalking".to_string(),
    ) {
        Ok(Some(doc)) => {
            match junobuild_utils::decode_doc_data::<ConfigData>(&doc.data) {
                Ok(config) => Some(config),
                Err(e) => {
                    ic_cdk::println!("❌ Failed to decode config: {}", e);
                    None
                }
            }
        }
        Ok(None) => {
            ic_cdk::println!("⚠️ No config found in datastore");
            None
        }
        Err(e) => {
            ic_cdk::println!("❌ Error getting config: {}", e);
            None
        }
    }
}

/// Get AT credentials and API URL - async version
/// Returns (username, api_key, api_url, is_sandbox)
/// FAILS if credentials not configured and not in playground mode
pub async fn get_at_credentials_async() -> Result<(String, String, String, bool), String> {
    match get_config_data().await {
        Some(config) => {
            // Check if playground mode
            if config.playground_mode.unwrap_or(false) {
                ic_cdk::println!("🎮 Playground mode enabled: using sandbox");
                return Ok((
                    "sandbox".to_string(),
                    "dummy".to_string(),
                    "https://api.sandbox.africastalking.com/version1/messaging".to_string(),
                    true
                ));
            }
            
            // Production mode - credentials MUST be configured
            match (config.at_username, config.at_api_key) {
                (Some(username), Some(api_key)) => {
                    ic_cdk::println!("✅ Using production Africa's Talking credentials");
                    Ok((
                        username,
                        api_key,
                        "https://api.africastalking.com/version1/messaging".to_string(),
                        false
                    ))
                }
                _ => {
                    Err("CRITICAL: Africa's Talking credentials not configured. Set at_username and at_api_key in config collection.".to_string())
                }
            }
        }
        None => {
            Err("CRITICAL: Config collection not found. Create 'afritalking' document in 'config' collection.".to_string())
        }
    }
}
