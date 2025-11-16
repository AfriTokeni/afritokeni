/**
 * ckUSD Data Service
 *
 * Provides helper functions for ckUSD balance queries.
 * Uses cryptoService to query crypto_canister.
 */

import { cryptoService } from "$lib/services";

/**
 * Fetch ckUSD balance for a user
 * @param userId - User identifier (phone number or user ID)
 * @param isDemoMode - Whether to return demo data (default: true)
 * @returns Balance in smallest unit (1/1,000,000 USDC)
 */
export async function fetchCkUSDBalance(
  userId: string | null,
  isDemoMode: boolean = true,
): Promise<number> {
  if (!userId) {
    return 0;
  }

  // Return demo balance for development
  if (isDemoMode) {
    // Demo: 100 USDC = 100,000,000 smallest units (6 decimals)
    return 100000000;
  }

  const balance = await cryptoService.checkBalance(userId, "ckUSD");
  return Number(balance);
}

/**
 * Format USD amount with proper currency symbol and decimals
 * @param amount - Amount in USD (dollars, not cents)
 * @returns Formatted string like "$123.45"
 */
export function formatUSD(amount: number): string {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
}
