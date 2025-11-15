/**
 * Crypto Canister Service
 *
 * Handles all cryptocurrency operations (ckBTC, ckUSD):
 * - Buy/Sell crypto with fiat
 * - Send crypto to external addresses
 * - Swap between cryptocurrencies
 * - Escrow management for crypto-to-cash exchanges
 *
 * IMPORTANT: All crypto operations route through this canister to collect platform fees (0.5%)
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "$/declarations/crypto_canister/crypto_canister.did.js";
import type { _SERVICE } from "$/declarations/crypto_canister/crypto_canister.did.d.ts";
import { CRYPTO_CANISTER_ID, IC_HOST } from "./config";
import type {
  BuyCryptoRequest,
  BuyCryptoResponse,
  SellCryptoRequest,
  SendCryptoRequest,
  SwapCryptoRequest,
  SwapCryptoResponse,
  CreateEscrowRequest,
  CreateEscrowResponse,
  VerifyEscrowRequest,
  Escrow,
  ReserveBalance,
} from "$/declarations/crypto_canister/crypto_canister.did";

/**
 * Create actor for crypto_canister
 */
function createCryptoActor(): _SERVICE {
  const agent = new HttpAgent({ host: IC_HOST });

  // Fetch root key for local development
  if (IC_HOST.includes("localhost")) {
    agent.fetchRootKey().catch((err) => {
      console.warn("Unable to fetch root key. Check if dfx is running:", err);
    });
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: CRYPTO_CANISTER_ID,
  });
}

/**
 * Crypto Canister Service
 */
export class CryptoCanisterService {
  private actor: _SERVICE;

  constructor() {
    this.actor = createCryptoActor();
  }

  /**
   * Buy cryptocurrency with fiat
   * Collects 0.5% platform fee automatically
   */
  async buyCrypto(request: BuyCryptoRequest): Promise<BuyCryptoResponse> {
    const result = await this.actor.buy_crypto(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Sell cryptocurrency for fiat
   * Collects 0.5% platform fee automatically
   */
  async sellCrypto(request: SellCryptoRequest): Promise<BuyCryptoResponse> {
    const result = await this.actor.sell_crypto(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Send cryptocurrency to external address
   */
  async sendCrypto(request: SendCryptoRequest): Promise<string> {
    const result = await this.actor.send_crypto(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Swap between cryptocurrencies (ckBTC ↔ ckUSD)
   * Collects 0.5% spread automatically
   */
  async swapCrypto(request: SwapCryptoRequest): Promise<SwapCryptoResponse> {
    const result = await this.actor.swap_crypto(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Check crypto balance
   * @param userIdentifier - User ID or phone number
   * @param cryptoType - "ckBTC" or "ckUSD"
   */
  async checkCryptoBalance(
    userIdentifier: string,
    cryptoType: string,
  ): Promise<bigint> {
    const result = await this.actor.check_crypto_balance(
      userIdentifier,
      cryptoType,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Create escrow for crypto-to-cash transaction with agent
   */
  async createEscrow(
    request: CreateEscrowRequest,
  ): Promise<CreateEscrowResponse> {
    const result = await this.actor.create_escrow(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Verify and claim escrow (agent receives crypto)
   */
  async verifyEscrow(request: VerifyEscrowRequest): Promise<string> {
    const result = await this.actor.verify_escrow(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Cancel escrow (user gets refund)
   */
  async cancelEscrow(
    code: string,
    userIdentifier: string,
    pin: string,
  ): Promise<void> {
    const result = await this.actor.cancel_escrow(code, userIdentifier, pin);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Get escrow status
   */
  async getEscrowStatus(code: string): Promise<Escrow> {
    const result = await this.actor.get_escrow_status(code);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get DEX provider
   */
  async getDexProvider(): Promise<string> {
    return await this.actor.get_dex_provider();
  }

  /**
   * Get spread in basis points (1 basis point = 0.01%)
   */
  async getSpreadBasisPoints(): Promise<bigint> {
    return await this.actor.get_spread_basis_points();
  }

  /**
   * Get platform reserve balance (admin only)
   */
  async getReserveBalance(btcPriceUsd: number): Promise<ReserveBalance> {
    const result = await this.actor.get_reserve_balance(btcPriceUsd);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }
}

/**
 * Singleton instance
 */
export const cryptoCanisterService = new CryptoCanisterService();
