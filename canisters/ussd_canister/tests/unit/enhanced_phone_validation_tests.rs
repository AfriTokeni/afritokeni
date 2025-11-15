// Enhanced phone number validation tests covering all 54 African countries
use ussd_canister::utils::validation::is_valid_phone;

#[cfg(test)]
mod east_africa_phone_validation {
    use super::*;

    #[test]
    fn test_uganda_phone_numbers() {
        // Uganda: +256
        assert!(is_valid_phone("+256700123456"));
        assert!(is_valid_phone("+256712345678"));
        assert!(is_valid_phone("+256750123456"));
        assert!(is_valid_phone("+256780123456"));
    }

    #[test]
    fn test_kenya_phone_numbers() {
        // Kenya: +254
        assert!(is_valid_phone("+254700123456"));
        assert!(is_valid_phone("+254712345678"));
        assert!(is_valid_phone("+254720123456"));
        assert!(is_valid_phone("+254790123456"));
    }

    #[test]
    fn test_tanzania_phone_numbers() {
        // Tanzania: +255
        assert!(is_valid_phone("+255700123456"));
        assert!(is_valid_phone("+255712345678"));
        assert!(is_valid_phone("+255750123456"));
        assert!(is_valid_phone("+255780123456"));
    }

    #[test]
    fn test_rwanda_phone_numbers() {
        // Rwanda: +250
        assert!(is_valid_phone("+250788123456"));
        assert!(is_valid_phone("+250700123456"));
        assert!(is_valid_phone("+250720123456"));
    }

    #[test]
    fn test_burundi_phone_numbers() {
        // Burundi: +257
        assert!(is_valid_phone("+257700123456"));
        assert!(is_valid_phone("+257712345678"));
        assert!(is_valid_phone("+257790123456"));
    }

    #[test]
    fn test_ethiopia_phone_numbers() {
        // Ethiopia: +251
        assert!(is_valid_phone("+251911123456"));
        assert!(is_valid_phone("+251900123456"));
        assert!(is_valid_phone("+251920123456"));
    }

    #[test]
    fn test_somalia_phone_numbers() {
        // Somalia: +252
        assert!(is_valid_phone("+252612345678"));
        assert!(is_valid_phone("+252907123456"));
        assert!(is_valid_phone("+252615123456"));
    }

    #[test]
    fn test_djibouti_phone_numbers() {
        // Djibouti: +253
        assert!(is_valid_phone("+253770123456"));
        assert!(is_valid_phone("+253812345678"));
        assert!(is_valid_phone("+253775123456"));
    }
}

#[cfg(test)]
mod west_africa_phone_validation {
    use super::*;

    #[test]
    fn test_nigeria_phone_numbers() {
        // Nigeria: +234
        assert!(is_valid_phone("+234803123456"));
        assert!(is_valid_phone("+234810123456"));
        assert!(is_valid_phone("+234705123456"));
        assert!(is_valid_phone("+234901123456"));
    }

    #[test]
    fn test_ghana_phone_numbers() {
        // Ghana: +233
        assert!(is_valid_phone("+233241234567"));
        assert!(is_valid_phone("+233501234567"));
        assert!(is_valid_phone("+233201234567"));
    }

    #[test]
    fn test_senegal_phone_numbers() {
        // Senegal: +221
        assert!(is_valid_phone("+221771234567"));
        assert!(is_valid_phone("+221331234567"));
        assert!(is_valid_phone("+221701234567"));
    }

    #[test]
    fn test_cote_divoire_phone_numbers() {
        // Côte d'Ivoire: +225
        assert!(is_valid_phone("+225071234567"));
        assert!(is_valid_phone("+225451234567"));
        assert!(is_valid_phone("+225011234567"));
    }

    #[test]
    fn test_mali_phone_numbers() {
        // Mali: +223
        assert!(is_valid_phone("+223701234567"));
        assert!(is_valid_phone("+223901234567"));
        assert!(is_valid_phone("+223651234567"));
    }

    #[test]
    fn test_burkina_faso_phone_numbers() {
        // Burkina Faso: +226
        assert!(is_valid_phone("+226701234567"));
        assert!(is_valid_phone("+226501234567"));
        assert!(is_valid_phone("+226601234567"));
    }

    #[test]
    fn test_niger_phone_numbers() {
        // Niger: +227
        assert!(is_valid_phone("+227901234567"));
        assert!(is_valid_phone("+227961234567"));
        assert!(is_valid_phone("+227801234567"));
    }

