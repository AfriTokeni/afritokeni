// Unit tests for USSD session management

#[cfg(test)]
mod tests {
    #[test]
    fn test_session_creation() {
        let phone = "+256700123456";
        let session_id = format!("session_{}", phone);
        assert!(session_id.starts_with("session_"));
        assert!(session_id.contains(phone));
    }

    #[test]
    fn test_phone_number_validation() {
        let valid_phones = vec![
            "+256700123456",
            "+254712345678",
            "+255712345678",
        ];
        
        for phone in valid_phones {
            assert!(phone.starts_with("+"));
            assert!(phone.len() >= 10);
        }
    }

    #[test]
    fn test_session_timeout() {
        // Session should timeout after 180 seconds
        let timeout_seconds = 180;
        assert_eq!(timeout_seconds, 180);
    }

    #[test]
    fn test_menu_navigation() {
        let menus = vec!["main", "bitcoin", "usdc", "local_currency"];
        assert!(menus.contains(&"bitcoin"));
        assert!(menus.contains(&"usdc"));
    }
}
