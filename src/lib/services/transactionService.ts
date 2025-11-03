import { nanoid } from 'nanoid';
import { getDoc, setDoc, listDocs } from '@junobuild/core';
import { FraudDetectionService } from './fraudDetection';
import { RateLimiter } from './rateLimiter';

export interface Transaction {
  id: string;
  userId: string;
  type: 'send' | 'receive' | 'withdraw' | 'deposit' | 'bitcoin_buy' | 'bitcoin_sell' | 'bitcoin_to_ugx' | 'ugx_to_bitcoin' | 'bitcoin_send' | 'bitcoin_receive';
  amount: number;
  fee?: number;
  currency: string;
  recipientId?: string;
  recipientPhone?: string;
  recipientName?: string;
  agentId?: string;
  fromUserId?: string;
  toUserId?: string;
  status: 'pending' | 'completed' | 'failed' | 'cancelled' | 'confirmed';
  smsCommand?: string;
  description?: string;
  createdAt: Date;
  updatedAt?: Date;
  completedAt?: Date;
  withdrawalCode?: string;
  depositCode?: string;
  bitcoinAddress?: string;
  exchangeRate?: number;
  location?: string;
  metadata?: any;
}

export class TransactionService {
  static async createTransaction(transaction: Omit<Transaction, 'id' | 'createdAt'>): Promise<Transaction> {
    const fraudCheck = await FraudDetectionService.checkTransaction(
      transaction.recipientPhone || transaction.userId,
      transaction.amount,
      transaction.recipientPhone || transaction.recipientId || 'unknown'
    );

    if (fraudCheck.isSuspicious && fraudCheck.requiresVerification) {
      throw new Error(`Transaction blocked: ${fraudCheck.reason}`);
    }

    const rateLimitCheck = RateLimiter.isAllowed(transaction.userId, 'transaction');
    if (!rateLimitCheck.allowed) {
      throw new Error(rateLimitCheck.message || 'Rate limit exceeded');
    }

    const now = new Date();
    const newTransaction: Transaction = {
      ...transaction,
      id: nanoid(),
      createdAt: now
    };

    const dataForJuno = {
      ...newTransaction,
      createdAt: now.toISOString(),
      updatedAt: newTransaction.updatedAt?.toISOString(),
      completedAt: newTransaction.completedAt?.toISOString()
    };

    await setDoc({
      collection: 'transactions',
      doc: {
        key: newTransaction.id,
        data: dataForJuno
      }
    });

    return newTransaction;
  }

  static async getTransaction(id: string): Promise<Transaction | null> {
    try {
      const doc = await getDoc({
        collection: 'transactions',
        key: id
      });

      if (!doc?.data) return null;

      const data = doc.data as any;
      return {
        ...data,
        createdAt: new Date(data.createdAt),
        updatedAt: data.updatedAt ? new Date(data.updatedAt) : undefined,
        completedAt: data.completedAt ? new Date(data.completedAt) : undefined
      };
    } catch (error) {
      console.error('Error getting transaction:', error);
      return null;
    }
  }

  static async getUserTransactions(userId: string): Promise<Transaction[]> {
    try {
      const docs = await listDocs({
        collection: 'transactions'
      });

      return docs.items
        .map(doc => doc.data as Transaction)
        .filter(transaction => transaction.userId === userId)
        .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
    } catch (error) {
      console.error('Error getting user transactions:', error);
      return [];
    }
  }

  static async getAgentTransactions(agentId: string): Promise<Transaction[]> {
    try {
      const docs = await listDocs({
        collection: 'transactions'
      });

      return docs.items
        .map(doc => doc.data as Transaction)
        .filter(transaction => transaction.agentId === agentId)
        .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
    } catch (error) {
      console.error('Error getting agent transactions:', error);
      return [];
    }
  }

  static async updateTransaction(id: string, updates: Partial<Transaction>): Promise<boolean> {
    try {
      const existing = await this.getTransaction(id);
      if (!existing) return false;

      const updated = { ...existing, ...updates };
      const dataForJuno = {
        ...updated,
        createdAt: updated.createdAt.toISOString(),
        updatedAt: updated.updatedAt?.toISOString(),
        completedAt: updated.completedAt?.toISOString()
      };

      const existingDoc = await getDoc({
        collection: 'transactions',
        key: id
      });

      await setDoc({
        collection: 'transactions',
        doc: {
          key: id,
          data: dataForJuno,
          version: existingDoc?.version || 1n
        }
      });

      return true;
    } catch (error) {
      console.error('Error updating transaction:', error);
      return false;
    }
  }

  static calculateAgentDailyEarnings(transactions: Transaction[]): {
    totalAmount: number;
    totalCommission: number;
    transactionCount: number;
    completedCount: number;
  } {
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const tomorrow = new Date(today);
    tomorrow.setDate(tomorrow.getDate() + 1);

    const todayTransactions = transactions.filter(transaction => {
      const transactionDate = new Date(transaction.createdAt);
      return transactionDate >= today && transactionDate < tomorrow;
    });

    const completedTransactions = todayTransactions.filter(
      transaction => transaction.status === 'completed'
    );

    const totalAmount = completedTransactions.reduce((sum, transaction) => sum + transaction.amount, 0);
    const totalCommission = completedTransactions.reduce((sum, transaction) => {
      if (transaction.type === 'withdraw') {
        return sum + (transaction.fee || 0);
      }
      return sum + (transaction.amount * 0.02);
    }, 0);

    return {
      totalAmount,
      totalCommission,
      transactionCount: todayTransactions.length,
      completedCount: completedTransactions.length
    };
  }

  static async processWithdrawal(transactionId: string): Promise<boolean> {
    try {
      await this.updateTransaction(transactionId, {
        status: 'completed',
        completedAt: new Date()
      });
      return true;
    } catch (error) {
      console.error('Error processing withdrawal:', error);
      return false;
    }
  }
}
