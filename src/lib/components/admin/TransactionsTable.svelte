<script lang="ts">
  import {
    Activity,
    ArrowUpDown,
    Search,
    RefreshCw,
    X,
    CheckCircle,
    Clock,
    XCircle,
    User,
    Building2,
    ArrowRight,
  } from "@lucide/svelte";
  import type { RevenueTransaction } from "$lib/services/juno/revenueService";
  import { goto } from "$app/navigation";

  interface Props {
    transactions: RevenueTransaction[];
    onRefresh?: () => void;
    showTabs?: boolean;
    showSearch?: boolean;
  }

  let { 
    transactions, 
    onRefresh,
    showTabs = false,
    showSearch = true,
  }: Props = $props();

  let searchQuery = $state("");
  let activeTab = $state<'all' | 'completed' | 'pending' | 'failed'>('all');
  let sortBy = $state<'type' | 'amount' | 'fee' | 'time'>('time');
  let sortOrder = $state<'asc' | 'desc'>('desc');

  // Filter and sort transactions
  let filteredTransactions = $derived(() => {
    let filtered = transactions;

    // Filter by tab
    if (showTabs && activeTab !== 'all') {
      filtered = filtered.filter(tx => tx.status === activeTab);
    }

    // Filter by search
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(tx =>
        tx.id.toLowerCase().includes(query) ||
        tx.user.toLowerCase().includes(query) ||
        tx.type.toLowerCase().includes(query)
      );
    }

    // Sort
    filtered = [...filtered].sort((a, b) => {
      let aVal, bVal;
      
      switch (sortBy) {
        case 'type':
          aVal = a.type;
          bVal = b.type;
          break;
        case 'amount':
          aVal = a.amount ?? 0;
          bVal = b.amount ?? 0;
          break;
        case 'fee':
          aVal = a.fee ?? 0;
          bVal = b.fee ?? 0;
          break;
        case 'time':
          aVal = new Date(a.createdAt).getTime();
          bVal = new Date(b.createdAt).getTime();
          break;
        default:
          return 0;
      }

      if (aVal < bVal) return sortOrder === 'asc' ? -1 : 1;
      if (aVal > bVal) return sortOrder === 'asc' ? 1 : -1;
      return 0;
    });

    return filtered;
  });

  let displayedTransactions = $derived(filteredTransactions());

  // Tab counts
  let completedCount = $derived(transactions.filter(tx => tx.status === 'completed').length);
  let pendingCount = $derived(transactions.filter(tx => tx.status === 'pending').length);
  let failedCount = $derived(transactions.filter(tx => tx.status === 'failed').length);

  // Modal state
  let showDetailModal = $state(false);
  let selectedTransaction = $state<RevenueTransaction | null>(null);

  function toggleSort(column: typeof sortBy) {
    if (sortBy === column) {
      sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
    } else {
      sortBy = column;
      sortOrder = 'desc';
    }
  }

  function viewTransaction(tx: RevenueTransaction) {
    selectedTransaction = tx;
    showDetailModal = true;
  }

  function closeModal() {
    showDetailModal = false;
    selectedTransaction = null;
  }

  function getTypeColor(type: string) {
    const lowerType = type.toLowerCase();
    if (lowerType === "deposit") return "bg-blue-100 text-blue-800";
    if (lowerType === "withdrawal") return "bg-purple-100 text-purple-800";
    if (lowerType === "exchange") return "bg-green-100 text-green-800";
    return "bg-gray-100 text-gray-800";
  }

  function getStatusColor(status: string) {
    const lowerStatus = status.toLowerCase();
    if (lowerStatus === "completed") return "bg-green-100 text-green-800";
    if (lowerStatus === "pending") return "bg-yellow-100 text-yellow-800";
    if (lowerStatus === "failed") return "bg-red-100 text-red-800";
    return "bg-gray-100 text-gray-800";
  }

  function formatDateTime(dateString: string) {
    const date = new Date(dateString);
    return date.toLocaleString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }
</script>

