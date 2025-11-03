/**
 * ICP Ledger Types - ckBTC and ckUSD
 * 
 * Type definitions for Chain-Key Bitcoin and Chain-Key USD on ICP.
 * 
 * ⚠️ SECURITY - NON-CUSTODIAL ARCHITECTURE:
 * 
 * These types represent USER-CONTROLLED assets on ICP blockchain:
 * - All balances are stored in user's ICP principal (NOT AfriTokeni's)
 * - All transactions require USER signature via Internet Identity/NFID
 * - AfriTokeni NEVER holds custody of these assets
 * - Transfers are ALWAYS direct: user→agent or agent→user
 * 
 * WHAT THESE TYPES REPRESENT:
 * ✅ User-owned balances on ICP blockchain
 * ✅ Transaction metadata for user's history
 * ✅ Request/response interfaces for user-signed operations
 * 
 * WHAT THESE TYPES NEVER REPRESENT:
 * ❌ AfriTokeni-controlled funds
 * ❌ Platform wallets or pooled funds
 * ❌ Custodial balances
 */

import { Principal } from '@dfinity/principal';

// ============================================================================
// ckBTC TYPES
// ============================================================================

/**
 * ckBTC Configuration
 * 
 * SECURITY: These are PUBLIC canister IDs, not secrets
 */
export interface CkBTCConfig {
	/** Ledger canister ID for ckBTC balance tracking */
	ledgerCanisterId: string;
	/** Minter canister ID for BTC deposits/withdrawals */
	minterCanisterId: string;
	/** KYT (Know Your Transaction) canister ID for compliance */
	kytCanisterId?: string;
	/** Network: 'testnet' or 'mainnet' */
	network: 'testnet' | 'mainnet';
	/** Minimum confirmations for BTC deposits */
	minConfirmations: number;
}

/**
 * ckBTC Transaction Types
 * 
 * SECURITY NOTE:
 * - All transaction types represent USER-initiated operations
 * - AfriTokeni does NOT initiate transactions on behalf of users
 * - Exchanges are direct user→agent or agent→user transfers
 */
export type CkBTCTransactionType =
	| 'deposit' // BTC → ckBTC (Bitcoin to ICP, user receives)
	| 'withdrawal' // ckBTC → BTC (ICP to Bitcoin, user sends)
	| 'transfer' // ckBTC → ckBTC (ICP to ICP, instant!) - legacy
	| 'transfer_out' // ckBTC sent to another user (debit)
	| 'transfer_in' // ckBTC received from another user (credit)
	| 'exchange_buy' // Local currency → ckBTC via agent (user→agent fiat, agent→user ckBTC)
	| 'exchange_sell'; // ckBTC → Local currency via agent (user→agent ckBTC, agent→user fiat)

/**
 * ckBTC Transaction Status
 */
export type CkBTCTransactionStatus =
	| 'pending' // Waiting for Bitcoin confirmations
	| 'confirming' // Bitcoin transaction confirming
	| 'minting' // ckBTC being minted on ICP
	| 'completed' // Transaction successful
	| 'failed' // Transaction failed
	| 'expired'; // Transaction expired

/**
 * ckBTC Transaction Record
 * 
 * SECURITY: This is METADATA only, stored in Juno
 * - Actual funds are on ICP blockchain, not in this record
 * - This is for displaying transaction history to users
 * - Cannot be used to move funds
 */
