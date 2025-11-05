<script lang="ts">
  import {
    TrendingUp,
    TrendingDown,
    Users,
    CreditCard,
    DollarSign,
    Activity,
    Info,
    ChevronDown,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

  let chartDateRange = $state<"7" | "30" | "90">("30");

  // Mock data - will be replaced with real canister/Juno data
  let stats = $state({
    revenue: 45385,
    revenueChange: 4.3,
    users: 2340,
    usersChange: 8.5,
    transactions: 4420,
    transactionsChange: 12.5,
    agents: 89,
    agentsChange: 5.2,
  });

  // Generate revenue chart data based on date range
  function getRevenueChartData() {
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
        revenue: [42000, 43500, 41800, 44200, 43000, 45000, 45385],
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
        revenue: [38000, 40000, 42000, 41000, 43000, 44500, 45385],
      };
    } else {
      return {
        categories: ["Aug", "Sep", "Oct", "Nov"],
        revenue: [35000, 39000, 42000, 45385],
      };
    }
  }

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
        name: "Revenue",
        data: getRevenueChartData().revenue,
        color: "#3b82f6",
      },
    ],
    xaxis: {
      categories: getRevenueChartData().categories,
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

  let topTransactions = $state([
    {
      id: "#FWB127364372",
      user: "Bonnie Green",
      date: "Apr 23, 2021",
      amount: 2300,
      status: "Completed",
    },
    {
      id: "#FWB125467980",
      user: "Jese Leos",
      date: "Apr 23, 2021",
      amount: 5467,
      status: "Completed",
    },
    {
      id: "#FWB139587023",
      user: "Thomas Lean",
      date: "Apr 18, 2021",
      amount: 3902,
      status: "Cancelled",
    },
    {
      id: "#FWB142592348",
      user: "Lana Byrd",
      date: "Apr 15, 2021",
      amount: 203,
      status: "In progress",
    },
  ]);

  function getStatusColor(status: string): string {
    if (status === "Completed") return "bg-green-100 text-green-800";
    if (status === "Cancelled") return "bg-red-100 text-red-800";
    if (status === "In progress") return "bg-yellow-100 text-yellow-800";
    return "bg-gray-100 text-gray-800";
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Stats Grid - 4 columns -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-4">
    <!-- Revenue Card -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between sm:mb-4">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2 sm:mb-3">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-blue-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <DollarSign class="h-5 w-5 text-blue-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900 sm:text-base">
                Revenue
              </p>
              <p class="text-xs text-gray-500">Total earnings</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3 sm:mb-4">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          ${stats.revenue.toLocaleString()}
        </span>
      </div>
      <div class="flex items-center space-x-1 border-t border-gray-100 pt-3">
        {#if stats.revenueChange > 0}
          <TrendingUp class="h-4 w-4 text-green-600" />
          <span class="text-sm font-medium text-green-600"
            >+{stats.revenueChange}%</span
          >
        {:else}
          <TrendingDown class="h-4 w-4 text-red-600" />
          <span class="text-sm font-medium text-red-600"
            >{stats.revenueChange}%</span
          >
        {/if}
        <span class="text-sm text-gray-500">vs last month</span>
      </div>
    </div>

    <!-- Users Card -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between sm:mb-4">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2 sm:mb-3">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-purple-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <Users class="h-5 w-5 text-purple-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900 sm:text-base">
                Total Users
              </p>
              <p class="text-xs text-gray-500">Active accounts</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3 sm:mb-4">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          {stats.users.toLocaleString()}
        </span>
      </div>
      <div class="flex items-center space-x-1 border-t border-gray-100 pt-3">
        <TrendingUp class="h-4 w-4 text-green-600" />
        <span class="text-sm font-medium text-green-600"
          >+{stats.usersChange}%</span
        >
        <span class="text-sm text-gray-500">vs last month</span>
      </div>
    </div>

    <!-- Transactions Card -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between sm:mb-4">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2 sm:mb-3">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-green-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <CreditCard class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900 sm:text-base">
                Transactions
              </p>
              <p class="text-xs text-gray-500">All time</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3 sm:mb-4">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          {stats.transactions.toLocaleString()}
        </span>
      </div>
      <div class="flex items-center space-x-1 border-t border-gray-100 pt-3">
        <TrendingUp class="h-4 w-4 text-green-600" />
        <span class="text-sm font-medium text-green-600"
          >+{stats.transactionsChange}%</span
        >
        <span class="text-sm text-gray-500">vs last month</span>
      </div>
    </div>

    <!-- Agents Card -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between sm:mb-4">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2 sm:mb-3">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-yellow-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <Activity class="h-5 w-5 text-yellow-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900 sm:text-base">
                Active Agents
              </p>
              <p class="text-xs text-gray-500">Network size</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3 sm:mb-4">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          {stats.agents}
        </span>
      </div>
      <div class="flex items-center space-x-1 border-t border-gray-100 pt-3">
        <TrendingUp class="h-4 w-4 text-green-600" />
        <span class="text-sm font-medium text-green-600"
          >+{stats.agentsChange}%</span
        >
        <span class="text-sm text-gray-500">vs last month</span>
      </div>
    </div>
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
        <p class="mb-1 font-semibold">Revenue Sources:</p>
        <ul class="space-y-0.5 text-blue-800">
          <li>• Deposit commissions (5%)</li>
          <li>• Withdrawal fees</li>
          <li>• Exchange spread</li>
        </ul>
      </div>
    </div>
  </div>

  <!-- Latest Transactions -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
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

    <div class="space-y-3 sm:space-y-4">
      {#each topTransactions as tx}
        <div
          class="flex items-center justify-between rounded-lg border border-gray-100 p-3 transition-all hover:border-gray-200 sm:p-4"
        >
          <div class="flex items-center space-x-3 sm:space-x-4">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-gray-50 sm:h-12 sm:w-12"
            >
              <span class="text-sm font-bold text-gray-900"
                >{tx.user.charAt(0)}</span
              >
            </div>
            <div class="min-w-0">
              <p
                class="truncate text-sm font-semibold text-gray-900 sm:text-base"
              >
                {tx.user}
              </p>
              <p class="text-xs text-gray-500 sm:text-sm">
                {tx.id} • {tx.date}
              </p>
            </div>
          </div>
          <div class="flex items-center space-x-3 sm:space-x-4">
            <span
              class="font-mono text-sm font-bold text-gray-900 sm:text-base"
            >
              ${tx.amount.toLocaleString()}
            </span>
            <span
              class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(
                tx.status,
              )}"
            >
              {tx.status}
            </span>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
