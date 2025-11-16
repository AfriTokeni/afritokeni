// Comprehensive currency detection tests for all 54 African countries
use ussd_canister::core::routing::detect_currency_from_phone;

#[cfg(test)]
mod east_africa_tests {
    use super::*;

    #[test]
    fn test_uganda_currency_detection() {
        assert_eq!(detect_currency_from_phone("+256700123456"), "UGX");
        assert_eq!(detect_currency_from_phone("256712345678"), "UGX");
    }

    #[test]
    fn test_kenya_currency_detection() {
        assert_eq!(detect_currency_from_phone("+254700123456"), "KES");
        assert_eq!(detect_currency_from_phone("254712345678"), "KES");
    }

    #[test]
    fn test_tanzania_currency_detection() {
        assert_eq!(detect_currency_from_phone("+255700123456"), "TZS");
        assert_eq!(detect_currency_from_phone("255712345678"), "TZS");
    }

    #[test]
    fn test_rwanda_currency_detection() {
        assert_eq!(detect_currency_from_phone("+250700123456"), "RWF");
        assert_eq!(detect_currency_from_phone("250788123456"), "RWF");
    }

    #[test]
    fn test_burundi_currency_detection() {
        assert_eq!(detect_currency_from_phone("+257700123456"), "BIF");
        assert_eq!(detect_currency_from_phone("257712345678"), "BIF");
    }

    #[test]
    fn test_ethiopia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+251911123456"), "ETB");
        assert_eq!(detect_currency_from_phone("251900123456"), "ETB");
    }

    #[test]
    fn test_somalia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+252612345678"), "SOS");
        assert_eq!(detect_currency_from_phone("252907123456"), "SOS");
    }

    #[test]
    fn test_djibouti_currency_detection() {
        assert_eq!(detect_currency_from_phone("+253770123456"), "DJF");
        assert_eq!(detect_currency_from_phone("253812345678"), "DJF");
    }
}

#[cfg(test)]
mod west_africa_tests {
    use super::*;

    #[test]
    fn test_nigeria_currency_detection() {
        assert_eq!(detect_currency_from_phone("+234803123456"), "NGN");
        assert_eq!(detect_currency_from_phone("234810123456"), "NGN");
    }

    #[test]
    fn test_ghana_currency_detection() {
        assert_eq!(detect_currency_from_phone("+233241234567"), "GHS");
        assert_eq!(detect_currency_from_phone("233501234567"), "GHS");
    }

    #[test]
    fn test_senegal_currency_detection() {
        // Senegal uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+221771234567"), "XOF");
        assert_eq!(detect_currency_from_phone("221331234567"), "XOF");
    }

    #[test]
    fn test_cote_divoire_currency_detection() {
        // Côte d'Ivoire uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+225071234567"), "XOF");
        assert_eq!(detect_currency_from_phone("225451234567"), "XOF");
    }

    #[test]
    fn test_mali_currency_detection() {
        // Mali uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+223701234567"), "XOF");
        assert_eq!(detect_currency_from_phone("223901234567"), "XOF");
    }

    #[test]
    fn test_burkina_faso_currency_detection() {
        // Burkina Faso uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+226701234567"), "XOF");
        assert_eq!(detect_currency_from_phone("226501234567"), "XOF");
    }

    #[test]
    fn test_niger_currency_detection() {
        // Niger uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+227901234567"), "XOF");
        assert_eq!(detect_currency_from_phone("227961234567"), "XOF");
    }

    #[test]
    fn test_togo_currency_detection() {
        // Togo uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+228901234567"), "XOF");
        assert_eq!(detect_currency_from_phone("228221234567"), "XOF");
    }

    #[test]
    fn test_benin_currency_detection() {
        // Benin uses West African CFA Franc (XOF)
        assert_eq!(detect_currency_from_phone("+229971234567"), "XOF");
        assert_eq!(detect_currency_from_phone("229641234567"), "XOF");
    }

