<script lang="ts">
	import { onMount } from 'svelte';
	import { formatCurrencyAmount, type AfricanCurrency } from '$lib/types/currency';
	import { getUserData, getUserBalance } from '$lib/services/user/userService';
	import AmountStep from './AmountStep.svelte';
	import AgentStep from './AgentStep.svelte';
	import ConfirmationStep from './ConfirmationStep.svelte';

	type WithdrawStep = 'amount' | 'agent' | 'confirmation';
	type Agent = {
		id: string;
		businessName: string;
		location: { city: string; latitude: number; longitude: number };
	};

	// State
	let currentStep = $state<WithdrawStep>('amount');
	let userLocation = $state<[number, number] | null>(null);
	let locationError = $state<string | null>(null);
	let selectedAgent = $state<Agent | null>(null);
	let withdrawalCode = $state('');
	let finalLocalAmount = $state(0);
	let finalBtcAmount = $state('');
	let withdrawType = $state<'cash' | 'bitcoin' | 'ckusd'>('cash');
	let withdrawalFee = $state(0);
	let isCreatingTransaction = $state(false);
	let transactionError = $state<string | null>(null);
	let selectedCurrency = $state('');
	let showConfirmModal = $state(false);
	let pendingAgent = $state<Agent | null>(null);

	// User data
	let currentUser = $state<any>(null);
	let userBalance = $state(0);
	const defaultCurrency = $derived(currentUser?.preferredCurrency || 'UGX');
	const userCurrency = $derived(selectedCurrency || defaultCurrency);

	onMount(async () => {
		currentUser = await getUserData();
		userBalance = await getUserBalance();
	});

	// Mock Bitcoin exchange rate
	function getBtcExchangeRate(currency: string) {
		const rates: Record<string, number> = {
			'NGN': 67500000,
			'KES': 6450000,
			'GHS': 540000,
			'ZAR': 774000,
			'EGP': 1333000,
			'UGX': 67500000
		};
		return rates[currency] || 67500000;
	}

	const exchangeRate = $derived(getBtcExchangeRate(userCurrency));

	onMount(() => {
		if (navigator.geolocation) {
			navigator.geolocation.getCurrentPosition(
				(position) => {
					userLocation = [position.coords.latitude, position.coords.longitude];
				},
				(error) => {
					locationError = `Error getting location: ${error.message}`;
					userLocation = [0.3136, 32.5811]; // Kampala default
				}
			);
		} else {
			locationError = 'Geolocation is not supported by this browser.';
			userLocation = [0.3136, 32.5811];
		}
	});

	function generateWithdrawalCode() {
		const code = Math.random().toString(36).substring(2, 8).toUpperCase();
		withdrawalCode = code;
		return code;
	}

	function handleAgentSelect(agent: Agent) {
		pendingAgent = agent;
		showConfirmModal = true;
	}

	async function confirmWithdrawal() {
		if (!pendingAgent) return;

		isCreatingTransaction = true;
		transactionError = null;
		showConfirmModal = false;

		try {
			const code = generateWithdrawalCode();
			
			// Simulate API call
			await new Promise(resolve => setTimeout(resolve, 500));
			console.log('🎭 Demo withdrawal created:', {
				agent: pendingAgent.businessName,
				amount: finalLocalAmount,
				currency: userCurrency,
				code
			});

			selectedAgent = pendingAgent;
			withdrawalCode = code;
			currentStep = 'confirmation';
		} catch (error) {
			console.error('Error creating withdraw transaction:', error);
			transactionError = 'Failed to create withdrawal. Please try again.';
		} finally {
			isCreatingTransaction = false;
			pendingAgent = null;
		}
	}

	function handleMakeAnotherWithdrawal() {
		currentStep = 'amount';
		selectedAgent = null;
		withdrawalCode = '';
		finalLocalAmount = 0;
		finalBtcAmount = '';
		withdrawType = 'cash';
		transactionError = null;
		isCreatingTransaction = false;
	}
</script>

