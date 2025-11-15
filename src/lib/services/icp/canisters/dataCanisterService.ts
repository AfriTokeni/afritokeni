/**
 * Data Canister Service
 *
 * Low-level wrapper for data_canister operations.
 * Provides direct access to data storage layer.
 *
 * USE WITH CAUTION:
 * - Most data operations should go through domain-specific services
 * - This service is for read-only queries and specialized operations
 * - All writes should be handled by domain canisters (user, wallet, crypto, agent)
 */

import { idlFactory } from "$/declarations/data_canister/data_canister.did.js";
import type {
  _SERVICE,
  AgentReview,
  CreateReviewRequest,
  Transaction,
} from "$/declarations/data_canister/data_canister.did.d.ts";
import { DATA_CANISTER_ID } from "./config";
import { AuthenticatedActorService } from "./actorFactory";

/**
 * Data Canister Service
 * Uses authenticated identity from Juno/Internet Identity for all calls
 */
export class DataCanisterService {
  private actorService: AuthenticatedActorService<_SERVICE>;

  constructor() {
    this.actorService = new AuthenticatedActorService<_SERVICE>(
      idlFactory,
      DATA_CANISTER_ID,
    );
  }

  /**
   * Get authenticated actor (creates on first use, reuses afterwards)
   */
  private async getActor(): Promise<_SERVICE> {
    return this.actorService.getActor();
  }

  /**
   * Get all reviews for an agent
   * Public query - anyone can read reviews
   */
  async getAgentReviews(agentId: string): Promise<AgentReview[]> {
    const result = await (await this.getActor()).get_agent_reviews(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get reviews by rating for an agent
   * Public query - anyone can read reviews
   */
  async getAgentReviewsByRating(
    agentId: string,
    rating: number,
  ): Promise<AgentReview[]> {
    if (rating < 1 || rating > 5) {
      throw new Error("Rating must be between 1 and 5");
    }

    const result = await (await this.getActor()).get_agent_reviews_by_rating(
      agentId,
      rating,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get verified reviews for an agent
   * Only returns reviews linked to actual transactions
   * Public query - anyone can read reviews
   */
  async getVerifiedReviews(agentId: string): Promise<AgentReview[]> {
    const result = await (await this.getActor()).get_verified_reviews(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get average rating for an agent
   * Returns [average_rating, total_count]
   * Public query - anyone can read reviews
   */
  async getAgentRating(agentId: string): Promise<[number, bigint]> {
    const result = await (await this.getActor()).get_agent_rating(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Create a review for an agent
   * Canister-only endpoint (called by agent_canister or user_canister)
   *
   * NOTE: This should typically be called from backend canisters, not directly from frontend
   */
  async createReview(request: CreateReviewRequest): Promise<AgentReview> {
    const result = await (await this.getActor()).create_review(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Delete a review (admin only)
   * For content moderation
   */
  async deleteReview(reviewId: string): Promise<void> {
    const result = await (await this.getActor()).delete_review(reviewId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Get transactions for current user
   * User can only access their own transactions
   * @param limit - Optional limit on number of transactions to return
   * @param offset - Optional offset for pagination
   */
  async getMyTransactions(
    limit?: bigint,
    offset?: bigint,
  ): Promise<Transaction[]> {
    const result = await (await this.getActor()).get_my_transactions(
      limit ? [limit] : [],
      offset ? [offset] : [],
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get transactions for a specific user
   * User can access their own, canisters can access any
   * @param userId - User identifier (phone, principal, or user ID)
   * @param limit - Optional limit on number of transactions to return
   * @param offset - Optional offset for pagination
   */
  async getUserTransactions(
    userId: string,
    limit?: bigint,
    offset?: bigint,
  ): Promise<Transaction[]> {
    const result = await (await this.getActor()).get_user_transactions(
      userId,
      limit ? [limit] : [],
      offset ? [offset] : [],
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }
}

/**
 * Singleton instance
 */
export const dataCanisterService = new DataCanisterService();