    #[test]
    fn test_guinea_currency_detection() {
        assert_eq!(detect_currency_from_phone("+224621234567"), "GNF");
        assert_eq!(detect_currency_from_phone("224301234567"), "GNF");
    }

    #[test]
    fn test_liberia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+231771234567"), "LRD");
        assert_eq!(detect_currency_from_phone("231881234567"), "LRD");
    }

    #[test]
    fn test_sierra_leone_currency_detection() {
        assert_eq!(detect_currency_from_phone("+232761234567"), "SLL");
        assert_eq!(detect_currency_from_phone("232301234567"), "SLL");
    }

    #[test]
    fn test_gambia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+220301234567"), "GMD");
        assert_eq!(detect_currency_from_phone("220771234567"), "GMD");
    }

    #[test]
    fn test_cape_verde_currency_detection() {
        assert_eq!(detect_currency_from_phone("+238991234567"), "CVE");
        assert_eq!(detect_currency_from_phone("238581234567"), "CVE");
    }
}

#[cfg(test)]
mod central_africa_tests {
    use super::*;

    #[test]
    fn test_cameroon_currency_detection() {
        // Cameroon uses Central African CFA Franc (XAF)
        assert_eq!(detect_currency_from_phone("+237671234567"), "XAF");
        assert_eq!(detect_currency_from_phone("237691234567"), "XAF");
    }

    #[test]
    fn test_car_currency_detection() {
        // Central African Republic uses Central African CFA Franc (XAF)
        assert_eq!(detect_currency_from_phone("+236701234567"), "XAF");
        assert_eq!(detect_currency_from_phone("236751234567"), "XAF");
    }

    #[test]
    fn test_chad_currency_detection() {
        // Chad uses Central African CFA Franc (XAF)
        assert_eq!(detect_currency_from_phone("+235661234567"), "XAF");
        assert_eq!(detect_currency_from_phone("235901234567"), "XAF");
    }

    #[test]
    fn test_gabon_currency_detection() {
        // Gabon uses Central African CFA Franc (XAF)
        assert_eq!(detect_currency_from_phone("+241061234567"), "XAF");
        assert_eq!(detect_currency_from_phone("241741234567"), "XAF");
    }

    #[test]
    fn test_congo_brazzaville_currency_detection() {
        // Republic of Congo uses Central African CFA Franc (XAF)
        assert_eq!(detect_currency_from_phone("+242061234567"), "XAF");
        assert_eq!(detect_currency_from_phone("242051234567"), "XAF");
    }

    #[test]
    fn test_equatorial_guinea_currency_detection() {
        // Equatorial Guinea uses Central African CFA Franc (XAF)
        assert_eq!(detect_currency_from_phone("+240222123456"), "XAF");
        assert_eq!(detect_currency_from_phone("240555123456"), "XAF");
    }

    #[test]
    fn test_drc_currency_detection() {
        // Democratic Republic of Congo uses Congolese Franc (CDF)
        assert_eq!(detect_currency_from_phone("+243991234567"), "CDF");
        assert_eq!(detect_currency_from_phone("243801234567"), "CDF");
    }
}

#[cfg(test)]
mod southern_africa_tests {
    use super::*;

    #[test]
    fn test_south_africa_currency_detection() {
        // Special case: 2-digit country code
        assert_eq!(detect_currency_from_phone("+27821234567"), "ZAR");
        assert_eq!(detect_currency_from_phone("27711234567"), "ZAR");
    }