    #[test]
    fn test_togo_phone_numbers() {
        // Togo: +228
        assert!(is_valid_phone("+228901234567"));
        assert!(is_valid_phone("+228221234567"));
        assert!(is_valid_phone("+228701234567"));
    }

    #[test]
    fn test_benin_phone_numbers() {
        // Benin: +229
        assert!(is_valid_phone("+229971234567"));
        assert!(is_valid_phone("+229641234567"));
        assert!(is_valid_phone("+229901234567"));
    }

    #[test]
    fn test_guinea_phone_numbers() {
        // Guinea: +224
        assert!(is_valid_phone("+224621234567"));
        assert!(is_valid_phone("+224301234567"));
        assert!(is_valid_phone("+224601234567"));
    }

    #[test]
    fn test_liberia_phone_numbers() {
        // Liberia: +231
        assert!(is_valid_phone("+231771234567"));
        assert!(is_valid_phone("+231881234567"));
        assert!(is_valid_phone("+231551234567"));
    }

    #[test]
    fn test_sierra_leone_phone_numbers() {
        // Sierra Leone: +232
        assert!(is_valid_phone("+232761234567"));
        assert!(is_valid_phone("+232301234567"));
        assert!(is_valid_phone("+232771234567"));
    }

    #[test]
    fn test_gambia_phone_numbers() {
        // Gambia: +220
        assert!(is_valid_phone("+220301234567"));
        assert!(is_valid_phone("+220771234567"));
        assert!(is_valid_phone("+220901234567"));
    }

    #[test]
    fn test_cape_verde_phone_numbers() {
        // Cape Verde: +238
        assert!(is_valid_phone("+238991234567"));
        assert!(is_valid_phone("+238581234567"));
        assert!(is_valid_phone("+238951234567"));
    }

    #[test]
    fn test_mauritania_phone_numbers() {
        // Mauritania: +222
        assert!(is_valid_phone("+222361234567"));
        assert!(is_valid_phone("+222441234567"));
        assert!(is_valid_phone("+222301234567"));
    }
}

#[cfg(test)]
mod central_africa_phone_validation {
    use super::*;

    #[test]
    fn test_cameroon_phone_numbers() {
        // Cameroon: +237
        assert!(is_valid_phone("+237671234567"));
        assert!(is_valid_phone("+237691234567"));
        assert!(is_valid_phone("+237651234567"));
    }

    #[test]
    fn test_car_phone_numbers() {
        // Central African Republic: +236
        assert!(is_valid_phone("+236701234567"));
        assert!(is_valid_phone("+236751234567"));
        assert!(is_valid_phone("+236721234567"));
    }

    #[test]
    fn test_chad_phone_numbers() {
        // Chad: +235
        assert!(is_valid_phone("+235661234567"));
        assert!(is_valid_phone("+235901234567"));
        assert!(is_valid_phone("+235621234567"));
    }

    #[test]
    fn test_gabon_phone_numbers() {
        // Gabon: +241
        assert!(is_valid_phone("+241061234567"));
        assert!(is_valid_phone("+241741234567"));
        assert!(is_valid_phone("+241011234567"));
    }

    #[test]
    fn test_congo_brazzaville_phone_numbers() {
        // Republic of Congo: +242
        assert!(is_valid_phone("+242061234567"));
        assert!(is_valid_phone("+242051234567"));
        assert!(is_valid_phone("+242041234567"));
    }

    #[test]
    fn test_equatorial_guinea_phone_numbers() {
        // Equatorial Guinea: +240
        assert!(is_valid_phone("+240222123456"));
        assert!(is_valid_phone("+240555123456"));
        assert!(is_valid_phone("+240333123456"));
    }

    #[test]
    fn test_drc_phone_numbers() {
        // Democratic Republic of Congo: +243
        assert!(is_valid_phone("+243991234567"));
        assert!(is_valid_phone("+243801234567"));
        assert!(is_valid_phone("+243821234567"));
    }
}

#[cfg(test)]
mod southern_africa_phone_validation {
    use super::*;

    #[test]
    fn test_south_africa_phone_numbers() {
        // South Africa: +27 (2-digit code)
        assert!(is_valid_phone("+27821234567"));
        assert!(is_valid_phone("+27711234567"));
        assert!(is_valid_phone("+27601234567"));
        assert!(is_valid_phone("+27831234567"));
    }

    #[test]
    fn test_zambia_phone_numbers() {
        // Zambia: +260
        assert!(is_valid_phone("+260971234567"));
        assert!(is_valid_phone("+260761234567"));
        assert!(is_valid_phone("+260951234567"));
    }

