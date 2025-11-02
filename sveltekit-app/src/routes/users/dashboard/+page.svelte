<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { Send, Bitcoin, ArrowUp, ArrowDown, Minus, Plus, Info } from '@lucide/svelte';
	import { AFRICAN_CURRENCIES, formatCurrencyAmount } from '$lib/types/currency';
	import CurrencySelector from '$lib/components/dashboard/CurrencySelector.svelte';
	import CkBTCBalanceCard from '$lib/components/shared/CkBTCBalanceCard.svelte';
	import CkUSDBalanceCard from '$lib/components/shared/CkUSDBalanceCard.svelte';
	import FiatBalanceCard from '$lib/components/shared/FiatBalanceCard.svelte';
	import { getUserData, getUserBalance, getTransactions, getCkBTCBalance, getCkUSDBalance } from '$lib/services/user/userService';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	// State
	let showDemoModal = $state(false);
	let showOnboarding = $state(false);
	let showBanner = $state(false);
	let missingFields = $state<string[]>([]);
	let bannerDismissed = $state(false);
	let currentUser = $state<any>(null);
	let balance = $state(0);
	let transactions = $state<any[]>([]);

	const userCurrency = $derived(currentUser?.preferredCurrency || 'UGX');
	const currencyInfo = $derived(AFRICAN_CURRENCIES[userCurrency as keyof typeof AFRICAN_CURRENCIES]);

	onMount(async () => {
		// Load user data from service
		currentUser = await getUserData();
		balance = await getUserBalance();
		transactions = await getTransactions();
		
		// Show demo modal on first login
		if (!browser || !currentUser?.id) return;
		
		const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
		const hasSeenModal = localStorage.getItem(globalModalKey);
		
		if (!hasSeenModal) {
			showDemoModal = true;
			localStorage.setItem(globalModalKey, 'true');
		}

		// Check for missing profile fields
		checkMissingFields();
	});

	function checkMissingFields() {
		if (!currentUser) return;

		const missing: string[] = [];
		
		if (!currentUser.firstName || !currentUser.lastName) {
			missing.push('Full Name');
		}
		if (!currentUser.preferredCurrency) {
			missing.push('Preferred Currency');
		}
		if (!currentUser.location?.country || !currentUser.location?.city) {
			missing.push('Location (Country & City)');
		}

		missingFields = missing;

		const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
		const hasSeenDemoModal = localStorage.getItem(globalModalKey);
		if (!hasSeenDemoModal) return;

		const hasCompletedOnboarding = localStorage.getItem(`onboarding_completed_${currentUser.id}`);
		if (missing.length > 0 && !hasCompletedOnboarding) {
			showOnboarding = true;
		} else if (missing.length > 0 && !bannerDismissed) {
			showBanner = true;
		}
	}

	function formatCurrency(amount: number): string {
		return formatCurrencyAmount(amount, userCurrency as any);
	}

	function formatDate(date: Date | string): string {
		const dateObj = date instanceof Date ? date : new Date(date);
		return dateObj.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getDisplayBalance(): number {
		return balance || 0;
	}

	function getDisplayTransactions() {
		return Array.isArray(transactions) ? transactions.slice(0, 5) : [];
	}

	function updateUserCurrency(currency: string) {
		if (currentUser) {
			currentUser.preferredCurrency = currency;
		}
	}
</script>

<div class="space-y-4 sm:space-y-6">
	<!-- Balance Cards - 3 columns -->
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6">
		<!-- Local Currency Balance -->
		<div class="bg-white border border-gray-200 p-4 sm:p-6 lg:p-8 rounded-xl sm:rounded-2xl hover:border-gray-300 transition-all">
			<div class="flex justify-between items-start mb-3 sm:mb-4 lg:mb-6">
				<div class="flex-1">
					<div class="flex items-center space-x-2 sm:space-x-3 mb-2 sm:mb-3">
						<div class="w-10 h-10 sm:w-12 sm:h-12 bg-gray-50 rounded-lg sm:rounded-xl flex items-center justify-center shrink-0">
							<span class="text-gray-900 font-bold text-xs sm:text-sm">{userCurrency}</span>
						</div>
						<div class="min-w-0">
							<p class="text-gray-900 font-semibold text-sm sm:text-base truncate">{currencyInfo.name}</p>
							<p class="text-gray-500 text-xs sm:text-sm">Primary balance</p>
						</div>
					</div>
				</div>
				<CurrencySelector 
					currentCurrency={userCurrency}
					onCurrencyChange={updateUserCurrency}
				/>
			</div>
			<div class="mb-4 sm:mb-6">
				<span class="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 font-mono break-all">
					{formatCurrency(getDisplayBalance())}
				</span>
			</div>
			<div class="pt-3 sm:pt-4 border-t border-gray-100 space-y-2 sm:space-y-3">
				<div class="flex justify-between items-center">
					<span class="text-gray-500 text-xs sm:text-sm">Available Balance</span>
					<div class="flex items-center space-x-1">
						<div class="w-2 h-2 bg-green-500 rounded-full"></div>
						<span class="text-green-600 font-medium text-xs sm:text-sm">Active</span>
					</div>
				</div>
				
				<!-- Info: How to add money -->
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-2 sm:p-3 flex items-start space-x-2">
					<Info class="w-3 h-3 sm:w-4 sm:h-4 text-blue-600 shrink-0 mt-0.5" />
					<div class="text-xs text-blue-900">
						<p class="font-semibold mb-1">How to add money:</p>
						<ul class="space-y-0.5 text-blue-800">
							<li>• Deposit cash via agents</li>
							<li>• Sell ckBTC/ckUSDC for cash</li>
							<li>• Receive from other users</li>
						</ul>
					</div>
				</div>
				
				<!-- Last updated timestamp -->
				<div class="text-xs text-gray-400">
					Last updated: {new Date().toLocaleString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })}
				</div>
			</div>
		</div>

		<!-- ckBTC Balance (Encapsulated - fetches own data) -->
		<CkBTCBalanceCard
			showActions={true}
			preferredCurrency={userCurrency}
			onDeposit={() => goto('/users/ckbtc/deposit')}
			onSend={() => goto('/users/ckbtc/send')}
			onExchange={() => goto('/users/ckbtc/exchange')}
		/>

		<!-- ckUSD Balance (Encapsulated - fetches own data) -->
		<CkUSDBalanceCard
			showActions={true}
			preferredCurrency={userCurrency}
			onDeposit={() => goto('/users/ckusdc/deposit')}
			onSend={() => goto('/users/ckusdc/send')}
			onExchange={() => goto('/users/ckusdc/exchange')}
		/>
	</div>

	<!-- Quick Actions -->
	<div>
		<h2 class="text-lg sm:text-xl font-bold text-gray-900 mb-3 sm:mb-4">Quick Actions</h2>
		<div class="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
			<button 
				onclick={() => goto('/users/deposit')}
				class="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
			>
				<div class="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-green-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-green-100 transition-colors">
					<Plus class="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-green-600" />
				</div>
				<span class="text-gray-900 font-semibold text-xs sm:text-sm block">Deposit Cash</span>
				<p class="text-gray-500 text-xs mt-1 hidden sm:block">Add money via agents</p>
			</button>

			<button 
				onclick={() => goto('/users/send')}
				class="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
			>
				<div class="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-blue-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-blue-100 transition-colors">
					<Send class="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-blue-600" />
				</div>
				<span class="text-gray-900 font-semibold text-xs sm:text-sm block">Send Money</span>
				<p class="text-gray-500 text-xs mt-1 hidden sm:block">Transfer to contacts</p>
			</button>

			<button 
				onclick={() => goto('/users/withdraw')}
				class="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
			>
				<div class="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-red-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-red-100 transition-colors">
					<Minus class="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-red-600" />
				</div>
				<span class="text-gray-900 font-semibold text-xs sm:text-sm block">Withdraw Cash</span>
				<p class="text-gray-500 text-xs mt-1 hidden sm:block">Get cash from agents</p>
			</button>

			<button 
				onclick={() => goto('/users/agents')}
				class="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
			>
				<div class="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-purple-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-purple-100 transition-colors">
					<Bitcoin class="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-purple-600" />
				</div>
				<span class="text-gray-900 font-semibold text-xs sm:text-sm block">Find Agents</span>
				<p class="text-gray-500 text-xs mt-1 hidden sm:block">Locate nearby agents</p>
			</button>
		</div>
	</div>

	<!-- Recent Transactions -->
	<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200">
		<div class="p-4 sm:p-6 border-b border-gray-100">
			<div class="flex justify-between items-center">
				<h3 class="text-lg sm:text-xl font-bold text-gray-900">Recent Transactions</h3>
				<button 
					onclick={() => goto('/users/history')}
					class="text-gray-600 text-xs sm:text-sm font-medium hover:text-gray-900 transition-colors px-2 sm:px-4 py-1.5 sm:py-2 rounded-lg hover:bg-gray-50"
				>
					View All
				</button>
			</div>
		</div>
		<div class="divide-y divide-gray-100">
			{#if getDisplayTransactions().length > 0}
				{#each getDisplayTransactions() as transaction}
					<div class="p-3 sm:p-4 lg:p-6 hover:bg-gray-50 transition-colors">
						<!-- Mobile Layout -->
						<div class="sm:hidden">
							<div class="flex items-start space-x-2 sm:space-x-3">
								<div class="w-8 h-8 sm:w-10 sm:h-10 bg-gray-100 rounded-lg sm:rounded-xl flex items-center justify-center shrink-0">
									{#if transaction.type === 'send'}
										<ArrowUp class="w-4 h-4 text-red-500" />
									{:else if transaction.type === 'receive'}
										<ArrowDown class="w-4 h-4 text-green-500" />
									{:else if transaction.type === 'withdraw'}
										<Minus class="w-4 h-4 text-orange-500" />
									{:else if transaction.type === 'deposit'}
										<Plus class="w-4 h-4 text-blue-500" />
									{:else}
										<ArrowUp class="w-4 h-4" />
									{/if}
								</div>
								<div class="flex-1 min-w-0">
									<div class="flex justify-between items-start mb-1">
										<p class="font-medium text-neutral-900 text-xs sm:text-sm truncate pr-2">{transaction.description}</p>
										<p class="font-semibold text-sm sm:text-base font-mono shrink-0 {transaction.type === 'send' || transaction.type === 'withdraw' ? 'text-red-600' : 'text-green-600'}">
											{transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
											{formatCurrency(transaction.amount)}
										</p>
									</div>
									<div class="flex justify-between items-center gap-2">
										<p class="text-xs text-neutral-500 truncate">{formatDate(transaction.createdAt)}</p>
										<p class="text-xs text-neutral-500 capitalize shrink-0">{transaction.status}</p>
									</div>
								</div>
							</div>
						</div>
						
						<!-- Desktop Layout -->
						<div class="hidden sm:flex items-center justify-between">
							<div class="flex items-center space-x-3 lg:space-x-4 flex-1 min-w-0">
								<div class="w-10 h-10 lg:w-12 lg:h-12 bg-neutral-100 rounded-xl flex items-center justify-center shrink-0">
									{#if transaction.type === 'send'}
										<ArrowUp class="w-4 h-4 text-red-500" />
									{:else if transaction.type === 'receive'}
										<ArrowDown class="w-4 h-4 text-green-500" />
									{:else if transaction.type === 'withdraw'}
										<Minus class="w-4 h-4 text-orange-500" />
									{:else if transaction.type === 'deposit'}
										<Plus class="w-4 h-4 text-blue-500" />
									{:else}
										<ArrowUp class="w-4 h-4" />
									{/if}
								</div>
								<div class="min-w-0 flex-1">
									<p class="font-medium text-neutral-900 text-sm lg:text-base truncate">{transaction.description}</p>
									<p class="text-xs sm:text-sm text-neutral-500">{formatDate(transaction.createdAt)}</p>
								</div>
							</div>
							<div class="text-right shrink-0 ml-2 lg:ml-4">
								<p class="font-semibold text-sm lg:text-base font-mono {transaction.type === 'send' || transaction.type === 'withdraw' ? 'text-red-600' : 'text-green-600'}">
									{transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
									{formatCurrency(transaction.amount)}
								</p>
								<p class="text-xs sm:text-sm text-neutral-500 capitalize">{transaction.status}</p>
							</div>
						</div>
					</div>
				{/each}
			{:else}
				<div class="p-6 sm:p-8 lg:p-12 text-center">
					<div class="w-12 h-12 sm:w-14 sm:h-14 lg:w-16 lg:h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
						<ArrowUp class="w-5 h-5 sm:w-6 sm:h-6 lg:w-8 lg:h-8 text-neutral-400" />
					</div>
					<h4 class="text-sm sm:text-base lg:text-lg font-semibold text-neutral-900 mb-2">No transactions yet</h4>
					<p class="text-neutral-500 mb-4 sm:mb-6 text-xs sm:text-sm lg:text-base">Start by sending money or making a deposit</p>
					<button 
						onclick={() => goto('/users/send')}
						class="bg-neutral-900 text-white px-4 sm:px-6 py-2 sm:py-2.5 lg:py-3 rounded-lg font-medium hover:bg-neutral-800 transition-colors text-xs sm:text-sm lg:text-base"
					>
						Send Money
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>
