use crate::models::*;
use super::data_client;

// ============================================================================
// Transaction History Service - Business Logic
// ============================================================================

/// Get transaction history for user
pub async fn get_history(
    user_identifier: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<TransactionRecord>, String> {
    // Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // TODO: Call data canister to get transactions
    // For now, return empty list
    Ok(vec![])
}

// ============================================================================
// Helper Functions
// ============================================================================

async fn get_user_by_identifier(identifier: &str) -> Result<data_client::User, String> {
    if let Some(user) = data_client::get_user_by_phone(identifier).await? {
        return Ok(user);
    }
    if let Some(user) = data_client::get_user(identifier).await? {
        return Ok(user);
    }
    Err(format!("User not found: {}", identifier))
}
