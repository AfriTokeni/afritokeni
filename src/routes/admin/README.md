# AfriTokeni Admin Dashboard

## Overview
Complete admin dashboard for monitoring and managing the AfriTokeni platform. Built with Flowbite Svelte components, matching the main site's look and feel.

## Pages Implemented

### 1. Overview Dashboard (`/admin`)
**Features:**
- Real-time stats (users, agents, transactions, revenue)
- Growth metrics with visual indicators
- Pending KYC alerts
- Failed transaction alerts
- System health status
- Recent transactions table
- Top performing agents
- Revenue trend chart (placeholder)

**Mock Data:** All stats and transactions are currently mock data

### 2. KYC Management (`/admin/kyc`)
**Features:**
- Tabbed interface (Pending, Approved, Rejected)
- Pending KYC count badge
- User/Agent KYC requests with documents
- Review modal with document preview
- Approve/Reject functionality
- Rejection reason input
- User avatars and contact info

**Mock Data:** Sample KYC submissions for users and agents

### 3. Revenue Analytics (`/admin/revenue`)
**Features:**
- Total revenue with growth percentage
- Revenue breakdown by source:
  - Deposit commissions (5%)
  - Withdrawal fees
  - Exchange spread
- Visual progress bars for each revenue source
- Top revenue-generating agents
- Monthly revenue history
- Revenue trend chart (placeholder)
- Revenue distribution pie chart (placeholder)
- Time range selector (7d, 30d, 90d, 1y)

**Mock Data:** Revenue stats and agent performance data

### 4. Transaction Monitoring (`/admin/transactions`)
**Features:**
- Real-time transaction feed (simulated updates every 10s)
- Live indicator badge
- Transaction stats overview
- Search by ID, user, or agent
- Filter by type (Deposit, Withdrawal, Exchange)
- Filter by status (Completed, Pending, Failed)
- Detailed transaction table with:
  - Transaction ID
  - Type badge
  - User and agent info
  - Amount and currency
  - Fee
  - Status badge
  - Timestamp
  - Canister ID
- Export functionality (placeholder)

**Mock Data:** Sample transactions with auto-refresh simulation

## Navigation Structure

```
/admin
├── /admin (Overview)
├── /admin/kyc (KYC Management)
├── /admin/revenue (Revenue Analytics)
├── /admin/transactions (Transaction Monitoring)
├── /admin/users (TODO)
├── /admin/agents (TODO)
└── /admin/system (TODO)
```

## Design System

**Colors:**
- Primary: Blue (users, deposits)
- Secondary: Purple (agents)
- Success: Green (completed, approved, revenue)
- Warning: Yellow (pending)
- Danger: Red (failed, rejected)

**Components Used:**
- Flowbite Svelte Cards
- Flowbite Svelte Tables
- Flowbite Svelte Badges
- Flowbite Svelte Modals
- Flowbite Svelte Tabs
- Flowbite Svelte Sidebar
- Flowbite Svelte Navbar
- Flowbite Svelte Buttons
- Flowbite Svelte Icons

## Next Steps: Real Data Integration

### Phase 1: Authentication & Authorization
1. **Add admin check to layout:**
   ```typescript
   // src/routes/admin/+layout.server.ts
   import { getDoc } from '@junobuild/core';
   
   export async function load({ locals }) {
     const user = locals.user; // From Juno auth
     
     if (!user) {
       throw redirect(302, '/login');
     }
     
     const userDoc = await getDoc({
       collection: 'users',
       key: user.key
     });
     
     if (!userDoc?.data?.isAdmin) {
       throw redirect(302, '/');
     }
     
     return { user: userDoc.data };
   }
   ```

2. **Manually set admin flag in Juno:**
   - Go to Juno console
   - Find your user document
   - Add field: `isAdmin: true`

### Phase 2: Connect Real Data Sources

#### Overview Dashboard
```typescript
// Fetch from Juno
const usersCount = await listDocs({ collection: 'users' });
const agentsCount = await listDocs({ collection: 'agents' });

// Fetch from canisters
const depositStats = await depositCanister.get_stats();
const withdrawalStats = await withdrawalCanister.get_stats();
const exchangeStats = await exchangeCanister.get_stats();
```

