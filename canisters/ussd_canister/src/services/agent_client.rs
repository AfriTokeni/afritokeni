/// Agent Canister Client
/// Handles deposits, withdrawals, agent balances, and settlements
use candid::Principal;
use ic_cdk::call::Call;
use shared_types::{FiatCurrency, DepositTransaction, WithdrawalTransaction};
use std::cell::RefCell;

thread_local! {
    static AGENT_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

// ============================================================================
// TEST MOCKS
// ============================================================================

#[cfg(any(test, feature = "test-utils"))]
use std::sync::Mutex;

#[cfg(any(test, feature = "test-utils"))]
lazy_static::lazy_static! {
    static ref MOCK_CREATE_DEPOSIT_REQUEST: Mutex<Option<Box<dyn Fn(String, String, u64, FiatCurrency) -> Result<DepositTransaction, String> + Send>>> = Mutex::new(None);
    static ref MOCK_CREATE_WITHDRAWAL_REQUEST: Mutex<Option<Box<dyn Fn(String, String, u64, FiatCurrency, String) -> Result<WithdrawalTransaction, String> + Send>>> = Mutex::new(None);
    static ref MOCK_GET_WITHDRAWAL_FEES: Mutex<Option<Box<dyn Fn(u64) -> Result<WithdrawalFeesResponse, String> + Send>>> = Mutex::new(None);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_create_deposit_request<F>(mock: F)
where
    F: Fn(String, String, u64, FiatCurrency) -> Result<DepositTransaction, String> + Send + 'static,
{
    *MOCK_CREATE_DEPOSIT_REQUEST.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_create_withdrawal_request<F>(mock: F)
where
    F: Fn(String, String, u64, FiatCurrency, String) -> Result<WithdrawalTransaction, String> + Send + 'static,
{
    *MOCK_CREATE_WITHDRAWAL_REQUEST.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn set_mock_get_withdrawal_fees<F>(mock: F)
where
    F: Fn(u64) -> Result<WithdrawalFeesResponse, String> + Send + 'static,
{
    *MOCK_GET_WITHDRAWAL_FEES.lock().unwrap() = Some(Box::new(mock));
}

#[cfg(any(test, feature = "test-utils"))]
pub fn clear_mocks() {
    *MOCK_CREATE_DEPOSIT_REQUEST.lock().unwrap() = None;
    *MOCK_CREATE_WITHDRAWAL_REQUEST.lock().unwrap() = None;
    *MOCK_GET_WITHDRAWAL_FEES.lock().unwrap() = None;
}

pub fn set_agent_canister_id(canister_id: Principal) {
    AGENT_CANISTER_ID.with(|id| {
        *id.borrow_mut() = Some(canister_id);
    });
}

pub fn get_agent_canister_id() -> Result<Principal, String> {
    AGENT_CANISTER_ID.with(|id| {
        id.borrow()
            .ok_or_else(|| "Agent Canister ID not set".to_string())
    })
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateDepositRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateDepositResponse {
    pub deposit_code: String,
    pub amount: u64,
    pub commission: u64,
    pub net_amount: u64,
    pub currency: FiatCurrency,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateWithdrawalRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: u64,
    pub currency: String,
    pub pin: String,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct CreateWithdrawalResponse {
    pub withdrawal_code: String,
    pub amount: u64,
    pub currency: String,
    pub total_fees: u64,
    pub net_to_user: u64,
    pub expires_at: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct WithdrawalFeesResponse {
    pub amount: u64,
    pub platform_fee: u64,
    pub agent_fee: u64,
    pub total_fees: u64,
    pub net_amount: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct AgentBalanceResponse {
    pub agent_id: String,
    pub currency: String,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub pending_commission: u64,
    pub paid_commission: u64,
}

// ============================================================================
// DEPOSIT OPERATIONS
// ============================================================================

/// Create deposit request (user brings cash to agent)
pub async fn create_deposit_request(
    user_identifier: String,
    agent_id: String,
    amount: u64,
    currency: FiatCurrency,
) -> Result<DepositTransaction, String> {
    #[cfg(any(test, feature = "test-utils"))]
    {
        if let Some(mock) = MOCK_CREATE_DEPOSIT_REQUEST.lock().unwrap().as_ref() {
            return mock(user_identifier, agent_id, amount, currency);
        }
    }
    
    let canister_id = get_agent_canister_id()?;
    
    ic_cdk::println!("📤 [AGENT_CLIENT] Calling create_deposit_request: user={}, agent={}, amount={}, currency={:?}", 
        user_identifier, agent_id, amount, currency);
    
    let request = CreateDepositRequest {
        amount,
        agent_id: agent_id.clone(),
        currency: currency.code().to_string(),
        user_id: user_identifier.clone(),
    };
    
    let response = Call::unbounded_wait(canister_id, "create_deposit_request")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [AGENT_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<DepositTransaction, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [AGENT_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(deposit) => ic_cdk::println!("✅ [AGENT_CLIENT] Deposit created: {} (status={:?})", 
            deposit.deposit_code, deposit.status),
        Err(e) => ic_cdk::println!("❌ [AGENT_CLIENT] Create deposit failed: {}", e),
    }
    
    result
}

/// Get deposit status by code
pub async fn get_deposit_status(deposit_code: String) -> Result<DepositTransaction, String> {
    let canister_id = get_agent_canister_id()?;
    
    ic_cdk::println!("📤 [AGENT_CLIENT] Calling get_deposit_status: code={}", deposit_code);
    
    let response = Call::unbounded_wait(canister_id, "get_deposit_status")
        .with_args(&(deposit_code.clone(),))
        .await
        .map_err(|e| format!("❌ [AGENT_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<DepositTransaction, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [AGENT_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(deposit) => ic_cdk::println!("✅ [AGENT_CLIENT] Got deposit: {} (status={:?})", 
            deposit_code, deposit.status),
        Err(e) => ic_cdk::println!("❌ [AGENT_CLIENT] Get deposit status failed: {}", e),
    }
    
    result
}

// ============================================================================
// WITHDRAWAL OPERATIONS
// ============================================================================

/// Get withdrawal fees estimate
pub async fn get_withdrawal_fees(amount: u64) -> Result<WithdrawalFeesResponse, String> {
    let canister_id = get_agent_canister_id()?;
    
    ic_cdk::println!("📤 [AGENT_CLIENT] Calling get_withdrawal_fees: amount={}", amount);
    
    let response = Call::unbounded_wait(canister_id, "get_withdrawal_fees")
        .with_args(&(amount,))
        .await
        .map_err(|e| format!("❌ [AGENT_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<WithdrawalFeesResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [AGENT_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(fees) => ic_cdk::println!("✅ [AGENT_CLIENT] Withdrawal fees: total={}, net={}", 
            fees.total_fees, fees.net_amount),
        Err(e) => ic_cdk::println!("❌ [AGENT_CLIENT] Get withdrawal fees failed: {}", e),
    }
    
    result
}

/// Create withdrawal request (user wants cash from agent)
pub async fn create_withdrawal_request(
    user_identifier: String,
    agent_id: String,
    amount: u64,
    currency: FiatCurrency,
    pin: String,
) -> Result<WithdrawalTransaction, String> {
    #[cfg(any(test, feature = "test-utils"))]
    {
        if let Some(mock) = MOCK_CREATE_WITHDRAWAL_REQUEST.lock().unwrap().as_ref() {
            return mock(user_identifier, agent_id, amount, currency, pin);
        }
    }
    
    let canister_id = get_agent_canister_id()?;
    
    ic_cdk::println!("📤 [AGENT_CLIENT] Calling create_withdrawal_request: user={}, agent={}, amount={}, currency={:?}", 
        user_identifier, agent_id, amount, currency);
    
    let request = CreateWithdrawalRequest {
        user_id: user_identifier.clone(),
        agent_id: agent_id.clone(),
        amount,
        currency: currency.code().to_string(),
        pin,
    };
    
    let response = Call::unbounded_wait(canister_id, "create_withdrawal_request")
        .with_args(&(request,))
        .await
        .map_err(|e| format!("❌ [AGENT_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<CreateWithdrawalResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [AGENT_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(resp) => ic_cdk::println!("✅ [AGENT_CLIENT] Withdrawal created: code={}, net={}",
            resp.withdrawal_code, resp.net_to_user),
        Err(e) => ic_cdk::println!("❌ [AGENT_CLIENT] Create withdrawal failed: {}", e),
    }

    // Convert response to WithdrawalTransaction
    result.map(|resp| {
        // Calculate individual fees from total
        let platform_fee = (resp.amount as f64 * 0.005).round() as u64;  // 0.5%
        let agent_fee = (resp.amount as f64 * 0.10).round() as u64;      // 10%

        WithdrawalTransaction {
            id: "".to_string(),
            user_id: user_identifier,
            agent_id,
            amount: resp.amount,
            currency: resp.currency.clone(),
            status: shared_types::AgentTransactionStatus::Pending,
            withdrawal_code: resp.withdrawal_code,
            timestamp: 0,
            agent_fee,
            agent_keeps: agent_fee,
            platform_revenue: platform_fee,
            confirmed_at: None,
        }
    })
}

/// Get withdrawal status by code
pub async fn get_withdrawal_status(withdrawal_code: String) -> Result<WithdrawalTransaction, String> {
    let canister_id = get_agent_canister_id()?;
    
    ic_cdk::println!("📤 [AGENT_CLIENT] Calling get_withdrawal_status: code={}", withdrawal_code);
    
    let response = Call::unbounded_wait(canister_id, "get_withdrawal_status")
        .with_args(&(withdrawal_code.clone(),))
        .await
        .map_err(|e| format!("❌ [AGENT_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<WithdrawalTransaction, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [AGENT_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(withdrawal) => ic_cdk::println!("✅ [AGENT_CLIENT] Got withdrawal: {} (status={:?})", 
            withdrawal_code, withdrawal.status),
        Err(e) => ic_cdk::println!("❌ [AGENT_CLIENT] Get withdrawal status failed: {}", e),
    }
    
    result
}

// ============================================================================
// AGENT BALANCE & INFO
// ============================================================================

/// Get agent balance for specific currency
pub async fn get_agent_balance(agent_id: String, currency: String) -> Result<AgentBalanceResponse, String> {
    let canister_id = get_agent_canister_id()?;
    
    ic_cdk::println!("📤 [AGENT_CLIENT] Calling get_agent_balance: agent={}, currency={}", 
        agent_id, currency);
    
    let response = Call::unbounded_wait(canister_id, "get_agent_balance")
        .with_args(&(agent_id.clone(), currency.clone()))
        .await
        .map_err(|e| format!("❌ [AGENT_CLIENT] Call failed: {:?}", e))?;
    
    let (result,): (Result<AgentBalanceResponse, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("❌ [AGENT_CLIENT] Decode failed: {}", e))?;
    
    match &result {
        Ok(balance) => ic_cdk::println!("✅ [AGENT_CLIENT] Agent balance: {} {} = deposits:{}, withdrawals:{}", 
            agent_id, currency, balance.total_deposits, balance.total_withdrawals),
        Err(e) => ic_cdk::println!("❌ [AGENT_CLIENT] Get agent balance failed: {}", e),
    }
    
    result
}

// ============================================================================
// NEARBY AGENTS (for finding agents)
// ============================================================================

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub location: String,
    pub currencies: Vec<String>,
    pub rating: f64,
}

/// Get nearby agents (simplified - in real implementation would use geolocation)
/// For USSD, we just return available agents for the currency
pub async fn get_nearby_agents(currency: String) -> Result<Vec<AgentInfo>, String> {
    // For now, return mock data since we don't have a get_nearby_agents endpoint yet
    // In production, this would call the agent_canister
    ic_cdk::println!("📤 [AGENT_CLIENT] Getting nearby agents for currency: {}", currency);
    
    // TODO: Implement actual endpoint in agent_canister
    // For now, return empty list
    Ok(Vec::new())
}
