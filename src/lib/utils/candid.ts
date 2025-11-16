/**
 * Candid Type Utilities
 *
 * Type-safe utilities for working with Candid types from Internet Computer canisters.
 * Handles optional values, enum conversions, and currency mappings.
 */

import type { AgentStatus } from "$/declarations/agent_canister/agent_canister.did";
import type { FiatCurrency } from "$/declarations/wallet_canister/wallet_canister.did";

/**
 * Unwraps a Candid optional value safely
 *
 * @param opt - Candid optional in the form [] | [T]
 * @returns The unwrapped value or null if empty
 */
export function unwrapCandidOptional<T>(opt: [] | [T]): T | null {
  return opt.length > 0 ? (opt[0] ?? null) : null;
}

/**
 * Wraps a value into Candid optional format
 *
 * @param value - Value to wrap, or null/undefined
 * @returns Candid optional [] | [T]
 */
export function wrapCandidOptional<T>(value: T | null | undefined): [] | [T] {
  return value !== null && value !== undefined ? [value] : [];
}

/**
 * Agent status type used in application layer
 */
export type AppAgentStatus = "available" | "busy" | "cash_out" | "offline";

/**
 * Converts Candid AgentStatus enum to application string
 *
 * @param candidStatus - AgentStatus from Candid interface
 * @returns Application-friendly status string
 */
export function convertAgentStatus(candidStatus: AgentStatus): AppAgentStatus {
  if ("Available" in candidStatus) return "available";
  if ("Busy" in candidStatus) return "busy";
  if ("CashOut" in candidStatus) return "cash_out";
  if ("Offline" in candidStatus) return "offline";

  // Fallback to available for unknown statuses
  return "available";
}

/**
 * Converts application status string to Candid AgentStatus enum
 *
 * @param status - Application status string
 * @returns Candid AgentStatus variant
 */
export function toCandidAgentStatus(status: AppAgentStatus): AgentStatus {
  const statusMap: Record<AppAgentStatus, AgentStatus> = {
    available: { Available: null },
    busy: { Busy: null },
    cash_out: { CashOut: null },
    offline: { Offline: null },
  };

  return statusMap[status];
}

/**
 * Supported currency codes (subset of FiatCurrency variants)
 */
export const SUPPORTED_CURRENCIES = [
  "AOA",
  "BIF",
  "BWP",
  "CDF",
  "CVE",
  "DJF",
  "DZD",
  "EGP",
  "ERN",
  "ETB",
  "GHS",
  "GMD",
  "KES",
  "KMF",
  "LRD",
  "LSL",
  "LYD",
  "MAD",
  "MGA",
  "MRU",
  "MUR",
  "MWK",
  "NAD",
  "NGN",
  "RWF",
  "SCR",
  "SDG",
  "SLL",
  "SOS",
  "SSP",
  "STN",
  "SZL",
  "TND",
  "TZS",
  "UGX",
  "XAF",
  "XOF",
  "ZAR",
  "ZMW",
] as const;

/**
 * Converts currency string to Candid FiatCurrency variant
 *
 * @param currency - Currency code (e.g., "UGX", "KES")
 * @returns Candid FiatCurrency variant
 * @throws Error if currency is not supported
 */
export function currencyStringToVariant(currency: string): FiatCurrency {
  const upperCurrency = currency.toUpperCase();

  // Type-safe mapping of currency strings to Candid variants
  const currencyMap: Record<string, FiatCurrency> = {
    AOA: { AOA: null },
    BIF: { BIF: null },
    BWP: { BWP: null },
    CDF: { CDF: null },
    CVE: { CVE: null },
    DJF: { DJF: null },
    DZD: { DZD: null },
    EGP: { EGP: null },
    ERN: { ERN: null },
    ETB: { ETB: null },
    GHS: { GHS: null },
    GMD: { GMD: null },
    KES: { KES: null },
    KMF: { KMF: null },
    LRD: { LRD: null },
    LSL: { LSL: null },
    LYD: { LYD: null },
    MAD: { MAD: null },
    MGA: { MGA: null },
    MRU: { MRU: null },
    MUR: { MUR: null },
    MWK: { MWK: null },
    NAD: { NAD: null },
    NGN: { NGN: null },
    RWF: { RWF: null },
    SCR: { SCR: null },
    SDG: { SDG: null },
    SLL: { SLL: null },
    SOS: { SOS: null },
    SSP: { SSP: null },
    STN: { STN: null },
    SZL: { SZL: null },
    TND: { TND: null },
    TZS: { TZS: null },
    UGX: { UGX: null },
    XAF: { XAF: null },
    XOF: { XOF: null },
    ZAR: { ZAR: null },
    ZMW: { ZMW: null },
  };

  const variant = currencyMap[upperCurrency];
  if (!variant) {
    throw new Error(
      `Unsupported currency: ${currency}. Must be one of: ${SUPPORTED_CURRENCIES.join(", ")}`,
    );
  }

  return variant;
}

/**
 * Converts Candid FiatCurrency variant to string
 *
 * @param variant - Candid FiatCurrency variant
 * @returns Currency code string
 */
export function variantToCurrencyString(variant: FiatCurrency): string {
  const keys = Object.keys(variant);
  if (keys.length === 0) {
    throw new Error("Invalid FiatCurrency variant: empty object");
  }
  return keys[0];
}

/**
 * Checks if a currency code is supported
 *
 * @param currency - Currency code to check
 * @returns True if currency is supported
 */
export function isSupportedCurrency(currency: string): boolean {
  return SUPPORTED_CURRENCIES.includes(currency.toUpperCase() as any);
}
