/**
 * ckUSD (Chain-Key USDC) Types for AfriTokeni
 *
 * ckUSD is an ICP-native representation of USDC stablecoin
 * Provides price stability for African financial transactions
 */

// import { Principal } from '@dfinity/principal'; // Use when connecting to ICP mainnet canisters

/**
 * ckUSD configuration for connecting to ICP canisters
 */
export interface CkUSDConfig {
  /** Ledger canister ID for ckUSD balance tracking */
  ledgerCanisterId: string;
  /** Minter canister ID for deposits/withdrawals */
  minterCanisterId: string;
  /** Ethereum helper contract address for deposits */
  helperContractAddress: string;
  /** Sepolia USDC token contract address */
  sepoliaUSDCAddress: string;
  /** Network: 'testnet' (Sepolia) or 'mainnet' */
  network: "testnet" | "mainnet";
}

/**
 * ckUSD transaction types
 */
export type CkUSDTransactionType =
  | "deposit" // USDC → ckUSD (Ethereum to ICP)
  | "withdrawal" // ckUSD → USDC (ICP to Ethereum)
  | "transfer" // ckUSD → ckUSD (ICP to ICP)
  | "exchange_buy" // Local currency → ckUSD via agent
  | "exchange_sell"; // ckUSD → Local currency via agent

/**
 * ckUSD transaction status
 */
export type CkUSDTransactionStatus =
  | "pending" // Waiting for confirmation
  | "confirming" // Ethereum transaction confirming
  | "minting" // ckUSD being minted on ICP
  | "completed" // Transaction successful
  | "failed" // Transaction failed
  | "expired"; // Transaction expired (24h timeout)

/**
 * ckUSD transaction record
 */
