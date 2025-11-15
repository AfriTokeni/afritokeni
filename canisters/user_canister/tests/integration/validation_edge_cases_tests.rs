use super::*;

// ============================================================================
// E.164 Phone Number Validation Edge Cases
// Tests comprehensive E.164 standard compliance
// ============================================================================

#[test]
fn test_e164_valid_international_formats() {
    let env = TestEnv::new();

    // Valid E.164 formats from different countries
    let valid_phones = vec![
        ("+256700123456", "UGX"),  // Uganda
        ("+254712345678", "KES"),  // Kenya
        ("+255712345678", "TZS"),  // Tanzania
        ("+234803456789", "NGN"),  // Nigeria (10 digits)
        ("+27821234567", "ZAR"),   // South Africa (9 digits)
        ("+12025551234", "KES"),   // USA (10 digits)
        ("+441234567890", "KES"),  // UK (10 digits)
        ("+8613800138000", "KES"), // China (11 digits)
    ];

    for (i, (phone, currency)) in valid_phones.iter().enumerate() {
        let email = format!("user{}@example.com", i);
        let result = env.register_user(
            Some(phone.to_string()),
            None,
            "Test",
            "User",
            &email,
            currency,
            "1234",
        );

        assert!(
            result.is_ok(),
            "Valid E.164 phone {} should be accepted: {:?}",
            phone,
            result
        );
    }
}

#[test]
fn test_e164_minimum_length_edge_cases() {
    let env = TestEnv::new();

    // Exactly 8 digits after + (minimum valid)
    let result = env.register_user(
        Some("+12345678".to_string()),
        None,
        "Test",
        "User",
        "test1@example.com",
        "KES",
        "1234",
    );
    assert!(result.is_ok(), "8 digits should be minimum valid length");

    // 7 digits after + (too short)
    let result = env.register_user(
        Some("+1234567".to_string()),
        None,
        "Test",
        "User",
        "test2@example.com",
        "KES",
        "1234",
    );
    assert!(result.is_err(), "7 digits should be too short");
    assert!(result.unwrap_err().contains("too short"));
}

#[test]
fn test_e164_maximum_length_edge_cases() {
    let env = TestEnv::new();

    // Exactly 15 digits after + (maximum valid per E.164)
    let result = env.register_user(
        Some("+123456789012345".to_string()),
        None,
        "Test",
        "User",
        "test1@example.com",
        "KES",
        "1234",
    );
    assert!(result.is_ok(), "15 digits should be maximum valid length");

    // 16 digits after + (too long)
    let result = env.register_user(
        Some("+1234567890123456".to_string()),
        None,
        "Test",
        "User",
        "test2@example.com",
        "KES",
        "1234",
    );
    assert!(result.is_err(), "16 digits should be too long");
    assert!(result.unwrap_err().contains("too long"));
}

#[test]
fn test_e164_invalid_formats() {
    let env = TestEnv::new();

    let invalid_phones = vec![
        ("256700123456", "Missing + prefix"),
        ("00256700123456", "International prefix instead of +"),
        ("+256 700 123 456", "Contains spaces"),
        ("+256-700-123-456", "Contains hyphens"),
        ("+256(700)123456", "Contains parentheses"),
        ("+256.700.123.456", "Contains dots"),
        ("+256700123456x789", "Contains extension"),
        ("+256700123456#", "Contains hash"),
        ("+256700123456*", "Contains asterisk"),
        ("+256abc123456", "Contains letters"),
        ("+", "Only plus sign"),
        ("", "Empty string"),
        ("+256", "Too short (country code only)"),
    ];

    for (i, (phone, reason)) in invalid_phones.iter().enumerate() {
        let email = format!("test{}@example.com", i);
        let result = env.register_user(
            Some(phone.to_string()),
            None,
            "Test",
            "User",
            &email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_err(),
            "Invalid phone '{}' should be rejected ({})",
            phone,
            reason
        );
    }
}

#[test]
fn test_e164_leading_zeros_after_country_code() {
    let env = TestEnv::new();

    // Some countries have leading zeros in national numbers
    // These should be REMOVED in E.164 format
    // E.164: +256700123456 (Uganda, national: 0700123456)

    // Correct E.164 (no leading zero after country code)
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Test",
        "User",
        "test1@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_ok(), "Proper E.164 without national prefix should work");

    // Incorrect E.164 (has leading zero - this is national format)
    let result = env.register_user(
        Some("+2560700123456".to_string()),
        None,
        "Test",
        "User",
        "test2@example.com",
        "UGX",
        "1234",
    );
    // This might be too long (16 digits)
    assert!(result.is_err(), "E.164 should not include national prefix zero");
}

