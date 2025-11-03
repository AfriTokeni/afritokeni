/**
 * Environment Configuration
 * 
 * Centralized environment variable access following SvelteKit conventions:
 * - Public vars (client + server): $env/static/public with PUBLIC_ prefix
 * - Private vars (server only): $env/static/private
 * 
 * This file provides a unified interface for accessing env vars throughout the app.
 */

import { browser } from '$app/environment';

// For server-side code, import from SvelteKit's env modules
// For client-side, we'll use public env vars

/**
 * Get environment variable with fallback
 * Works in both browser and server contexts
 */
function getEnv(key: string, defaultValue: string = ''): string {
	if (browser) {
		// Client-side: only access public vars (PUBLIC_ prefix)
		// @ts-ignore - window.__ENV__ is set by SvelteKit
		return window?.__ENV__?.[key] || defaultValue;
	}
	
	// Server-side: access via process.env (available in +server.ts files)
	if (typeof process !== 'undefined' && process.env) {
		return process.env[key] || defaultValue;
	}
	
	return defaultValue;
}

/**
 * Public environment variables (available on client and server)
 * These MUST be prefixed with PUBLIC_ in .env
 */
export const PUBLIC_ENV = {
	// SNS DAO Canisters
	SNS_GOVERNANCE_CANISTER: getEnv('PUBLIC_SNS_GOVERNANCE_CANISTER', 'kly22-hyaaa-aaaac-qceeq-cai'),
	SNS_LEDGER_CANISTER: getEnv('PUBLIC_SNS_LEDGER_CANISTER', 'kf2xs-4iaaa-aaaac-qcefq-cai'),
	SNS_ROOT_CANISTER: getEnv('PUBLIC_SNS_ROOT_CANISTER', 'kq5g7-5aaaa-aaaac-qcega-cai'),
	SNS_SWAP_CANISTER: getEnv('PUBLIC_SNS_SWAP_CANISTER', 'kx4al-qyaaa-aaaac-qcegq-cai'),
	SNS_INDEX_CANISTER: getEnv('PUBLIC_SNS_INDEX_CANISTER', 'kc3rg-rqaaa-aaaac-qcefa-cai'),
	
	// Canister IDs
	DEPOSIT_CANISTER_ID: getEnv('PUBLIC_DEPOSIT_CANISTER_ID', ''),
	WITHDRAWAL_CANISTER_ID: getEnv('PUBLIC_WITHDRAWAL_CANISTER_ID', ''),
	EXCHANGE_CANISTER_ID: getEnv('PUBLIC_EXCHANGE_CANISTER_ID', ''),
	
	// Development Canister IDs
	DEV_DEPOSIT_CANISTER_ID: getEnv('PUBLIC_DEV_DEPOSIT_CANISTER_ID', ''),
	DEV_WITHDRAWAL_CANISTER_ID: getEnv('PUBLIC_DEV_WITHDRAWAL_CANISTER_ID', ''),
	
	// Juno
	JUNO_SATELLITE_ID: getEnv('PUBLIC_JUNO_SATELLITE_ID', ''),
	
	// Feature flags
	DEMO_MODE: getEnv('PUBLIC_DEMO_MODE', 'true') === 'true',
	USE_LOCAL_REPLICA: getEnv('PUBLIC_USE_LOCAL_REPLICA', 'false') === 'true',
	
	// DAO Config
	DAO_MIN_TOKENS_TO_PROPOSE: Number(getEnv('PUBLIC_DAO_MIN_TOKENS_TO_PROPOSE', '1000')),
	DAO_MIN_TOKENS_TO_VOTE: Number(getEnv('PUBLIC_DAO_MIN_TOKENS_TO_VOTE', '1')),
	DAO_QUORUM_PERCENTAGE: Number(getEnv('PUBLIC_DAO_QUORUM_PERCENTAGE', '10')),
	DAO_PASS_THRESHOLD: Number(getEnv('PUBLIC_DAO_PASS_THRESHOLD', '50')),
	DAO_VOTING_PERIOD_DAYS: Number(getEnv('PUBLIC_DAO_VOTING_PERIOD_DAYS', '7')),
	
	// Exchange rates
	EXCHANGE_RATE_API_URL: getEnv('PUBLIC_EXCHANGE_RATE_API_URL', 'https://api.coingecko.com/api/v3/simple/price'),
	EXCHANGE_RATE_CACHE_DURATION: Number(getEnv('PUBLIC_EXCHANGE_RATE_CACHE_DURATION', '60000')),
	EXCHANGE_SPREAD_PERCENTAGE: Number(getEnv('PUBLIC_EXCHANGE_SPREAD_PERCENTAGE', '0.5')),
};

/**
 * Server-only environment variables
 * These are NEVER exposed to the client
 * 
 * Import these only in +server.ts files or server-side code
 */
export const PRIVATE_ENV = {
	// Africa's Talking (SMS/USSD)
	AT_USERNAME: getEnv('AT_USERNAME', 'sandbox'),
	AT_API_KEY: getEnv('AT_API_KEY', ''),
	AT_SHORT_CODE: getEnv('AT_SHORT_CODE', ''),
	
	// Email (Resend)
	RESEND_API_KEY: getEnv('RESEND_API_KEY', ''),
	EMAIL_FROM_DOMAIN: getEnv('EMAIL_FROM_DOMAIN', 'afritokeni.com'),
	
	// Node environment
	NODE_ENV: getEnv('NODE_ENV', 'development'),
};

/**
 * Helper to check if we're in development
 */
export const isDevelopment = PRIVATE_ENV.NODE_ENV === 'development';

/**
 * Helper to check if we're in production
 */
export const isProduction = PRIVATE_ENV.NODE_ENV === 'production';
