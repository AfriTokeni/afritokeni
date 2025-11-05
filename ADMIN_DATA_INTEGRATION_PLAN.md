# Admin Dashboard - Real Data Integration Plan

## Architecture Overview

### Data Sources
1. **Juno DB** - User profiles, KYC documents, agent data, reviews, analytics
2. **Deposit Canister** - Deposit transactions, pending deposits
3. **Withdrawal Canister** - Withdrawal transactions, pending withdrawals
4. **Exchange Canister** - Exchange transactions, rates
5. **ckBTC Ledger** - Bitcoin balances, transfers
6. **ckUSDC Ledger** - USDC balances, transfers
7. **ICP Management** - Canister status, cycles, system health

### Service Layer Structure
```
src/lib/services/
├── juno/
│   ├── kycService.ts          # KYC document CRUD
│   ├── userService.ts         # User profile management
│   ├── agentService.ts        # Agent profile management
│   ├── reviewService.ts       # Review management
│   └── analyticsService.ts    # Analytics aggregation
├── icp/
│   ├── depositService.ts      # Deposit canister calls
│   ├── withdrawalService.ts   # Withdrawal canister calls
│   ├── exchangeService.ts     # Exchange canister calls
│   ├── ledgerService.ts       # ckBTC/ckUSDC ledger calls
│   └── systemService.ts       # Canister health, cycles
└── admin/
    ├── dashboardService.ts    # Aggregate dashboard stats
    ├── transactionService.ts  # Combined transaction data
    └── revenueService.ts      # Revenue calculations
```

---

## Step 1: Data Layer Architecture

### Create Base Service Files

**Priority: HIGH** - Foundation for everything else

#### Files to Create:
1. `src/lib/services/juno/kycService.ts`
2. `src/lib/services/juno/userService.ts`
3. `src/lib/services/juno/agentService.ts`
4. `src/lib/services/juno/reviewService.ts`
5. `src/lib/services/icp/depositService.ts`
6. `src/lib/services/icp/withdrawalService.ts`
7. `src/lib/services/icp/exchangeService.ts`
8. `src/lib/services/icp/ledgerService.ts`
9. `src/lib/services/icp/systemService.ts`
10. `src/lib/services/admin/dashboardService.ts`

#### TypeScript Interfaces:
- Define types for all data structures
- Match Juno collections schema
- Match canister interface types

---

## Step 2: KYC Management Page

**File:** `/admin/kyc/+page.svelte`

### Data Sources:
- **Juno Collection:** `kyc_documents`
- **Juno Collection:** `user_profiles`

### Service File:
`src/lib/services/juno/kycService.ts`

### Functions Needed:
```typescript
// Fetch KYC documents with filters
async function listKYCDocuments(filters: {
  status?: 'pending' | 'approved' | 'rejected';
  searchQuery?: string;
  limit?: number;
  offset?: number;
}): Promise<KYCDocument[]>

// Update KYC status
async function updateKYCStatus(
  docId: string, 
  status: 'approved' | 'rejected',
  adminNotes?: string
): Promise<void>

// Get KYC stats
async function getKYCStats(): Promise<{
  pending: number;
  approved: number;
  rejected: number;
}>
```

### Page Changes:
- Replace mock `kycApplications` with real data from `listKYCDocuments()`
- Wire up approve/reject buttons to `updateKYCStatus()`
- Add real-time updates with Juno subscriptions
- Keep pagination/filtering/search logic

---

## Step 3: Transactions Page

**File:** `/admin/transactions/+page.svelte`

### Data Sources:
- **Deposit Canister:** `list_deposits()`
- **Withdrawal Canister:** `list_withdrawals()`
- **Exchange Canister:** `list_exchanges()`
- **Juno Collection:** `transaction_logs` (for metadata)

### Service File:
`src/lib/services/admin/transactionService.ts`

