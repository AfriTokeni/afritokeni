<!--
 * Create Proposal Modal
 * Ported from: src/components/CreateProposalModal.tsx
 * 
 * Allows users to create governance proposals for the DAO
 -->
<script lang="ts">
	import { X, FileText, DollarSign, Globe, Shield, Lightbulb } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { DAO_CONSTANTS } from '$lib/services/data/daoData';
	import { toast } from '$lib/stores/toast';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		userId: string;
		userTokens: number;
		onSuccess: () => void;
	}

	let { isOpen, onClose, userId, userTokens, onSuccess }: Props = $props();

	// State
	let proposalType = $state<string>('other');
	let title = $state('');
	let description = $state('');
	let isLoading = $state(false);

	const proposalTypes = [
		{ value: 'fee_adjustment', label: 'Fee Adjustment', icon: DollarSign, color: 'text-green-600' },
		{ value: 'currency_addition', label: 'Add Currency', icon: Globe, color: 'text-blue-600' },
		{ value: 'agent_standards', label: 'Agent Standards', icon: Shield, color: 'text-purple-600' },
		{ value: 'treasury', label: 'Treasury Management', icon: FileText, color: 'text-orange-600' },
		{ value: 'other', label: 'Other', icon: Lightbulb, color: 'text-gray-600' },
	];

	const canCreateProposal = $derived(userTokens >= DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		
		if (!title || !description) {
			alert('Please fill in all fields');
			return;
		}

		isLoading = true;
		try {
			if ($demoMode) {
				// Demo mode: simulate proposal creation
				await new Promise(resolve => setTimeout(resolve, 1000));
				console.log('📋 Demo proposal created:', { type: proposalType, title, description });
				// TODO: Add to demo proposals store
			} else {
				// Real mode: Submit to SNS Governance
				console.log('📋 Creating SNS proposal:', { type: proposalType, title, description });
				// TODO: Implement SNS proposal creation
				// await createSNSProposal(userId, { type: proposalType, title, description, executionData: {} }, userTokens);
				throw new Error('SNS proposal creation not yet implemented');
			}
			
			// Success
			onSuccess();
			onClose();
			
			// Reset form
			title = '';
			description = '';
			proposalType = 'other';
		} catch (error: any) {
			console.error('Error creating proposal:', error);
			alert(error.message || 'Failed to create proposal');
		} finally {
			isLoading = false;
		}
	}

	function handleClose() {
		if (!isLoading) {
			onClose();
		}
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
			<!-- Header -->
			<div class="sticky top-0 bg-white border-b border-gray-200 p-6 flex items-center justify-between">
				<h2 class="text-2xl font-bold text-gray-900">Create Proposal</h2>
				<button
					onclick={handleClose}
					disabled={isLoading}
					class="p-2 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50"
					type="button"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Content -->
			<form onsubmit={handleSubmit} class="p-6 space-y-6">
				<!-- Token Requirement Warning -->
				{#if !canCreateProposal}
					<div class="bg-red-50 border border-red-200 rounded-lg p-4">
						<p class="text-red-800 font-semibold">Insufficient Tokens</p>
						<p class="text-red-600 text-sm mt-1">
							You need at least {DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE.toLocaleString()} AFRI tokens to create a proposal.
							You currently have {userTokens.toLocaleString()} AFRI.
						</p>
					</div>
				{/if}

				<!-- Proposal Type -->
				<div>
					<label class="block text-sm font-semibold text-gray-900 mb-3">
						Proposal Type
					</label>
					<div class="grid grid-cols-2 md:grid-cols-3 gap-3">
						{#each proposalTypes as type}
							{@const Icon = type.icon}
							<button
								type="button"
								onclick={() => proposalType = type.value}
								class="p-4 rounded-lg border-2 transition-all {proposalType === type.value
									? 'border-gray-900 bg-gray-50'
									: 'border-gray-200 hover:border-gray-300'}"
							>
								<Icon class="w-6 h-6 {type.color} mx-auto mb-2" />
								<p class="text-sm font-medium text-gray-900 text-center">
									{type.label}
								</p>
							</button>
						{/each}
					</div>
				</div>

				<!-- Title -->
				<div>
					<label for="proposal-title" class="block text-sm font-semibold text-gray-900 mb-2">
						Proposal Title
					</label>
					<input
						id="proposal-title"
						type="text"
						bind:value={title}
						placeholder="e.g., Reduce transaction fees by 10%"
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent"
						required
					/>
				</div>

				<!-- Description -->
				<div>
					<label for="proposal-description" class="block text-sm font-semibold text-gray-900 mb-2">
						Description
					</label>
					<textarea
						id="proposal-description"
						bind:value={description}
						placeholder="Explain your proposal in detail. Include rationale, expected impact, and implementation details..."
						rows={6}
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent resize-none"
						required
					></textarea>
				</div>

				<!-- Info Box -->
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
					<p class="text-blue-800 font-semibold mb-2">Voting Parameters</p>
					<ul class="text-blue-700 text-sm space-y-1">
						<li>• Voting period: {DAO_CONSTANTS.VOTING_PERIOD_DAYS} days</li>
						<li>• Quorum required: {DAO_CONSTANTS.QUORUM_PERCENTAGE}% of total supply</li>
						<li>• Passing threshold: {DAO_CONSTANTS.PASS_THRESHOLD}% yes votes</li>
						<li>• Your voting power: {userTokens.toLocaleString()} AFRI</li>
					</ul>
				</div>

				<!-- Actions -->
				<div class="flex gap-3 pt-4">
					<button
						type="button"
						onclick={handleClose}
						disabled={isLoading}
						class="flex-1 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-semibold disabled:opacity-50"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={isLoading || !canCreateProposal}
						class="flex-1 px-6 py-3 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{isLoading ? 'Creating...' : 'Create Proposal'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
