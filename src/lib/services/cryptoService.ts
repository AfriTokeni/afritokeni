/**
 * Crypto Service - Routes all crypto operations through crypto_canister
 *
 * CRITICAL: This service ensures ALL crypto operations go through crypto_canister
 * to collect platform fees (0.5%) instead of bypassing to ledger canisters directly.
 *
 * ⚠️ DO NOT use direct ledger calls (ckBTC.ts/ckUSD.ts) for buy/sell/swap operations!
 *
 * Handles:
 * - Buy ckBTC/ckUSD with fiat (collects 0.5% fee)
 * - Sell ckBTC/ckUSD for fiat (collects 0.5% fee)
 * - Send crypto to external addresses
 * - Swap ckBTC ↔ ckUSD (collects 0.5% spread)
 * - Escrow for crypto-to-cash with agents
 */

import { cryptoCanisterService } from "./icp/canisters/cryptoCanisterService";
import type {
  BuyCryptoRequest,
  BuyCryptoResponse,
  SellCryptoRequest,
  SendCryptoRequest,
  SwapCryptoRequest,
  SwapCryptoResponse,
  CreateEscrowRequest as CryptoEscrowRequest,
  CreateEscrowResponse,
  VerifyEscrowRequest,
  Escrow,
} from "$/declarations/crypto_canister/crypto_canister.did";

/**
 * Buy Crypto Request (simplified)
 */
export interface BuyCryptoParams {
  userIdentifier: string; // Phone number or user ID
  pin: string;
  cryptoType: "ckBTC" | "ckUSD";
  currency: string; // e.g., "UGX"
  fiatAmount: number; // Amount in fiat currency
  deviceFingerprint?: string;
  geoLocation?: string;
}

/**
 * Sell Crypto Request (simplified)
 */
export interface SellCryptoParams {
  userIdentifier: string;
  pin: string;
  cryptoType: "ckBTC" | "ckUSD";
  currency: string;
  cryptoAmount: number; // Amount in satoshis/smallest unit
  deviceFingerprint?: string;
  geoLocation?: string;
}

/**
 * Send Crypto Request (simplified)
 */
export interface SendCryptoParams {
  userIdentifier: string;
  pin: string;
  cryptoType: "ckBTC" | "ckUSD";
  toAddress: string; // ICP principal or external address
  amount: number; // Amount in satoshis/smallest unit
  deviceFingerprint?: string;
  geoLocation?: string;
}

/**
 * Swap Crypto Request (simplified)
 */
export interface SwapCryptoParams {
  userIdentifier: string;
  pin: string;
  fromCrypto: "ckBTC" | "ckUSD";
  toCrypto: "ckBTC" | "ckUSD";
  amount: number; // Amount in satoshis/smallest unit
}

/**
 * Create Escrow Request (simplified)
 */
export interface CreateEscrowParams {
  userIdentifier: string;
  pin: string;
  agentId: string;
  cryptoType: "ckBTC" | "ckUSD";
  amount: number; // Amount in satoshis/smallest unit
  deviceFingerprint?: string;
  geoLocation?: string;
}

/**
 * Crypto Service Class
 */
export class CryptoService {
  /**
   * Buy cryptocurrency with fiat
   * Routes through crypto_canister to collect 0.5% platform fee
   */
  static async buyCrypto(params: BuyCryptoParams): Promise<BuyCryptoResponse> {
    const request: BuyCryptoRequest = {
      user_identifier: params.userIdentifier,
      pin: params.pin,
      crypto_type: params.cryptoType,
      currency: params.currency,
      fiat_amount: BigInt(Math.round(params.fiatAmount)),
      device_fingerprint: params.deviceFingerprint
        ? [params.deviceFingerprint]
        : [],
      geo_location: params.geoLocation ? [params.geoLocation] : [],
    };

    return await cryptoCanisterService.buyCrypto(request);
  }

  /**
   * Sell cryptocurrency for fiat
   * Routes through crypto_canister to collect 0.5% platform fee
   */
  static async sellCrypto(
    params: SellCryptoParams,
  ): Promise<BuyCryptoResponse> {
    const request: SellCryptoRequest = {
      user_identifier: params.userIdentifier,
      pin: params.pin,
      crypto_type: params.cryptoType,
      currency: params.currency,
      crypto_amount: BigInt(Math.round(params.cryptoAmount)),
      device_fingerprint: params.deviceFingerprint
        ? [params.deviceFingerprint]
        : [],
      geo_location: params.geoLocation ? [params.geoLocation] : [],
    };

    return await cryptoCanisterService.sellCrypto(request);
  }

