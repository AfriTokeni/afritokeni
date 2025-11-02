<!--
 * Transaction History Component (FULLY ENCAPSULATED)
 * 
 * Self-contained component that:
 * - Subscribes to demoMode and auth stores internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for navigation
 * 
 * Usage: <TransactionHistory maxTransactions={5} onViewAll={() => goto('/users/history')} />
-->
<script lang="ts">
	import { ArrowUp, ArrowDown, Minus, Plus, RefreshCw } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { fetchTransactions, getTransactionTypeInfo, isOutgoingTransaction } from '$lib/services/data/transactionsData';
	import { formatCurrencyAmount } from '$lib/types/currency';

	interface Props {
		maxTransactions?: number;
		showViewAll?: boolean;
		onViewAll?: () => void;
		currency?: string;
	}

	let {
		maxTransactions = 5,
		showViewAll = true,
		onViewAll,
		currency = 'UGX'
	}: Props = $props();

	// Internal state
	let transactions = $state<any[]>([]);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);

	// Reactive: auto-refetch when demoMode or principalId changes
	$effect(() => {
		loadTransactions($demoMode, $principalId);
	});

	async function loadTransactions(isDemoMode: boolean, principal: string | null) {
		try {
			error = null;
			const data = await fetchTransactions(principal, isDemoMode, maxTransactions);
			transactions = data;
		} catch (err: any) {
			console.error('Error fetching transactions:', err);
			error = err.message || 'Failed to load transactions';
		} finally {
			isLoading = false;
			isRefreshing = false;
		}
	}

	async function handleRefresh() {
		isRefreshing = true;
		await loadTransactions($demoMode, $principalId);
	}

	function formatCurrency(amount: number): string {
		return formatCurrencyAmount(amount, currency as any);
	}

	function formatDate(date: Date | string): string {
		const dateObj = date instanceof Date ? date : new Date(date);
		return dateObj.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getIcon(type: string) {
		switch (type) {
			case 'send': return ArrowUp;
			case 'receive': return ArrowDown;
			case 'withdraw': return Minus;
			case 'deposit': return Plus;
			default: return ArrowUp;
		}
	}
</script>

<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200">
	<!-- Header -->
	<div class="p-4 sm:p-6 border-b border-gray-100">
		<div class="flex justify-between items-center">
			<div class="flex items-center gap-2">
				<h3 class="text-lg sm:text-xl font-bold text-gray-900">Recent Transactions</h3>
				{#if !isLoading && transactions.length > 0}
					<span class="text-xs sm:text-sm text-gray-500">({transactions.length})</span>
				{/if}
			</div>
			<div class="flex items-center gap-2">
				<button
					onclick={handleRefresh}
					disabled={isRefreshing}
					class="p-2 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50"
					title="Refresh transactions"
				>
					<RefreshCw class="w-4 h-4 sm:w-5 sm:h-5 text-gray-600 shrink-0 {isRefreshing ? 'animate-spin' : ''}" />
				</button>
				{#if showViewAll && onViewAll}
					<button 
						onclick={onViewAll}
						class="text-gray-600 text-xs sm:text-sm font-medium hover:text-gray-900 transition-colors px-2 sm:px-4 py-1.5 sm:py-2 rounded-lg hover:bg-gray-50"
					>
						View All
					</button>
				{/if}
			</div>
		</div>
	</div>

	<!-- Transactions List -->
	<div class="divide-y divide-gray-100">
		{#if isLoading}
			<div class="p-6 sm:p-8 text-center">
				<div class="inline-block animate-spin rounded-full h-6 w-6 sm:h-8 sm:w-8 border-b-2 border-gray-900"></div>
				<p class="mt-3 sm:mt-4 text-sm sm:text-base text-gray-600">Loading transactions...</p>
			</div>
		{:else if error}
			<div class="p-6 sm:p-8 text-center">
				<p class="text-sm text-red-600 mb-3">{error}</p>
				<button
					onclick={handleRefresh}
					class="text-sm text-gray-600 hover:text-gray-900 flex items-center gap-2 mx-auto"
				>
					<RefreshCw class="w-4 h-4 shrink-0" />
					Try Again
				</button>
			</div>
		{:else if transactions.length === 0}
			<div class="p-6 sm:p-8 lg:p-12 text-center">
				<div class="w-12 h-12 sm:w-14 sm:h-14 lg:w-16 lg:h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
					<ArrowUp class="w-5 h-5 sm:w-6 sm:h-6 lg:w-8 lg:h-8 text-neutral-400" />
				</div>
				<h4 class="text-sm sm:text-base lg:text-lg font-semibold text-neutral-900 mb-2">No transactions yet</h4>
				<p class="text-neutral-500 text-xs sm:text-sm lg:text-base">Your transaction history will appear here</p>
			</div>
		{:else}
			{#each transactions as transaction}
				{@const typeInfo = getTransactionTypeInfo(transaction.type)}
				{@const Icon = getIcon(transaction.type)}
				{@const isOutgoing = isOutgoingTransaction(transaction.type)}
				
				<div class="p-3 sm:p-4 lg:p-6 hover:bg-gray-50 transition-colors">
					<!-- Mobile Layout -->
					<div class="sm:hidden">
						<div class="flex items-start space-x-2 sm:space-x-3">
							<div class="w-8 h-8 sm:w-10 sm:h-10 {typeInfo.bgColor} rounded-lg sm:rounded-xl flex items-center justify-center shrink-0">
								<Icon class="w-4 h-4 {typeInfo.color}" />
							</div>
							<div class="flex-1 min-w-0">
								<div class="flex justify-between items-start mb-1">
									<p class="font-medium text-neutral-900 text-xs sm:text-sm truncate pr-2">
										{transaction.description}
									</p>
									<p class="font-semibold text-sm sm:text-base font-mono shrink-0 {typeInfo.textColor}">
										{isOutgoing ? '-' : '+'}
										{formatCurrency(transaction.amount)}
									</p>
								</div>
								<div class="flex justify-between items-center gap-2">
									<p class="text-xs text-neutral-500 truncate">
										{formatDate(transaction.createdAt)}
									</p>
									<p class="text-xs text-neutral-500 capitalize shrink-0">
										{transaction.status}
									</p>
								</div>
							</div>
						</div>
					</div>
					
					<!-- Desktop Layout -->
					<div class="hidden sm:flex items-center justify-between">
						<div class="flex items-center space-x-3 lg:space-x-4 flex-1 min-w-0">
							<div class="w-10 h-10 lg:w-12 lg:h-12 {typeInfo.bgColor} rounded-xl flex items-center justify-center shrink-0">
								<Icon class="w-4 h-4 {typeInfo.color}" />
							</div>
							<div class="min-w-0 flex-1">
								<p class="font-medium text-neutral-900 text-sm lg:text-base truncate">
									{transaction.description}
								</p>
								<p class="text-xs sm:text-sm text-neutral-500">
									{formatDate(transaction.createdAt)}
								</p>
							</div>
						</div>
						<div class="text-right shrink-0 ml-2 lg:ml-4">
							<p class="font-semibold text-sm lg:text-base font-mono {typeInfo.textColor}">
								{isOutgoing ? '-' : '+'}
								{formatCurrency(transaction.amount)}
							</p>
							<p class="text-xs sm:text-sm text-neutral-500 capitalize">
								{transaction.status}
							</p>
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
