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
export async function fetchCkUSDBalance(userId: string): Promise<number> {
  const balance = await cryptoService.checkBalance(userId, "ckUSDC");
  return Number(balance);
}