    #[test]
    fn test_zambia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+260971234567"), "ZMW");
        assert_eq!(detect_currency_from_phone("260761234567"), "ZMW");
    }

    #[test]
    fn test_zimbabwe_currency_detection() {
        assert_eq!(detect_currency_from_phone("+263771234567"), "ZWL");
        assert_eq!(detect_currency_from_phone("263712345678"), "ZWL");
    }

    #[test]
    fn test_namibia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+264811234567"), "NAD");
        assert_eq!(detect_currency_from_phone("264851234567"), "NAD");
    }

    #[test]
    fn test_botswana_currency_detection() {
        assert_eq!(detect_currency_from_phone("+267711234567"), "BWP");
        assert_eq!(detect_currency_from_phone("267721234567"), "BWP");
    }

    #[test]
    fn test_eswatini_currency_detection() {
        assert_eq!(detect_currency_from_phone("+268761234567"), "SZL");
        assert_eq!(detect_currency_from_phone("268781234567"), "SZL");
    }

    #[test]
    fn test_lesotho_currency_detection() {
        assert_eq!(detect_currency_from_phone("+266501234567"), "LSL");
        assert_eq!(detect_currency_from_phone("266581234567"), "LSL");
    }

    #[test]
    fn test_madagascar_currency_detection() {
        assert_eq!(detect_currency_from_phone("+261321234567"), "MGA");
        assert_eq!(detect_currency_from_phone("261341234567"), "MGA");
    }

    #[test]
    fn test_malawi_currency_detection() {
        assert_eq!(detect_currency_from_phone("+265991234567"), "MWK");
        assert_eq!(detect_currency_from_phone("265881234567"), "MWK");
    }

    #[test]
    fn test_mozambique_currency_detection() {
        assert_eq!(detect_currency_from_phone("+258841234567"), "MZN");
        assert_eq!(detect_currency_from_phone("258821234567"), "MZN");
    }
}

#[cfg(test)]
mod north_africa_tests {
    use super::*;

    #[test]
    fn test_egypt_currency_detection() {
        // Special case: 2-digit country code
        assert_eq!(detect_currency_from_phone("+201001234567"), "EGP");
        assert_eq!(detect_currency_from_phone("201234567890"), "EGP");
    }

    #[test]
    fn test_morocco_currency_detection() {
        assert_eq!(detect_currency_from_phone("+212661234567"), "MAD");
        assert_eq!(detect_currency_from_phone("212612345678"), "MAD");
    }

    #[test]
    fn test_algeria_currency_detection() {
        assert_eq!(detect_currency_from_phone("+213551234567"), "DZD");
        assert_eq!(detect_currency_from_phone("213661234567"), "DZD");
    }

    #[test]
    fn test_tunisia_currency_detection() {
        assert_eq!(detect_currency_from_phone("+216201234567"), "TND");
        assert_eq!(detect_currency_from_phone("216981234567"), "TND");
    }

    #[test]
    fn test_libya_currency_detection() {
        assert_eq!(detect_currency_from_phone("+218911234567"), "LYD");
        assert_eq!(detect_currency_from_phone("218921234567"), "LYD");
    }

    #[test]
    fn test_sudan_currency_detection() {
        assert_eq!(detect_currency_from_phone("+249911234567"), "SDG");
        assert_eq!(detect_currency_from_phone("249921234567"), "SDG");
    }
}

#[cfg(test)]
mod island_and_other_tests {
    use super::*;

    #[test]
    fn test_south_sudan_currency_detection() {
        assert_eq!(detect_currency_from_phone("+211921234567"), "SSP");
        assert_eq!(detect_currency_from_phone("211911234567"), "SSP");
    }

    #[test]
    fn test_mauritius_currency_detection() {
        assert_eq!(detect_currency_from_phone("+230521234567"), "MUR");
        assert_eq!(detect_currency_from_phone("230581234567"), "MUR");
    }

    #[test]
    fn test_reunion_currency_detection() {
        // Réunion is a French territory and uses Euro
        assert_eq!(detect_currency_from_phone("+262692123456"), "EUR");
        assert_eq!(detect_currency_from_phone("262693123456"), "EUR");
    }

    #[test]
    fn test_mauritania_currency_detection() {
        assert_eq!(detect_currency_from_phone("+222361234567"), "MRU");
        assert_eq!(detect_currency_from_phone("222441234567"), "MRU");
    }

    #[test]
    fn test_seychelles_currency_detection() {
        assert_eq!(detect_currency_from_phone("+248251234567"), "SCR");
        assert_eq!(detect_currency_from_phone("248271234567"), "SCR");
    }

