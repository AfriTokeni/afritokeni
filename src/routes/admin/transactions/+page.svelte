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
  import TransactionsTable from "$lib/components/admin/TransactionsTable.svelte";
  import type { RevenueTransaction } from "$lib/services/juno/revenueService";

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
  <TransactionsTable
    transactions={transactions.map((tx) => ({
      id: tx.id,
      user: tx.userName,
      type: tx.type,
      amount: tx.amount,
      fee: tx.fee,
      time: tx.createdAt,
      status: tx.status,
      createdAt: tx.createdAt,
    }))}
    showTabs={true}
    showSearch={true}
    onRefresh={refreshData}
  />
</div>
