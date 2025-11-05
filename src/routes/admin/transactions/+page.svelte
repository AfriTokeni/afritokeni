<script lang="ts">
  import {
    Search,
    Filter,
    Download,
    Activity,
    Info,
    ChevronDown,
    X,
    CheckCircle,
    Clock,
    XCircle,
    User,
    Building2,
    Calendar,
    Hash,
    ArrowRight,
    RefreshCw,
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
  import {
    listTransactions,
    getTransactionStats,
    getTransactionChartData,
  } from "$lib/services/juno/transactionService";
  import { toast } from "$lib/stores/toast";
  import { junoInitialized } from "$lib/stores/auth";
  import type { Transaction, TransactionStats } from "$lib/types/admin";
  import StatCard from "$lib/components/admin/StatCard.svelte";

  // Real transaction data from Juno
  let transactions = $state<Transaction[]>([]);
  let stats = $state<TransactionStats>({
    total: 0,
    pending: 0,
    completed: 0,
    failed: 0,
    volume24h: 0,
    fees24h: 0,
  });
  let lastUpdated = $state(new Date().toLocaleTimeString());
  let isLoading = $state(true);

  // Load data when Juno is initialized
  $effect(() => {
    if ($junoInitialized) {
      loadData();
      loadChartData();
    }
  });

  // Load/refresh data
  async function loadData() {
    try {
      isLoading = true;
      const [newTransactions, newStats] = await Promise.all([
        listTransactions({ limit: 100 }),
        getTransactionStats(),
      ]);
      transactions = newTransactions;
      stats = newStats;
      lastUpdated = new Date().toLocaleTimeString();
    } catch (error) {
      console.error("Failed to load transaction data:", error);
      toast.show("error", "Failed to load transaction data");
    } finally {
      isLoading = false;
    }
  }

  // Load chart data based on selected date range
  async function loadChartData() {
    try {
      const days =
        chartDateRange === "7" ? 7 : chartDateRange === "30" ? 30 : 90;
      const data = await getTransactionChartData(days);

      // Group transactions by type
      const depositVolumes: number[] = [];
      const withdrawalVolumes: number[] = [];
      const exchangeVolumes: number[] = [];

      // For each date, calculate volumes by type
      for (let i = 0; i < data.dates.length; i++) {
        const date = data.dates[i];
        const dayTransactions = transactions.filter((t) => {
          const txDate = new Date(t.createdAt).toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
          });
          return txDate === date && t.status === "completed";
        });

        depositVolumes.push(
          dayTransactions
            .filter((t) => t.type === "deposit")
            .reduce((sum, t) => sum + t.amount, 0),
        );
        withdrawalVolumes.push(
          dayTransactions
            .filter((t) => t.type === "withdrawal")
            .reduce((sum, t) => sum + t.amount, 0),
        );
        exchangeVolumes.push(
          dayTransactions
            .filter((t) => t.type === "exchange")
            .reduce((sum, t) => sum + t.amount, 0),
        );
      }

      chartData = {
        dates: data.dates,
        deposits: depositVolumes,
        withdrawals: withdrawalVolumes,
        exchanges: exchangeVolumes,
      };
    } catch (error) {
      console.error("Failed to load chart data:", error);
    }
  }

  // Reload chart when date range changes
  $effect(() => {
    chartDateRange;
    if (transactions.length > 0) {
      loadChartData();
    }
  });

  // Refresh data
  async function refreshData() {
    toast.show("info", "Refreshing transaction data...");
    await loadData();
    toast.show("success", "Transaction data refreshed");
  }

  let chartDateRange = $state<"7" | "30" | "90">("30");
  let chartData = $state<{
    dates: string[];
    deposits: number[];
    withdrawals: number[];
    exchanges: number[];
  }>({
    dates: [],
    deposits: [],
    withdrawals: [],
    exchanges: [],
  });
  let searchQuery = $state("");
  let activeTab = $state<"all" | "completed" | "pending" | "failed">("all");
  let selectedTransaction = $state<Transaction | null>(null);
  let showDetailModal = $state(false);

  // Pagination state
  let itemsPerPage = 20;
  let displayedCount = $state(itemsPerPage);

  function loadMore() {
    displayedCount += itemsPerPage;
  }

  // Reset displayed count when tab changes
  $effect(() => {
    activeTab;
    displayedCount = itemsPerPage;
  });

  function viewTransaction(txn: any) {
    selectedTransaction = txn;
    showDetailModal = true;
  }

  function closeModal() {
    showDetailModal = false;
    selectedTransaction = null;
  }

  // Transaction volume chart with real data
  let volumeChartOptions = $derived<ApexOptions>({
    chart: {
      height: "320px",
      type: "area",
      fontFamily: "Inter, sans-serif",
      dropShadow: { enabled: false },
      toolbar: { show: false },
    },
    tooltip: { enabled: true, x: { show: false } },
    fill: {
      type: "gradient",
      gradient: {
        opacityFrom: 0.55,
        opacityTo: 0,
      },
    },
    dataLabels: { enabled: false },
    stroke: { width: 2, curve: "smooth" },
    grid: {
      show: true,
      strokeDashArray: 4,
      padding: { left: 2, right: 2, top: 0 },
    },
    series: [
      {
        name: "Deposits",
        data: chartData.deposits,
        color: "#3b82f6",
      },
      {
        name: "Withdrawals",
        data: chartData.withdrawals,
        color: "#8b5cf6",
      },
      {
        name: "Exchanges",
        data: chartData.exchanges,
        color: "#10b981",
      },
    ],
    xaxis: {
      categories: chartData.dates,
      labels: {
        show: true,
        style: {
          fontFamily: "Inter, sans-serif",
          cssClass: "text-xs font-normal fill-gray-500",
        },
      },
      axisBorder: { show: false },
      axisTicks: { show: false },
    },
    yaxis: { show: true },
    legend: { show: true, position: "top" },
  });

  // Simulate real-time updates

  function getStatusColor(status: string) {
    if (status === "completed") return "bg-green-100 text-green-800";
    if (status === "pending") return "bg-yellow-100 text-yellow-800";
    if (status === "failed") return "bg-red-100 text-red-800";
    return "bg-gray-100 text-gray-800";
  }

  function getTypeColor(type: string) {
    if (type === "Deposit") return "bg-blue-100 text-blue-800";
    if (type === "Withdrawal") return "bg-purple-100 text-purple-800";
    if (type === "Exchange") return "bg-green-100 text-green-800";
    return "bg-gray-100 text-gray-800";
  }

  // Filter transactions based on active tab and search
  let filteredTransactions = $derived(
    transactions.filter((txn) => {
      const matchesTab = activeTab === "all" || txn.status === activeTab;
      const matchesSearch =
        !searchQuery ||
        txn.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
        txn.userName.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (txn.agentId &&
          txn.agentId.toLowerCase().includes(searchQuery.toLowerCase()));
      return matchesTab && matchesSearch;
    }),
  );

  // Paginated list
  let displayedTransactions = $derived(
    filteredTransactions.slice(0, displayedCount),
  );

  // Check if there are more items to load
  let hasMore = $derived(displayedCount < filteredTransactions.length);

  // Count by status
  let completedCount = $derived(
    transactions.filter((t) => t.status === "completed").length,
  );
  let pendingCount = $derived(
    transactions.filter((t) => t.status === "pending").length,
  );
  let failedCount = $derived(
    transactions.filter((t) => t.status === "failed").length,
  );
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-5">
    <StatCard
      label="Total"
      value={stats.total}
      {lastUpdated}
      onRefresh={refreshData}
    />
    <StatCard
      label="Completed"
      value={completedCount}
      valueColor="text-green-600"
      onClick={() => (activeTab = "completed")}
    />
    <StatCard
      label="Pending"
      value={pendingCount}
      valueColor="text-yellow-600"
      onClick={() => (activeTab = "pending")}
    />
    <StatCard
      label="Failed"
      value={failedCount}
      valueColor="text-red-600"
      onClick={() => (activeTab = "failed")}
    />
    <StatCard
      label="24h Volume"
      value={`$${stats.volume24h.toLocaleString()}`}
      valueColor="text-blue-600"
    />
  </div>

  <!-- Transaction Volume Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Transaction Volume
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">Volume by type</p>
      </div>
      <div class="relative">
        <Button size="sm" color="light" class="gap-2">
          {chartDateRange === "7"
            ? "Last 7 days"
            : chartDateRange === "30"
              ? "Last 30 days"
              : "Last 3 months"}
          <ChevronDown class="h-4 w-4" />
        </Button>
        <Dropdown class="z-50 w-44 !shadow-md">
          <DropdownItem onclick={() => (chartDateRange = "7")}
            >Last 7 days</DropdownItem
          >
          <DropdownItem onclick={() => (chartDateRange = "30")}
            >Last 30 days</DropdownItem
          >
          <DropdownItem onclick={() => (chartDateRange = "90")}
            >Last 3 months</DropdownItem
          >
        </Dropdown>
      </div>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={volumeChartOptions} />
    </div>
  </div>

  <!-- Tabs -->
  <div class="rounded-xl border border-gray-200 bg-white sm:rounded-2xl">
    <div class="border-b border-gray-200 px-4 sm:px-6">
      <div class="flex items-center justify-between">
        <div class="flex space-x-8">
          <button
            onclick={() => (activeTab = "all")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
            'all'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            All ({transactions.length})
          </button>
          <button
            onclick={() => (activeTab = "completed")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
            'completed'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            Completed ({completedCount})
          </button>
          <button
            onclick={() => (activeTab = "pending")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
            'pending'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            Pending ({pendingCount})
          </button>
          <button
            onclick={() => (activeTab = "failed")}
            class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
            'failed'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
          >
            Failed ({failedCount})
          </button>
        </div>
        <button
          onclick={refreshData}
          class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
          title="Refresh transactions"
        >
          <RefreshCw class="h-4 w-4" />
        </button>
      </div>
    </div>

    <!-- Search Bar -->
    <div class="border-b border-gray-200 px-4 py-3 sm:px-6">
      <div class="relative">
        <Search
          class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400"
        />
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search by ID, user, or agent..."
          class="w-full rounded-lg border border-gray-200 py-2 pr-4 pl-10 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
        />
      </div>
    </div>

    <!-- Tab Content -->
    <div class="p-4 sm:p-6">
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
                    class="rounded-full px-2 py-1 text-xs font-medium {getTypeColor(
                      tx.type,
                    )}"
                  >
                    {tx.type}
                  </span>
                </div>
                <p class="mt-1 text-xs text-gray-500">
                  {tx.userName}
                  {tx.agentId ? `• ${tx.agentId}` : ""}
                </p>
                <p class="text-xs text-gray-400">
                  {new Date(tx.createdAt).toLocaleString()}
                </p>
              </div>
            </div>
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div class="text-right">
                <p
                  class="font-mono text-sm font-bold text-gray-900 sm:text-base"
                >
                  {tx.amount}
                  {tx.currency}
                </p>
                <p class="text-xs text-gray-500">Fee: ${tx.fee}</p>
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

      <!-- Load More Button -->
      {#if hasMore}
        <div class="mt-6 flex justify-center">
          <button
            onclick={loadMore}
            class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-all hover:bg-blue-700"
          >
            Load More ({filteredTransactions.length - displayedCount} remaining)
          </button>
        </div>
      {/if}
    </div>
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
                  class="mt-2 inline-block rounded-full px-3 py-1 text-sm font-semibold {getTypeColor(
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
                  {selectedTransaction.amount}
                  {selectedTransaction.currency}
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
                  ${selectedTransaction.fee}
                </p>
              </div>
              <div>
                <p
                  class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                >
                  Timestamp
                </p>
                <p class="mt-2 text-sm font-medium text-gray-900">
                  {new Date(selectedTransaction.createdAt).toLocaleString()}
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
                    {selectedTransaction.userName}
                  </p>
                </div>
                <ArrowRight class="h-4 w-4 text-gray-400" />
              </button>
              {#if selectedTransaction.agentId}
                <button
                  onclick={() => goto("/admin/agents")}
                  class="flex w-full items-center gap-3 rounded-lg p-3 text-left transition-all hover:bg-purple-50"
                >
                  <Building2 class="h-5 w-5 text-purple-600" />
                  <div class="flex-1">
                    <p class="text-xs text-gray-500">Agent</p>
                    <p
                      class="font-semibold text-purple-600 hover:text-purple-700"
                    >
                      {selectedTransaction.agentId}
                    </p>
                  </div>
                  <ArrowRight class="h-4 w-4 text-gray-400" />
                </button>
              {/if}
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
