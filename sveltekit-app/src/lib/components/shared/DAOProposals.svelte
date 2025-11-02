<!--
 * DAO Proposals Component (FULLY ENCAPSULATED)
 * 
 * Self-contained component that:
 * - Subscribes to demoMode store internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for voting actions
 * 
 * Usage: <DAOProposals onVote={(proposalId, vote) => ...} />
-->
<script lang="ts">
	import { Vote, Clock, CheckCircle, XCircle, RefreshCw } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { fetchDAOProposals } from '$lib/services/data/daoData';

	interface Props {
		onVote?: (proposalId: string, vote: 'yes' | 'no' | 'abstain') => void;
		maxProposals?: number;
	}

	let {
		onVote,
		maxProposals = 5
	}: Props = $props();

	// Internal state
	let proposals = $state<any[]>([]);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);

	// Reactive: auto-refetch when demoMode changes
	$effect(() => {
		loadProposals($demoMode);
	});

	async function loadProposals(isDemoMode: boolean) {
		try {
			error = null;
			const data = await fetchDAOProposals(isDemoMode);
			proposals = data.slice(0, maxProposals);
		} catch (err: any) {
			console.error('Error fetching DAO proposals:', err);
			error = err.message || 'Failed to load proposals';
		} finally {
			isLoading = false;
			isRefreshing = false;
		}
	}

	async function handleRefresh() {
		isRefreshing = true;
		await loadProposals($demoMode);
	}

	function getStatusColor(status: string) {
		switch (status) {
			case 'active': return 'text-blue-600 bg-blue-50';
			case 'passed': return 'text-green-600 bg-green-50';
			case 'rejected': return 'text-red-600 bg-red-50';
			default: return 'text-neutral-600 bg-neutral-50';
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'active': return Clock;
			case 'passed': return CheckCircle;
			case 'rejected': return XCircle;
			default: return Vote;
		}
	}
</script>

<div class="bg-white rounded-xl shadow-sm border border-neutral-200 p-4 sm:p-5 md:p-6">
	<!-- Header -->
	<div class="flex items-center justify-between mb-4 sm:mb-6">
		<div class="flex items-center gap-2 sm:gap-3">
			<div class="p-2 bg-purple-50 rounded-lg">
				<Vote class="w-5 h-5 sm:w-6 sm:h-6 text-purple-600 shrink-0" />
			</div>
			<div>
				<h2 class="text-lg sm:text-xl font-bold text-neutral-900">DAO Proposals</h2>
				<p class="text-xs sm:text-sm text-neutral-600">Community Governance</p>
			</div>
		</div>
		<button
			onclick={handleRefresh}
			disabled={isRefreshing}
			class="p-2 hover:bg-neutral-100 rounded-lg transition-colors disabled:opacity-50"
			title="Refresh proposals"
		>
			<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-neutral-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
		</button>
	</div>

	{#if isLoading}
		<div class="space-y-3 sm:space-y-4">
			{#each Array(3) as _}
				<div class="animate-pulse p-3 sm:p-4 border border-neutral-200 rounded-lg">
					<div class="h-4 bg-neutral-200 rounded w-3/4 mb-2"></div>
					<div class="h-3 bg-neutral-200 rounded w-1/2"></div>
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
	{:else if proposals.length === 0}
		<div class="text-center py-6 sm:py-8">
			<Vote class="w-10 h-10 sm:w-12 sm:h-12 text-neutral-300 mx-auto mb-3" />
			<p class="text-sm text-neutral-600">No active proposals</p>
		</div>
	{:else}
		<div class="space-y-3 sm:space-y-4">
			{#each proposals as proposal}
				{@const StatusIcon = getStatusIcon(proposal.status)}
				<div class="p-3 sm:p-4 border border-neutral-200 rounded-lg hover:border-purple-200 transition-colors">
					<!-- Proposal Header -->
					<div class="flex items-start justify-between gap-2 mb-2 sm:mb-3">
						<div class="flex-1 min-w-0">
							<h3 class="text-sm sm:text-base font-semibold text-neutral-900 mb-1 truncate">
								{proposal.title}
							</h3>
							<p class="text-xs sm:text-sm text-neutral-600 line-clamp-2">
								{proposal.description}
							</p>
						</div>
						<div class="flex items-center gap-1.5 px-2 py-1 rounded-full {getStatusColor(proposal.status)} shrink-0">
							<StatusIcon class="w-3 h-3 shrink-0" />
							<span class="text-xs font-medium capitalize">{proposal.status}</span>
						</div>
					</div>

					<!-- Voting Stats -->
					<div class="grid grid-cols-3 gap-2 mb-3">
						<div class="text-center p-2 bg-green-50 rounded">
							<div class="text-xs text-neutral-600">Yes</div>
							<div class="text-sm font-bold text-green-600">{proposal.votesYes || 0}</div>
						</div>
						<div class="text-center p-2 bg-red-50 rounded">
							<div class="text-xs text-neutral-600">No</div>
							<div class="text-sm font-bold text-red-600">{proposal.votesNo || 0}</div>
						</div>
						<div class="text-center p-2 bg-neutral-50 rounded">
							<div class="text-xs text-neutral-600">Abstain</div>
							<div class="text-sm font-bold text-neutral-600">{proposal.votesAbstain || 0}</div>
						</div>
					</div>

					<!-- Vote Buttons (only for active proposals) -->
					{#if proposal.status === 'active' && onVote}
						<div class="grid grid-cols-3 gap-2">
							<button
								onclick={() => onVote?.(proposal.id, 'yes')}
								class="px-3 py-1.5 text-xs font-medium text-green-700 bg-green-50 hover:bg-green-100 rounded-lg transition-colors"
							>
								Vote Yes
							</button>
							<button
								onclick={() => onVote?.(proposal.id, 'no')}
								class="px-3 py-1.5 text-xs font-medium text-red-700 bg-red-50 hover:bg-red-100 rounded-lg transition-colors"
							>
								Vote No
							</button>
							<button
								onclick={() => onVote?.(proposal.id, 'abstain')}
								class="px-3 py-1.5 text-xs font-medium text-neutral-700 bg-neutral-50 hover:bg-neutral-100 rounded-lg transition-colors"
							>
								Abstain
							</button>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>
