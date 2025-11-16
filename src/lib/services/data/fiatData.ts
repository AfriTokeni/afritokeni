/**
 * Fiat Data Service (UI Helpers Only)
 *
 * Currency formatting and display utilities.
 * NO BUSINESS LOGIC - For UI display only.
 *
 * For real fiat operations, use walletService.
 */

import { walletService } from "$lib/services";

/**
 * Fetch fiat balance
 * Uses walletService to query wallet_canister
 */
export async function fetchFiatBalance(
  userId: string,
  currency: string,
): Promise<number> {
  return await walletService.getBalance(userId, currency);
}

/**
 * Currency symbols for display
 */
const CURRENCY_SYMBOLS: Record<string, string> = {
  UGX: "UGX",
  KES: "KES",
  TZS: "TZS",
  NGN: "₦",
  GHS: "GH₵",
  ZAR: "R",
  USD: "$",
  EUR: "€",
  GBP: "£",
};

/**
 * Get currency symbol for display
 */
export function getCurrencySymbol(currency: string): string {
  return CURRENCY_SYMBOLS[currency.toUpperCase()] || currency.toUpperCase();
}

/**
 * Format currency amount for display
 * Example: formatCurrency(1000000, "UGX") => "UGX 1,000,000"
 */
export function formatCurrency(amount: number, currency: string): string {
  const symbol = getCurrencySymbol(currency);
  const formatted = amount.toLocaleString("en-US", {
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  });
  return `${symbol} ${formatted}`;
}

/**
 * Format amount with currency symbol (compact)
 * Example: formatCurrencyCompact(1500000, "UGX") => "UGX 1.5M"
 */
export function formatCurrencyCompact(
  amount: number,
  currency: string,
): string {
  const symbol = getCurrencySymbol(currency);

  if (amount >= 1_000_000_000) {
    return `${symbol} ${(amount / 1_000_000_000).toFixed(1)}B`;
  } else if (amount >= 1_000_000) {
    return `${symbol} ${(amount / 1_000_000).toFixed(1)}M`;
  } else if (amount >= 1_000) {
    return `${symbol} ${(amount / 1_000).toFixed(1)}K`;
  } else {
    return `${symbol} ${amount.toFixed(0)}`;
  }
}

/**
 * Parse currency amount from string
 * Example: parseCurrencyAmount("UGX 1,000,000") => 1000000
 */
export function parseCurrencyAmount(value: string): number {
  // Remove currency symbols and commas
  const cleaned = value.replace(/[^0-9.]/g, "");
  return parseFloat(cleaned) || 0;
}

/**
 * Validate currency code
 */
export function isValidCurrency(currency: string): boolean {
  return currency.toUpperCase() in CURRENCY_SYMBOLS;
}

/**
 * Get supported currencies
 */
export function getSupportedCurrencies(): string[] {
  return Object.keys(CURRENCY_SYMBOLS);
}
