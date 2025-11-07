// Configuration for Africa's Talking credentials
// Uses config_loader to load from config.toml

use crate::config_loader::get_config;

/// Get AT credentials and API URL - async version
/// Returns (username, api_key, api_url, is_sandbox)
#[allow(dead_code)]
pub async fn get_at_credentials_async() -> Result<(String, String, String, bool), String> {
    let config = get_config();
    Ok((
        config.africas_talking.username.clone(),
        config.africas_talking.api_key.clone(),
        config.africas_talking.api_url.clone(),
        config.africas_talking.is_sandbox,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_credentials() {
        let result = get_at_credentials_async().await;
        assert!(result.is_ok());
        let (username, _, _, is_sandbox) = result.unwrap();
        assert_eq!(username, "sandbox");
        assert!(is_sandbox);
    }
}
