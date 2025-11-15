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
 * @param isDemoMode - Whether to return demo data (default: true)
 * @returns Balance in satoshis
 */
export async function fetchCkBTCBalance(
  userId: string | null,
  isDemoMode: boolean = true,
): Promise<number> {
  if (!userId) {
    return 0;
  }

  // Return demo balance for development
  if (isDemoMode) {
    // Demo: 0.0025 BTC = 250,000 satoshis (~$100 USD at $40k/BTC)
    return 250000;
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
