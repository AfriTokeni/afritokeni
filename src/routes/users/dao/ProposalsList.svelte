<script lang="ts">
	import { CheckCircle, XCircle, Clock, DollarSign, Globe, Shield, FileText, Lightbulb } from '@lucide/svelte';
	import { getTotalVotes, calculateVotePercentage, type Proposal } from '$lib/utils/dao';

	interface Props {
		proposals: Proposal[];
		onVote: (proposalId: string, choice: 'yes' | 'no' | 'abstain') => void;
	}

	let { proposals, onVote }: Props = $props();

	function getProposalTypeInfo(category: string) {
		const types: Record<string, { icon: any; label: string; color: string }> = {
			'Economic': { icon: DollarSign, label: 'Economic', color: 'bg-green-100 text-green-700' },
			'Feature': { icon: Lightbulb, label: 'Feature', color: 'bg-blue-100 text-blue-700' },
			'Security': { icon: Shield, label: 'Security', color: 'bg-red-100 text-red-700' },
			'Governance': { icon: FileText, label: 'Governance', color: 'bg-purple-100 text-purple-700' },
			'Community': { icon: Globe, label: 'Community', color: 'bg-yellow-100 text-yellow-700' }
		};
		return types[category] || types['Community'];
	}

	function getStatusColor(status: string) {
		const colors: Record<string, string> = {
			'active': 'bg-blue-100 text-blue-700',
			'passed': 'bg-green-100 text-green-700',
			'rejected': 'bg-red-100 text-red-700',
			'pending': 'bg-gray-100 text-gray-700'
		};
		return colors[status] || colors['pending'];
	}
</script>

<div class="space-y-3 sm:space-y-4">
	{#each proposals as proposal}
		{@const totalVotes = getTotalVotes(proposal)}
		{@const yesPercentage = calculateVotePercentage(proposal.votesYes, totalVotes)}
		{@const noPercentage = calculateVotePercentage(proposal.votesNo, totalVotes)}
		{@const typeInfo = getProposalTypeInfo(proposal.category)}
		{@const TypeIcon = typeInfo.icon}

		<div class="bg-white rounded-lg sm:rounded-xl p-4 sm:p-6 border border-gray-200">
			<div class="flex items-start justify-between mb-3 sm:mb-4">
				<div class="flex-1 min-w-0">
					<div class="flex flex-wrap items-center gap-1.5 sm:gap-2 mb-2 sm:mb-3">
						<span class="px-2 sm:px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1 sm:gap-1.5 {typeInfo.color}">
							<TypeIcon class="w-3 h-3 sm:w-3.5 sm:h-3.5 shrink-0" />
							<span class="whitespace-nowrap">{typeInfo.label}</span>
						</span>
						<span class="px-2 sm:px-3 py-1 rounded-full text-xs font-semibold whitespace-nowrap {getStatusColor(proposal.status)}">
							{proposal.status.toUpperCase()}
						</span>
						<span class="text-xs text-gray-400 truncate">{proposal.id}</span>
					</div>
					<h3 class="text-base sm:text-lg lg:text-xl font-bold text-gray-900 mb-2 wrap-break-word">{proposal.title}</h3>
					<p class="text-sm sm:text-base text-gray-600 mb-3 sm:mb-4 wrap-break-word">{proposal.description}</p>
				</div>
			</div>

			<!-- Voting Progress -->
			<div class="mb-3 sm:mb-4">
				<div class="flex items-center justify-between text-xs sm:text-sm mb-2">
					<span class="text-gray-600">Voting Progress</span>
					<span class="text-gray-900 font-semibold">{totalVotes.toLocaleString()} votes</span>
				</div>
				<div class="h-2 bg-gray-100 rounded-full overflow-hidden flex">
					<div class="bg-green-500" style="width: {yesPercentage}%"></div>
					<div class="bg-red-500" style="width: {noPercentage}%"></div>
				</div>
				<div class="flex items-center justify-between mt-2 text-xs sm:text-sm gap-2">
					<span class="text-green-600 font-semibold">{yesPercentage}% Yes ({(proposal.votes?.yes || 0).toLocaleString()})</span>
					<span class="text-red-600 font-semibold">{noPercentage}% No ({(proposal.votes?.no || 0).toLocaleString()})</span>
				</div>
			</div>

			<!-- Voting Buttons -->
			{#if proposal.status === 'active'}
				<div class="flex flex-col sm:flex-row gap-2 sm:gap-3">
					<button
						onclick={() => onVote(proposal.id, 'yes')}
						class="flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 sm:px-4 sm:py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-sm sm:text-base"
					>
						<CheckCircle class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
						<span class="hidden xs:inline">Vote </span>Yes
					</button>
					<button
						onclick={() => onVote(proposal.id, 'no')}
						class="flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 sm:px-4 sm:py-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors text-sm sm:text-base"
					>
						<XCircle class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
						<span class="hidden xs:inline">Vote </span>No
					</button>
					<button
						onclick={() => onVote(proposal.id, 'abstain')}
						class="px-4 py-2.5 sm:px-6 sm:py-3 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors text-sm sm:text-base"
					>
						Abstain
					</button>
				</div>
			{/if}

			<!-- Time Remaining -->
			<div class="flex items-center gap-1.5 sm:gap-2 mt-3 sm:mt-4 text-xs sm:text-sm text-gray-500">
				<Clock class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" />
				<span class="wrap-break-word">Ends: {new Date(proposal.votingEndsAt || 0).toLocaleDateString()}</span>
			</div>
		</div>
	{/each}
</div>
