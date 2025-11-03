<script lang="ts">
	import type { LeaderboardEntry } from '$lib/utils/dao';

	interface Props {
		leaderboard: LeaderboardEntry[];
		totalSupply: number;
	}

	let { leaderboard, totalSupply }: Props = $props();
</script>

<div class="bg-white rounded-lg sm:rounded-xl p-4 sm:p-6 border border-gray-200">
	<h3 class="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4">Top Token Holders</h3>
	{#if leaderboard.length === 0}
		<div class="text-center py-6 sm:py-8 text-sm sm:text-base text-gray-500">
			Loading leaderboard data...
		</div>
	{:else}
		<div class="space-y-2.5 sm:space-y-3">
			{#each leaderboard as holder}
				<div class="flex items-center justify-between p-2.5 sm:p-3 bg-gray-50 rounded-lg gap-2">
					<div class="flex items-center gap-2 sm:gap-3 min-w-0 flex-1">
						<div class="w-7 h-7 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-sm sm:text-base font-bold shrink-0 {
							holder.rank === 1 ? 'bg-yellow-400 text-yellow-900' :
							holder.rank === 2 ? 'bg-gray-300 text-gray-700' :
							holder.rank === 3 ? 'bg-orange-400 text-orange-900' :
							'bg-gray-200 text-gray-600'
						}">
							{holder.rank}
						</div>
						<div class="min-w-0">
							<div class="text-sm sm:text-base font-semibold text-gray-900 truncate">{holder.address}</div>
							<div class="text-xs sm:text-sm text-gray-500 truncate">Token Holder</div>
						</div>
					</div>
					<div class="text-right shrink-0">
						<div class="text-sm sm:text-base font-bold text-gray-900 whitespace-nowrap">
							{holder.balance.toLocaleString()} AFRI
						</div>
						<div class="text-xs sm:text-sm text-gray-500 whitespace-nowrap">
							{holder.percentage}% voting power
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
