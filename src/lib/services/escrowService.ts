/**
 * Escrow Service for AfriTokeni
 * 
 * Manages secure escrow transactions for ckBTC and ckUSDC exchanges
 * Protects both users and agents with 6-digit codes and 24hr refunds
 */

import { nanoid } from 'nanoid';
import { setDoc, getDoc, listDocs, SatelliteOptions } from '@junobuild/core';
import { AnonymousIdentity } from '@dfinity/agent';
import { AfricanCurrency } from '../types/currency';

export type AssetType = 'ckBTC' | 'ckUSDC';

export interface EscrowTransaction {
  id: string;
  userId: string;
  agentId: string;
  assetType: AssetType; // ckBTC or ckUSDC
  amount: number; // in satoshis for ckBTC, or smallest unit for ckUSDC
  localAmount: number;
  currency: AfricanCurrency;
  escrowAddress: string; // ICP Principal
  exchangeCode: string;
  status: 'pending' | 'funded' | 'completed' | 'disputed' | 'refunded' | 'expired';
  createdAt: Date;
  expiresAt: Date;
  fundedAt?: Date;
  completedAt?: Date;
  transactionHash?: string;
}

export interface Agent {
  id: string;
  name: string;
  location: string;
  principalId: string; // ICP Principal for ckBTC
  rating: number;
  totalTransactions: number;
  successRate: number;
  fee: number; // percentage
  isActive: boolean;
}

export class EscrowService {
  private static readonly ESCROW_TIMEOUT_HOURS = 24;
  
  // Satellite configuration for Juno
  private static satellite: SatelliteOptions = {
    identity: new AnonymousIdentity(),
    satelliteId: (typeof process !== 'undefined' ? process.env.VITE_DEVELOPMENT_JUNO_SATELLITE_ID : undefined) || "uxrrr-q7777-77774-qaaaq-cai",
    container: true
  };

  /**
   * Create a new escrow transaction for ckBTC or ckUSDC exchange
   */
  static async createEscrowTransaction(
    userId: string,
    agentId: string,
    assetType: AssetType,
    amount: number,
    localAmount: number,
    currency: AfricanCurrency
  ): Promise<EscrowTransaction> {
    // Generate unique escrow address (ICP Principal)
    const escrowAddress = `escrow_${nanoid()}`;
    
    // Generate 6-digit exchange code
    const exchangeCode = this.generateExchangeCode(assetType);
    
    // Calculate expiration time
    const expiresAt = new Date();
    expiresAt.setHours(expiresAt.getHours() + this.ESCROW_TIMEOUT_HOURS);

    const transaction: EscrowTransaction = {
      id: nanoid(),
      userId,
      agentId,
      assetType,
      amount,
      localAmount,
      currency,
      escrowAddress,
      exchangeCode,
      status: 'pending',
      createdAt: new Date(),
      expiresAt,
    };

    // Save to Juno
    await this.saveTransaction(transaction);
    
    // Notify agent of pending transaction
    await this.notifyAgent(agentId, transaction);

    return transaction;
  }

  /**
   * Update escrow transaction status (for testing/admin)
   */
  static async updateTransactionStatus(
    transactionId: string,
    status: 'pending' | 'funded' | 'completed' | 'disputed' | 'refunded' | 'expired',
    fundedAt?: Date
  ): Promise<EscrowTransaction | null> {
    // Get the raw doc with version
    const doc = await getDoc({
      collection: 'escrow_transactions',
      key: transactionId,
      satellite: this.satellite
    });
    
    if (!doc?.data) return null;
    
    const transaction = doc.data as any;
    transaction.status = status;
    if (fundedAt) transaction.fundedAt = fundedAt.toISOString();
    
    // Update with the current version from the doc (Juno auto-increments)
    await setDoc({
      collection: 'escrow_transactions',
      doc: {
        key: transactionId,
        data: transaction,
        version: doc.version // Use current version, Juno will increment it
      },
      satellite: this.satellite
    });
    
    // Return the updated transaction
    return {
      ...transaction,
      createdAt: new Date(transaction.createdAt),
      expiresAt: new Date(transaction.expiresAt),
      fundedAt: transaction.fundedAt ? new Date(transaction.fundedAt) : undefined,
      completedAt: transaction.completedAt ? new Date(transaction.completedAt) : undefined,
    };
  }

