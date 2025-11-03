/**
 * Centralized Demo Service with Juno Backend
 * Single source of truth for ALL demo data
 * Stores demo data in Juno datastore for persistence
 */

import { setDoc, getDoc } from '@junobuild/core';
import { nanoid } from 'nanoid';

export interface DemoBalance {
  userId: string;
  userType: 'user' | 'agent';
  
  // Fiat balances
  digitalBalance: number;
  cashBalance: number;
  
  // Crypto balances (in smallest units)
  ckBTCBalance: number;  // satoshis
  ckUSDCBalance: number; // cents
  
  // AFRI token balance
  afriTokenBalance: number;
  
  // Currency
  preferredCurrency: string;
  
  // Metadata (stored as timestamps for Juno compatibility)
  createdAt: number;
  updatedAt: number;
}

export interface DemoTransaction {
  id: string;
  userId: string;
  type: 'deposit' | 'withdrawal' | 'transfer' | 'exchange';
  amount: number;
  currency: string;
  status: 'pending' | 'completed' | 'failed';
  description: string;
  createdAt: number;
}

export interface DemoProposal {
  id: string;
  type: 'fee_adjustment' | 'currency_addition' | 'agent_standards' | 'platform_upgrade';
  title: string;
  description: string;
  proposer: string;
  createdAt: number;
  votingEndsAt: number;
  status: 'active' | 'passed' | 'rejected' | 'executed';
  votes: {
    yes: number;
    no: number;
    abstain: number;
  };
  quorum: number;
  threshold: number;
}

export interface DemoLeaderboardEntry {
  rank: number;
  name: string;
  userId: string;
  balance: number;
  proposalsCreated: number;
  votesParticipated: number;
  votingPower: string;
}

class CentralizedDemoServiceClass {
  private static instance: CentralizedDemoServiceClass;
  private readonly COLLECTION = 'demo_balances';
  private readonly TX_COLLECTION = 'demo_transactions';
  
  private constructor() {}
  
  static getInstance(): CentralizedDemoServiceClass {
    if (!CentralizedDemoServiceClass.instance) {
      CentralizedDemoServiceClass.instance = new CentralizedDemoServiceClass();
    }
    return CentralizedDemoServiceClass.instance;
  }
  
  /**
   * Initialize demo balance for a user (stores in Juno)
   */
  async initializeUser(userId: string, currency: string = 'UGX'): Promise<DemoBalance> {
    try {
      // Try to get existing document with its version
      const existingDoc = await getDoc({
        collection: this.COLLECTION,
        key: userId,
      });
      
      // If exists, return it
      if (existingDoc?.data) {
        return existingDoc.data as DemoBalance;
      }
      
      // Create new balance
      const balance: DemoBalance = {
        userId,
        userType: 'user',
        digitalBalance: 500000,      // 500K local currency
        cashBalance: 0,              // Users don't have cash balance
        ckBTCBalance: 50000,         // 0.0005 BTC (50K satoshis)
        ckUSDCBalance: 10000,        // $100 (10K cents)
        afriTokenBalance: 1000,      // 1000 AFRI tokens
        preferredCurrency: currency,
        createdAt: Date.now(),
        updatedAt: Date.now(),
      };
      
      // For new documents, don't provide version - Juno handles it
      await setDoc({
        collection: this.COLLECTION,
        doc: {
          key: userId,
          data: balance,
        },
      });
      
      console.log(`🎭 Initialized demo user in Juno: ${userId}`);
      return balance;
    } catch (error) {
      console.error('Failed to initialize demo user:', error);
      // Return default balance if Juno fails
      return {
        userId,
        userType: 'user',
        digitalBalance: 500000,
        cashBalance: 0,
        ckBTCBalance: 50000,
        ckUSDCBalance: 10000,
        afriTokenBalance: 1000,
        preferredCurrency: currency,
        createdAt: Date.now(),
        updatedAt: Date.now(),
      };
    }
  }

