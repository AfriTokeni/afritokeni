/**
 * Review Service
 *
 * Handles agent review operations via data_canister.
 * Reviews are stored on-chain in data_canister for transparency.
 */

import { dataCanisterService } from "$lib/services/icp/canisters/dataCanisterService";
import type { AgentReview } from "$/declarations/data_canister/data_canister.did";

/**
 * List all reviews for an agent
 * @param agentId - Agent identifier
 * @returns Array of reviews, sorted by most recent first
 */
export async function listAgentReviews(
  agentId: string,
): Promise<AgentReview[]> {
  return await dataCanisterService.getAgentReviews(agentId);
}

/**
 * Get verified reviews for an agent
 * Only returns reviews linked to actual transactions
 * @param agentId - Agent identifier
 * @returns Array of verified reviews
 */
export async function listVerifiedReviews(
  agentId: string,
): Promise<AgentReview[]> {
  return await dataCanisterService.getVerifiedReviews(agentId);
}

/**
 * Get reviews by rating for an agent
 * @param agentId - Agent identifier
 * @param rating - Filter by rating (1-5)
 * @returns Array of reviews with the specified rating
 */
export async function listReviewsByRating(
  agentId: string,
  rating: number,
): Promise<AgentReview[]> {
  return await dataCanisterService.getAgentReviewsByRating(agentId, rating);
}

/**
 * Get agent rating summary
 * @param agentId - Agent identifier
 * @returns Object with average rating and total review count
 */
export async function getAgentRatingSummary(agentId: string): Promise<{
  averageRating: number;
  totalReviews: number;
}> {
  const [avgRating, count] = await dataCanisterService.getAgentRating(agentId);
  return {
    averageRating: avgRating,
    totalReviews: Number(count),
  };
}
