import { listDocs } from "@junobuild/core";

const TRANSACTIONS_COLLECTION = "transactions";

export interface RevenueStats {
  totalRevenue: number;
  depositCommissions: number;
  withdrawalFees: number;
  exchangeSpread: number;
  totalRevenueChange?: number;
  depositCommissionsChange?: number;
  withdrawalFeesChange?: number;
  exchangeSpreadChange?: number;
}

export interface RevenueTransaction {
  id: string;
  user: string;
  type: string;
  amount: number;
  fee: number;
  time: string;
  status: string;
  createdAt: string;
}

/**
 * Get revenue statistics from transactions
 */
export async function getRevenueStats(): Promise<RevenueStats> {
  try {
    const { items } = await listDocs({
      collection: TRANSACTIONS_COLLECTION,
      filter: {},
    });

    const transactions = items.map((item) => item.data);

    // Calculate current period stats
    const now = new Date();
    const lastMonth = new Date(
      now.getFullYear(),
      now.getMonth() - 1,
      now.getDate(),
    );

    // Current period
    const currentDeposits = transactions
      .filter((tx: any) => tx.type === "deposit" && tx.status === "completed")
      .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);

    const currentWithdrawals = transactions
      .filter(
        (tx: any) => tx.type === "withdrawal" && tx.status === "completed",
      )
      .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);

    const currentExchange = transactions
      .filter((tx: any) => tx.type === "exchange" && tx.status === "completed")
      .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);

    const currentTotal = currentDeposits + currentWithdrawals + currentExchange;

    // Last month period
    const lastMonthTransactions = transactions.filter(
      (tx: any) => new Date(tx.createdAt) <= lastMonth,
    );

    const lastMonthDeposits = lastMonthTransactions
      .filter((tx: any) => tx.type === "deposit" && tx.status === "completed")
      .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);

    const lastMonthWithdrawals = lastMonthTransactions
      .filter(
        (tx: any) => tx.type === "withdrawal" && tx.status === "completed",
      )
      .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);

    const lastMonthExchange = lastMonthTransactions
      .filter((tx: any) => tx.type === "exchange" && tx.status === "completed")
      .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);

    const lastMonthTotal =
      lastMonthDeposits + lastMonthWithdrawals + lastMonthExchange;

    // Calculate percentage changes
    const calculateChange = (current: number, previous: number) => {
      if (previous === 0) return 0;
      return Math.round(((current - previous) / previous) * 100);
    };

    return {
      totalRevenue: currentTotal,
      depositCommissions: currentDeposits,
      withdrawalFees: currentWithdrawals,
      exchangeSpread: currentExchange,
      totalRevenueChange: calculateChange(currentTotal, lastMonthTotal),
      depositCommissionsChange: calculateChange(
        currentDeposits,
        lastMonthDeposits,
      ),
      withdrawalFeesChange: calculateChange(
        currentWithdrawals,
        lastMonthWithdrawals,
      ),
      exchangeSpreadChange: calculateChange(currentExchange, lastMonthExchange),
    };
  } catch (error) {
    console.error("Error getting revenue stats:", error);
    return {
      totalRevenue: 0,
      depositCommissions: 0,
      withdrawalFees: 0,
      exchangeSpread: 0,
      totalRevenueChange: 0,
      depositCommissionsChange: 0,
      withdrawalFeesChange: 0,
      exchangeSpreadChange: 0,
    };
  }
}

/**
 * Get revenue chart data for a specific period
 */
export async function getRevenueChartData(days: number = 90): Promise<{
  categories: string[];
  totalRevenue: number[];
  deposits: number[];
  withdrawals: number[];
}> {
  try {
    const { items } = await listDocs({
      collection: TRANSACTIONS_COLLECTION,
      filter: {},
    });

    const transactions = items.map((item) => item.data);

    // Group by time periods
    const periods = days === 30 ? 5 : days === 90 ? 4 : 6;
    const daysPerPeriod = Math.floor(days / periods);

    const dateRange = Array.from({ length: periods }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (days - (i + 1) * daysPerPeriod));
      return date;
    });

    const categories = dateRange.map((date) => {
      if (days === 30) {
        return date.toLocaleDateString("en-US", {
          month: "short",
          day: "numeric",
        });
      } else {
        return date.toLocaleDateString("en-US", { month: "short" });
      }
    });

    // Calculate cumulative revenue for each period
    const totalRevenue = dateRange.map((date) => {
      return transactions
        .filter((tx: any) => {
          const txDate = new Date(tx.createdAt);
          return txDate <= date && tx.status === "completed";
        })
        .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);
    });

    const deposits = dateRange.map((date) => {
      return transactions
        .filter((tx: any) => {
          const txDate = new Date(tx.createdAt);
          return (
            txDate <= date && tx.type === "deposit" && tx.status === "completed"
          );
        })
        .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);
    });

    const withdrawals = dateRange.map((date) => {
      return transactions
        .filter((tx: any) => {
          const txDate = new Date(tx.createdAt);
          return (
            txDate <= date &&
            tx.type === "withdrawal" &&
            tx.status === "completed"
          );
        })
        .reduce((sum: number, tx: any) => sum + (tx.fee ?? 0), 0);
    });

    return { categories, totalRevenue, deposits, withdrawals };
  } catch (error) {
    console.error("Error getting revenue chart data:", error);
    return {
      categories: [],
      totalRevenue: [],
      deposits: [],
      withdrawals: [],
    };
  }
}

/**
 * Get revenue-generating transactions with search and sort
 */
export async function getRevenueTransactions(options?: {
  searchQuery?: string;
  sortBy?: "type" | "amount" | "fee" | "time";
  sortOrder?: "asc" | "desc";
  limit?: number;
}): Promise<RevenueTransaction[]> {
  try {
    const { items } = await listDocs({
      collection: TRANSACTIONS_COLLECTION,
      filter: {},
    });

    let transactions = items
      .map((item) => {
        const data = item.data as any;
        return {
          id: item.key,
          user: data.userName ?? data.userId ?? "Unknown",
          type: data.type ?? "unknown",
          amount: data.amount ?? 0,
          fee: data.fee ?? 0,
          time: data.createdAt ?? new Date().toISOString(),
          status: data.status ?? "pending",
          createdAt: data.createdAt ?? new Date().toISOString(),
        };
      })
      .filter((tx) => tx.status === "completed" && tx.fee > 0);

    // Search filter
    if (options?.searchQuery) {
      const query = options.searchQuery.toLowerCase();
      transactions = transactions.filter(
        (tx) =>
          tx.id.toLowerCase().includes(query) ||
          tx.user.toLowerCase().includes(query) ||
          tx.type.toLowerCase().includes(query),
      );
    }

    // Sort
    if (options?.sortBy) {
      transactions.sort((a, b) => {
        let aVal, bVal;

        switch (options.sortBy) {
          case "type":
            aVal = a.type;
            bVal = b.type;
            break;
          case "amount":
            aVal = a.amount;
            bVal = b.amount;
            break;
          case "fee":
            aVal = a.fee;
            bVal = b.fee;
            break;
          case "time":
            aVal = new Date(a.createdAt).getTime();
            bVal = new Date(b.createdAt).getTime();
            break;
          default:
            return 0;
        }

        if (aVal < bVal) return options.sortOrder === "asc" ? -1 : 1;
        if (aVal > bVal) return options.sortOrder === "asc" ? 1 : -1;
        return 0;
      });
    }

    // Limit
    if (options?.limit) {
      transactions = transactions.slice(0, options.limit);
    }

    return transactions;
  } catch (error) {
    console.error("Error getting revenue transactions:", error);
    return [];
  }
}
