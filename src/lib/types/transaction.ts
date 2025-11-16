/**
 * Transaction Type Definitions
 *
 * Type-safe definitions for transactions, transfers, and financial operations
 * in the AfriTokeni platform.
 */

import type { AfricanCurrency } from "./currency";

/**
 * Transaction type
 */
export type TransactionType =
  | "send"
  | "receive"
  | "withdraw"
  | "deposit"
  | "bitcoin_buy"
  | "bitcoin_sell"
  | "bitcoin_to_fiat"
  | "fiat_to_bitcoin"
  | "bitcoin_send"
  | "bitcoin_receive"
  | "usdc_buy"
  | "usdc_sell"
  | "usdc_send"
  | "usdc_receive";

/**
 * Transaction status
 */
export type TransactionStatus =
  | "pending"
  | "completed"
  | "failed"
  | "cancelled"
  | "confirmed";

/**
 * Transaction metadata (flexible for different transaction types)
 */
export interface TransactionMetadata {
  [key: string]: string | number | boolean | undefined;
}

/**
 * Transaction record
 */
export interface Transaction {
  id: string;
  userId: string;
  type: TransactionType;
  amount: number;
  fee?: number;
  currency: AfricanCurrency | "BTC" | "USDC";
  senderId?: string;
  recipientId?: string;
  recipientPhone?: string;
  recipientName?: string;
  agentId?: string;
  fromUserId?: string;
  toUserId?: string;
  status: TransactionStatus;
  smsCommand?: string;
  description?: string;
  createdAt: Date;
  updatedAt?: Date;
  completedAt?: Date;
  withdrawalCode?: string;
  depositCode?: string;
  bitcoinAddress?: string;
  exchangeRate?: number;
  location?: string;
  metadata?: TransactionMetadata;
}

/**
 * User balance by currency
 */
export interface UserBalance {
  [currency: string]: number;
}

/**
 * Transaction filter options
 */
export interface TransactionFilters {
  userId?: string;
  type?: TransactionType[];
  status?: TransactionStatus[];
  currency?: AfricanCurrency | "BTC" | "USDC";
  dateFrom?: Date;
  dateTo?: Date;
  minAmount?: number;
  maxAmount?: number;
}

/**
 * Paginated transaction results
 */
export interface TransactionPage {
  transactions: Transaction[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
}