export interface CkBTCTransaction {
	/** Unique transaction ID */
	id: string;
	/** User ID who initiated transaction */
	userId: string;
	/** Transaction type */
	type: CkBTCTransactionType;
	/** Amount in satoshis */
	amountSatoshis: number;
	/** Amount in BTC (human-readable) */
	amountBTC: number;
	/** Transaction status */
	status: CkBTCTransactionStatus;
	/** Bitcoin transaction ID (for deposits/withdrawals) */
	btcTxId?: string;
	/** ICP block index (for ckBTC transfers) */
	icpBlockIndex?: bigint;
	/** Recipient (for transfers) */
	recipient?: string;
	/** Bitcoin address (for deposits/withdrawals) */
	btcAddress?: string;
	/** Agent ID (for exchange transactions) */
	agentId?: string;
	/** Local currency amount (for exchanges) */
	localCurrencyAmount?: number;
	/** Currency code (for exchanges) */
	currency?: string;
	/** Local currency code (for exchanges) */
	localCurrency?: string;
	/** Exchange rate used (for exchanges) */
	exchangeRate?: number;
	/** Fee in satoshis */
	feeSatoshis: number;
	/** Confirmations (for Bitcoin transactions) */
	confirmations?: number;
	/** Error message if failed */
	error?: string;
	/** Creation timestamp */
	createdAt: Date;
	/** Last update timestamp */
	updatedAt: Date;
	/** Expiration timestamp (for pending transactions) */
	expiresAt?: Date;
}

/**
 * ckBTC Balance Information
 * 
 * SECURITY: This represents balance in USER'S principal
 * - Balance is on ICP blockchain, not in AfriTokeni database
 * - AfriTokeni can READ this (public ledger) but cannot MOVE funds
 */
export interface CkBTCBalance {
	/** Balance in satoshis */
	balanceSatoshis: number;
	/** Balance in BTC (human-readable) */
	balanceBTC: string;
	/** Equivalent in user's preferred local currency */
	localCurrencyEquivalent?: number;
	/** Local currency code */
	localCurrency?: string;
	/** Last update timestamp */
	lastUpdated: Date;
}

/**
 * Bitcoin Deposit Address
 * 
 * SECURITY:
 * - Each user gets UNIQUE Bitcoin address
 * - Deposits go directly to user's ICP principal
 * - AfriTokeni does NOT receive Bitcoin deposits
 */
export interface BitcoinDepositAddress {
	/** Bitcoin address to send BTC to */
	address: string;
	/** User's Principal ID */
	principalId: string;
	/** Subaccount (optional) */
	subaccount?: Uint8Array;
	/** Creation timestamp */
	createdAt: Date;
	/** Expiration timestamp */
	expiresAt: Date;
}

/**
 * ckBTC Deposit Request
 */
export interface CkBTCDepositRequest {
	/** User's Principal ID */
	principalId: string;
	/** Optional subaccount */
	subaccount?: Uint8Array;
}

/**
 * ckBTC Deposit Response
 */
export interface CkBTCDepositResponse {
	/** Success status */
	success: boolean;
	/** Bitcoin address to deposit to */
	depositAddress?: string;
	/** Minimum confirmations required */
	minConfirmations?: number;
	/** Error message if failed */
	error?: string;
}

/**
 * ckBTC Withdrawal Request
 * 
 * SECURITY:
 * - User specifies THEIR OWN Bitcoin address
 * - Requires user signature
 * - AfriTokeni CANNOT withdraw to our address
 */
export interface CkBTCWithdrawalRequest {
	/** Amount in satoshis to withdraw */
	amountSatoshis: number;
	/** Destination Bitcoin address (USER'S address) */
	btcAddress: string;
	/** User's Principal ID */
	principalId: string;
}

/**
 * ckBTC Withdrawal Response
 */
export interface CkBTCWithdrawalResponse {
	/** Success status */
	success: boolean;
	/** Transaction ID */
	transactionId?: string;
	/** Bitcoin transaction ID (once broadcast) */
	btcTxId?: string;
	/** ICP block index */
	icpBlockIndex?: bigint;
	/** Fee in satoshis */
	feeSatoshis?: number;
	/** Error message if failed */
	error?: string;
	/** Estimated confirmation time in minutes */
	estimatedConfirmationTime?: number;
}

/**
 * ckBTC Transfer Request
 * 
 * SECURITY:
 * - Direct transfer from user's principal to recipient's principal
 * - AfriTokeni is NOT involved in the transfer
 * - Requires user signature
 */
