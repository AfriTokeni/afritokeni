import { nanoid } from 'nanoid';
import { getDoc, setDoc, listDocs } from '@junobuild/core';
import { FraudDetectionService } from './fraudDetection';

export interface Agent {
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
  status: 'available' | 'busy' | 'cash_out' | 'offline';
  cashBalance: number;
  digitalBalance: number;
  commissionRate: number;
  createdAt: Date | string;
  rating?: number;
  reviewCount?: number;
  reviews?: any[];
}

export class AgentService {
  static async createAgent(agent: Omit<Agent, 'id' | 'createdAt'>): Promise<Agent> {
    const existingAgent = await this.getAgentByUserId(agent.userId);
    if (existingAgent) {
      console.warn(`Agent already exists for userId ${agent.userId}`);
      return existingAgent;
    }

    const now = new Date();
    const newAgent: Agent = {
      ...agent,
      id: nanoid(),
      createdAt: now
    };

    const dataForJuno = {
      ...newAgent,
      createdAt: now.toISOString()
    };

    await setDoc({
      collection: 'agents',
      doc: {
        key: newAgent.id,
        data: dataForJuno
      }
    });

    return newAgent;
  }

  static async getAgent(id: string): Promise<Agent | null> {
    try {
      const doc = await getDoc({
        collection: 'agents',
        key: id
      });
      return doc?.data as Agent || null;
    } catch (error) {
      console.error('Error getting agent:', error);
      return null;
    }
  }

  static async getAgentByUserId(userId: string): Promise<Agent | null> {
    try {
      const docs = await listDocs({
        collection: 'agents'
      });

      for (const doc of docs.items) {
        const agentData = doc.data as Agent;
        if (agentData.userId === userId) {
          return {
            ...agentData,
            createdAt: agentData.createdAt ? new Date(agentData.createdAt) : new Date()
          };
        }
      }

      return null;
    } catch (error) {
      console.error('Error getting agent by userId:', error);
      return null;
    }
  }

  static async updateAgentStatus(
    agentId: string,
    status: 'available' | 'busy' | 'cash_out' | 'offline'
  ): Promise<boolean> {
    try {
      const existingAgent = await this.getAgent(agentId);
      if (!existingAgent) return false;

      const existingDoc = await getDoc({
        collection: 'agents',
        key: agentId
      });

      if (!existingDoc) return false;

      const updatedAgent = {
        ...existingAgent,
        status,
        createdAt: typeof existingAgent.createdAt === 'string'
          ? existingAgent.createdAt
          : existingAgent.createdAt.toISOString()
      };

      await setDoc({
        collection: 'agents',
        doc: {
          key: agentId,
          data: updatedAgent,
          version: existingDoc.version
        }
      });

      return true;
    } catch (error) {
      console.error('Error updating agent status:', error);
      return false;
    }
  }

  static async updateAgentStatusByUserId(
    userId: string,
    status: 'available' | 'busy' | 'cash_out' | 'offline'
  ): Promise<boolean> {
    const agent = await this.getAgentByUserId(userId);
    if (!agent) return false;
    return this.updateAgentStatus(agent.id, status);
  }

  static async updateAgentBalance(
    agentId: string,
    updates: { cashBalance?: number; digitalBalance?: number }
  ): Promise<boolean> {
    try {
      const existingAgent = await this.getAgent(agentId);
      if (!existingAgent) return false;

      const existingDoc = await getDoc({
        collection: 'agents',
        key: agentId
      });

      if (!existingDoc) return false;

      const updatedAgent = {
        ...existingAgent,
        cashBalance: updates.cashBalance !== undefined ? updates.cashBalance : existingAgent.cashBalance,
        digitalBalance: updates.digitalBalance !== undefined ? updates.digitalBalance : existingAgent.digitalBalance,
        createdAt: typeof existingAgent.createdAt === 'string'
          ? existingAgent.createdAt
          : existingAgent.createdAt.toISOString()
      };

      await setDoc({
        collection: 'agents',
        doc: {
          key: agentId,
          data: updatedAgent,
          version: existingDoc.version
        }
      });

      return true;
    } catch (error) {
      console.error('Error updating agent balance:', error);
      return false;
    }
  }

  static async updateAgentBalanceByUserId(
    userId: string,
    updates: { cashBalance?: number; digitalBalance?: number }
  ): Promise<boolean> {
    const agent = await this.getAgentByUserId(userId);
    if (!agent) return false;
    return this.updateAgentBalance(agent.id, updates);
  }

  static async depositCashToAgent(
    agentId: string,
    amount: number,
    description?: string
  ): Promise<boolean> {
    try {
      const agent = await this.getAgent(agentId);
      if (!agent) return false;

      const newCashBalance = agent.cashBalance + amount;
      return await this.updateAgentBalance(agentId, { cashBalance: newCashBalance });
    } catch (error) {
      console.error('Error depositing cash to agent:', error);
      return false;
    }
  }

