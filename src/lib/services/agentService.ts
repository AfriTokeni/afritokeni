import { nanoid } from "nanoid";
import { getDoc, listDocs, setDoc } from "@junobuild/core";
import { agentCanisterService } from "./icp/canisters/agentCanisterService";
import { walletCanisterService } from "./icp/canisters/walletCanisterService";

/**
 * Agent metadata stored in Juno
 * Balances are NOT stored here - they come from canisters
 */
export interface AgentMetadata {
  id: string;
  userId: string;
  businessName: string;
  phoneNumber?: string;
  email?: string;
  location: {
    country: string;
    state: string;
    city: string;
    address: string;
    coordinates: {
      lat: number;
      lng: number;
    };
  };
  isActive: boolean;
  status: "available" | "busy" | "cash_out" | "offline";
  commissionRate: number;
  createdAt: Date | string;
  rating?: number;
  reviewCount?: number;
  reviews?: any[];
}

/**
 * Complete agent profile with balances fetched from canisters
 */
export interface Agent extends AgentMetadata {
  cashBalance: number;
  digitalBalance: number;
}

/**
 * Agent balances fetched from domain canisters
 */
export interface AgentBalances {
  cashBalance: number; // From agent_canister (outstanding balance + credit)
  digitalBalance: number; // From wallet_canister (fiat balance)
  creditLimit: number; // Agent's credit limit
  availableCredit: number; // How much credit is available
  outstandingBalance: number; // How much agent owes the platform
  commissionEarned: number; // Total commission earned
  commissionPending: number; // Commission not yet paid out
}

export class AgentService {
  /**
   * Create new agent (stores only metadata in Juno)
   */
  static async createAgent(
    agent: Omit<AgentMetadata, "id" | "createdAt">,
  ): Promise<AgentMetadata> {
    const existingAgent = await this.getAgentByUserId(agent.userId);
    if (existingAgent) {
      console.warn(`Agent already exists for userId ${agent.userId}`);
      return existingAgent;
    }

    const now = new Date();
    const newAgent: AgentMetadata = {
      ...agent,
      id: nanoid(),
      createdAt: now,
    };

    const dataForJuno = {
      ...newAgent,
      createdAt: now.toISOString(),
    };

    await setDoc({
      collection: "agents",
      doc: {
        key: newAgent.id,
        data: dataForJuno,
      },
    });

    return newAgent;
  }

  /**
   * Get agent metadata from Juno (no balances)
   */
  static async getAgentMetadata(id: string): Promise<AgentMetadata | null> {
    try {
      const doc = await getDoc({
        collection: "agents",
        key: id,
      });
      return (doc?.data as AgentMetadata) || null;
    } catch (error) {
      console.error("Error getting agent metadata:", error);
      return null;
    }
  }

