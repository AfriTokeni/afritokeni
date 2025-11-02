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
 * Usage: <DAOProposals onVote={(proposalId, vote) => ...} onCreateProposal={() => ...} />
-->
<script lang="ts">
	import { Vote, Clock, CheckCircle, XCircle, RefreshCw, Plus } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { fetchDAOProposals, DAO_CONSTANTS } from '$lib/services/data/daoData';

	interface Props {
		onVote?: (proposalId: string, vote: 'yes' | 'no' | 'abstain') => void;
		onCreateProposal?: () => void;
		maxProposals?: number;
		userTokenBalance?: number;
	}

	let {
		onVote,
		onCreateProposal,
		maxProposals = 5,
		userTokenBalance = 0
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
			isLoading = true; // Ensure loading state is set
			const data = await fetchDAOProposals(isDemoMode, maxProposals);
			proposals = data;
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
			<Vote class="w-5 h-5 sm:w-6 sm:h-6 text-purple-600 shrink-0" />
			<h2 class="text-base sm:text-lg font-bold text-neutral-900">Active Proposals</h2>
		</div>
		<div class="flex items-center gap-2">
			{#if onCreateProposal}
				<button
					onclick={onCreateProposal}
					disabled={userTokenBalance < DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE}
					class="flex items-center gap-1.5 px-3 py-1.5 text-xs sm:text-sm font-medium text-white bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					title={userTokenBalance < DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE ? `Need ${DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE} AFRI to create proposal` : 'Create new proposal'}
				>
					<Plus class="w-4 h-4 shrink-0" />
					<span class="hidden sm:inline">Create</span>
				</button>
			{/if}
			<button
				onclick={handleRefresh}
				disabled={isRefreshing}
				class="p-2 hover:bg-neutral-100 rounded-lg transition-colors disabled:opacity-50"
				title="Refresh proposals"
			>
				<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-neutral-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
			</button>
		</div>
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
		<div class="text-center py-8 sm:py-12">
			<Vote class="w-12 h-12 sm:w-16 sm:h-16 text-neutral-300 mx-auto mb-4" />
			<h3 class="text-base sm:text-lg font-semibold text-neutral-900 mb-2">No Active Proposals</h3>
			<p class="text-sm text-neutral-600 mb-4">Be the first to create a governance proposal</p>
			
			{#if onCreateProposal}
				{#if userTokenBalance >= DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE}
					<button
						onclick={onCreateProposal}
						class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors"
					>
						<Plus class="w-4 h-4 shrink-0" />
						Create Proposal
					</button>
				{:else}
					<div class="inline-block px-4 py-3 bg-yellow-50 border border-yellow-200 rounded-lg">
						<p class="text-sm text-yellow-800">
							<strong>Need {DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE} AFRI tokens</strong> to create a proposal
						</p>
						<p class="text-xs text-yellow-700 mt-1">
							You have {userTokenBalance.toFixed(2)} AFRI
						</p>
					</div>
				{/if}
			{/if}
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
							<div class="text-sm font-bold text-green-600">{proposal.votes?.yes?.toLocaleString() || 0}</div>
						</div>
						<div class="text-center p-2 bg-red-50 rounded">
							<div class="text-xs text-neutral-600">No</div>
							<div class="text-sm font-bold text-red-600">{proposal.votes?.no?.toLocaleString() || 0}</div>
						</div>
						<div class="text-center p-2 bg-neutral-50 rounded">
							<div class="text-xs text-neutral-600">Abstain</div>
							<div class="text-sm font-bold text-neutral-600">{proposal.votes?.abstain?.toLocaleString() || 0}</div>
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
