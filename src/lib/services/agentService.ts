/**
 * Agent Service
 *
 * Manages agent profiles with demo mode and production mode support.
 *
 * Demo Mode: Uses localStorage for agent storage
 * Production Mode: Uses agent_canister (NOT IMPLEMENTED YET)
 *
 * Note: Balances are always fetched from domain canisters (agent_canister, wallet_canister)
 */

import { nanoid } from "nanoid";
import { browser } from "$app/environment";
import { agentCanisterService } from "./icp/canisters/agentCanisterService";
import { walletCanisterService } from "./icp/canisters/walletCanisterService";

const DEMO_AGENTS_KEY = "afritokeni_demo_agents";
const DEMO_MODE_KEY = "afritokeni_demo_mode";

/**
 * Agent metadata
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
   * Check if demo mode is enabled
   */
  private static isDemoMode(): boolean {
    if (!browser) return false;
    return localStorage.getItem(DEMO_MODE_KEY) === "true";
  }

  /**
   * Load agents from localStorage (demo mode)
   */
  private static loadDemoAgents(): AgentMetadata[] {
    if (!browser) return [];

    try {
      const stored = localStorage.getItem(DEMO_AGENTS_KEY);
      if (!stored) return [];

      const agents = JSON.parse(stored);
      return agents.map((agent: any) => ({
        ...agent,
        createdAt: new Date(agent.createdAt),
      }));
    } catch (error) {
      console.error("Error loading demo agents from localStorage:", error);
      return [];
    }
  }

  /**
   * Save agents to localStorage (demo mode)
   */
  private static saveDemoAgents(agents: AgentMetadata[]): void {
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
      console.error("Error saving demo agents to localStorage:", error);
    }
  }

  /**
   * Load demo agents from JSON file on first access
   */
  private static async loadDemoAgentsFromJSON(): Promise<AgentMetadata[]> {
    try {
      const response = await fetch("/data/demo/agents.json");
      if (!response.ok) {
        throw new Error(`Failed to fetch demo agents: ${response.statusText}`);
      }

      const data = await response.json();
      return data.map((agent: any) => ({
        ...agent,
        createdAt: new Date(agent.createdAt),
      }));
    } catch (error) {
      console.error("Error loading demo agents from JSON:", error);
      return [];
    }
  }

  /**
   * Initialize demo agents if localStorage is empty
   */
  private static async initializeDemoAgents(): Promise<void> {
    if (!browser) return;

    const stored = localStorage.getItem(DEMO_AGENTS_KEY);
    if (stored) return; // Already initialized

    console.log("📦 Initializing demo agents from JSON...");
    const demoAgents = await this.loadDemoAgentsFromJSON();
    this.saveDemoAgents(demoAgents);
  }

  /**
   * Create new agent
   */
  static async createAgent(
    agent: Omit<AgentMetadata, "id" | "createdAt">,
  ): Promise<AgentMetadata> {
    const demoMode = this.isDemoMode();
    console.log(`🎯 AgentService.createAgent called (demo mode: ${demoMode})`, {
      userId: agent.userId,
      businessName: agent.businessName,
    });

    // Check if agent already exists
    const existingAgent = await this.getAgentByUserId(agent.userId);
    if (existingAgent) {
      console.warn(`⚠️ Agent already exists for userId ${agent.userId}, returning existing profile`);
      return existingAgent;
    }

    if (demoMode) {
      // Demo mode: Store in localStorage
      await this.initializeDemoAgents();

      const now = new Date();
      const newAgent: AgentMetadata = {
        ...agent,
        id: nanoid(),
        createdAt: now,
        status: agent.status || "available",
        isActive: agent.isActive !== false,
        commissionRate: agent.commissionRate || 0.02,
      };

      const agents = this.loadDemoAgents();
      agents.push(newAgent);
      this.saveDemoAgents(agents);

      console.log("✅ Agent created in demo mode:", newAgent.id);
      return newAgent;
    } else {
      // Production mode: Call agent_canister
      try {
        console.log("📝 Creating agent profile in production mode:", {
          userId: agent.userId,
          businessName: agent.businessName,
          location: agent.location,
        });

        const result = await agentCanisterService.createAgentProfile({
          user_id: agent.userId,
          business_name: agent.businessName,
          business_address: agent.location.address,
          location: {
            country: agent.location.country,
            state: agent.location.state,
            city: agent.location.city,
            address: agent.location.address,
            latitude: agent.location.coordinates.lat,
            longitude: agent.location.coordinates.lng,
          },
          commission_rate: agent.commissionRate || 0.02,
        });

        console.log(
          "✅ Agent created in production mode for user:",
          result.user_id,
        );

        // Convert Candid AgentStatus back to string
        let status: "available" | "busy" | "cash_out" | "offline" = "available";
        if ("Available" in result.status) status = "available";
        else if ("Busy" in result.status) status = "busy";
        else if ("CashOut" in result.status) status = "cash_out";
        else if ("Offline" in result.status) status = "offline";

        return {
          id: result.user_id, // Use user_id as agent id
          userId: result.user_id,
          businessName: result.business_name,
          phoneNumber: agent.phoneNumber,
          email: agent.email,
          location: {
            country: result.location.country,
            state: result.location.state,
            city: result.location.city,
            address: result.location.address,
            coordinates: {
              lat: result.location.latitude,
              lng: result.location.longitude,
            },
          },
          isActive: result.is_active,
          status,
          commissionRate: result.commission_rate,
          createdAt: new Date(Number(result.created_at) / 1_000_000), // Convert nanoseconds to ms
        };
      } catch (error) {
        console.error("Failed to create agent in production mode:", error);
        throw new Error(
          `Failed to create agent profile: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    }
  }

  /**
   * Get agent metadata (no balances)
   */
  static async getAgentMetadata(id: string): Promise<AgentMetadata | null> {
    if (this.isDemoMode()) {
      await this.initializeDemoAgents();
      const agents = this.loadDemoAgents();
      return agents.find((a) => a.id === id) || null;
    } else {
      // Production mode: Call agent_canister when available
      // TODO: Implement agent_canister.get_agent_metadata(id) call
      console.warn(
        "⚠️  Agent retrieval in production mode requires agent_canister. Enable demo mode or implement agent_canister.",
      );
      return null; // Graceful fallback: no agent found
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
    if (this.isDemoMode()) {
      // Demo mode: Return demo balances from localStorage
      await this.initializeDemoAgents();
      const agents = this.loadDemoAgents();
      const agent = agents.find((a) => a.id === agentId);

      if (agent && "cashBalance" in agent && "digitalBalance" in agent) {
        const cashBalance = (agent as any).cashBalance || 0;
        const digitalBalance = (agent as any).digitalBalance || 0;

        return {
          cashBalance,
          digitalBalance,
          creditLimit: 5_000_000, // Demo credit limit
          availableCredit: 5_000_000 - cashBalance,
          outstandingBalance: cashBalance,
          commissionEarned: 0,
          commissionPending: 0,
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
      };
    }

    // Production mode: Fetch from canisters
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
    if (this.isDemoMode()) {
      await this.initializeDemoAgents();
      const agents = this.loadDemoAgents();
      return agents.find((a) => a.userId === userId) || null;
    } else {
      // Production mode: Call agent_canister
      try {
        console.log("🔍 Fetching agent profile for user:", userId);
        const result = await agentCanisterService.getAgentProfile(userId);

        if (!result) {
          console.log("❌ No agent profile found in agent_canister for user:", userId);
          return null;
        }

        console.log("✅ Found agent profile:", result.user_id);

        // Convert Candid AgentStatus back to string
        let status: "available" | "busy" | "cash_out" | "offline" = "available";
        if ("Available" in result.status) status = "available";
        else if ("Busy" in result.status) status = "busy";
        else if ("CashOut" in result.status) status = "cash_out";
        else if ("Offline" in result.status) status = "offline";

        return {
          id: result.user_id, // Use user_id as agent id
          userId: result.user_id,
          businessName: result.business_name,
          phoneNumber: undefined, // Not stored in AgentProfile
          email: undefined, // Not stored in AgentProfile
          location: {
            country: result.location.country,
            state: result.location.state,
            city: result.location.city,
            address: result.location.address,
            coordinates: {
              lat: result.location.latitude,
              lng: result.location.longitude,
            },
          },
          isActive: result.is_active,
          status,
          commissionRate: result.commission_rate,
          createdAt: new Date(Number(result.created_at) / 1_000_000), // Convert nanoseconds to ms
        };
      } catch (error) {
        console.error("Failed to get agent profile:", error);
        return null; // Graceful fallback: no agent found
      }
    }
  }

  /**
   * Update agent status
   */
  static async updateAgentStatus(
    agentId: string,
    status: "available" | "busy" | "cash_out" | "offline",
  ): Promise<boolean> {
    if (this.isDemoMode()) {
      await this.initializeDemoAgents();
      const agents = this.loadDemoAgents();
      const agentIndex = agents.findIndex((a) => a.id === agentId);

      if (agentIndex === -1) return false;

      agents[agentIndex].status = status;
      this.saveDemoAgents(agents);

      console.log(`✅ Agent ${agentId} status updated to ${status}`);
      return true;
    } else {
      // Production mode: Call agent_canister
      try {
        // Map status to Candid enum format
        const statusMap: Record<string, any> = {
          available: { Available: null },
          busy: { Busy: null },
          cash_out: { CashOut: null },
          offline: { Offline: null },
        };

        await agentCanisterService.updateAgentProfile({
          user_id: agentId,
          business_name: [],
          business_address: [],
          location: [],
          commission_rate: [],
          status: [statusMap[status]],
        });

        console.log(`✅ Agent ${agentId} status updated to ${status}`);
        return true;
      } catch (error) {
        console.error("Failed to update agent status:", error);
        return false; // Graceful fallback: update failed
      }
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
    if (this.isDemoMode()) {
      await this.initializeDemoAgents();
      const agents = this.loadDemoAgents();

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
    } else {
      // Production mode: Call agent_canister
      try {
        const results = await agentCanisterService.getNearbyAgentProfiles(
          lat,
          lng,
          radius,
          BigInt(100), // Limit to 100 results
        );

        const agents: AgentMetadata[] = results.map((result) => {
          // Convert Candid AgentStatus back to string
          let status: "available" | "busy" | "cash_out" | "offline" =
            "available";
          if ("Available" in result.status) status = "available";
          else if ("Busy" in result.status) status = "busy";
          else if ("CashOut" in result.status) status = "cash_out";
          else if ("Offline" in result.status) status = "offline";

          return {
            id: result.user_id,
            userId: result.user_id,
            businessName: result.business_name,
            phoneNumber: undefined,
            email: undefined,
            location: {
              country: result.location.country,
              state: result.location.state,
              city: result.location.city,
              address: result.location.address,
              coordinates: {
                lat: result.location.latitude,
                lng: result.location.longitude,
              },
            },
            isActive: result.is_active,
            status,
            commissionRate: result.commission_rate,
            createdAt: new Date(Number(result.created_at) / 1_000_000),
          };
        });

        // Filter by status if specified
        if (includeStatuses) {
          return agents.filter((agent) =>
            includeStatuses.includes(agent.status),
          );
        }

        return agents;
      } catch (error) {
        console.error("Failed to get nearby agents:", error);
        return []; // Graceful fallback: no agents found
      }
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
      // Create new agent metadata
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
