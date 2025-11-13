/// Pure presentation logic for crypto flows (buy/sell/send Bitcoin and USDC)
/// No I/O, no async, no IC calls - fully testable

use crate::logic::validation;

/// Parse USSD input and determine current step
pub fn determine_step(text: &str) -> usize {
    let parts: Vec<&str> = text.split('*').collect();
    if parts.len() <= 2 {
        0
    } else {
        parts.len() - 2
    }
}

/// Extract amount from USSD input
pub fn extract_amount(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(2).map(|s| s.to_string())
}

/// Extract recipient/address from USSD input
pub fn extract_recipient(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(2).map(|s| s.to_string())
}

/// Extract PIN from USSD input (step 2 for buy, step 3 for send)
pub fn extract_pin_buy(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(3).map(|s| s.to_string())
}

pub fn extract_pin_send(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(4).map(|s| s.to_string())
}

/// Convert BTC amount to satoshis
pub fn btc_to_satoshis(btc: f64) -> u64 {
    (btc * 100_000_000.0).round() as u64
}

/// Convert satoshis to BTC
pub fn satoshis_to_btc(sats: u64) -> f64 {
    sats as f64 / 100_000_000.0
}

/// Convert USDC amount to e6 (6 decimals)
pub fn usdc_to_e6(usdc: f64) -> u64 {
    (usdc * 1_000_000.0).round() as u64
}

/// Convert e6 to USDC
pub fn e6_to_usdc(e6: u64) -> f64 {
    e6 as f64 / 1_000_000.0
}

/// Validate buy crypto step (amount + PIN)
pub fn validate_buy_amount(text: &str) -> Result<f64, String> {
    let amount_str = extract_amount(text)
        .ok_or_else(|| "No amount provided".to_string())?;
    
    validation::parse_and_validate_amount(&amount_str)
}

/// Validate sell crypto step (amount)
pub fn validate_sell_amount(text: &str) -> Result<f64, String> {
    let amount_str = extract_amount(text)
        .ok_or_else(|| "No amount provided".to_string())?;
    
    validation::parse_and_validate_amount(&amount_str)
}

/// Validate send Bitcoin step (address)
pub fn validate_btc_recipient(text: &str) -> Result<String, String> {
    let address = extract_recipient(text)
        .ok_or_else(|| "No address provided".to_string())?;
    
    validation::validate_btc_address_format(&address)?;
    Ok(address)
}

/// Validate send USDC step (phone or address)
pub fn validate_usdc_recipient(text: &str) -> Result<String, String> {
    let recipient = extract_recipient(text)
        .ok_or_else(|| "No recipient provided".to_string())?;
    
    // USDC can be sent to phone number or address
    if recipient.starts_with('+') {
        validation::validate_phone_format(&recipient)?;
    }
    // If not phone, assume it's an address (basic validation)
    
    Ok(recipient)
}

/// Check if user has sufficient crypto balance
pub fn check_sufficient_crypto_balance(balance: u64, amount: u64) -> Result<(), String> {
    if balance < amount {
        return Err(format!(
            "Insufficient balance. You have {}, need {}",
            balance, amount
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_step() {
        assert_eq!(determine_step("2*3"), 0);
        assert_eq!(determine_step("2*3*10000"), 1);
        assert_eq!(determine_step("2*3*10000*1234"), 2);
    }

    #[test]
    fn test_extract_amount() {
        assert_eq!(extract_amount("2*3*10000"), Some("10000".to_string()));
        assert_eq!(extract_amount("2*3"), None);
    }

    #[test]
    fn test_extract_recipient() {
        assert_eq!(extract_recipient("2*5*1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"), 
                   Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()));
    }

    #[test]
    fn test_btc_to_satoshis() {
        assert_eq!(btc_to_satoshis(1.0), 100_000_000);
        assert_eq!(btc_to_satoshis(0.5), 50_000_000);
        assert_eq!(btc_to_satoshis(0.00000001), 1);
    }

    #[test]
    fn test_satoshis_to_btc() {
        assert_eq!(satoshis_to_btc(100_000_000), 1.0);
        assert_eq!(satoshis_to_btc(50_000_000), 0.5);
        assert_eq!(satoshis_to_btc(1), 0.00000001);
    }

    #[test]
    fn test_usdc_to_e6() {
        assert_eq!(usdc_to_e6(100.0), 100_000_000);
        assert_eq!(usdc_to_e6(50.5), 50_500_000);
        assert_eq!(usdc_to_e6(0.000001), 1);
    }

    #[test]
    fn test_e6_to_usdc() {
        assert_eq!(e6_to_usdc(100_000_000), 100.0);
        assert_eq!(e6_to_usdc(50_500_000), 50.5);
        assert_eq!(e6_to_usdc(1), 0.000001);
    }

    #[test]
    fn test_validate_buy_amount_valid() {
        assert!(validate_buy_amount("2*3*10000").is_ok());
        assert_eq!(validate_buy_amount("2*3*10000").unwrap(), 10000.0);
    }

    #[test]
    fn test_validate_buy_amount_invalid() {
        assert!(validate_buy_amount("2*3*abc").is_err());
        assert!(validate_buy_amount("2*3*0").is_err());
        assert!(validate_buy_amount("2*3").is_err());
    }

    #[test]
    fn test_validate_btc_recipient_valid() {
        let result = validate_btc_recipient("2*5*1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_btc_recipient_invalid() {
        assert!(validate_btc_recipient("2*5*invalid").is_err());
        assert!(validate_btc_recipient("2*5").is_err());
    }

    #[test]
    fn test_validate_usdc_recipient_phone() {
        let result = validate_usdc_recipient("3*5*+256700000002");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "+256700000002");
    }

    #[test]
    fn test_validate_usdc_recipient_address() {
        let result = validate_usdc_recipient("3*5*0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_sufficient_crypto_balance_ok() {
        assert!(check_sufficient_crypto_balance(100_000_000, 50_000_000).is_ok());
    }

    #[test]
    fn test_check_sufficient_crypto_balance_insufficient() {
        assert!(check_sufficient_crypto_balance(50_000_000, 100_000_000).is_err());
    }

    #[test]
    fn test_check_sufficient_crypto_balance_exact() {
        assert!(check_sufficient_crypto_balance(100_000_000, 100_000_000).is_ok());
    }
}
