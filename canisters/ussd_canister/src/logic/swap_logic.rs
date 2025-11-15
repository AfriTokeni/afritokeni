/// Pure presentation logic for crypto swap flow
/// No I/O, no async, no IC calls - fully testable

use crate::logic::validation;

/// Crypto type for swaps
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoType {
    CkBTC,
    CkUSD,
}

impl CryptoType {
    pub fn from_menu_choice(choice: &str) -> Option<Self> {
        match choice {
            "1" => Some(CryptoType::CkBTC),
            "2" => Some(CryptoType::CkUSD),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            CryptoType::CkBTC => "CkBTC",
            CryptoType::CkUSD => "CkUSD",
        }
    }
}

/// Parse USSD input and determine current step
pub fn determine_step(text: &str) -> usize {
    let parts: Vec<&str> = text.split('*').collect();
    if parts.len() <= 2 {
        0
    } else {
        parts.len() - 2
    }
}

/// Extract "from" crypto choice from USSD input
pub fn extract_from_crypto(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(2).map(|s| s.to_string())
}

/// Extract "to" crypto choice from USSD input
pub fn extract_to_crypto(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(3).map(|s| s.to_string())
}

/// Extract amount from USSD input
pub fn extract_amount(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(4).map(|s| s.to_string())
}

/// Extract PIN from USSD input
pub fn extract_pin(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('*').collect();
    parts.get(5).map(|s| s.to_string())
}

/// Validate swap pair (can't swap same crypto)
pub fn validate_swap_pair(from: &str, to: &str) -> Result<(CryptoType, CryptoType), String> {
    if from == to {
        return Err("Cannot swap same cryptocurrency".to_string());
    }

    let from_crypto = CryptoType::from_menu_choice(from)
        .ok_or_else(|| "Invalid from cryptocurrency".to_string())?;
    
    let to_crypto = CryptoType::from_menu_choice(to)
        .ok_or_else(|| "Invalid to cryptocurrency".to_string())?;

    Ok((from_crypto, to_crypto))
}

/// Validate swap amount
pub fn validate_swap_amount(text: &str) -> Result<f64, String> {
    let amount_str = extract_amount(text)
        .ok_or_else(|| "No amount provided".to_string())?;
    
    validation::parse_and_validate_amount(&amount_str)
}

/// Validate swap PIN
pub fn validate_swap_pin(text: &str) -> Result<String, String> {
    let pin = extract_pin(text)
        .ok_or_else(|| "No PIN provided".to_string())?;
    
    validation::validate_pin_format(&pin)?;
    Ok(pin)
}

/// Calculate spread amount (in basis points)
pub fn calculate_spread_amount(amount: f64, spread_bps: u64) -> f64 {
    amount * (spread_bps as f64 / 10000.0)
}

/// Calculate net amount after spread
pub fn calculate_net_after_spread(amount: f64, spread_bps: u64) -> f64 {
    amount - calculate_spread_amount(amount, spread_bps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_type_from_menu_choice() {
        assert_eq!(CryptoType::from_menu_choice("1"), Some(CryptoType::CkBTC));
        assert_eq!(CryptoType::from_menu_choice("2"), Some(CryptoType::CkUSD));
        assert_eq!(CryptoType::from_menu_choice("3"), None);
    }

    #[test]
    fn test_crypto_type_to_string() {
        assert_eq!(CryptoType::CkBTC.to_string(), "CkBTC");
        assert_eq!(CryptoType::CkUSD.to_string(), "CkUSD");
    }

    #[test]
    fn test_determine_step() {
        assert_eq!(determine_step("4"), 0);
        assert_eq!(determine_step("4*1"), 0);
        assert_eq!(determine_step("4*1*2"), 1);
        assert_eq!(determine_step("4*1*2*1000"), 2);
        assert_eq!(determine_step("4*1*2*1000*1234"), 3);
    }

    #[test]
    fn test_extract_from_crypto() {
        assert_eq!(extract_from_crypto("4*1*1"), Some("1".to_string()));
        assert_eq!(extract_from_crypto("4*1"), None);
    }

    #[test]
    fn test_extract_to_crypto() {
        assert_eq!(extract_to_crypto("4*1*1*2"), Some("2".to_string()));
        assert_eq!(extract_to_crypto("4*1*1"), None);
    }

    #[test]
    fn test_extract_amount() {
        assert_eq!(extract_amount("4*1*1*2*1000"), Some("1000".to_string()));
        assert_eq!(extract_amount("4*1*1*2"), None);
    }

    #[test]
    fn test_extract_pin() {
        assert_eq!(extract_pin("4*1*1*2*1000*1234"), Some("1234".to_string()));
        assert_eq!(extract_pin("4*1*1*2*1000"), None);
    }

    #[test]
    fn test_validate_swap_pair_valid() {
        let result = validate_swap_pair("1", "2");
        assert!(result.is_ok());
        let (from, to) = result.unwrap();
        assert_eq!(from, CryptoType::CkBTC);
        assert_eq!(to, CryptoType::CkUSD);
    }

    #[test]
    fn test_validate_swap_pair_same_crypto() {
        assert!(validate_swap_pair("1", "1").is_err());
        assert!(validate_swap_pair("2", "2").is_err());
    }

    #[test]
    fn test_validate_swap_pair_invalid() {
        assert!(validate_swap_pair("1", "3").is_err());
        assert!(validate_swap_pair("0", "2").is_err());
    }

    #[test]
    fn test_validate_swap_amount_valid() {
        let result = validate_swap_amount("4*1*1*2*1000");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1000.0);
    }

    #[test]
    fn test_validate_swap_amount_invalid() {
        assert!(validate_swap_amount("4*1*1*2*abc").is_err());
        assert!(validate_swap_amount("4*1*1*2*0").is_err());
        assert!(validate_swap_amount("4*1*1*2").is_err());
    }

    #[test]
    fn test_validate_swap_pin_valid() {
        let result = validate_swap_pin("4*1*1*2*1000*1234");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1234");
    }

    #[test]
    fn test_validate_swap_pin_invalid() {
        assert!(validate_swap_pin("4*1*1*2*1000*123").is_err());
        assert!(validate_swap_pin("4*1*1*2*1000").is_err());
    }

    #[test]
    fn test_calculate_spread_amount() {
        // 30 bps = 0.3%
        assert_eq!(calculate_spread_amount(1000.0, 30), 3.0);
        // 50 bps = 0.5%
        assert_eq!(calculate_spread_amount(10000.0, 50), 50.0);
    }

    #[test]
    fn test_calculate_net_after_spread() {
        // 1000 - 3 (0.3%) = 997
        assert_eq!(calculate_net_after_spread(1000.0, 30), 997.0);
        // 10000 - 50 (0.5%) = 9950
        assert_eq!(calculate_net_after_spread(10000.0, 50), 9950.0);
    }
}
