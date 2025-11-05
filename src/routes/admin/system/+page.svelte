<script lang="ts">
  import {
    Activity,
    Info,
    ChevronDown,
    RefreshCw,
    ArrowUpDown,
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
  import StatCard from "$lib/components/admin/StatCard.svelte";
  import {
    getCanisterStatus,
    getSystemHealth,
    getSystemLogs,
    getAPIStatus,
    getCyclesChartData,
    type CanisterStatus,
    type SystemLog,
    type APIStatus,
    type SystemHealth,
  } from "$lib/services/juno/systemService";
  import { toast } from "$lib/stores/toast";

  let chartDateRange = $state<"7" | "30" | "90">("30");
  let logFilterSeverity = $state<"all" | "error" | "warning" | "info">("all");
  let logSortOrder = $state<"newest" | "oldest">("newest");
  let displayedLogsCount = $state(10);
  let activeSection = $state<"canisters" | "api" | "logs">("canisters");
  let loading = $state(true);

  // Data states
  let canisters = $state<CanisterStatus[]>([]);
  let systemHealth = $state<SystemHealth | null>(null);
  let errorLogs = $state<SystemLog[]>([]);
  let apiStatus = $state<APIStatus[]>([]);
  let cyclesChartData = $state<{ categories: string[]; series: any[] }>({
    categories: [],
    series: [],
  });

  // Last updated timestamps
  let canistersLastUpdated = $state(new Date().toLocaleTimeString());
  let apiLastUpdated = $state(new Date().toLocaleTimeString());
  let logsLastUpdated = $state(new Date().toLocaleTimeString());

  function loadMoreLogs() {
    displayedLogsCount += 10;
  }

  async function refreshCanisters() {
    try {
      canisters = await getCanisterStatus();
      systemHealth = await getSystemHealth();
      canistersLastUpdated = new Date().toLocaleTimeString();
      toast.show("success", "Canister status refreshed");
    } catch (error) {
      console.error("Error refreshing canisters:", error);
      toast.show("error", "Failed to refresh canister status");
    }
  }

  async function refreshAPI() {
    try {
      apiStatus = await getAPIStatus();
      apiLastUpdated = new Date().toLocaleTimeString();
      toast.show("success", "API status refreshed");
    } catch (error) {
      console.error("Error refreshing API status:", error);
      toast.show("error", "Failed to refresh API status");
    }
  }

  async function refreshLogs() {
    try {
      errorLogs = await getSystemLogs();
      logsLastUpdated = new Date().toLocaleTimeString();
      toast.show("success", "Logs refreshed");
    } catch (error) {
      console.error("Error refreshing logs:", error);
      toast.show("error", "Failed to refresh logs");
    }
  }

  async function refreshSystemHealth() {
    try {
      systemHealth = await getSystemHealth();
      toast.show("success", "System health refreshed");
    } catch (error) {
      console.error("Error refreshing system health:", error);
      toast.show("error", "Failed to refresh system health");
    }
  }

  async function loadData() {
    loading = true;
    try {
      const [canistersData, healthData, logsData, apiData] = await Promise.all([
        getCanisterStatus(),
        getSystemHealth(),
        getSystemLogs(),
        getAPIStatus(),
      ]);

      canisters = canistersData;
      systemHealth = healthData;
      errorLogs = logsData;
      apiStatus = apiData;

      // Check if we have controller access to canisters
      const hasControllerAccess = canisters.some((c) => c.cycles !== null);
      if (!hasControllerAccess) {
        toast.show(
          "warning",
          "Cycles data unavailable: Requires controller access to canisters"
        );
      }

      await loadChartData();
    } catch (error) {
      console.error("Error loading system data:", error);
      toast.show("error", "Failed to load system data");
    } finally {
      loading = false;
    }
  }

  async function loadChartData() {
    try {
      const days =
        chartDateRange === "7" ? 7 : chartDateRange === "30" ? 30 : 90;
      cyclesChartData = await getCyclesChartData(days);
    } catch (error) {
      console.error("Error loading chart data:", error);
    }
  }

  $effect(() => {
    loadChartData();
  });

  function scrollToSection(section: typeof activeSection) {
    activeSection = section;
    const element = document.getElementById(section);
    if (element) {
      element.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  onMount(() => {
    loadData();
  });

  // Filter, sort, and paginate logs
  let filteredLogs = $derived(
    errorLogs
      .filter(
        (log) => logFilterSeverity === "all" || log.level === logFilterSeverity,
      )
      .sort((a, b) => {
        const dateA = new Date(a.timestamp).getTime();
        const dateB = new Date(b.timestamp).getTime();
        return logSortOrder === "newest" ? dateB - dateA : dateA - dateB;
      }),
  );

  let displayedLogs = $derived(filteredLogs.slice(0, displayedLogsCount));
  let hasMoreLogs = $derived(displayedLogsCount < filteredLogs.length);

  // Count logs by severity
  let errorCount = $derived(
    errorLogs.filter((l) => l.level === "error").length,
  );
  let warningCount = $derived(
    errorLogs.filter((l) => l.level === "warning").length,
  );
  let infoCount = $derived(errorLogs.filter((l) => l.level === "info").length);

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
    series: cyclesChartData.series.map((s, index) => ({
      ...s,
      color: ["#3b82f6", "#8b5cf6", "#f59e0b"][index] ?? "#3b82f6",
    })),
    xaxis: {
      categories: cyclesChartData.categories,
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
  {#if loading}
    <div class="flex items-center justify-center py-12">
      <RefreshCw class="h-8 w-8 animate-spin text-blue-600" />
    </div>
  {:else}
    <!-- System Overview -->
    <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-4">
      <StatCard
        label="System Health"
        value={systemHealth?.overall === "healthy"
          ? "Healthy"
          : systemHealth?.overall === "warning"
            ? "Warning"
            : "Error"}
        valueColor={systemHealth?.overall === "healthy"
          ? "text-green-600"
          : systemHealth?.overall === "warning"
            ? "text-yellow-600"
            : "text-red-600"}
        onClick={() => scrollToSection("logs")}
        onRefresh={refreshSystemHealth}
      />

      <StatCard
        label="Uptime"
        value={systemHealth ? systemHealth.uptime : "Loading..."}
        trend={systemHealth?.uptimeTrend}
        trendLabel="vs last period"
        onClick={() => scrollToSection("canisters")}
      />

      <StatCard
        label="Total Cycles"
        value={systemHealth && systemHealth.totalCycles > 0 ? `${(systemHealth.totalCycles / 1_000_000_000_000).toFixed(2)}T` : "N/A"}
        trend={systemHealth?.cycleTrend}
        trendLabel="vs last period"
        onClick={() => scrollToSection("canisters")}
      />

      <StatCard
        label="Canisters"
        value={canisters.length}
        onClick={() => scrollToSection("canisters")}
      />
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
                <p class="mt-1 font-mono text-xs text-gray-500">
                  {canister.id}
                </p>
              </div>
              <div class="text-right">
                <p class="text-sm font-semibold text-gray-500">Cycles</p>
                <p class="font-mono text-lg font-bold text-gray-900">
                  {canister.cycles !== null ? `${(canister.cycles / 1_000_000_000_000).toFixed(2)}T` : "N/A"}
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
            class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity ===
            'all'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
          >
            All ({errorLogs.length})
          </button>
          <button
            onclick={() => (logFilterSeverity = "error")}
            class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity ===
            'error'
              ? 'bg-red-600 text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
          >
            Errors ({errorCount})
          </button>
          <button
            onclick={() => (logFilterSeverity = "warning")}
            class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity ===
            'warning'
              ? 'bg-yellow-600 text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
          >
            Warnings ({warningCount})
          </button>
          <button
            onclick={() => (logFilterSeverity = "info")}
            class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {logFilterSeverity ===
            'info'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
          >
            Info ({infoCount})
          </button>
        </div>
        <button
          onclick={() =>
            (logSortOrder = logSortOrder === "newest" ? "oldest" : "newest")}
          class="flex items-center gap-2 rounded-lg border border-gray-200 px-3 py-1 text-xs font-medium text-gray-600 transition-colors hover:bg-gray-50"
        >
          <ArrowUpDown class="h-3 w-3" />
          {logSortOrder === "newest" ? "Newest First" : "Oldest First"}
        </button>
      </div>

      <div class="max-h-96 space-y-3 overflow-y-auto">
        {#if displayedLogs.length === 0}
          <div
            class="flex flex-col items-center justify-center py-12 text-center"
          >
            <Activity class="mb-4 h-12 w-12 text-gray-300" />
            <h3 class="mb-2 text-lg font-semibold text-gray-900">
              No system logs found
            </h3>
            <p class="text-sm text-gray-500">
              System logs will appear here once monitoring is active
            </p>
          </div>
        {:else}
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
        {/if}

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
          <h4 class="text-base font-bold text-blue-900 sm:text-lg">
            System Monitoring
          </h4>
          <p class="mt-2 text-sm text-blue-800">
            <span class="font-semibold">Last deployment:</span>
            {systemHealth ? systemHealth.lastDeployment : "Loading..."}
          </p>
          <p class="mt-2 text-sm text-blue-800">
            Monitor canister cycles and top up when below 2T to ensure
            continuous operation. Use the refresh buttons above to get real-time
            updates.
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>
