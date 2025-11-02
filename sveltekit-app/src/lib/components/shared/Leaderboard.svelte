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
	import { Trophy, TrendingUp, Award, RefreshCw } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { fetchLeaderboard } from '$lib/services/data/daoData';

	interface Props {
		maxEntries?: number;
	}

	let {
		maxEntries = 10
	}: Props = $props();

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
			const data = await fetchLeaderboard(isDemoMode);
			leaderboard = data.slice(0, maxEntries);
		} catch (err: any) {
			console.error('Error fetching leaderboard:', err);
			error = err.message || 'Failed to load leaderboard';
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
		if (rank === 1) return 'text-yellow-600 bg-yellow-50';
		if (rank === 2) return 'text-gray-600 bg-gray-50';
		if (rank === 3) return 'text-orange-600 bg-orange-50';
		return 'text-neutral-600 bg-neutral-50';
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

<div class="bg-white rounded-xl shadow-sm border border-neutral-200 p-4 sm:p-5 md:p-6">
	<!-- Header -->
	<div class="flex items-center justify-between mb-4 sm:mb-6">
		<div class="flex items-center gap-2 sm:gap-3">
			<div class="p-2 bg-yellow-50 rounded-lg">
				<Trophy class="w-5 h-5 sm:w-6 sm:h-6 text-yellow-600 shrink-0" />
			</div>
			<div>
				<h2 class="text-lg sm:text-xl font-bold text-neutral-900">Leaderboard</h2>
				<p class="text-xs sm:text-sm text-neutral-600">Top Contributors</p>
			</div>
		</div>
		<button
			onclick={handleRefresh}
			disabled={isRefreshing}
			class="p-2 hover:bg-neutral-100 rounded-lg transition-colors disabled:opacity-50"
			title="Refresh leaderboard"
		>
			<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-neutral-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
		</button>
	</div>

	{#if isLoading}
		<div class="space-y-2 sm:space-y-3">
			{#each Array(5) as _}
				<div class="animate-pulse flex items-center gap-3 p-2 sm:p-3">
					<div class="h-8 w-8 bg-neutral-200 rounded-full"></div>
					<div class="flex-1">
						<div class="h-4 bg-neutral-200 rounded w-1/2 mb-1"></div>
						<div class="h-3 bg-neutral-200 rounded w-1/3"></div>
					</div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div class="text-center py-6 sm:py-8">
			<p class="text-sm text-red-600 mb-3">{error}</p>
			<button
				onclick={handleRefresh}
				class="text-sm text-neutral-600 hover:text-neutral-900 flex items-center gap-2 mx-auto"
			>
				<RefreshCw class="w-4 h-4 shrink-0" />
				Try Again
			</button>
		</div>
	{:else if leaderboard.length === 0}
		<div class="text-center py-6 sm:py-8">
			<Trophy class="w-10 h-10 sm:w-12 sm:h-12 text-neutral-300 mx-auto mb-3" />
			<p class="text-sm text-neutral-600">No leaderboard data</p>
		</div>
	{:else}
		<div class="space-y-2 sm:space-y-3">
			{#each leaderboard as entry, index}
				{@const rank = index + 1}
				{@const Icon = getRankIcon(rank)}
				<div class="flex items-center gap-2 sm:gap-3 p-2 sm:p-3 rounded-lg hover:bg-neutral-50 transition-colors">
					<!-- Rank Badge -->
					<div class="flex items-center justify-center w-8 h-8 sm:w-10 sm:h-10 rounded-full {getRankColor(rank)} shrink-0">
						{#if rank <= 3}
							<Icon class="w-4 h-4 sm:w-5 sm:h-5 shrink-0" />
						{:else}
							<span class="text-xs sm:text-sm font-bold">{rank}</span>
						{/if}
					</div>

					<!-- User Info -->
					<div class="flex-1 min-w-0">
						<div class="flex items-center gap-2">
							<h3 class="text-sm sm:text-base font-semibold text-neutral-900 truncate">
								{entry.name || entry.username || 'Anonymous'}
							</h3>
							{#if entry.verified}
								<div class="w-4 h-4 bg-blue-500 rounded-full flex items-center justify-center shrink-0">
									<svg class="w-2.5 h-2.5 text-white" fill="currentColor" viewBox="0 0 20 20">
										<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
									</svg>
								</div>
							{/if}
						</div>
						<div class="flex items-center gap-2 text-xs sm:text-sm text-neutral-600">
							<TrendingUp class="w-3 h-3 sm:w-4 sm:h-4 shrink-0" />
							<span>{formatNumber(entry.points || entry.votes || 0)} points</span>
						</div>
					</div>

					<!-- Stats -->
					<div class="text-right shrink-0">
						<div class="text-xs sm:text-sm font-bold text-neutral-900">
							{entry.contributionCount || 0}
						</div>
						<div class="text-[10px] sm:text-xs text-neutral-600">
							contributions
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
