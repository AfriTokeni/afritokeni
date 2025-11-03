/**
 * Centralized Mock Service
 * Single source of truth for mock data in unit tests and playground
 */

// Global flag for test mode (set by test setup)
declare global {
  var __AFRITOKENI_TEST_MODE__: boolean | undefined;
}

/**
 * Check if we should use mocks
 * Returns true for:
 * - Unit tests (global test flag or NODE_ENV === 'unit-test' or 'test')
 * - USSD Playground (browser on localhost with /ussd path)
 */
export function shouldUseMocks(): boolean {
  // Check global test flag first (set by test setup)
  if (typeof globalThis !== 'undefined' && globalThis.__AFRITOKENI_TEST_MODE__) {
    console.log('🎭 shouldUseMocks: true (global test flag)');
    return true;
  }
  
  // Unit test mode (Node.js environment)
  if (typeof process !== 'undefined' && process.env) {
    const nodeEnv = process.env.NODE_ENV;
    if (nodeEnv === 'unit-test' || nodeEnv === 'test') {
      console.log('🎭 shouldUseMocks: true (NODE_ENV=' + nodeEnv + ')');
      return true;
    }
  }
  
  // Playground mode (browser only - USSD playground page)
  if (typeof window !== 'undefined') {
    const isLocalhost = window.location?.hostname === 'localhost';
    const isUSSDPlayground = window.location?.pathname?.includes('/ussd');
    
    if (isLocalhost && isUSSDPlayground) {
      console.log('🎭 shouldUseMocks: true (USSD playground mode)');
      return true;
    }
  }
  
  console.log('🎭 shouldUseMocks: false (production mode)');
  return false;
}

/**
 * Mock ckBTC balance
 */
export const MOCK_CKBTC_BALANCE = {
  balanceSatoshis: 50000,
  balanceBTC: '0.0005',
  localCurrencyEquivalent: 193208,
  lastUpdated: new Date()
};

/**
 * Mock ckUSDC balance
 */
export const MOCK_CKUSDC_BALANCE = {
  balanceUSDC: '100.00',
  balanceCents: 10000,
  localCurrencyEquivalent: 380000,
  lastUpdated: new Date()
};

/**
 * Mock BTC exchange rate (1 BTC = 386M UGX)
 */
export const MOCK_BTC_RATE = {
  rate: 386416858,
  lastUpdated: new Date(),
  source: 'Mock',
  currency: 'UGX'
};

/**
 * Mock USDC exchange rate (1 USDC = 3800 UGX)
 */
export const MOCK_USDC_RATE = {
  rate: 3800,
  lastUpdated: new Date(),
  source: 'Mock',
  currency: 'UGX'
};

/**
 * Mock user balance (1M UGX)
 */
export const MOCK_USER_BALANCE = {
  userId: 'mock-user',
  balance: 1000000,
  currency: 'UGX' as const,
  lastUpdated: new Date()
};
