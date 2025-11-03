<script lang="ts">
	import PrimaryBalanceCard from '$lib/components/dashboard/PrimaryBalanceCard.svelte';
	import CkBTCBalanceCard from '$lib/components/dashboard/CkBTCBalanceCard.svelte';
	import CkUSDBalanceCard from '$lib/components/dashboard/CkUSDBalanceCard.svelte';

	interface Props {
		exchangeRate: number;
		userBalance: number;
		preferredCurrency: string;
		ckBTCBalance: number;
		ckUSDBalance: number;
		onCurrencyChange: (currency: string) => void;
		onContinue: (localAmount: string, btcAmount: string, fee: number, withdrawType: 'cash' | 'bitcoin' | 'ckusdc') => void;
	}

	let { exchangeRate, userBalance, preferredCurrency, ckBTCBalance, ckUSDBalance, onCurrencyChange, onContinue }: Props = $props();

	let amount = $state('');
	let withdrawType = $state<'cash' | 'bitcoin' | 'ckusdc'>('cash');

	function handleContinue() {
		const fee = parseFloat(amount) * 0.01; // 1% fee
		onContinue(amount, '0', fee, withdrawType);
	}
</script>

<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-4 sm:p-6 lg:p-8">
	<h2 class="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">Enter Withdrawal Amount</h2>
	
	<!-- Balance Cards - All in one row -->
	<div class="mb-4 sm:mb-6">
		<div class="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4">
			<PrimaryBalanceCard
				balance={userBalance}
				currency={preferredCurrency}
				{onCurrencyChange}
			/>
			<CkBTCBalanceCard
				principalId="user_123"
				{preferredCurrency}
				showActions={false}
			/>
			<CkUSDBalanceCard
				principalId="user_123"
				{preferredCurrency}
				showActions={false}
			/>
		</div>
	</div>

	<div class="mb-4">
		<label for="withdraw-amount" class="block text-sm font-medium text-gray-700 mb-2">Amount</label>
		<input
			id="withdraw-amount"
			type="number"
			bind:value={amount}
			placeholder="Enter amount"
			class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-500 focus:border-transparent"
		/>
	</div>

	<button
		onclick={handleContinue}
		disabled={!amount || parseFloat(amount) <= 0}
		class="w-full bg-gray-900 text-white py-3 rounded-lg font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed"
	>
		Continue
	</button>
</div>
