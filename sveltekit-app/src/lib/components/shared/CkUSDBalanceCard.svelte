<!--
 * ckUSD Balance Card Component (FULLY ENCAPSULATED)
 * 
 * Self-contained component that:
 * - Subscribes to demoMode and auth stores internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for user actions
 * 
 * Usage: <CkUSDBalanceCard onDeposit={...} onSend={...} />
-->
<script lang="ts">
	import { DollarSign, TrendingUp, Send, Download, RefreshCw } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { fetchCkUSDBalance, formatUSD } from '$lib/services/data/ckusdData';

	interface Props {
		showActions?: boolean;
		preferredCurrency?: string;
		onDeposit?: () => void;
		onSend?: () => void;
		onExchange?: () => void;
	}

	let {
		showActions = true,
		preferredCurrency = 'UGX',
		onDeposit,
		onSend,
		onExchange
	}: Props = $props();

	// Internal state
	let balanceUSD = $state(0);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);
	let lastUpdated = $state<Date>(new Date());

	// Reactive: auto-refetch when demoMode or principalId changes
	$effect(() => {
		loadBalance($demoMode, $principalId);
	});

	async function loadBalance(isDemoMode: boolean, principal: string | null) {
		try {
			error = null;
			const balance = await fetchCkUSDBalance(principal, isDemoMode);
			balanceUSD = balance;
			lastUpdated = new Date();
		} catch (err: any) {
			console.error('Error fetching ckUSD balance:', err);
			error = err.message || 'Failed to load balance';
		} finally {
			isLoading = false;
			isRefreshing = false;
		}
	}

	async function handleRefresh() {
		isRefreshing = true;
		await loadBalance($demoMode, $principalId);
	}
</script>

{#if isLoading}
	<div class="bg-white rounded-xl shadow-sm border border-neutral-200 p-4 sm:p-5 md:p-6">
		<div class="animate-pulse">
			<div class="flex items-center justify-between mb-3 sm:mb-4">
				<div class="h-5 sm:h-6 bg-neutral-200 rounded w-24 sm:w-32"></div>
				<div class="h-8 w-8 sm:h-10 sm:w-10 bg-neutral-200 rounded-full"></div>
			</div>
			<div class="h-8 sm:h-10 bg-neutral-200 rounded w-40 sm:w-48 mb-2"></div>
			<div class="h-3 sm:h-4 bg-neutral-200 rounded w-24 sm:w-32"></div>
		</div>
	</div>
{:else if error}
	<div class="bg-white rounded-xl shadow-sm border border-neutral-200 p-4 sm:p-5 md:p-6">
		<div class="flex items-center justify-between mb-3 sm:mb-4">
			<h3 class="text-base sm:text-lg font-semibold text-neutral-900">ckUSD Balance</h3>
			<div class="p-1.5 sm:p-2 bg-red-50 rounded-full">
				<DollarSign class="w-5 h-5 sm:w-6 sm:h-6 text-red-500 shrink-0" />
			</div>
		</div>
		<p class="text-xs sm:text-sm text-red-600 wrap-break-word">{error}</p>
		<button
			onclick={handleRefresh}
			class="mt-3 sm:mt-4 text-xs sm:text-sm text-neutral-600 hover:text-neutral-900 flex items-center gap-2"
		>
			<RefreshCw class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" />
			Try Again
		</button>
	</div>
{:else}
	<div class="bg-linear-to-br from-green-50 to-emerald-50 rounded-xl shadow-sm border border-green-200 p-4 sm:p-5 md:p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-3 sm:mb-4">
			<div>
				<h3 class="text-base sm:text-lg font-semibold text-neutral-900">ckUSD Balance</h3>
				<p class="text-xs sm:text-sm text-neutral-600 mt-1">Stable Value Storage</p>
			</div>
			<div class="flex items-center gap-1 sm:gap-2">
				<button
					onclick={handleRefresh}
					disabled={isRefreshing}
					class="p-1.5 sm:p-2 hover:bg-green-100 rounded-lg transition-colors disabled:opacity-50"
					title="Refresh balance"
				>
					<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-green-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
				</button>
				<div class="p-1.5 sm:p-2 bg-green-100 rounded-full">
					<DollarSign class="w-5 h-5 sm:w-6 sm:h-6 text-green-600 shrink-0" />
				</div>
			</div>
		</div>

		<!-- Balance Display -->
		<div class="mb-3 sm:mb-4">
			<div class="flex items-baseline gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
				<span class="text-xl sm:text-2xl md:text-3xl font-bold text-neutral-900 font-mono wrap-break-word">
					{formatUSD(balanceUSD)}
				</span>
				<span class="text-xs sm:text-sm text-neutral-600 font-semibold">ckUSD</span>
			</div>
		</div>

		<!-- Info Badge -->
		<div class="mb-3 sm:mb-4 p-2.5 sm:p-3 bg-white/60 rounded-lg border border-green-200">
			<p class="text-xs sm:text-sm text-neutral-700">
				<span class="font-semibold">Stable Value:</span> ckUSD is pegged 1:1 with USD
			</p>
		</div>

		<!-- Quick Actions -->
		{#if showActions}
			<div class="grid grid-cols-3 gap-1.5 sm:gap-2">
				<button
					onclick={onDeposit}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-green-50 rounded-lg border border-green-200 transition-colors"
				>
					<Download class="w-4 h-4 sm:w-5 sm:h-5 text-green-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Deposit</span>
				</button>
				
				<button
					onclick={onSend}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-green-50 rounded-lg border border-green-200 transition-colors"
				>
					<Send class="w-4 h-4 sm:w-5 sm:h-5 text-green-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Send</span>
				</button>
				
				<button
					onclick={onExchange}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-green-50 rounded-lg border border-green-200 transition-colors"
				>
					<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-green-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Exchange</span>
				</button>
			</div>
		{/if}

		<!-- Last Updated -->
		<div class="text-[10px] sm:text-xs text-neutral-400 mt-2 sm:mt-3 wrap-break-word">
			Last updated: {lastUpdated.toLocaleString('en-US', { 
				month: 'short', 
				day: 'numeric', 
				year: 'numeric', 
				hour: '2-digit', 
				minute: '2-digit' 
			})}
		</div>
	</div>
{/if}
