<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft, ArrowDownUp, RefreshCw } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	import { getExchangeRates } from '$lib/services/exchangeRateService';
	import { fetchCkBTCBalance } from '$lib/services/data/ckbtcData';
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { onMount } from 'svelte';
	
	let fromAmount = $state('');
	let toAmount = $state('');
	let exchangeRate = $state(0);
	let isExchanging = $state(false);
	let isLoadingRate = $state(true);
	let lastUpdated = $state<Date | null>(null);
	let userBalance = $state(0);
	
	onMount(async () => {
		await loadExchangeRate();
		await loadBalance();
	});
	
	async function loadBalance() {
		try {
			userBalance = await fetchCkBTCBalance($principalId, $demoMode);
		} catch (error) {
			console.error('Failed to load balance:', error);
		}
	}
	
	async function loadExchangeRate() {
		isLoadingRate = true;
		try {
			const rates = await getExchangeRates();
			exchangeRate = rates.btcToUsdc;
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
		const btcAmount = parseFloat(fromAmount);
		toAmount = (btcAmount * exchangeRate).toFixed(2);
	}
	
	async function handleExchange() {
		const amountNum = parseFloat(fromAmount);
		
		if (!fromAmount || amountNum <= 0) {
			toast.show('error', 'Please enter a valid amount');
			return;
		}
		
		// Check if user has enough balance
		if (amountNum > userBalance) {
			toast.show('error', `Insufficient balance. You have ${userBalance.toFixed(8)} ckBTC`);
			return;
		}
		
		isExchanging = true;
		try {
			// TODO: Call AfriTokeni Exchange Canister
			// The canister will:
			// 1. Transfer ckBTC from user's principal
			// 2. Deduct 0.5% spread → send to DAO treasury
			// 3. Swap remaining 99.5% for ckUSDC
			// 4. Transfer ckUSDC to user's principal
			// 
			// Example call:
			// await exchangeCanister.swapBtcToUsdc({
			//   amount: amountNum,
			//   minOutput: parseFloat(toAmount) * 0.99 // 1% slippage tolerance
			// });
			
			if (!$demoMode) {
				throw new Error('Exchange canister not yet deployed. Please try demo mode.');
			}
			
			await new Promise(resolve => setTimeout(resolve, 1500));
			toast.show('success', 'Exchange completed successfully!');
			goto('/users/dashboard');
		} catch (error: any) {
			toast.show('error', error.message || 'Exchange failed');
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
		<h1 class="text-2xl font-bold">Exchange ckBTC</h1>
	</div>
	
	<div class="bg-white rounded-xl border border-gray-200 p-6 space-y-6">
		<div class="bg-gray-50 p-4 rounded-lg">
			<div class="flex justify-between items-start gap-2 mb-2">
				<span class="text-sm text-gray-600 shrink-0">Exchange Rate</span>
				<div class="flex items-center gap-2 min-w-0">
					{#if isLoadingRate}
						<span class="text-sm text-gray-500">Loading...</span>
					{:else}
						<div class="text-right min-w-0">
							<div class="font-bold text-sm sm:text-lg break-words">
								1 ckBTC ≈
							</div>
							<div class="font-bold text-sm sm:text-lg break-words">
								${exchangeRate.toLocaleString()} USDC
							</div>
						</div>
						<button
							onclick={loadExchangeRate}
							class="p-1 hover:bg-gray-200 rounded shrink-0"
							title="Refresh rate"
						>
							<RefreshCw class="w-4 h-4 text-gray-600" />
						</button>
					{/if}
				</div>
			</div>
			{#if lastUpdated}
				<div class="text-xs text-gray-500 text-right">
					Updated: {lastUpdated.toLocaleTimeString()}
				</div>
			{/if}
		</div>
		
		<div>
			<label for="from" class="block text-sm font-medium text-gray-700 mb-2">
				From (ckBTC)
			</label>
			<input
				id="from"
				type="number"
				step="0.00000001"
				bind:value={fromAmount}
				oninput={calculateExchange}
				placeholder="0.00000000"
				class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-600 focus:border-transparent"
			/>
			<p class="text-sm text-gray-600 mt-1">
				Available: <span class="font-semibold">{userBalance.toFixed(8)} ckBTC</span>
			</p>
		</div>
		
		<div class="flex justify-center">
			<div class="w-10 h-10 bg-orange-100 rounded-full flex items-center justify-center">
				<ArrowDownUp class="w-5 h-5 text-orange-600" />
			</div>
		</div>
		
		<div>
			<label for="to" class="block text-sm font-medium text-gray-700 mb-2">
				To (ckUSDC)
			</label>
			<input
				id="to"
				type="text"
				value={toAmount}
				readonly
				placeholder="0.00"
				class="w-full px-4 py-3 border border-gray-300 rounded-lg bg-gray-50"
			/>
		</div>
		
		<div class="bg-orange-50 border border-orange-200 rounded-lg p-4">
			<p class="text-sm text-orange-800">
				<strong>Note:</strong> To get fiat currency (UGX), use the Withdraw feature to get cash from an agent.
			</p>
		</div>
		
		<button
			onclick={handleExchange}
			disabled={isExchanging || !fromAmount || parseFloat(fromAmount) > userBalance || parseFloat(fromAmount) <= 0}
			class="w-full bg-orange-600 text-white py-3 rounded-lg hover:bg-orange-700 font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{#if isExchanging}
				Exchanging...
			{:else if parseFloat(fromAmount) > userBalance}
				Insufficient Balance
			{:else}
				Exchange to ckUSDC
			{/if}
		</button>
	</div>
</div>