  /**
   * Verify exchange code and complete transaction
   */
  static async verifyAndComplete(exchangeCode: string, agentId: string): Promise<EscrowTransaction> {
    const transaction = await this.getTransactionByCode(exchangeCode);
    
    if (!transaction) {
      throw new Error('Invalid exchange code');
    }

    if (transaction.agentId !== agentId) {
      throw new Error('Exchange code does not belong to this agent');
    }

    if (transaction.status !== 'funded') {
      throw new Error('Transaction not funded yet');
    }

    if (new Date() > transaction.expiresAt) {
      throw new Error('Exchange code expired');
    }

    try {
      // Transfer ckBTC from escrow to agent
      // In production, this would call ICP canister
      
      // Use updateTransactionStatus which handles versioning properly
      const updated = await this.updateTransactionStatus(
        transaction.id,
        'completed',
        undefined
      );
      
      if (updated) {
        updated.completedAt = new Date();
      }

      // Update agent stats
      await this.updateAgentStats(agentId, true);

      // Notify both parties
      await this.notifyTransactionComplete(transaction);

      return updated || transaction;
    } catch (error) {
      console.error('Error completing transaction:', error);
      await this.updateTransactionStatus(transaction.id, 'disputed');
      throw error;
    }
  }

  /**
   * Handle expired transactions - refund to user
   */
  static async processExpiredTransactions(): Promise<void> {
    const expiredTransactions = await this.getExpiredTransactions();
    
    for (const transaction of expiredTransactions) {
      if (transaction.status === 'funded') {
        await this.refundTransaction(transaction);
      } else {
        transaction.status = 'expired';
        await this.saveTransaction(transaction);
      }
    }
  }

  /**
   * Refund ckBTC to user
   */
  private static async refundTransaction(transaction: EscrowTransaction): Promise<void> {
    try {
      // Transfer ckBTC back to user
      // In production, this would call ICP canister

      transaction.status = 'refunded';
      await this.saveTransaction(transaction);

      // Notify user of refund
      await this.notifyRefund(transaction);
    } catch (error) {
      console.error('Error refunding transaction:', error);
      transaction.status = 'disputed';
      await this.saveTransaction(transaction);
    }
  }

  /**
   * Generate unique 6-digit exchange code
   */
  private static generateExchangeCode(assetType: AssetType): string {
    const code = Math.floor(100000 + Math.random() * 900000).toString();
    const prefix = assetType === 'ckBTC' ? 'BTC' : 'USD';
    return `${prefix}-${code}`;
  }

  // Juno database operations
  private static async saveTransaction(transaction: EscrowTransaction): Promise<void> {
    await setDoc({
      collection: 'escrow_transactions',
      doc: {
        key: transaction.id,
        data: {
          ...transaction,
          createdAt: transaction.createdAt.toISOString(),
          expiresAt: transaction.expiresAt.toISOString(),
          fundedAt: transaction.fundedAt?.toISOString(),
          completedAt: transaction.completedAt?.toISOString(),
        }
        // No version for new docs, Juno handles it automatically
      },
      satellite: this.satellite
    });
  }

  static async getTransaction(id: string): Promise<(EscrowTransaction & { _version?: bigint }) | null> {
    try {
      const doc = await getDoc({
        collection: 'escrow_transactions',
        key: id,
        satellite: this.satellite
      });
      
      if (!doc?.data) return null;
      
      const data = doc.data as any;
      return {
        ...data,
        createdAt: new Date(data.createdAt),
        expiresAt: new Date(data.expiresAt),
        fundedAt: data.fundedAt ? new Date(data.fundedAt) : undefined,
        completedAt: data.completedAt ? new Date(data.completedAt) : undefined,
        _version: doc.version // Store version for updates
      };
    } catch {
      return null;
    }
  }

  private static async getTransactionByCode(code: string): Promise<EscrowTransaction | null> {
    try {
      const results = await listDocs({
        collection: 'escrow_transactions',
        satellite: this.satellite
      });
      
      const transaction = results.items.find((item: any) => item.data.exchangeCode === code);
      if (!transaction) return null;
      
      const data = transaction.data as any;
      return {
        ...data,
        createdAt: new Date(data.createdAt),
        expiresAt: new Date(data.expiresAt),
        fundedAt: data.fundedAt ? new Date(data.fundedAt) : undefined,
        completedAt: data.completedAt ? new Date(data.completedAt) : undefined,
      };
    } catch {
      return null;
    }
  }

