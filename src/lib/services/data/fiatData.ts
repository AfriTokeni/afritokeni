/**
 * Fiat Data Service
 *
 * Pure data fetching functions for fiat (local African currency) balances.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 */

import { getDoc } from "@junobuild/core";

/**
 * Fetch fiat balance
 *
 * @param principalId - User's principal ID (for Juno lookup)
 * @param isDemoMode - Whether to use demo data or real backend
 * @returns Balance object with amount and currency
 */
export async function fetchFiatBalance(
  principalId: string | null,
  isDemoMode: boolean,
): Promise<{ amount: number; currency: string }> {
  if (isDemoMode) {
    try {
      const response = await fetch("/data/demo/user.json");
      if (!response.ok) {
        throw new Error("Failed to fetch demo data");
      }
      const data = await response.json();
      return {
        amount: data.balance || 0,
        currency: data.preferredCurrency || "UGX",
      };
    } catch (error) {
      console.error("Failed to fetch demo fiat balance:", error);
      return { amount: 0, currency: "UGX" };
    }
  }

  // Real mode: query Juno datastore
  if (!principalId) {
    console.warn("No principal ID provided for fiat balance query");
    return { amount: 0, currency: "UGX" };
  }

  try {
    const result = await getDoc({
      collection: "users",
      key: principalId,
    });

    if (!result || !result.data) {
      console.warn("No user data found in Juno for principal:", principalId);
      return { amount: 0, currency: "UGX" };
    }

    const userData = result.data as any;
    return {
      amount: userData.balance || 0,
      currency: userData.preferredCurrency || "UGX",
    };
  } catch (error) {
    console.error("Failed to fetch fiat balance from Juno:", error);
    return { amount: 0, currency: "UGX" };
  }
}

/**
 * Format currency amount with proper locale
 *
 * @param amount - Amount to format
 * @param currency - Currency code (NGN, KES, UGX, etc.)
 * @returns Formatted string
 */
export function formatCurrency(amount: number, currency: string): string {
  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
}

/**
 * Get currency symbol
 */
export function getCurrencySymbol(currency: string): string {
  const symbols: Record<string, string> = {
    UGX: "UGX",
    NGN: "₦",
    KES: "KSh",
    GHS: "₵",
    ZAR: "R",
    TZS: "TSh",
    RWF: "FRw",
    // Add more as needed
  };
  return symbols[currency] || currency;
}
