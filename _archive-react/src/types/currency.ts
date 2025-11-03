// African currency support for AfriTokeni
export type AfricanCurrency = 
  // East Africa
  | 'UGX' // Uganda Shilling
  | 'KES' // Kenyan Shilling
  | 'TZS' // Tanzanian Shilling
  | 'RWF' // Rwandan Franc
  | 'BIF' // Burundian Franc
  | 'ETB' // Ethiopian Birr
  | 'SOS' // Somali Shilling
  
  // West Africa
  | 'NGN' // Nigerian Naira
  | 'GHS' // Ghanaian Cedi
  | 'XOF' // West African CFA Franc (Benin, Burkina Faso, Côte d'Ivoire, Guinea-Bissau, Mali, Niger, Senegal, Togo)
  | 'SLL' // Sierra Leonean Leone
  | 'LRD' // Liberian Dollar
  | 'GMD' // Gambian Dalasi
  | 'GNF' // Guinean Franc
  
  // Central Africa
  | 'XAF' // Central African CFA Franc (Cameroon, Central African Republic, Chad, Republic of the Congo, Equatorial Guinea, Gabon)
  | 'CDF' // Congolese Franc (DRC)
  | 'AOA' // Angolan Kwanza
  
  // Southern Africa
  | 'ZAR' // South African Rand
  | 'BWP' // Botswana Pula
  | 'SZL' // Swazi Lilangeni
  | 'LSL' // Lesotho Loti
  | 'NAD' // Namibian Dollar
  | 'ZMW' // Zambian Kwacha
  | 'ZWL' // Zimbabwean Dollar
  | 'MWK' // Malawian Kwacha
  | 'MZN' // Mozambican Metical
  
  // North Africa
  | 'EGP' // Egyptian Pound
  | 'MAD' // Moroccan Dirham
  | 'TND' // Tunisian Dinar
  | 'DZD' // Algerian Dinar
  | 'LYD' // Libyan Dinar
  | 'SDG' // Sudanese Pound
  | 'MRU' // Mauritanian Ouguiya
  
  // Other
  | 'MUR' // Mauritian Rupee
  | 'SCR' // Seychellois Rupee
  | 'CVE' // Cape Verdean Escudo
  | 'STN' // São Tomé and Príncipe Dobra
  | 'KMF' // Comorian Franc
  | 'DJF' // Djiboutian Franc
  | 'ERN' // Eritrean Nakfa
  | 'MGA' // Malagasy Ariary
  | 'BTC'; // Bitcoin

export interface CurrencyInfo {
  code: AfricanCurrency;
  name: string;
  symbol: string;
  country: string;
  region: 'East Africa' | 'West Africa' | 'Central Africa' | 'Southern Africa' | 'North Africa' | 'Island Nations' | 'Digital';
  decimals: number;
  isActive: boolean; // For enabling/disabling currencies
}

