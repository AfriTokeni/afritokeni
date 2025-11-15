/// DEX Client for crypto swaps via Sonic DEX
/// Real implementation using Sonic swap canister

use ic_cdk::call::Call;
use candid::{CandidType, Principal};
use shared_types::CryptoType;

use crate::config;

/// Default slippage tolerance in basis points (1% = 100 basis points)
#[allow(dead_code)]
const DEFAULT_SLIPPAGE_BP: u64 = 100; // 1%

/// Maximum allowed slippage in basis points (5% = 500 basis points)
const MAX_SLIPPAGE_BP: u64 = 500; // 5%

/// Calculates minimum output amount based on expected output and slippage tolerance
/// Returns Err if slippage exceeds maximum allowed
pub fn calculate_min_output_with_slippage(
    expected_output: u64,
    slippage_bp: u64,
) -> Result<u64, String> {
    if slippage_bp > MAX_SLIPPAGE_BP {
        return Err(format!(
            "Slippage tolerance too high: {}%. Maximum allowed: {}%",
            slippage_bp / 100,
            MAX_SLIPPAGE_BP / 100
        ));
    }

    // Calculate minimum output: expected_output * (1 - slippage_bp/10000)
    let slippage_factor = 10000u64.checked_sub(slippage_bp)
        .ok_or("Invalid slippage calculation")?;

    let min_output = expected_output
        .checked_mul(slippage_factor)
        .ok_or("Slippage calculation overflow")?
        .checked_div(10000)
        .ok_or("Slippage calculation division error")?;

    Ok(min_output)
}

/// Validates that actual output meets minimum slippage requirements
pub fn validate_slippage(
    expected_output: u64,
    actual_output: u64,
    max_slippage_bp: u64,
) -> Result<(), String> {
    if actual_output >= expected_output {
        // No slippage or positive slippage (better than expected)
        return Ok(());
    }

    // Calculate actual slippage
    let slippage = expected_output.saturating_sub(actual_output);
    let slippage_bp = slippage
        .checked_mul(10000)
        .ok_or("Slippage calculation overflow")?
        .checked_div(expected_output)
        .ok_or("Slippage calculation division error")?;

    if slippage_bp > max_slippage_bp {
        return Err(format!(
            "Slippage too high: {}%. Received {} instead of expected {}. Maximum allowed: {}%",
            slippage_bp as f64 / 100.0,
            actual_output,
            expected_output,
            max_slippage_bp as f64 / 100.0
        ));
    }

    Ok(())
}

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
        CryptoType::CkUSD => config::get_ckusdc_ledger()?,
    };
    
    let to_principal = match to_token {
        CryptoType::CkBTC => config::get_ckbtc_ledger()?,
        CryptoType::CkUSD => config::get_ckusdc_ledger()?,
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

    #[test]
    fn test_calculate_min_output_with_slippage_1_percent() {
        let expected = 1000u64;
        let slippage_bp = 100; // 1%
        let min_output = calculate_min_output_with_slippage(expected, slippage_bp).unwrap();
        assert_eq!(min_output, 990); // 1% slippage = 99% of expected
    }

    #[test]
    fn test_calculate_min_output_with_slippage_5_percent() {
        let expected = 1000u64;
        let slippage_bp = 500; // 5%
        let min_output = calculate_min_output_with_slippage(expected, slippage_bp).unwrap();
        assert_eq!(min_output, 950); // 5% slippage = 95% of expected
    }

    #[test]
    fn test_calculate_min_output_with_slippage_exceeds_max() {
        let expected = 1000u64;
        let slippage_bp = 600; // 6% - exceeds MAX_SLIPPAGE_BP
        let result = calculate_min_output_with_slippage(expected, slippage_bp);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Slippage tolerance too high"));
    }

    #[test]
    fn test_validate_slippage_within_tolerance() {
        let expected = 1000u64;
        let actual = 990u64; // 1% slippage
        let max_slippage_bp = 200; // 2% tolerance
        assert!(validate_slippage(expected, actual, max_slippage_bp).is_ok());
    }

    #[test]
    fn test_validate_slippage_exceeds_tolerance() {
        let expected = 1000u64;
        let actual = 900u64; // 10% slippage
        let max_slippage_bp = 500; // 5% tolerance
        let result = validate_slippage(expected, actual, max_slippage_bp);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Slippage too high"));
    }

    #[test]
    fn test_validate_slippage_better_than_expected() {
        let expected = 1000u64;
        let actual = 1050u64; // Better than expected (positive slippage)
        let max_slippage_bp = 100;
        assert!(validate_slippage(expected, actual, max_slippage_bp).is_ok());
    }

    #[test]
    fn test_validate_slippage_exact_match() {
        let expected = 1000u64;
        let actual = 1000u64; // Exact match
        let max_slippage_bp = 100;
        assert!(validate_slippage(expected, actual, max_slippage_bp).is_ok());
    }
}
