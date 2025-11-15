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
export async function fetchCkBTCBalance(
  userId: string | null,
): Promise<number> {
  if (!userId) {
    return 0;
  }
  const balance = await cryptoService.checkBalance(userId, "ckBTC");
  return Number(balance);
}

/**
 * Convert satoshis to BTC
 * @param satoshis - Amount in satoshis
 * @returns Amount in BTC
 */
export function satoshisToBTC(satoshis: number): number {
  return satoshis / 100_000_000;
}
