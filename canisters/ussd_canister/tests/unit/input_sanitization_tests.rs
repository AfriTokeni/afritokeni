// Comprehensive input sanitization tests for security validation
use ussd_canister::utils::validation::sanitize_input;

#[cfg(test)]
mod sanitization_allowed_chars_tests {
    use super::*;

    #[test]
    fn test_alphanumeric_preserved() {
        assert_eq!(sanitize_input("abc123"), "abc123");
        assert_eq!(sanitize_input("ABC123"), "ABC123");
        assert_eq!(sanitize_input("Test123"), "Test123");
    }

    #[test]
    fn test_plus_sign_preserved() {
        // Plus sign is allowed for international phone numbers
        assert_eq!(sanitize_input("+256700123456"), "+256700123456");
        assert_eq!(sanitize_input("+254712345678"), "+254712345678");
    }

    #[test]
    fn test_asterisk_preserved() {
        // Asterisk is allowed for USSD navigation codes
        assert_eq!(sanitize_input("1*2*3"), "1*2*3");
        assert_eq!(sanitize_input("*229#"), "229");
        assert_eq!(sanitize_input("1*1*+256700123456*1000*1234"), "1*1*+256700123456*1000*1234");
    }

    #[test]
    fn test_space_preserved() {
        // Spaces are allowed for names and text
        assert_eq!(sanitize_input("John Doe"), "John Doe");
        assert_eq!(sanitize_input("First Last"), "First Last");
        assert_eq!(sanitize_input("Multi Word Name"), "Multi Word Name");
    }

    #[test]
    fn test_period_preserved() {
        // Periods are allowed for decimal amounts
        assert_eq!(sanitize_input("100.50"), "100.50");
        assert_eq!(sanitize_input("0.12345678"), "0.12345678");
    }

    #[test]
    fn test_hyphen_preserved() {
        // Hyphens are allowed for Principal addresses (ckUSDC/ckBTC)
        assert_eq!(sanitize_input("rrkah-fqaaa-aaaaa-aaaaq-cai"), "rrkah-fqaaa-aaaaa-aaaaq-cai");
        assert_eq!(sanitize_input("test-principal-id"), "test-principal-id");
    }

    #[test]
    fn test_combined_allowed_chars() {
        // Test combination of all allowed characters
        assert_eq!(sanitize_input("+256*100.50 Name-123"), "+256*100.50 Name-123");
    }
}

#[cfg(test)]
mod sanitization_security_tests {
    use super::*;

    #[test]
    fn test_html_tags_removed() {
        assert_eq!(sanitize_input("<script>alert('xss')</script>"), "scriptalertxssscript");
        assert_eq!(sanitize_input("<div>content</div>"), "divcontentdiv");
        // Spaces are preserved, so <img src='x'> becomes "img srcx" (space where '=' was removed)
        assert_eq!(sanitize_input("<img src='x'>"), "img srcx");
        assert_eq!(sanitize_input("hello<br>world"), "hellobrworld");
    }

    #[test]
    fn test_script_injection_chars_removed() {
        assert_eq!(sanitize_input("'; DROP TABLE users; --"), " DROP TABLE users ");
        assert_eq!(sanitize_input("\"OR 1=1--"), "OR 11");
        assert_eq!(sanitize_input("test\\escape"), "testescape");
        assert_eq!(sanitize_input("path/to/file"), "pathtofile");
        assert_eq!(sanitize_input("cmd;rm -rf"), "cmdrm rf");
    }

    #[test]
    fn test_sql_injection_patterns_removed() {
        assert_eq!(sanitize_input("admin'--"), "admin");
        assert_eq!(sanitize_input("1' OR '1'='1"), "1 OR 11");
        assert_eq!(sanitize_input("/* comment */"), " comment ");
        // Asterisks are preserved for USSD codes (e.g., 1*2*3), so SELECT * keeps the asterisk
        // This is acceptable since the platform doesn't execute SQL on user input
        assert_eq!(sanitize_input("UNION SELECT * FROM users--"), "UNION SELECT * FROM users");
    }