export interface CkBTCTransferRequest {
	/** Amount in satoshis to transfer */
	amountSatoshis: number;
	/** Recipient's Principal ID or phone number */
	recipient: string;
	/** Sender's Principal ID */
	senderId: string;
	/** Optional memo */
	memo?: string;
}

/**
 * ckBTC Transfer Response
 */
export interface CkBTCTransferResponse {
	/** Success status */
	success: boolean;
	/** Transaction ID */
	transactionId?: string;
	/** ICP block index */
	icpBlockIndex?: bigint;
	/** Fee in satoshis */
	feeSatoshis?: number;
	/** Error message if failed */
	error?: string;
}

/**
 * ckBTC Exchange Request
 * 
 * SECURITY:
 * - Exchange is direct user→agent or agent→user transfer
 * - AfriTokeni facilitates matching, does NOT hold funds
 * - Actual transfer is on-chain between user and agent principals
 */
export interface CkBTCExchangeRequest {
	/** Amount in satoshis (for sell) or local currency (for buy) */
	amount: number;
	/** Local currency code */
	currency: string;
	/** Exchange type */
	type: 'buy' | 'sell';
	/** User ID */
	userId: string;
	/** Agent ID */
	agentId: string;
	/** User's location for fee calculation */
	userLocation?: {
		latitude: number;
		longitude: number;
	};
}

/**
 * ckBTC Exchange Response
 */
export interface CkBTCExchangeResponse {
	/** Success status */
	success: boolean;
	/** Transaction ID */
	transactionId?: string;
	/** Exchange rate used (1 BTC = X local currency) */
	exchangeRate: number;
	/** Amount in satoshis */
	amountSatoshis: number;
	/** Amount in BTC */
	amountBTC: number;
	/** Amount in local currency */
	localCurrencyAmount: number;
	/** Fee in satoshis */
	feeSatoshis: number;
	/** Fee percentage */
	feePercentage: number;
	/** Agent commission in local currency */
	agentCommission: number;
	/** Error message if failed */
	error?: string;
	/** Exchange code for in-person verification */
	exchangeCode?: string;
}

/**
 * Bitcoin Exchange Rate Information
 */
export interface BitcoinExchangeRate {
	/** Currency code */
	currency: string;
	/** Rate: 1 BTC = X local currency */
	rate: number;
	/** Last update timestamp */
	lastUpdated: Date;
	/** Source of rate data */
	source: string;
}

// ============================================================================
// ckUSD TYPES (renamed from ckUSDC)
// ============================================================================

/**
 * ckUSD Configuration
 * 
 * SECURITY: These are PUBLIC canister IDs, not secrets
 * 
 * NOTE: Renamed from ckUSDC to ckUSD for consistency
 */
export interface CkUSDConfig {
	/** Ledger canister ID for ckUSD balance tracking */
	ledgerCanisterId: string;
	/** Minter canister ID for USDC deposits/withdrawals */
	minterCanisterId?: string;
	/** Network: 'testnet' or 'mainnet' */
	network: 'testnet' | 'mainnet';
	/** Ethereum network for USDC bridge */
	ethereumNetwork?: 'mainnet' | 'sepolia';
}

/**
 * ckUSD Transaction Types
 * 
 * SECURITY NOTE:
 * - All transaction types represent USER-initiated operations
 * - AfriTokeni does NOT initiate transactions on behalf of users
 * - Exchanges are direct user→agent or agent→user transfers
 */
export type CkUSDTransactionType =
	| 'deposit' // USDC → ckUSD (Ethereum to ICP, user receives)
	| 'withdrawal' // ckUSD → USDC (ICP to Ethereum, user sends)
	| 'transfer' // ckUSD → ckUSD (ICP to ICP, instant!) - legacy
	| 'transfer_out' // ckUSD sent to another user (debit)
	| 'transfer_in' // ckUSD received from another user (credit)
	| 'exchange_buy' // Local currency → ckUSD via agent
	| 'exchange_sell'; // ckUSD → Local currency via agent

/**
 * ckUSD Transaction Status
 */