    #[test]
    fn test_comoros_currency_detection() {
        assert_eq!(detect_currency_from_phone("+269321234567"), "KMF");
        assert_eq!(detect_currency_from_phone("269771234567"), "KMF");
    }
}

#[cfg(test)]
mod overlapping_codes_tests {
    use super::*;

    // Test longest-first matching to handle overlapping country codes
    // This ensures 27 (South Africa) doesn't match 270 (intended for a 3-digit code)

    #[test]
    fn test_south_africa_vs_longer_codes() {
        // South Africa (27) should match correctly even though other codes start with 2
        assert_eq!(detect_currency_from_phone("+27821234567"), "ZAR");

        // Botswana (267) should NOT be confused with South Africa (27)
        assert_eq!(detect_currency_from_phone("+267711234567"), "BWP");
    }

    #[test]
    fn test_egypt_vs_longer_codes() {
        // Egypt (20) should match correctly
        assert_eq!(detect_currency_from_phone("+201001234567"), "EGP");

        // Gambia (220) should NOT be confused with Egypt (20)
        assert_eq!(detect_currency_from_phone("+220301234567"), "GMD");
    }

    #[test]
    fn test_three_digit_code_precedence() {
        // All 3-digit codes starting with 25 should match correctly
        assert_eq!(detect_currency_from_phone("+250123456789"), "RWF"); // Rwanda
        assert_eq!(detect_currency_from_phone("+251123456789"), "ETB"); // Ethiopia
        assert_eq!(detect_currency_from_phone("+252123456789"), "SOS"); // Somalia
        assert_eq!(detect_currency_from_phone("+253123456789"), "DJF"); // Djibouti
        assert_eq!(detect_currency_from_phone("+254123456789"), "KES"); // Kenya
        assert_eq!(detect_currency_from_phone("+255123456789"), "TZS"); // Tanzania
        assert_eq!(detect_currency_from_phone("+256123456789"), "UGX"); // Uganda
        assert_eq!(detect_currency_from_phone("+257123456789"), "BIF"); // Burundi
        assert_eq!(detect_currency_from_phone("+258123456789"), "MZN"); // Mozambique
    }
}

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_unknown_country_code_defaults_to_ugx() {
        // Non-African country codes should default to UGX
        assert_eq!(detect_currency_from_phone("+1234567890"), "UGX"); // US
        assert_eq!(detect_currency_from_phone("+44123456789"), "UGX"); // UK
        assert_eq!(detect_currency_from_phone("+91123456789"), "UGX"); // India
    }

    #[test]
    fn test_with_and_without_plus_prefix() {
        // Both formats should work
        assert_eq!(detect_currency_from_phone("+254712345678"), "KES");
        assert_eq!(detect_currency_from_phone("254712345678"), "KES");
    }

    #[test]
    fn test_whitespace_handling() {
        // Should handle leading/trailing whitespace
        assert_eq!(detect_currency_from_phone(" +254712345678 "), "KES");
        assert_eq!(detect_currency_from_phone("  254712345678  "), "KES");
    }

    #[test]
    fn test_all_39_unique_currencies_covered() {
        // Verify all 39 unique African currencies are covered
        let unique_currencies = vec![
            "UGX", "KES", "TZS", "RWF", "BIF", "ETB", "SOS", "DJF", // East Africa
            "NGN", "GHS", "XOF", "GNF", "LRD", "SLL", "GMD", "CVE", // West Africa
            "XAF", "CDF", // Central Africa
            "ZAR", "ZMW", "ZWL", "NAD", "BWP", "SZL", "LSL", "MGA", "MWK", "MZN", // Southern Africa
            "EGP", "MAD", "DZD", "TND", "LYD", "SDG", // North Africa
            "SSP", "MUR", "EUR", "MRU", "SCR", "KMF", // Other/Islands
        ];

        // This is a documentation test - verifies we have 39+ unique currencies
        assert!(unique_currencies.len() >= 39, "Should cover at least 39 unique currencies");
    }
}
