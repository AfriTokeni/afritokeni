<script lang="ts">
  import {
    TrendingUp,
    DollarSign,
    CreditCard,
    Banknote,
    ArrowUpRight,
    Info,
    ChevronDown,
    ArrowRight,
    Search,
    RefreshCw,
    ArrowUpDown,
    Users,
    Activity,
  } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
  import StatCard from "$lib/components/admin/StatCard.svelte";
  import SearchBar from "$lib/components/admin/SearchBar.svelte";
  import TransactionsTable from "$lib/components/admin/TransactionsTable.svelte";
  import { listAgents } from "$lib/services/juno/agentService";
  import type { AgentProfile } from "$lib/types/admin";
  import {
    getRevenueStats,
    getRevenueChartData,
    getRevenueTransactions,
    type RevenueStats,
    type RevenueTransaction,
  } from "$lib/services/juno/revenueService";
  import { junoInitialized } from "$lib/stores/auth";
  import { toast } from "$lib/stores/toast";

  let chartDateRange = $state<"30" | "90" | "180">("90");
  let isLoading = $state(false);
  let lastUpdated = $state("");

  // Real revenue data from Juno
  let revenueStats = $state<RevenueStats>({
    totalRevenue: 0,
    depositCommissions: 0,
    withdrawalFees: 0,
    exchangeSpread: 0,
    totalRevenueChange: 0,
    depositCommissionsChange: 0,
    withdrawalFeesChange: 0,
    exchangeSpreadChange: 0,
  });

  let recentTransactions = $state<RevenueTransaction[]>([]);
  let topAgents = $state<AgentProfile[]>([]);

  let chartData = $state({
    categories: [] as string[],
    totalRevenue: [] as number[],
    deposits: [] as number[],
    withdrawals: [] as number[],
  });

  // Load data from Juno
  async function loadData() {
    if (isLoading) return;

    isLoading = true;
    try {
      const [stats, chart, transactions, agents] = await Promise.all([
        getRevenueStats(),
        getRevenueChartData(parseInt(chartDateRange)),
        getRevenueTransactions({ limit: 20 }),
        listAgents({ limit: 4 }),
      ]);

      revenueStats = stats;
      chartData = chart;
      recentTransactions = transactions;
      topAgents = agents
        .sort((a, b) => (b.revenue ?? 0) - (a.revenue ?? 0))
        .slice(0, 4);
      lastUpdated = new Date().toLocaleTimeString();
    } catch (error) {
      console.error("Error loading revenue data:", error);
      toast.show("error", "Failed to load revenue data");
    } finally {
      isLoading = false;
    }
  }

  // Refresh data
  function refreshData() {
    loadData();
  }

  // Load data when Juno is initialized or filters change
  $effect(() => {
    if ($junoInitialized) {
      chartDateRange;
      loadData();
    }
  });

  // Revenue trend chart
  let revenueChartOptions = $derived<ApexOptions>({
    chart: {
      height: "350px",
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
        name: "Total Revenue",
        data: chartData.totalRevenue,
        color: "#3b82f6",
      },
      {
        name: "Deposit Commissions",
        data: chartData.deposits,
        color: "#8b5cf6",
      },
      {
        name: "Withdrawal Fees",
        data: chartData.withdrawals,
        color: "#22c55e",
      },
    ],
    xaxis: {
      categories: chartData.categories,
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
    yaxis: {
      show: true,
      labels: {
        formatter: (value) => "$" + value.toLocaleString(),
      },
    },
    legend: { show: true, position: "top" },
  });
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Revenue Overview Cards -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-4">
    <!-- Total Revenue -->
    <StatCard
      label="Total Revenue"
      value={`$${revenueStats.totalRevenue.toLocaleString()}`}
      trend={revenueStats.totalRevenueChange}
      onRefresh={refreshData}
      {lastUpdated}
    />

    <!-- Deposit Commissions -->
    <StatCard
      label="Platform Fees"
      value={`$${revenueStats.depositCommissions.toLocaleString()}`}
      trend={revenueStats.depositCommissionsChange}
    />

    <!-- Withdrawal Fees -->
    <StatCard
      label="Agent Fees"
      value={`$${revenueStats.withdrawalFees.toLocaleString()}`}
      trend={revenueStats.withdrawalFeesChange}
    />

    <!-- Exchange Spread -->
    <StatCard
      label="Exchange Spreads"
      value={`$${revenueStats.exchangeSpread.toLocaleString()}`}
      trend={revenueStats.exchangeSpreadChange}
    />
  </div>

  <!-- Revenue Trend Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Revenue Trend
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">Revenue over time</p>
      </div>
      <div class="relative">
        <Button size="sm" color="light" class="gap-2">
          {chartDateRange === "30"
            ? "Last 30 days"
            : chartDateRange === "90"
              ? "Last 3 months"
              : "Last 6 months"}
          <ChevronDown class="h-4 w-4" />
        </Button>
        <Dropdown class="z-50 w-44 !shadow-md">
          <DropdownItem onclick={() => (chartDateRange = "30")}
            >Last 30 days</DropdownItem
          >
          <DropdownItem onclick={() => (chartDateRange = "90")}
            >Last 3 months</DropdownItem
          >
          <DropdownItem onclick={() => (chartDateRange = "180")}
            >Last 6 months</DropdownItem
          >
        </Dropdown>
      </div>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={revenueChartOptions} />
    </div>
  </div>

  <!-- Recent Revenue Transactions -->
  <TransactionsTable
    transactions={recentTransactions}
    showSearch={true}
    showTabs={true}
    onRefresh={refreshData}
  />

  <!-- Top Revenue Agents -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Top Revenue Agents
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">Highest earning agents</p>
      </div>
      <button
        onclick={() => goto("/admin/agents")}
        class="flex items-center gap-1 text-sm font-medium text-blue-600 transition-colors hover:text-blue-700"
      >
        View all
        <ArrowRight class="h-4 w-4" />
      </button>
    </div>

    {#if topAgents.length === 0}
      <div class="py-12 text-center">
        <Users class="mx-auto h-12 w-12 text-gray-400" />
        <h3 class="mt-4 text-lg font-semibold text-gray-900">
          No agents found
        </h3>
        <p class="mt-2 text-sm text-gray-500">
          Top revenue agents will appear here once they start earning
        </p>
      </div>
    {:else}
      <div class="space-y-3 sm:space-y-4">
        {#each topAgents as agent, i}
          <button
            onclick={() => goto("/admin/agents")}
            class="flex w-full items-center justify-between rounded-lg border border-gray-100 p-3 text-left transition-all hover:border-blue-400 hover:shadow-md sm:p-4"
          >
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-lg font-bold text-blue-600 sm:h-12 sm:w-12"
              >
                {i + 1}
              </div>
              <div class="min-w-0">
                <p
                  class="truncate text-sm font-semibold text-gray-900 sm:text-base"
                >
                  {agent.name}
                </p>
                <p class="text-xs text-gray-500">
                  {agent.transactionCount ?? 0} transactions
                </p>
              </div>
            </div>
            <div class="text-right">
              <p class="font-mono text-sm font-bold text-gray-900 sm:text-base">
                ${(agent.revenue ?? 0).toLocaleString()}
              </p>
              <p class="text-xs text-gray-500">
                Commission: ${(agent.commission ?? 0).toLocaleString()}
              </p>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>
