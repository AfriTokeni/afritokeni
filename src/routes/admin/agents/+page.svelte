<script lang="ts">
  import {
    Search,
    MapPin,
    Star,
    TrendingUp,
    DollarSign,
    Activity,
    Info,
    X,
    Ban,
    CheckCircle,
    XCircle,
  } from "@lucide/svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";

  let searchQuery = $state("");
  let filterStatus = $state("all");
  let selectedAgent = $state<any>(null);
  let showAgentModal = $state(false);
  let sortBy = $state<"joinDate" | "commission" | "revenue" | "rating">(
    "joinDate",
  );
  let sortOrder = $state<"asc" | "desc">("desc");

  // Review filtering and pagination
  let reviewFilterRating = $state<number | "all">("all");
  let displayedReviewsCount = $state(5);

  // Mock reviews data
  let mockReviews = $state([
    {
      user: "John Doe",
      rating: 5,
      comment: "Excellent service! Very professional and quick.",
      date: "2 days ago",
    },
    {
      user: "Jane Smith",
      rating: 4,
      comment: "Good experience overall. Would use again.",
      date: "5 days ago",
    },
    {
      user: "Bob Johnson",
      rating: 5,
      comment: "Great agent! Highly recommended for transactions.",
      date: "1 week ago",
    },
    {
      user: "Alice Williams",
      rating: 3,
      comment: "Decent service but could be faster.",
      date: "1 week ago",
    },
    {
      user: "Charlie Brown",
      rating: 5,
      comment: "Fast and reliable. No issues at all!",
      date: "2 weeks ago",
    },
    {
      user: "Diana Prince",
      rating: 4,
      comment: "Professional and courteous. Good rates.",
      date: "2 weeks ago",
    },
    {
      user: "Ethan Hunt",
      rating: 5,
      comment: "Best agent I've worked with. Highly efficient.",
      date: "3 weeks ago",
    },
    {
      user: "Fiona Green",
      rating: 2,
      comment: "Service was slow and communication could be better.",
      date: "3 weeks ago",
    },
    {
      user: "George Miller",
      rating: 4,
      comment: "Reliable and trustworthy. Would recommend.",
      date: "1 month ago",
    },
    {
      user: "Hannah Lee",
      rating: 5,
      comment: "Outstanding service! Very happy with the experience.",
      date: "1 month ago",
    },
  ]);

  function loadMoreReviews() {
    displayedReviewsCount += 5;
  }

  function toggleSort(field: typeof sortBy) {
    if (sortBy === field) {
      sortOrder = sortOrder === "asc" ? "desc" : "asc";
    } else {
      sortBy = field;
      sortOrder = "desc";
    }
  }

  function viewAgent(agent: any) {
    selectedAgent = agent;
    showAgentModal = true;
  }

  function closeModal() {
    showAgentModal = false;
    selectedAgent = null;
  }

  function banAgent(agent: any) {
    // TODO: Implement ban logic with Juno
    console.log("Banning agent:", agent.name);
    agent.status = "offline";
    closeModal();
  }

  // Pagination state
  let itemsPerPage = 20;
  let displayedCount = $state(itemsPerPage);

  function loadMore() {
    displayedCount += itemsPerPage;
  }

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

  // Filter, sort and paginate agents
  let filteredAgents = $derived(
    agents
      .filter((agent) => {
        const matchesStatus =
          filterStatus === "all" || agent.status === filterStatus;
        const matchesSearch =
          !searchQuery ||
          agent.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          agent.location.toLowerCase().includes(searchQuery.toLowerCase());
        return matchesStatus && matchesSearch;
      })
      .sort((a, b) => {
        let comparison = 0;
        if (sortBy === "joinDate") {
          comparison =
            new Date(a.joinedAt).getTime() - new Date(b.joinedAt).getTime();
        } else if (sortBy === "commission") {
          comparison = a.commission - b.commission;
        } else if (sortBy === "revenue") {
          comparison = a.revenue - b.revenue;
        } else if (sortBy === "rating") {
          comparison = a.rating - b.rating;
        }
        return sortOrder === "asc" ? comparison : -comparison;
      }),
  );

  // Filter and paginate reviews
  let filteredReviews = $derived(
    mockReviews.filter((review) => {
      return (
        reviewFilterRating === "all" || review.rating === reviewFilterRating
      );
    }),
  );

  let displayedReviews = $derived(
    filteredReviews.slice(0, displayedReviewsCount),
  );
  let hasMoreReviews = $derived(displayedReviewsCount < filteredReviews.length);

  let displayedAgents = $derived(filteredAgents.slice(0, displayedCount));
  let hasMore = $derived(displayedCount < filteredAgents.length);

  // Agent performance chart
  let performanceChartOptions: ApexOptions = {
    chart: {
      height: "320px",
      type: "bar",
      fontFamily: "Inter, sans-serif",
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
    stroke: { show: true, width: 1, colors: ["transparent"] },
    grid: {
      show: true,
      strokeDashArray: 4,
      padding: { left: 2, right: 2, top: 0 },
    },
    series: [
      {
        name: "Revenue",
        data: [12500, 10200, 9800, 7600, 5200],
        color: "#3b82f6",
      },
    ],
    xaxis: {
      categories: [
        "Lagos Central",
        "Nairobi East",
        "Accra West",
        "Kampala North",
        "Kigali Center",
      ],
      labels: {
        show: true,
        style: {
          fontFamily: "Inter, sans-serif",
          cssClass: "text-xs font-normal fill-gray-500",
        },
        formatter: (value) => "$" + value.toLocaleString(),
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
  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-5">
    <button
      onclick={() => (filterStatus = "all")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-gray-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

    <button
      onclick={() => (filterStatus = "active")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-green-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

    <button
      onclick={() => (filterStatus = "busy")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-yellow-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

    <button
      onclick={() => (filterStatus = "offline")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-gray-400 hover:shadow-md sm:rounded-2xl sm:p-6"
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
    </button>

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

  <!-- Agent Performance Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
        Top Agent Performance
      </h3>
      <p class="text-xs text-gray-500 sm:text-sm">Revenue by agent</p>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={performanceChartOptions} />
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
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
            All Agents
          </h3>
          <p class="text-xs text-gray-500 sm:text-sm">
            Manage agent network and performance
          </p>
        </div>
        <div class="flex gap-2">
          <button
            onclick={() => toggleSort("joinDate")}
            class="rounded-lg border px-3 py-2 text-xs font-medium transition-colors {sortBy ===
            'joinDate'
              ? 'border-blue-600 bg-blue-50 text-blue-600'
              : 'border-gray-200 text-gray-600 hover:bg-gray-50'}"
          >
            Join Date {sortBy === "joinDate"
              ? sortOrder === "asc"
                ? "↑"
                : "↓"
              : ""}
          </button>
          <button
            onclick={() => toggleSort("commission")}
            class="rounded-lg border px-3 py-2 text-xs font-medium transition-colors {sortBy ===
            'commission'
              ? 'border-blue-600 bg-blue-50 text-blue-600'
              : 'border-gray-200 text-gray-600 hover:bg-gray-50'}"
          >
            Commission {sortBy === "commission"
              ? sortOrder === "asc"
                ? "↑"
                : "↓"
              : ""}
          </button>
          <button
            onclick={() => toggleSort("revenue")}
            class="rounded-lg border px-3 py-2 text-xs font-medium transition-colors {sortBy ===
            'revenue'
              ? 'border-blue-600 bg-blue-50 text-blue-600'
              : 'border-gray-200 text-gray-600 hover:bg-gray-50'}"
          >
            Revenue {sortBy === "revenue"
              ? sortOrder === "asc"
                ? "↑"
                : "↓"
              : ""}
          </button>
          <button
            onclick={() => toggleSort("rating")}
            class="rounded-lg border px-3 py-2 text-xs font-medium transition-colors {sortBy ===
            'rating'
              ? 'border-blue-600 bg-blue-50 text-blue-600'
              : 'border-gray-200 text-gray-600 hover:bg-gray-50'}"
          >
            Rating {sortBy === "rating"
              ? sortOrder === "asc"
                ? "↑"
                : "↓"
              : ""}
          </button>
        </div>
      </div>
    </div>

    <div class="space-y-3 sm:space-y-4">
      {#each displayedAgents as agent}
        <button
          onclick={() => viewAgent(agent)}
          class="w-full rounded-lg border border-gray-100 p-4 text-left transition-all hover:border-blue-400 hover:shadow-md"
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
        </button>
      {/each}
    </div>

    <!-- Load More Button -->
    {#if hasMore}
      <div class="mt-6 flex justify-center">
        <button
          onclick={loadMore}
          class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-all hover:bg-blue-700"
        >
          Load More ({filteredAgents.length - displayedCount} remaining)
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
      <p class="font-semibold">Agent Network:</p>
      <p class="mt-1 text-blue-800">
        Monitor agent performance, manage commissions, and track network growth.
        Click on an agent to view detailed analytics.
      </p>
    </div>
  </div>
</div>

<!-- Agent Detail Modal -->
{#if showAgentModal && selectedAgent}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div
      class="max-h-[95vh] w-full max-w-4xl overflow-y-auto rounded-2xl bg-white shadow-xl"
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-10 border-b border-gray-100 bg-gradient-to-r from-purple-50 to-white px-8 py-6"
      >
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-2xl font-bold text-gray-900">Agent Profile</h3>
            <p class="mt-1 text-sm text-gray-500">{selectedAgent.name}</p>
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
          <!-- Agent Info -->
          <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              Agent Information
            </h4>
            <div class="grid grid-cols-2 gap-6">
              <div>
                <p class="text-xs text-gray-500">Location</p>
                <div class="mt-2 flex items-center gap-2">
                  <MapPin class="h-4 w-4 text-gray-600" />
                  <p class="text-sm font-medium text-gray-900">
                    {selectedAgent.location}
                  </p>
                </div>
              </div>
              <div>
                <p class="text-xs text-gray-500">Status</p>
                <span
                  class="mt-2 inline-block rounded-full px-3 py-1 text-xs font-semibold {getStatusColor(
                    selectedAgent.status,
                  )}"
                >
                  {selectedAgent.status.toUpperCase()}
                </span>
              </div>
              <div>
                <p class="text-xs text-gray-500">Rating</p>
                <div class="mt-2 flex items-center gap-1">
                  <Star class="h-5 w-5 fill-yellow-400 text-yellow-400" />
                  <p class="text-base font-bold text-gray-900">
                    {selectedAgent.rating}
                  </p>
                  <p class="text-sm text-gray-500">
                    ({selectedAgent.reviews} reviews)
                  </p>
                </div>
              </div>
              <div>
                <p class="text-xs text-gray-500">Joined</p>
                <p class="mt-2 text-sm font-medium text-gray-900">
                  {selectedAgent.joinedAt}
                </p>
              </div>
            </div>
          </div>

          <!-- Performance Stats -->
          <div class="grid grid-cols-3 gap-4">
            <div
              class="rounded-xl border border-gray-200 bg-gradient-to-br from-blue-50 to-white p-4 shadow-sm"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-xs font-semibold text-gray-500">Revenue</p>
                  <p class="mt-2 font-mono text-2xl font-bold text-blue-600">
                    ${selectedAgent.revenue.toLocaleString()}
                  </p>
                </div>
                <DollarSign class="h-8 w-8 text-blue-600 opacity-50" />
              </div>
            </div>
            <div
              class="rounded-xl border border-gray-200 bg-gradient-to-br from-green-50 to-white p-4 shadow-sm"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-xs font-semibold text-gray-500">
                    Transactions
                  </p>
                  <p class="mt-2 font-mono text-2xl font-bold text-green-600">
                    {selectedAgent.transactions}
                  </p>
                </div>
                <Activity class="h-8 w-8 text-green-600 opacity-50" />
              </div>
            </div>
            <div
              class="rounded-xl border border-gray-200 bg-gradient-to-br from-purple-50 to-white p-4 shadow-sm"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-xs font-semibold text-gray-500">Commission</p>
                  <p class="mt-2 font-mono text-2xl font-bold text-purple-600">
                    ${selectedAgent.commission}
                  </p>
                </div>
                <TrendingUp class="h-8 w-8 text-purple-600 opacity-50" />
              </div>
            </div>
          </div>

          <!-- Reviews Section -->
          <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
            <div class="mb-4 flex items-center justify-between">
              <h4
                class="text-sm font-semibold tracking-wide text-gray-500 uppercase"
              >
                Reviews ({filteredReviews.length})
              </h4>
              <div class="flex gap-2">
                <button
                  onclick={() => (reviewFilterRating = "all")}
                  class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {reviewFilterRating ===
                  'all'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
                >
                  All
                </button>
                {#each [5, 4, 3, 2, 1] as rating}
                  <button
                    onclick={() => (reviewFilterRating = rating)}
                    class="rounded-lg px-3 py-1 text-xs font-medium transition-colors {reviewFilterRating ===
                    rating
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
                  >
                    {rating}★
                  </button>
                {/each}
              </div>
            </div>
            <div class="max-h-96 space-y-4 overflow-y-auto">
              {#each displayedReviews as review}
                <div class="rounded-lg border border-gray-100 bg-gray-50 p-4">
                  <div class="flex items-start justify-between">
                    <div class="flex-1">
                      <div class="flex items-center gap-2">
                        <p class="font-semibold text-gray-900">{review.user}</p>
                        <div class="flex items-center gap-1">
                          {#each Array(5) as _, starIndex}
                            <Star
                              class="h-3 w-3 {starIndex < review.rating
                                ? 'fill-yellow-400 text-yellow-400'
                                : 'text-gray-300'}"
                            />
                          {/each}
                        </div>
                      </div>
                      <p class="mt-2 text-sm text-gray-600">
                        {review.comment}
                      </p>
                      <p class="mt-2 text-xs text-gray-400">{review.date}</p>
                    </div>
                  </div>
                </div>
              {/each}

              <!-- Load More Reviews Button -->
              {#if hasMoreReviews}
                <button
                  onclick={loadMoreReviews}
                  class="w-full rounded-lg border-2 border-dashed border-gray-300 py-3 text-sm font-medium text-gray-600 transition-colors hover:border-blue-600 hover:text-blue-600"
                >
                  Load More Reviews ({filteredReviews.length -
                    displayedReviewsCount} remaining)
                </button>
              {/if}
            </div>
          </div>
        </div>
      </div>

      <!-- Footer with Actions -->
      <div class="border-t border-gray-100 bg-gray-50 px-8 py-6">
        <div class="flex gap-4">
          <button
            onclick={() => banAgent(selectedAgent)}
            class="flex-1 rounded-xl border-2 border-red-600 bg-white px-6 py-4 font-semibold text-red-600 shadow-lg transition-all hover:bg-red-600 hover:text-white"
          >
            <div class="flex items-center justify-center gap-2">
              <Ban class="h-5 w-5" />
              Ban Agent
            </div>
          </button>
          <button
            onclick={closeModal}
            class="flex-1 rounded-xl bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 font-semibold text-white shadow-lg transition-all hover:from-blue-700 hover:to-blue-800 hover:shadow-xl"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