export const AFRICAN_CURRENCIES: Record<AfricanCurrency, CurrencyInfo> = {
  // East Africa
  UGX: { code: 'UGX', name: 'Uganda Shilling', symbol: 'USh', country: 'Uganda', region: 'East Africa', decimals: 0, isActive: true },
  KES: { code: 'KES', name: 'Kenyan Shilling', symbol: 'KSh', country: 'Kenya', region: 'East Africa', decimals: 2, isActive: true },
  TZS: { code: 'TZS', name: 'Tanzanian Shilling', symbol: 'TSh', country: 'Tanzania', region: 'East Africa', decimals: 2, isActive: true },
  RWF: { code: 'RWF', name: 'Rwandan Franc', symbol: 'RF', country: 'Rwanda', region: 'East Africa', decimals: 0, isActive: true },
  BIF: { code: 'BIF', name: 'Burundian Franc', symbol: 'FBu', country: 'Burundi', region: 'East Africa', decimals: 0, isActive: true },
  ETB: { code: 'ETB', name: 'Ethiopian Birr', symbol: 'Br', country: 'Ethiopia', region: 'East Africa', decimals: 2, isActive: true },
  SOS: { code: 'SOS', name: 'Somali Shilling', symbol: 'Sh', country: 'Somalia', region: 'East Africa', decimals: 2, isActive: false },
  
  // West Africa
  NGN: { code: 'NGN', name: 'Nigerian Naira', symbol: '₦', country: 'Nigeria', region: 'West Africa', decimals: 2, isActive: true },
  GHS: { code: 'GHS', name: 'Ghanaian Cedi', symbol: '₵', country: 'Ghana', region: 'West Africa', decimals: 2, isActive: true },
  XOF: { code: 'XOF', name: 'West African CFA Franc', symbol: 'CFA', country: 'WAEMU', region: 'West Africa', decimals: 0, isActive: true },
  SLL: { code: 'SLL', name: 'Sierra Leonean Leone', symbol: 'Le', country: 'Sierra Leone', region: 'West Africa', decimals: 2, isActive: true },
  LRD: { code: 'LRD', name: 'Liberian Dollar', symbol: 'L$', country: 'Liberia', region: 'West Africa', decimals: 2, isActive: true },
  GMD: { code: 'GMD', name: 'Gambian Dalasi', symbol: 'D', country: 'Gambia', region: 'West Africa', decimals: 2, isActive: true },
  GNF: { code: 'GNF', name: 'Guinean Franc', symbol: 'FG', country: 'Guinea', region: 'West Africa', decimals: 0, isActive: true },
  
  // Central Africa
  XAF: { code: 'XAF', name: 'Central African CFA Franc', symbol: 'FCFA', country: 'CEMAC', region: 'Central Africa', decimals: 0, isActive: true },
  CDF: { code: 'CDF', name: 'Congolese Franc', symbol: 'FC', country: 'DR Congo', region: 'Central Africa', decimals: 2, isActive: true },
  AOA: { code: 'AOA', name: 'Angolan Kwanza', symbol: 'Kz', country: 'Angola', region: 'Central Africa', decimals: 2, isActive: true },
  
  // Southern Africa
  ZAR: { code: 'ZAR', name: 'South African Rand', symbol: 'R', country: 'South Africa', region: 'Southern Africa', decimals: 2, isActive: true },
  BWP: { code: 'BWP', name: 'Botswana Pula', symbol: 'P', country: 'Botswana', region: 'Southern Africa', decimals: 2, isActive: true },
  SZL: { code: 'SZL', name: 'Swazi Lilangeni', symbol: 'L', country: 'Eswatini', region: 'Southern Africa', decimals: 2, isActive: true },
  LSL: { code: 'LSL', name: 'Lesotho Loti', symbol: 'L', country: 'Lesotho', region: 'Southern Africa', decimals: 2, isActive: true },
  NAD: { code: 'NAD', name: 'Namibian Dollar', symbol: 'N$', country: 'Namibia', region: 'Southern Africa', decimals: 2, isActive: true },
  ZMW: { code: 'ZMW', name: 'Zambian Kwacha', symbol: 'ZK', country: 'Zambia', region: 'Southern Africa', decimals: 2, isActive: true },
  ZWL: { code: 'ZWL', name: 'Zimbabwean Dollar', symbol: 'Z$', country: 'Zimbabwe', region: 'Southern Africa', decimals: 2, isActive: false },
  MWK: { code: 'MWK', name: 'Malawian Kwacha', symbol: 'MK', country: 'Malawi', region: 'Southern Africa', decimals: 2, isActive: true },
  MZN: { code: 'MZN', name: 'Mozambican Metical', symbol: 'MT', country: 'Mozambique', region: 'Southern Africa', decimals: 2, isActive: true },
  
  // North Africa
  EGP: { code: 'EGP', name: 'Egyptian Pound', symbol: '£E', country: 'Egypt', region: 'North Africa', decimals: 2, isActive: true },
  MAD: { code: 'MAD', name: 'Moroccan Dirham', symbol: 'DH', country: 'Morocco', region: 'North Africa', decimals: 2, isActive: true },
  TND: { code: 'TND', name: 'Tunisian Dinar', symbol: 'DT', country: 'Tunisia', region: 'North Africa', decimals: 3, isActive: true },
  DZD: { code: 'DZD', name: 'Algerian Dinar', symbol: 'DA', country: 'Algeria', region: 'North Africa', decimals: 2, isActive: true },
  LYD: { code: 'LYD', name: 'Libyan Dinar', symbol: 'LD', country: 'Libya', region: 'North Africa', decimals: 3, isActive: false },
  SDG: { code: 'SDG', name: 'Sudanese Pound', symbol: '£SD', country: 'Sudan', region: 'North Africa', decimals: 2, isActive: false },
  MRU: { code: 'MRU', name: 'Mauritanian Ouguiya', symbol: 'UM', country: 'Mauritania', region: 'North Africa', decimals: 2, isActive: true },
  
  // Island Nations
  MUR: { code: 'MUR', name: 'Mauritian Rupee', symbol: '₨', country: 'Mauritius', region: 'Island Nations', decimals: 2, isActive: true },
  SCR: { code: 'SCR', name: 'Seychellois Rupee', symbol: '₨', country: 'Seychelles', region: 'Island Nations', decimals: 2, isActive: true },
  CVE: { code: 'CVE', name: 'Cape Verdean Escudo', symbol: '$', country: 'Cape Verde', region: 'Island Nations', decimals: 2, isActive: true },
  STN: { code: 'STN', name: 'São Tomé and Príncipe Dobra', symbol: 'Db', country: 'São Tomé and Príncipe', region: 'Island Nations', decimals: 2, isActive: true },
  KMF: { code: 'KMF', name: 'Comorian Franc', symbol: 'CF', country: 'Comoros', region: 'Island Nations', decimals: 0, isActive: true },
  DJF: { code: 'DJF', name: 'Djiboutian Franc', symbol: 'Fdj', country: 'Djibouti', region: 'East Africa', decimals: 0, isActive: true },
  ERN: { code: 'ERN', name: 'Eritrean Nakfa', symbol: 'Nfk', country: 'Eritrea', region: 'East Africa', decimals: 2, isActive: false },
  MGA: { code: 'MGA', name: 'Malagasy Ariary', symbol: 'Ar', country: 'Madagascar', region: 'Island Nations', decimals: 2, isActive: true },
  
  // Digital
  BTC: { code: 'BTC', name: 'Bitcoin', symbol: '₿', country: 'Global', region: 'Digital', decimals: 8, isActive: true }
};

