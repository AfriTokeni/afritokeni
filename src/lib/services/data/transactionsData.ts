/**
 * Transaction Data Service (Demo Mode Only)
 *
 * Loads transaction demo data and provides UI formatting helpers.
 * NO BUSINESS LOGIC - For UI display only.
 *
 * For real operations, use:
 * - walletService (fiat transfers)
 * - cryptoService (crypto transfers)
 * - agentOperationsService (deposits/withdrawals)
 */

export interface Transaction {
  id: string;
  type: string;
  amount: number;
  currency: string;
  status: string;
  timestamp: number;
  fromUser?: string;
  toUser?: string;
  description?: string;
}

/**
 * Fetch transactions from demo data
 * @param userId - User identifier (optional, for filtering)
 * @param isDemoMode - Whether to use demo data
 * @param maxTransactions - Maximum number of transactions to return
 * @returns Array of transactions
 */
export async function fetchTransactions(
  userId?: string | null,
  _isDemoMode: boolean = true,
  maxTransactions?: number,
): Promise<Transaction[]> {
  try {
    const response = await fetch("/data/demo/transactions.json");
    if (!response.ok) {
      throw new Error(
        `Failed to fetch demo transactions: ${response.statusText}`,
      );
    }
    const data = await response.json();
    // Extract array from "user-transactions" key
    const transactions = Array.isArray(data) ? data : data["user-transactions"] || [];
    return maxTransactions
      ? transactions.slice(0, maxTransactions)
      : transactions;
  } catch (error) {
    console.error("Error loading demo transactions:", error);
    return [];
  }
}

/**
 * UI helper: Get icon name for transaction type
 */
export function getTransactionIcon(type: string): string {
  const iconMap: Record<string, string> = {
    deposit: "arrow-down-circle",
    withdrawal: "arrow-up-circle",
    transfer: "arrow-right-left",
    buy: "shopping-cart",
    sell: "banknote",
    send: "send",
    receive: "inbox",
  };
  return iconMap[type.toLowerCase()] || "circle";
}

/**
 * UI helper: Get color for transaction type
 */
export function getTransactionColor(type: string): string {
  const colorMap: Record<string, string> = {
    deposit: "text-green-600",
    withdrawal: "text-red-600",
    transfer: "text-blue-600",
    buy: "text-purple-600",
    sell: "text-orange-600",
    send: "text-blue-600",
    receive: "text-green-600",
  };
  return colorMap[type.toLowerCase()] || "text-gray-600";
}

/**
 * UI helper: Get transaction type info
 */
export function getTransactionTypeInfo(type: string): {
  label: string;
  icon: string;
  color: string;
  bgColor: string;
  textColor: string;
} {
  const colorMap: Record<string, { bg: string; text: string }> = {
    deposit: { bg: "bg-green-100", text: "text-green-700" },
    withdrawal: { bg: "bg-red-100", text: "text-red-700" },
    transfer: { bg: "bg-blue-100", text: "text-blue-700" },
    buy: { bg: "bg-purple-100", text: "text-purple-700" },
    sell: { bg: "bg-orange-100", text: "text-orange-700" },
    send: { bg: "bg-blue-100", text: "text-blue-700" },
    receive: { bg: "bg-green-100", text: "text-green-700" },
  };

  const colors = colorMap[type.toLowerCase()] || {
    bg: "bg-gray-100",
    text: "text-gray-700",
  };

  return {
    label: type.charAt(0).toUpperCase() + type.slice(1),
    icon: getTransactionIcon(type),
    color: getTransactionColor(type),
    bgColor: colors.bg,
    textColor: colors.text,
  };
}

/**
 * UI helper: Check if transaction is outgoing (based on type only)
 * @param transactionType - Transaction type string
 * @returns True if transaction is outgoing (withdrawal, send, transfer)
 */
export function isOutgoingTransaction(transactionType: string): boolean {
  const outgoingTypes = ["withdrawal", "send", "transfer"];
  return outgoingTypes.includes(transactionType.toLowerCase());
}

/**
 * UI helper: Format transaction description
 */
export function formatTransactionDescription(transaction: Transaction): string {
  if (transaction.description) {
    return transaction.description;
  }

  const typeDescriptions: Record<string, string> = {
    deposit: "Cash deposit",
    withdrawal: "Cash withdrawal",
    transfer: "P2P transfer",
    buy: "Buy cryptocurrency",
    sell: "Sell cryptocurrency",
    send: "Send cryptocurrency",
    receive: "Receive cryptocurrency",
  };

  return typeDescriptions[transaction.type.toLowerCase()] || transaction.type;
}