<div class="space-y-4 sm:space-y-6">
	<!-- Step Indicator -->
	<div class="mb-6 sm:mb-8 flex items-center justify-center px-2">
		<div class="flex items-center space-x-2 sm:space-x-4 overflow-x-auto pb-2">
			<div class="flex items-center space-x-1.5 sm:space-x-2 {currentStep === 'amount' ? 'text-gray-900' : 'text-green-600'}">
				<div class="w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold shrink-0 {currentStep === 'amount' ? 'bg-gray-900 text-white' : 'bg-green-600 text-white'}">
					1
				</div>
				<span class="text-xs sm:text-sm font-medium whitespace-nowrap">Enter Amount</span>
			</div>
			
			<div class="w-4 sm:w-8 h-0.5 shrink-0 {currentStep === 'agent' || currentStep === 'confirmation' ? 'bg-green-600' : 'bg-gray-200'}"></div>
			
			<div class="flex items-center space-x-1.5 sm:space-x-2 {currentStep === 'agent' ? 'text-gray-900' : currentStep === 'confirmation' ? 'text-green-600' : 'text-gray-400'}">
				<div class="w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold shrink-0 {currentStep === 'agent' ? 'bg-gray-900 text-white' : currentStep === 'confirmation' ? 'bg-green-600 text-white' : 'bg-gray-200'}">
					2
				</div>
				<span class="text-xs sm:text-sm font-medium whitespace-nowrap">Select Agent</span>
			</div>
			
			<div class="w-4 sm:w-8 h-0.5 shrink-0 {currentStep === 'confirmation' ? 'bg-green-600' : 'bg-gray-200'}"></div>
			
			<div class="flex items-center space-x-1.5 sm:space-x-2 {currentStep === 'confirmation' ? 'text-gray-900' : 'text-gray-400'}">
				<div class="w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold shrink-0 {currentStep === 'confirmation' ? 'bg-gray-900 text-white' : 'bg-gray-200'}">
					3
				</div>
				<span class="text-xs sm:text-sm font-medium whitespace-nowrap">Confirmation</span>
			</div>
		</div>
	</div>

	<!-- Render Current Step -->
	{#if currentStep === 'amount'}
		<AmountStep
			{exchangeRate}
			{userBalance}
			preferredCurrency={userCurrency}
			ckBTCBalance={50000}
			ckUSDCBalance={10000}
			onCurrencyChange={(currency) => selectedCurrency = currency}
			onContinue={(localAmount, btcAmount, fee, type) => {
				finalLocalAmount = parseFloat(localAmount) || 0;
				finalBtcAmount = btcAmount;
				withdrawType = type;
				withdrawalFee = fee;
				
				if (type === 'bitcoin' || type === 'ckusdc') {
					generateWithdrawalCode();
					currentStep = 'confirmation';
				} else {
					currentStep = 'agent';
				}
			}}
		/>
	{/if}

	{#if currentStep === 'agent'}
		<AgentStep
			{userLocation}
			{locationError}
			localAmount={finalLocalAmount}
			btcAmount={finalBtcAmount}
			{userCurrency}
			onBackToAmount={() => currentStep = 'amount'}
			onAgentSelect={handleAgentSelect}
			{isCreatingTransaction}
			{transactionError}
		/>
	{/if}

	{#if currentStep === 'confirmation' && (withdrawType === 'cash' ? selectedAgent : true)}
		<ConfirmationStep
			localAmount={finalLocalAmount}
			btcAmount={finalBtcAmount}
			{withdrawType}
			{userCurrency}
			fee={withdrawalFee}
			{userLocation}
			selectedAgent={selectedAgent || undefined}
			{withdrawalCode}
			onMakeAnotherWithdrawal={handleMakeAnotherWithdrawal}
		/>
	{/if}

	<!-- Confirmation Modal -->
	{#if showConfirmModal && pendingAgent}
		<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
			<div class="bg-white rounded-xl sm:rounded-2xl max-w-md w-full p-4 sm:p-6 space-y-3 sm:space-y-4">
				<h3 class="text-lg sm:text-xl font-bold text-gray-900">Confirm Withdrawal</h3>
				
				<div class="bg-amber-50 border border-amber-200 rounded-lg p-3 sm:p-4">
					<p class="text-xs sm:text-sm text-amber-800 font-medium mb-2">⚠️ Legal Binding Agreement</p>
					<p class="text-xs text-amber-700 leading-relaxed">
						By confirming this withdrawal, you are entering into a legally binding agreement between you and <strong>{pendingAgent.businessName}</strong>. 
						You agree to meet the agent at the specified location to collect your cash withdrawal of <strong>{formatCurrencyAmount(finalLocalAmount, userCurrency as AfricanCurrency)}</strong>.
					</p>
				</div>

				<div class="bg-gray-50 rounded-lg p-3 sm:p-4 space-y-2 text-xs sm:text-sm">
					<div class="flex justify-between gap-2">
						<span class="text-gray-600">Agent:</span>
						<span class="font-medium text-gray-900 text-right wrap-break-word">{pendingAgent.businessName}</span>
					</div>
					<div class="flex justify-between gap-2">
						<span class="text-gray-600">Location:</span>
						<span class="font-medium text-gray-900 text-right wrap-break-word">{pendingAgent.location.city}</span>
					</div>
					<div class="flex justify-between gap-2">
						<span class="text-gray-600">Amount:</span>
						<span class="font-medium text-gray-900">{formatCurrencyAmount(finalLocalAmount, userCurrency as AfricanCurrency)}</span>
					</div>
					<div class="flex justify-between gap-2">
						<span class="text-gray-600">Fee:</span>
						<span class="font-medium text-red-600">{formatCurrencyAmount(withdrawalFee, userCurrency as AfricanCurrency)}</span>
					</div>
				</div>

				<div class="flex flex-col sm:flex-row gap-2 sm:gap-3">
					<button
						onclick={() => {
							showConfirmModal = false;
							pendingAgent = null;
						}}
						class="flex-1 px-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg text-sm sm:text-base font-semibold text-gray-700 hover:bg-gray-50 transition-colors"
					>
						Cancel
					</button>
					<button
						onclick={confirmWithdrawal}
						disabled={isCreatingTransaction}
						class="flex-1 px-4 py-2.5 sm:py-3 bg-gray-900 text-white rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
					>
						{isCreatingTransaction ? 'Processing...' : 'I Agree & Confirm'}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>
