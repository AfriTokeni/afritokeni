<script lang="ts">
    import {ChevronDown, Filter, Star, TrendingUp} from "@lucide/svelte";
    import {demoMode} from "$lib/stores/demoMode";
    import {principalId} from "$lib/stores/auth";
    import {listDocs} from "@junobuild/core";

    interface Props {
    agentData: any;
    expanded: boolean;
    onToggle: () => void;
  }

  let { agentData, expanded, onToggle }: Props = $props();

  let reviews = $state<any[]>([]);
  let selectedFilter = $state<number | null>(null);
  let isLoading = $state(false);

  // Load reviews when component mounts or demo mode changes
  $effect(() => {
    if (expanded) {
      loadReviews($demoMode, $principalId);
    }
  });

  async function loadReviews(
    isDemoMode: boolean,
    agentPrincipalId: string | null,
  ) {
    isLoading = true;

    if (isDemoMode) {
      // Demo data
      reviews = [
        {
          id: "1",
          customerName: "John Kamau",
          rating: 5,
          comment:
            "Excellent service! Very professional and quick transaction.",
          date: "2024-01-15",
          transactionType: "Cash Deposit",
        },
        {
          id: "2",
          customerName: "Mary Achieng",
          rating: 4,
          comment: "Good agent, but had to wait a bit. Overall satisfied.",
          date: "2024-01-14",
          transactionType: "Withdrawal",
        },
        {
          id: "3",
          customerName: "David Omondi",
          rating: 5,
          comment: "Best agent in the area! Always available and helpful.",
          date: "2024-01-13",
          transactionType: "Bitcoin Exchange",
        },
        {
          id: "4",
          customerName: "Sarah Wanjiku",
          rating: 3,
          comment: "Service was okay, but location was hard to find.",
          date: "2024-01-12",
          transactionType: "Cash Deposit",
        },
        {
          id: "5",
          customerName: "Peter Mutua",
          rating: 5,
          comment: "Very trustworthy and efficient. Highly recommend!",
          date: "2024-01-11",
          transactionType: "Withdrawal",
        },
      ];
      isLoading = false;
      return;
    }

    if (!agentPrincipalId) {
      reviews = [];
      isLoading = false;
      return;
    }

    try {
      // Load reviews from Juno
      const { items } = await listDocs({
        collection: "agent_reviews",
        filter: {
          matcher: {
            key: agentPrincipalId,
          },
        },
      });

      reviews = items.map((item) => ({
        id: item.key,
        ...item.data,
      }));
    } catch (error) {
      console.error("Failed to load reviews:", error);
      reviews = [];
    } finally {
      isLoading = false;
    }
  }

  // Calculate rating distribution
  const ratingDistribution = $derived(() => {
    const dist = { 5: 0, 4: 0, 3: 0, 2: 0, 1: 0 };
    reviews.forEach((r) => dist[r.rating as keyof typeof dist]++);
    return dist;
  });

  const filteredReviews = $derived(
    selectedFilter
      ? reviews.filter((r) => r.rating === selectedFilter)
      : reviews,
  );

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }

  function getStarPercentage(stars: number): number {
    const total = reviews.length;
    return (
      (ratingDistribution()[
        stars as keyof ReturnType<typeof ratingDistribution>
      ] /
        total) *
      100
    );
  }
</script>

