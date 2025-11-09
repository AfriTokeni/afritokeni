use business_logic_canister::logic::*;
use candid::Principal;

#[test]
fn test_validate_amount_positive() {
    assert!(validate_amount_positive(100).is_ok());
    assert!(validate_amount_positive(0).is_err());
}

#[test]
fn test_validate_phone_number_format() {
    assert!(validate_phone_number_format("+254712345678").is_ok());
    assert!(validate_phone_number_format("254712345678").is_err());
    assert!(validate_phone_number_format("+254").is_err());
    assert!(validate_phone_number_format("").is_err());
}

#[test]
fn test_validate_pin_format() {
    assert!(validate_pin_format("1234").is_ok());
    assert!(validate_pin_format("123").is_err());
    assert!(validate_pin_format("12345").is_err());
    assert!(validate_pin_format("abcd").is_err());
}

#[test]
fn test_validate_crypto_address_bitcoin() {
    assert!(validate_crypto_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", "BTC").is_ok());
    assert!(validate_crypto_address("short", "BTC").is_err());
    assert!(validate_crypto_address("", "BTC").is_err());
}

#[test]
fn test_validate_crypto_address_ethereum() {
    assert!(validate_crypto_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", "USDC").is_err()); // wrong length
    assert!(validate_crypto_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0", "USDC").is_ok());
    assert!(validate_crypto_address("742d35Cc6634C0532925a3b844Bc9e7595f0bEb0", "USDC").is_err()); // no 0x
}

#[test]
fn test_validate_amount_within_limit() {
    assert!(validate_amount_within_limit(100, 1000).is_ok());
    assert!(validate_amount_within_limit(1000, 1000).is_ok());
    assert!(validate_amount_within_limit(1001, 1000).is_err());
}

#[test]
fn test_calculate_fee() {
    assert_eq!(calculate_fee(10000, 50).unwrap(), 50); // 0.5%
    assert_eq!(calculate_fee(10000, 100).unwrap(), 100); // 1%
    assert!(calculate_fee(u64::MAX, 10000).is_err()); // overflow
}

#[test]
fn test_validate_identifier_format() {
    assert!(validate_identifier_format("+254712345678").is_ok());
    assert!(validate_identifier_format("aaaaa-aa").is_ok());
    assert!(validate_identifier_format("").is_err());
    assert!(validate_identifier_format("invalid-principal").is_err());
}

#[test]
fn test_is_suspicious_amount() {
    assert!(is_suspicious_amount(1000000, 500000));
    assert!(!is_suspicious_amount(100000, 500000));
}

#[test]
fn test_is_round_number() {
    assert!(is_round_number(10000));
    assert!(is_round_number(100000));
    assert!(!is_round_number(10001));
    assert!(!is_round_number(0));
}
