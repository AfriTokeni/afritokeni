/**
 * Canister Configuration
 * 
 * SNS (Service Nervous System) canister IDs for AfriTokeni DAO
 * Uses centralized env config
 */

import { PUBLIC_ENV } from './env';

// Get canister IDs from environment variables
export const CANISTER_IDS = {
	// SNS Governance - handles proposals and voting
	SNS_GOVERNANCE: PUBLIC_ENV.SNS_GOVERNANCE_CANISTER,
	
	// SNS Index - indexes ledger transactions
	SNS_INDEX: PUBLIC_ENV.SNS_INDEX_CANISTER,
	
	// SNS Ledger - token ledger
	SNS_LEDGER: PUBLIC_ENV.SNS_LEDGER_CANISTER,
	
	// SNS Root - root canister
	SNS_ROOT: PUBLIC_ENV.SNS_ROOT_CANISTER,
	
	// SNS Swap - token swap
	SNS_SWAP: PUBLIC_ENV.SNS_SWAP_CANISTER
} as const;

// Network configuration
export const NETWORK = {
	// Use IC mainnet
	HOST: 'https://ic0.app',
	
	// Local development (only use if explicitly set)
	LOCAL_HOST: 'http://localhost:4943'
} as const;

// Determine if we should use local replica
export const USE_LOCAL_REPLICA = PUBLIC_ENV.USE_LOCAL_REPLICA;

// Get the appropriate host
export const getHost = () => USE_LOCAL_REPLICA ? NETWORK.LOCAL_HOST : NETWORK.HOST;

// DAO Governance Configuration
export const DAO_CONFIG = {
	MIN_TOKENS_TO_PROPOSE: PUBLIC_ENV.DAO_MIN_TOKENS_TO_PROPOSE,
	MIN_TOKENS_TO_VOTE: PUBLIC_ENV.DAO_MIN_TOKENS_TO_VOTE,
	QUORUM_PERCENTAGE: PUBLIC_ENV.DAO_QUORUM_PERCENTAGE,
	PASS_THRESHOLD: PUBLIC_ENV.DAO_PASS_THRESHOLD,
	VOTING_PERIOD_DAYS: PUBLIC_ENV.DAO_VOTING_PERIOD_DAYS,
} as const;
