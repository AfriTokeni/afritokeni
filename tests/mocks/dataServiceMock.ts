/**
 * Mock DataService for USSD tests
 * Intercepts DataService calls and returns test data
 */

import { WebhookDataService } from '../../src/lib/services/webHookServices';
import { listDocs } from '@junobuild/core';
import { CkBTCService } from '../../src/lib/services/ckBTCService';

// Store original methods
const originalGetUserBalance = WebhookDataService.getUserBalance;
const originalListDocs = listDocs;
const originalGetBalance = CkBTCService.getBalance;

// Mock data storage
const mockBalances = new Map<string, number>();
const phoneToUserId = new Map<string, string>();
const mockAgents = [
  {
    id: 'test-agent-1',
    businessName: 'Test Agent Kampala',
    location: {
      city: 'Kampala',
      address: 'Central Road',
      latitude: 0.3476,
      longitude: 32.5825
    },
    phoneNumber: '+256700111111',
    isOnline: true,
    isActive: true,
    cashAvailable: 500000,
    commission: 2.5,
    rating: 4.8
  }
];

export function enableDataServiceMock() {
  // Mock getUserBalance
  WebhookDataService.getUserBalance = async (phoneNumber: string) => {
    // Try with and without + prefix
    const cleanPhone = phoneNumber.replace('+', '');
    const withPlus = phoneNumber.startsWith('+') ? phoneNumber : `+${phoneNumber}`;
    const balance = mockBalances.get(phoneNumber) || mockBalances.get(cleanPhone) || mockBalances.get(withPlus) || 100000;
    return {
      userId: phoneNumber,
      balance,
      currency: 'UGX',
      lastUpdated: new Date()
    };
  };
  
  // Mock getAvailableAgents
  (WebhookDataService as any).getAvailableAgents = async () => {
    return mockAgents;
  };
  
  // Mock findUserByPhoneNumber
  (WebhookDataService as any).findUserByPhoneNumber = async (phoneNumber: string) => {
    return {
      id: phoneNumber,
      phoneNumber,
      firstName: 'Test',
      lastName: 'User',
      email: `${phoneNumber}@test.com`
    };
  };
  
  // Mock verifyUserPin - always return true for tests
  (WebhookDataService as any).verifyUserPin = async (phoneNumber: string, pin: string) => {
    return true;
  };
  
  // Mock processSendMoney - update balances
  (WebhookDataService as any).processSendMoney = async (senderPhone: string, recipientPhone: string, amount: number, fee: number) => {
    const cleanSender = senderPhone.replace('+', '');
    const withPlusSender = senderPhone.startsWith('+') ? senderPhone : `+${senderPhone}`;
    
    // Get current sender balance
    const currentBalance = mockBalances.get(senderPhone) || mockBalances.get(cleanSender) || mockBalances.get(withPlusSender) || 100000;
    
    // Update sender balance in mock
    const newBalance = currentBalance - amount - fee;
    mockBalances.set(senderPhone, newBalance);
    mockBalances.set(cleanSender, newBalance);
    mockBalances.set(withPlusSender, newBalance);
    
    // Also update in BalanceService for the test to see
    const userId = phoneToUserId.get(senderPhone) || phoneToUserId.get(cleanSender) || phoneToUserId.get(withPlusSender);
    if (userId) {
      const { BalanceService } = await import('../../src/services/balanceService');
      try {
        await BalanceService.updateUserBalance(userId, newBalance);
      } catch (e) {
        // Ignore errors
      }
    }
    
    return {
      success: true,
      transactionId: `TXN-${Date.now()}`
    };
  };
  
  // Mock getAvailableAgentsBalance for Bitcoin balance checks
  CkBTCService.getBalance = async (userId: string, useSatellite?: boolean, isDemoMode?: boolean) => {
    return {
      balance: 1000000, // 0.01 BTC in satoshis
      balanceSatoshis: 1000000,
      balanceBTC: '0.01',
      balanceUSD: 600, // Assuming $60k BTC price
      lastUpdated: new Date()
    };
  };
  
  // Mock listDocs to return agents when querying agents collection
  (global as any).listDocs = async (params: any) => {
    if (params.collection === 'agents') {
      return {
        items: mockAgents.map((agent, index) => ({
          key: `agent-${index}`,
          data: agent,
          created_at: BigInt(Date.now()),
          updated_at: BigInt(Date.now()),
          version: BigInt(1)
        })),
        items_length: BigInt(mockAgents.length),
        items_page: BigInt(0),
        matches_length: BigInt(mockAgents.length),
        matches_pages: BigInt(1)
      };
    }
    return originalListDocs(params);
  };
}

export function disableDataServiceMock() {
  WebhookDataService.getUserBalance = originalGetUserBalance;
  CkBTCService.getBalance = originalGetBalance;
  (global as any).listDocs = originalListDocs;
}

export function setMockBalance(phoneNumber: string, balance: number) {
  mockBalances.set(phoneNumber, balance);
}

export function setPhoneToUserId(phoneNumber: string, userId: string) {
  phoneToUserId.set(phoneNumber, userId);
  phoneToUserId.set(phoneNumber.replace('+', ''), userId);
  phoneToUserId.set(phoneNumber.startsWith('+') ? phoneNumber : `+${phoneNumber}`, userId);
}

export function clearMockData() {
  mockBalances.clear();
  phoneToUserId.clear();
}
