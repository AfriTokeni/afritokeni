<!--
 * Leaderboard Component (FULLY ENCAPSULATED)
 *
 * Self-contained component that:
 * - Subscribes to demoMode store internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 *
 * Usage: <Leaderboard maxEntries={10} />
-->
<script lang="ts">
  import { Award, RefreshCw, TrendingUp, Trophy } from "@lucide/svelte";
  import { demoMode } from "$lib/stores/demoMode";
  import { fetchLeaderboard } from "$lib/services/data/daoData";

  interface Props {
    maxEntries?: number;
  }

  let { maxEntries = 10 }: Props = $props();

  // Internal state
  let leaderboard = $state<any[]>([]);
  let isLoading = $state(true);
  let isRefreshing = $state(false);
  let error = $state<string | null>(null);

  // Reactive: auto-refetch when demoMode changes
  $effect(() => {
    loadLeaderboard($demoMode);
  });

  async function loadLeaderboard(isDemoMode: boolean) {
    try {
      error = null;
      isLoading = true; // Ensure loading state is set
      const data = await fetchLeaderboard(isDemoMode, maxEntries);
      leaderboard = data;
    } catch (err: any) {
      console.error("Error fetching leaderboard:", err);
      error = err.message || "Failed to load leaderboard";
    } finally {
      isLoading = false;
      isRefreshing = false;
    }
  }

  async function handleRefresh() {
    isRefreshing = true;
    await loadLeaderboard($demoMode);
  }

  function getRankColor(rank: number) {
    if (rank === 1) return "text-yellow-600 bg-yellow-50";
    if (rank === 2) return "text-gray-600 bg-gray-50";
    if (rank === 3) return "text-orange-600 bg-orange-50";
    return "text-neutral-600 bg-neutral-50";
  }

  function getRankIcon(rank: number) {
    if (rank <= 3) return Trophy;
    return Award;
  }

  function formatNumber(num: number): string {
    if (num >= 1_000_000) return `${(num / 1_000_000).toFixed(1)}M`;
    if (num >= 1_000) return `${(num / 1_000).toFixed(1)}K`;
    return num.toString();
  }
</script>

<div
  class="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm sm:p-5 md:p-6"
>
  <!-- Header -->
  <div class="mb-4 flex items-center justify-between sm:mb-6">
    <div class="flex items-center gap-2 sm:gap-3">
      <div class="rounded-lg bg-yellow-50 p-2">
        <Trophy class="h-5 w-5 shrink-0 text-yellow-600 sm:h-6 sm:w-6" />
      </div>
      <div>
        <h2 class="text-lg font-bold text-neutral-900 sm:text-xl">
          Leaderboard
        </h2>
        <p class="text-xs text-neutral-600 sm:text-sm">Top Contributors</p>
      </div>
    </div>
    <button
      onclick={handleRefresh}
      disabled={isRefreshing}
      class="rounded-lg p-2 transition-colors hover:bg-neutral-100 disabled:opacity-50"
      title="Refresh leaderboard"
    >
      <RefreshCw
        class="h-4 w-4 shrink-0 text-neutral-600 sm:h-5 sm:w-5 {isRefreshing
          ? 'animate-spin'
          : ''}"
      />
    </button>
  </div>

  {#if isLoading}
    <div class="space-y-2 sm:space-y-3">
      {#each Array(5) as _}
        <div class="flex animate-pulse items-center gap-3 p-2 sm:p-3">
          <div class="h-8 w-8 rounded-full bg-neutral-200"></div>
          <div class="flex-1">
            <div class="mb-1 h-4 w-1/2 rounded bg-neutral-200"></div>
            <div class="h-3 w-1/3 rounded bg-neutral-200"></div>
          </div>
        </div>
      {/each}
    </div>
  {:else if error}
    <div class="py-6 text-center sm:py-8">
      <p class="mb-3 text-sm text-red-600">{error}</p>
      <button
        onclick={handleRefresh}
        class="mx-auto flex items-center gap-2 text-sm text-neutral-600 hover:text-neutral-900"
      >
        <RefreshCw class="h-4 w-4 shrink-0" />
        Try Again
      </button>
    </div>
  {:else if leaderboard.length === 0}
    <div class="py-6 text-center sm:py-8">
      <Trophy class="mx-auto mb-3 h-10 w-10 text-neutral-300 sm:h-12 sm:w-12" />
      <p class="text-sm text-neutral-600">No leaderboard data</p>
    </div>
  {:else}
    <div class="space-y-2 sm:space-y-3">
      {#each leaderboard as entry, index}
        {@const rank = index + 1}
        {@const Icon = getRankIcon(rank)}
        <div
          class="flex items-center gap-2 rounded-lg p-2 transition-colors hover:bg-neutral-50 sm:gap-3 sm:p-3"
        >
          <!-- Rank Badge -->
          <div
            class="flex h-8 w-8 items-center justify-center rounded-full sm:h-10 sm:w-10 {getRankColor(
              rank,
            )} shrink-0"
          >
            {#if rank <= 3}
              <Icon class="h-4 w-4 shrink-0 sm:h-5 sm:w-5" />
            {:else}
              <span class="text-xs font-bold sm:text-sm">{rank}</span>
            {/if}
          </div>

          <!-- User Info -->
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <h3
                class="truncate text-sm font-semibold text-neutral-900 sm:text-base"
              >
                {entry.name || entry.username || "Anonymous"}
              </h3>
              {#if entry.verified}
                <div
                  class="flex h-4 w-4 shrink-0 items-center justify-center rounded-full bg-blue-500"
                >
                  <svg
                    class="h-2.5 w-2.5 text-white"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                      clip-rule="evenodd"
                    />
                  </svg>
                </div>
              {/if}
            </div>
            <div
              class="flex items-center gap-2 text-xs text-neutral-600 sm:text-sm"
            >
              <TrendingUp class="h-3 w-3 shrink-0 sm:h-4 sm:w-4" />
              <span
                >{formatNumber(entry.points || entry.votes || 0)} points</span
              >
            </div>
          </div>

          <!-- Stats -->
          <div class="shrink-0 text-right">
            <div class="text-xs font-bold text-neutral-900 sm:text-sm">
              {entry.contributionCount || 0}
            </div>
            <div class="text-[10px] text-neutral-600 sm:text-xs">
              contributions
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
