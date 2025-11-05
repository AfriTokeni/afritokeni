<script lang="ts">
  import { Search, Users, CheckCircle, XCircle, Clock, Eye, Info } from "lucide-svelte";
  import type { ApexOptions } from 'apexcharts';
  import { Chart } from '@flowbite-svelte-plugins/chart';

  let searchQuery = $state("");
  let filterKYC = $state("all");

  // Mock user data
  let users = $state([
    {
      id: "U-001",
      name: "John Doe",
      email: "john@example.com",
      phone: "+234 801 234 5678",
      kycStatus: "approved",
      balance: 5000,
      joinedAt: "Oct 15, 2024",
      avatar:
        "https://ui-avatars.com/api/?name=John+Doe&background=3b82f6&color=fff",
    },
    {
      id: "U-002",
      name: "Jane Smith",
      email: "jane@example.com",
      phone: "+254 712 345 678",
      kycStatus: "pending",
      balance: 2300,
      joinedAt: "Oct 20, 2024",
      avatar:
        "https://ui-avatars.com/api/?name=Jane+Smith&background=8b5cf6&color=fff",
    },
    {
      id: "U-003",
      name: "Bob Johnson",
      email: "bob@example.com",
      phone: "+233 20 123 4567",
      kycStatus: "approved",
      balance: 8900,
      joinedAt: "Oct 12, 2024",
      avatar:
        "https://ui-avatars.com/api/?name=Bob+Johnson&background=10b981&color=fff",
    },
    {
      id: "U-004",
      name: "Alice Brown",
      email: "alice@example.com",
      phone: "+234 802 345 6789",
      kycStatus: "rejected",
      balance: 0,
      joinedAt: "Nov 1, 2024",
      avatar:
        "https://ui-avatars.com/api/?name=Alice+Brown&background=ef4444&color=fff",
    },
    {
      id: "U-005",
      name: "Charlie Wilson",
      email: "charlie@example.com",
      phone: "+254 713 456 789",
      kycStatus: "not_submitted",
      balance: 1200,
      joinedAt: "Nov 3, 2024",
      avatar:
        "https://ui-avatars.com/api/?name=Charlie+Wilson&background=f59e0b&color=fff",
    },
  ]);

  let stats = $state({
    total: 2340,
    kycApproved: 1850,
    kycPending: 23,
    active: 2100,
  });
  
  // User growth chart
  let userGrowthOptions: ApexOptions = {
    chart: {
      height: '320px',
      type: 'area',
      fontFamily: 'Inter, sans-serif',
      dropShadow: { enabled: false },
      toolbar: { show: false },
    },
    tooltip: { enabled: true },
    fill: {
      type: 'gradient',
      gradient: {
        opacityFrom: 0.55,
        opacityTo: 0,
      },
    },
    dataLabels: { enabled: false },
    stroke: { width: 2, curve: 'smooth' },
    grid: {
      show: true,
      strokeDashArray: 4,
      padding: { left: 2, right: 2, top: 0 },
    },
    series: [
      {
        name: 'Total Users',
        data: [1850, 1920, 2050, 2150, 2240, 2310, 2340],
        color: '#3b82f6',
      },
      {
        name: 'Active Users',
        data: [1650, 1720, 1840, 1920, 2010, 2070, 2100],
        color: '#8b5cf6',
      },
    ],
    xaxis: {
      categories: ['Oct 29', 'Oct 30', 'Oct 31', 'Nov 1', 'Nov 2', 'Nov 3', 'Nov 4'],
      labels: {
        show: true,
        style: {
          fontFamily: 'Inter, sans-serif',
          cssClass: 'text-xs font-normal fill-gray-500',
        },
      },
      axisBorder: { show: false },
      axisTicks: { show: false },
    },
    yaxis: { show: true },
    legend: { show: true, position: 'top' },
  };

  function getKYCStatusColor(status: string) {
    if (status === "approved") return "bg-green-100 text-green-800";
    if (status === "pending") return "bg-yellow-100 text-yellow-800";
    if (status === "rejected") return "bg-red-100 text-red-800";
    return "bg-gray-100 text-gray-800";
  }

  function getKYCIcon(status: string) {
    if (status === "approved") return CheckCircle;
    if (status === "pending") return Clock;
    if (status === "rejected") return XCircle;
    return Clock;
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- User Growth Chart -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">User Growth</h3>
      <p class="text-xs text-gray-500 sm:text-sm">Last 7 days trend</p>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={userGrowthOptions} />
    </div>
  </div>
  
  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-4">
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Total Users</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-gray-900 sm:text-3xl"
          >
            {stats.total.toLocaleString()}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-blue-50"
        >
          <Users class="h-6 w-6 text-blue-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">KYC Approved</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-green-600 sm:text-3xl"
          >
            {stats.kycApproved.toLocaleString()}
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
          <p class="text-sm font-semibold text-gray-500">KYC Pending</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-yellow-600 sm:text-3xl"
          >
            {stats.kycPending}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-yellow-50"
        >
          <Clock class="h-6 w-6 text-yellow-600" />
        </div>
      </div>
    </div>

    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Active Users</p>
          <p
            class="mt-2 font-mono text-2xl font-bold text-purple-600 sm:text-3xl"
          >
            {stats.active.toLocaleString()}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-purple-50"
        >
          <Users class="h-6 w-6 text-purple-600" />
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
            placeholder="Search by name, email, or phone..."
            class="w-full rounded-lg border border-gray-200 py-2 pr-4 pl-10 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
          />
        </div>
      </div>
      <select
        bind:value={filterKYC}
        class="rounded-lg border border-gray-200 px-4 py-2 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
      >
        <option value="all">All KYC Status</option>
        <option value="approved">Approved</option>
        <option value="pending">Pending</option>
        <option value="rejected">Rejected</option>
        <option value="not_submitted">Not Submitted</option>
      </select>
    </div>
  </div>

  <!-- Users List -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
        All Users
      </h3>
      <p class="text-xs text-gray-500 sm:text-sm">
        Manage user accounts and KYC status
      </p>
    </div>

    <div class="space-y-3 sm:space-y-4">
      {#each users as user}
        <div
          class="flex items-center justify-between rounded-lg border border-gray-100 p-3 transition-all hover:border-gray-200 sm:p-4"
        >
          <div class="flex items-center space-x-3 sm:space-x-4">
            <img
              src={user.avatar}
              alt={user.name}
              class="h-12 w-12 rounded-lg"
            />
            <div class="min-w-0">
              <div class="flex items-center space-x-2">
                <p class="font-semibold text-gray-900">{user.name}</p>
                <span
                  class="rounded-full px-2 py-1 text-xs font-medium {getKYCStatusColor(
                    user.kycStatus,
                  )}"
                >
                  {user.kycStatus.replace("_", " ")}
                </span>
              </div>
              <p class="mt-1 text-sm text-gray-500">{user.email}</p>
              <p class="text-xs text-gray-400">
                {user.phone} • Joined {user.joinedAt}
              </p>
            </div>
          </div>
          <div class="flex items-center space-x-3 sm:space-x-4">
            <div class="text-right">
              <p class="text-sm font-semibold text-gray-900">Balance</p>
              <p class="font-mono text-sm font-bold text-gray-900">
                ${user.balance.toLocaleString()}
              </p>
            </div>
            <button
              class="rounded-lg border border-gray-200 px-3 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
            >
              <Eye class="h-4 w-4" />
            </button>
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
      <p class="font-semibold">User Management:</p>
      <p class="mt-1 text-blue-800">
        Click on a user to view detailed profile, transaction history, and
        manage account settings.
      </p>
    </div>
  </div>
</div>
