import { nanoid } from 'nanoid';
import { setDoc, listDocs } from '@junobuild/core';

export interface PlatformRevenue {
  id: string;
  amount: number;
  currency: string;
  source: 'settlement_fee' | 'liquidity_fee' | 'transaction_fee' | 'exchange_fee' | 'other';
  description?: string;
  agentId?: string;
  userId?: string;
  transactionId?: string;
  createdAt: Date;
}

export interface PlatformRevenueInput {
  amount: number;
  currency: string;
  source: 'settlement_fee' | 'liquidity_fee' | 'transaction_fee' | 'exchange_fee' | 'other';
  description?: string;
  agentId?: string;
  userId?: string;
  transactionId?: string;
}

export class RevenueService {
  static async recordPlatformRevenue(revenueData: PlatformRevenueInput): Promise<PlatformRevenue> {
    try {
      const revenueId = nanoid();
      const now = new Date();

      const revenue: PlatformRevenue = {
        id: revenueId,
        amount: revenueData.amount,
        currency: revenueData.currency,
        source: revenueData.source,
        description: revenueData.description,
        agentId: revenueData.agentId,
        userId: revenueData.userId,
        transactionId: revenueData.transactionId,
        createdAt: now
      };

      const dataForJuno = {
        ...revenue,
        createdAt: now.toISOString()
      };

      await setDoc({
        collection: 'platform_revenue',
        doc: {
          key: revenueId,
          data: dataForJuno
        }
      });

      return revenue;
    } catch (error) {
      console.error('Error recording platform revenue:', error);
      throw error;
    }
  }

  static async getTotalPlatformRevenue(currency?: string): Promise<number> {
    try {
      const docs = await listDocs({
        collection: 'platform_revenue'
      });

      let revenues = docs.items.map(doc => doc.data as any);

      if (currency) {
        revenues = revenues.filter(rev => rev.currency === currency);
      }

      return revenues.reduce((total, rev) => total + rev.amount, 0);
    } catch (error) {
      console.error('Error getting total platform revenue:', error);
      return 0;
    }
  }

  static async getRevenueBySource(source: string, currency?: string): Promise<number> {
    try {
      const docs = await listDocs({
        collection: 'platform_revenue'
      });

      let revenues = docs.items.map(doc => doc.data as any);

      revenues = revenues.filter(rev => rev.source === source);

      if (currency) {
        revenues = revenues.filter(rev => rev.currency === currency);
      }

      return revenues.reduce((total, rev) => total + rev.amount, 0);
    } catch (error) {
      console.error('Error getting revenue by source:', error);
      return 0;
    }
  }

  static async getRevenueByAgent(agentId: string): Promise<number> {
    try {
      const docs = await listDocs({
        collection: 'platform_revenue'
      });

      const revenues = docs.items
        .map(doc => doc.data as any)
        .filter(rev => rev.agentId === agentId);

      return revenues.reduce((total, rev) => total + rev.amount, 0);
    } catch (error) {
      console.error('Error getting revenue by agent:', error);
      return 0;
    }
  }
}
