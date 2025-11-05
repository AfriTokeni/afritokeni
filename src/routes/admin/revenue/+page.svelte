<script lang="ts">
  import {
    TrendingUp,
    DollarSign,
    CreditCard,
    Banknote,
    ArrowUpRight,
    Info,
    ChevronDown,
  } from "lucide-svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

  let chartDateRange = $state<"30" | "90" | "180">("90");

  // Mock revenue data
  let revenueStats = $state({
    totalRevenue: 45678.5,
    depositCommissions: 28450.3,
    withdrawalFees: 12340.2,
    exchangeSpread: 4888.0,
    growth: 12.5,
  });

  let revenueBreakdown = $state([
    {
      source: "Deposit Commissions",
      amount: 28450.3,
      percentage: 62,
      transactions: 1234,
      color: "blue",
    },
    {
      source: "Withdrawal Fees",
      amount: 12340.2,
      percentage: 27,
      transactions: 856,
      color: "purple",
    },
    {
      source: "Exchange Spread",
      amount: 4888.0,
      percentage: 11,
      transactions: 432,
      color: "green",
    },
  ]);

  let topAgents = $state([
    {
      name: "Agent Lagos Central",
      revenue: 12500,
      transactions: 234,
      commission: 625,
    },
    {
      name: "Agent Nairobi East",
      revenue: 10200,
      transactions: 198,
      commission: 510,
    },
    {
      name: "Agent Accra West",
      revenue: 9800,
      transactions: 176,
      commission: 490,
    },
    {
      name: "Agent Kampala North",
      revenue: 7600,
      transactions: 145,
      commission: 380,
    },
  ]);

  let monthlyRevenue = $state([
    { month: "Oct 2024", revenue: 42340, growth: 8.2 },
    { month: "Sep 2024", revenue: 39150, growth: 12.5 },
    { month: "Aug 2024", revenue: 34820, growth: 15.3 },
    { month: "Jul 2024", revenue: 30200, growth: 9.8 },
  ]);

  // Generate revenue chart data based on date range
  function getRevenueChartData() {
    if (chartDateRange === "30") {
      return {
        categories: ["Oct 5", "Oct 12", "Oct 19", "Oct 26", "Nov 2"],
        totalRevenue: [42000, 43200, 44100, 44800, 45678],
        deposits: [26000, 26800, 27400, 28000, 28450],
        withdrawals: [11800, 12000, 12150, 12250, 12340],
      };
    } else if (chartDateRange === "90") {
      return {
        categories: ["Aug", "Sep", "Oct", "Nov"],
        totalRevenue: [34820, 39150, 42340, 45678],
        deposits: [21200, 24000, 26500, 28450],
        withdrawals: [10500, 11800, 12100, 12340],
      };
    } else {
      return {
        categories: ["Jun", "Jul", "Aug", "Sep", "Oct", "Nov"],
        totalRevenue: [28500, 30200, 34820, 39150, 42340, 45678],
        deposits: [17500, 18500, 21200, 24000, 26500, 28450],
        withdrawals: [8800, 9200, 10500, 11800, 12100, 12340],
      };
    }
  }

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
        data: getRevenueChartData().totalRevenue,
        color: "#3b82f6",
      },
      {
        name: "Deposit Commissions",
        data: getRevenueChartData().deposits,
        color: "#8b5cf6",
      },
      {
        name: "Withdrawal Fees",
        data: getRevenueChartData().withdrawals,
        color: "#22c55e",
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
    legend: { show: true, position: "top" },
  });
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Revenue Overview Cards -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-4">
    <!-- Total Revenue -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-blue-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <DollarSign class="h-5 w-5 text-blue-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900">Total Revenue</p>
              <p class="text-xs text-gray-500">All sources</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          ${revenueStats.totalRevenue.toLocaleString()}
        </span>
      </div>
      <div class="flex items-center space-x-1 border-t border-gray-100 pt-3">
        <TrendingUp class="h-4 w-4 text-green-600" />
        <span class="text-sm font-medium text-green-600"
          >+{revenueStats.growth}%</span
        >
        <span class="text-sm text-gray-500">vs last month</span>
      </div>
    </div>

    <!-- Deposit Commissions -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-purple-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <CreditCard class="h-5 w-5 text-purple-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900">Platform Fees</p>
              <p class="text-xs text-gray-500">0.5% of transactions</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          ${revenueStats.depositCommissions.toLocaleString()}
        </span>
      </div>
      <div class="border-t border-gray-100 pt-3">
        <span class="text-sm text-gray-500">62% of total</span>
      </div>
    </div>

    <!-- Withdrawal Fees -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-green-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <Banknote class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900">Agent Fees</p>
              <p class="text-xs text-gray-500">10% of commissions</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          ${revenueStats.withdrawalFees.toLocaleString()}
        </span>
      </div>
      <div class="border-t border-gray-100 pt-3">
        <span class="text-sm text-gray-500">27% of total</span>
      </div>
    </div>

    <!-- Exchange Spread -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-yellow-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <ArrowUpRight class="h-5 w-5 text-yellow-600 sm:h-6 sm:w-6" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-semibold text-gray-900">
                Exchange Spreads
              </p>
              <p class="text-xs text-gray-500">0.5% on crypto trades</p>
            </div>
          </div>
        </div>
      </div>
      <div class="mb-3">
        <span class="font-mono text-2xl font-bold text-gray-900 sm:text-3xl">
          ${revenueStats.exchangeSpread.toLocaleString()}
        </span>
      </div>
      <div class="border-t border-gray-100 pt-3">
        <span class="text-sm text-gray-500">11% of total</span>
      </div>
    </div>
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

  <!-- Revenue Breakdown -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6 lg:p-8"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Revenue Sources
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">
          Breakdown by transaction type
        </p>
      </div>
    </div>

    <div class="space-y-6">
      {#each revenueBreakdown as item}
        <div>
          <div class="mb-2 flex items-center justify-between">
            <span class="text-sm font-medium text-gray-900">{item.source}</span>
            <div class="text-right">
              <p class="font-mono text-sm font-bold text-gray-900">
                ${item.amount.toLocaleString()}
              </p>
              <p class="text-xs text-gray-500">
                {item.transactions} transactions
              </p>
            </div>
          </div>
          <div class="h-3 w-full overflow-hidden rounded-full bg-gray-100">
            <div
              class="h-full rounded-full bg-{item.color}-500"
              style="width: {item.percentage}%"
            ></div>
          </div>
        </div>
      {/each}
    </div>

    <!-- Info Box -->
    <div
      class="mt-6 flex items-start space-x-2 rounded-lg border border-blue-200 bg-blue-50 p-3"
    >
      <Info class="mt-0.5 h-4 w-4 shrink-0 text-blue-600" />
      <div class="text-xs text-blue-900">
        <p class="font-semibold">Revenue Model:</p>
        <ul class="mt-1 space-y-0.5 text-blue-800">
          <li>• Deposits: 5% commission on all deposit transactions</li>
          <li>• Withdrawals: Fixed fee per transaction</li>
          <li>• Exchange: Spread between buy/sell rates</li>
        </ul>
      </div>
    </div>
  </div>

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
      <a
        href="/admin/agents"
        class="text-sm font-medium text-blue-600 hover:text-blue-700"
      >
        View all →
      </a>
    </div>

    <div class="space-y-3 sm:space-y-4">
      {#each topAgents as agent, i}
        <div
          class="flex items-center justify-between rounded-lg border border-gray-100 p-3 transition-all hover:border-gray-200 sm:p-4"
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
                {agent.transactions} transactions
              </p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-mono text-sm font-bold text-gray-900 sm:text-base">
              ${agent.revenue.toLocaleString()}
            </p>
            <p class="text-xs text-gray-500">Commission: ${agent.commission}</p>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Monthly Revenue History -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Monthly Revenue
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">Historical performance</p>
      </div>
    </div>

    <div class="space-y-3 sm:space-y-4">
      {#each monthlyRevenue as month}
        <div
          class="flex items-center justify-between rounded-lg border border-gray-100 p-3 sm:p-4"
        >
          <div>
            <p class="font-medium text-gray-900">{month.month}</p>
          </div>
          <div class="flex items-center space-x-4">
            <p class="font-mono text-sm font-bold text-gray-900 sm:text-base">
              ${month.revenue.toLocaleString()}
            </p>
            <div class="flex items-center space-x-1">
              <TrendingUp class="h-4 w-4 text-green-600" />
              <span class="text-sm font-medium text-green-600"
                >+{month.growth}%</span
              >
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