  /**
   * Send cryptocurrency to external address
   * For withdrawals or P2P transfers
   */
  static async sendCrypto(params: SendCryptoParams): Promise<string> {
    const request: SendCryptoRequest = {
      user_identifier: params.userIdentifier,
      pin: params.pin,
      crypto_type: params.cryptoType,
      to_address: params.toAddress,
      amount: BigInt(Math.round(params.amount)),
      device_fingerprint: params.deviceFingerprint
        ? [params.deviceFingerprint]
        : [],
      geo_location: params.geoLocation ? [params.geoLocation] : [],
    };

    return await cryptoCanisterService.sendCrypto(request);
  }

  /**
   * Swap between cryptocurrencies (ckBTC ↔ ckUSD)
   * Routes through crypto_canister to collect 0.5% spread
   */
  static async swapCrypto(
    params: SwapCryptoParams,
  ): Promise<SwapCryptoResponse> {
    const request: SwapCryptoRequest = {
      user_identifier: params.userIdentifier,
      pin: params.pin,
      from_crypto: params.fromCrypto,
      to_crypto: params.toCrypto,
      amount: BigInt(Math.round(params.amount)),
    };

    return await cryptoCanisterService.swapCrypto(request);
  }

  /**
   * Check crypto balance
   * @param userIdentifier - User ID or phone number
   * @param cryptoType - "ckBTC" or "ckUSD"
   * @returns Balance in smallest unit (satoshis for ckBTC, cents for ckUSD)
   */
  static async checkBalance(
    userIdentifier: string,
    cryptoType: "ckBTC" | "ckUSD",
  ): Promise<bigint> {
    return await cryptoCanisterService.checkCryptoBalance(
      userIdentifier,
      cryptoType,
    );
  }

  /**
   * Create escrow for crypto-to-cash transaction with agent
   * User locks crypto, agent gives cash, then claims crypto with code
   */
  static async createEscrow(
    params: CreateEscrowParams,
  ): Promise<CreateEscrowResponse> {
    const request: CryptoEscrowRequest = {
      user_identifier: params.userIdentifier,
      pin: params.pin,
      agent_id: params.agentId,
      crypto_type: params.cryptoType,
      amount: BigInt(Math.round(params.amount)),
      device_fingerprint: params.deviceFingerprint
        ? [params.deviceFingerprint]
        : [],
      geo_location: params.geoLocation ? [params.geoLocation] : [],
    };

    return await cryptoCanisterService.createEscrow(request);
  }

  /**
   * Verify and claim escrow (agent receives crypto after giving cash)
   */
  static async verifyEscrow(
    code: string,
    agentId: string,
    agentPin: string,
  ): Promise<string> {
    const request: VerifyEscrowRequest = {
      code,
      agent_id: agentId,
      pin: agentPin,
    };

    return await cryptoCanisterService.verifyEscrow(request);
  }

  /**
   * Cancel escrow (user gets refund)
   */
  static async cancelEscrow(
    code: string,
    userIdentifier: string,
    pin: string,
  ): Promise<void> {
    return await cryptoCanisterService.cancelEscrow(code, userIdentifier, pin);
  }

  /**
   * Get escrow status
   */
  static async getEscrowStatus(code: string): Promise<Escrow> {
    return await cryptoCanisterService.getEscrowStatus(code);
  }

  /**
   * Convert satoshis to BTC
   */
  static satoshisToBTC(satoshis: number): number {
    return satoshis / 100_000_000;
  }

  /**
   * Convert BTC to satoshis
   */
  static btcToSatoshis(btc: number): number {
    return Math.round(btc * 100_000_000);
  }

  /**
   * Convert smallest unit to ckUSD
   */
  static smallestToUSDC(smallest: number): number {
    return smallest / 100; // Assuming 2 decimal places
  }

  /**
   * Convert ckUSD to smallest unit
   */
  static usdcToSmallest(usdc: number): number {
    return Math.round(usdc * 100);
  }

  /**
   * Format crypto amount for display
   */
  static formatAmount(amount: number, cryptoType: "ckBTC" | "ckUSD"): string {
    if (cryptoType === "ckBTC") {
      return `${this.satoshisToBTC(amount).toFixed(8)} BTC`;
    } else {
      return `${this.smallestToUSDC(amount).toFixed(2)} USDC`;
    }
  }
}

/**
 * Export singleton for convenience
 */
export const cryptoService = CryptoService;
