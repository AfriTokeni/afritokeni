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
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

  let chartDateRange = $state<"7" | "30" | "90">("30");
  let searchQuery = $state("");
  let activeTab = $state<"all" | "completed" | "pending" | "failed">("all");
  let selectedTransaction = $state<any>(null);
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

  // Mock transaction data
  let transactions = $state([
    {
      id: "TXN-12345",
      type: "Deposit",
      user: "John Doe",
      agent: "Agent Lagos",
      amount: 500,
      currency: "NGN",
      fee: 25,
      status: "completed",
      timestamp: "Nov 5, 2024 2:30 PM",
    },
    {
      id: "TXN-12344",
      type: "Withdrawal",
      user: "Jane Smith",
      agent: "Agent Nairobi",
      amount: 300,
      currency: "KES",
      fee: 15,
      status: "pending",
      timestamp: "Nov 5, 2024 2:28 PM",
    },
    {
      id: "TXN-12343",
      type: "Exchange",
      user: "Bob Johnson",
      agent: null,
      amount: 1000,
      currency: "GHS",
      fee: 10,
      status: "completed",
      timestamp: "Nov 5, 2024 2:25 PM",
    },
    {
      id: "TXN-12342",
      type: "Deposit",
      user: "Alice Brown",
      agent: "Agent Accra",
      amount: 750,
      currency: "NGN",
      fee: 37.5,
      status: "completed",
      timestamp: "Nov 5, 2024 2:20 PM",
    },
    {
      id: "TXN-12341",
      type: "Withdrawal",
      user: "Charlie Wilson",
      agent: "Agent Kampala",
      amount: 200,
      currency: "KES",
      fee: 10,
      status: "failed",
      timestamp: "Nov 5, 2024 2:15 PM",
    },
  ]);

  let stats = $state({
    total: 3542,
    completed: 3201,
    pending: 12,
    failed: 5,
    totalVolume: 1245678,
  });

  // Generate transaction volume chart data
  function getVolumeChartData() {
    if (chartDateRange === "7") {
      return {
        categories: [
          "Oct 29",
          "Oct 30",
          "Oct 31",
          "Nov 1",
          "Nov 2",
          "Nov 3",
          "Nov 4",
        ],
        deposits: [420, 532, 516, 575, 519, 623, 584],
        withdrawals: [336, 412, 398, 445, 402, 478, 451],
        exchanges: [245, 298, 276, 312, 289, 345, 318],
      };
    } else if (chartDateRange === "30") {
      return {
        categories: [
          "Oct 5",
          "Oct 10",
          "Oct 15",
          "Oct 20",
          "Oct 25",
          "Oct 30",
          "Nov 4",
        ],
        deposits: [2100, 2300, 2450, 2600, 2750, 2900, 3050],
        withdrawals: [1680, 1840, 1960, 2080, 2200, 2320, 2440],
        exchanges: [1225, 1340, 1430, 1520, 1610, 1700, 1790],
      };
    } else {
      return {
        categories: ["Aug", "Sep", "Oct", "Nov"],
        deposits: [6300, 7200, 8100, 9000],
        withdrawals: [5040, 5760, 6480, 7200],
        exchanges: [3675, 4200, 4725, 5250],
      };
    }
  }

  // Transaction volume chart
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
        data: getVolumeChartData().deposits,
        color: "#3b82f6",
      },
      {
        name: "Withdrawals",
        data: getVolumeChartData().withdrawals,
        color: "#8b5cf6",
      },
      {
        name: "Exchanges",
        data: getVolumeChartData().exchanges,
        color: "#10b981",
      },
    ],
    xaxis: {
      categories: getVolumeChartData().categories,
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
  onMount(() => {
    const interval = setInterval(() => {
      const newTx = {
        id: `TXN-${Math.floor(Math.random() * 99999)}`,
        type: ["Deposit", "Withdrawal", "Exchange"][
          Math.floor(Math.random() * 3)
        ],
        user: ["John Doe", "Jane Smith", "Bob Johnson"][
          Math.floor(Math.random() * 3)
        ],
        agent: Math.random() > 0.5 ? "Agent Lagos" : null,
        amount: Math.floor(Math.random() * 1000) + 100,
        currency: ["NGN", "KES", "GHS"][Math.floor(Math.random() * 3)],
        fee: Math.floor(Math.random() * 50) + 5,
        status: "completed",
        timestamp: new Date().toLocaleString("en-US", {
          month: "short",
          day: "numeric",
          year: "numeric",
          hour: "numeric",
          minute: "2-digit",
        }),
      };
      transactions = [newTx, ...transactions].slice(0, 20);
      stats.total++;
      stats.completed++;
    }, 10000); // Every 10 seconds

    return () => clearInterval(interval);
  });

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
        txn.user.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (txn.agent &&
          txn.agent.toLowerCase().includes(searchQuery.toLowerCase()));
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
    <button
      onclick={() => (activeTab = "all")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-gray-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <p class="text-sm font-semibold text-gray-500">Total</p>
      <p class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
        {transactions.length.toLocaleString()}
      </p>
    </button>
    <button
      onclick={() => (activeTab = "completed")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-green-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <p class="text-sm font-semibold text-gray-500">Completed</p>
      <p class="mt-2 font-mono text-2xl font-bold text-green-600 sm:text-3xl">
        {completedCount.toLocaleString()}
      </p>
    </button>
    <button
      onclick={() => (activeTab = "pending")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-yellow-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <p class="text-sm font-semibold text-gray-500">Pending</p>
      <p class="mt-2 font-mono text-2xl font-bold text-yellow-600 sm:text-3xl">
        {pendingCount}
      </p>
    </button>
    <button
      onclick={() => (activeTab = "failed")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-red-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <p class="text-sm font-semibold text-gray-500">Failed</p>
      <p class="mt-2 font-mono text-2xl font-bold text-red-600 sm:text-3xl">
        {failedCount}
      </p>
    </button>
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <p class="text-sm font-semibold text-gray-500">Volume</p>
      <p class="mt-2 font-mono text-2xl font-bold text-blue-600 sm:text-3xl">
        ${stats.totalVolume.toLocaleString()}
      </p>
    </div>
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
                  {tx.user}
                  {tx.agent ? `• ${tx.agent}` : ""}
                </p>
                <p class="text-xs text-gray-400">{tx.timestamp}</p>
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
                  {selectedTransaction.timestamp}
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
              {#if selectedTransaction.agent}
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
                      {selectedTransaction.agent}
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
