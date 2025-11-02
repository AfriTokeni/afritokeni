<script lang="ts">
	import { onMount } from 'svelte';
	import { Vote, TrendingUp, Users, Coins } from '@lucide/svelte';
	import { getUserData } from '$lib/services/user/userService';
	import DAOStats from './DAOStats.svelte';
	import TokensTab from './TokensTab.svelte';
	import DAOProposals from '$lib/components/shared/DAOProposals.svelte';
	import Leaderboard from '$lib/components/shared/Leaderboard.svelte';

	type Tab = 'proposals' | 'my-tokens' | 'leaderboard';

	// State
	let activeTab = $state<Tab>('proposals');
	let currentUser = $state<any>(null);
	let tokenBalance = $state(0);
	let totalSupply = $state(1000000); // Mock for now
	let totalHolders = $state(0);
	let activeProposalsCount = $state(0);

	onMount(async () => {
		// Only load user-specific data
		currentUser = await getUserData();
		tokenBalance = currentUser?.daoTokens || 0;
	});

	function handleVote(proposalId: string, choice: 'yes' | 'no' | 'abstain') {
		console.log('Vote:', proposalId, choice);
		// TODO: Implement real voting
	}
</script>

<div class="space-y-6">
	<!-- Stats -->
	<DAOStats
		{tokenBalance}
		{totalSupply}
		{totalHolders}
		{activeProposalsCount}
	/>

	<!-- Tabs -->
	<div class="border-b border-gray-200">
		<nav class="-mb-px flex space-x-8 overflow-x-auto">
			<button
				onclick={() => activeTab = 'proposals'}
				class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm {activeTab === 'proposals' ? 'border-gray-900 text-gray-900' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
			>
				<div class="flex items-center gap-2">
					<Vote class="w-5 h-5" />
					Proposals
				</div>
			</button>
			<button
				onclick={() => activeTab = 'my-tokens'}
				class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm {activeTab === 'my-tokens' ? 'border-gray-900 text-gray-900' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
			>
				<div class="flex items-center gap-2">
					<Coins class="w-5 h-5" />
					My Tokens
				</div>
			</button>
			<button
				onclick={() => activeTab = 'leaderboard'}
				class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm {activeTab === 'leaderboard' ? 'border-gray-900 text-gray-900' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
			>
				<div class="flex items-center gap-2">
					<TrendingUp class="w-5 h-5" />
					Leaderboard
				</div>
			</button>
		</nav>
	</div>

	<!-- Tab Content -->
	{#if activeTab === 'proposals'}
		<!-- Encapsulated component - fetches own data -->
		<DAOProposals onVote={handleVote} maxProposals={10} />
	{:else if activeTab === 'my-tokens'}
		<TokensTab balance={tokenBalance} {totalSupply} breakdown={currentUser?.daoTokensBreakdown} />
	{:else if activeTab === 'leaderboard'}
		<!-- Encapsulated component - fetches own data -->
		<Leaderboard maxEntries={20} />
	{/if}
</div>
