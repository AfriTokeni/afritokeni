/**
 * Agents Page Data Loader
 * Loads agent data from Juno on page load
 */

import { listAgents, getAgentStats } from "$lib/services/juno/agentService";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  try {
    // Load all agents and stats
    const [agents, stats] = await Promise.all([
      listAgents({ limit: 100 }),
      getAgentStats(),
    ]);

    return {
      agents,
      stats,
    };
  } catch (error) {
    console.error("Error loading agent data:", error);
    // Return empty data on error
    return {
      agents: [],
      stats: {
        total: 0,
        active: 0,
        busy: 0,
        offline: 0,
        totalRevenue: 0,
        totalCommission: 0,
      },
    };
  }
};