#[test]
fn test_e164_special_short_codes_rejected() {
    let env = TestEnv::new();

    // Short codes and emergency numbers should be rejected
    let short_codes = vec![
        "+911",      // US emergency
        "+999",      // UK emergency
        "+112",      // EU emergency
        "+100",      // Various services
        "+256*229#", // USSD codes
    ];

    for (i, code) in short_codes.iter().enumerate() {
        let email = format!("test{}@example.com", i);
        let result = env.register_user(
            Some(code.to_string()),
            None,
            "Test",
            "User",
            &email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_err(),
            "Short code '{}' should be rejected",
            code
        );
    }
}

// ============================================================================
// RFC 5322 Email Validation Edge Cases
// Tests comprehensive RFC 5322 compliance
// ============================================================================

#[test]
fn test_rfc5322_valid_email_formats() {
    let env = TestEnv::new();

    let valid_emails = vec![
        "simple@example.com",
        "user.name@example.com",
        "user+tag@example.com",
        "user_name@example.com",
        "user-name@example.com",
        "123@example.com",
        "a@example.co.uk",
        "test@subdomain.example.com",
        "user@example-domain.com",
        "user@123.456.789.012",  // IP-like domain (technically valid)
    ];

    for (i, email) in valid_emails.iter().enumerate() {
        let phone = format!("+25670012345{}", i);
        let result = env.register_user(
            Some(phone),
            None,
            "Test",
            "User",
            email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_ok(),
            "Valid RFC 5322 email '{}' should be accepted: {:?}",
            email,
            result
        );
    }
}

#[test]
fn test_rfc5322_local_part_length_limits() {
    let env = TestEnv::new();

    // Exactly 64 characters (maximum for local part)
    let local_64 = "a".repeat(64);
    let email_max = format!("{}@example.com", local_64);
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Test",
        "User",
        &email_max,
        "UGX",
        "1234",
    );
    assert!(result.is_ok(), "64-char local part should be valid");

    // 65 characters (too long)
    let local_65 = "a".repeat(65);
    let email_too_long = format!("{}@example.com", local_65);
    let result = env.register_user(
        Some("+256700123457".to_string()),
        None,
        "Test",
        "User",
        &email_too_long,
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "65-char local part should be rejected");
    assert!(result.unwrap_err().contains("too long"));
}

#[test]
fn test_rfc5322_domain_part_length_limits() {
    let env = TestEnv::new();

    // Exactly 255 characters (maximum for domain part)
    // Format: aaa...aaa.com (251 a's + .com = 255)
    let domain_251 = "a".repeat(251);
    let email_max = format!("user@{}.com", domain_251);
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Test",
        "User",
        &email_max,
        "UGX",
        "1234",
    );
    assert!(result.is_ok(), "255-char domain should be valid");

    // 256 characters (too long)
    let domain_252 = "a".repeat(252);
    let email_too_long = format!("user@{}.com", domain_252);
    let result = env.register_user(
        Some("+256700123457".to_string()),
        None,
        "Test",
        "User",
        &email_too_long,
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "256-char domain should be rejected");
    assert!(result.unwrap_err().contains("too long"));
}

#[test]
fn test_rfc5322_invalid_email_formats() {
    let env = TestEnv::new();

    let invalid_emails = vec![
        ("", "Empty string"),
        ("@example.com", "Missing local part"),
        ("user@", "Missing domain"),
        ("user", "Missing @ and domain"),
        ("user@@example.com", "Double @"),
        ("user@example", "Missing TLD"),
        ("user @example.com", "Space in local part"),
        ("user@exam ple.com", "Space in domain"),
        ("user@.example.com", "Domain starts with dot"),
        ("user@example.com.", "Domain ends with dot"),
        (".user@example.com", "Local starts with dot"),
        ("user.@example.com", "Local ends with dot"),
        ("user@-example.com", "Domain starts with hyphen"),
        ("user@example-.com", "Domain ends with hyphen"),
        ("user@example.c", "TLD too short (1 char)"),
        ("user@example.123", "TLD is numeric"),
        ("user@example..com", "Consecutive dots in domain"),
        ("user..name@example.com", "Consecutive dots in local"),
    ];

    for (i, (email, reason)) in invalid_emails.iter().enumerate() {
        let phone = format!("+25670012345{}", i);
        let result = env.register_user(
            Some(phone),
            None,
            "Test",
            "User",
            email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_err(),
            "Invalid email '{}' should be rejected ({})",
            email,
            reason
        );
    }
}

