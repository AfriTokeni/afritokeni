/**
 * ICP Canister Configuration
 * 
 * SECURITY NOTICE:
 * These are PUBLIC canister IDs on the Internet Computer blockchain.
 * They are NOT private keys or secrets.
 * 
 * NON-CUSTODIAL ARCHITECTURE:
 * - AfriTokeni does NOT own these canisters
 * - These are ICP mainnet canisters managed by DFINITY Foundation
 * - Users interact DIRECTLY with these canisters using their own principals
 * - AfriTokeni NEVER holds user funds - all funds are in user-controlled principals
 */

import { browser } from '$app/environment';

/**
 * Get environment variable (works in both browser and server)
 */
function getEnv(key: string): string | undefined {
	if (browser) {
		// Browser: use import.meta.env
		return (import.meta as any).env[key];
	}
	// Server: use process.env
	return process.env[key];
}

/**
 * ckBTC Ledger Canister ID
 * 
 * This is the ICRC-1 ledger for Chain-Key Bitcoin on ICP mainnet.
 * - Mainnet: mxzaz-hqaaa-aaaar-qaada-cai
 * - Local dfx: Set via CANISTER_ID_CKBTC_LEDGER env var
 * 
 * SECURITY:
 * - This canister is controlled by ICP Network Nervous System (NNS)
 * - AfriTokeni has NO special access to this canister
 * - All users interact with it using their own principals
 * - Balances are stored on-chain, NOT in AfriTokeni databases
 */
export const CKBTC_LEDGER_CANISTER_ID = 
	getEnv('CANISTER_ID_CKBTC_LEDGER') || 
	'mxzaz-hqaaa-aaaar-qaada-cai';

/**
 * ckBTC Minter Canister ID
 * 
 * This canister handles Bitcoin deposits and withdrawals.
 * - Mainnet: mqygn-kiaaa-aaaar-qaadq-cai
 * 
 * SECURITY:
 * - Managed by DFINITY Foundation
 * - Users get unique Bitcoin deposit addresses
 * - AfriTokeni does NOT control Bitcoin deposits
 * - Users withdraw Bitcoin directly to their own addresses
 */
export const CKBTC_MINTER_CANISTER_ID = 
	'mqygn-kiaaa-aaaar-qaadq-cai';

/**
 * ckUSD Ledger Canister ID
 * 
 * This is the ICRC-1 ledger for Chain-Key USD (stablecoin) on ICP mainnet.
 * - Mainnet: xevnm-gaaaa-aaaar-qafnq-cai
 * - Local dfx: Set via CANISTER_ID_CKUSD_LEDGER env var
 * 
 * SECURITY:
 * - This canister is controlled by ICP Network Nervous System (NNS)
 * - AfriTokeni has NO special access to this canister
 * - All users interact with it using their own principals
 * - Balances are stored on-chain, NOT in AfriTokeni databases
 * 
 * NOTE: Previously called ckUSDC in old codebase, now renamed to ckUSD
 */
export const CKUSD_LEDGER_CANISTER_ID = 
	getEnv('CANISTER_ID_CKUSD_LEDGER') || 
	'xevnm-gaaaa-aaaar-qafnq-cai';

/**
 * Juno Satellite Canister ID
 * 
 * This is AfriTokeni's Juno satellite for storing metadata.
 * - Development: uxrrr-q7777-77774-qaaaq-cai
 * - Production: Set via VITE_JUNO_SATELLITE_ID env var
 * 
 * WHAT WE STORE IN JUNO:
 * ✅ Transaction metadata (IDs, timestamps, status)
 * ✅ User preferences (currency, language)
 * ✅ Agent information (location, ratings)
 * 
 * WHAT WE NEVER STORE IN JUNO:
 * ❌ Private keys
 * ❌ Actual funds
 * ❌ Control over user balances
 * 
 * SECURITY:
 * - Juno is for METADATA only, not for holding funds
 * - Even if Juno is compromised, user funds are safe (they're on ICP ledger)
 * - Users can still access funds via Internet Identity even if Juno is down
 */