  /**
   * Get complete agent profile with balances from canisters
   */
  static async getAgent(
    id: string,
    currency: string = "UGX",
  ): Promise<Agent | null> {
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
   * Get agent balances from domain canisters
   * @param agentId - Agent identifier
   * @param currency - Currency code (default: UGX)
   */
  static async getAgentBalances(
    agentId: string,
    currency: string = "UGX",
  ): Promise<AgentBalances> {
    try {
      // Fetch cash balance from agent_canister
      const agentBalance = await agentCanisterService.getAgentBalance(
        agentId,
        currency,
      );

      // Fetch digital balance from wallet_canister
      const digitalBalance = await walletCanisterService.getFiatBalance(
        agentId,
        { [currency]: null } as any,
      );

      // Calculate cash balance as outstanding balance + available credit
      // This represents how much cash the agent has on hand
      const cashBalance = Number(agentBalance.outstanding_balance);

      return {
        cashBalance,
        digitalBalance: Number(digitalBalance),
        creditLimit: Number(agentBalance.credit_limit),
        availableCredit: Number(agentBalance.credit_limit) - cashBalance,
        outstandingBalance: Number(agentBalance.outstanding_balance),
        commissionEarned: Number(agentBalance.commission_earned),
        commissionPending: Number(agentBalance.commission_pending),
      };
    } catch (error) {
      console.error("Error fetching agent balances:", error);
      // Return zero balances on error
      return {
        cashBalance: 0,
        digitalBalance: 0,
        creditLimit: 0,
        availableCredit: 0,
        outstandingBalance: 0,
        commissionEarned: 0,
        commissionPending: 0,
      };
    }
  }

  /**
   * Get agent by user ID
   */
  static async getAgentByUserId(userId: string): Promise<AgentMetadata | null> {
    try {
      const docs = await listDocs({
        collection: "agents",
      });

      for (const doc of docs.items) {
        const agentData = doc.data as AgentMetadata;
        if (agentData.userId === userId) {
          return {
            ...agentData,
            createdAt: agentData.createdAt
              ? new Date(agentData.createdAt)
              : new Date(),
          };
        }
      }

      return null;
    } catch (error) {
      console.error("Error getting agent by userId:", error);
      return null;
    }
  }

  /**
   * Update agent status
   */
  static async updateAgentStatus(
    agentId: string,
    status: "available" | "busy" | "cash_out" | "offline",
  ): Promise<boolean> {
    try {
      const existingAgent = await this.getAgentMetadata(agentId);
      if (!existingAgent) return false;

      const existingDoc = await getDoc({
        collection: "agents",
        key: agentId,
      });

      if (!existingDoc) return false;

      const updatedAgent = {
        ...existingAgent,
        status,
        createdAt:
          typeof existingAgent.createdAt === "string"
            ? existingAgent.createdAt
            : existingAgent.createdAt.toISOString(),
      };

      await setDoc({
        collection: "agents",
        doc: {
          key: agentId,
          data: updatedAgent,
          version: existingDoc.version,
        },
      });

      return true;
    } catch (error) {
      console.error("Error updating agent status:", error);
      return false;
    }
  }

  /**
   * Update agent status by user ID
   */
  static async updateAgentStatusByUserId(
    userId: string,
    status: "available" | "busy" | "cash_out" | "offline",
  ): Promise<boolean> {
    const agent = await this.getAgentByUserId(userId);
    if (!agent) return false;
    return this.updateAgentStatus(agent.id, status);
  }

  /**
   * Get nearby agents with metadata
   * Balances are NOT included to avoid excessive canister calls
   * Fetch balances separately for specific agents if needed
   */
  static async getNearbyAgents(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: ("available" | "busy" | "cash_out" | "offline")[],
  ): Promise<AgentMetadata[]> {
    try {
      const docs = await listDocs({
        collection: "agents",
      });

      const agents = docs.items.map((doc) => doc.data as AgentMetadata);

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
    } catch (error) {
      console.error("Error getting nearby agents:", error);
      return [];
    }
  }

  /**
   * Get nearby agents with full profiles (including balances)
   * Use sparingly as this makes canister calls for each agent
   */
  static async getNearbyAgentsWithBalances(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: ("available" | "busy" | "cash_out" | "offline")[],
    currency: string = "UGX",
  ): Promise<Agent[]> {
    const nearbyMetadata = await this.getNearbyAgents(
      lat,
      lng,
      radius,
      includeStatuses,
    );

    // Fetch balances in parallel for all agents
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

  private static calculateDistance(
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

  private static deg2rad(deg: number): number {
    return deg * (Math.PI / 180);
  }

  /**
   * Complete agent KYC and create agent profile
   * Balances are initialized automatically by canisters on first use
   */
  static async completeAgentKYC(agentKYCData: {
    userId: string;
    firstName: string;
    lastName: string;
    phoneNumber: string;
    businessName?: string;
    location: {
      country: string;
      state: string;
      city: string;
      address: string;
      coordinates: { lat: number; lng: number };
    };
    operatingHours?: string;
    operatingDays?: string[];
    documentType?: string;
    documentNumber?: string;
    businessLicense?: string;
  }): Promise<{ user: any; agent: AgentMetadata }> {
    const { UserService } = await import("./userService");

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
    const existingAgent = await this.getAgentByUserId(agentKYCData.userId);
    let newAgent: AgentMetadata;

    if (existingAgent) {
      // Reactivate existing agent
      await this.updateAgentStatus(existingAgent.id, "available");
      const updatedAgent = await this.getAgentByUserId(agentKYCData.userId);
      if (!updatedAgent) throw new Error("Failed to retrieve updated agent");
      newAgent = updatedAgent;
    } else {
      // Create new agent metadata in Juno
      const agentData: Omit<AgentMetadata, "id" | "createdAt"> = {
        userId: agentKYCData.userId,
        businessName:
          agentKYCData.businessName ||
          `${agentKYCData.firstName} ${agentKYCData.lastName} Agent`,
        location: agentKYCData.location,
        isActive: true,
        status: "available",
        commissionRate: 0.02, // 2% commission rate
      };
      newAgent = await this.createAgent(agentData);
    }

    // Note: Agent balances are initialized automatically by domain canisters
    // - agent_canister initializes credit limit based on tier (default: New tier)
    // - wallet_canister tracks digital balance from first deposit
    // No manual balance initialization needed here

    return { user: updatedUser, agent: newAgent };
  }
}