// Get active currencies by region
export function getCurrenciesByRegion(region: CurrencyInfo['region']): CurrencyInfo[] {
  return Object.values(AFRICAN_CURRENCIES).filter(
    currency => currency.region === region && currency.isActive
  );
}

// Get all active currencies
export function getActiveCurrencies(): CurrencyInfo[] {
  return Object.values(AFRICAN_CURRENCIES).filter(currency => currency.isActive);
}

// Format currency amount with proper decimals and symbol
export function formatCurrencyAmount(amount: number, currency: AfricanCurrency): string {
  const currencyInfo = AFRICAN_CURRENCIES[currency];
  const formattedAmount = amount.toLocaleString('en-US', {
    minimumFractionDigits: currencyInfo.decimals,
    maximumFractionDigits: currencyInfo.decimals
  });
  
  return `${currencyInfo.symbol} ${formattedAmount}`;
}

// Parse currency amount from string
export function parseCurrencyAmount(amountString: string, currency: AfricanCurrency): number {
  const currencyInfo = AFRICAN_CURRENCIES[currency];
  const cleanAmount = amountString.replace(/[^\d.-]/g, '');
  const amount = parseFloat(cleanAmount);
  
  if (isNaN(amount)) return 0;
  
  // Round to appropriate decimal places
  return Math.round(amount * Math.pow(10, currencyInfo.decimals)) / Math.pow(10, currencyInfo.decimals);
}

// Get currency by country code (ISO 3166-1 alpha-2)
export function getCurrencyByCountry(countryCode: string): CurrencyInfo | null {
  const countryToCurrency: Record<string, AfricanCurrency> = {
    'UG': 'UGX', 'KE': 'KES', 'TZ': 'TZS', 'RW': 'RWF', 'BI': 'BIF', 'ET': 'ETB', 'SO': 'SOS',
    'NG': 'NGN', 'GH': 'GHS', 'SL': 'SLL', 'LR': 'LRD', 'GM': 'GMD', 'GN': 'GNF',
    'CM': 'XAF', 'CF': 'XAF', 'TD': 'XAF', 'CG': 'XAF', 'GQ': 'XAF', 'GA': 'XAF',
    'CD': 'CDF', 'AO': 'AOA',
    'ZA': 'ZAR', 'BW': 'BWP', 'SZ': 'SZL', 'LS': 'LSL', 'NA': 'NAD', 
    'ZM': 'ZMW', 'ZW': 'ZWL', 'MW': 'MWK', 'MZ': 'MZN',
    'EG': 'EGP', 'MA': 'MAD', 'TN': 'TND', 'DZ': 'DZD', 'LY': 'LYD', 'SD': 'SDG', 'MR': 'MRU',
    'MU': 'MUR', 'SC': 'SCR', 'CV': 'CVE', 'ST': 'STN', 'KM': 'KMF', 
    'DJ': 'DJF', 'ER': 'ERN', 'MG': 'MGA',
    // CFA Franc countries
    'BJ': 'XOF', 'BF': 'XOF', 'CI': 'XOF', 'GW': 'XOF', 'ML': 'XOF', 'NE': 'XOF', 'SN': 'XOF', 'TG': 'XOF'
  };
  
  const currency = countryToCurrency[countryCode.toUpperCase()];
  return currency ? AFRICAN_CURRENCIES[currency] : null;
}
