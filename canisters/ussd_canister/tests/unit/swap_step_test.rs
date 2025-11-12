// Unit test to verify swap flow step calculation
#[cfg(test)]
mod swap_step_tests {
    #[test]
    fn test_step_calculation() {
        // Test different input patterns
        let test_cases = vec![
            ("4", 1, 0),                          // Just menu
            ("4*1", 2, 0),                        // Menu + from
            ("4*1*2", 3, 1),                      // Menu + from + to
            ("4*1*2*50000", 4, 2),                // Menu + from + to + amount
            ("4*1*2*50000*1", 5, 3),              // Menu + from + to + amount + confirm
            ("4*1*2*50000*1*1234", 6, 4),         // Full path with PIN
        ];
        
        for (text, expected_len, expected_step) in test_cases {
            let parts: Vec<&str> = text.split('*').collect();
            let step = if parts.len() <= 2 { 0 } else { parts.len() - 2 };
            
            println!("Text: '{}' -> parts={:?}, len={}, step={}", text, parts, parts.len(), step);
            
            assert_eq!(parts.len(), expected_len, "Parts length mismatch for '{}'", text);
            assert_eq!(step, expected_step, "Step mismatch for '{}'", text);
        }
    }
    
    #[test]
    fn test_parts_indices() {
        let text = "4*1*2*50000*1*1234";
        let parts: Vec<&str> = text.split('*').collect();
        
        println!("Full path: {:?}", parts);
        println!("parts[0] (menu): {}", parts[0]);
        println!("parts[1] (from): {}", parts[1]);
        println!("parts[2] (to): {}", parts[2]);
        println!("parts[3] (amount): {}", parts[3]);
        println!("parts[4] (confirm): {}", parts[4]);
        println!("parts[5] (pin): {}", parts[5]);
        
        assert_eq!(parts[0], "4");      // Menu
        assert_eq!(parts[1], "1");      // From BTC
        assert_eq!(parts[2], "2");      // To USDC
        assert_eq!(parts[3], "50000");  // Amount
        assert_eq!(parts[4], "1");      // Confirm
        assert_eq!(parts[5], "1234");   // PIN
    }
}
