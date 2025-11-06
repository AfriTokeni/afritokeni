use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call::Call;
use ic_cdk_macros::*;
use serde::Deserialize as SerdeDeserialize;
use std::cell::RefCell;

// Configuration loaded from TOML
const CONFIG_TOML: &str = include_str!("../exchange_config.toml");

#[derive(SerdeDeserialize, Clone)]
struct Config {
    company_wallet: CompanyWalletConfig,
    spread: SpreadConfig,
    dex: DexConfig,
    tokens: TokensConfig,
}

#[derive(SerdeDeserialize, Clone)]
struct CompanyWalletConfig {
    principal: String,
}

#[derive(SerdeDeserialize, Clone)]
struct SpreadConfig {
    basis_points: u64,
}

#[derive(SerdeDeserialize, Clone)]
struct DexConfig {
    provider: String,
    sonic: SonicConfig,
}

#[derive(SerdeDeserialize, Clone)]
struct SonicConfig {
    swap_canister: String,
}

#[derive(SerdeDeserialize, Clone)]
struct TokensConfig {
    ckbtc: TokenConfig,
    ckusdc: TokenConfig,
}

#[derive(SerdeDeserialize, Clone)]
struct TokenConfig {
    ledger: String,
    #[allow(dead_code)]
    decimals: u8,
}

// Runtime state
thread_local! {
    static CONFIG: RefCell<Option<Config>> = RefCell::new(None);
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ExchangeRequest {
    pub from_token: Token,
    pub to_token: Token,
    pub amount: u64,
    pub min_output: u64, // Slippage protection
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Debug)]
pub enum Token {
    CkBTC,
    CkUSDC,
}

#[derive(CandidType, Deserialize)]
pub struct ExchangeResult {
    pub output_amount: u64,
    pub spread_amount: u64,
    pub tx_id: String,
}

#[init]
fn init() {
    // Load configuration from TOML
    let config: Config = toml::from_str(CONFIG_TOML)
        .expect("Failed to parse exchange_config.toml");
    
    CONFIG.with(|c| *c.borrow_mut() = Some(config));
}

fn get_config() -> Config {
    CONFIG.with(|c| {
        c.borrow()
            .clone()
            .expect("Config not initialized. Call init() first.")
    })
}

#[update]
async fn swap_tokens(request: ExchangeRequest) -> Result<ExchangeResult, String> {
    let caller = ic_cdk::api::msg_caller();
    
    // Validate request
    if request.amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    
    if request.from_token == request.to_token {
        return Err("Cannot swap same token".to_string());
    }
    
    let config = get_config();
    
    // Get company wallet principal from config (this is YOUR revenue!)
    let company_wallet = Principal::from_text(&config.company_wallet.principal)
        .map_err(|e| format!("Invalid company wallet principal: {}", e))?;
    
    // Calculate spread from config (platform revenue)
    let spread_amount = (request.amount * config.spread.basis_points) / 10000;
    let swap_amount = request.amount - spread_amount;
    
    // Step 1: Transfer input tokens from user to this canister
    transfer_from_user(caller, request.from_token.clone(), request.amount).await?;
    
    // Step 2: Send spread to company wallet (platform revenue)
    transfer_to_company_wallet(company_wallet, request.from_token.clone(), spread_amount).await?;
    
    // Step 3: Swap remaining tokens (this would call a DEX or use internal liquidity)
    let output_amount = perform_swap(request.from_token, request.to_token.clone(), swap_amount).await?;
    
    // Check slippage
    if output_amount < request.min_output {
        return Err(format!(
            "Slippage too high. Expected at least {}, got {}",
            request.min_output, output_amount
        ));
    }
    
    // Step 4: Transfer output tokens to user
    transfer_to_user(caller, request.to_token, output_amount).await?;
    
    Ok(ExchangeResult {
        output_amount,
        spread_amount,
        tx_id: format!("{}-{}", ic_cdk::api::time(), caller.to_text()),
    })
}

