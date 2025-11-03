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
	import { ArrowUp, ArrowDown, Minus, Plus, RefreshCw, Search } from 'lucide-svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { fetchTransactions, getTransactionTypeInfo, isOutgoingTransaction } from '$lib/services/data/transactionsData';
	import { formatCurrencyAmount } from '$lib/types/currency';

	interface Props {
		maxTransactions?: number;
		showViewAll?: boolean;
		onViewAll?: () => void;
		currency?: string;
		showFilters?: boolean;
	}

	let {
		maxTransactions = 5,
		showViewAll = true,
		onViewAll,
		currency = 'UGX',
		showFilters = false
	}: Props = $props();

	// Internal state
	let transactions = $state<any[]>([]);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);
	let searchQuery = $state('');
	let typeFilter = $state<string>('all');
	let displayCount = $state(10); // Start with 10, load more on scroll
	let loadMoreElement: HTMLDivElement;

	// Reactive: auto-refetch when demoMode or principalId changes
	$effect(() => {
		loadTransactions($demoMode, $principalId);
	});

	// Infinite scroll observer
	$effect(() => {
		if (!loadMoreElement || !showFilters) return;

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting && displayCount < filteredTransactions.length) {
					displayCount += 10; // Load 10 more
				}
			},
			{ threshold: 0.1 }
		);

		observer.observe(loadMoreElement);

		return () => observer.disconnect();
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

	// Filtered transactions
	const filteredTransactions = $derived(
		transactions.filter(tx => {
			// Search filter
			const matchesSearch = searchQuery === '' || 
				tx.description?.toLowerCase().includes(searchQuery.toLowerCase()) ||
				tx.type?.toLowerCase().includes(searchQuery.toLowerCase()) ||
				tx.amount?.toString().includes(searchQuery);

			// Type filter
			const matchesType = typeFilter === 'all' || tx.type === typeFilter;

			return matchesSearch && matchesType;
		})
	);
</script>

<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200">
	<!-- Header -->
	<div class="p-4 sm:p-6 border-b border-gray-100">
		<div class="flex justify-between items-center">
			<div class="flex items-center gap-2">
				<h3 class="text-lg sm:text-xl font-bold text-gray-900">
					{showFilters ? 'Transaction History' : 'Recent Transactions'}
				</h3>
				{#if !isLoading && filteredTransactions.length > 0}
					<span class="text-xs sm:text-sm text-gray-500">({filteredTransactions.length})</span>
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

	<!-- Search and Filters -->
	{#if showFilters}
		<div class="p-4 sm:p-6 border-b border-gray-100 space-y-4">
			<!-- Search -->
			<div class="relative">
				<Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4 sm:w-5 sm:h-5 shrink-0" />
				<input
					type="text"
					placeholder="Search transactions..."
					bind:value={searchQuery}
					class="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2 sm:py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent text-sm sm:text-base"
				/>
			</div>

			<!-- Type Filters -->
			<div class="flex gap-2 overflow-x-auto scrollbar-hide pb-2">
				<button
					onclick={() => (typeFilter = 'all')}
					class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {typeFilter ===
					'all'
						? 'bg-black text-white'
						: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
				>
					All
				</button>
				<button
					onclick={() => (typeFilter = 'deposit')}
					class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {typeFilter ===
					'deposit'
						? 'bg-black text-white'
						: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
				>
					Deposits
				</button>
				<button
					onclick={() => (typeFilter = 'withdraw')}
					class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {typeFilter ===
					'withdraw'
						? 'bg-black text-white'
						: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
				>
					Withdrawals
				</button>
				<button
					onclick={() => (typeFilter = 'send')}
					class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {typeFilter ===
					'send'
						? 'bg-black text-white'
						: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
				>
					Sent
				</button>
				<button
					onclick={() => (typeFilter = 'receive')}
					class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {typeFilter ===
					'receive'
						? 'bg-black text-white'
						: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
				>
					Received
				</button>
			</div>
		</div>
	{/if}

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
		{:else if filteredTransactions.length === 0}
			<div class="p-6 sm:p-8 lg:p-12 text-center">
				<div class="w-12 h-12 sm:w-14 sm:h-14 lg:w-16 lg:h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
					<ArrowUp class="w-5 h-5 sm:w-6 sm:h-6 lg:w-8 lg:h-8 text-neutral-400" />
				</div>
				<h4 class="text-sm sm:text-base lg:text-lg font-semibold text-neutral-900 mb-2">No transactions yet</h4>
				<p class="text-neutral-500 text-xs sm:text-sm lg:text-base">Your transaction history will appear here</p>
			</div>
		{:else}
			{#each filteredTransactions.slice(0, showFilters ? displayCount : filteredTransactions.length) as transaction}
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

			<!-- Infinite scroll trigger (only when filters enabled) -->
			{#if showFilters && displayCount < filteredTransactions.length}
				<div bind:this={loadMoreElement} class="p-4 text-center text-sm text-gray-500">
					Loading more...
				</div>
			{/if}
		{/if}
	</div>
</div>