### Functions Needed:
```typescript
// Aggregate all transaction types
async function listAllTransactions(filters: {
  type?: 'deposit' | 'withdrawal' | 'exchange' | 'all';
  status?: 'pending' | 'completed' | 'failed';
  searchQuery?: string;
  limit?: number;
}): Promise<Transaction[]>

// Get transaction stats
async function getTransactionStats(): Promise<{
  total: number;
  pending: number;
  completed: number;
  failed: number;
  volume24h: number;
}>

// Get transaction details
async function getTransactionDetails(
  txId: string,
  type: 'deposit' | 'withdrawal' | 'exchange'
): Promise<TransactionDetail>
```

### Page Changes:
- Replace mock `transactions` with `listAllTransactions()`
- Fetch real user/agent names from Juno
- Wire up transaction detail modal with real data
- Add real-time updates for pending transactions

---

## Step 4: Users Page

**File:** `/admin/users/+page.svelte`

### Data Sources:
- **Juno Collection:** `user_profiles`
- **ckBTC Ledger:** `icrc1_balance_of()`
- **ckUSDC Ledger:** `icrc1_balance_of()`
- **Juno Collection:** `transaction_logs` (for counts)

### Service File:
`src/lib/services/admin/userService.ts`

### Functions Needed:
```typescript
// List users with balances
async function listUsers(filters: {
  kycStatus?: 'approved' | 'pending' | 'rejected' | 'all';
  searchQuery?: string;
  limit?: number;
}): Promise<UserWithBalance[]>

// Get user stats
async function getUserStats(): Promise<{
  total: number;
  kycApproved: number;
  kycPending: number;
  activeUsers: number;
}>

// Get user activity (for modal)
async function getUserActivity(userId: string): Promise<{
  transactionCount: number;
  feesPaid: number;
  reviewsGiven: number;
}>
```

### Page Changes:
- Replace mock `users` with `listUsers()`
- Fetch real balances from ledgers
- Wire up user modal with real activity data
- Add KYC status management

---

## Step 5: Agents Page

**File:** `/admin/agents/+page.svelte`

### Data Sources:
- **Juno Collection:** `agent_profiles`
- **Juno Collection:** `agent_reviews`
- **Juno Collection:** `transaction_logs` (for commission calc)

### Service File:
`src/lib/services/admin/agentService.ts`

### Functions Needed:
```typescript
// List agents with stats
async function listAgents(filters: {
  status?: 'active' | 'busy' | 'offline' | 'all';
  searchQuery?: string;
  sortBy?: 'joinDate' | 'commission' | 'revenue' | 'rating';
  limit?: number;
}): Promise<AgentWithStats[]>

// Get agent stats
async function getAgentStats(): Promise<{
  total: number;
  active: number;
  busy: number;
  offline: number;
  totalRevenue: number;
}>

// Get agent reviews
async function getAgentReviews(
  agentId: string,
  filters?: { rating?: number }
): Promise<Review[]>

// Ban agent
async function banAgent(agentId: string, reason: string): Promise<void>
```

### Page Changes:
- Replace mock `agents` with `listAgents()`
- Wire up sorting/filtering
- Fetch real reviews for modal
- Implement ban functionality

---

## Step 6: Revenue Page

**File:** `/admin/revenue/+page.svelte`

### Data Sources:
- **All Canisters:** Transaction fees
- **Juno Collection:** `revenue_analytics`
- **Juno Collection:** `agent_commissions`

### Service File:
`src/lib/services/admin/revenueService.ts`

### Functions Needed:
```typescript
// Get revenue overview
async function getRevenueOverview(): Promise<{
  totalRevenue: number;
  monthlyRevenue: number;
  transactionFees: number;
  agentCommissions: number;
}>

// Get revenue by source
async function getRevenueBySource(): Promise<{
  deposits: number;
  withdrawals: number;
  exchanges: number;
}>

// Get revenue trends
async function getRevenueTrends(
  period: '7d' | '30d' | '90d'
): Promise<ChartData>

// Get top revenue agents
async function getTopRevenueAgents(limit: number): Promise<Agent[]>
```

### Page Changes:
- Replace all mock revenue data
- Calculate real fees from transactions
- Aggregate commission data
- Generate real charts

---

## Step 7: System Health Page

**File:** `/admin/system/+page.svelte`

### Data Sources:
- **ICP Management Canister:** Canister status, cycles
- **Juno Analytics:** System logs
- **External APIs:** Service health checks

