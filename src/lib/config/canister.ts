/**
 * Canister Configuration
 * 
 * SNS (Service Nervous System) canister IDs for AfriTokeni DAO
 * Uses SvelteKit's public environment variables
 */

import {
	PUBLIC_SNS_GOVERNANCE_CANISTER,
	PUBLIC_SNS_INDEX_CANISTER,
	PUBLIC_SNS_LEDGER_CANISTER,
	PUBLIC_SNS_ROOT_CANISTER,
	PUBLIC_SNS_SWAP_CANISTER,
	PUBLIC_USE_LOCAL_REPLICA,
	PUBLIC_DAO_MIN_TOKENS_TO_PROPOSE,
	PUBLIC_DAO_MIN_TOKENS_TO_VOTE,
	PUBLIC_DAO_QUORUM_PERCENTAGE,
	PUBLIC_DAO_PASS_THRESHOLD,
	PUBLIC_DAO_VOTING_PERIOD_DAYS
} from '$env/static/public';

// Get canister IDs from environment variables
export const CANISTER_IDS = {
	// SNS Governance - handles proposals and voting
	SNS_GOVERNANCE: PUBLIC_SNS_GOVERNANCE_CANISTER,
	
	// SNS Index - indexes ledger transactions
	SNS_INDEX: PUBLIC_SNS_INDEX_CANISTER,
	
	// SNS Ledger - token ledger
	SNS_LEDGER: PUBLIC_SNS_LEDGER_CANISTER,
	
	// SNS Root - root canister
	SNS_ROOT: PUBLIC_SNS_ROOT_CANISTER,
	
	// SNS Swap - token swap
	SNS_SWAP: PUBLIC_SNS_SWAP_CANISTER
} as const;

// Network configuration
export const NETWORK = {
	// Use IC mainnet
	HOST: 'https://ic0.app',
	
	// Local development (only use if explicitly set)
	LOCAL_HOST: 'http://localhost:4943'
} as const;

// Determine if we should use local replica
export const USE_LOCAL_REPLICA = PUBLIC_USE_LOCAL_REPLICA === 'true';

// Get the appropriate host
export const getHost = () => USE_LOCAL_REPLICA ? NETWORK.LOCAL_HOST : NETWORK.HOST;

// DAO Governance Configuration
// NO FALLBACKS - env vars must be set in .env
export const DAO_CONFIG = {
	MIN_TOKENS_TO_PROPOSE: Number(PUBLIC_DAO_MIN_TOKENS_TO_PROPOSE),
	MIN_TOKENS_TO_VOTE: Number(PUBLIC_DAO_MIN_TOKENS_TO_VOTE),
	QUORUM_PERCENTAGE: Number(PUBLIC_DAO_QUORUM_PERCENTAGE),
	PASS_THRESHOLD: Number(PUBLIC_DAO_PASS_THRESHOLD),
	VOTING_PERIOD_DAYS: Number(PUBLIC_DAO_VOTING_PERIOD_DAYS),
} as const;
