/**
 * ICP Service - Main Export
 * 
 * Central export point for all ICP-related functionality.
 * 
 * ⚠️ CRITICAL SECURITY REMINDER:
 * 
 * AfriTokeni is 100% NON-CUSTODIAL:
 * - We NEVER hold user funds
 * - We NEVER control user private keys
 * - We NEVER sign transactions without user consent
 * - All funds are in user-controlled ICP principals
 * - Transfers are ALWAYS direct: user → agent or agent → user
 * 
 * This service provides:
 * ✅ Read-only queries to display user balances
 * ✅ Interfaces for users to sign their own transactions
 * ✅ Metadata storage (transaction history, NOT funds)
 * 
 * This service NEVER provides:
 * ❌ Custody of user funds
 * ❌ Control over user balances
 * ❌ Ability to move funds without user signature
 * 
 * See /static/WHITEPAPER.md Section 6.3 for full details.
 */

// Canister configuration
export {
	CKBTC_LEDGER_CANISTER_ID,
	CKBTC_MINTER_CANISTER_ID,
	CKUSD_LEDGER_CANISTER_ID,
	JUNO_SATELLITE_ID,
	IC_HOST,
	IS_LOCAL_DEV,
	AFRITOKENI_FEE_CANISTER_ID,
	AFRITOKENI_PRINCIPAL_ID,
	PLATFORM_FEE_PERCENTAGE,
	EXCHANGE_SPREAD_PERCENTAGE,
	AGENT_COMMISSION_CUT,
	getCanisterConfig,
	logCanisterConfig
} from './canisters/config';

// ICP Actors (ledger interfaces)
export {
	getCkBTCLedgerActor,
	getCkBTCMinterActor,
	getCkUSDLedgerActor,
	toPrincipal,
	toSubaccount,
	testICPConnection
} from './ledger/actors';

// Type definitions (ckBTC and ckUSD)
export * from './ledger/types';

// Mock utilities (testing only)
export {
	shouldUseMocks,
	MOCK_CKBTC_BALANCE,
	MOCK_CKUSD_BALANCE,
	MOCK_BTC_RATE,
	MOCK_USD_RATE,
	MOCK_PRINCIPAL_ID,
	generateMockTransactionId,
	logMockWarning,
	simulateICPDelay
} from './utils/mock';

// ckBTC Service
export { CkBTCService } from './ledger/ckBTC';

// Export ckUSD service
export { CkUSDService } from './ledger/ckUSD';

// TODO: Export Juno storage when ported
// export * from './storage/juno';