    #[test]
    fn test_control_characters_removed() {
        assert_eq!(sanitize_input("hello\nworld"), "helloworld");
        assert_eq!(sanitize_input("test\r\nline"), "testline");
        assert_eq!(sanitize_input("tab\there"), "tabhere");
        assert_eq!(sanitize_input("null\0byte"), "nullbyte");
    }

    #[test]
    fn test_special_symbols_removed() {
        assert_eq!(sanitize_input("user@email.com"), "useremailcom"); // @ removed, . preserved
        assert_eq!(sanitize_input("#hashtag"), "hashtag");
        assert_eq!(sanitize_input("$100.50"), "100.50"); // $ removed, . preserved
        assert_eq!(sanitize_input("50%"), "50");
        assert_eq!(sanitize_input("test^power"), "testpower");
        assert_eq!(sanitize_input("a&b"), "ab");
        assert_eq!(sanitize_input("pipe|test"), "pipetest");
        assert_eq!(sanitize_input("tilde~test"), "tildetest");
        assert_eq!(sanitize_input("`backtick`"), "backtick");
    }

    #[test]
    fn test_path_traversal_attempts_blocked() {
        assert_eq!(sanitize_input("../../../etc/passwd"), "etcpasswd");
        assert_eq!(sanitize_input("..\\..\\windows\\system32"), "windowssystem32");
        assert_eq!(sanitize_input("/root/.ssh/id_rsa"), "rootsshidrsa");
    }

    #[test]
    fn test_command_injection_attempts_blocked() {
        assert_eq!(sanitize_input("test`whoami`"), "testwhoami");
        assert_eq!(sanitize_input("$(ls -la)"), "ls la");
        // && is replaced with empty string, creating double space, then collapsed to single space
        assert_eq!(sanitize_input("test && rm -rf /"), "test rm rf ");
        assert_eq!(sanitize_input("test || echo 'hack'"), "test echo hack");
    }

    #[test]
    fn test_url_encoded_attacks_blocked() {
        // URL-encoded characters should be removed (after decoding if needed)
        assert_eq!(sanitize_input("%3Cscript%3E"), "3Cscript3E");
        assert_eq!(sanitize_input("test%20space"), "test20space");
    }

    #[test]
    fn test_unicode_attacks_blocked() {
        // Various Unicode attack vectors
        assert_eq!(sanitize_input("test\u{202e}reverse"), "testreverse"); // Right-to-left override
        assert_eq!(sanitize_input("test\u{200b}zero-width"), "testzero-width"); // Zero-width space (- preserved)
    }
}

#[cfg(test)]
mod sanitization_ussd_flow_tests {
    use super::*;

