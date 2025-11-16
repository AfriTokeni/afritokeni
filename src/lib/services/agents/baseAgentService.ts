/**
 * Base Agent Service (Abstract)
 *
 * Defines the contract for all agent service implementations.
 * Both demo and production services must implement this interface.
 */

import type {
  AgentMetadata,
  AgentBalances,
  Agent,
  AgentKYCData,
  AgentStatus,
} from "$lib/types/agent";
import type { AfricanCurrency } from "$lib/types/currency";

export abstract class BaseAgentService {
  /**
   * Create a new agent profile
   */
  abstract createAgent(
    agent: Omit<AgentMetadata, "id" | "createdAt">,
  ): Promise<AgentMetadata>;

  /**
   * Get agent metadata by ID (no balances)
   */
  abstract getAgentMetadata(id: string): Promise<AgentMetadata | null>;

  /**
   * Get complete agent profile with balances
   */
  abstract getAgent(
    id: string,
    currency: AfricanCurrency,
  ): Promise<Agent | null>;

  /**
   * Get agent by principal ID
   */
  abstract getAgentByPrincipal(
    principalId: string,
  ): Promise<AgentMetadata | null>;

  /**
   * Get agent balances from domain canisters
   */
  abstract getAgentBalances(
    agentId: string,
    currency: AfricanCurrency,
  ): Promise<AgentBalances>;

  /**
   * Update agent status
   */
  abstract updateAgentStatus(
    agentId: string,
    status: AgentStatus,
  ): Promise<boolean>;

  /**
   * Update agent status by user ID
   */
  abstract updateAgentStatusByUserId(
    userId: string,
    status: AgentStatus,
  ): Promise<boolean>;

  /**
   * Get nearby agents (metadata only, no balances)
   */
  abstract getNearbyAgents(
    lat: number,
    lng: number,
    radius: number,
    includeStatuses?: AgentStatus[],
  ): Promise<AgentMetadata[]>;

  /**
   * Get nearby agents with balances
   */
  abstract getNearbyAgentsWithBalances(
    lat: number,
    lng: number,
    radius: number,
    includeStatuses?: AgentStatus[],
    currency?: AfricanCurrency,
  ): Promise<Agent[]>;

  /**
   * Complete agent KYC and create agent profile
   */
  abstract completeAgentKYC(
    agentKYCData: AgentKYCData,
  ): Promise<{ user: unknown; agent: AgentMetadata }>;
}
