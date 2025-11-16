/**
 * Production Agent Service
 *
 * Implementation using ICP canisters for production mode.
 * All data stored and retrieved from agent_canister, wallet_canister, and user_canister.
 */

import { BaseAgentService } from "./BaseAgentService";
import { agentCanisterService } from "../icp/canisters/agentCanisterService";
import { walletCanisterService } from "../icp/canisters/walletCanisterService";
import { userCanisterService } from "../icp/canisters/userCanisterService";
import { createLogger } from "$lib/utils/secureLogger";
import { sanitizeError } from "$lib/utils/errorHandler";
import { validateAgentInput, type AgentInput } from "$lib/utils/validation";
import {
  convertAgentStatus,
  toCandidAgentStatus,
  currencyStringToVariant,
} from "$lib/utils/candid";
import { nanosToDate } from "$lib/utils/time";
import type {
  AgentMetadata,
  AgentBalances,
  Agent,
  AgentKYCData,
  AgentStatus,
} from "$lib/types/agent";
import type { AfricanCurrency } from "$lib/types/currency";

const logger = createLogger("ProductionAgentService");

export class ProductionAgentService extends BaseAgentService {
  /**
   * Create a new agent profile
   */
  async createAgent(
    agent: Omit<AgentMetadata, "id" | "createdAt">,
  ): Promise<AgentMetadata> {
    logger.debug("createAgent (production mode)", {
      businessName: agent.businessName,
    });

    // Validate agent input
    try {
      validateAgentInput(agent as AgentInput);
    } catch (error) {
      throw sanitizeError(error, "Invalid agent data");
    }

    // Check if agent already exists
    const existingAgent = await this.getAgentByPrincipal(agent.userId);
    if (existingAgent) {
      logger.warn("Agent already exists, returning existing profile");
      return existingAgent;
    }

    try {
      logger.debug("Creating agent profile in production mode");

      // Get user profile to get actual user_id from principal
      const userProfile = await userCanisterService.getUserByPrincipalUpdate(
        agent.userId,
      );

      if (!userProfile) {
        throw new Error(`User not found for principal: ${agent.userId}`);
      }

      logger.debug("Found user profile for principal");

      const result = await agentCanisterService.createAgentProfile({
        user_id: userProfile.id,
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

      logger.info("Agent created in production mode");

      const status = convertAgentStatus(result.status);

      return {
        id: result.user_id,
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
        createdAt: nanosToDate(result.created_at),
      };
    } catch (error) {
      throw sanitizeError(error, "Failed to create agent profile");
    }
  }

  /**
   * Get agent metadata by ID
   */
  async getAgentMetadata(id: string): Promise<AgentMetadata | null> {
    try {
      const result = await agentCanisterService.getAgentProfile(id);
      if (!result) return null;

      const status = convertAgentStatus(result.status);

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
        createdAt: nanosToDate(result.created_at),
      };
    } catch (error) {
      logger.error("Failed to get agent metadata:", error);
      return null;
    }
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
    try {
      logger.debug("Fetching agent profile for principal");

      const userProfile =
        await userCanisterService.getUserByPrincipalUpdate(principalId);

      if (!userProfile) {
        logger.debug("No user found for principal");
        return null;
      }

      logger.debug("Found user profile for principal");

      const result = await agentCanisterService.getAgentProfile(userProfile.id);

      if (!result) {
        logger.debug("No agent profile found in agent_canister for user");
        return null;
      }

      logger.debug("Found agent profile");

      const status = convertAgentStatus(result.status);

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
        createdAt: nanosToDate(result.created_at),
      };
    } catch (error) {
      logger.error("Failed to get agent profile:", error);
      return null;
    }
  }

  /**
   * Get agent balances from domain canisters
   */
  async getAgentBalances(
    agentId: string,
    currency: AfricanCurrency,
  ): Promise<AgentBalances> {
    try {
      const agentBalance = await agentCanisterService.getAgentBalance(
        agentId,
        currency,
      );

      const currencyVariant = currencyStringToVariant(currency);
      const digitalBalance = await walletCanisterService.getFiatBalance(
        agentId,
        currencyVariant,
      );

      const cashBalance = Number(agentBalance.outstanding_balance);

      return {
        cashBalance,
        digitalBalance: Number(digitalBalance),
        creditLimit: Number(agentBalance.credit_limit),
        availableCredit: Number(agentBalance.credit_limit) - cashBalance,
        outstandingBalance: Number(agentBalance.outstanding_balance),
        commissionEarned: Number(agentBalance.commission_earned),
        commissionPending: Number(agentBalance.commission_pending),
        currency,
      };
    } catch (error) {
      logger.error("Error fetching agent balances:", error);
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
  }

  /**
   * Update agent status
   */
  async updateAgentStatus(
    agentId: string,
    status: AgentStatus,
  ): Promise<boolean> {
    try {
      const candidStatus = toCandidAgentStatus(status);

      await agentCanisterService.updateAgentProfile({
        user_id: agentId,
        business_name: [],
        business_address: [],
        location: [],
        commission_rate: [],
        status: [candidStatus],
      });

      logger.debug("Agent status updated in production mode");
      return true;
    } catch (error) {
      logger.error("Failed to update agent status:", error);
      return false;
    }
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
    try {
      const results = await agentCanisterService.getNearbyAgentProfiles(
        lat,
        lng,
        radius,
        BigInt(100),
      );

      const agents: AgentMetadata[] = results.map((result) => ({
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
        status: convertAgentStatus(result.status),
        commissionRate: result.commission_rate,
        createdAt: nanosToDate(result.created_at),
      }));

      if (includeStatuses) {
        return agents.filter((agent) => includeStatuses.includes(agent.status));
      }

      return agents;
    } catch (error) {
      logger.error("Failed to get nearby agents:", error);
      return [];
    }
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
    const { UserService } = await import("../userService");

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
}
