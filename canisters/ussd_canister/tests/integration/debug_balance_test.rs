// Debug test to check balance setting
use super::*;

#[test]
fn test_debug_balance_setting() {
    let env = get_test_env();
    
    let phone = "+256700999999";
    
    // Register user
    let reg_result = env.register_user_direct(phone, "Debug", "User", "debug@test.com", "UGX", "1234");
    println!("Registration result: {:?}", reg_result);
    
    // Set balance
    let set_result = env.set_fiat_balance(phone, "UGX", 100000);
    println!("Set balance result: {:?}", set_result);
    
    // Check balance
    let check_result = env.check_fiat_balance(phone, "UGX");
    println!("Check balance result: {:?}", check_result);
    
    // Get user to see what user_id is
    let user_result = env.get_user(phone);
    println!("Get user result: {:?}", user_result);
    
    assert!(check_result.is_ok(), "Should be able to check balance");
    let balance = check_result.unwrap();
    assert_eq!(balance, 100000, "Balance should be 100000, got: {}", balance);
}