#[test]
fn test_rfc5322_tld_validation() {
    let env = TestEnv::new();

    // Valid TLDs (at least 2 characters, alphabetic)
    let valid_tlds = vec![
        "user@example.co",
        "user@example.com",
        "user@example.org",
        "user@example.net",
        "user@example.info",
        "user@example.io",
        "user@example.tech",
    ];

    for (i, email) in valid_tlds.iter().enumerate() {
        let phone = format!("+25670012345{}", i);
        let result = env.register_user(
            Some(phone),
            None,
            "Test",
            "User",
            email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_ok(),
            "Email with valid TLD '{}' should be accepted: {:?}",
            email,
            result
        );
    }

    // Invalid TLDs
    let invalid_tlds = vec![
        ("user@example.c", "TLD too short"),
        ("user@example.123", "TLD is numeric"),
        ("user@example.c0m", "TLD contains digit"),
        ("user@example.c-m", "TLD contains hyphen"),
    ];

    for (i, (email, reason)) in invalid_tlds.iter().enumerate() {
        let phone = format!("+25670022345{}", i);
        let result = env.register_user(
            Some(phone),
            None,
            "Test",
            "User",
            email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_err(),
            "Email '{}' should be rejected ({})",
            email,
            reason
        );
    }
}

#[test]
fn test_rfc5322_email_trimming() {
    let env = TestEnv::new();

    // Emails with whitespace should be trimmed
    let email_with_spaces = "  user@example.com  ";
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Test",
        "User",
        email_with_spaces,
        "UGX",
        "1234",
    );

    // Should succeed after trimming
    assert!(
        result.is_ok(),
        "Email should be accepted after trimming whitespace: {:?}",
        result
    );
}

#[test]
fn test_rfc5322_subdomain_validation() {
    let env = TestEnv::new();

    // Valid subdomains
    let valid_subdomains = vec![
        "user@mail.example.com",
        "user@sub.mail.example.com",
        "user@a.b.c.example.com",
    ];

    for (i, email) in valid_subdomains.iter().enumerate() {
        let phone = format!("+25670012345{}", i);
        let result = env.register_user(
            Some(phone),
            None,
            "Test",
            "User",
            email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_ok(),
            "Email with subdomain '{}' should be accepted",
            email
        );
    }
}

#[test]
fn test_rfc5322_case_sensitivity() {
    let env = TestEnv::new();

    // Email addresses should be accepted with any case
    // (though case is technically preserved in local part)
    let emails = vec![
        "user@example.com",
        "User@Example.com",
        "USER@EXAMPLE.COM",
        "UsEr@ExAmPlE.CoM",
    ];

    for (i, email) in emails.iter().enumerate() {
        let phone = format!("+25670012345{}", i);
        let result = env.register_user(
            Some(phone),
            None,
            "Test",
            "User",
            email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_ok(),
            "Email '{}' should be accepted regardless of case",
            email
        );
    }
}

// ============================================================================
// Name Validation Edge Cases
// ============================================================================

#[test]
fn test_name_length_boundaries() {
    let env = TestEnv::new();

    // Exactly 2 characters (minimum valid)
    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "Jo",
        "Li",
        "test@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_ok(), "2-char names should be valid");

    // 1 character (too short)
    let result = env.register_user(
        Some("+256700123457".to_string()),
        None,
        "J",
        "Doe",
        "test2@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "1-char first name should be rejected");
    assert!(result.unwrap_err().contains("at least 2 characters"));

    // Exactly 50 characters (maximum valid)
    let long_name = "a".repeat(50);
    let result = env.register_user(
        Some("+256700123458".to_string()),
        None,
        &long_name,
        &long_name,
        "test3@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_ok(), "50-char names should be valid");

    // 51 characters (too long)
    let too_long = "a".repeat(51);
    let result = env.register_user(
        Some("+256700123459".to_string()),
        None,
        &too_long,
        "Doe",
        "test4@example.com",
        "UGX",
        "1234",
    );
    assert!(result.is_err(), "51-char first name should be rejected");
    assert!(result.unwrap_err().contains("at most 50 characters"));
}

#[test]
fn test_name_with_unicode_characters() {
    let env = TestEnv::new();

    // Names with Unicode should be accepted
    let unicode_names = vec![
        ("Müller", "German umlaut"),
        ("José", "Spanish accent"),
        ("François", "French cedilla"),
        ("Владимир", "Cyrillic"),
        ("محمد", "Arabic"),
        ("李明", "Chinese"),
        ("Nguyễn", "Vietnamese"),
        ("O'Brien", "Apostrophe"),
        ("Mary-Jane", "Hyphen"),
    ];

    for (i, (name, description)) in unicode_names.iter().enumerate() {
        let phone = format!("+25670012345{}", i);
        let email = format!("test{}@example.com", i);
        let result = env.register_user(
            Some(phone),
            None,
            name,
            "Test",
            &email,
            "UGX",
            "1234",
        );

        assert!(
            result.is_ok(),
            "Name '{}' ({}) should be accepted: {:?}",
            name,
            description,
            result
        );
    }
}

#[test]
fn test_name_empty_string() {
    let env = TestEnv::new();

    let result = env.register_user(
        Some("+256700123456".to_string()),
        None,
        "",
        "Doe",
        "test@example.com",
        "UGX",
        "1234",
    );

    assert!(result.is_err(), "Empty first name should be rejected");
    assert!(result.unwrap_err().contains("cannot be empty"));
}
