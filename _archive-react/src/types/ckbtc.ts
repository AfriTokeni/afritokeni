/**
 * ckBTC (Chain-Key Bitcoin) Types for AfriTokeni
 * 
 * ckBTC is ICP's native Bitcoin representation with Lightning-like speed
 * Provides instant Bitcoin transfers without Lightning Network complexity
 */

import { Principal } from '@dfinity/principal';

/**
 * ckBTC configuration for ICP canisters
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
 * ckBTC transaction types
 */
export type CkBTCTransactionType =
  | 'deposit'      // BTC → ckBTC (Bitcoin to ICP)
  | 'withdrawal'   // ckBTC → BTC (ICP to Bitcoin)
  | 'transfer'     // ckBTC → ckBTC (ICP to ICP, instant!) - legacy
  | 'transfer_out' // ckBTC sent to another user (debit)
  | 'transfer_in'  // ckBTC received from another user (credit)
  | 'exchange_buy' // Local currency → ckBTC via agent
  | 'exchange_sell'; // ckBTC → Local currency via agent

/**
 * ckBTC transaction status
 */
export type CkBTCTransactionStatus =
  | 'pending'      // Waiting for Bitcoin confirmations
  | 'confirming'   // Bitcoin transaction confirming
  | 'minting'      // ckBTC being minted on ICP
  | 'completed'    // Transaction successful
  | 'failed'       // Transaction failed
  | 'expired';     // Transaction expired

/**
 * ckBTC transaction record
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
 * ckBTC balance information
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
 * Bitcoin deposit address (for BTC → ckBTC)
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
 * ckBTC deposit request
 */
export interface CkBTCDepositRequest {
  /** User's Principal ID */
  principalId: string;
  /** Optional subaccount */
  subaccount?: Uint8Array;
}

/**
 * ckBTC deposit response
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
 * ckBTC withdrawal request (ckBTC → BTC)
 */
export interface CkBTCWithdrawalRequest {
  /** Amount in satoshis to withdraw */
  amountSatoshis: number;
  /** Destination Bitcoin address */
  btcAddress: string;
  /** User's Principal ID */
  principalId: string;
}

/**
 * ckBTC withdrawal response
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
 * ckBTC transfer request (instant ICP-to-ICP)
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
 * ckBTC transfer response
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
 * ckBTC exchange request (via agent)
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
 * ckBTC exchange response
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
 * Bitcoin exchange rate information
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

/**
 * ckBTC constants
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
  DEFAULT_EXCHANGE_FEE: 0.02, // 2%
} as const;

/**
 * Testnet configuration (for development)
 */
export const CKBTC_TESTNET_CONFIG: CkBTCConfig = {
  // These will be filled after canister deployment
  ledgerCanisterId: 'mxzaz-hqaaa-aaaar-qaada-cai', // ckBTC testnet ledger
  minterCanisterId: 'mqygn-kiaaa-aaaar-qaadq-cai', // ckBTC testnet minter
  network: 'testnet',
  minConfirmations: 6,
};

/**
 * Mainnet configuration (for production)
 */
export const CKBTC_MAINNET_CONFIG: CkBTCConfig = {
  ledgerCanisterId: 'mxzaz-hqaaa-aaaar-qaada-cai', // ckBTC mainnet ledger
  minterCanisterId: 'mqygn-kiaaa-aaaar-qaadq-cai', // ckBTC mainnet minter
  network: 'mainnet',
  minConfirmations: 12,
};

/**
 * Utility functions
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
  },
};
