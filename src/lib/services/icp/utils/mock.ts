/**
 * Mock Service for ICP Integration Testing
 * 
 * Provides mock data for development and testing without real ICP calls.
 * 
 * SECURITY NOTE:
 * - This is for TESTING only
 * - Mock data does NOT represent real user funds
 * - In production, all data comes from ICP blockchain
 * - Users ALWAYS control their own funds (even in production)
 */

import { browser } from '$app/environment';

/**
 * Global test mode flag
 * 
 * Set by test environment to force mock usage
 */
declare global {
	var __AFRITOKENI_TEST_MODE__: boolean | undefined;
}

/**
 * Determine if we should use mocks instead of real ICP calls
 * 
 * USAGE:
 * - Returns true in test environment (unit tests, BDD tests)
 * - Returns false in production (real ICP blockchain)
 * - Demo mode uses JSON files, not mocks (handled separately)
 * 
 * SECURITY:
 * - Mocks are for TESTING only
 * - Production ALWAYS uses real ICP blockchain
 * - Users ALWAYS control their own funds
 */
export function shouldUseMocks(): boolean {
	// Check global test flag (set by test environment)
	if (typeof globalThis !== 'undefined' && globalThis.__AFRITOKENI_TEST_MODE__) {
		return true;
	}

	// Check Node.js test environment
	if (typeof process !== 'undefined' && process.env?.NODE_ENV === 'test') {
		return true;
	}

	// Check browser test environment
	if (browser && typeof window !== 'undefined') {
		// Check if running in test environment
		if ((window as any).__AFRITOKENI_TEST_MODE__) {
			return true;
		}
	}

	// Production: use real ICP
	return false;
}

/**
 * Mock ckBTC Balance
 * 
 * TESTING ONLY - Not real funds!
 */
export const MOCK_CKBTC_BALANCE = {
	balanceSatoshis: 50000000, // 0.5 ckBTC (in satoshis)
	balanceBTC: '0.50000000',
	lastUpdated: new Date()
};

/**
 * Mock ckUSD Balance
 * 
 * TESTING ONLY - Not real funds!
 */
export const MOCK_CKUSD_BALANCE = {
	balance: BigInt(100000000), // 100 ckUSD (in smallest units)
	decimals: 6
};

/**
 * Mock Bitcoin Exchange Rate
 * 
 * TESTING ONLY - Not real market data!
 */
export const MOCK_BTC_RATE = {
	currency: 'USD',
	rate: 45000,
	lastUpdated: new Date(),
	source: 'mock'
};

/**
 * Mock USD Exchange Rate
 * 
 * TESTING ONLY - Not real market data!
 */
export const MOCK_USD_RATE = {
	currency: 'UGX',
	rate: 3800, // 1 USD = 3800 UGX
	lastUpdated: new Date(),
	source: 'mock'
};

/**
 * Mock Transaction ID Generator
 * 
 * TESTING ONLY - Not real blockchain transaction IDs!
 */
export function generateMockTransactionId(): string {
	return `MOCK-TX-${Date.now()}-${Math.random().toString(36).substring(7)}`;
}

/**
 * Mock Principal ID for Testing
 * 
 * TESTING ONLY - Not a real ICP principal!
 */
export const MOCK_PRINCIPAL_ID = 'aaaaa-aa';

/**
 * Log mock usage warning
 * 
 * Warns developers when mocks are being used
 */
export function logMockWarning(operation: string): void {
	if (shouldUseMocks()) {
		console.warn(
			`⚠️ MOCK MODE: ${operation} is using mock data (not real ICP blockchain)`
		);
		console.warn('🔒 In production, all operations use real ICP with user-controlled funds');
	}
}

/**
 * Simulate ICP call delay for realistic testing
 * 
 * TESTING ONLY - Simulates network latency
 */
export async function simulateICPDelay(ms: number = 100): Promise<void> {
	if (shouldUseMocks()) {
		await new Promise((resolve) => setTimeout(resolve, ms));
	}
}