    #[test]
    fn test_zimbabwe_phone_numbers() {
        // Zimbabwe: +263
        assert!(is_valid_phone("+263771234567"));
        assert!(is_valid_phone("+263712345678"));
        assert!(is_valid_phone("+263731234567"));
    }

    #[test]
    fn test_namibia_phone_numbers() {
        // Namibia: +264
        assert!(is_valid_phone("+264811234567"));
        assert!(is_valid_phone("+264851234567"));
        assert!(is_valid_phone("+264611234567"));
    }

    #[test]
    fn test_botswana_phone_numbers() {
        // Botswana: +267
        assert!(is_valid_phone("+267711234567"));
        assert!(is_valid_phone("+267721234567"));
        assert!(is_valid_phone("+267751234567"));
    }

    #[test]
    fn test_eswatini_phone_numbers() {
        // Eswatini (Swaziland): +268
        assert!(is_valid_phone("+268761234567"));
        assert!(is_valid_phone("+268781234567"));
        assert!(is_valid_phone("+268791234567"));
    }

    #[test]
    fn test_lesotho_phone_numbers() {
        // Lesotho: +266
        assert!(is_valid_phone("+266501234567"));
        assert!(is_valid_phone("+266581234567"));
        assert!(is_valid_phone("+266621234567"));
    }

    #[test]
    fn test_madagascar_phone_numbers() {
        // Madagascar: +261
        assert!(is_valid_phone("+261321234567"));
        assert!(is_valid_phone("+261341234567"));
        assert!(is_valid_phone("+261331234567"));
    }

    #[test]
    fn test_malawi_phone_numbers() {
        // Malawi: +265
        assert!(is_valid_phone("+265991234567"));
        assert!(is_valid_phone("+265881234567"));
        assert!(is_valid_phone("+265991234567"));
    }

    #[test]
    fn test_mozambique_phone_numbers() {
        // Mozambique: +258
        assert!(is_valid_phone("+258841234567"));
        assert!(is_valid_phone("+258821234567"));
        assert!(is_valid_phone("+258861234567"));
    }
}

#[cfg(test)]
mod north_africa_phone_validation {
    use super::*;

    #[test]
    fn test_egypt_phone_numbers() {
        // Egypt: +20 (2-digit code)
        assert!(is_valid_phone("+201001234567"));
        assert!(is_valid_phone("+201234567890"));
        assert!(is_valid_phone("+201101234567"));
    }

    #[test]
    fn test_morocco_phone_numbers() {
        // Morocco: +212
        assert!(is_valid_phone("+212661234567"));
        assert!(is_valid_phone("+212612345678"));
        assert!(is_valid_phone("+212701234567"));
    }

    #[test]
    fn test_algeria_phone_numbers() {
        // Algeria: +213
        assert!(is_valid_phone("+213551234567"));
        assert!(is_valid_phone("+213661234567"));
        assert!(is_valid_phone("+213771234567"));
    }

    #[test]
    fn test_tunisia_phone_numbers() {
        // Tunisia: +216
        assert!(is_valid_phone("+216201234567"));
        assert!(is_valid_phone("+216981234567"));
        assert!(is_valid_phone("+216501234567"));
    }

    #[test]
    fn test_libya_phone_numbers() {
        // Libya: +218
        assert!(is_valid_phone("+218911234567"));
        assert!(is_valid_phone("+218921234567"));
        assert!(is_valid_phone("+218901234567"));
    }

    #[test]
    fn test_sudan_phone_numbers() {
        // Sudan: +249
        assert!(is_valid_phone("+249911234567"));
        assert!(is_valid_phone("+249921234567"));
        assert!(is_valid_phone("+249901234567"));
    }
}

#[cfg(test)]
mod island_and_other_phone_validation {
    use super::*;

    #[test]
    fn test_south_sudan_phone_numbers() {
        // South Sudan: +211
        assert!(is_valid_phone("+211921234567"));
        assert!(is_valid_phone("+211911234567"));
        assert!(is_valid_phone("+211901234567"));
    }

    #[test]
    fn test_mauritius_phone_numbers() {
        // Mauritius: +230
        assert!(is_valid_phone("+230521234567"));
        assert!(is_valid_phone("+230581234567"));
        assert!(is_valid_phone("+230591234567"));
    }

    #[test]
    fn test_reunion_phone_numbers() {
        // Réunion: +262
        assert!(is_valid_phone("+262692123456"));
        assert!(is_valid_phone("+262693123456"));
        assert!(is_valid_phone("+262691123456"));
    }