export type CkUSDTransactionStatus =
	| 'pending' // Waiting for Ethereum confirmations
	| 'confirming' // Ethereum transaction confirming
	| 'minting' // ckUSD being minted on ICP
	| 'completed' // Transaction successful
	| 'failed' // Transaction failed
	| 'expired'; // Transaction expired

/**
 * ckUSD Transaction Record
 * 
 * SECURITY: This is METADATA only, stored in Juno
 * - Actual funds are on ICP blockchain, not in this record
 * - This is for displaying transaction history to users
 * - Cannot be used to move funds
 */
export interface CkUSDTransaction {
	/** Unique transaction ID */
	id: string;
	/** User ID who initiated transaction */
	userId: string;
	/** Transaction type */
	type: CkUSDTransactionType;
	/** Amount in smallest units (6 decimals for USD) */
	amountUnits: number;
	/** Amount in USD (human-readable) */
	amountUSD: number;
	/** Transaction status */
	status: CkUSDTransactionStatus;
	/** Ethereum transaction hash (for deposits/withdrawals) */
	ethTxHash?: string;
	/** ICP block index (for ckUSD transfers) */
	icpBlockIndex?: bigint;
	/** Recipient (for transfers) */
	recipient?: string;
	/** Ethereum address (for deposits/withdrawals) */
	ethAddress?: string;
	/** Agent ID (for exchange transactions) */
	agentId?: string;
	/** Local currency amount (for exchanges) */
	localCurrencyAmount?: number;
	/** Currency code (for exchanges) */
	currency?: string;
	/** Local currency code (for exchanges) */
	localCurrency?: string;
	/** Exchange rate used (for exchanges) */
	exchangeRate?: number;
	/** Fee in smallest units */
	feeUnits: number;
	/** Confirmations (for Ethereum transactions) */
	confirmations?: number;
	/** Error message if failed */
	error?: string;
	/** Creation timestamp */
	createdAt: Date;
	/** Last update timestamp */
	updatedAt: Date;
	/** Expiration timestamp (for pending transactions) */
	expiresAt?: Date;
}

/**
 * ckUSD Balance Information
 * 
 * SECURITY: This represents balance in USER'S principal
 * - Balance is on ICP blockchain, not in AfriTokeni database
 * - AfriTokeni can READ this (public ledger) but cannot MOVE funds
 */
export interface CkUSDBalance {
	/** Balance in smallest units (6 decimals) */
	balanceUnits: number;
	/** Balance in USD (human-readable) */
	balanceUSD: string;
	/** Equivalent in user's preferred local currency */
	localCurrencyEquivalent?: number;
	/** Local currency code */
	localCurrency?: string;
	/** Last update timestamp */
	lastUpdated: Date;
}

/**
 * ckUSD Deposit Request
 */
export interface CkUSDDepositRequest {
	/** Amount in USD */
	amountUSD: number;
	/** User's Principal ID */
	principalId: string;
	/** User's Ethereum address */
	ethAddress: string;
}

/**
 * ckUSD Deposit Response
 */
export interface CkUSDDepositResponse {
	/** Success status */
	success: boolean;
	/** Deposit address (ICP minter's Ethereum address) */
	depositAddress?: string;
	/** Minimum confirmations required */
	minConfirmations?: number;
	/** Error message if failed */
	error?: string;
}

/**
 * ckUSD Withdrawal Request
 * 
 * SECURITY:
 * - User specifies THEIR OWN Ethereum address
 * - Requires user signature
 * - AfriTokeni CANNOT withdraw to our address
 */
export interface CkUSDWithdrawalRequest {
	/** Amount in smallest units to withdraw */
	amountUnits: number;
	/** Destination Ethereum address (USER'S address) */
	ethAddress: string;
	/** User's Principal ID */
	principalId: string;
}

/**
 * ckUSD Withdrawal Response
 */
export interface CkUSDWithdrawalResponse {
	/** Success status */
	success: boolean;
	/** Transaction ID */
	transactionId?: string;
	/** Ethereum transaction hash (once broadcast) */
	ethTxHash?: string;
	/** ICP block index */
	icpBlockIndex?: bigint;
	/** Fee in smallest units */
	feeUnits?: number;
	/** Error message if failed */
	error?: string;
	/** Estimated confirmation time in minutes */
	estimatedConfirmationTime?: number;
}