<div class="overflow-hidden rounded-xl border border-gray-200 bg-white">
  <!-- Header -->
  <button
    onclick={onToggle}
    class="flex w-full items-center justify-between px-4 py-4 transition-colors hover:bg-gray-50 sm:px-6"
  >
    <div class="flex items-center gap-3">
      <div class="rounded-lg bg-yellow-100 p-2">
        <Star class="h-5 w-5 text-yellow-600" />
      </div>
      <div class="text-left">
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          Reviews & Ratings
        </h3>
        <p class="text-xs text-gray-600 sm:text-sm">
          {agentData.totalReviews} reviews • {agentData.rating.toFixed(1)} average
        </p>
      </div>
    </div>
    <ChevronDown
      class="h-5 w-5 text-gray-400 transition-transform {expanded
        ? 'rotate-180'
        : ''}"
    />
  </button>

  <!-- Content -->
  {#if expanded}
    <div class="space-y-6 px-4 pb-6 sm:px-6">
      <!-- Rating Overview -->
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
        <!-- Average Rating -->
        <div class="rounded-xl bg-linear-to-br from-yellow-50 to-orange-50 p-6">
          <div class="text-center">
            <div class="mb-2 text-5xl font-bold text-gray-900">
              {agentData.rating.toFixed(1)}
            </div>
            <div class="mb-2 flex items-center justify-center gap-1">
              {#each Array(5) as _, i}
                <Star
                  class="h-5 w-5 {i < Math.round(agentData.rating)
                    ? 'fill-yellow-500 text-yellow-500'
                    : 'text-gray-300'}"
                />
              {/each}
            </div>
            <p class="text-sm text-gray-600">
              {agentData.totalReviews} total reviews
            </p>
          </div>
        </div>

        <!-- Rating Distribution -->
        <div class="space-y-2">
          {#each [5, 4, 3, 2, 1] as stars}
            <button
              onclick={() =>
                (selectedFilter = selectedFilter === stars ? null : stars)}
              class="flex w-full items-center gap-3 rounded-lg p-2 transition-colors hover:bg-gray-50 {selectedFilter ===
              stars
                ? 'bg-yellow-50'
                : ''}"
            >
              <div class="flex w-16 items-center gap-1">
                <span class="text-sm font-medium text-gray-700">{stars}</span>
                <Star class="h-4 w-4 fill-yellow-500 text-yellow-500" />
              </div>
              <div class="h-2 flex-1 rounded-full bg-gray-200">
                <div
                  class="h-2 rounded-full bg-yellow-500 transition-all"
                  style="width: {getStarPercentage(stars)}%"
                ></div>
              </div>
              <span class="w-12 text-right text-sm text-gray-600">
                {ratingDistribution()[
                  stars as keyof ReturnType<typeof ratingDistribution>
                ]}
              </span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Filter Info -->
      {#if selectedFilter}
        <div
          class="flex items-center justify-between rounded-lg border border-yellow-200 bg-yellow-50 p-3"
        >
          <div class="flex items-center gap-2">
            <Filter class="h-4 w-4 text-yellow-600" />
            <span class="text-sm font-medium text-yellow-900">
              Showing {selectedFilter}-star reviews
            </span>
          </div>
          <button
            onclick={() => (selectedFilter = null)}
            class="text-sm font-medium text-yellow-700 hover:text-yellow-900"
          >
            Clear filter
          </button>
        </div>
      {/if}

      <!-- Reviews List with Scroll -->
      <div class="max-h-96 space-y-4 overflow-y-auto pr-2">
        {#if isLoading}
          <div class="py-8 text-center text-gray-500">
            <div
              class="mx-auto mb-3 h-8 w-8 animate-spin rounded-full border-4 border-gray-300 border-t-gray-600"
            ></div>
            <p>Loading reviews...</p>
          </div>
        {:else if filteredReviews.length === 0}
          <div class="py-8 text-center text-gray-500">
            <Star class="mx-auto mb-3 h-12 w-12 text-gray-300" />
            <p>
              {selectedFilter
                ? "No reviews match this filter"
                : "No reviews yet"}
            </p>
          </div>
        {:else}
          {#each filteredReviews as review (review.id)}
            <div
              class="rounded-lg border border-gray-200 p-4 transition-colors hover:border-gray-300"
            >
              <div class="mb-2 flex items-start justify-between">
                <div>
                  <h4 class="font-semibold text-gray-900">
                    {review.customerName}
                  </h4>
                  <p class="text-xs text-gray-500">
                    {formatDate(review.date)} • {review.transactionType}
                  </p>
                </div>
                <div class="flex items-center gap-1">
                  {#each Array(5) as _, i}
                    <Star
                      class="h-4 w-4 {i < review.rating
                        ? 'fill-yellow-500 text-yellow-500'
                        : 'text-gray-300'}"
                    />
                  {/each}
                </div>
              </div>
              <p class="text-sm text-gray-700">{review.comment}</p>
            </div>
          {/each}
        {/if}
      </div>

      <!-- Stats -->
      <div class="grid grid-cols-2 gap-4 border-t border-gray-200 pt-4">
        <div class="text-center">
          <div class="mb-1 flex items-center justify-center gap-2">
            <TrendingUp class="h-5 w-5 text-green-600" />
            <span class="text-2xl font-bold text-gray-900">
              {Math.round((ratingDistribution()[5] / reviews.length) * 100)}%
            </span>
          </div>
          <p class="text-sm text-gray-600">5-star reviews</p>
        </div>
        <div class="text-center">
          <div class="mb-1 flex items-center justify-center gap-2">
            <Star class="h-5 w-5 fill-yellow-500 text-yellow-500" />
            <span class="text-2xl font-bold text-gray-900"
              >{agentData.rating.toFixed(2)}</span
            >
          </div>
          <p class="text-sm text-gray-600">Average rating</p>
        </div>
      </div>
    </div>
  {/if}
</div>
