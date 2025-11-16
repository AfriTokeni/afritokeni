/**
 * Agent Data Service (Demo Mode Only)
 *
 * Loads agent demo data from static JSON files.
 * NO BUSINESS LOGIC - For UI display only.
 *
 * For real operations, use agentOperationsService and agentCanisterService.
 */

import type { Agent } from "$lib/utils/agents";

/**
 * Fetch agents from demo data
 * Used only in demo mode for UI display
 * @param _demoMode - Unused parameter for backward compatibility
 */
export async function fetchAgents(_demoMode?: boolean): Promise<Agent[]> {
  try {
    const response = await fetch("/data/demo/agents.json");
    if (!response.ok) {
      throw new Error(`Failed to fetch demo agents: ${response.statusText}`);
    }
    return await response.json();
  } catch (error) {
    console.error("Error loading demo agents:", error);
    return [];
  }
}

/**
 * Get single agent by ID (demo data)
 */
export async function getAgentById(agentId: string): Promise<Agent | null> {
  const agents = await fetchAgents();
  return agents.find((agent) => agent.id === agentId) || null;
}

/**
 * Calculate distance between two points (UI helper only)
 * For real distance-based features, use backend logic
 */
export function calculateDistance(
  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number,
): number {
  const R = 6371; // Earth's radius in km
  const dLat = toRad(lat2 - lat1);
  const dLon = toRad(lon2 - lon1);

  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(toRad(lat1)) *
      Math.cos(toRad(lat2)) *
      Math.sin(dLon / 2) *
      Math.sin(dLon / 2);

  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  return R * c;
}

function toRad(degrees: number): number {
  return (degrees * Math.PI) / 180;
}
