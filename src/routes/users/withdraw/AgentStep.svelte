<script lang="ts">
	type Agent = {
		id: string;
		businessName: string;
		location: { city: string; latitude: number; longitude: number };
	};

	interface Props {
		userLocation: [number, number] | null;
		locationError: string | null;
		localAmount: number;
		btcAmount: string;
		userCurrency: string;
		onBackToAmount: () => void;
		onAgentSelect: (agent: Agent) => void;
		isCreatingTransaction: boolean;
		transactionError: string | null;
	}

	let { userLocation, locationError, localAmount, btcAmount, userCurrency, onBackToAmount, onAgentSelect, isCreatingTransaction, transactionError }: Props = $props();

	// Mock agents
	const mockAgents: Agent[] = [
		{ id: '1', businessName: 'Kampala Cash Point', location: { city: 'Kampala', latitude: 0.3476, longitude: 32.5825 } },
		{ id: '2', businessName: 'Downtown Money Services', location: { city: 'Kampala', latitude: 0.3136, longitude: 32.5811 } }
	];
</script>

<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-4 sm:p-6 lg:p-8">
	<h2 class="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">Select Agent</h2>
	
	<p class="text-sm text-gray-600 mb-4">Amount to withdraw: {userCurrency} {localAmount.toLocaleString()}</p>

	<div class="space-y-3">
		{#each mockAgents as agent}
			<button
				onclick={() => onAgentSelect(agent)}
				class="w-full p-4 border border-gray-200 rounded-lg hover:border-gray-900 transition-colors text-left"
			>
				<div class="font-semibold text-gray-900">{agent.businessName}</div>
				<div class="text-sm text-gray-600">{agent.location.city}</div>
			</button>
		{/each}
	</div>

	<button
		onclick={onBackToAmount}
		class="w-full mt-4 bg-gray-100 text-gray-700 py-3 rounded-lg font-semibold hover:bg-gray-200"
	>
		Back
	</button>
</div>
