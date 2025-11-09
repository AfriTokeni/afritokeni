use candid::Principal;

pub fn validate_amount_positive(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    Ok(())
}

pub fn validate_phone_number_format(phone: &str) -> Result<(), String> {
    if phone.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }
    if !phone.starts_with('+') {
        return Err("Phone number must start with +".to_string());
    }
    if phone.len() < 10 {
        return Err("Phone number too short".to_string());
    }
    Ok(())
}

pub fn validate_pin_format(pin: &str) -> Result<(), String> {
    if pin.len() != 4 {
        return Err("PIN must be exactly 4 digits".to_string());
    }
    if !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err("PIN must contain only digits".to_string());
    }
    Ok(())
}

pub fn validate_crypto_address(address: &str, crypto_type: &str) -> Result<(), String> {
    if address.is_empty() {
        return Err("Address cannot be empty".to_string());
    }
    
    match crypto_type {
        "BTC" | "Bitcoin" => {
            if address.len() < 26 || address.len() > 62 {
                return Err("Invalid Bitcoin address length".to_string());
            }
        }
        "USDC" | "Ethereum" => {
            if !address.starts_with("0x") || address.len() != 42 {
                return Err("Invalid Ethereum address format".to_string());
            }
        }
        _ => return Err(format!("Unsupported crypto type: {}", crypto_type)),
    }
    
    Ok(())
}

pub fn validate_amount_within_limit(amount: u64, max_limit: u64) -> Result<(), String> {
    if amount > max_limit {
        return Err(format!("Amount {} exceeds maximum limit {}", amount, max_limit));
    }
    Ok(())
}

pub fn calculate_fee(amount: u64, fee_basis_points: u64) -> Result<u64, String> {
    amount.checked_mul(fee_basis_points)
        .and_then(|v| v.checked_div(10000))
        .ok_or_else(|| "Fee calculation overflow".to_string())
}

pub fn validate_identifier_format(identifier: &str) -> Result<(), String> {
    if identifier.is_empty() {
        return Err("Identifier cannot be empty".to_string());
    }
    
    if identifier.starts_with('+') {
        validate_phone_number_format(identifier)
    } else {
        Principal::from_text(identifier)
            .map(|_| ())
            .map_err(|_| "Invalid principal format".to_string())
    }
}

pub fn is_suspicious_amount(amount: u64, threshold: u64) -> bool {
    amount >= threshold
}

pub fn is_round_number(amount: u64) -> bool {
    amount % 10000 == 0 && amount > 0
}