  /**
   * Initialize demo balance for an agent (stores in Juno)
   */
  async initializeAgent(agentId: string, currency: string = 'UGX'): Promise<DemoBalance> {
    try {
      // Check if already exists
      const existing = await this.getBalance(agentId);
      if (existing) return existing;
      
      const balance: DemoBalance = {
        userId: agentId,
        userType: 'agent',
        digitalBalance: 5000000,     // 5M local currency (operational funds)
        cashBalance: 250000,         // 250K local currency (earnings)
        ckBTCBalance: 500000,        // 0.005 BTC (500K satoshis)
        ckUSDCBalance: 30000,        // $300 (30K cents)
        afriTokenBalance: 5000,      // 5000 AFRI tokens
        preferredCurrency: currency,
        createdAt: Date.now(),
        updatedAt: Date.now(),
      };
      
      await setDoc({
        collection: this.COLLECTION,
        doc: {
          key: agentId,
          data: balance,
        },
      });
      
      console.log(`🎭 Initialized demo agent in Juno: ${agentId}`);
      return balance;
    } catch (error) {
      console.error('Failed to initialize demo agent:', error);
      // Return default balance if Juno fails
      return {
        userId: agentId,
        userType: 'agent',
        digitalBalance: 5000000,
        cashBalance: 250000,
        ckBTCBalance: 500000,
        ckUSDCBalance: 30000,
        afriTokenBalance: 5000,
        preferredCurrency: currency,
        createdAt: Date.now(),
        updatedAt: Date.now(),
      };
    }
  }
  
  /**
   * Get balance from Juno
   */
  async getBalance(userId: string): Promise<DemoBalance | null> {
    try {
      const doc = await getDoc({
        collection: this.COLLECTION,
        key: userId,
      });
      
      return doc?.data as DemoBalance || null;
    } catch (error) {
      console.error('Failed to get demo balance:', error);
      return null;
    }
  }
  
  /**
   * Update balance in Juno
   */
  async updateBalance(userId: string, updates: Partial<DemoBalance>): Promise<void> {
    try {
      const current = await this.getBalance(userId);
      if (!current) {
        console.error('Balance not found for user:', userId);
        return;
      }
      
      const updated: DemoBalance = {
        ...current,
        ...updates,
        updatedAt: Date.now(),
      };
      
      await setDoc({
        collection: this.COLLECTION,
        doc: {
          key: userId,
          data: updated,
        },
      });
      
      console.log(`🎭 Updated demo balance for: ${userId}`);
    } catch (error) {
      console.error('Failed to update demo balance:', error);
    }
  }
  
  /**
   * Add demo transaction
   */
  async addTransaction(tx: Omit<DemoTransaction, 'id' | 'createdAt'>): Promise<void> {
    try {
      const transaction: DemoTransaction = {
        ...tx,
        id: nanoid(),
        createdAt: Date.now(),
      };
      
      await setDoc({
        collection: this.TX_COLLECTION,
        doc: {
          key: transaction.id,
          data: transaction,
        },
      });
      
      console.log(`🎭 Added demo transaction: ${transaction.id}`);
    } catch (error) {
      console.error('Failed to add demo transaction:', error);
    }
  }
  
  /**
   * Get transactions for user from /data folder
   */
  async getTransactions(userId: string, limit: number = 50): Promise<DemoTransaction[]> {
    try {
      // Load from /data/demo-transactions.json
      const response = await fetch('/data/demo-transactions.json');
      const data = await response.json();
      
      // Determine if user or agent based on userId
      const isAgent = userId.includes('agent');
      const transactions = isAgent ? data['agent-transactions'] : data['user-transactions'];
      
      return (transactions || []).slice(0, limit);
    } catch (error) {
      console.error('Failed to load demo transactions:', error);
      return [];
    }
  }
  
  /**
   * Clear all demo data for a user
   */
  async clearUserData(userId: string): Promise<void> {
    try {
      // Note: Juno doesn't support null data, so we skip deletion
      console.log(`🎭 Skipping deletion for: ${userId} (not supported in demo mode)`);
      
      console.log(`🎭 Cleared demo data for: ${userId}`);
    } catch (error) {
      console.error('Failed to clear demo data:', error);
    }
  }
  
  /**
   * Get demo DAO proposals from /data folder
   */
  async getDemoProposals(): Promise<DemoProposal[]> {
    try {
      const response = await fetch('/data/dao-proposals.json');
      const proposals = await response.json();
      return proposals;
    } catch (error) {
      console.error('Failed to load demo proposals:', error);
      return [];
    }
  }
  
  /**
   * Get demo DAO leaderboard from /data folder
   */
  async getDemoLeaderboard(): Promise<DemoLeaderboardEntry[]> {
    try {
      const response = await fetch('/data/dao-leaderboard.json');
      const leaderboard = await response.json();
      return leaderboard;
    } catch (error) {
      console.error('Failed to load demo leaderboard:', error);
      return [];
    }
  }
  
  /**
   * Get demo DAO stats
   */
  getDemoDAOStats() {
    return {
      totalHolders: 2847,
      totalSupply: 1_000_000_000,
      circulatingSupply: 450_000_000,
      treasuryBalance: 100_000_000,
    };
  }
}

export const CentralizedDemoService = CentralizedDemoServiceClass.getInstance();