/**
 * ckUSD Transfer Request
 * 
 * SECURITY:
 * - Direct transfer from user's principal to recipient's principal
 * - AfriTokeni is NOT involved in the transfer
 * - Requires user signature
 */
export interface CkUSDTransferRequest {
	/** Amount in smallest units to transfer */
	amountUnits: number;
	/** Recipient's Principal ID or phone number */
	recipient: string;
	/** Sender's Principal ID */
	senderId: string;
	/** Optional memo */
	memo?: string;
}

/**
 * ckUSD Transfer Response
 */
export interface CkUSDTransferResponse {
	/** Success status */
	success: boolean;
	/** Transaction ID */
	transactionId?: string;
	/** ICP block index */
	icpBlockIndex?: bigint;
	/** Fee in smallest units */
	feeUnits?: number;
	/** Error message if failed */
	error?: string;
}

/**
 * ckUSD Exchange Request
 * 
 * SECURITY:
 * - Exchange is direct user→agent or agent→user transfer
 * - AfriTokeni facilitates matching, does NOT hold funds
 * - Actual transfer is on-chain between user and agent principals
 */
export interface CkUSDExchangeRequest {
	/** Amount in smallest units (for sell) or local currency (for buy) */
	amount: number;
	/** Local currency code */
	currency: string;
	/** Exchange type */
	type: 'buy' | 'sell';
	/** User ID */
	userId: string;
	/** Agent ID */
	agentId: string;
	/** User's location for fee calculation */
	userLocation?: {
		latitude: number;
		longitude: number;
	};
}

/**
 * ckUSD Exchange Response
 */
export interface CkUSDExchangeResponse {
	/** Success status */
	success: boolean;
	/** Transaction ID */
	transactionId?: string;
	/** Exchange rate used (1 USD = X local currency) */
	exchangeRate: number;
	/** Amount in smallest units */
	amountUnits: number;
	/** Amount in USD */
	amountUSD: number;
	/** Amount in local currency */
	localCurrencyAmount: number;
	/** Fee in smallest units */
	feeUnits: number;
	/** Fee percentage */
	feePercentage: number;
	/** Agent commission in local currency */
	agentCommission: number;
	/** Error message if failed */
	error?: string;
	/** Exchange code for in-person verification */
	exchangeCode?: string;
}

/**
 * USD Exchange Rate Information
 */
export interface USDExchangeRate {
	/** Currency code */
	currency: string;
	/** Rate: 1 USD = X local currency */
	rate: number;
	/** Last update timestamp */
	lastUpdated: Date;
	/** Source of rate data */
	source: string;
}

// ============================================================================
// CONSTANTS
// ============================================================================

/**
 * ckBTC Constants
 */
export const CKBTC_CONSTANTS = {
	/** Satoshis per Bitcoin */
	SATOSHIS_PER_BTC: 100_000_000,
	/** Minimum transfer amount in satoshis */
	MIN_TRANSFER_SATOSHIS: 1000, // 0.00001 BTC
	/** Minimum withdrawal amount in satoshis */
	MIN_WITHDRAWAL_SATOSHIS: 10000, // 0.0001 BTC
	/** Default Bitcoin confirmations required */
	DEFAULT_CONFIRMATIONS: 6,
	/** Transaction expiration time in milliseconds (24 hours) */
	TX_EXPIRATION_MS: 24 * 60 * 60 * 1000,
	/** ckBTC transfer fee in satoshis (very low!) */
	TRANSFER_FEE_SATOSHIS: 10, // ~$0.01 at $100k BTC
	/** Default exchange fee percentage */
	DEFAULT_EXCHANGE_FEE: 0.02 // 2%
} as const;

/**
 * ckUSD Constants
 */
