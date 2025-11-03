/**
 * Demo Mode Configuration
 * 
 * Controlled by VITE_DEMO_MODE environment variable
 * Set to 'true' to use mock data from JSON files
 * Set to 'false' for production with real backend data
 */

export const DEMO_MODE = import.meta.env.VITE_DEMO_MODE === 'true';

/**
 * Demo Data Paths
 */
export const DEMO_DATA = {
  AGENTS: '/data/agents.json',
  USERS: '/data/users.json',
} as const;

/**
 * Demo User Data - Mock user with transactions
 */
export const DEMO_USER = {
  id: 'demo-user-001',
  firstName: 'Demo',
  lastName: 'User',
  email: 'demo@afritokeni.com',
  phone: '+256700000000',
  preferredCurrency: 'UGX',
  kycStatus: 'verified',
  isVerified: true,
  balance: 500000, // 500K UGX
  bitcoinBalance: 0.00125, // ~$50 worth
};

/**
 * Demo Transactions
 */
export const DEMO_TRANSACTIONS = [
  {
    id: 'tx-001',
    type: 'deposit',
    amount: 100000,
    currency: 'UGX',
    status: 'completed',
    description: 'Cash deposit at agent',
    createdAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(), // 2 days ago
  },
  {
    id: 'tx-002',
    type: 'send',
    amount: 50000,
    currency: 'UGX',
    status: 'completed',
    description: 'Sent to John Doe',
    createdAt: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(), // 1 day ago
  },
  {
    id: 'tx-003',
    type: 'receive',
    amount: 75000,
    currency: 'UGX',
    status: 'completed',
    description: 'Received from Jane Smith',
    createdAt: new Date(Date.now() - 12 * 60 * 60 * 1000).toISOString(), // 12 hours ago
  },
  {
    id: 'tx-004',
    type: 'withdraw',
    amount: 30000,
    currency: 'UGX',
    status: 'completed',
    description: 'Cash withdrawal at agent',
    createdAt: new Date(Date.now() - 6 * 60 * 60 * 1000).toISOString(), // 6 hours ago
  },
  {
    id: 'tx-005',
    type: 'deposit',
    amount: 200000,
    currency: 'UGX',
    status: 'pending',
    description: 'Cash deposit pending',
    createdAt: new Date(Date.now() - 1 * 60 * 60 * 1000).toISOString(), // 1 hour ago
  },
];

/**
 * Demo Agent Data
 */
export const DEMO_AGENT = {
  id: 'demo-agent-001',
  userId: 'demo-agent-user-001',
  businessName: 'Demo Agent Services',
  cashBalance: 150000, // Earnings
  digitalBalance: 800000, // Operations fund
  bitcoinBalance: 0.005,
  commissionRate: 0.025,
  status: 'available',
  isActive: true,
};

/**
 * Check if we're in demo mode
 */
export const isDemoMode = () => DEMO_MODE;

/**
 * Get demo data path
 */
export const getDemoDataPath = (key: keyof typeof DEMO_DATA) => {
  return DEMO_DATA[key];
};

/**
 * Get demo user data
 */
export const getDemoUser = () => DEMO_USER;

/**
 * Get demo transactions
 */
export const getDemoTransactions = () => DEMO_TRANSACTIONS;

/**
 * Get demo agent data
 */
export const getDemoAgent = () => DEMO_AGENT;