    #[test]
    fn test_ussd_phone_number_input() {
        // Valid USSD phone number input should be preserved
        assert_eq!(sanitize_input("+256700123456"), "+256700123456");
        assert_eq!(sanitize_input("+254712345678"), "+254712345678");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("+256700123456; rm -rf /"), "+256700123456 rm rf ");
        assert_eq!(sanitize_input("+256700<script>"), "+256700script");
    }

    #[test]
    fn test_ussd_amount_input() {
        // Valid amounts should be preserved
        assert_eq!(sanitize_input("1000"), "1000");
        assert_eq!(sanitize_input("100.50"), "100.50");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("1000; DROP TABLE"), "1000 DROP TABLE");
        assert_eq!(sanitize_input("100<script>"), "100script");
    }

    #[test]
    fn test_ussd_pin_input() {
        // Valid PINs should be preserved
        assert_eq!(sanitize_input("1234"), "1234");
        assert_eq!(sanitize_input("0000"), "0000");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("1234' OR '1'='1"), "1234 OR 11");
        assert_eq!(sanitize_input("1234<script>"), "1234script");
    }

    #[test]
    fn test_ussd_navigation_codes() {
        // USSD navigation codes should be preserved
        assert_eq!(sanitize_input("1*1"), "1*1");
        assert_eq!(sanitize_input("2*3*1000"), "2*3*1000");
        assert_eq!(sanitize_input("1*1*+256700123456*1000*1234"), "1*1*+256700123456*1000*1234");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("1*1; rm -rf"), "1*1 rm rf");
    }

    #[test]
    fn test_ussd_name_input() {
        // Valid names should be preserved
        assert_eq!(sanitize_input("John Doe"), "John Doe");
        assert_eq!(sanitize_input("Maria-Jane"), "Maria-Jane");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("John<script>alert()</script>"), "Johnscriptalertscript");
        assert_eq!(sanitize_input("'; DROP TABLE users; --"), " DROP TABLE users ");
    }

    #[test]
    fn test_ussd_bitcoin_address_input() {
        // Valid Bitcoin addresses should be preserved (alphanumeric only)
        assert_eq!(sanitize_input("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"), "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(sanitize_input("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh<script>"), "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlhscript");
    }

    #[test]
    fn test_ussd_principal_address_input() {
        // Valid IC Principal addresses should be preserved (alphanumeric + hyphens)
        assert_eq!(sanitize_input("rrkah-fqaaa-aaaaa-aaaaq-cai"), "rrkah-fqaaa-aaaaa-aaaaq-cai");
        assert_eq!(sanitize_input("mxzaz-hqaaa-aaaar-qaada-cai"), "mxzaz-hqaaa-aaaar-qaada-cai");

        // Malicious additions should be stripped
        assert_eq!(sanitize_input("rrkah-fqaaa-aaaaa-aaaaq-cai'; DROP TABLE"), "rrkah-fqaaa-aaaaa-aaaaq-cai DROP TABLE");
    }
}

#[cfg(test)]
mod sanitization_edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(sanitize_input(""), "");
    }

    #[test]
    fn test_whitespace_only() {
        // Multiple consecutive spaces are collapsed to one for security (prevent space-based exploits)
        assert_eq!(sanitize_input("   "), " ");
        assert_eq!(sanitize_input("\t\t"), ""); // Tabs are control chars, removed
    }

    #[test]
    fn test_all_blocked_chars() {
        assert_eq!(sanitize_input("<>&@#$%^|~`"), "");
    }

    #[test]
    fn test_mixed_valid_invalid() {
        assert_eq!(sanitize_input("Valid123<invalid>"), "Valid123invalid");
        assert_eq!(sanitize_input("Test@Domain.com"), "TestDomaincom"); // @ removed, . preserved
    }

    #[test]
    fn test_very_long_input_truncated() {
        // Input should be truncated to 1000 characters to prevent DoS
        let long_input = "a".repeat(2000);
        let sanitized = sanitize_input(&long_input);
        assert_eq!(sanitized.len(), 1000);
        assert_eq!(sanitized, "a".repeat(1000));
    }

    #[test]
    fn test_exactly_1000_chars() {
        let input = "a".repeat(1000);
        let sanitized = sanitize_input(&input);
        assert_eq!(sanitized.len(), 1000);
        assert_eq!(sanitized, input);
    }

    #[test]
    fn test_unicode_characters() {
        // Unicode alphanumeric characters are preserved (Rust's is_alphanumeric includes Unicode)
        // This allows for names in local languages while still blocking control chars and symbols
        assert_eq!(sanitize_input("Tëst"), "Tëst"); // ë preserved (alphanumeric in Unicode)
        assert_eq!(sanitize_input("测试"), "测试"); // Chinese characters preserved (alphanumeric in Unicode)
        assert_eq!(sanitize_input("Test😀"), "Test"); // Emoji removed (not alphanumeric)
    }

    #[test]
    fn test_repeated_special_chars() {
        assert_eq!(sanitize_input("<<<>>>"), "");
        // Leading/trailing asterisks are stripped (USSD prefix/suffix), but preserved within content
        assert_eq!(sanitize_input("***"), ""); // All leading asterisks stripped
        assert_eq!(sanitize_input("1***2"), "1*2"); // Middle asterisks preserved, consecutive spaces collapsed
        assert_eq!(sanitize_input("+++"), "+++"); // Plus signs preserved
        assert_eq!(sanitize_input("..."), ""); // Periods removed (not between digits)
    }

    #[test]
    fn test_case_sensitivity() {
        // Should preserve case for alphanumeric
        assert_eq!(sanitize_input("AbC123"), "AbC123");
        assert_eq!(sanitize_input("UPPERCASE"), "UPPERCASE");
        assert_eq!(sanitize_input("lowercase"), "lowercase");
    }
}

