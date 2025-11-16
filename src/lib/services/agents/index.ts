/**
 * Agent Service Factory
 *
 * Exports the appropriate agent service implementation based on mode.
 * Switches between demo (localStorage) and production (canisters) automatically.
 */

import { browser } from "$app/environment";
import { ProductionAgentService } from "./productionAgentService";
import { DemoAgentService } from "./demoAgentService";
import type { BaseAgentService } from "./baseAgentService";

const DEMO_MODE_KEY = "afritokeni_demo_mode";

/**
 * Check if demo mode is enabled
 */
function isDemoMode(): boolean {
  if (!browser) return false;
  return localStorage.getItem(DEMO_MODE_KEY) === "true";
}

/**
 * Singleton instance
 */
let instance: BaseAgentService | null = null;

/**
 * Get the agent service instance
 * Returns demo or production implementation based on mode
 */
export function getAgentService(): BaseAgentService {
  if (!instance) {
    instance = isDemoMode()
      ? new DemoAgentService()
      : new ProductionAgentService();
  }
  return instance;
}

/**
 * Reset the agent service instance
 * Call this when switching between demo and production mode
 */
export function resetAgentService(): void {
  instance = null;
}

/**
 * Enable demo mode
 */
export function enableDemoMode(): void {
  if (!browser) return;
  localStorage.setItem(DEMO_MODE_KEY, "true");
  resetAgentService();
}

/**
 * Disable demo mode (use production)
 */
export function disableDemoMode(): void {
  if (!browser) return;
  localStorage.setItem(DEMO_MODE_KEY, "false");
  resetAgentService();
}

/**
 * Check if currently in demo mode
 */
export function isInDemoMode(): boolean {
  return isDemoMode();
}

// Export for direct use (singleton pattern)
export const AgentService = getAgentService();

// Export types for consumers
export type { BaseAgentService } from "./baseAgentService";
export { DemoAgentService } from "./demoAgentService";
export { ProductionAgentService } from "./productionAgentService";

// Re-export agent types for convenience
export type {
  AgentMetadata,
  Agent,
  AgentBalances,
  AgentStatus,
  AgentLocation,
  AgentKYCData,
} from "$lib/types/agent";
