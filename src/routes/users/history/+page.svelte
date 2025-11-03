<script lang="ts">
	import { onMount } from 'svelte';
	import { ArrowUp, ArrowDown, Plus, Minus, Search, Send, Banknote, LayoutDashboard } from '@lucide/svelte';
	import { formatCurrency, formatDate, getTransactionDescription, type Transaction, type TransactionType } from '$lib/utils/transactions';
	import { getTransactions } from '$lib/services/user/userService';

	// State
	let transactions = $state<Transaction[]>([]);
	let searchQuery = $state('');
	let filterType = $state<TransactionType | 'all'>('all');
	let loading = $state(true);

	onMount(() => {
		loadTransactions();
		
		// Get search query from URL parameter
		if (typeof window !== 'undefined') {
			const urlParams = new URLSearchParams(window.location.search);
			const searchParam = urlParams.get('search');
			if (searchParam) {
				searchQuery = searchParam;
			}
		}
	});

	async function loadTransactions() {
		loading = true;
		transactions = await getTransactions();
		loading = false;
	}

	const filteredTransactions = $derived(() => {
		let filtered = transactions;

		// Apply search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(t =>
				(t.description || '').toLowerCase().includes(query) ||
				t.type.toLowerCase().includes(query) ||
				t.status.toLowerCase().includes(query)
			);
		}

		// Apply type filter
		if (filterType !== 'all') {
			filtered = filtered.filter(t => t.type === filterType);
		}

		return filtered;
	});

	function getTransactionIcon(type: TransactionType) {
		const icons = {
			send: { component: ArrowUp, color: 'text-red-500' },
			receive: { component: ArrowDown, color: 'text-green-500' },
			withdraw: { component: Minus, color: 'text-orange-500' },
			deposit: { component: Plus, color: 'text-blue-500' }
		};
		return icons[type] || icons.send;
	}
</script>

<div class="space-y-4 sm:space-y-6">
	{#if loading}
		<div class="flex justify-center items-center py-8 sm:py-12">
			<div class="text-center">
				<div class="animate-spin rounded-full h-6 w-6 sm:h-8 sm:w-8 border-b-2 border-gray-900 mx-auto mb-3 sm:mb-4"></div>
				<p class="text-sm sm:text-base text-gray-600">Loading transactions...</p>
			</div>
		</div>
	{:else if !transactions.length}
		<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-8 sm:p-12 text-center">
			<div class="w-12 h-12 sm:w-16 sm:h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
				<ArrowUp class="w-6 h-6 sm:w-8 sm:h-8 text-gray-400" />
			</div>
			<h3 class="text-base sm:text-lg font-semibold text-gray-900 mb-2">No transactions yet</h3>
			<p class="text-sm sm:text-base text-gray-600 max-w-md mx-auto">Your transaction history will appear here once you start sending or receiving money.</p>
		</div>
	{:else}
		<!-- Search Bar -->
		<div class="relative">
			<Search class="absolute left-2.5 sm:left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-400" />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search by description, type, or status..."
				class="w-full pl-8 sm:pl-10 pr-3 sm:pr-4 py-2.5 sm:py-3 bg-white border border-gray-200 rounded-lg text-xs sm:text-sm focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent"
			/>
		</div>

		<!-- Filter Buttons -->
		<div class="flex gap-1.5 sm:gap-2 overflow-x-auto scrollbar-hide pb-2">
			<button
				onclick={() => filterType = 'all'}
				class="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {filterType === 'all' ? 'bg-black text-white' : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
			>
				<LayoutDashboard class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
				All
			</button>
			<button
				onclick={() => filterType = 'send'}
				class="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {filterType === 'send' ? 'bg-black text-white' : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
			>
				<Send class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
				Send
			</button>
			<button
				onclick={() => filterType = 'receive'}
				class="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {filterType === 'receive' ? 'bg-black text-white' : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
			>
				<ArrowDown class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
				Receive
			</button>
			<button
				onclick={() => filterType = 'deposit'}
				class="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {filterType === 'deposit' ? 'bg-black text-white' : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
			>
				<Plus class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
				Deposit
			</button>
			<button
				onclick={() => filterType = 'withdraw'}
				class="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {filterType === 'withdraw' ? 'bg-black text-white' : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
			>
				<Banknote class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
				Withdraw
			</button>
		</div>

		<!-- Transactions List -->
		<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200 divide-y divide-gray-200">
			{#each filteredTransactions() as transaction}
				{@const icon = getTransactionIcon(transaction.type)}
				{@const IconComponent = icon.component}
				<div class="p-4 sm:p-6 hover:bg-gray-50 transition-colors">
					<div class="flex items-center gap-3 sm:gap-4">
						<div class="w-10 h-10 sm:w-12 sm:h-12 rounded-full bg-gray-100 flex items-center justify-center shrink-0">
							<IconComponent class="w-5 h-5 {icon.color}" />
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-sm sm:text-base font-semibold text-gray-900 truncate">{getTransactionDescription(transaction)}</p>
							<p class="text-xs sm:text-sm text-gray-600">{formatDate(transaction.createdAt)}</p>
						</div>
						<div class="text-right shrink-0">
							<p class="text-sm sm:text-base font-bold text-gray-900 {transaction.type === 'send' || transaction.type === 'withdraw' ? 'text-red-600' : 'text-green-600'}">
								{transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}{formatCurrency(transaction.amount, transaction.currency)}
							</p>
							<span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium {transaction.status === 'completed' ? 'bg-green-100 text-green-800' : transaction.status === 'pending' ? 'bg-yellow-100 text-yellow-800' : 'bg-red-100 text-red-800'}">
								{transaction.status}
							</span>
						</div>
					</div>
				</div>
			{/each}
		</div>

		{#if filteredTransactions().length === 0}
			<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-8 sm:p-12 text-center">
				<p class="text-sm sm:text-base text-gray-600">No transactions match your search.</p>
			</div>
		{/if}
	{/if}
</div>
