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
    RefreshCw,
    ChevronDown,
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import StatCard from "$lib/components/admin/StatCard.svelte";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
  import {
    banAgent,
    listAgents,
    getAgentStats,
  } from "$lib/services/juno/agentService";
  import { listAgentReviews } from "$lib/services/juno/reviewService";
  import { toast } from "$lib/stores/toast";
  import type { AgentProfile, AgentReview } from "$lib/types/admin";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();

  // Helper to generate avatar URL
  function getAvatarUrl(name: string): string {
    const colors = ["3b82f6", "8b5cf6", "10b981", "f59e0b", "ef4444"];
    const colorIndex = name.length % colors.length;
    return `https://ui-avatars.com/api/?name=${encodeURIComponent(name)}&background=${colors[colorIndex]}&color=fff`;
  }

  // Last updated timestamp
  let lastUpdated = $state(new Date().toLocaleTimeString());

  // Refresh data
  async function refreshData() {
    try {
      const [newAgents, newStats] = await Promise.all([
        listAgents({ limit: 100 }),
        getAgentStats(),
      ]);
      agents = newAgents;
      stats = newStats;
      lastUpdated = new Date().toLocaleTimeString();
      toast.show("success", "Agent data refreshed");
    } catch (error) {
      console.error("Failed to refresh agent data:", error);
      toast.show("error", "Failed to refresh data");
    }
  }

  let searchQuery = $state("");
  let filterStatus = $state("all");
  let selectedAgent = $state<AgentProfile | null>(null);
  let showAgentModal = $state(false);
  let sortBy = $state<"joinDate" | "commission" | "revenue" | "rating">(
    "joinDate",
  );
  let sortOrder = $state<"asc" | "desc">("desc");

  // Review filtering and pagination
  let reviewFilterRating = $state<number | "all">("all");
  let displayedReviewsCount = $state(5);
  let agentReviews = $state<AgentReview[]>([]);
  let loadingReviews = $state(false);

  // Load reviews when agent modal opens
  async function loadAgentReviews(agentId: string) {
    loadingReviews = true;
    try {
      agentReviews = await listAgentReviews(agentId);
    } catch (error) {
      console.error("Failed to load reviews:", error);
      agentReviews = [];
    } finally {
      loadingReviews = false;
    }
  }

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

  async function viewAgent(agent: AgentProfile) {
    selectedAgent = agent;
    showAgentModal = true;
    // Load reviews for this agent
    await loadAgentReviews(agent.id);
  }

  function closeModal() {
    showAgentModal = false;
    selectedAgent = null;
  }

  async function handleBanAgent(agent: AgentProfile, reason: string) {
    try {
      await banAgent(agent.id, reason);
      // Update local state
      agent.status = "offline";
      closeModal();
    } catch (error) {
      console.error("Failed to ban agent:", error);
    }
  }

  // Pagination state
  let itemsPerPage = 20;
  let displayedCount = $state(itemsPerPage);

  function loadMore() {
    displayedCount += itemsPerPage;
  }

  // Real agent data from Juno
  let agents = $state<AgentProfile[]>(data.agents);
  let stats = $state(data.stats);

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
    agentReviews.filter((review) => {
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

  // Agent performance chart - use real data from top 5 agents by revenue
  let topAgents = $derived(
    agents.sort((a, b) => b.revenue - a.revenue).slice(0, 5),
  );

  let performanceChartOptions = $derived<ApexOptions>({
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
        data: topAgents.map((agent) => agent.revenue),
        color: "#3b82f6",
      },
    ],
    xaxis: {
      categories: topAgents.map((agent) => agent.name),
      labels: {
        show: true,
        style: {
          fontFamily: "Inter, sans-serif",
          cssClass: "text-xs font-normal fill-gray-500",
        },
      },
    },
    yaxis: {
      show: true,
    },
    legend: { show: false },
  });

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
    <StatCard
      label="Total Agents"
      value={stats.total}
      {lastUpdated}
      onRefresh={refreshData}
    />

    <StatCard label="Active" value={stats.active} valueColor="text-green-600" />

    <StatCard label="Busy" value={stats.busy} valueColor="text-yellow-600" />

    <StatCard
      label="Offline"
      value={stats.offline}
      valueColor="text-gray-600"
    />

    <StatCard
      label="Total Revenue"
      value={`$${stats.totalRevenue.toLocaleString()}`}
      valueColor="text-purple-600"
    />
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
      <div class="relative">
        <Button size="sm" color="light" class="gap-2">
          {filterStatus === "all"
            ? "All Status"
            : filterStatus === "active"
              ? "Active"
              : filterStatus === "busy"
                ? "Busy"
                : "Offline"}
          <ChevronDown class="h-4 w-4" />
        </Button>
        <Dropdown placement="bottom" class="z-50 w-44 !shadow-md">
          <DropdownItem onclick={() => (filterStatus = "all")}>
            All Status
          </DropdownItem>
          <DropdownItem onclick={() => (filterStatus = "active")}>
            Active
          </DropdownItem>
          <DropdownItem onclick={() => (filterStatus = "busy")}>
            Busy
          </DropdownItem>
          <DropdownItem onclick={() => (filterStatus = "offline")}>
            Offline
          </DropdownItem>
        </Dropdown>
      </div>
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
                  >({agent.reviewCount} reviews)</span
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
                {agent.transactionCount}
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
                    ({selectedAgent.reviewCount} reviews)
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
                    {selectedAgent.transactionCount}
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
                        <p class="font-semibold text-gray-900">
                          {review.userName}
                        </p>
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
                      <p class="text-xs text-gray-500">
                        {new Date(review.createdAt).toLocaleDateString()}
                      </p>
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
            onclick={() => {
              if (selectedAgent)
                banAgent(selectedAgent.id, "Banned from admin panel");
            }}
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
