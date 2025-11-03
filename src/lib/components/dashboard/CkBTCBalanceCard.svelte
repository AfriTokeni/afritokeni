<!--
 * ckBTC Balance Card Component
 * 
 * Displays user's ckBTC balance with local currency equivalent
 * Lightning-like instant transfers with near-zero fees
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { Download, Send, RefreshCw, Bitcoin, Zap, TrendingUp } from '@lucide/svelte';

	interface Props {
		principalId: string;
		preferredCurrency?: string;
		showActions?: boolean;
		isAgent?: boolean;
		onDeposit?: () => void;
		onSend?: () => void;
		onExchange?: () => void;
	}

	let {
		principalId,
		preferredCurrency = 'UGX',
		showActions = true,
		isAgent = false,
		onDeposit,
		onSend,
		onExchange
	}: Props = $props();

	let balance = $state<any>(null);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);

	async function fetchBalance() {
		try {
			error = null;
			// Import from userService
			const { getCkBTCBalance } = await import('$lib/services/user/userService');
			const ckbtcBalance = await getCkBTCBalance();
			balance = {
				balanceSatoshis: Math.round(ckbtcBalance * 100000000),
				balanceBTC: ckbtcBalance.toFixed(8),
				localCurrencyEquivalent: 0,
				localCurrency: preferredCurrency,
				lastUpdated: new Date()
			};
		} catch (err: any) {
			console.error('Error fetching ckBTC balance:', err);
			error = err.message || 'Failed to load balance';
		} finally {
			isLoading = false;
			isRefreshing = false;
		}
	}

	onMount(() => {
		fetchBalance();
	});

	async function handleRefresh() {
		isRefreshing = true;
		await fetchBalance();
	}

	function formatLocalCurrency(amount: number | undefined) {
		if (!amount) return '0.00';
		return new Intl.NumberFormat('en-US', {
			minimumFractionDigits: 2,
			maximumFractionDigits: 2
		}).format(amount);
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
			<h3 class="text-base sm:text-lg font-semibold text-neutral-900">ckBTC Balance</h3>
			<div class="p-1.5 sm:p-2 bg-red-50 rounded-full">
				<Bitcoin class="w-5 h-5 sm:w-6 sm:h-6 text-red-500 shrink-0" />
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
	<div class="bg-linear-to-br from-orange-50 to-amber-50 rounded-xl shadow-sm border border-orange-200 p-4 sm:p-5 md:p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-3 sm:mb-4">
			<div>
				<h3 class="text-base sm:text-lg font-semibold text-neutral-900">ckBTC Balance</h3>
				<div class="flex items-center gap-1.5 sm:gap-2 mt-1">
					<Zap class="w-3 h-3 sm:w-3.5 sm:h-3.5 text-orange-600 shrink-0" />
					<p class="text-xs sm:text-sm text-neutral-600">Instant Transfers</p>
				</div>
			</div>
			<div class="flex items-center gap-1 sm:gap-2">
				<button
					onclick={handleRefresh}
					disabled={isRefreshing}
					class="p-1.5 sm:p-2 hover:bg-orange-100 rounded-lg transition-colors disabled:opacity-50"
					title="Refresh balance"
				>
					<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
				</button>
				<div class="p-1.5 sm:p-2 bg-orange-100 rounded-full">
					<Bitcoin class="w-5 h-5 sm:w-6 sm:h-6 text-orange-600 shrink-0" />
				</div>
			</div>
		</div>

		<!-- Balance Display -->
		<div class="mb-3 sm:mb-4">
			<div class="flex items-baseline gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
				<span class="text-xl sm:text-2xl md:text-3xl font-bold text-neutral-900 font-mono break-all">
					₿{balance?.balanceBTC || '0.00000000'}
				</span>
			</div>
			
			{#if balance?.localCurrencyEquivalent !== undefined}
				<div class="flex items-center gap-1.5 sm:gap-2 text-xs sm:text-sm text-neutral-600">
					<TrendingUp class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" />
					<span class="wrap-break-word">
						≈ {formatLocalCurrency(balance.localCurrencyEquivalent)} {balance.localCurrency}
					</span>
				</div>
			{/if}
		</div>

		<!-- Info Badge - Hidden on mobile -->
		<div class="mb-3 sm:mb-4 p-2.5 sm:p-3 bg-white/60 rounded-lg border border-orange-200 hidden md:block">
			<div class="flex items-start gap-2">
				<Zap class="w-4 h-4 text-orange-600 mt-0.5 shrink-0" />
				<p class="text-xs sm:text-sm text-neutral-700 wrap-break-word">
					<span class="font-semibold">Lightning-Fast:</span> Send Bitcoin instantly with ~$0.01 fees.
				</p>
			</div>
		</div>

		<!-- Quick Actions -->
		{#if showActions}
			<div class="grid grid-cols-3 gap-1.5 sm:gap-2">
				<button
					onclick={onDeposit}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-orange-50 rounded-lg border border-orange-200 transition-colors"
				>
					<Download class="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Deposit</span>
				</button>
				
				<button
					onclick={onSend}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-orange-50 rounded-lg border border-orange-200 transition-colors"
				>
					<Send class="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Send</span>
				</button>
				
				<button
					onclick={onExchange}
					class="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-orange-50 rounded-lg border border-orange-200 transition-colors"
				>
					<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 shrink-0" />
					<span class="text-[10px] sm:text-xs font-medium text-neutral-900">Exchange</span>
				</button>
			</div>
		{/if}

		<!-- Last Updated -->
		<div class="text-[10px] sm:text-xs text-gray-400 mt-2 sm:mt-3 wrap-break-word">
			Last updated: {balance?.lastUpdated.toLocaleString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })}
		</div>
	</div>
{/if}
