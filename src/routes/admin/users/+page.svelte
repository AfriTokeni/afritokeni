<script lang="ts">
  import {
    Search,
    Users,
    CheckCircle,
    XCircle,
    Clock,
    Eye,
    Info,
    X,
    FileText,
    ChevronLeft,
    ChevronRight,
    Activity,
    DollarSign,
  } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import StatCard from "$lib/components/admin/StatCard.svelte";
  import SearchBar from "$lib/components/admin/SearchBar.svelte";
  import FilterDropdown from "$lib/components/admin/FilterDropdown.svelte";
  import {
    listUsers,
    getUserStats,
    getUserGrowthData,
  } from "$lib/services/juno/userService";
  import { junoInitialized } from "$lib/stores/auth";
  import { toast } from "$lib/stores/toast";
  import type { UserProfile, UserStats } from "$lib/types/admin";

  let searchQuery = $state("");
  let filterKYC = $state("all");
  let chartDateRange = $state<"7" | "30" | "90">("7");
  let selectedUser = $state<any>(null);
  let showUserModal = $state(false);

  function viewUser(user: any) {
    selectedUser = user;
    showUserModal = true;
  }

  function closeModal() {
    showUserModal = false;
    selectedUser = null;
  }

  // Pagination state
  let itemsPerPage = 20;
  let displayedCount = $state(itemsPerPage);

  function loadMore() {
    displayedCount += itemsPerPage;
  }

  // User data
  let users = $state<UserProfile[]>([]);
  let isLoading = $state(false);
  let lastUpdated = $state("");

  let stats = $state<
    UserStats & {
      totalChange?: number;
      kycApprovedChange?: number;
      kycPendingChange?: number;
      activeTodayChange?: number;
    }
  >({
    total: 0,
    kycApproved: 0,
    kycPending: 0,
    kycRejected: 0,
    activeToday: 0,
    totalChange: 0,
    kycApprovedChange: 0,
    kycPendingChange: 0,
    activeTodayChange: 0,
  });

  // Load data from Juno
  async function loadData() {
    if (isLoading) return;

    isLoading = true;
    try {
      const [usersData, statsData, chartData] = await Promise.all([
        listUsers({ kycStatus: filterKYC, searchQuery }),
        getUserStats(),
        getUserGrowthData(parseInt(chartDateRange)),
      ]);

      users = usersData;
      stats = statsData as typeof stats;

      // Update chart with real data
      userGrowthOptions = {
        ...userGrowthOptions,
        series: [
          {
            name: "Total Users",
            data: chartData.totalUsers,
            color: "#3b82f6",
          },
          {
            name: "Active Users",
            data: chartData.activeUsers,
            color: "#8b5cf6",
          },
        ],
        xaxis: {
          ...userGrowthOptions.xaxis,
          categories: chartData.categories,
        },
      };

      lastUpdated = new Date().toLocaleTimeString();
    } catch (error) {
      console.error("Error loading users:", error);
      toast.show("error", "Failed to load users");
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
      // Track dependencies
      filterKYC;
      searchQuery;
      chartDateRange;
      loadData();
    }
  });

  // User growth chart
  let userGrowthOptions: ApexOptions = $state({
    chart: {
      height: "320px",
      type: "area",
      fontFamily: "Inter, sans-serif",
      dropShadow: { enabled: false },
      toolbar: { show: false },
    },
    tooltip: { enabled: true },
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
        name: "Total Users",
        data: [],
        color: "#3b82f6",
      },
      {
        name: "Active Users",
        data: [],
        color: "#8b5cf6",
      },
    ],
    xaxis: {
      categories: [],
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

  function getKYCStatusColor(status: string) {
    if (status === "approved") return "bg-green-100 text-green-800";
    if (status === "pending") return "bg-yellow-100 text-yellow-800";
    if (status === "rejected") return "bg-red-100 text-red-800";
    return "bg-gray-100 text-gray-800";
  }

  // Filter and paginate users
  let filteredUsers = $derived(
    users.filter((user) => {
      const matchesKYC = filterKYC === "all" || user.kycStatus === filterKYC;
      const matchesSearch =
        !searchQuery ||
        user.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        user.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (user.phone &&
          user.phone.toLowerCase().includes(searchQuery.toLowerCase()));
      return matchesKYC && matchesSearch;
    }),
  );

  let displayedUsers = $derived(filteredUsers.slice(0, displayedCount));
  let hasMore = $derived(displayedCount < filteredUsers.length);

  function getKYCIcon(status: string) {
    if (status === "approved") return CheckCircle;
    if (status === "pending") return Clock;
    if (status === "rejected") return XCircle;
    return Clock;
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-4">
    <StatCard
      label="Total Users"
      value={stats.total}
      trend={stats.totalChange}
      {lastUpdated}
      onRefresh={refreshData}
    />

    <StatCard
      label="KYC Approved"
      value={stats.kycApproved}
      trend={stats.kycApprovedChange}
      valueColor="text-green-600"
      onClick={() => (filterKYC = "approved")}
    />

    <StatCard
      label="KYC Pending"
      value={stats.kycPending}
      trend={stats.kycPendingChange}
      valueColor="text-yellow-600"
      onClick={() => (filterKYC = "pending")}
    />

    <StatCard
      label="Active Today"
      value={stats.activeToday}
      trend={stats.activeTodayChange}
      valueColor="text-purple-600"
    />
  </div>

  <!-- User Growth Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          User Growth
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">
          Last {chartDateRange} days trend
        </p>
      </div>
      <FilterDropdown
        bind:value={chartDateRange}
        options={[
          { value: "7", label: "Last 7 days" },
          { value: "30", label: "Last 30 days" },
          { value: "90", label: "Last 90 days" },
        ]}
      />
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={userGrowthOptions} />
    </div>
  </div>

  <!-- Filters -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="flex flex-wrap gap-4">
      <SearchBar bind:value={searchQuery} placeholder="Search users..." />
      <FilterDropdown
        bind:value={filterKYC}
        options={[
          { value: "all", label: "All KYC Status" },
          { value: "approved", label: "Approved" },
          { value: "pending", label: "Pending" },
          { value: "rejected", label: "Rejected" },
          { value: "not_submitted", label: "Not Submitted" },
        ]}
      />
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

    {#if displayedUsers.length === 0}
      <!-- Empty State -->
      <div class="py-12 text-center">
        <Users class="mx-auto h-12 w-12 text-gray-400" />
        <h3 class="mt-4 text-lg font-semibold text-gray-900">No users found</h3>
        <p class="mt-2 text-sm text-gray-500">
          {#if searchQuery || filterKYC !== "all"}
            Try adjusting your filters or search query
          {:else}
            Users will appear here once they register on the platform
          {/if}
        </p>
      </div>
    {:else}
      <div class="space-y-3 sm:space-y-4">
        {#each displayedUsers as user}
          <button
            onclick={() => viewUser(user)}
            class="flex w-full items-center justify-between rounded-lg border border-gray-100 p-3 text-left transition-all hover:border-blue-400 hover:shadow-md sm:p-4"
          >
            <div class="flex items-center space-x-3 sm:space-x-4">
              <img
                src={`https://ui-avatars.com/api/?name=${encodeURIComponent(user.name)}&background=3b82f6&color=fff`}
                alt={user.name}
                class="h-12 w-12 rounded-lg"
              />
              <div class="min-w-0">
                <div class="flex items-center space-x-2">
                  <p class="font-semibold text-gray-900">{user.name}</p>
                  <span
                    class="rounded-full px-2 py-1 text-xs font-medium {getKYCStatusColor(
                      user.kycStatus ?? 'pending',
                    )}"
                  >
                    {(user.kycStatus ?? "pending").replace("_", " ")}
                  </span>
                </div>
                <p class="mt-1 text-sm text-gray-500">{user.email}</p>
                <p class="text-xs text-gray-400">
                  {user.phone ?? "No phone"} • Joined {user.joinedAt ??
                    "Unknown"}
                </p>
              </div>
            </div>
            <div class="flex items-center space-x-3 sm:space-x-4">
              <div class="text-right">
                <p class="text-sm font-semibold text-gray-900">Balance</p>
                <p class="font-mono text-sm font-bold text-gray-900">
                  ${(user.balance ?? 0).toLocaleString()}
                </p>
              </div>
              <Eye class="h-5 w-5 text-blue-600" />
            </div>
          </button>
        {/each}
      </div>
    {/if}

    <!-- Load More Button -->
    {#if hasMore}
      <div class="mt-6 flex justify-center">
        <button
          onclick={loadMore}
          class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-all hover:bg-blue-700"
        >
          Load More ({filteredUsers.length - displayedCount} remaining)
        </button>
      </div>
    {/if}
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

<!-- User KYC Modal -->
{#if showUserModal && selectedUser}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div
      class="max-h-[95vh] w-full max-w-4xl overflow-y-auto rounded-2xl bg-white shadow-xl"
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-10 border-b border-gray-100 bg-gradient-to-r from-blue-50 to-white px-8 py-6"
      >
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-2xl font-bold text-gray-900">User Profile & KYC</h3>
            <p class="mt-1 text-sm text-gray-500">{selectedUser.name}</p>
          </div>
          <button
            onclick={closeModal}
            class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
          >
            <X class="h-6 w-6" />
          </button>
        </div>
      </div>

      <div class="p-8">
        <div class="space-y-6">
          <!-- User Info -->
          <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              User Information
            </h4>
            <div class="flex items-start space-x-6">
              <img
                src={selectedUser.avatar}
                alt={selectedUser.name}
                class="h-20 w-20 rounded-xl"
              />
              <div class="flex-1 space-y-3">
                <div>
                  <p class="text-xs text-gray-500">Full Name</p>
                  <p class="mt-1 text-base font-semibold text-gray-900">
                    {selectedUser.name}
                  </p>
                </div>
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <p class="text-xs text-gray-500">Email</p>
                    <p class="mt-1 text-sm font-medium text-gray-900">
                      {selectedUser.email}
                    </p>
                  </div>
                  <div>
                    <p class="text-xs text-gray-500">Phone</p>
                    <p class="mt-1 text-sm font-medium text-gray-900">
                      {selectedUser.phone}
                    </p>
                  </div>
                  <div>
                    <p class="text-xs text-gray-500">Balance</p>
                    <p
                      class="mt-1 font-mono text-base font-bold text-green-600"
                    >
                      ${(selectedUser.balance ?? 0).toLocaleString()}
                    </p>
                  </div>
                  <div>
                    <p class="text-xs text-gray-500">Joined</p>
                    <p class="mt-1 text-sm font-medium text-gray-900">
                      {selectedUser.joinedAt}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Activity Stats -->
          <div class="grid grid-cols-3 gap-4">
            <div
              class="rounded-xl border border-gray-200 bg-gradient-to-br from-blue-50 to-white p-4 shadow-sm"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-xs font-semibold text-gray-500">
                    Transactions
                  </p>
                  <p class="mt-2 font-mono text-2xl font-bold text-blue-600">
                    {Math.floor(Math.random() * 50) + 10}
                  </p>
                </div>
                <Activity class="h-8 w-8 text-blue-600 opacity-50" />
              </div>
            </div>
            <div
              class="rounded-xl border border-gray-200 bg-gradient-to-br from-green-50 to-white p-4 shadow-sm"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-xs font-semibold text-gray-500">Fees Paid</p>
                  <p class="mt-2 font-mono text-2xl font-bold text-green-600">
                    ${Math.floor(Math.random() * 500) + 50}
                  </p>
                </div>
                <DollarSign class="h-8 w-8 text-green-600 opacity-50" />
              </div>
            </div>
            <div
              class="rounded-xl border border-gray-200 bg-gradient-to-br from-purple-50 to-white p-4 shadow-sm"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-xs font-semibold text-gray-500">Reviews</p>
                  <p class="mt-2 font-mono text-2xl font-bold text-purple-600">
                    {Math.floor(Math.random() * 20) + 1}
                  </p>
                </div>
                <svg
                  class="h-8 w-8 text-purple-600 opacity-50"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"
                  />
                </svg>
              </div>
            </div>
          </div>

          <!-- KYC Status -->
          <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              KYC Status
            </h4>
            <div class="flex items-center justify-between">
              <span class="text-lg font-semibold text-gray-900"
                >Verification Status</span
              >
              <span
                class="flex items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold {getKYCStatusColor(
                  selectedUser.kycStatus,
                )}"
              >
                {#if selectedUser.kycStatus === "approved"}
                  <CheckCircle class="h-4 w-4" />
                {:else if selectedUser.kycStatus === "pending"}
                  <Clock class="h-4 w-4" />
                {:else if selectedUser.kycStatus === "rejected"}
                  <XCircle class="h-4 w-4" />
                {/if}
                {selectedUser.kycStatus.replace("_", " ").toUpperCase()}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="border-t border-gray-100 bg-gray-50 px-8 py-6">
        <button
          onclick={closeModal}
          class="w-full rounded-xl bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 font-semibold text-white shadow-lg transition-all hover:from-blue-700 hover:to-blue-800 hover:shadow-xl"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
