<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft, ArrowDownUp, RefreshCw } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	import { getExchangeRates } from '$lib/services/exchangeRateService';
	import { onMount } from 'svelte';
	
	let fromAmount = $state('');
	let toAmount = $state('');
	let exchangeRate = $state(0);
	let isExchanging = $state(false);
	let isLoadingRate = $state(true);
	let lastUpdated = $state<Date | null>(null);
	
	onMount(async () => {
		await loadExchangeRate();
	});
	
	async function loadExchangeRate() {
		isLoadingRate = true;
		try {
			const rates = await getExchangeRates();
			exchangeRate = rates.usdcToBtc;
			lastUpdated = rates.lastUpdated;
			calculateExchange();
		} catch (error) {
			toast.show('error', 'Failed to load exchange rate');
		} finally {
			isLoadingRate = false;
		}
	}
	
	function calculateExchange() {
		if (!fromAmount || !exchangeRate) {
			toAmount = '';
			return;
		}
		const usdcAmount = parseFloat(fromAmount);
		toAmount = (usdcAmount * exchangeRate).toFixed(8);
	}
	
	async function handleExchange() {
		if (!fromAmount || parseFloat(fromAmount) <= 0) {
			toast.show('error', 'Please enter a valid amount');
			return;
		}
		
		isExchanging = true;
		try {
			// TODO: Call AfriTokeni Exchange Canister
			// The canister will:
			// 1. Transfer ckUSDC from user's principal
			// 2. Deduct 0.5% spread → send to DAO treasury
			// 3. Swap remaining 99.5% for ckBTC
			// 4. Transfer ckBTC to user's principal
			// 
			// Example call:
			// await exchangeCanister.swapUsdcToBtc({
			//   amount: parseFloat(fromAmount),
			//   minOutput: parseFloat(toAmount) * 0.99 // 1% slippage tolerance
			// });
			
			await new Promise(resolve => setTimeout(resolve, 1500));
			toast.show('success', 'Exchange completed successfully!');
			goto('/users/dashboard');
		} catch (error) {
			toast.show('error', 'Exchange failed');
		} finally {
			isExchanging = false;
		}
	}
</script>

<div class="max-w-2xl mx-auto">
	<div class="flex items-center gap-4 mb-6">
		<button onclick={() => goto('/users/dashboard')} class="p-2 hover:bg-gray-100 rounded-lg">
			<ArrowLeft class="w-5 h-5" />
		</button>
		<h1 class="text-2xl font-bold">Exchange ckUSDC</h1>
	</div>
	
	<div class="bg-white rounded-xl border border-gray-200 p-6 space-y-6">
		<div class="bg-gray-50 p-4 rounded-lg">
			<div class="flex justify-between items-center mb-2">
				<span class="text-sm text-gray-600">Exchange Rate</span>
				<div class="flex items-center gap-2">
					{#if isLoadingRate}
						<span class="text-sm text-gray-500">Loading...</span>
					{:else}
						<span class="text-lg font-bold">$1 USDC ≈ {exchangeRate.toFixed(8)} BTC</span>
						<button
							onclick={loadExchangeRate}
							class="p-1 hover:bg-gray-200 rounded"
							title="Refresh rate"
						>
							<RefreshCw class="w-4 h-4 text-gray-600" />
						</button>
					{/if}
				</div>
			</div>
			{#if lastUpdated}
				<div class="text-xs text-gray-500">
					Updated: {lastUpdated.toLocaleTimeString()}
				</div>
			{/if}
		</div>
		
		<div>
			<label for="from" class="block text-sm font-medium text-gray-700 mb-2">
				From (ckUSDC)
			</label>
			<input
				id="from"
				type="number"
				step="0.01"
				bind:value={fromAmount}
				oninput={calculateExchange}
				placeholder="0.00"
				class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-600 focus:border-transparent"
			/>
		</div>
		
		<div class="flex justify-center">
			<div class="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
				<ArrowDownUp class="w-5 h-5 text-blue-600" />
			</div>
		</div>
		
		<div>
			<label for="to" class="block text-sm font-medium text-gray-700 mb-2">
				To (ckBTC)
			</label>
			<input
				id="to"
				type="text"
				value={toAmount}
				readonly
				placeholder="0.00000000"
				class="w-full px-4 py-3 border border-gray-300 rounded-lg bg-gray-50"
			/>
		</div>
		
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<p class="text-sm text-blue-800">
				<strong>Note:</strong> To get fiat currency (UGX), use the Withdraw feature to get cash from an agent.
			</p>
		</div>
		
		<button
			onclick={handleExchange}
			disabled={isExchanging || !fromAmount}
			class="w-full bg-blue-600 text-white py-3 rounded-lg hover:bg-blue-700 font-semibold disabled:opacity-50"
		>
			{isExchanging ? 'Exchanging...' : 'Exchange to ckBTC'}
		</button>
	</div>
</div>
