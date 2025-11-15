/**
 * ckBTC Data Service
 *
 * Provides helper functions for ckBTC balance queries.
 * Uses cryptoService to query crypto_canister.
 */

import { cryptoService } from "$lib/services";

/**
 * Fetch ckBTC balance for a user
 * @param userId - User identifier (phone number or user ID)
 * @returns Balance in satoshis
 */
export async function fetchCkBTCBalance(userId: string): Promise<number> {
  const balance = await cryptoService.checkBalance(userId, "ckBTC");
  return Number(balance);
}
