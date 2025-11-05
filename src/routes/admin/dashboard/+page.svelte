<script lang="ts">
  import { Info, ChevronDown } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
  import StatCard from "$lib/components/admin/StatCard.svelte";
  import {
    getRevenueStats,
    getRevenueChartData,
    type RevenueTransaction,
  } from "$lib/services/juno/revenueService";
  import { getUserStats } from "$lib/services/juno/userService";
  import {
    getTransactionStats,
    listTransactions,
  } from "$lib/services/juno/transactionService";
  import { getAgentStats } from "$lib/services/juno/agentService";
  import { toast } from "$lib/stores/toast";
  import TransactionsTable from "$lib/components/admin/TransactionsTable.svelte";

  let chartDateRange = $state<"7" | "30" | "90">("30");
  let isLoading = $state(true);

  // Real data from Juno
  let stats = $state({
    revenue: 0,
    revenueChange: 0,
    users: 0,
    usersChange: 0,
    transactions: 0,
    transactionsChange: 0,
    agents: 0,
    agentsChange: 0,
  });

  // Chart data from Juno
  let chartData = $state({
    categories: [] as string[],
    totalRevenue: [] as number[],
    deposits: [] as number[],
    withdrawals: [] as number[],
  });

  // Latest transactions from Juno
  let latestTransactions = $state<RevenueTransaction[]>([]);

  // Load all dashboard data
  async function loadDashboardData() {
    isLoading = true;
    try {
      // Fetch all stats in parallel
      const [
        revenueData,
        userData,
        transactionData,
        agentData,
        chartDataResult,
        transactionsResult,
      ] = await Promise.all([
        getRevenueStats(),
        getUserStats(),
        getTransactionStats(),
        getAgentStats(),
        getRevenueChartData(Number(chartDateRange)),
        listTransactions({ limit: 10 }),
      ]);

      // Convert transactions to RevenueTransaction format
      const revenueTransactions: RevenueTransaction[] = transactionsResult.map(
        (tx) => ({
          id: tx.id,
          user: tx.userName,
          type: tx.type,
          amount: tx.amount,
          fee: tx.fee,
          time: tx.createdAt,
          status: tx.status,
          createdAt: tx.createdAt,
        }),
      );

      // Update stats
      stats = {
        revenue: revenueData.totalRevenue,
        revenueChange: revenueData.totalRevenueChange ?? 0,
        users: userData.total,
        usersChange: userData.totalChange ?? 0,
        transactions: transactionData.total,
        transactionsChange: 0, // Transaction service doesn't provide change yet
        agents: agentData.total,
        agentsChange: 0, // Agent service doesn't provide change yet
      };

      // Update chart data
      chartData = chartDataResult;

      // Update latest transactions
      latestTransactions = revenueTransactions;
    } catch (error) {
      console.error("Error loading dashboard data:", error);
      toast.show("error", "Failed to load dashboard data");
    } finally {
      isLoading = false;
    }
  }

  // Reload chart data when date range changes
  async function updateChartData() {
    try {
      const result = await getRevenueChartData(Number(chartDateRange));
      chartData = result;
    } catch (error) {
      console.error("Error updating chart data:", error);
      toast.show("error", "Failed to update chart");
    }
  }

  // Load data on mount
  onMount(() => {
    loadDashboardData();
  });

  // Revenue trend chart
  let revenueChartOptions = $derived<ApexOptions>({
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
        shade: "#1C64F2",
        gradientToColors: ["#1C64F2"],
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
        name: "Deposits",
        data: chartData.deposits,
        color: "#22c55e",
      },
      {
        name: "Withdrawals",
        data: chartData.withdrawals,
        color: "#f59e0b",
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
    legend: { show: false },
  });
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Stats Grid - 4 columns -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-4">
    <!-- Revenue Card -->
    <StatCard
      label="Revenue"
      subtitle="Total earnings"
      value={`$${stats.revenue.toLocaleString()}`}
      trend={stats.revenueChange}
      onClick={() => goto("/admin/revenue")}
    />

    <!-- Users Card -->
    <StatCard
      label="Total Users"
      subtitle="Active accounts"
      value={stats.users}
      trend={stats.usersChange}
      onClick={() => goto("/admin/users")}
    />

    <!-- Transactions Card -->
    <StatCard
      label="Transactions"
      subtitle="All time"
      value={stats.transactions}
      trend={stats.transactionsChange}
      onClick={() => goto("/admin/transactions")}
    />

    <!-- Agents Card -->
    <StatCard
      label="Active Agents"
      subtitle="Network size"
      value={stats.agents}
      trend={stats.agentsChange}
      onClick={() => goto("/admin/agents")}
    />
  </div>

  <!-- Revenue Trend Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6 lg:p-8"
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
      <Chart options={revenueChartOptions} />
    </div>

    <!-- Info box -->
    <div
      class="mt-4 flex items-start space-x-2 rounded-lg border border-blue-200 bg-blue-50 p-2 sm:p-3"
    >
      <Info class="mt-0.5 h-3 w-3 shrink-0 text-blue-600 sm:h-4 sm:w-4" />
      <div class="text-xs text-blue-900">
        <p class="mb-1 font-semibold">Platform Revenue Sources:</p>
        <ul class="space-y-0.5 text-blue-800">
          <li>• Platform fees (0.5% of all transactions)</li>
          <li>• Agent network fees (10% of agent commissions)</li>
          <li>• Exchange spreads (0.5% on crypto trades)</li>
        </ul>
      </div>
    </div>
  </div>

  <!-- Latest Transactions -->
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Latest Transactions
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">Recent platform activity</p>
      </div>
      <a
        href="/admin/transactions"
        class="text-sm font-medium text-blue-600 hover:text-blue-700"
      >
        View all →
      </a>
    </div>

    {#if isLoading}
      <div
        class="flex items-center justify-center rounded-xl border border-gray-200 bg-white py-12"
      >
        <div class="text-sm text-gray-500">Loading transactions...</div>
      </div>
    {:else}
      <TransactionsTable
        transactions={latestTransactions}
        showTabs={false}
        showSearch={false}
      />
    {/if}
  </div>
</div>
