/**
 * Canister Configuration
 * 
 * SNS (Service Nervous System) canister IDs for AfriTokeni DAO
 * Loaded from environment variables (.env file)
 */

// Get canister IDs from environment variables (VITE_ prefix for SvelteKit)
export const CANISTER_IDS = {
	// SNS Governance - handles proposals and voting
	SNS_GOVERNANCE: import.meta.env.VITE_SNS_GOVERNANCE_CANISTER || 'kly22-hyaaa-aaaac-qceeq-cai',
	
	// SNS Index - indexes ledger transactions
	SNS_INDEX: import.meta.env.VITE_SNS_INDEX_CANISTER || 'kc3rg-rqaaa-aaaac-qcefa-cai',
	
	// SNS Ledger - token ledger
	SNS_LEDGER: import.meta.env.VITE_SNS_LEDGER_CANISTER || 'kf2xs-4iaaa-aaaac-qcefq-cai',
	
	// SNS Root - root canister
	SNS_ROOT: import.meta.env.VITE_SNS_ROOT_CANISTER || 'kq5g7-5aaaa-aaaac-qcega-cai',
	
	// SNS Swap - token swap
	SNS_SWAP: import.meta.env.VITE_SNS_SWAP_CANISTER || 'kx4al-qyaaa-aaaac-qcegq-cai'
} as const;

// Network configuration
export const NETWORK = {
	// Use IC mainnet
	HOST: 'https://ic0.app',
	
	// Local development (only use if explicitly set)
	LOCAL_HOST: 'http://localhost:4943'
} as const;

// Determine if we should use local replica
// Only use local if explicitly set via env variable
export const USE_LOCAL_REPLICA = import.meta.env.VITE_USE_LOCAL_REPLICA === 'true';

// Get the appropriate host
// For SNS canisters, ALWAYS use mainnet unless explicitly told to use local
export const getHost = () => USE_LOCAL_REPLICA ? NETWORK.LOCAL_HOST : NETWORK.HOST;

// DAO Governance Configuration (from environment variables)
export const DAO_CONFIG = {
	MIN_TOKENS_TO_PROPOSE: Number(import.meta.env.VITE_DAO_MIN_TOKENS_TO_PROPOSE) || 1000,
	MIN_TOKENS_TO_VOTE: Number(import.meta.env.VITE_DAO_MIN_TOKENS_TO_VOTE) || 1,
	QUORUM_PERCENTAGE: Number(import.meta.env.VITE_DAO_QUORUM_PERCENTAGE) || 10,
	PASS_THRESHOLD: Number(import.meta.env.VITE_DAO_PASS_THRESHOLD) || 50,
	VOTING_PERIOD_DAYS: Number(import.meta.env.VITE_DAO_VOTING_PERIOD_DAYS) || 7,
} as const;
