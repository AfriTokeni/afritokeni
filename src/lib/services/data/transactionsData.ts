/**
 * Transactions Data Service
 *
 * Pure data fetching functions for user transactions.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 */

import { listDocs } from "@junobuild/core";

/**
 * Fetch user transactions
 *
 * @param principalId - User's principal ID (for Juno lookup)
 * @param isDemoMode - Whether to use demo data or real backend
 * @param limit - Maximum number of transactions to fetch
 * @returns Array of transactions
 */
export async function fetchTransactions(
  principalId: string | null,
  isDemoMode: boolean,
  limit: number = 50,
): Promise<any[]> {
  if (isDemoMode) {
    try {
      const response = await fetch("/data/demo/transactions.json");
      if (!response.ok) {
        throw new Error("Failed to fetch demo data");
      }
      const data = await response.json();
      // Handle both array and object with user-transactions key
      const transactions = Array.isArray(data)
        ? data
        : data["user-transactions"] || [];
      return transactions.slice(0, limit);
    } catch (error) {
      console.error("Failed to fetch demo transactions:", error);
      return [];
    }
  }

  // Real mode: query Juno datastore
  if (!principalId) {
    console.warn("No principal ID provided for transactions query");
    return [];
  }

  try {
    // Fetch transactions from Juno collection
    const result = await listDocs({
      collection: "transactions",
      filter: {
        matcher: {
          key: principalId,
        },
      },
    });

    if (!result || !result.items) {
      console.warn("No transactions found in Juno for principal:", principalId);
      return [];
    }

    // Sort by date (most recent first) and limit
    const transactions = result.items
      .map((item: any) => item.data)
      .sort((a: any, b: any) => {
        const dateA = new Date(a.createdAt || a.timestamp || 0).getTime();
        const dateB = new Date(b.createdAt || b.timestamp || 0).getTime();
        return dateB - dateA;
      })
      .slice(0, limit);

    return transactions;
  } catch (error) {
    console.error("Failed to fetch transactions from Juno:", error);
    return [];
  }
}

/**
 * Get transaction type icon/color info
 */
export function getTransactionTypeInfo(type: string): {
  color: string;
  bgColor: string;
  textColor: string;
} {
  switch (type) {
    case "send":
      return {
        color: "text-red-500",
        bgColor: "bg-red-50",
        textColor: "text-red-600",
      };
    case "receive":
      return {
        color: "text-green-500",
        bgColor: "bg-green-50",
        textColor: "text-green-600",
      };
    case "withdraw":
      return {
        color: "text-orange-500",
        bgColor: "bg-orange-50",
        textColor: "text-orange-600",
      };
    case "deposit":
      return {
        color: "text-blue-500",
        bgColor: "bg-blue-50",
        textColor: "text-blue-600",
      };
    default:
      return {
        color: "text-neutral-500",
        bgColor: "bg-neutral-50",
        textColor: "text-neutral-600",
      };
  }
}

/**
 * Check if transaction is outgoing (negative amount display)
 */
export function isOutgoingTransaction(type: string): boolean {
  return type === "send" || type === "withdraw";
}