  static async getNearbyAgents(
    lat: number,
    lng: number,
    radius: number = 5,
    includeStatuses?: ('available' | 'busy' | 'cash_out' | 'offline')[]
  ): Promise<Agent[]> {
    try {
      const docs = await listDocs({
        collection: 'agents'
      });

      const agents = docs.items.map(doc => doc.data as Agent);

      const nearbyAgents = agents.filter(agent => {
        if (!agent.isActive) return false;
        if (includeStatuses && !includeStatuses.includes(agent.status)) return false;

        const distance = this.calculateDistance(
          lat,
          lng,
          agent.location.coordinates.lat,
          agent.location.coordinates.lng
        );

        return distance <= radius;
      });

      return nearbyAgents.sort((a, b) => {
        const distA = this.calculateDistance(lat, lng, a.location.coordinates.lat, a.location.coordinates.lng);
        const distB = this.calculateDistance(lat, lng, b.location.coordinates.lat, b.location.coordinates.lng);
        return distA - distB;
      });
    } catch (error) {
      console.error('Error getting nearby agents:', error);
      return [];
    }
  }

  private static calculateDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const R = 6371;
    const dLat = this.deg2rad(lat2 - lat1);
    const dLon = this.deg2rad(lon2 - lon1);
    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.deg2rad(lat1)) * Math.cos(this.deg2rad(lat2)) *
      Math.sin(dLon / 2) * Math.sin(dLon / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return R * c;
  }

  private static deg2rad(deg: number): number {
    return deg * (Math.PI / 180);
  }

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
      coordinates: { lat: number; lng: number; };
    };
    operatingHours?: string;
    operatingDays?: string[];
    documentType?: string;
    documentNumber?: string;
    businessLicense?: string;
  }): Promise<{ user: any; agent: Agent }> {
    const { UserService } = await import('./userService');
    const { BalanceService } = await import('./balanceService');

    const userUpdates = {
      firstName: agentKYCData.firstName,
      lastName: agentKYCData.lastName,
      email: agentKYCData.phoneNumber,
      kycStatus: 'approved' as const,
      isVerified: true
    };

    const userUpdateSuccess = await UserService.updateUser(agentKYCData.userId, userUpdates, 'web');
    if (!userUpdateSuccess) throw new Error('Failed to update user details');

    const updatedUser = await UserService.getUserByKey(agentKYCData.userId);
    if (!updatedUser) throw new Error('Failed to retrieve updated user');

    const userBalance = await BalanceService.getUserBalance(agentKYCData.userId);
    let digitalBalance = userBalance?.balance || 0;
    const cashBalance = Number(process.env.VITE_AGENT_INITIAL_CASH_BALANCE ?? 0);

    const existingAgent = await this.getAgentByUserId(agentKYCData.userId);
    let newAgent: Agent;

    if (existingAgent) {
      await this.updateAgentStatus(existingAgent.id, 'available');
      await this.updateAgentBalance(existingAgent.id, { cashBalance, digitalBalance });
      const updatedAgent = await this.getAgentByUserId(agentKYCData.userId);
      if (!updatedAgent) throw new Error('Failed to retrieve updated agent');
      newAgent = updatedAgent;
    } else {
      const agentData: Omit<Agent, 'id' | 'createdAt'> = {
        userId: agentKYCData.userId,
        businessName: agentKYCData.businessName || `${agentKYCData.firstName} ${agentKYCData.lastName} Agent`,
        location: agentKYCData.location,
        isActive: true,
        status: 'available',
        cashBalance,
        digitalBalance,
        commissionRate: 0.02
      };
      newAgent = await this.createAgent(agentData);
    }

    if (!userBalance && digitalBalance === 0) {
      await BalanceService.updateUserBalance(agentKYCData.userId, 0);
    }

    return { user: updatedUser, agent: newAgent };
  }

  static async initializeAllAgentsCashBalance(): Promise<{ success: boolean; updated: number; errors: string[] }> {
    try {
      const agents = await listDocs({
        collection: 'agents'
      });

      let updated = 0;
      const errors: string[] = [];

      for (const agentDoc of agents.items) {
        try {
          const agent = agentDoc.data as any;
          
          if (agent.cashBalance === undefined || agent.cashBalance === null) {
            await setDoc({
              collection: 'agents',
              doc: {
                key: agentDoc.key,
                data: {
                  ...agent,
                  cashBalance: 0,
                  digitalBalance: agent.digitalBalance || 0,
                  updatedAt: new Date().toISOString(),
                }
              }
            });
            updated++;
          }
        } catch (error) {
          errors.push(`Failed to update agent ${agentDoc.key}: ${error}`);
        }
      }

      return {
        success: errors.length === 0,
        updated,
        errors
      };
    } catch (error) {
      console.error('Error initializing agent balances:', error);
      return {
        success: false,
        updated: 0,
        errors: [`Failed to initialize: ${error}`]
      };
    }
  }
}