<div class="rounded-xl border border-gray-200 bg-white sm:rounded-2xl">
  {#if showTabs}
    <!-- Tabs -->
    <div class="border-b border-gray-200 px-4 sm:px-6">
      <div class="flex items-center justify-between">
        <div class="flex space-x-8">
          <button
            onclick={() => (activeTab = "all")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab === 'all'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            All ({transactions.length})
          </button>
          <button
            onclick={() => (activeTab = "completed")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab === 'completed'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            Completed ({completedCount})
          </button>
          <button
            onclick={() => (activeTab = "pending")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab === 'pending'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            Pending ({pendingCount})
          </button>
          <button
            onclick={() => (activeTab = "failed")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab === 'failed'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            Failed ({failedCount})
          </button>
        </div>
        {#if onRefresh}
          <button
            onclick={onRefresh}
            class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
            title="Refresh transactions"
          >
            <RefreshCw class="h-4 w-4" />
          </button>
        {/if}
      </div>
    </div>
  {/if}

  {#if showSearch}
    <!-- Search Bar -->
    <div class="border-b border-gray-200 px-4 py-3 sm:px-6">
      <div class="relative">
        <Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400" />
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search by ID, user, or type..."
          class="w-full rounded-lg border border-gray-200 py-2 pr-4 pl-10 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
        />
      </div>
    </div>
  {/if}

  <!-- Tab Content -->
  <div class="p-4 sm:p-6">
    {#if displayedTransactions.length === 0}
      <!-- Empty State -->
      <div class="py-12 text-center">
        <Activity class="mx-auto h-12 w-12 text-gray-400" />
        <h3 class="mt-4 text-lg font-semibold text-gray-900">No transactions found</h3>
        <p class="mt-2 text-sm text-gray-500">
          {#if searchQuery || (showTabs && activeTab !== 'all')}
            Try adjusting your filters or search query
          {:else}
            Transactions will appear here once users start trading
          {/if}
        </p>
      </div>
    {:else}
      <div class="space-y-3 sm:space-y-4">
        {#each displayedTransactions as tx}
          <button
            onclick={() => viewTransaction(tx)}
            class="flex w-full items-center justify-between rounded-lg border border-gray-100 p-3 text-left transition-all hover:border-blue-400 hover:shadow-md sm:p-4"
          >
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-gray-50 sm:h-12 sm:w-12"
              >
                <Activity class="h-5 w-5 text-gray-600 sm:h-6 sm:w-6" />
              </div>
              <div class="min-w-0">
                <div class="flex items-center space-x-2">
                  <p class="font-mono text-sm font-semibold text-gray-900">
                    {tx.id}
                  </p>
                  <span
                    class="rounded-full px-2 py-1 text-xs font-medium capitalize {getTypeColor(
                      tx.type,
                    )}"
                  >
                    {tx.type}
                  </span>
                </div>
                <p class="mt-1 text-xs text-gray-500">
                  {tx.user}
                </p>
                <p class="text-xs text-gray-400">
                  {formatDateTime(tx.createdAt)}
                </p>
              </div>
            </div>
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div class="text-right">
                <p
                  class="font-mono text-sm font-bold text-gray-900 sm:text-base"
                >
                  ${(tx.amount ?? 0).toLocaleString()}
                </p>
                <p class="text-xs text-gray-500">Fee: ${(tx.fee ?? 0).toLocaleString()}</p>
              </div>
              <span
                class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(
                  tx.status,
                )}"
              >
                {tx.status}
              </span>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Transaction Detail Modal -->
{#if showDetailModal && selectedTransaction}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div
      class="max-h-[95vh] w-full max-w-3xl overflow-y-auto rounded-2xl bg-white shadow-xl"
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-10 border-b border-gray-100 bg-gradient-to-r from-blue-50 to-white px-8 py-6"
      >
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-2xl font-bold text-gray-900">
              Transaction Details
            </h3>
            <p class="mt-1 font-mono text-sm text-gray-500">
              {selectedTransaction.id}
            </p>
          </div>
          <button
            onclick={closeModal}
            class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
          >
            <X class="h-6 w-6" />
          </button>
        </div>
      </div>

      <div class="p-8">
        <div class="space-y-6">
          <!-- Status Badge -->
          <div class="flex items-center justify-between">
            <span class="text-lg font-semibold text-gray-900">Status</span>
            <span
              class="flex items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold {getStatusColor(
                selectedTransaction.status,
              )}"
            >
              {#if selectedTransaction.status === "completed"}
                <CheckCircle class="h-4 w-4" />
              {:else if selectedTransaction.status === "pending"}
                <Clock class="h-4 w-4" />
              {:else}
                <XCircle class="h-4 w-4" />
              {/if}
              {selectedTransaction.status.toUpperCase()}
            </span>
          </div>

          <!-- Transaction Info Grid -->
          <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              Transaction Information
            </h4>
            <div class="grid grid-cols-2 gap-6">
              <div>
                <p
                  class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                >
                  Type
                </p>
                <span
                  class="mt-2 inline-block rounded-full px-3 py-1 text-sm font-semibold capitalize {getTypeColor(
                    selectedTransaction.type,
                  )}"
                >
                  {selectedTransaction.type}
                </span>
              </div>
              <div>
                <p
                  class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                >
                  Amount
                </p>
                <p class="mt-2 font-mono text-lg font-bold text-gray-900">
                  ${(selectedTransaction.amount ?? 0).toLocaleString()}
                </p>
              </div>
              <div>
                <p
                  class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                >
                  Fee
                </p>
                <p
                  class="mt-2 font-mono text-base font-semibold text-green-600"
                >
                  ${(selectedTransaction.fee ?? 0).toLocaleString()}
                </p>
              </div>
              <div>
                <p
                  class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                >
                  Timestamp
                </p>
                <p class="mt-2 text-sm font-medium text-gray-900">
                  {formatDateTime(selectedTransaction.createdAt)}
                </p>
              </div>
            </div>
          </div>

          <!-- User Info -->
          <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              Participant Information
            </h4>
            <div class="space-y-4">
              <button
                onclick={() => goto("/admin/users")}
                class="flex w-full items-center gap-3 rounded-lg p-3 text-left transition-all hover:bg-blue-50"
              >
                <User class="h-5 w-5 text-blue-600" />
                <div class="flex-1">
                  <p class="text-xs text-gray-500">User</p>
                  <p class="font-semibold text-blue-600 hover:text-blue-700">
                    {selectedTransaction.user}
                  </p>
                </div>
                <ArrowRight class="h-4 w-4 text-gray-400" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="border-t border-gray-100 bg-gray-50 px-8 py-6">
        <button
          onclick={closeModal}
          class="w-full rounded-xl bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 font-semibold text-white shadow-lg transition-all hover:from-blue-700 hover:to-blue-800 hover:shadow-xl"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