// Helper: Transfer tokens from user to canister
async fn transfer_from_user(
    from: Principal,
    token: Token,
    amount: u64,
) -> Result<(), String> {
    let canister_id = get_token_canister(token)?;
    
    // ICRC-2 transferFrom pattern
    let response = Call::unbounded_wait(
        canister_id,
        "icrc2_transfer_from",
    )
    .with_arg((from, ic_cdk::api::canister_self(), amount))
    .await
    .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let result: (Result<u64, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Decode failed: {:?}", e))?;
    
    match result.0 {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Transfer from user failed: {}", e)),
    }
}

// Helper: Transfer tokens to company wallet (platform revenue)
async fn transfer_to_company_wallet(
    company_wallet: Principal,
    token: Token,
    amount: u64,
) -> Result<(), String> {
    let canister_id = get_token_canister(token)?;
    
    let response = Call::unbounded_wait(
        canister_id,
        "icrc1_transfer",
    )
    .with_arg((company_wallet, amount))
    .await
    .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let result: (Result<u64, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Decode failed: {:?}", e))?;
    
    match result.0 {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Transfer to company wallet failed: {}", e)),
    }
}

// Helper: Transfer tokens to user
async fn transfer_to_user(
    to: Principal,
    token: Token,
    amount: u64,
) -> Result<(), String> {
    let canister_id = get_token_canister(token)?;
    
    let response = Call::unbounded_wait(
        canister_id,
        "icrc1_transfer",
    )
    .with_arg((to, amount))
    .await
    .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let result: (Result<u64, String>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Decode failed: {:?}", e))?;
    
    match result.0 {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Transfer to user failed: {}", e)),
    }
}

// Helper: Perform the actual swap using Sonic DEX
async fn perform_swap(
    from_token: Token,
    to_token: Token,
    amount: u64,
) -> Result<u64, String> {
    let config = get_config();
    
    // Get Sonic swap canister from config
    let sonic_canister = Principal::from_text(&config.dex.sonic.swap_canister)
        .map_err(|e| format!("Invalid Sonic canister: {}", e))?;
    
    // Get token principals
    let from_principal = get_token_canister(from_token.clone())?;
    let to_principal = get_token_canister(to_token.clone())?;
    
    // Sonic swap parameters
    #[derive(CandidType)]
    struct SwapArgs {
        amount_in: candid::Nat,
        amount_out_min: candid::Nat,
        path: Vec<String>,
        to: Principal,
        deadline: u64,
    }
    
    let swap_args = SwapArgs {
        amount_in: candid::Nat::from(amount),
        amount_out_min: candid::Nat::from(0u64), // Will be validated by slippage check later
        path: vec![from_principal.to_text(), to_principal.to_text()],
        to: ic_cdk::api::canister_self(), // Swap to this canister
        deadline: ic_cdk::api::time() + 300_000_000_000, // 5 minutes from now
    };
    
    // Call Sonic swap
    let response = Call::unbounded_wait(
        sonic_canister,
        "swapExactTokensForTokens",
    )
    .with_arg((
        swap_args.amount_in,
        swap_args.amount_out_min,
        swap_args.path,
        swap_args.to,
        swap_args.deadline,
    ))
    .await
    .map_err(|e| format!("Sonic swap failed: {:?}", e))?;
    
    let (amounts,): (Vec<candid::Nat>,) = candid::decode_args(&response.into_bytes())
        .map_err(|e| format!("Decode failed: {:?}", e))?;
    
    // Sonic returns array of amounts [input_amount, output_amount]
    if amounts.len() < 2 {
        return Err("Invalid Sonic response".to_string());
    }
    
    // Get output amount (last element in array)
    let output = amounts.last().unwrap();
    let output_u64: u64 = output.0.clone().try_into()
        .map_err(|_| "Output amount too large".to_string())?;
    
    Ok(output_u64)
}

// Helper: Get token canister ID from config
fn get_token_canister(token: Token) -> Result<Principal, String> {
    let config = get_config();
    
    let ledger_id = match token {
        Token::CkBTC => &config.tokens.ckbtc.ledger,
        Token::CkUSDC => &config.tokens.ckusdc.ledger,
    };
    
    Principal::from_text(ledger_id)
        .map_err(|e| format!("Invalid token canister ID: {}", e))
}

#[query]
fn get_company_wallet() -> String {
    let config = get_config();
    config.company_wallet.principal
}

#[query]
fn get_spread_percentage() -> u64 {
    let config = get_config();
    config.spread.basis_points
}

#[query]
fn get_dex_provider() -> String {
    let config = get_config();
    config.dex.provider
}

#[query]
fn get_sonic_canister() -> String {
    let config = get_config();
    config.dex.sonic.swap_canister
}

// Tests module
#[cfg(test)]
mod tests;

// Export Candid interface
ic_cdk::export_candid!();
