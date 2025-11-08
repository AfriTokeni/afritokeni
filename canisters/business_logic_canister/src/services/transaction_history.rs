use crate::models::*;
use super::data_client;

// ============================================================================
// Transaction History Service - Business Logic
// ============================================================================

/// Get user transaction history
pub async fn get_history(
    user_identifier: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<TransactionRecord>, String> {
    // Get user
    let user = get_user_by_identifier(&user_identifier).await?;
    
    // Query transactions from data canister with limit and offset
    let transactions = data_client::get_user_transactions(
        &user.id,
        limit,
        offset
    ).await?;
    
    Ok(transactions)
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to get user by identifier (phone or principal)
async fn get_user_by_identifier(identifier: &str) -> Result<crate::services::data_client::User, String> {
    // Try as phone first
    if let Some(user) = data_client::get_user_by_phone(identifier).await? {
        return Ok(user);
    }
    
    // Try as user ID
    if let Some(user) = data_client::get_user(identifier).await? {
        return Ok(user);
    }
    
    Err(format!("User not found: {}", identifier))
}
