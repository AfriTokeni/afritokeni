<script lang="ts">
  import {
    Activity,
    AlertCircle,
    CheckCircle,
    Server,
    Database,
    Zap,
    Info,
    ChevronDown,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

  let chartDateRange = $state<"7" | "30" | "90">("30");

  // Mock system data
  let canisters = $state([
    {
      name: "Deposit Canister",
      id: "453vh-eqaaa-aaaac-qctia-cai",
      cycles: 5.2,
      status: "healthy",
      uptime: "99.9%",
    },
    {
      name: "Withdrawal Canister",
      id: "422tt-jiaaa-aaaac-qctiq-cai",
      cycles: 4.8,
      status: "healthy",
      uptime: "99.8%",
    },
    {
      name: "Exchange Canister",
      id: "4tzyp-7aaaa-aaaac-qctja-cai",
      cycles: 3.9,
      status: "warning",
      uptime: "99.5%",
    },
  ]);

  let errorLogs = $state([
    {
      timestamp: "Nov 5, 2024 2:45 PM",
      level: "error",
      message: "Failed to process withdrawal TXN-12341",
      canister: "Withdrawal Canister",
    },
    {
      timestamp: "Nov 5, 2024 1:30 PM",
      level: "warning",
      message: "Low cycles detected on Exchange Canister",
      canister: "Exchange Canister",
    },
    {
      timestamp: "Nov 5, 2024 12:15 PM",
      level: "info",
      message: "Canister upgrade completed successfully",
      canister: "Deposit Canister",
    },
  ]);

  let apiStatus = $state([
    { name: "Juno DB", status: "operational", responseTime: "45ms" },
    { name: "ICP Network", status: "operational", responseTime: "120ms" },
    { name: "SMS Gateway", status: "operational", responseTime: "230ms" },
    { name: "Email Service", status: "operational", responseTime: "180ms" },
  ]);

  let systemHealth = $state({
    overall: "healthy",
    uptime: "99.8%",
    totalCycles: 13.9,
    lastDeployment: "Nov 4, 2024 3:20 PM",
  });

  // Generate cycles chart data based on date range
  function getCyclesChartData() {
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
        deposit: [5.8, 5.6, 5.5, 5.4, 5.3, 5.2, 5.2],
        withdrawal: [5.2, 5.1, 5.0, 4.9, 4.9, 4.8, 4.8],
        exchange: [4.5, 4.3, 4.2, 4.1, 4.0, 3.9, 3.9],
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
        deposit: [6.5, 6.2, 5.9, 5.7, 5.5, 5.3, 5.2],
        withdrawal: [5.8, 5.6, 5.4, 5.2, 5.0, 4.9, 4.8],
        exchange: [5.2, 4.9, 4.7, 4.5, 4.3, 4.1, 3.9],
      };
    } else {
      return {
        categories: ["Aug", "Sep", "Oct", "Nov"],
        deposit: [7.5, 6.8, 6.0, 5.2],
        withdrawal: [6.5, 5.9, 5.3, 4.8],
        exchange: [6.0, 5.2, 4.6, 3.9],
      };
    }
  }

  // Cycles usage trend chart
  let cyclesChartOptions = $derived<ApexOptions>({
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
        name: "Deposit Canister",
        data: getCyclesChartData().deposit,
        color: "#3b82f6",
      },
      {
        name: "Withdrawal Canister",
        data: getCyclesChartData().withdrawal,
        color: "#8b5cf6",
      },
      {
        name: "Exchange Canister",
        data: getCyclesChartData().exchange,
        color: "#f59e0b",
      },
    ],
    xaxis: {
      categories: getCyclesChartData().categories,
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
        formatter: (value) => value.toFixed(1) + "T",
      },
    },
    legend: { show: true, position: "top" },
  });

  function getStatusColor(status: string) {
    if (status === "healthy" || status === "operational")
      return "bg-green-100 text-green-800";
    if (status === "warning") return "bg-yellow-100 text-yellow-800";
    if (status === "error" || status === "degraded")
      return "bg-red-100 text-red-800";
    return "bg-gray-100 text-gray-800";
  }

  function getLogLevelColor(level: string) {
    if (level === "error") return "bg-red-100 text-red-800";
    if (level === "warning") return "bg-yellow-100 text-yellow-800";
    if (level === "info") return "bg-blue-100 text-blue-800";
    return "bg-gray-100 text-gray-800";
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Cycles Usage Trend Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Cycles Usage Trend
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">
          Canister cycles consumption
        </p>
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
      <Chart options={cyclesChartOptions} />
    </div>
  </div>

  <!-- System Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-4">
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">System Health</p>
          <p class="mt-2 text-2xl font-bold text-green-600 sm:text-3xl">
            Healthy
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-green-50"
        >
          <CheckCircle class="h-6 w-6 text-green-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Uptime</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl"
          >
            {systemHealth.uptime}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-blue-50"
        >
          <Activity class="h-6 w-6 text-blue-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Total Cycles</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl"
          >
            {systemHealth.totalCycles}T
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-purple-50"
        >
          <Zap class="h-6 w-6 text-purple-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Canisters</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl"
          >
            {canisters.length}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-yellow-50"
        >
          <Server class="h-6 w-6 text-yellow-600" />
        </div>
      </div>
    </div>
  </div>

  <!-- Canister Status -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
        Canister Status
      </h3>
      <p class="text-xs text-gray-500 sm:text-sm">
        Monitor canister health and cycles
      </p>
    </div>

    <div class="space-y-3 sm:space-y-4">
      {#each canisters as canister}
        <div
          class="rounded-lg border border-gray-100 p-4 transition-all hover:border-gray-200"
        >
          <div class="flex items-start justify-between">
            <div class="flex-1">
              <div class="flex items-center space-x-2">
                <h4 class="font-semibold text-gray-900">{canister.name}</h4>
                <span
                  class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(
                    canister.status,
                  )}"
                >
                  {canister.status}
                </span>
              </div>
              <p class="mt-1 font-mono text-xs text-gray-500">{canister.id}</p>
            </div>
            <div class="text-right">
              <p class="text-sm font-semibold text-gray-500">Cycles</p>
              <p class="font-mono text-lg font-bold text-gray-900">
                {canister.cycles}T
              </p>
            </div>
          </div>
          <div class="mt-3 border-t border-gray-100 pt-3">
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-500">Uptime</span>
              <span class="font-mono text-sm font-semibold text-gray-900"
                >{canister.uptime}</span
              >
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- API Status -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
        API Status
      </h3>
      <p class="text-xs text-gray-500 sm:text-sm">External service health</p>
    </div>

    <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4">
      {#each apiStatus as api}
        <div class="rounded-lg border border-gray-100 p-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="font-semibold text-gray-900">{api.name}</p>
              <p class="mt-1 text-xs text-gray-500">
                Response: {api.responseTime}
              </p>
            </div>
            <span
              class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(
                api.status,
              )}"
            >
              {api.status}
            </span>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Error Logs -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
        Recent Logs
      </h3>
      <p class="text-xs text-gray-500 sm:text-sm">System events and errors</p>
    </div>

    <div class="space-y-3">
      {#each errorLogs as log}
        <div class="rounded-lg border border-gray-100 p-3">
          <div class="flex items-start justify-between">
            <div class="flex-1">
              <div class="flex items-center space-x-2">
                <span
                  class="rounded-full px-2 py-1 text-xs font-medium {getLogLevelColor(
                    log.level,
                  )}"
                >
                  {log.level}
                </span>
                <span class="text-xs text-gray-500">{log.timestamp}</span>
              </div>
              <p class="mt-2 text-sm text-gray-900">{log.message}</p>
              <p class="mt-1 text-xs text-gray-500">{log.canister}</p>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Info Box -->
  <div
    class="flex items-start space-x-2 rounded-lg border border-blue-200 bg-blue-50 p-3 sm:p-4"
  >
    <Info class="mt-0.5 h-4 w-4 shrink-0 text-blue-600" />
    <div class="text-xs text-blue-900 sm:text-sm">
      <p class="font-semibold">System Monitoring:</p>
      <p class="mt-1 text-blue-800">
        Last deployment: {systemHealth.lastDeployment}. Monitor canister cycles
        and top up when below 2T to ensure continuous operation.
      </p>
    </div>
  </div>
</div>
