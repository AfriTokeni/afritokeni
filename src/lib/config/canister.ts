/**
 * Canister Configuration
 *
 * SNS (Service Nervous System) canister IDs for AfriTokeni DAO
 * Uses SvelteKit's public environment variables
 */

import * as env from "$env/static/public";

// Get canister IDs from environment variables (with fallbacks for development)
export const CANISTER_IDS = {
  // SNS Governance - handles proposals and voting
  SNS_GOVERNANCE:
    (env as Record<string, string>).PUBLIC_SNS_GOVERNANCE_CANISTER || "",

  // SNS Index - indexes ledger transactions
  SNS_INDEX: (env as Record<string, string>).PUBLIC_SNS_INDEX_CANISTER || "",

  // SNS Ledger - token ledger
  SNS_LEDGER: (env as Record<string, string>).PUBLIC_SNS_LEDGER_CANISTER || "",

  // SNS Root - root canister
  SNS_ROOT: (env as Record<string, string>).PUBLIC_SNS_ROOT_CANISTER || "",

  // SNS Swap - token swap
  SNS_SWAP: (env as Record<string, string>).PUBLIC_SNS_SWAP_CANISTER || "",
} as const;

// Network configuration
export const NETWORK = {
  // Use IC mainnet
  HOST: "https://ic0.app",

  // Local development (only use if explicitly set)
  LOCAL_HOST: "http://localhost:4943",
} as const;

// Determine if we should use local replica
export const USE_LOCAL_REPLICA =
  (env as Record<string, string>).PUBLIC_USE_LOCAL_REPLICA === "true";

// Get the appropriate host
export const getHost = () =>
  USE_LOCAL_REPLICA ? NETWORK.LOCAL_HOST : NETWORK.HOST;

// DAO Governance Configuration (with sensible defaults)
export const DAO_CONFIG = {
  MIN_TOKENS_TO_PROPOSE:
    Number((env as Record<string, string>).PUBLIC_DAO_MIN_TOKENS_TO_PROPOSE) ||
    1000,
  MIN_TOKENS_TO_VOTE:
    Number((env as Record<string, string>).PUBLIC_DAO_MIN_TOKENS_TO_VOTE) || 1,
  QUORUM_PERCENTAGE:
    Number((env as Record<string, string>).PUBLIC_DAO_QUORUM_PERCENTAGE) || 20,
  PASS_THRESHOLD:
    Number((env as Record<string, string>).PUBLIC_DAO_PASS_THRESHOLD) || 51,
  VOTING_PERIOD_DAYS:
    Number((env as Record<string, string>).PUBLIC_DAO_VOTING_PERIOD_DAYS) || 7,
} as const;
