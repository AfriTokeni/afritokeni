<!--
 * Fiat Balance Card Component (FULLY ENCAPSULATED)
 * 
 * Self-contained component that:
 * - Subscribes to demoMode and auth stores internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for user actions
 * 
 * Usage: <FiatBalanceCard onDeposit={...} onWithdraw={...} />
-->
<script lang="ts">
	import { Wallet, Send, Download, Upload, RefreshCw } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { fetchFiatBalance, formatCurrency, getCurrencySymbol } from '$lib/services/data/fiatData';

	interface Props {
		showActions?: boolean;
		onDeposit?: () => void;
		onWithdraw?: () => void;
		onSend?: () => void;
	}

	let {
		showActions = true,
		onDeposit,
		onWithdraw,
		onSend
	}: Props = $props();

	// Internal state
	let balance = $state(0);
	let currency = $state('UGX');
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
			const result = await fetchFiatBalance(principal, isDemoMode);
			balance = result.amount;
			currency = result.currency;
			lastUpdated = new Date();
		} catch (err: any) {
			console.error('Error fetching fiat balance:', err);
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
			<h3 class="text-base sm:text-lg font-semibold text-neutral-900">Local Currency</h3>
			<div class="p-1.5 sm:p-2 bg-red-50 rounded-full">
				<Wallet class="w-5 h-5 sm:w-6 sm:h-6 text-red-500 shrink-0" />
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
	<div class="bg-linear-to-br from-blue-50 to-indigo-50 rounded-xl shadow-sm border border-blue-200 p-4 sm:p-5 md:p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-3 sm:mb-4">
			<div>
				<h3 class="text-base sm:text-lg font-semibold text-neutral-900">Local Currency</h3>
				<p class="text-xs sm:text-sm text-neutral-600 mt-1">Primary Balance</p>
			</div>
			<div class="flex items-center gap-1 sm:gap-2">
				<button
					onclick={handleRefresh}
					disabled={isRefreshing}
					class="p-1.5 sm:p-2 hover:bg-blue-100 rounded-lg transition-colors disabled:opacity-50"
					title="Refresh balance"
				>
					<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
				</button>
				<div class="p-1.5 sm:p-2 bg-blue-100 rounded-full">
					<Wallet class="w-5 h-5 sm:w-6 sm:h-6 text-blue-600 shrink-0" />
				</div>
			</div>
		</div>

		<!-- Balance Display -->
		<div class="mb-3 sm:mb-4">
			<div class="flex items-baseline gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
				<span class="text-xl sm:text-2xl md:text-3xl font-bold text-neutral-900 font-mono wrap-break-word">
					{formatCurrency(balance, currency)}
				</span>
				<span class="text-xs sm:text-sm text-neutral-600 font-semibold">{getCurrencySymbol(currency)}</span>
			</div>
		</div>

		<!-- Info Badge -->
		<div class="mb-3 sm:mb-4 p-2.5 sm:p-3 bg-white/60 rounded-lg border border-blue-200">
			<p class="text-xs sm:text-sm text-neutral-700">
				<span class="font-semibold">Available:</span> Use for transactions and agent services
			</p>
		</div>

		<!-- Quick Actions -->
		{#if showActions}
			<div class="grid grid-cols-3 gap-1.5 sm:gap-2">
				<button
					onclick={onDeposit}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-blue-50 rounded-lg border border-blue-200 transition-colors"
				>
					<Download class="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Deposit</span>
				</button>
				
				<button
					onclick={onWithdraw}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-blue-50 rounded-lg border border-blue-200 transition-colors"
				>
					<Upload class="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Withdraw</span>
				</button>
				
				<button
					onclick={onSend}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-blue-50 rounded-lg border border-blue-200 transition-colors"
				>
					<Send class="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Send</span>
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