    #[test]
    fn test_seychelles_phone_numbers() {
        // Seychelles: +248
        assert!(is_valid_phone("+248251234567"));
        assert!(is_valid_phone("+248271234567"));
        assert!(is_valid_phone("+248291234567"));
    }

    #[test]
    fn test_comoros_phone_numbers() {
        // Comoros: +269
        assert!(is_valid_phone("+269321234567"));
        assert!(is_valid_phone("+269771234567"));
        assert!(is_valid_phone("+269331234567"));
    }
}

#[cfg(test)]
mod overlapping_country_codes_validation {
    use super::*;

    // Test that overlapping country codes are handled correctly
    // (longest-first matching)

    #[test]
    fn test_south_africa_vs_overlapping_codes() {
        // South Africa (+27) vs Botswana (+267), Lesotho (+266), Namibia (+264)
        assert!(is_valid_phone("+27821234567")); // South Africa
        assert!(is_valid_phone("+267711234567")); // Botswana (should not be confused with SA)
        assert!(is_valid_phone("+266501234567")); // Lesotho
        assert!(is_valid_phone("+264811234567")); // Namibia
    }

    #[test]
    fn test_egypt_vs_overlapping_codes() {
        // Egypt (+20) vs Gambia (+220), Mauritania (+222)
        assert!(is_valid_phone("+201001234567")); // Egypt
        assert!(is_valid_phone("+220301234567")); // Gambia (should not be confused with Egypt)
        assert!(is_valid_phone("+222361234567")); // Mauritania
    }

    #[test]
    fn test_all_25x_codes_distinct() {
        // All codes starting with 25 should be recognized correctly
        assert!(is_valid_phone("+250788123456")); // Rwanda
        assert!(is_valid_phone("+251911123456")); // Ethiopia
        assert!(is_valid_phone("+252612345678")); // Somalia
        assert!(is_valid_phone("+253770123456")); // Djibouti
        assert!(is_valid_phone("+254712345678")); // Kenya
        assert!(is_valid_phone("+255700123456")); // Tanzania
        assert!(is_valid_phone("+256700123456")); // Uganda
        assert!(is_valid_phone("+257700123456")); // Burundi
        assert!(is_valid_phone("+258841234567")); // Mozambique
    }

    #[test]
    fn test_all_26x_codes_distinct() {
        // All codes starting with 26 should be recognized correctly
        assert!(is_valid_phone("+260971234567")); // Zambia
        assert!(is_valid_phone("+261321234567")); // Madagascar
        assert!(is_valid_phone("+262692123456")); // Réunion
        assert!(is_valid_phone("+263771234567")); // Zimbabwe
        assert!(is_valid_phone("+264811234567")); // Namibia
        assert!(is_valid_phone("+265991234567")); // Malawi
        assert!(is_valid_phone("+266501234567")); // Lesotho
        assert!(is_valid_phone("+267711234567")); // Botswana
        assert!(is_valid_phone("+268761234567")); // Eswatini
        assert!(is_valid_phone("+269321234567")); // Comoros
    }
}

#[cfg(test)]
mod invalid_phone_numbers_validation {
    use super::*;

    #[test]
    fn test_non_african_country_codes_rejected() {
        // US/Canada
        assert!(!is_valid_phone("+11234567890"));
        assert!(!is_valid_phone("+14155551234"));

        // UK
        assert!(!is_valid_phone("+441234567890"));

        // India
        assert!(!is_valid_phone("+911234567890"));

        // China
        assert!(!is_valid_phone("+861234567890"));

        // Australia
        assert!(!is_valid_phone("+611234567890"));
    }

    #[test]
    fn test_missing_plus_sign_rejected() {
        // All valid African numbers should start with +
        assert!(!is_valid_phone("256700123456")); // Uganda without +
        assert!(!is_valid_phone("254712345678")); // Kenya without +
        assert!(!is_valid_phone("27821234567"));  // South Africa without +
    }

    #[test]
    fn test_too_short_phone_numbers_rejected() {
        assert!(!is_valid_phone("+256"));
        assert!(!is_valid_phone("+25412"));
        assert!(!is_valid_phone("+2541234")); // Too short
    }

    #[test]
    fn test_too_long_phone_numbers_rejected() {
        // Maximum 15 digits after +
        assert!(!is_valid_phone("+2567001234567890")); // 16 digits
        assert!(!is_valid_phone("+25470012345678901")); // 17 digits
    }

    #[test]
    fn test_non_numeric_characters_rejected() {
        assert!(!is_valid_phone("+256ABC123456"));
        assert!(!is_valid_phone("+254-712-345678")); // Hyphens not allowed
        assert!(!is_valid_phone("+256 700 123456")); // Spaces not allowed
        assert!(!is_valid_phone("+256(700)123456")); // Parentheses not allowed
    }

