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
    RefreshCw,
    ArrowUpDown,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

  let chartDateRange = $state<"7" | "30" | "90">("30");
  let logFilterSeverity = $state<"all" | "error" | "warning" | "info">("all");
  let logSortOrder = $state<"newest" | "oldest">("newest");
  let displayedLogsCount = $state(10);
  let activeSection = $state<"canisters" | "api" | "logs">("canisters");
  
  // Last updated timestamps
  let canistersLastUpdated = $state(new Date().toLocaleTimeString());
  let apiLastUpdated = $state(new Date().toLocaleTimeString());
  let logsLastUpdated = $state(new Date().toLocaleTimeString());
  
  function loadMoreLogs() {
    displayedLogsCount += 10;
  }
  
  function refreshCanisters() {
    console.log('Refreshing canisters...');
    canistersLastUpdated = new Date().toLocaleTimeString();
    // TODO: Implement with Juno
  }
  
  function refreshAPI() {
    console.log('Refreshing API status...');
    apiLastUpdated = new Date().toLocaleTimeString();
    // TODO: Implement with Juno
  }
  
  function refreshLogs() {
    console.log('Refreshing logs...');
    logsLastUpdated = new Date().toLocaleTimeString();
    // TODO: Implement with Juno
  }
  
  function scrollToSection(section: typeof activeSection) {
    activeSection = section;
    const element = document.getElementById(section);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  }

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
    { timestamp: "Nov 5, 2024 2:45 PM", level: "error", message: "Failed to process withdrawal TXN-12341", canister: "Withdrawal Canister" },
    { timestamp: "Nov 5, 2024 2:30 PM", level: "error", message: "Database connection timeout", canister: "Deposit Canister" },
    { timestamp: "Nov 5, 2024 1:30 PM", level: "warning", message: "Low cycles detected on Exchange Canister", canister: "Exchange Canister" },
    { timestamp: "Nov 5, 2024 1:15 PM", level: "warning", message: "High memory usage detected", canister: "Withdrawal Canister" },
    { timestamp: "Nov 5, 2024 12:15 PM", level: "info", message: "Canister upgrade completed successfully", canister: "Deposit Canister" },
    { timestamp: "Nov 5, 2024 11:45 AM", level: "info", message: "Backup completed successfully", canister: "Exchange Canister" },
    { timestamp: "Nov 5, 2024 10:30 AM", level: "error", message: "API rate limit exceeded", canister: "Deposit Canister" },
    { timestamp: "Nov 5, 2024 9:20 AM", level: "warning", message: "Slow query performance detected", canister: "Exchange Canister" },
    { timestamp: "Nov 5, 2024 8:15 AM", level: "info", message: "System health check passed", canister: "Withdrawal Canister" },
    { timestamp: "Nov 5, 2024 7:00 AM", level: "info", message: "Daily maintenance completed", canister: "Deposit Canister" },
    { timestamp: "Nov 4, 2024 11:30 PM", level: "error", message: "Transaction validation failed", canister: "Exchange Canister" },
    { timestamp: "Nov 4, 2024 10:15 PM", level: "warning", message: "Unusual traffic pattern detected", canister: "Withdrawal Canister" },
    { timestamp: "Nov 4, 2024 9:00 PM", level: "info", message: "Cache cleared successfully", canister: "Deposit Canister" },
    { timestamp: "Nov 4, 2024 8:45 PM", level: "info", message: "Scheduled task completed", canister: "Exchange Canister" },
    { timestamp: "Nov 4, 2024 7:30 PM", level: "error", message: "Network connectivity issue", canister: "Withdrawal Canister" },
  ]);
  
  // Filter, sort, and paginate logs
  let filteredLogs = $derived(
    errorLogs
      .filter((log) => logFilterSeverity === "all" || log.level === logFilterSeverity)
      .sort((a, b) => {
        const dateA = new Date(a.timestamp).getTime();
        const dateB = new Date(b.timestamp).getTime();
        return logSortOrder === "newest" ? dateB - dateA : dateA - dateB;
      })
  );
  
  let displayedLogs = $derived(filteredLogs.slice(0, displayedLogsCount));
  let hasMoreLogs = $derived(displayedLogsCount < filteredLogs.length);
  
  // Count logs by severity
  let errorCount = $derived(errorLogs.filter(l => l.level === "error").length);
  let warningCount = $derived(errorLogs.filter(l => l.level === "warning").length);
  let infoCount = $derived(errorLogs.filter(l => l.level === "info").length);

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
  <!-- System Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-4">
    <button
      onclick={() => scrollToSection('logs')}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-green-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

    <button
      onclick={() => scrollToSection('canisters')}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-blue-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

    <button
      onclick={() => scrollToSection('canisters')}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-purple-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

    <button
      onclick={() => scrollToSection('canisters')}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-yellow-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>
  </div>

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

  <!-- Canister Status -->
  <div
    id="canisters"
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Canister Status
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">
          Last updated: {canistersLastUpdated}
        </p>
      </div>
      <button
        onclick={refreshCanisters}
        class="flex items-center gap-2 rounded-lg border border-gray-200 px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-blue-50 hover:text-blue-600"
      >
        <RefreshCw class="h-4 w-4" />
        Refresh
      </button>
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
    id="api"
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          API Status
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">
          Last updated: {apiLastUpdated}
        </p>
      </div>
      <button
        onclick={refreshAPI}
        class="flex items-center gap-2 rounded-lg border border-gray-200 px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-blue-50 hover:text-blue-600"
      >
        <RefreshCw class="h-4 w-4" />
        Refresh
      </button>
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
    id="logs"
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          System Logs ({filteredLogs.length})
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">
          Last updated: {logsLastUpdated}
        </p>
      </div>
      <button
        onclick={refreshLogs}
        class="flex items-center gap-2 rounded-lg border border-gray-200 px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-blue-50 hover:text-blue-600"
      >
        <RefreshCw class="h-4 w-4" />
        Refresh
      </button>
    </div>

    <!-- Filter and Sort Controls -->
    <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
      <div class="flex gap-2">
        <button
          onclick={() => (logFilterSeverity = "all")}
          class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity === 'all' ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
        >
          All ({errorLogs.length})
        </button>
        <button
          onclick={() => (logFilterSeverity = "error")}
          class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity === 'error' ? 'bg-red-600 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
        >
          Errors ({errorCount})
        </button>
        <button
          onclick={() => (logFilterSeverity = "warning")}
          class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity === 'warning' ? 'bg-yellow-600 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
        >
          Warnings ({warningCount})
        </button>
        <button
          onclick={() => (logFilterSeverity = "info")}
          class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity === 'info' ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
        >
          Info ({infoCount})
        </button>
      </div>
      <button
        onclick={() => (logSortOrder = logSortOrder === "newest" ? "oldest" : "newest")}
        class="flex items-center gap-2 rounded-lg border border-gray-200 px-3 py-1 text-xs font-medium text-gray-600 transition-colors hover:bg-gray-50"
      >
        <ArrowUpDown class="h-3 w-3" />
        {logSortOrder === "newest" ? "Newest First" : "Oldest First"}
      </button>
    </div>

    <div class="max-h-96 space-y-3 overflow-y-auto">
      {#each displayedLogs as log}
        <div class="rounded-lg border border-gray-100 p-3">
          <div class="flex items-start justify-between">
            <div class="flex-1">
              <div class="flex items-center space-x-2">
                <span
                  class="rounded-full px-2 py-1 text-xs font-medium {getLogLevelColor(
                    log.level,
                  )}"
                >
                  {log.level.toUpperCase()}
                </span>
                <span class="text-xs text-gray-500">{log.timestamp}</span>
              </div>
              <p class="mt-2 text-sm text-gray-900">{log.message}</p>
              <p class="mt-1 text-xs text-gray-500">{log.canister}</p>
            </div>
          </div>
        </div>
      {/each}
      
      <!-- Load More Logs Button -->
      {#if hasMoreLogs}
        <button
          onclick={loadMoreLogs}
          class="w-full rounded-lg border-2 border-dashed border-gray-300 py-3 text-sm font-medium text-gray-600 transition-colors hover:border-blue-600 hover:text-blue-600"
        >
          Load More Logs ({filteredLogs.length - displayedLogsCount} remaining)
        </button>
      {/if}
    </div>
  </div>

  <!-- Info Box -->
  <div
    class="rounded-xl border-2 border-blue-300 bg-gradient-to-r from-blue-50 to-blue-100 p-4 shadow-md sm:p-6"
  >
    <div class="flex items-start space-x-3">
      <div class="rounded-lg bg-blue-600 p-2">
        <Info class="h-5 w-5 text-white" />
      </div>
      <div class="flex-1">
        <h4 class="text-base font-bold text-blue-900 sm:text-lg">System Monitoring</h4>
        <p class="mt-2 text-sm text-blue-800">
          <span class="font-semibold">Last deployment:</span> {systemHealth.lastDeployment}
        </p>
        <p class="mt-2 text-sm text-blue-800">
          Monitor canister cycles and top up when below 2T to ensure continuous operation.
          Use the refresh buttons above to get real-time updates.
        </p>
      </div>
    </div>
  </div>
</div>
