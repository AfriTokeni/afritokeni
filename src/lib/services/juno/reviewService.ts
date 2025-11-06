/**
 * Review Service
 * Handles agent review operations with Juno DB
 */

import { listDocs } from "@junobuild/core";
import type { Doc } from "@junobuild/core";
import { toast } from "$lib/stores/toast";
import type { AgentReview } from "$lib/types/admin";

const COLLECTION = "agent_reviews";

/**
 * Juno document data structure for Review
 */
export interface ReviewDocData {
  agentId: string;
  userId: string;
  userName: string;
  rating: number;
  comment: string;
  createdAt: string;
}

/**
 * List reviews for an agent
 */
export async function listAgentReviews(
  agentId: string,
  filters?: { rating?: number },
): Promise<AgentReview[]> {
  if (!agentId) {
    throw new Error("Agent ID is required");
  }

  try {
    const { items } = await listDocs<ReviewDocData>({
      collection: COLLECTION,
      filter: {},
    });

    let reviews = items
      .filter((doc) => doc.data.agentId === agentId)
      .map((doc) => docToReview(doc));

    // Apply rating filter
    if (filters?.rating) {
      reviews = reviews.filter((review) => review.rating === filters.rating);
    }

    // Sort by newest first
    reviews.sort(
      (a, b) =>
        new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime(),
    );

    return reviews;
  } catch (error) {
    console.error("Error listing agent reviews:", error);
    toast.show("error", "Failed to load reviews");
    throw error;
  }
}

/**
 * Convert Juno doc to AgentReview
 */
function docToReview(doc: Doc<ReviewDocData>): AgentReview {
  const data = doc.data;

  return {
    id: doc.key,
    agentId: data.agentId,
    userId: data.userId,
    userName: data.userName,
    rating: data.rating,
    comment: data.comment,
    createdAt: data.createdAt,
  };
}
