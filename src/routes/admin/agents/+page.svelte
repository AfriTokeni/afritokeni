<script lang="ts">
  import { Search, MapPin, Star, TrendingUp, DollarSign, Activity, Info } from "lucide-svelte";
  import type { ApexOptions } from 'apexcharts';
  import { Chart } from '@flowbite-svelte-plugins/chart';

  let searchQuery = $state("");
  let filterStatus = $state("all");

  // Mock agent data
  let agents = $state([
    {
      id: "A-001",
      name: "Agent Lagos Central",
      location: "Lagos, Nigeria",
      rating: 4.8,
      reviews: 234,
      revenue: 12500,
      transactions: 456,
      commission: 625,
      status: "active",
      joinedAt: "Sep 10, 2024",
    },
    {
      id: "A-002",
      name: "Agent Nairobi East",
      location: "Nairobi, Kenya",
      rating: 4.9,
      reviews: 198,
      revenue: 10200,
      transactions: 389,
      commission: 510,
      status: "active",
      joinedAt: "Sep 15, 2024",
    },
    {
      id: "A-003",
      name: "Agent Accra West",
      location: "Accra, Ghana",
      rating: 4.7,
      reviews: 176,
      revenue: 9800,
      transactions: 312,
      commission: 490,
      status: "active",
      joinedAt: "Sep 20, 2024",
    },
    {
      id: "A-004",
      name: "Agent Kampala North",
      location: "Kampala, Uganda",
      rating: 4.6,
      reviews: 145,
      revenue: 7600,
      transactions: 267,
      commission: 380,
      status: "busy",
      joinedAt: "Oct 1, 2024",
    },
    {
      id: "A-005",
      name: "Agent Kigali Center",
      location: "Kigali, Rwanda",
      rating: 4.5,
      reviews: 89,
      revenue: 5200,
      transactions: 178,
      commission: 260,
      status: "offline",
      joinedAt: "Oct 15, 2024",
    },
  ]);

  let stats = $state({
    total: 89,
    active: 67,
    busy: 15,
    offline: 7,
    totalRevenue: 125000,
  });
  
  // Agent performance chart
  let performanceChartOptions: ApexOptions = {
    chart: {
      height: '320px',
      type: 'bar',
      fontFamily: 'Inter, sans-serif',
      toolbar: { show: false },
    },
    plotOptions: {
      bar: {
        horizontal: true,
        borderRadius: 8,
      },
    },
    tooltip: { enabled: true },
    dataLabels: { enabled: false },
    stroke: { show: true, width: 1, colors: ['transparent'] },
    grid: {
      show: true,
      strokeDashArray: 4,
      padding: { left: 2, right: 2, top: 0 },
    },
    series: [
      {
        name: 'Revenue',
        data: [12500, 10200, 9800, 7600, 5200],
        color: '#3b82f6',
      },
    ],
    xaxis: {
      categories: ['Lagos Central', 'Nairobi East', 'Accra West', 'Kampala North', 'Kigali Center'],
      labels: {
        show: true,
        style: {
          fontFamily: 'Inter, sans-serif',
          cssClass: 'text-xs font-normal fill-gray-500',
        },
        formatter: (value) => '$' + value.toLocaleString(),
      },
    },
    yaxis: {
      show: true,
    },
    legend: { show: false },
  };

  function getStatusColor(status: string) {
    if (status === "active") return "bg-green-100 text-green-800";
    if (status === "busy") return "bg-yellow-100 text-yellow-800";
    if (status === "offline") return "bg-gray-100 text-gray-800";
    return "bg-gray-100 text-gray-800";
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Agent Performance Chart -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">Top Agent Performance</h3>
      <p class="text-xs text-gray-500 sm:text-sm">Revenue by agent</p>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={performanceChartOptions} />
    </div>
  </div>
  
  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-5">
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Total Agents</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl"
          >
            {stats.total}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-blue-50"
        >
          <MapPin class="h-6 w-6 text-blue-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Active</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-green-600 sm:text-3xl"
          >
            {stats.active}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-green-50"
        >
          <Activity class="h-6 w-6 text-green-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Busy</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-yellow-600 sm:text-3xl"
          >
            {stats.busy}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-yellow-50"
        >
          <Activity class="h-6 w-6 text-yellow-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Offline</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-gray-600 sm:text-3xl"
          >
            {stats.offline}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-gray-50"
        >
          <Activity class="h-6 w-6 text-gray-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Total Revenue</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-purple-600 sm:text-3xl"
          >
            ${stats.totalRevenue.toLocaleString()}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-between rounded-xl bg-purple-50"
        >
          <DollarSign class="h-6 w-6 text-purple-600" />
        </div>
      </div>
    </div>
  </div>

  <!-- Filters -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="flex flex-wrap gap-4">
      <div class="min-w-[200px] flex-1">
        <div class="relative">
          <Search
            class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400"
          />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search by name or location..."
            class="w-full rounded-lg border border-gray-200 py-2 pr-4 pl-10 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
          />
        </div>
      </div>
      <select
        bind:value={filterStatus}
        class="rounded-lg border border-gray-200 px-4 py-2 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
      >
        <option value="all">All Status</option>
        <option value="active">Active</option>
        <option value="busy">Busy</option>
        <option value="offline">Offline</option>
      </select>
    </div>
  </div>

  <!-- Agents List -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
        All Agents
      </h3>
      <p class="text-xs text-gray-500 sm:text-sm">
        Manage agent network and performance
      </p>
    </div>

    <div class="space-y-3 sm:space-y-4">
      {#each agents as agent}
        <div
          class="rounded-lg border border-gray-100 p-4 transition-all hover:border-gray-200"
        >
          <div class="flex items-start justify-between">
            <div class="flex-1">
              <div class="flex items-center space-x-2">
                <h4 class="font-semibold text-gray-900">{agent.name}</h4>
                <span
                  class="rounded-full px-2 py-1 text-xs font-medium {getStatusColor(
                    agent.status,
                  )}"
                >
                  {agent.status}
                </span>
              </div>
              <div
                class="mt-2 flex items-center space-x-1 text-sm text-gray-500"
              >
                <MapPin class="h-4 w-4" />
                <span>{agent.location}</span>
              </div>
              <div class="mt-2 flex items-center space-x-1">
                <Star class="h-4 w-4 fill-yellow-400 text-yellow-400" />
                <span class="text-sm font-medium text-gray-900"
                  >{agent.rating}</span
                >
                <span class="text-sm text-gray-500"
                  >({agent.reviews} reviews)</span
                >
              </div>
            </div>
            <div class="text-right">
              <p class="text-sm font-semibold text-gray-500">Revenue</p>
              <p class="font-mono text-lg font-bold text-gray-900">
                ${agent.revenue.toLocaleString()}
              </p>
            </div>
          </div>

          <div
            class="mt-4 grid grid-cols-3 gap-4 border-t border-gray-100 pt-4"
          >
            <div>
              <p class="text-xs text-gray-500">Transactions</p>
              <p class="mt-1 font-mono text-sm font-semibold text-gray-900">
                {agent.transactions}
              </p>
            </div>
            <div>
              <p class="text-xs text-gray-500">Commission Owed</p>
              <p class="mt-1 font-mono text-sm font-semibold text-green-600">
                ${agent.commission}
              </p>
            </div>
            <div>
              <p class="text-xs text-gray-500">Joined</p>
              <p class="mt-1 text-sm text-gray-900">{agent.joinedAt}</p>
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
      <p class="font-semibold">Agent Network:</p>
      <p class="mt-1 text-blue-800">
        Monitor agent performance, manage commissions, and track network growth.
        Click on an agent to view detailed analytics.
      </p>
    </div>
  </div>
</div>