  private static async getExpiredTransactions(): Promise<EscrowTransaction[]> {
    try {
      const results = await listDocs({
        collection: 'escrow_transactions',
        satellite: this.satellite
      });
      
      const now = new Date();
      return results.items
        .map((item: any) => {
          const data = item.data;
          return {
            ...data,
            createdAt: new Date(data.createdAt),
            expiresAt: new Date(data.expiresAt),
            fundedAt: data.fundedAt ? new Date(data.fundedAt) : undefined,
            completedAt: data.completedAt ? new Date(data.completedAt) : undefined,
          };
        })
        .filter((tx: EscrowTransaction) => 
          tx.expiresAt < now && (tx.status === 'pending' || tx.status === 'funded')
        );
    } catch {
      return [];
    }
  }

  // Notification methods - implement with real notification service
  private static async notifyAgent(agentId: string, transaction: EscrowTransaction): Promise<void> {
    console.log(`Notifying agent ${agentId} of new transaction ${transaction.id}`);
  }

  private static async notifyTransactionComplete(transaction: EscrowTransaction): Promise<void> {
    console.log(`Transaction ${transaction.id} completed successfully`);
  }

  private static async notifyRefund(transaction: EscrowTransaction): Promise<void> {
    console.log(`Refunding transaction ${transaction.id} to user ${transaction.userId}`);
  }

  private static async updateAgentStats(_agentId: string, _success: boolean): Promise<void> {
    // Update agent statistics in database
    console.log('Updating agent stats');
  }

  /**
   * Get available agents
   */
  static async getAvailableAgents(): Promise<Agent[]> {
    // In production, fetch from database
    return [
      {
        id: 'agent_1',
        name: 'John Mukasa',
        location: 'Kampala, Uganda',
        principalId: 'principal_mock_1',
        rating: 4.9,
        totalTransactions: 150,
        successRate: 0.98,
        fee: 2.5,
        isActive: true
      },
      {
        id: 'agent_2',
        name: 'Sarah Nakato',
        location: 'Accra, Ghana',
        principalId: 'principal_mock_2',
        rating: 4.8,
        totalTransactions: 89,
        successRate: 0.96,
        fee: 2.0,
        isActive: true
      },
      {
        id: 'agent_3',
        name: 'David Okello',
        location: 'Nairobi, Kenya',
        principalId: 'principal_mock_3',
        rating: 4.7,
        totalTransactions: 67,
        successRate: 0.94,
        fee: 3.0,
        isActive: true
      }
    ];
  }

  /**
   * Get transaction status for user
   */
  static async getUserTransactions(userId: string): Promise<EscrowTransaction[]> {
    try {
      const results = await listDocs({
        collection: 'escrow_transactions',
        satellite: this.satellite
      });
      
      return results.items
        .filter((item: any) => item.data.userId === userId)
        .map((item: any) => {
          const data = item.data;
          return {
            ...data,
            createdAt: new Date(data.createdAt),
            expiresAt: new Date(data.expiresAt),
            fundedAt: data.fundedAt ? new Date(data.fundedAt) : undefined,
            completedAt: data.completedAt ? new Date(data.completedAt) : undefined,
          };
        });
    } catch {
      return [];
    }
  }

  /**
   * Get pending transactions for agent
   */
  static async getAgentTransactions(agentId: string): Promise<EscrowTransaction[]> {
    try {
      const results = await listDocs({
        collection: 'escrow_transactions',
        satellite: this.satellite
      });
      
      return results.items
        .filter((item: any) => item.data.agentId === agentId)
        .map((item: any) => {
          const data = item.data;
          return {
            ...data,
            createdAt: new Date(data.createdAt),
            expiresAt: new Date(data.expiresAt),
            fundedAt: data.fundedAt ? new Date(data.fundedAt) : undefined,
            completedAt: data.completedAt ? new Date(data.completedAt) : undefined,
          };
        });
    } catch {
      return [];
    }
  }
}
