<script lang="ts">
  import { Search, Filter, Download, Activity, Info } from 'lucide-svelte';
  import { onMount } from 'svelte';
  
  let searchQuery = $state('');
  let filterType = $state('all');
  let filterStatus = $state('all');
  
  // Mock transaction data
  let transactions = $state([
    { id: 'TXN-12345', type: 'Deposit', user: 'John Doe', agent: 'Agent Lagos', amount: 500, currency: 'NGN', fee: 25, status: 'completed', timestamp: 'Nov 5, 2024 2:30 PM' },
    { id: 'TXN-12344', type: 'Withdrawal', user: 'Jane Smith', agent: 'Agent Nairobi', amount: 300, currency: 'KES', fee: 15, status: 'pending', timestamp: 'Nov 5, 2024 2:28 PM' },
    { id: 'TXN-12343', type: 'Exchange', user: 'Bob Johnson', agent: null, amount: 1000, currency: 'GHS', fee: 10, status: 'completed', timestamp: 'Nov 5, 2024 2:25 PM' },
    { id: 'TXN-12342', type: 'Deposit', user: 'Alice Brown', agent: 'Agent Accra', amount: 750, currency: 'NGN', fee: 37.5, status: 'completed', timestamp: 'Nov 5, 2024 2:20 PM' },
    { id: 'TXN-12341', type: 'Withdrawal', user: 'Charlie Wilson', agent: 'Agent Kampala', amount: 200, currency: 'KES', fee: 10, status: 'failed', timestamp: 'Nov 5, 2024 2:15 PM' },
  ]);
  
  let stats = $state({
    total: 3542,
    completed: 3201,
    pending: 12,
    failed: 5,
    totalVolume: 1245678
  });
  
  // Simulate real-time updates
  onMount(() => {
    const interval = setInterval(() => {
      const newTx = {
        id: `TXN-${Math.floor(Math.random() * 99999)}`,
        type: ['Deposit', 'Withdrawal', 'Exchange'][Math.floor(Math.random() * 3)],
        user: ['John Doe', 'Jane Smith', 'Bob Johnson'][Math.floor(Math.random() * 3)],
        agent: Math.random() > 0.5 ? 'Agent Lagos' : null,
        amount: Math.floor(Math.random() * 1000) + 100,
        currency: ['NGN', 'KES', 'GHS'][Math.floor(Math.random() * 3)],
        fee: Math.floor(Math.random() * 50) + 5,
        status: 'completed',
        timestamp: new Date().toLocaleString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: 'numeric', minute: '2-digit' })
      };
      transactions = [newTx, ...transactions].slice(0, 20);
      stats.total++;
      stats.completed++;
    }, 10000); // Every 10 seconds
    
    return () => clearInterval(interval);
  });
  
  function getStatusColor(status: string) {
    if (status === 'completed') return 'bg-green-100 text-green-800';
    if (status === 'pending') return 'bg-yellow-100 text-yellow-800';
    if (status === 'failed') return 'bg-red-100 text-red-800';
    return 'bg-gray-100 text-gray-800';
  }
  
  function getTypeColor(type: string) {
    if (type === 'Deposit') return 'bg-blue-100 text-blue-800';
    if (type === 'Withdrawal') return 'bg-purple-100 text-purple-800';
    if (type === 'Exchange') return 'bg-green-100 text-green-800';
    return 'bg-gray-100 text-gray-800';
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-5">
    <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
      <p class="text-sm font-semibold text-gray-500">Total</p>
      <p class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl">{stats.total.toLocaleString()}</p>
    </div>
    <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
      <p class="text-sm font-semibold text-gray-500">Completed</p>
      <p class="mt-2 font-mono text-2xl font-bold text-green-600 sm:text-3xl">{stats.completed.toLocaleString()}</p>
    </div>
    <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
      <p class="text-sm font-semibold text-gray-500">Pending</p>
      <p class="mt-2 font-mono text-2xl font-bold text-yellow-600 sm:text-3xl">{stats.pending}</p>
    </div>
    <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
      <p class="text-sm font-semibold text-gray-500">Failed</p>
      <p class="mt-2 font-mono text-2xl font-bold text-red-600 sm:text-3xl">{stats.failed}</p>
    </div>
    <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
      <p class="text-sm font-semibold text-gray-500">Volume</p>
      <p class="mt-2 font-mono text-2xl font-bold text-blue-600 sm:text-3xl">${stats.totalVolume.toLocaleString()}</p>
    </div>
  </div>
  
  <!-- Filters -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
    <div class="flex flex-wrap gap-4">
      <div class="flex-1 min-w-[200px]">
        <div class="relative">
          <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search by ID, user, or agent..."
            class="w-full rounded-lg border border-gray-200 py-2 pl-10 pr-4 text-sm focus:border-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-600"
          />
        </div>
      </div>
      <select
        bind:value={filterType}
        class="rounded-lg border border-gray-200 px-4 py-2 text-sm focus:border-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-600"
      >
        <option value="all">All Types</option>
        <option value="deposit">Deposits</option>
        <option value="withdrawal">Withdrawals</option>
        <option value="exchange">Exchanges</option>
      </select>
      <select
        bind:value={filterStatus}
        class="rounded-lg border border-gray-200 px-4 py-2 text-sm focus:border-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-600"
      >
        <option value="all">All Status</option>
        <option value="completed">Completed</option>
        <option value="pending">Pending</option>
        <option value="failed">Failed</option>
      </select>
    </div>
  </div>
  
  <!-- Transactions List -->
  <div class="rounded-xl border border-gray-200 bg-white transition-all hover:border-gray-300 sm:rounded-2xl">
    <div class="border-b border-gray-200 p-4 sm:p-6">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-base font-semibold text-gray-900 sm:text-lg">Recent Transactions</h3>
          <p class="text-xs text-gray-500 sm:text-sm">Real-time transaction feed</p>
        </div>
        <div class="flex items-center space-x-2">
          <div class="flex h-2 w-2 animate-pulse rounded-full bg-green-500"></div>
          <span class="text-sm font-medium text-green-600">Live</span>
        </div>
      </div>
    </div>
    
    <div class="p-4 sm:p-6">
      <div class="space-y-3 sm:space-y-4">
        {#each transactions as tx}
          <div class="flex items-center justify-between rounded-lg border border-gray-100 p-3 transition-all hover:border-gray-200 sm:p-4">
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-gray-50 sm:h-12 sm:w-12">
                <Activity class="h-5 w-5 text-gray-600 sm:h-6 sm:w-6" />
              </div>
              <div class="min-w-0">
                <div class="flex items-center space-x-2">
                  <p class="font-mono text-sm font-semibold text-gray-900">{tx.id}</p>
                  <span class="rounded-full px-2 py-1 text-xs font-medium {getTypeColor(tx.type)}">
                    {tx.type}
                  </span>
                </div>
                <p class="mt-1 text-xs text-gray-500">
                  {tx.user} {tx.agent ? `• ${tx.agent}` : ''}
                </p>
                <p class="text-xs text-gray-400">{tx.timestamp}</p>
              </div>
            </div>
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div class="text-right">
                <p class="font-mono text-sm font-bold text-gray-900 sm:text-base">
                  {tx.amount} {tx.currency}
                </p>
                <p class="text-xs text-gray-500">Fee: ${tx.fee}</p>
              </div>
              <span class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(tx.status)}">
                {tx.status}
              </span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Info Box -->
  <div class="flex items-start space-x-2 rounded-lg border border-blue-200 bg-blue-50 p-3 sm:p-4">
    <Info class="mt-0.5 h-4 w-4 shrink-0 text-blue-600" />
    <div class="text-xs text-blue-900 sm:text-sm">
      <p class="font-semibold">Transaction Monitoring:</p>
      <p class="mt-1 text-blue-800">
        This feed updates automatically every 10 seconds. Use filters to narrow down specific transaction types or statuses.
      </p>
    </div>
  </div>
</div>
