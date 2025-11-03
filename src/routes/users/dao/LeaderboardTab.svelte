<script lang="ts">
  import type { LeaderboardEntry } from "$lib/utils/dao";

  interface Props {
    leaderboard: LeaderboardEntry[];
    totalSupply: number;
  }

  let { leaderboard, totalSupply }: Props = $props();
</script>

<div
  class="rounded-lg border border-gray-200 bg-white p-4 sm:rounded-xl sm:p-6"
>
  <h3 class="mb-3 text-base font-bold text-gray-900 sm:mb-4 sm:text-lg">
    Top Token Holders
  </h3>
  {#if leaderboard.length === 0}
    <div class="py-6 text-center text-sm text-gray-500 sm:py-8 sm:text-base">
      Loading leaderboard data...
    </div>
  {:else}
    <div class="space-y-2.5 sm:space-y-3">
      {#each leaderboard as holder}
        <div
          class="flex items-center justify-between gap-2 rounded-lg bg-gray-50 p-2.5 sm:p-3"
        >
          <div class="flex min-w-0 flex-1 items-center gap-2 sm:gap-3">
            <div
              class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full text-sm font-bold sm:h-8 sm:w-8 sm:text-base {holder.rank ===
              1
                ? 'bg-yellow-400 text-yellow-900'
                : holder.rank === 2
                  ? 'bg-gray-300 text-gray-700'
                  : holder.rank === 3
                    ? 'bg-orange-400 text-orange-900'
                    : 'bg-gray-200 text-gray-600'}"
            >
              {holder.rank}
            </div>
            <div class="min-w-0">
              <div
                class="truncate text-sm font-semibold text-gray-900 sm:text-base"
              >
                {holder.address}
              </div>
              <div class="truncate text-xs text-gray-500 sm:text-sm">
                Token Holder
              </div>
            </div>
          </div>
          <div class="shrink-0 text-right">
            <div
              class="text-sm font-bold whitespace-nowrap text-gray-900 sm:text-base"
            >
              {holder.balance.toLocaleString()} AFRI
            </div>
            <div class="text-xs whitespace-nowrap text-gray-500 sm:text-sm">
              {holder.percentage}% voting power
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
