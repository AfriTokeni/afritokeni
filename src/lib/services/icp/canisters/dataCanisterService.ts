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

import { Actor, HttpAgent } from "@dfinity/agent";
import {
  idlFactory,
  type _SERVICE,
  type AgentReview,
  type CreateReviewRequest,
} from "$/declarations/data_canister";
import { DATA_CANISTER_ID, IC_HOST } from "./config";

/**
 * Create actor for data_canister
 */
function createDataActor(): _SERVICE {
  const agent = new HttpAgent({ host: IC_HOST });

  // Fetch root key for local development
  if (IC_HOST.includes("localhost")) {
    agent.fetchRootKey().catch((err) => {
      console.warn("Unable to fetch root key. Check if dfx is running:", err);
    });
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: DATA_CANISTER_ID,
  });
}

/**
 * Data Canister Service
 */
export class DataCanisterService {
  private actor: _SERVICE;

  constructor() {
    this.actor = createDataActor();
  }

  /**
   * Get all reviews for an agent
   * Public query - anyone can read reviews
   */
  async getAgentReviews(agentId: string): Promise<AgentReview[]> {
    const result = await this.actor.get_agent_reviews(agentId);

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

    const result = await this.actor.get_agent_reviews_by_rating(
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
    const result = await this.actor.get_verified_reviews(agentId);

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
    const result = await this.actor.get_agent_rating(agentId);

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
    const result = await this.actor.create_review(request);

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
    const result = await this.actor.delete_review(reviewId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }
}

/**
 * Singleton instance
 */
export const dataCanisterService = new DataCanisterService();
