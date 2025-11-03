/**
 * Pan-African Phone Number Utilities
 * Supports all African countries with proper country code detection
 */

import { AfricanCurrency } from '../types/currency';

export interface AfricanCountry {
  name: string;
  code: string; // ISO 3166-1 alpha-2
  dialCode: string; // Country calling code
  currency: AfricanCurrency;
  flag: string;
}

// Africa's Talking supported countries (with full SMS/USSD support)
export const AFRICAS_TALKING_COUNTRIES: AfricanCountry[] = [
  { name: 'Kenya', code: 'KE', dialCode: '254', currency: 'KES', flag: '🇰🇪' },
  { name: 'Uganda', code: 'UG', dialCode: '256', currency: 'UGX', flag: '🇺🇬' },
  { name: 'Nigeria', code: 'NG', dialCode: '234', currency: 'NGN', flag: '🇳🇬' },
  { name: 'Tanzania', code: 'TZ', dialCode: '255', currency: 'TZS', flag: '🇹🇿' },
  { name: 'Rwanda', code: 'RW', dialCode: '250', currency: 'RWF', flag: '🇷🇼' },
  { name: 'Malawi', code: 'MW', dialCode: '265', currency: 'MWK', flag: '🇲🇼' },
  { name: 'South Africa', code: 'ZA', dialCode: '27', currency: 'ZAR', flag: '🇿🇦' },
  { name: 'Zambia', code: 'ZM', dialCode: '260', currency: 'ZMW', flag: '🇿🇲' },
];

// All African countries (for reference and future expansion)
export const ALL_AFRICAN_COUNTRIES: AfricanCountry[] = [
  ...AFRICAS_TALKING_COUNTRIES,
  { name: 'Ghana', code: 'GH', dialCode: '233', currency: 'GHS', flag: '🇬🇭' },
  { name: 'Egypt', code: 'EG', dialCode: '20', currency: 'EGP', flag: '🇪🇬' },
  { name: 'Morocco', code: 'MA', dialCode: '212', currency: 'MAD', flag: '🇲🇦' },
  { name: 'Algeria', code: 'DZ', dialCode: '213', currency: 'DZD', flag: '🇩🇿' },
  { name: 'Tunisia', code: 'TN', dialCode: '216', currency: 'TND', flag: '🇹🇳' },
  { name: 'Ethiopia', code: 'ET', dialCode: '251', currency: 'ETB', flag: '🇪🇹' },
  { name: 'Senegal', code: 'SN', dialCode: '221', currency: 'XOF', flag: '🇸🇳' },
  { name: 'Ivory Coast', code: 'CI', dialCode: '225', currency: 'XOF', flag: '🇨🇮' },
  { name: 'Cameroon', code: 'CM', dialCode: '237', currency: 'XAF', flag: '🇨🇲' },
  { name: 'Angola', code: 'AO', dialCode: '244', currency: 'AOA', flag: '🇦🇴' },
  { name: 'Mozambique', code: 'MZ', dialCode: '258', currency: 'MZN', flag: '🇲🇿' },
  { name: 'Zimbabwe', code: 'ZW', dialCode: '263', currency: 'ZWL', flag: '🇿🇼' },
  { name: 'Botswana', code: 'BW', dialCode: '267', currency: 'BWP', flag: '🇧🇼' },
  { name: 'Namibia', code: 'NA', dialCode: '264', currency: 'NAD', flag: '🇳🇦' },
  { name: 'Libya', code: 'LY', dialCode: '218', currency: 'LYD', flag: '🇱🇾' },
  { name: 'Sudan', code: 'SD', dialCode: '249', currency: 'SDG', flag: '🇸🇩' },
  { name: 'Somalia', code: 'SO', dialCode: '252', currency: 'SOS', flag: '🇸🇴' },
  { name: 'Burundi', code: 'BI', dialCode: '257', currency: 'BIF', flag: '🇧🇮' },
  { name: 'Benin', code: 'BJ', dialCode: '229', currency: 'XOF', flag: '🇧🇯' },
  { name: 'Burkina Faso', code: 'BF', dialCode: '226', currency: 'XOF', flag: '🇧🇫' },
  { name: 'Chad', code: 'TD', dialCode: '235', currency: 'XAF', flag: '🇹🇩' },
  { name: 'Congo', code: 'CG', dialCode: '242', currency: 'XAF', flag: '🇨🇬' },
  { name: 'DR Congo', code: 'CD', dialCode: '243', currency: 'CDF', flag: '🇨🇩' },
  { name: 'Gabon', code: 'GA', dialCode: '241', currency: 'XAF', flag: '🇬🇦' },
  { name: 'Guinea', code: 'GN', dialCode: '224', currency: 'GNF', flag: '🇬🇳' },
  { name: 'Lesotho', code: 'LS', dialCode: '266', currency: 'LSL', flag: '🇱🇸' },
  { name: 'Liberia', code: 'LR', dialCode: '231', currency: 'LRD', flag: '🇱🇷' },
  { name: 'Madagascar', code: 'MG', dialCode: '261', currency: 'MGA', flag: '🇲🇬' },
  { name: 'Mali', code: 'ML', dialCode: '223', currency: 'XOF', flag: '🇲🇱' },
  { name: 'Mauritania', code: 'MR', dialCode: '222', currency: 'MRU', flag: '🇲🇷' },
  { name: 'Mauritius', code: 'MU', dialCode: '230', currency: 'MUR', flag: '🇲🇺' },
  { name: 'Niger', code: 'NE', dialCode: '227', currency: 'XOF', flag: '🇳🇪' },
  { name: 'Sierra Leone', code: 'SL', dialCode: '232', currency: 'SLL', flag: '🇸🇱' },
  { name: 'Togo', code: 'TG', dialCode: '228', currency: 'XOF', flag: '🇹🇬' },
];

