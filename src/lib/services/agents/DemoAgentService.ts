/**
 * Demo Agent Service
 *
 * Implementation using localStorage for demo/development mode.
 * No canister calls - all data stored in browser localStorage.
 */

import { nanoid } from "nanoid";
import { browser } from "$app/environment";
import { BaseAgentService } from "./BaseAgentService";
import { createLogger } from "$lib/utils/secureLogger";
import { validateAgentInput, type AgentInput } from "$lib/utils/validation";
import { sanitizeError } from "$lib/utils/errorHandler";
import type {
  AgentMetadata,
  AgentBalances,
  Agent,
  AgentKYCData,
  AgentStatus,
} from "$lib/types/agent";
import type { AfricanCurrency } from "$lib/types/currency";

const logger = createLogger("DemoAgentService");

const DEMO_AGENTS_KEY = "afritokeni_demo_agents";

export class DemoAgentService extends BaseAgentService {
  /**
   * Load agents from localStorage
   */
  private loadAgents(): AgentMetadata[] {
    if (!browser) return [];

    try {
      const stored = localStorage.getItem(DEMO_AGENTS_KEY);
      if (!stored) return [];

      const agents = JSON.parse(stored) as AgentMetadata[];
      return agents.map((agent) => ({
        ...agent,
        createdAt: new Date(agent.createdAt),
      }));
    } catch (error) {
      logger.error("Error loading demo agents from localStorage:", error);
      return [];
    }
  }

  /**
   * Save agents to localStorage
   */
  private saveAgents(agents: AgentMetadata[]): void {
    if (!browser) return;

    try {
      const dataToStore = agents.map((agent) => ({
        ...agent,
        createdAt:
          typeof agent.createdAt === "string"
            ? agent.createdAt
            : agent.createdAt.toISOString(),
      }));

      localStorage.setItem(DEMO_AGENTS_KEY, JSON.stringify(dataToStore));
    } catch (error) {
      logger.error("Error saving demo agents to localStorage:", error);
    }
  }

  /**
   * Load demo agents from JSON file
   */
  private async loadDemoAgentsFromJSON(): Promise<AgentMetadata[]> {
    try {
      const response = await fetch("/data/demo/agents.json");
      if (!response.ok) {
        throw new Error(`Failed to fetch demo agents: ${response.statusText}`);
      }

      const data = (await response.json()) as AgentMetadata[];
      return data.map((agent) => ({
        ...agent,
        createdAt: new Date(agent.createdAt),
      }));
    } catch (error) {
      logger.error("Error loading demo agents from JSON:", error);
      return [];
    }
  }

  /**
   * Initialize demo agents if localStorage is empty
   */
  private async initializeDemoAgents(): Promise<void> {
    if (!browser) return;

    const stored = localStorage.getItem(DEMO_AGENTS_KEY);
    if (stored) return;

    logger.debug("Initializing demo agents from JSON...");
    const demoAgents = await this.loadDemoAgentsFromJSON();
    this.saveAgents(demoAgents);
  }

  /**
   * Create a new agent profile
   */
  async createAgent(
    agent: Omit<AgentMetadata, "id" | "createdAt">,
  ): Promise<AgentMetadata> {
    logger.debug("createAgent (demo mode)", {
      businessName: agent.businessName,
    });

    // Validate agent input
    try {
      validateAgentInput(agent as AgentInput);
    } catch (error) {
      throw sanitizeError(error, "Invalid agent data");
    }

    await this.initializeDemoAgents();

    // Check if agent already exists
    const existingAgent = await this.getAgentByPrincipal(agent.userId);
    if (existingAgent) {
      logger.warn("Agent already exists, returning existing profile");
      return existingAgent;
    }

    const now = new Date();
    const newAgent: AgentMetadata = {
      ...agent,
      id: nanoid(),
      createdAt: now,
      status: agent.status || "available",
      isActive: agent.isActive !== false,
      commissionRate: agent.commissionRate || 0.02,
    };

    const agents = this.loadAgents();
    agents.push(newAgent);
    this.saveAgents(agents);

    logger.debug("Agent created in demo mode");
    return newAgent;
  }

  /**
   * Get agent metadata by ID
   */
  async getAgentMetadata(id: string): Promise<AgentMetadata | null> {
    await this.initializeDemoAgents();
    const agents = this.loadAgents();
    return agents.find((a) => a.id === id) || null;
  }

  /**
   * Get complete agent profile with balances
   */
  async getAgent(id: string, currency: AfricanCurrency): Promise<Agent | null> {
    const metadata = await this.getAgentMetadata(id);
    if (!metadata) return null;

    const balances = await this.getAgentBalances(id, currency);

    return {
      ...metadata,
      cashBalance: balances.cashBalance,
      digitalBalance: balances.digitalBalance,
    };
  }

  /**
   * Get agent by principal ID
   */
  async getAgentByPrincipal(
    principalId: string,
  ): Promise<AgentMetadata | null> {
    await this.initializeDemoAgents();
    const agents = this.loadAgents();
    return agents.find((a) => a.userId === principalId) || null;
  }

