/**
 * Transaction Service
 * Handles all transaction-related operations with Juno DB
 *
 * ⚠️ DEPRECATED: This service uses Juno datastore for transaction data.
 *
 * MIGRATION STATUS: Transactions should be stored in domain canisters, not Juno.
 * - Fiat transactions → wallet_canister
 * - Crypto transactions → crypto_canister
 * - Agent operations → agent_canister
 * - Transaction history → data_canister.get_user_transactions()
 *
 * This file is kept for backward compatibility with admin dashboard.
 * DO NOT use for new features. Use domain canister services instead.
 */

import { listDocs, type Doc } from "@junobuild/core";
import type {
  Transaction,
  TransactionStats,
  TransactionStatus,
  TransactionType,
} from "$lib/types/admin";

const TRANSACTIONS_COLLECTION = "transactions";

/**
 * Juno document data structure for Transaction
 */
export interface TransactionDocData {
  type: TransactionType;
  userId: string;
  userName: string;
  agentId?: string;
  agentName?: string;
  amount: number;
  currency: string;
  fee: number;
  status: TransactionStatus;
  createdAt: string;
  completedAt?: string;
  failureReason?: string;
  // Additional fields for exchange transactions
  fromCurrency?: string;
  toCurrency?: string;
  exchangeRate?: number;
  blockchainTxId?: string;
  confirmations?: number;
}

/**
 * Convert Juno document to Transaction type
 */
function docToTransaction(doc: Doc<TransactionDocData>): Transaction {
  return {
    id: doc.key,
    type: doc.data.type,
    userId: doc.data.userId,
    userName: doc.data.userName,
    agentId: doc.data.agentId,
    agentName: doc.data.agentName,
    amount: doc.data.amount,
    currency: doc.data.currency,
    fee: doc.data.fee,
    status: doc.data.status,
    createdAt: doc.data.createdAt,
    completedAt: doc.data.completedAt,
    failureReason: doc.data.failureReason,
  };
}

/**
 * List transactions with optional filters
 */
export async function listTransactions(options?: {
  status?: TransactionStatus;
  type?: TransactionType;
  limit?: number;
}): Promise<Transaction[]> {
  try {
    const { items } = await listDocs<TransactionDocData>({
      collection: TRANSACTIONS_COLLECTION,
      filter: {
        order: {
          desc: true,
          field: "created_at",
        },
        ...(options?.limit && { paginate: { limit: options.limit } }),
      },
    });

    let transactions = items.map(docToTransaction);

    // Apply filters
    if (options?.status) {
      transactions = transactions.filter((t) => t.status === options.status);
    }
    if (options?.type) {
      transactions = transactions.filter((t) => t.type === options.type);
    }

    return transactions;
  } catch (error) {
    console.error("Error listing transactions:", error);
    throw error;
  }
}

/**
 * Get transaction statistics
 */
export async function getTransactionStats(): Promise<TransactionStats> {
  try {
    const { items } = await listDocs<TransactionDocData>({
      collection: TRANSACTIONS_COLLECTION,
      filter: {
        order: {
          desc: true,
          field: "created_at",
        },
      },
    });

    const transactions = items.map(docToTransaction);
    const now = new Date();
    const twentyFourHoursAgo = new Date(now.getTime() - 24 * 60 * 60 * 1000);

    // Calculate stats
    const stats: TransactionStats = {
      total: transactions.length,
      pending: transactions.filter((t) => t.status === "pending").length,
      completed: transactions.filter((t) => t.status === "completed").length,
      failed: transactions.filter((t) => t.status === "failed").length,
      volume24h: 0,
      fees24h: 0,
    };

    // Calculate 24h volume and fees
    transactions.forEach((txn) => {
      const txnDate = new Date(txn.createdAt);
      if (txnDate >= twentyFourHoursAgo) {
        if (txn.status === "completed") {
          stats.volume24h += txn.amount;
          stats.fees24h += txn.fee;
        }
      }
    });

    return stats;
  } catch (error) {
    console.error("Error getting transaction stats:", error);
    throw error;
  }
}

/**
 * Get transactions for chart data grouped by date
 */
export async function getTransactionChartData(days: number = 30): Promise<{
  dates: string[];
  volumes: number[];
  counts: number[];
}> {
  try {
    const { items } = await listDocs<TransactionDocData>({
      collection: TRANSACTIONS_COLLECTION,
      filter: {
        order: {
          desc: true,
          field: "created_at",
        },
      },
    });

    const transactions = items.map(docToTransaction);
    const dateMap = new Map<string, { volume: number; count: number }>();

    // Initialize all dates with 0
    const today = new Date();
    for (let i = days - 1; i >= 0; i--) {
      const date = new Date(today);
      date.setDate(date.getDate() - i);
      const dateStr = date.toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
      });
      dateMap.set(dateStr, { volume: 0, count: 0 });
    }

    // Count transactions by date
    transactions.forEach((txn) => {
      if (txn.status === "completed") {
        const date = new Date(txn.createdAt);
        const dateStr = date.toLocaleDateString("en-US", {
          month: "short",
          day: "numeric",
        });
        const stats = dateMap.get(dateStr);
        if (stats) {
          stats.volume += txn.amount;
          stats.count++;
        }
      }
    });

    const dates = Array.from(dateMap.keys());
    const volumes = Array.from(dateMap.values()).map((v) => v.volume);
    const counts = Array.from(dateMap.values()).map((v) => v.count);

    return { dates, volumes, counts };
  } catch (error) {
    console.error("Error getting transaction chart data:", error);
    throw error;
  }
}