/**
 * Detect country from phone number
 */
export function detectCountryFromPhone(phoneNumber: string): AfricanCountry | null {
  const cleaned = phoneNumber.replace(/\D/g, '');
  
  // Try to match with all African countries (longest dial codes first)
  const sortedCountries = [...ALL_AFRICAN_COUNTRIES].sort((a, b) => 
    b.dialCode.length - a.dialCode.length
  );
  
  for (const country of sortedCountries) {
    if (cleaned.startsWith(country.dialCode)) {
      return country;
    }
    // Also check with + prefix
    if (cleaned.startsWith(`+${country.dialCode}`)) {
      return country;
    }
  }
  
  return null;
}

/**
 * Format phone number to E.164 format
 * Auto-detects country or uses provided country code
 */
export function formatPhoneNumber(
  phoneNumber: string,
  defaultCountryCode?: string
): string {
  // Remove all non-numeric characters except +
  let cleaned = phoneNumber.replace(/[^\d+]/g, '');
  
  // If already in E.164 format, validate and return
  if (cleaned.startsWith('+')) {
    const country = detectCountryFromPhone(cleaned);
    if (country) {
      return cleaned;
    }
  }
  
  // Remove leading + if present
  cleaned = cleaned.replace(/^\+/, '');
  
  // Try to detect country from the number
  const detectedCountry = detectCountryFromPhone(cleaned);
  if (detectedCountry) {
    // Already has country code
    if (cleaned.startsWith(detectedCountry.dialCode)) {
      return `+${cleaned}`;
    }
  }
  
  // If starts with 0, it's a local number - need country code
  if (cleaned.startsWith('0')) {
    const localNumber = cleaned.substring(1);
    
    // Use provided default or try to detect
    if (defaultCountryCode) {
      return `+${defaultCountryCode}${localNumber}`;
    }
    
    // Default to Kenya if no country code provided (Africa's Talking default)
    return `+254${localNumber}`;
  }
  
  // If no country code detected, add default
  if (defaultCountryCode) {
    return `+${defaultCountryCode}${cleaned}`;
  }
  
  // Last resort: assume Kenya
  return `+254${cleaned}`;
}

/**
 * Validate phone number format
 */
export function isValidPhoneNumber(phoneNumber: string): boolean {
  // E.164 format: +[country code][number]
  const e164Regex = /^\+[1-9]\d{1,14}$/;
  
  if (!e164Regex.test(phoneNumber)) {
    return false;
  }
  
  // Check if it's a valid African number
  const country = detectCountryFromPhone(phoneNumber);
  return country !== null;
}

/**
 * Get currency from phone number
 */
export function getCurrencyFromPhone(phoneNumber: string): AfricanCurrency | null {
  const country = detectCountryFromPhone(phoneNumber);
  return country ? country.currency : null;
}

/**
 * Check if country has full Africa's Talking support (SMS + USSD)
 */
export function hasFullSMSSupport(phoneNumber: string): boolean {
  const country = detectCountryFromPhone(phoneNumber);
  if (!country) return false;
  
  return AFRICAS_TALKING_COUNTRIES.some(c => c.dialCode === country.dialCode);
}

/**
 * Get country info from phone number
 */
export function getCountryInfo(phoneNumber: string): AfricanCountry | null {
  return detectCountryFromPhone(phoneNumber);
}

/**
 * Format phone number for display
 */
export function formatPhoneForDisplay(phoneNumber: string): string {
  const country = detectCountryFromPhone(phoneNumber);
  if (!country) return phoneNumber;
  
  const cleaned = phoneNumber.replace(/\D/g, '');
  const withoutCountryCode = cleaned.substring(country.dialCode.length);
  
  // Format based on country (basic formatting)
  if (withoutCountryCode.length >= 9) {
    return `${country.flag} +${country.dialCode} ${withoutCountryCode.substring(0, 3)} ${withoutCountryCode.substring(3, 6)} ${withoutCountryCode.substring(6)}`;
  }
  
  return `${country.flag} +${country.dialCode} ${withoutCountryCode}`;
}

/**
 * Get all supported countries for dropdown/selection
 */
export function getSupportedCountries(): AfricanCountry[] {
  return AFRICAS_TALKING_COUNTRIES;
}

/**
 * Get all African countries
 */
export function getAllAfricanCountries(): AfricanCountry[] {
  return ALL_AFRICAN_COUNTRIES;
}
