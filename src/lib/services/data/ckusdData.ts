/**
 * ckUSD Data Service
 *
 * Provides helper functions for ckUSDC balance queries.
 * Uses cryptoService to query crypto_canister.
 */

import { cryptoService } from "$lib/services";

/**
 * Fetch ckUSDC balance for a user
 * @param userId - User identifier (phone number or user ID)
 * @returns Balance in smallest unit (cents)
 */
export async function fetchCkUSDBalance(
  userId: string | null,
): Promise<number> {
  if (!userId) {
    return 0;
  }
  const balance = await cryptoService.checkBalance(userId, "ckUSDC");
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
