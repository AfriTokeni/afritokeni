/**
 * Wallet Canister Service
 *
 * Handles fiat currency operations:
 * - P2P fiat transfers between users
 * - Balance queries
 * - Transaction history
 * - Escrow management for fiat
 *
 * Collects 0.5% platform fee on all transfers
 */

import { idlFactory } from "$/declarations/wallet_canister/wallet_canister.did.js";
import { AuthenticatedActorService } from "./actorFactory";
import type { _SERVICE } from "$/declarations/wallet_canister/wallet_canister.did.d.ts";
import { WALLET_CANISTER_ID } from "./config";
import type {
  TransferRequest,
  TransferResponse,
  Transaction,
  FiatCurrency,
  CreateEscrowRequest,
  CreateEscrowResponse,
  Escrow,
} from "$/declarations/wallet_canister/wallet_canister.did";

/**
 * Wallet Canister Service
 * Uses authenticated identity from Juno/Internet Identity for all calls
 */
export class WalletCanisterService {
  private actorService: AuthenticatedActorService<_SERVICE>;

  constructor() {
    this.actorService = new AuthenticatedActorService<_SERVICE>(
      idlFactory,
      WALLET_CANISTER_ID,
    );
  }

  /**
   * Get authenticated actor (creates on first use, reuses afterwards)
   */
  private async getActor(): Promise<_SERVICE> {
    return this.actorService.getActor();
  }

  /**
   * Transfer fiat between users (P2P)
   * Automatically deducts 0.5% platform fee
   */
  async transferFiat(request: TransferRequest): Promise<TransferResponse> {
    const result = await (await this.getActor()).transfer_fiat(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get fiat balance for a user
   */
  async getFiatBalance(
    userId: string,
    currency: FiatCurrency,
  ): Promise<bigint> {
    const result = await (
      await this.getActor()
    ).get_fiat_balance(userId, currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Add to fiat balance (for deposits via agent_canister)
   */
  async addFiatBalance(
    userId: string,
    amount: bigint,
    currency: FiatCurrency,
  ): Promise<bigint> {
    const result = await (
      await this.getActor()
    ).add_fiat_balance(userId, amount, currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Deduct from fiat balance (for withdrawals via agent_canister)
   */
  async deductFiatBalance(
    userId: string,
    amount: bigint,
    currency: FiatCurrency,
  ): Promise<bigint> {
    const result = await (
      await this.getActor()
    ).deduct_fiat_balance(userId, amount, currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get transaction history for a user
   * @param userId - User identifier
   * @param startTime - Optional start timestamp filter
   * @param endTime - Optional end timestamp filter
   */
  async getTransactionHistory(
    userId: string,
    startTime?: bigint,
    endTime?: bigint,
  ): Promise<Transaction[]> {
    const result = await (
      await this.getActor()
    ).get_transaction_history(
      userId,
      startTime ? [startTime] : [],
      endTime ? [endTime] : [],
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Create escrow for crypto sale to agent
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
   * Get escrow details
   */
  async getEscrow(code: string): Promise<Escrow> {
    const result = await (await this.getActor()).get_escrow(code);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Cancel escrow and refund crypto to user
   */
  async cancelEscrow(code: string, userId: string, pin: string): Promise<void> {
    const result = await (
      await this.getActor()
    ).cancel_escrow(code, userId, pin);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Claim escrow (agent receives crypto)
   */
  async claimEscrow(code: string, agentId: string): Promise<void> {
    const result = await (await this.getActor()).claim_escrow(code, agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Get wallet configuration info
   */
  async getConfigInfo(): Promise<string> {
    return await (await this.getActor()).get_config_info();
  }
}

/**
 * Singleton instance
 */
export const walletCanisterService = new WalletCanisterService();
