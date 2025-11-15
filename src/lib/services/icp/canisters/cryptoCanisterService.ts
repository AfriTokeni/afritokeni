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


import { idlFactory } from "$/declarations/crypto_canister/crypto_canister.did.js";
import { AuthenticatedActorService } from "./actorFactory";
import type { _SERVICE } from "$/declarations/crypto_canister/crypto_canister.did.d.ts";
import { CRYPTO_CANISTER_ID } from "./config";
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
 * Crypto Canister Service
 * Uses authenticated identity from Juno/Internet Identity for all calls
 */
export class CryptoCanisterService {
  private actorService: AuthenticatedActorService<_SERVICE>;

  constructor() {
    this.actorService = new AuthenticatedActorService<_SERVICE>(
      idlFactory,
      CRYPTO_CANISTER_ID,
    );
  }

  /**
   * Get authenticated actor (creates on first use, reuses afterwards)
   */
  private async getActor(): Promise<_SERVICE> {
    return this.actorService.getActor();
  }

  /**
   * Buy cryptocurrency with fiat
   * Collects 0.5% platform fee automatically
   */
  async buyCrypto(request: BuyCryptoRequest): Promise<BuyCryptoResponse> {
    const result = await (await this.getActor()).buy_crypto(request);

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
    const result = await (await this.getActor()).sell_crypto(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Send cryptocurrency to external address
   */
  async sendCrypto(request: SendCryptoRequest): Promise<string> {
    const result = await (await this.getActor()).send_crypto(request);

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
    const result = await (await this.getActor()).swap_crypto(request);

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
    const result = await (await this.getActor()).check_crypto_balance(
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
    const result = await (await this.getActor()).create_escrow(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Verify and claim escrow (agent receives crypto)
   */
  async verifyEscrow(request: VerifyEscrowRequest): Promise<string> {
    const result = await (await this.getActor()).verify_escrow(request);

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
    const result = await (await this.getActor()).cancel_escrow(code, userIdentifier, pin);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Get escrow status
   */
  async getEscrowStatus(code: string): Promise<Escrow> {
    const result = await (await this.getActor()).get_escrow_status(code);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get DEX provider
   */
  async getDexProvider(): Promise<string> {
    return await (await this.getActor()).get_dex_provider();
  }

  /**
   * Get spread in basis points (1 basis point = 0.01%)
   */
  async getSpreadBasisPoints(): Promise<bigint> {
    return await (await this.getActor()).get_spread_basis_points();
  }

  /**
   * Get platform reserve balance (admin only)
   */
  async getReserveBalance(btcPriceUsd: number): Promise<ReserveBalance> {
    const result = await (await this.getActor()).get_reserve_balance(btcPriceUsd);

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