export const JUNO_SATELLITE_ID = 
	getEnv('VITE_JUNO_SATELLITE_ID') || 
	getEnv('VITE_DEVELOPMENT_JUNO_SATELLITE_ID') ||
	'uxrrr-q7777-77774-qaaaq-cai';

/**
 * ICP Host URL
 * 
 * - Local development: http://localhost:4943 (dfx)
 * - Production: https://ic0.app (ICP mainnet)
 * 
 * SECURITY:
 * - Always use HTTPS in production
 * - Local HTTP is only for development with dfx
 */
export const IC_HOST = 
	getEnv('DFX_NETWORK') === 'local' 
		? 'http://localhost:4943'
		: 'https://ic0.app';

/**
 * AfriTokeni Fee Canister ID
 * 
 * This is AfriTokeni's fee collection canister (smart contract).
 * - Handles automatic 0.5% platform fee deduction
 * - Handles 10% cut of agent commissions
 * - Transparent, auditable code
 * - Non-custodial (only routes funds, doesn't hold them)
 * 
 * SECURITY:
 * - Open source canister code
 * - Upgradeable via DAO governance
 * - All transactions logged on-chain
 * - Cannot hold funds (atomic routing only)
 * 
 * TODO: Deploy fee canister and update this ID
 */
export const AFRITOKENI_FEE_CANISTER_ID = 
	getEnv('VITE_AFRITOKENI_FEE_CANISTER_ID') || 
	'aaaaa-aa'; // Placeholder - deploy canister first

/**
 * AfriTokeni Principal (for receiving fees)
 * 
 * This is where platform fees are collected.
 * - 0.5% of all transfers
 * - 0.5% of crypto swaps
 * - 10% of agent commissions
 * 
 * SECURITY:
 * - This is AfriTokeni's ICP principal (like a wallet address)
 * - Fees are sent here automatically by fee canister
 * - Controlled by AfriTokeni DAO (not individual)
 * - Transparent on-chain
 * 
 * TODO: Set up AfriTokeni DAO principal
 */
export const AFRITOKENI_PRINCIPAL_ID = 
	getEnv('VITE_AFRITOKENI_PRINCIPAL_ID') || 
	'aaaaa-aa'; // Placeholder - set up DAO principal first

/**
 * Platform Fee Percentage
 * 
 * Charged on all transfers (user→user, user→agent)
 */
export const PLATFORM_FEE_PERCENTAGE = 0.005; // 0.5%

/**
 * Exchange Spread Percentage
 * 
 * Charged on ckBTC ↔ ckUSD swaps
 */
export const EXCHANGE_SPREAD_PERCENTAGE = 0.005; // 0.5%

/**
 * Agent Commission Cut
 * 
 * AfriTokeni's share of agent commissions
 * Agents keep 90%, AfriTokeni gets 10%
 */
export const AGENT_COMMISSION_CUT = 0.10; // 10%

/**
 * Is Local Development?
 */
export const IS_LOCAL_DEV = getEnv('DFX_NETWORK') === 'local';

/**
 * Canister Configuration Summary
 * 
 * For debugging and verification
 */
export function getCanisterConfig() {
	return {
		ckBTC: {
			ledger: CKBTC_LEDGER_CANISTER_ID,
			minter: CKBTC_MINTER_CANISTER_ID,
		},
		ckUSD: {
			ledger: CKUSD_LEDGER_CANISTER_ID,
		},
		juno: {
			satellite: JUNO_SATELLITE_ID,
		},
		network: {
			host: IC_HOST,
			isLocal: IS_LOCAL_DEV,
		},
	};
}

/**
 * Verify Canister Configuration
 * 
 * Logs configuration for debugging
 * SAFE to call - only logs public canister IDs (not secrets)
 */
export function logCanisterConfig() {
	const config = getCanisterConfig();
	console.log('🔧 ICP Canister Configuration:', config);
	console.log('🔒 NON-CUSTODIAL: AfriTokeni does NOT control user funds');
	console.log('✅ All user balances are stored on-chain in user-controlled principals');
}
