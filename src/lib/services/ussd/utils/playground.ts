/**
 * Playground Mode Utilities
 * Provides mock data for USSD Playground to avoid hitting production Juno canister
 */

/**
 * Check if we're in playground mode
 * Playground sessions have sessionId starting with 'playground_'
 */
export function isPlaygroundMode(sessionId: string): boolean {
  return sessionId.startsWith('playground_');
}

/**
 * Get mock user balance for playground
 */
export function getMockBalance(): number {
  return 50000; // 50,000 UGX
}

/**
 * Get mock agents for playground
 */
export function getMockAgents() {
  return [
    {
      id: 'agent_001',
      userId: 'user_agent_001',
      businessName: 'Kampala Central Agent',
      location: {
        country: 'Uganda',
        state: 'Central',
        city: 'Kampala',
        address: 'Plot 123, Kampala Road',
        coordinates: { lat: 0.3476, lng: 32.5825 }
      },
      isActive: true,
      status: 'available' as const,
      cashBalance: 5000000,
      digitalBalance: 10000000,
      commissionRate: 0.02,
      createdAt: new Date()
    },
    {
      id: 'agent_002',
      userId: 'user_agent_002',
      businessName: 'Entebbe Agent Services',
      location: {
        country: 'Uganda',
        state: 'Central',
        city: 'Entebbe',
        address: 'Airport Road, Entebbe',
        coordinates: { lat: 0.0640, lng: 32.4435 }
      },
      isActive: true,
      status: 'available' as const,
      cashBalance: 3000000,
      digitalBalance: 8000000,
      commissionRate: 0.025,
      createdAt: new Date()
    },
    {
      id: 'agent_003',
      userId: 'user_agent_003',
      businessName: 'Jinja Money Exchange',
      location: {
        country: 'Uganda',
        state: 'Eastern',
        city: 'Jinja',
        address: 'Main Street, Jinja',
        coordinates: { lat: 0.4244, lng: 33.2040 }
      },
      isActive: true,
      status: 'available' as const,
      cashBalance: 2000000,
      digitalBalance: 6000000,
      commissionRate: 0.03,
      createdAt: new Date()
    }
  ];
}

/**
 * Get mock transactions for playground
 */
export function getMockTransactions() {
  return [
    {
      id: 'tx_001',
      userId: 'demo_user',
      type: 'receive' as const,
      amount: 25000,
      fee: 0,
      currency: 'UGX' as const,
      status: 'completed' as const,
      description: 'Received money from +256700999888',
      createdAt: new Date(Date.now() - 86400000), // 1 day ago
      completedAt: new Date(Date.now() - 86400000)
    },
    {
      id: 'tx_002',
      userId: 'demo_user',
      type: 'send' as const,
      amount: 10000,
      fee: 100,
      currency: 'UGX' as const,
      status: 'completed' as const,
      description: 'Sent money to +256700111222',
      createdAt: new Date(Date.now() - 172800000), // 2 days ago
      completedAt: new Date(Date.now() - 172800000)
    },
    {
      id: 'tx_003',
      userId: 'demo_user',
      type: 'deposit' as const,
      amount: 50000,
      fee: 0,
      currency: 'UGX' as const,
      status: 'completed' as const,
      description: 'Cash deposit via agent',
      createdAt: new Date(Date.now() - 259200000), // 3 days ago
      completedAt: new Date(Date.now() - 259200000)
    }
  ];
}
