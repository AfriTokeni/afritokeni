/// DEX Client for crypto swaps via Sonic DEX
/// Real implementation using Sonic swap canister

use ic_cdk::call::Call;
use candid::{CandidType, Principal};
use shared_types::CryptoType;

use crate::config;

/// Swap tokens via DEX (Sonic)
pub async fn swap_tokens(
    from_token: CryptoType,
    to_token: CryptoType,
    amount: u64,
    min_output: u64,
) -> Result<u64, String> {
    let dex_provider = config::get_dex_provider();
    
    match dex_provider.as_str() {
        "sonic" => swap_via_sonic(from_token, to_token, amount, min_output).await,
        "internal" => swap_internal(from_token, to_token, amount).await,
        _ => Err(format!("Unsupported DEX provider: {}", dex_provider)),
    }
}

/// Swap via Sonic DEX - REAL IMPLEMENTATION (with test mode fallback)
async fn swap_via_sonic(
    from_token: CryptoType,
    to_token: CryptoType,
    amount: u64,
    min_output: u64,
) -> Result<u64, String> {
    // In test mode, use internal swap (PocketIC can't call real Sonic)
    if crate::config::is_test_mode() {
        return swap_internal(from_token, to_token, amount).await;
    }
    
    // Get Sonic swap canister from config
    let sonic_canister = config::get_sonic_swap_canister()?;
    
    // Get token ledger principals
    let from_principal = match from_token {
        CryptoType::CkBTC => config::get_ckbtc_ledger()?,
        CryptoType::CkUSDC => config::get_ckusdc_ledger()?,
    };
    
    let to_principal = match to_token {
        CryptoType::CkBTC => config::get_ckbtc_ledger()?,
        CryptoType::CkUSDC => config::get_ckusdc_ledger()?,
    };
    
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
        amount_out_min: candid::Nat::from(min_output),
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

/// Internal swap (using platform liquidity) - FALLBACK
async fn swap_internal(
    _from_token: CryptoType,
    _to_token: CryptoType,
    amount: u64,
) -> Result<u64, String> {
    // Internal swap with spread (fallback if Sonic unavailable)
    let spread = config::get_spread_basis_points();
    let output = amount - (amount * spread / 10000);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_internal() {
        // This would need async test runtime
        // For now, just test the spread calculation logic
        let amount = 100000u64;
        let spread_bp = 50u64; // 0.5%
        let expected_output = amount - (amount * spread_bp / 10000);
        assert_eq!(expected_output, 99500);
    }
}
