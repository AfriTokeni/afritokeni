/**
 * Agent Service
 * Handles all agent operations with Juno DB
 */

import { listDocs, setDoc, getDoc } from "@junobuild/core";
import type { Doc } from "@junobuild/core";
import { toast } from "$lib/stores/toast";
import type {
  AgentProfile,
  AgentStats,
  AgentFilters,
  AgentStatus,
} from "$lib/types/admin";

const COLLECTION = "agents";

/**
 * Juno document data structure for Agent
 */
export interface AgentDocData {
  name: string;
  email: string;
  phone: string;
  location: string;
  country: string;
  status: AgentStatus;
  rating: number;
  reviewCount: number;
  transactionCount: number;
  revenue: number;
  commission: number;
  joinedAt: string;
  lastActive: string;
}

/**
 * List agents with filters
 */
export async function listAgents(
  filters: AgentFilters = {},
): Promise<AgentProfile[]> {
  try {
    const { items } = await listDocs<AgentDocData>({
      collection: COLLECTION,
      filter: {},
    });

    let agents = items.map((doc) => docToAgent(doc));

    // Apply status filter
    if (filters.status && filters.status !== "all") {
      agents = agents.filter((agent) => agent.status === filters.status);
    }

    // Apply search filter
    if (filters.searchQuery) {
      const query = filters.searchQuery.toLowerCase();
      agents = agents.filter(
        (agent) =>
          agent.name.toLowerCase().includes(query) ||
          agent.email.toLowerCase().includes(query) ||
          agent.location.toLowerCase().includes(query),
      );
    }

    // Apply sorting
    const sortBy = filters.sortBy ?? "joinDate";
    const sortOrder = filters.sortOrder ?? "desc";

    agents.sort((a, b) => {
      let comparison = 0;

      switch (sortBy) {
        case "joinDate":
          comparison =
            new Date(a.joinedAt).getTime() - new Date(b.joinedAt).getTime();
          break;
        case "commission":
          comparison = a.commission - b.commission;
          break;
        case "revenue":
          comparison = a.revenue - b.revenue;
          break;
        case "rating":
          comparison = a.rating - b.rating;
          break;
      }

      return sortOrder === "asc" ? comparison : -comparison;
    });

    // Apply pagination
    const offset = filters.offset ?? 0;
    const limit = filters.limit ?? 20;
    return agents.slice(offset, offset + limit);
  } catch (error) {
    console.error("Error listing agents:", error);
    toast.show("error", "Failed to load agents");
    throw error;
  }
}

/**
 * Get agent by ID
 */
export async function getAgent(id: string): Promise<AgentProfile> {
  if (!id) {
    throw new Error("Agent ID is required");
  }

  try {
    const doc = await getDoc<AgentDocData>({
      collection: COLLECTION,
      key: id,
    });

    if (!doc) {
      throw new Error("Agent not found");
    }

    return docToAgent(doc);
  } catch (error) {
    console.error("Error getting agent:", error);
    toast.show("error", "Failed to load agent");
    throw error;
  }
}

/**
 * Update agent status
 */
export async function updateAgentStatus(
  agentId: string,
  status: AgentStatus,
): Promise<void> {
  if (!agentId) {
    throw new Error("Agent ID is required");
  }

  if (!status) {
    throw new Error("Status is required");
  }

  try {
    const doc = await getDoc<AgentDocData>({
      collection: COLLECTION,
      key: agentId,
    });

    if (!doc) {
      throw new Error("Agent not found");
    }

    await setDoc({
      collection: COLLECTION,
      doc: {
        ...doc,
        data: {
          ...doc.data,
          status,
          lastActive: new Date().toISOString(),
        },
        updated_at: BigInt(Date.now() * 1000000),
      },
    });

    toast.show("success", `Agent status updated to ${status}`);
  } catch (error) {
    console.error("Error updating agent status:", error);
    toast.show("error", "Failed to update agent status");
    throw error;
  }
}

/**
 * Ban agent
 */
export async function banAgent(agentId: string, reason: string): Promise<void> {
  if (!agentId) {
    throw new Error("Agent ID is required");
  }

  try {
    const doc = await getDoc<AgentDocData>({
      collection: COLLECTION,
      key: agentId,
    });

    if (!doc) {
      throw new Error("Agent not found");
    }

    await setDoc({
      collection: COLLECTION,
      doc: {
        ...doc,
        data: {
          ...doc.data,
          status: "offline",
          lastActive: new Date().toISOString(),
        },
        updated_at: BigInt(Date.now() * 1000000),
      },
    });

    // TODO: Store ban reason in separate collection
    console.log(`Agent ${agentId} banned. Reason: ${reason}`);

    toast.show("success", "Agent banned successfully");
  } catch (error) {
    console.error("Error banning agent:", error);
    toast.show("error", "Failed to ban agent");
    throw error;
  }
}

/**
 * Get agent statistics
 */
export async function getAgentStats(): Promise<AgentStats> {
  try {
    const { items } = await listDocs<AgentDocData>({
      collection: COLLECTION,
      filter: {},
    });

    const stats: AgentStats = {
      total: items.length,
      active: 0,
      busy: 0,
      offline: 0,
      totalRevenue: 0,
      totalCommission: 0,
    };

    items.forEach((doc) => {
      const agent = doc.data;

      if (agent.status === "active") stats.active++;
      if (agent.status === "busy") stats.busy++;
      if (agent.status === "offline") stats.offline++;

      stats.totalRevenue += agent.revenue;
      stats.totalCommission += agent.commission;
    });

    return stats;
  } catch (error) {
    console.error("Error getting agent stats:", error);
    toast.show("error", "Failed to load agent statistics");
    throw error;
  }
}

/**
 * Convert Juno doc to AgentProfile
 */
function docToAgent(doc: Doc<AgentDocData>): AgentProfile {
  const data = doc.data;

  return {
    id: doc.key,
    name: data.name,
    email: data.email,
    phone: data.phone,
    location: data.location,
    country: data.country,
    status: data.status,
    rating: data.rating,
    reviewCount: data.reviewCount,
    transactionCount: data.transactionCount,
    revenue: data.revenue,
    commission: data.commission,
    joinedAt: data.joinedAt,
    lastActive: data.lastActive,
  };
}
