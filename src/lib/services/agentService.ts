/**
 * Agent Service (Legacy Export)
 *
 * This file provides backward compatibility for existing code.
 * All methods delegate to the new modular implementation.
 *
 * @deprecated Import from '$lib/services/agents' instead
 */

import { getAgentService } from './agents';
import type {
  AgentMetadata,
  Agent,
  AgentBalances,
  AgentKYCData,
  AgentStatus
} from '$lib/types/agent';
import type { AfricanCurrency } from '$lib/types/currency';

// Re-export types for backward compatibility
export type {
  AgentMetadata,
  Agent,
  AgentBalances,
  AgentStatus,
  AgentLocation,
  AgentKYCData
} from '$lib/types/agent';

/**
 * Legacy AgentService class
 * All methods delegate to the new modular implementation
 */
export class AgentService {
  private static get service() {
    return getAgentService();
  }

  static async createAgent(
    agent: Omit<AgentMetadata, 'id' | 'createdAt'>
  ): Promise<AgentMetadata> {
    return this.service.createAgent(agent);
  }

  static async getAgentMetadata(id: string): Promise<AgentMetadata | null> {
    return this.service.getAgentMetadata(id);
  }

  static async getAgent(id: string, currency: AfricanCurrency = 'UGX'): Promise<Agent | null> {
    return this.service.getAgent(id, currency);
  }

  static async getAgentBalances(
    agentId: string,
    currency: AfricanCurrency = 'UGX'
  ): Promise<AgentBalances> {
    return this.service.getAgentBalances(agentId, currency);
  }

  static async getAgentByPrincipal(principalId: string): Promise<AgentMetadata | null> {
    return this.service.getAgentByPrincipal(principalId);
  }

  static async getAgentByUserId(userId: string): Promise<AgentMetadata | null> {
    return this.service.getAgentByPrincipal(userId);
  }

  static async updateAgentStatus(agentId: string, status: AgentStatus): Promise<boolean> {
    return this.service.updateAgentStatus(agentId, status);
  }

  static async updateAgentStatusByUserId(userId: string, status: AgentStatus): Promise<boolean> {
    return this.service.updateAgentStatusByUserId(userId, status);
  }

  static async getNearbyAgents(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: AgentStatus[]
  ): Promise<AgentMetadata[]> {
    return this.service.getNearbyAgents(lat, lng, radius, includeStatuses);
  }

  static async getNearbyAgentsWithBalances(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: AgentStatus[],
    currency: AfricanCurrency = 'UGX'
  ): Promise<Agent[]> {
    return this.service.getNearbyAgentsWithBalances(lat, lng, radius, includeStatuses, currency);
  }

  static async completeAgentKYC(
    agentKYCData: AgentKYCData
  ): Promise<{ user: unknown; agent: AgentMetadata }> {
    return this.service.completeAgentKYC(agentKYCData);
  }
}
