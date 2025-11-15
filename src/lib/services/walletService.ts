/**
 * Wallet Service
 *
 * Handles fiat currency operations:
 * - P2P fiat transfers between users
 * - Balance queries
 * - Transaction history
 *
 * All transfers automatically collect 0.5% platform fee
 */

import { walletCanisterService } from "./icp/canisters/walletCanisterService";
import type {
  TransferRequest,
  TransferResponse,
  Transaction,
  FiatCurrency,
} from "$/declarations/wallet_canister/wallet_canister.did";

/**
 * Transfer Request (simplified)
 */
export interface TransferFiatParams {
  fromUserId: string;
  toUserId: string;
  amount: number; // In smallest currency unit
  currency: string; // e.g., "UGX"
  pin: string;
  description?: string;
}

/**
 * Currency mapping helper
 */
const CURRENCY_MAP: Record<string, keyof typeof FiatCurrency> = {
  UGX: "UGX",
  KES: "KES",
  TZS: "TZS",
  NGN: "NGN",
  GHS: "GHS",
  ZAR: "ZAR",
  // Add more as needed
};

/**
 * Wallet Service
 */
export class WalletService {
  /**
   * Transfer fiat between users (P2P)
   * Automatically collects 0.5% platform fee
   */
  static async transferFiat(
    params: TransferFiatParams,
  ): Promise<TransferResponse> {
    const request: TransferRequest = {
      from_user_id: params.fromUserId,
      to_user_id: params.toUserId,
      amount: BigInt(Math.round(params.amount)),
      currency: params.currency,
      pin: params.pin,
      description: params.description ? [params.description] : [],
    };

    return await walletCanisterService.transferFiat(request);
  }

  /**
   * Get fiat balance
   */
  static async getBalance(userId: string, currency: string): Promise<number> {
    const currencyEnum = this.stringToCurrency(currency);
    const balance = await walletCanisterService.getFiatBalance(
      userId,
      currencyEnum,
    );
    return Number(balance);
  }

  /**
   * Get transaction history
   */
  static async getTransactionHistory(
    userId: string,
    startTime?: Date,
    endTime?: Date,
  ): Promise<Transaction[]> {
    const startBigInt = startTime
      ? BigInt(Math.floor(startTime.getTime() / 1000))
      : undefined;
    const endBigInt = endTime
      ? BigInt(Math.floor(endTime.getTime() / 1000))
      : undefined;

    return await walletCanisterService.getTransactionHistory(
      userId,
      startBigInt,
      endBigInt,
    );
  }

  /**
   * Add to fiat balance (for deposits via agent)
   * This is called by agent_canister, not directly by users
   */
  static async addFiatBalance(
    userId: string,
    amount: number,
    currency: string,
  ): Promise<number> {
    const currencyEnum = this.stringToCurrency(currency);
    const newBalance = await walletCanisterService.addFiatBalance(
      userId,
      BigInt(Math.round(amount)),
      currencyEnum,
    );
    return Number(newBalance);
  }

  /**
   * Deduct from fiat balance (for withdrawals via agent)
   * This is called by agent_canister, not directly by users
   */
  static async deductFiatBalance(
    userId: string,
    amount: number,
    currency: string,
  ): Promise<number> {
    const currencyEnum = this.stringToCurrency(currency);
    const newBalance = await walletCanisterService.deductFiatBalance(
      userId,
      BigInt(Math.round(amount)),
      currencyEnum,
    );
    return Number(newBalance);
  }

  /**
   * Convert string currency code to FiatCurrency enum
   */
  private static stringToCurrency(currency: string): FiatCurrency {
    const upperCurrency = currency.toUpperCase();
    const currencyKey = CURRENCY_MAP[upperCurrency];

    if (!currencyKey) {
      throw new Error(`Unsupported currency: ${currency}`);
    }

    // Create FiatCurrency object with the appropriate variant
    return { [currencyKey]: null } as FiatCurrency;
  }

  /**
   * Format amount for display
   */
  static formatAmount(amount: number, currency: string): string {
    return `${amount.toLocaleString()} ${currency}`;
  }

  /**
   * Convert bigint to number (for display)
   */
  static bigintToNumber(value: bigint): number {
    return Number(value);
  }

  /**
   * Calculate platform fee (0.5%)
   */
  static calculatePlatformFee(amount: number): number {
    return Math.round(amount * 0.005); // 0.5%
  }

  /**
   * Calculate net amount after fee
   */
  static calculateNetAmount(amount: number): number {
    const fee = this.calculatePlatformFee(amount);
    return amount - fee;
  }
}

/**
 * Export singleton for convenience
 */
export const walletService = WalletService;
