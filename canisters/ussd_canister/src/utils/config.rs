// Configuration for Africa's Talking credentials
// TODO: Implement proper config storage

/// Get AT credentials and API URL - async version
/// Returns (username, api_key, api_url, is_sandbox)
pub async fn get_at_credentials_async() -> Result<(String, String, String, bool), String> {
    // For now, use sandbox mode
    ic_cdk::println!("🎮 Using sandbox mode");
    Ok((
        "sandbox".to_string(),
        "dummy".to_string(),
        "https://api.sandbox.africastalking.com/version1/messaging".to_string(),
        true
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