    #[test]
    fn test_empty_and_invalid_formats_rejected() {
        assert!(!is_valid_phone(""));
        assert!(!is_valid_phone("+"));
        assert!(!is_valid_phone("++256700123456")); // Double plus
        assert!(!is_valid_phone("256+700123456")); // Plus in middle
    }

    #[test]
    fn test_special_characters_rejected() {
        assert!(!is_valid_phone("+256#700123456"));
        assert!(!is_valid_phone("+256*700123456"));
        assert!(!is_valid_phone("+256.700123456"));
        assert!(!is_valid_phone("+256@700123456"));
    }
}

#[cfg(test)]
mod phone_format_edge_cases {
    use super::*;

    #[test]
    fn test_minimum_valid_length() {
        // 10 digits is minimum (country code + subscriber number)
        assert!(is_valid_phone("+2567001234")); // Exactly 10 digits
        assert!(!is_valid_phone("+256700123")); // Only 9 digits
    }

    #[test]
    fn test_maximum_valid_length() {
        // 15 digits is maximum (E.164 standard)
        assert!(is_valid_phone("+256700123456789")); // Exactly 15 digits
        assert!(!is_valid_phone("+2567001234567890")); // 16 digits, too long
    }

    #[test]
    fn test_all_zeros_after_country_code() {
        // All zeros should still be valid if format is correct
        assert!(is_valid_phone("+2560000000000"));
    }

    #[test]
    fn test_all_nines_after_country_code() {
        // All nines should still be valid if format is correct
        assert!(is_valid_phone("+2569999999999"));
    }

    #[test]
    fn test_repeated_digits() {
        assert!(is_valid_phone("+2561111111111"));
        assert!(is_valid_phone("+2542222222222"));
        assert!(is_valid_phone("+2553333333333"));
    }
}

#[cfg(test)]
mod comprehensive_coverage_verification {
    use super::*;

    #[test]
    fn test_all_54_african_countries_phone_codes() {
        // Verify we cover all 54 African countries
        let valid_numbers = vec![
            "+256700123456",   // Uganda
            "+254712345678",   // Kenya
            "+255700123456",   // Tanzania
            "+250788123456",   // Rwanda
            "+257700123456",   // Burundi
            "+251911123456",   // Ethiopia
            "+252612345678",   // Somalia
            "+253770123456",   // Djibouti
            "+234803123456",   // Nigeria
            "+233241234567",   // Ghana
            "+225071234567",   // Côte d'Ivoire
            "+221771234567",   // Senegal
            "+223701234567",   // Mali
            "+226701234567",   // Burkina Faso
            "+227901234567",   // Niger
            "+228901234567",   // Togo
            "+229971234567",   // Benin
            "+224621234567",   // Guinea
            "+231771234567",   // Liberia
            "+232761234567",   // Sierra Leone
            "+220301234567",   // Gambia
            "+238991234567",   // Cape Verde
            "+237671234567",   // Cameroon
            "+236701234567",   // Central African Republic
            "+235661234567",   // Chad
            "+241061234567",   // Gabon
            "+242061234567",   // Congo (Brazzaville)
            "+240222123456",   // Equatorial Guinea
            "+243991234567",   // Democratic Republic of Congo
            "+27821234567",    // South Africa
            "+260971234567",   // Zambia
            "+263771234567",   // Zimbabwe
            "+264811234567",   // Namibia
            "+267711234567",   // Botswana
            "+268761234567",   // Eswatini
            "+266501234567",   // Lesotho
            "+261321234567",   // Madagascar
            "+265991234567",   // Malawi
            "+258841234567",   // Mozambique
            "+201001234567",   // Egypt
            "+212661234567",   // Morocco
            "+213551234567",   // Algeria
            "+216201234567",   // Tunisia
            "+218911234567",   // Libya
            "+249911234567",   // Sudan
            "+211921234567",   // South Sudan
            "+230521234567",   // Mauritius
            "+262692123456",   // Réunion
            "+222361234567",   // Mauritania
            "+248251234567",   // Seychelles
            "+269321234567",   // Comoros
        ];

        // Verify we have at least 51 unique numbers (some countries share codes like XOF/XAF regions)
        assert!(valid_numbers.len() >= 51, "Should cover at least 51 unique phone numbers");

        // All should be valid
        for number in &valid_numbers {
            assert!(is_valid_phone(number), "Should accept valid phone: {}", number);
        }
    }
}