  /**
   * Get agent balances (demo mode returns mock balances from localStorage)
   */
  async getAgentBalances(
    agentId: string,
    currency: AfricanCurrency,
  ): Promise<AgentBalances> {
    await this.initializeDemoAgents();
    const agents = this.loadAgents();
    const agent = agents.find((a) => a.id === agentId);

    // Check if agent has balances stored (from demo data)
    if (agent && "cashBalance" in agent && "digitalBalance" in agent) {
      const cashBalance = (agent as Agent).cashBalance || 0;
      const digitalBalance = (agent as Agent).digitalBalance || 0;

      return {
        cashBalance,
        digitalBalance,
        creditLimit: 5_000_000,
        availableCredit: 5_000_000 - cashBalance,
        outstandingBalance: cashBalance,
        commissionEarned: 0,
        commissionPending: 0,
        currency,
      };
    }

    return {
      cashBalance: 0,
      digitalBalance: 0,
      creditLimit: 0,
      availableCredit: 0,
      outstandingBalance: 0,
      commissionEarned: 0,
      commissionPending: 0,
      currency,
    };
  }

  /**
   * Update agent status
   */
  async updateAgentStatus(
    agentId: string,
    status: AgentStatus,
  ): Promise<boolean> {
    await this.initializeDemoAgents();
    const agents = this.loadAgents();
    const agentIndex = agents.findIndex((a) => a.id === agentId);

    if (agentIndex === -1) return false;

    agents[agentIndex].status = status;
    this.saveAgents(agents);

    logger.debug("Agent status updated in demo mode");
    return true;
  }

  /**
   * Update agent status by user ID
   */
  async updateAgentStatusByUserId(
    userId: string,
    status: AgentStatus,
  ): Promise<boolean> {
    const agent = await this.getAgentByPrincipal(userId);
    if (!agent) return false;
    return this.updateAgentStatus(agent.id, status);
  }

  /**
   * Get nearby agents
   */
  async getNearbyAgents(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: AgentStatus[],
  ): Promise<AgentMetadata[]> {
    await this.initializeDemoAgents();
    const agents = this.loadAgents();

    const nearbyAgents = agents.filter((agent) => {
      if (!agent.isActive) return false;
      if (includeStatuses && !includeStatuses.includes(agent.status))
        return false;

      const distance = this.calculateDistance(
        lat,
        lng,
        agent.location.coordinates.lat,
        agent.location.coordinates.lng,
      );

      return distance <= radius;
    });

    return nearbyAgents.sort((a, b) => {
      const distA = this.calculateDistance(
        lat,
        lng,
        a.location.coordinates.lat,
        a.location.coordinates.lng,
      );
      const distB = this.calculateDistance(
        lat,
        lng,
        b.location.coordinates.lat,
        b.location.coordinates.lng,
      );
      return distA - distB;
    });
  }

  /**
   * Get nearby agents with balances
   */
  async getNearbyAgentsWithBalances(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: AgentStatus[],
    currency: AfricanCurrency = "UGX",
  ): Promise<Agent[]> {
    const nearbyMetadata = await this.getNearbyAgents(
      lat,
      lng,
      radius,
      includeStatuses,
    );

    const agentPromises = nearbyMetadata.map(async (metadata) => {
      const balances = await this.getAgentBalances(metadata.id, currency);
      return {
        ...metadata,
        cashBalance: balances.cashBalance,
        digitalBalance: balances.digitalBalance,
      };
    });

    return Promise.all(agentPromises);
  }

  /**
   * Complete agent KYC
   */
  async completeAgentKYC(
    agentKYCData: AgentKYCData,
  ): Promise<{ user: unknown; agent: AgentMetadata }> {
    // Import UserService dynamically to avoid circular dependencies
    const { UserService } = await import("../userService");

    // Update user to approved KYC status
    const userUpdates = {
      firstName: agentKYCData.firstName,
      lastName: agentKYCData.lastName,
      email: agentKYCData.phoneNumber,
      kycStatus: "approved" as const,
      isVerified: true,
    };

    const userUpdateSuccess = await UserService.updateUser(
      agentKYCData.userId,
      userUpdates,
      "web",
    );
    if (!userUpdateSuccess) throw new Error("Failed to update user details");

    const updatedUser = await UserService.getUserByKey(agentKYCData.userId);
    if (!updatedUser) throw new Error("Failed to retrieve updated user");

    // Check if agent already exists
    const existingAgent = await this.getAgentByPrincipal(agentKYCData.userId);
    let newAgent: AgentMetadata;

    if (existingAgent) {
      await this.updateAgentStatus(existingAgent.id, "available");
      const updatedAgent = await this.getAgentByPrincipal(agentKYCData.userId);
      if (!updatedAgent) throw new Error("Failed to retrieve updated agent");
      newAgent = updatedAgent;
    } else {
      const agentData: Omit<AgentMetadata, "id" | "createdAt"> = {
        userId: agentKYCData.userId,
        businessName:
          agentKYCData.businessName ||
          `${agentKYCData.firstName} ${agentKYCData.lastName} Agent`,
        location: agentKYCData.location,
        isActive: true,
        status: "available",
        commissionRate: 0.02,
      };
      newAgent = await this.createAgent(agentData);
    }

    return { user: updatedUser, agent: newAgent };
  }

  /**
   * Calculate distance between two coordinates (Haversine formula)
   */
  private calculateDistance(
    lat1: number,
    lon1: number,
    lat2: number,
    lon2: number,
  ): number {
    const R = 6371;
    const dLat = this.deg2rad(lat2 - lat1);
    const dLon = this.deg2rad(lon2 - lon1);
    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.deg2rad(lat1)) *
        Math.cos(this.deg2rad(lat2)) *
        Math.sin(dLon / 2) *
        Math.sin(dLon / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return R * c;
  }

  private deg2rad(deg: number): number {
    return deg * (Math.PI / 180);
  }
}