export interface CkUSDTransaction {
  /** Unique transaction ID */
  id: string;
  /** User ID who initiated transaction */
  userId: string;
  /** Transaction type */
  type: CkUSDTransactionType;
  /** Amount in ckUSD (smallest unit: 1e-6) */
  amount: number;
  /** Transaction status */
  status: CkUSDTransactionStatus;
  /** Ethereum transaction hash (for deposits/withdrawals) */
  ethTxHash?: string;
  /** ICP transaction hash */
  icpTxHash?: string;
  /** Recipient (for transfers) */
  recipient?: string;
  /** Agent ID (for exchange transactions) */
  agentId?: string;
  /** Local currency amount (for exchanges) */
  localCurrencyAmount?: number;
  /** Local currency code (for exchanges) */
  localCurrency?: string;
  /** Exchange rate used (for exchanges) */
  exchangeRate?: number;
  /** Fee charged */
  fee: number;
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
 * ckUSD balance information
 */
export interface CkUSDBalance {
  /** Balance in USDC (e.g., "100.50") */
  balanceUSDC: string;
  /** Equivalent in user's preferred local currency */
  localCurrencyEquivalent?: number;
  /** Local currency code */
  localCurrency?: string;
  /** Last update timestamp */
  lastUpdated: Date;
}

/**
 * ckUSD deposit request
 */
export interface CkUSDDepositRequest {
  /** Amount in USDC to deposit */
  amount: number;
  /** User's Principal ID on ICP */
  principalId: string;
  /** User's Ethereum wallet address */
  ethereumAddress: string;
}

/**
 * ckUSD deposit response
 */
export interface CkUSDDepositResponse {
  /** Success status */
  success: boolean;
  /** Transaction ID */
  transactionId?: string;
  /** Byte32 deposit address for ICP */
  depositAddress?: string;
  /** Ethereum transaction hash */
  ethTxHash?: string;
  /** Error message if failed */
  error?: string;
  /** Estimated confirmation time in minutes */
  estimatedConfirmationTime?: number;
}

/**
 * ckUSD withdrawal request
 */
export interface CkUSDWithdrawalRequest {
  /** Amount in ckUSD to withdraw */
  amount: number;
  /** Destination Ethereum address */
  ethereumAddress: string;
  /** User's Principal ID on ICP */
  principalId: string;
}

/**
 * ckUSD withdrawal response
 */
export interface CkUSDWithdrawalResponse {
  /** Success status */
  success: boolean;
  /** Transaction ID */
  transactionId?: string;
  /** ICP transaction hash */
  icpTxHash?: string;
  /** Ethereum transaction hash (once processed) */
  ethTxHash?: string;
  /** Error message if failed */
  error?: string;
  /** Estimated processing time in minutes */
  estimatedProcessingTime?: number;
}

/**
 * ckUSD transfer request (ICP to ICP)
 */
export interface CkUSDTransferRequest {
  /** Amount in ckUSD to transfer */
  amount: number;
  /** Recipient's Principal ID or phone number */
  recipient: string;
  /** Sender's Principal ID */
  senderId: string;
  /** Optional memo/note */
  memo?: string;
}

/**
 * ckUSD transfer response
 */
export interface CkUSDTransferResponse {
  /** Success status */
  success: boolean;
  /** Transaction ID */
  transactionId?: string;
  /** ICP transaction hash */
  icpTxHash?: string;
  /** Error message if failed */
  error?: string;
  /** Fee charged */
  fee: number;
}

/**
 * ckUSD exchange request (via agent)
 */
export interface CkUSDExchangeRequest {
  /** Amount in ckUSD (for sell) or local currency (for buy) */
  amount: number;
  /** Local currency code */
  currency: string;
  /** Exchange type */
  type: "buy" | "sell";
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
 * ckUSD exchange response
 */
export interface CkUSDExchangeResponse {
  /** Success status */
  success: boolean;
  /** Transaction ID */
  transactionId?: string;
  /** Exchange rate used */
  exchangeRate: number;
  /** Amount in ckUSD */
  ckusdcAmount: number;
  /** Amount in local currency */
  localCurrencyAmount: number;
  /** Fee charged */
  fee: number;
  /** Fee percentage */
  feePercentage: number;
  /** Agent commission */
  agentCommission: number;
  /** Error message if failed */
  error?: string;
  /** Exchange code for in-person verification */
  exchangeCode?: string;
}

/**
 * ckUSD exchange rate information
 */
export interface CkUSDExchangeRate {
  /** Currency code */
  currency: string;
  /** Rate: 1 ckUSD = X local currency */
  rate: number;
  /** Last update timestamp */
  lastUpdated: Date;
  /** Source of rate data */
  source: string;
}

/**
 * ERC20 ABI for USDC approve function
 */
export const ERC20_APPROVE_ABI = [
  {
    constant: false,
    inputs: [
      { name: "_spender", type: "address" },
      { name: "_value", type: "uint256" },
    ],
    name: "approve",
    outputs: [{ name: "", type: "bool" }],
    type: "function",
  },
];

/**
 * Helper contract ABI for USDC deposits
 */
export const HELPER_CONTRACT_ABI = [
  {
    constant: false,
    inputs: [
      { name: "_token", type: "address" },
      { name: "_amount", type: "uint256" },
      { name: "_depositAddress", type: "bytes32" },
    ],
    name: "deposit",
    outputs: [{ name: "", type: "bool" }],
    type: "function",
  },
];

/**
 * ckUSD constants
 */
export const CKUSDC_CONSTANTS = {
  /** Decimals for ckUSD (same as USDC) */
  DECIMALS: 6,
  /** Minimum deposit amount in USDC */
  MIN_DEPOSIT: 1,
  /** Maximum deposit amount in USDC */
  MAX_DEPOSIT: 10000,
  /** Minimum transfer amount in ckUSD */
  MIN_TRANSFER: 0.01,
  /** Transaction expiration time in milliseconds (24 hours) */
  TX_EXPIRATION_MS: 24 * 60 * 60 * 1000,
  /** Ethereum confirmation blocks required */
  ETH_CONFIRMATIONS: 12,
  /** Default fee percentage for exchanges */
  DEFAULT_EXCHANGE_FEE: 0.02, // 2%
} as const;

/**
 * Sepolia testnet configuration
 */
export const SEPOLIA_CONFIG: CkUSDConfig = {
  ledgerCanisterId: "", // To be filled after deployment
  minterCanisterId: "", // To be filled after deployment
  helperContractAddress: "", // To be filled from tutorial
  sepoliaUSDCAddress: "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238",
  network: "testnet",
};

/**
 * Mainnet configuration (for future use)
 */
export const MAINNET_CONFIG: CkUSDConfig = {
  ledgerCanisterId: "", // To be filled after mainnet deployment
  minterCanisterId: "", // To be filled after mainnet deployment
  helperContractAddress: "", // To be filled after mainnet deployment
  sepoliaUSDCAddress: "", // Mainnet USDC address
  network: "mainnet",
};