#### KYC Management
```typescript
// Fetch pending KYC from Juno
const pendingKYC = await listDocs({
  collection: 'kyc_submissions',
  filter: { status: 'pending' }
});

// Approve KYC
await setDoc({
  collection: 'kyc_submissions',
  doc: { key: kycId, data: { ...kyc, status: 'approved' } }
});

// Update user status
await setDoc({
  collection: 'users',
  doc: { key: userId, data: { ...user, kycStatus: 'approved' } }
});
```

#### Revenue Analytics
```typescript
// Fetch from deposit canister
const depositRevenue = await depositCanister.get_total_commission();

// Fetch from withdrawal canister
const withdrawalRevenue = await withdrawalCanister.get_total_fees();

// Fetch from crypto_canister
const exchangeRevenue = await exchangeCanister.get_total_spread();

// Calculate totals
const totalRevenue = depositRevenue + withdrawalRevenue + exchangeRevenue;
```

#### Transaction Monitoring
```typescript
// Fetch recent transactions from all canisters
const deposits = await depositCanister.get_recent_transactions(50);
const withdrawals = await withdrawalCanister.get_recent_transactions(50);
const exchanges = await exchangeCanister.get_recent_transactions(50);

// Merge and sort by timestamp
const allTransactions = [...deposits, ...withdrawals, ...exchanges]
  .sort((a, b) => b.timestamp - a.timestamp);

// Real-time updates using polling
setInterval(async () => {
  const newTxs = await fetchLatestTransactions();
  transactions = [...newTxs, ...transactions];
}, 10000);
```

### Phase 3: Add Charts
Install chart library:
```bash
pnpm add chart.js svelte-chartjs
```

Replace chart placeholders with real charts:
```svelte
<script>
  import { Line, Pie } from 'svelte-chartjs';
  
  const revenueChartData = {
    labels: monthLabels,
    datasets: [{
      label: 'Revenue',
      data: revenueData,
      borderColor: 'rgb(59, 130, 246)',
      backgroundColor: 'rgba(59, 130, 246, 0.1)',
    }]
  };
</script>

<Line data={revenueChartData} />
```

### Phase 4: Add Canister Query Functions

Add these query functions to your Rust canisters:

**deposit_canister:**
```rust
#[query]
fn get_stats() -> DepositStats {
    DepositStats {
        total_deposits: DEPOSITS.with(|d| d.borrow().len() as u64),
        total_commission: calculate_total_commission(),
        pending_count: get_pending_count(),
    }
}

#[query]
fn get_recent_transactions(limit: u64) -> Vec<DepositTransaction> {
    // Return last N transactions
}
```

**withdrawal_canister:**
```rust
#[query]
fn get_stats() -> WithdrawalStats {
    WithdrawalStats {
        total_withdrawals: WITHDRAWALS.with(|w| w.borrow().len() as u64),
        total_fees: calculate_total_fees(),
        pending_count: get_pending_count(),
    }
}
```

**crypto_canister:** (formerly exchange_canister)
```rust
#[query]
fn get_stats() -> ExchangeStats {
    ExchangeStats {
        total_exchanges: EXCHANGES.with(|e| e.borrow().len() as u64),
        total_spread: calculate_total_spread(),
    }
}
```

## TypeScript Errors

The current TypeScript errors about `flowbite-svelte-icons` will resolve after:
1. IDE reload/restart
2. Running `pnpm install` (already done)
3. TypeScript server restart

These are just type definition loading issues, not actual code problems.

## Access

Once authentication is implemented:
1. Login with your ICP identity
2. Ensure your user document in Juno has `isAdmin: true`
3. Navigate to `/admin`
4. Full dashboard access granted

## Security Notes

- Admin routes MUST be protected with auth middleware
- Never expose admin functionality to non-admin users
- All admin actions should be logged
- Consider adding 2FA for admin accounts
- Rate limit admin API calls
- Audit trail for KYC approvals/rejections

## Future Enhancements

- [ ] User management page
- [ ] Agent management page with map
- [ ] System health monitoring
- [ ] Email notifications for KYC decisions
- [ ] Bulk KYC operations
- [ ] Advanced analytics and reports
- [ ] Export to CSV/PDF
- [ ] Real-time WebSocket updates
- [ ] Admin activity logs
- [ ] Role-based permissions (super admin, support, etc.)
