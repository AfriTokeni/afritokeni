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

import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "$/declarations/wallet_canister/wallet_canister.did.js";
import type { _SERVICE } from "$/declarations/wallet_canister/wallet_canister.did.d.ts";
import { WALLET_CANISTER_ID, IC_HOST } from "./config";
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
 * Create actor for wallet_canister
 */
function createWalletActor(): _SERVICE {
  const agent = new HttpAgent({ host: IC_HOST });

  // Fetch root key for local development
  if (IC_HOST.includes("localhost")) {
    agent.fetchRootKey().catch((err) => {
      console.warn("Unable to fetch root key. Check if dfx is running:", err);
    });
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: WALLET_CANISTER_ID,
  });
}

/**
 * Wallet Canister Service
 */
export class WalletCanisterService {
  private actor: _SERVICE;

  constructor() {
    this.actor = createWalletActor();
  }

  /**
   * Transfer fiat between users (P2P)
   * Automatically deducts 0.5% platform fee
   */
  async transferFiat(request: TransferRequest): Promise<TransferResponse> {
    const result = await this.actor.transfer_fiat(request);

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
    const result = await this.actor.get_fiat_balance(userId, currency);

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
    const result = await this.actor.add_fiat_balance(userId, amount, currency);

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
    const result = await this.actor.deduct_fiat_balance(
      userId,
      amount,
      currency,
    );

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
    const result = await this.actor.get_transaction_history(
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
    const result = await this.actor.create_escrow(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get escrow details
   */
  async getEscrow(code: string): Promise<Escrow> {
    const result = await this.actor.get_escrow(code);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Cancel escrow and refund crypto to user
   */
  async cancelEscrow(code: string, userId: string, pin: string): Promise<void> {
    const result = await this.actor.cancel_escrow(code, userId, pin);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Claim escrow (agent receives crypto)
   */
  async claimEscrow(code: string, agentId: string): Promise<void> {
    const result = await this.actor.claim_escrow(code, agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Get wallet configuration info
   */
  async getConfigInfo(): Promise<string> {
    return await this.actor.get_config_info();
  }
}

/**
 * Singleton instance
 */
export const walletCanisterService = new WalletCanisterService();