export const CKUSD_CONSTANTS = {
	/** Smallest units per USD (6 decimals) */
	UNITS_PER_USD: 1_000_000,
	/** Minimum transfer amount in smallest units */
	MIN_TRANSFER_UNITS: 1000, // 0.001 USD
	/** Minimum withdrawal amount in smallest units */
	MIN_WITHDRAWAL_UNITS: 10000, // 0.01 USD
	/** Default Ethereum confirmations required */
	DEFAULT_CONFIRMATIONS: 12,
	/** Transaction expiration time in milliseconds (24 hours) */
	TX_EXPIRATION_MS: 24 * 60 * 60 * 1000,
	/** ckUSD transfer fee in smallest units */
	TRANSFER_FEE_UNITS: 100, // 0.0001 USD
	/** Default exchange fee percentage */
	DEFAULT_EXCHANGE_FEE: 0.02 // 2%
} as const;

/**
 * Testnet Configuration
 */
export const CKBTC_TESTNET_CONFIG: CkBTCConfig = {
	ledgerCanisterId: 'mxzaz-hqaaa-aaaar-qaada-cai',
	minterCanisterId: 'mqygn-kiaaa-aaaar-qaadq-cai',
	network: 'testnet',
	minConfirmations: 6
};

/**
 * Mainnet Configuration
 */
export const CKBTC_MAINNET_CONFIG: CkBTCConfig = {
	ledgerCanisterId: 'mxzaz-hqaaa-aaaar-qaada-cai',
	minterCanisterId: 'mqygn-kiaaa-aaaar-qaadq-cai',
	network: 'mainnet',
	minConfirmations: 12
};

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/**
 * ckBTC Utility Functions
 */
export const CkBTCUtils = {
	/** Convert satoshis to BTC */
	satoshisToBTC(satoshis: number): number {
		return satoshis / CKBTC_CONSTANTS.SATOSHIS_PER_BTC;
	},

	/** Convert BTC to satoshis */
	btcToSatoshis(btc: number): number {
		return Math.round(btc * CKBTC_CONSTANTS.SATOSHIS_PER_BTC);
	},

	/** Format satoshis as BTC string */
	formatBTC(satoshis: number): string {
		const btc = this.satoshisToBTC(satoshis);
		return btc.toFixed(8); // 8 decimal places for BTC
	},

	/** Format satoshis as BTC with symbol */
	formatBTCWithSymbol(satoshis: number): string {
		return `₿${this.formatBTC(satoshis)}`;
	},

	/** Validate Bitcoin address */
	isValidBitcoinAddress(address: string): boolean {
		// Basic validation - in production use proper Bitcoin address validation
		return /^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$|^bc1[a-z0-9]{39,59}$/.test(address);
	},

	/** Validate Principal ID */
	isValidPrincipalId(principalId: string): boolean {
		try {
			Principal.fromText(principalId);
			return true;
		} catch {
			return false;
		}
	}
};

/**
 * ckUSD Utility Functions
 */
export const CkUSDUtils = {
	/** Convert smallest units to USD */
	unitsToUSD(units: number): number {
		return units / CKUSD_CONSTANTS.UNITS_PER_USD;
	},

	/** Convert USD to smallest units */
	usdToUnits(usd: number): number {
		return Math.round(usd * CKUSD_CONSTANTS.UNITS_PER_USD);
	},

	/** Format smallest units as USD string */
	formatUSD(units: number): string {
		const usd = this.unitsToUSD(units);
		return usd.toFixed(6); // 6 decimal places for USD
	},

	/** Format smallest units as USD with symbol */
	formatUSDWithSymbol(units: number): string {
		return `$${this.formatUSD(units)}`;
	},

	/** Validate Ethereum address */
	isValidEthereumAddress(address: string): boolean {
		// Basic validation - in production use proper Ethereum address validation
		return /^0x[a-fA-F0-9]{40}$/.test(address);
	},

	/** Validate Principal ID */
	isValidPrincipalId(principalId: string): boolean {
		try {
			Principal.fromText(principalId);
			return true;
		} catch {
			return false;
		}
	}
};