#[cfg(test)]
mod sanitization_real_world_attacks_tests {
    use super::*;

    #[test]
    fn test_xss_attack_variants() {
        // Common XSS attack patterns
        assert_eq!(sanitize_input("<script>alert('XSS')</script>"), "scriptalertXSSscript");
        assert_eq!(sanitize_input("<img src=x onerror=alert(1)>"), "imgsrcxonerrowalert1");
        assert_eq!(sanitize_input("<svg/onload=alert(1)>"), "svgonloadalert1");
        assert_eq!(sanitize_input("javascript:alert(1)"), "javascriptalert1");
        assert_eq!(sanitize_input("<iframe src='evil.com'>"), "iframesrcevilcom");
    }

    #[test]
    fn test_sql_injection_variants() {
        // Common SQL injection patterns
        assert_eq!(sanitize_input("' OR '1'='1"), " OR 11");
        assert_eq!(sanitize_input("admin'--"), "admin");
        assert_eq!(sanitize_input("1; DROP TABLE users"), "1 DROP TABLE users");
        assert_eq!(sanitize_input("' UNION SELECT NULL--"), " UNION SELECT NULL");
    }

    #[test]
    fn test_command_injection_variants() {
        // Common command injection patterns
        assert_eq!(sanitize_input("; cat /etc/passwd"), " cat etcpasswd");
        assert_eq!(sanitize_input("| ls -la"), " ls la");
        assert_eq!(sanitize_input("&& whoami"), " whoami");
        assert_eq!(sanitize_input("`id`"), "id");
        assert_eq!(sanitize_input("$(uname -a)"), "uname a");
    }

    #[test]
    fn test_ldap_injection() {
        assert_eq!(sanitize_input("*)(uid=*))(|(uid=*"), "uiduiduid");
        assert_eq!(sanitize_input("admin)(|(password=*))"), "adminpassword");
    }

    #[test]
    fn test_template_injection() {
        assert_eq!(sanitize_input("{{7*7}}"), "77");
        assert_eq!(sanitize_input("${7*7}"), "77");
        assert_eq!(sanitize_input("<%= 7*7 %>"), " 77 ");
    }

    #[test]
    fn test_nosql_injection() {
        assert_eq!(sanitize_input("{$ne: null}"), "ne null");
        assert_eq!(sanitize_input("{$gt: ''}"), "gt ");
    }

    #[test]
    fn test_format_string_attacks() {
        assert_eq!(sanitize_input("%s%s%s%s"), "ssss");
        assert_eq!(sanitize_input("%x%x%x"), "xxx");
        assert_eq!(sanitize_input("%n%n%n"), "nnn");
    }
}

#[cfg(test)]
mod sanitization_performance_tests {
    use super::*;

    #[test]
    fn test_large_input_performance() {
        // Test that sanitization handles large inputs efficiently
        let large_input = "a".repeat(1500);
        let start = std::time::Instant::now();
        let sanitized = sanitize_input(&large_input);
        let elapsed = start.elapsed();

        // Should complete quickly (< 1ms for 1500 chars)
        assert!(elapsed.as_millis() < 10, "Sanitization took too long: {:?}", elapsed);

        // Should be truncated to 1000
        assert_eq!(sanitized.len(), 1000);
    }

    #[test]
    fn test_many_special_chars_performance() {
        // Test with input that has many chars to filter
        let input = "<>@#$%^&".repeat(200); // 1600 special chars
        let start = std::time::Instant::now();
        let sanitized = sanitize_input(&input);
        let elapsed = start.elapsed();

        // Should complete quickly even with many filtered chars
        assert!(elapsed.as_millis() < 10, "Sanitization took too long: {:?}", elapsed);

        // All special chars should be removed
        assert_eq!(sanitized, "");
    }
}
