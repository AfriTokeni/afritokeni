/**
 * Service Layer Exports
 *
 * Central export point for all AfriTokeni services
 *
 * ARCHITECTURE:
 * - All crypto operations route through crypto_canister (collects 0.5% fee)
 * - All deposit/withdrawal operations route through agent_canister
 * - All fiat transfers route through wallet_canister (collects 0.5% fee)
 * - Reviews stored in data_canister
 * - Authentication and file storage via Juno
 */

// ============================================================================
// NEW DOMAIN CANISTER SERVICES (USE THESE!)
// ============================================================================

/**
 * Crypto Service - All ckBTC/ckUSD operations
 * Routes through crypto_canister to collect platform fees
 */
export { cryptoService, CryptoService } from "./cryptoService";
export type {
  BuyCryptoParams,
  SellCryptoParams,
  SendCryptoParams,
  SwapCryptoParams,
  CreateEscrowParams,
} from "./cryptoService";

/**
 * Wallet Service - All fiat currency operations
 * Routes through wallet_canister to collect platform fees
 */
export { walletService, WalletService } from "./walletService";
export type { TransferFiatParams } from "./walletService";

/**
 * Agent Operations Service - Cash deposits and withdrawals
 * Routes through agent_canister (replaces deposit/withdrawal canisters)
 */
export {
  agentOperationsService,
  AgentOperationsService,
} from "./agentOperationsService";
export type {
  CreateDepositParams,
  ConfirmDepositParams,
  CreateWithdrawalParams,
  ConfirmWithdrawalParams,
} from "./agentOperationsService";

/**
 * User Service - User metadata management (Juno)
 * For authentication and PIN operations, use userCanisterService
 */
export { UserService } from "./userService";

// ============================================================================
// LOW-LEVEL CANISTER SERVICE WRAPPERS (Advanced use only)
// ============================================================================

/**
 * Direct canister service wrappers
 * Only use these if you need fine-grained control
 * Most users should use the high-level services above
 */
export { cryptoCanisterService } from "./icp/canisters/cryptoCanisterService";
export { walletCanisterService } from "./icp/canisters/walletCanisterService";
export { agentCanisterService } from "./icp/canisters/agentCanisterService";
export { userCanisterService } from "./icp/canisters/userCanisterService";
export { dataCanisterService } from "./icp/canisters/dataCanisterService";

// ============================================================================
// JUNO SERVICES (Metadata, Auth, Storage)
// ============================================================================

/**
 * Juno services for metadata, authentication, and file storage
 * Import individual functions from juno/* as needed
 */
// Juno services export individual functions, not default exports
// Import directly from juno/agentService, juno/kycService, juno/userService, etc.

// ============================================================================
// UTILITY SERVICES
// ============================================================================

// Exchange rate service moved to frontend utils
// Use crypto_canister for actual exchange operations
