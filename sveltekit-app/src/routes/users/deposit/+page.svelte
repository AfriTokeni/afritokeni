<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft, MapPin, Check } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	import { demoMode } from '$lib/stores/demoMode';
	import { fetchAgents } from '$lib/services/data/agentsData';
	import { onMount } from 'svelte';
	
	// State
	let step = $state<'amount' | 'agent' | 'confirmation'>('amount');
	let amount = $state('');
	let selectedAgent = $state<any>(null);
	let depositCode = $state('');
	let isCreating = $state(false);
	let agents = $state<any[]>([]);
	let isLoadingAgents = $state(false);
	
	// Load agents when component mounts
	onMount(async () => {
		await loadAgents();
	});
	
	async function loadAgents() {
		isLoadingAgents = true;
		try {
			agents = await fetchAgents($demoMode);
		} catch (error) {
			console.error('Failed to load agents:', error);
			toast.show('error', 'Failed to load agents');
		} finally {
			isLoadingAgents = false;
		}
	}
	
	// Progress steps
	const steps = [
		{ key: 'amount', label: 'Amount', number: 1 },
		{ key: 'agent', label: 'Select Agent', number: 2 },
		{ key: 'confirmation', label: 'Confirmation', number: 3 }
	];
	
	function handleAmountSubmit() {
		if (!amount || parseFloat(amount) <= 0) {
			toast.show('error', 'Please enter a valid amount');
			return;
		}
		step = 'agent';
	}
	
	function handleAgentSelect(agent: any) {
		selectedAgent = agent;
		step = 'confirmation';
	}
	
	async function handleConfirm() {
		isCreating = true;
		try {
			// Simulate deposit creation
			await new Promise(resolve => setTimeout(resolve, 1000));
			depositCode = Math.random().toString(36).substring(2, 8).toUpperCase();
			toast.show('success', 'Deposit request created!');
		} catch (error) {
			toast.show('error', 'Failed to create deposit');
		} finally {
			isCreating = false;
		}
	}
	
	function goBack() {
		if (step === 'agent') step = 'amount';
		else if (step === 'confirmation') step = 'agent';
		else goto('/users/dashboard');
	}
</script>

<div class="max-w-2xl mx-auto">
	<!-- Header -->
	<div class="flex items-center gap-4 mb-6">
		<button onclick={goBack} class="p-2 hover:bg-gray-100 rounded-lg">
			<ArrowLeft class="w-5 h-5" />
		</button>
		<h1 class="text-2xl font-bold">Deposit Cash</h1>
	</div>
	
	<!-- Progress Indicator -->
	<div class="mb-8">
		<div class="flex items-center justify-between">
			{#each steps as s, i}
				<div class="flex items-center {i < steps.length - 1 ? 'flex-1' : ''}">
					<div class="flex flex-col items-center">
						<div class="w-10 h-10 rounded-full flex items-center justify-center {
							s.key === step ? 'bg-purple-600 text-white' :
							steps.findIndex(x => x.key === step) > i ? 'bg-green-600 text-white' :
							'bg-gray-200 text-gray-600'
						}">
							{#if steps.findIndex(x => x.key === step) > i}
								<Check class="w-5 h-5" />
							{:else}
								{s.number}
							{/if}
						</div>
						<span class="text-xs mt-2 {s.key === step ? 'text-purple-600 font-semibold' : 'text-gray-600'}">{s.label}</span>
					</div>
					{#if i < steps.length - 1}
						<div class="flex-1 h-0.5 mx-4 {steps.findIndex(x => x.key === step) > i ? 'bg-green-600' : 'bg-gray-200'}"></div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
	
	<!-- Step Content -->
	<div class="bg-white rounded-xl border border-gray-200 p-6">
		{#if step === 'amount'}
			<h2 class="text-xl font-bold mb-4">Enter Amount</h2>
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">Amount (UGX)</label>
					<input
						type="number"
						bind:value={amount}
						placeholder="Enter amount"
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
					/>
				</div>
				<button
					onclick={handleAmountSubmit}
					class="w-full bg-purple-600 text-white py-3 rounded-lg hover:bg-purple-700 font-semibold"
				>
					Continue
				</button>
			</div>
		{:else if step === 'agent'}
			<h2 class="text-xl font-bold mb-4">Select Agent</h2>
			
			{#if isLoadingAgents}
				<div class="text-center py-8">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-600 mx-auto"></div>
					<p class="text-gray-600 mt-2">Loading agents...</p>
				</div>
			{:else if agents.length === 0}
				<div class="text-center py-8">
					<p class="text-gray-600">No agents available</p>
				</div>
			{:else}
				<div class="space-y-3">
					{#each agents as agent}
						<button
							onclick={() => handleAgentSelect(agent)}
							disabled={!agent.isOnline}
							class="w-full p-4 border border-gray-200 rounded-lg hover:border-purple-600 text-left disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div class="flex items-center justify-between">
								<div>
									<p class="font-semibold">{agent.businessName}</p>
									<div class="flex items-center gap-1 text-sm text-gray-600 mt-1">
										<MapPin class="w-4 h-4" />
										<span>{agent.location?.address || 'Location not available'}</span>
									</div>
								</div>
								<div class="text-sm {agent.isOnline ? 'text-green-600' : 'text-red-600'}">
									{agent.isOnline ? 'Available' : 'Offline'}
								</div>
							</div>
						</button>
					{/each}
				</div>
			{/if}
		{:else if step === 'confirmation'}
			<h2 class="text-xl font-bold mb-4">Confirm Deposit</h2>
			
			{#if !depositCode}
				<div class="space-y-4">
					<div class="bg-gray-50 p-4 rounded-lg space-y-2">
						<div class="flex justify-between">
							<span class="text-gray-600">Amount:</span>
							<span class="font-semibold">{amount} UGX</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-600">Agent:</span>
							<span class="font-semibold">{selectedAgent?.businessName}</span>
						</div>
					</div>
					
					<button
						onclick={handleConfirm}
						disabled={isCreating}
						class="w-full bg-purple-600 text-white py-3 rounded-lg hover:bg-purple-700 font-semibold disabled:opacity-50"
					>
						{isCreating ? 'Creating...' : 'Confirm Deposit'}
					</button>
				</div>
			{:else}
				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto">
						<Check class="w-8 h-8 text-green-600" />
					</div>
					<h3 class="text-lg font-bold">Deposit Request Created!</h3>
					<div class="bg-purple-50 p-6 rounded-lg">
						<p class="text-sm text-gray-600 mb-2">Your deposit code:</p>
						<p class="text-3xl font-bold text-purple-600 tracking-wider">{depositCode}</p>
					</div>
					<p class="text-sm text-gray-600">Show this code to the agent to complete your deposit</p>
					<button
						onclick={() => goto('/users/dashboard')}
						class="w-full bg-purple-600 text-white py-3 rounded-lg hover:bg-purple-700 font-semibold"
					>
						Back to Dashboard
					</button>
				</div>
			{/if}
		{/if}
	</div>
</div>