### Service File:
`src/lib/services/icp/systemService.ts`

### Functions Needed:
```typescript
// Get canister status
async function getCanisterStatus(): Promise<CanisterStatus[]>

// Get canister cycles
async function getCanisterCycles(
  canisterId: string
): Promise<number>

// Get system logs
async function getSystemLogs(filters: {
  level?: 'error' | 'warning' | 'info' | 'all';
  limit?: number;
}): Promise<SystemLog[]>

// Check API health
async function checkAPIHealth(): Promise<APIStatus[]>
```

### Page Changes:
- Replace mock canister data with real status
- Fetch real cycles balance
- Pull actual system logs
- Implement real refresh functionality

---

## Step 8: Admin Dashboard/Overview

**File:** `/admin/dashboard/+page.svelte`

### Data Sources:
- **Aggregate from all services**

### Service File:
`src/lib/services/admin/dashboardService.ts`

### Functions Needed:
```typescript
// Get dashboard overview
async function getDashboardOverview(): Promise<{
  revenue: { total: number; change: number };
  users: { total: number; change: number };
  transactions: { total: number; change: number };
  agents: { total: number; change: number };
}>

// Get latest transactions
async function getLatestTransactions(limit: number): Promise<Transaction[]>

// Get KYC pending count
async function getPendingKYCCount(): Promise<number>

// Get chart data
async function getChartData(
  type: 'revenue' | 'users' | 'transactions',
  period: '7d' | '30d' | '90d'
): Promise<ChartData>
```

### Page Changes:
- Replace all mock stats
- Fetch real latest transactions
- Generate real charts
- Add real-time updates

---

## Implementation Order

### Phase 1: Foundation (Week 1)
1. ✅ Create all service files with TypeScript interfaces
2. ✅ Set up Juno collections schema
3. ✅ Test canister connections
4. ✅ Create error handling utilities

### Phase 2: Core Pages (Week 2)
1. ✅ KYC Management (most critical)
2. ✅ Transactions (high visibility)
3. ✅ Users (frequently used)

### Phase 3: Analytics (Week 3)
1. ✅ Revenue page
2. ✅ Admin Dashboard/Overview
3. ✅ Charts and trends

### Phase 4: Operations (Week 4)
1. ✅ Agents page
2. ✅ System Health
3. ✅ Real-time updates
4. ✅ Performance optimization

---

## Code Organization Rules

### Keep Pages Clean
- **Max 300 lines** per +page.svelte file
- Move data fetching to service files
- Move complex logic to utility functions
- Keep only UI and state management in components

### Service File Pattern
```typescript
// src/lib/services/[domain]/[entity]Service.ts

import { listDocs, setDoc, getDoc } from '@junobuild/core';
import type { Entity } from '$lib/types/[entity]';

export async function list[Entity]s(filters: Filters): Promise<Entity[]> {
  // Implementation
}

export async function get[Entity](id: string): Promise<Entity> {
  // Implementation
}

export async function update[Entity](id: string, data: Partial<Entity>): Promise<void> {
  // Implementation
}
```

### Error Handling
```typescript
import { toast } from '$lib/stores/toast';

try {
  const data = await fetchData();
  return data;
} catch (error) {
  console.error('Error fetching data:', error);
  toast.error('Failed to load data. Please try again.');
  throw error; // Re-throw for caller to handle
}
```

---

## Testing Strategy

1. **Unit Tests** - Test each service function
2. **Integration Tests** - Test canister interactions
3. **E2E Tests** - Test full user flows
4. **Load Tests** - Test with realistic data volumes

---

## Performance Considerations

1. **Pagination** - Always use limit/offset
2. **Caching** - Cache frequently accessed data
3. **Lazy Loading** - Load data on demand
4. **Real-time Updates** - Use Juno subscriptions sparingly
5. **Batch Requests** - Combine multiple canister calls

---

## Next Steps

1. Review and approve this plan
2. Set up Juno collections schema
3. Start with Step 1: Create service files
4. Wire up one page at a time
5. Test thoroughly before moving to next page
