<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import DashboardLayout from '$lib/components/dashboard/DashboardLayout.svelte';
	import DemoModeModal from '$lib/components/dashboard/DemoModeModal.svelte';
	import KYCStatusAlert from '$lib/components/dashboard/KYCStatusAlert.svelte';
	import CurrencySelector from '$lib/components/dashboard/CurrencySelector.svelte';
	import CkBTCBalanceCard from '$lib/components/dashboard/CkBTCBalanceCard.svelte';
	import CkUSDCBalanceCard from '$lib/components/dashboard/CkUSDCBalanceCard.svelte';
	import OnboardingModal from '$lib/components/dashboard/OnboardingModal.svelte';
	import ProfileIncompleteBanner from '$lib/components/dashboard/ProfileIncompleteBanner.svelte';
	import { Send, Bitcoin, ArrowUp, ArrowDown, Minus, Plus, Info } from '@lucide/svelte';

	// State
	let isDemoMode = $state(false);
	let showDemoModal = $state(false);
	let showOnboarding = $state(false);
	let showBanner = $state(false);
	let missingFields = $state<string[]>([]);
	let bannerDismissed = $state(false);
	
	// Mock user data
	let currentUser = $state({
		id: 'user123',
		firstName: '',
		lastName: '',
		email: 'user@example.com',
		preferredCurrency: 'UGX',
		kycStatus: 'not_started' as 'not_started' | 'pending' | 'rejected' | 'approved',
		location: { country: '', city: '' }
	});

	let userCurrency = $derived(currentUser.preferredCurrency || 'UGX');
	let balance = $state(250000);
	let transactions = $state([
		{ id: 1, type: 'receive', amount: 50000, description: 'Received from John Doe', createdAt: '2025-10-30', status: 'completed' },
		{ id: 2, type: 'send', amount: 25000, description: 'Sent to Jane Smith', createdAt: '2025-10-29', status: 'completed' },
		{ id: 3, type: 'deposit', amount: 100000, description: 'Cash deposit via Agent #123', createdAt: '2025-10-28', status: 'completed' }
	]);

	onMount(() => {
		// Show demo modal on first login
		const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
		const hasSeenModal = localStorage.getItem(globalModalKey);
		
		if (!hasSeenModal) {
			showDemoModal = true;
			localStorage.setItem(globalModalKey, 'true');
		}

		// Check for missing profile fields
		const missing: string[] = [];
		if (!currentUser.firstName || !currentUser.lastName) missing.push('Full Name');
		if (!currentUser.preferredCurrency) missing.push('Preferred Currency');
		if (!currentUser.location?.country || !currentUser.location?.city) missing.push('Location (Country & City)');
		
		missingFields = missing;

		// Show onboarding if profile incomplete
		const hasCompletedOnboarding = localStorage.getItem(`onboarding_completed_${currentUser.id}`);
		if (missing.length > 0 && !hasCompletedOnboarding && hasSeenModal) {
			showOnboarding = true;
		} else if (missing.length > 0 && !bannerDismissed) {
			showBanner = true;
		}
	});

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-UG', {
			style: 'currency',
			currency: 'UGX',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(date: string): string {
		return new Date(date).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getTransactionIcon(type: string) {
		switch (type) {
			case 'send': return ArrowUp;
			case 'receive': return ArrowDown;
			case 'withdraw': return Minus;
			case 'deposit': return Plus;
			default: return ArrowUp;
		}
	}

	function handleOnboardingComplete(data: any) {
		localStorage.setItem(`onboarding_completed_${currentUser.id}`, 'true');
		showOnboarding = false;
		showBanner = false;
		// TODO: Update user profile
	}

	function enableDemoMode() {
		isDemoMode = true;
	}

	function updateUserCurrency(currency: string) {
		currentUser.preferredCurrency = currency;
	}
</script>

<svelte:head>
	<title>Dashboard - AfriTokeni</title>
</svelte:head>

<DemoModeModal 
	isOpen={showDemoModal}
	onClose={() => showDemoModal = false}
	userType="user"
	onEnableDemo={enableDemoMode}
/>

<OnboardingModal
	isOpen={showOnboarding}
	onClose={() => {
		showOnboarding = false;
		localStorage.setItem(`onboarding_completed_${currentUser.id}`, 'true');
	}}
	onComplete={handleOnboardingComplete}
	currentData={{
		firstName: currentUser.firstName,
		lastName: currentUser.lastName,
		email: currentUser.email,
		phone: '',
		preferredCurrency: currentUser.preferredCurrency,
		country: currentUser.location?.country || '',
		city: currentUser.location?.city || ''
	}}
/>

<DashboardLayout userType="user">
	<div class="space-y-4 sm:space-y-6">
		{#if showBanner && missingFields.length > 0}
			<ProfileIncompleteBanner
				{missingFields}
				onDismiss={() => {
					bannerDismissed = true;
					showBanner = false;
				}}
				onComplete={() => {
					showBanner = false;
					showOnboarding = true;
				}}
			/>
		{/if}

		<KYCStatusAlert userType="user" kycStatus={currentUser.kycStatus} />

		<!-- Balance Cards -->
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6">
			<!-- Local Currency Balance -->
			<div class="bg-white border border-gray-200 p-4 sm:p-6 lg:p-8 rounded-xl sm:rounded-2xl hover:border-gray-300 transition-all">
				<div class="flex justify-between items-start mb-3 sm:mb-4 lg:mb-6">
					<div class="flex-1">
						<div class="flex items-center space-x-2 sm:space-x-3 mb-2 sm:mb-3">
							<div class="w-10 h-10 sm:w-12 sm:h-12 bg-gray-50 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
								<span class="text-gray-900 font-bold text-xs sm:text-sm">{userCurrency}</span>
							</div>
							<div class="min-w-0">
								<p class="text-gray-900 font-semibold text-sm sm:text-base truncate">Ugandan Shilling</p>
								<p class="text-gray-500 text-xs sm:text-sm">Primary balance</p>
							</div>
						</div>
					</div>
					<CurrencySelector 
						currentCurrency={userCurrency}
						onCurrencyChange={(currency) => updateUserCurrency(currency)}
					/>
				</div>
				<div class="mb-4 sm:mb-6">
					<span class="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 font-mono break-all">
						{formatCurrency(balance)}
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
					
					<div class="bg-blue-50 border border-blue-200 rounded-lg p-2 sm:p-3 flex items-start space-x-2">
						<Info class="w-3 h-3 sm:w-4 sm:h-4 text-blue-600 flex-shrink-0 mt-0.5" />
						<div class="text-xs text-blue-900">
							<p class="font-semibold mb-1">How to add money:</p>
							<ul class="space-y-0.5 text-blue-800">
								<li>• Deposit cash via agents</li>
								<li>• Sell ckBTC/ckUSDC for cash</li>
								<li>• Receive from other users</li>
							</ul>
						</div>
					</div>
					
					<div class="text-xs text-gray-400">
						Last updated: {new Date().toLocaleString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })}
			<CkBTCBalanceCard
				principalId={currentUser.id}
				preferredCurrency={userCurrency}
				showActions={true}
				isDemoMode={isDemoMode}
				onDeposit={() => goto('/users/ckbtc/deposit')}
				onSend={() => goto('/users/ckbtc/send')}
				onExchange={() => goto('/users/ckbtc/exchange')}
			/>

			<CkUSDCBalanceCard
				principalId={currentUser.id}
				preferredCurrency={userCurrency}
				showActions={true}
				isDemoMode={isDemoMode}
				onDeposit={() => goto('/users/ckusdc/deposit')}
				onSend={() => goto('/users/ckusdc/send')}
				onExchange={() => goto('/users/ckusdc/exchange')}
			/>
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
				{#if transactions.length > 0}
					{#each transactions.slice(0, 5) as transaction}
						<div class="p-3 sm:p-4 lg:p-6 hover:bg-gray-50 transition-colors">
							<!-- Mobile Layout -->
							<div class="sm:hidden">
								<div class="flex items-start space-x-2 sm:space-x-3">
									<div class="w-8 h-8 sm:w-10 sm:h-10 bg-gray-100 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
										<svelte:component this={getTransactionIcon(transaction.type)} class="w-4 h-4 {transaction.type === 'send' || transaction.type === 'withdraw' ? 'text-red-500' : transaction.type === 'receive' ? 'text-green-500' : transaction.type === 'deposit' ? 'text-blue-500' : 'text-orange-500'}" />
									</div>
									<div class="flex-1 min-w-0">
										<div class="flex justify-between items-start mb-1">
											<p class="font-medium text-neutral-900 text-xs sm:text-sm truncate pr-2">{transaction.description}</p>
											<p class="font-semibold text-sm sm:text-base font-mono flex-shrink-0 {transaction.type === 'send' || transaction.type === 'withdraw' ? 'text-red-600' : 'text-green-600'}">
												{transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
												{formatCurrency(transaction.amount)}
											</p>
										</div>
										<div class="flex justify-between items-center gap-2">
											<p class="text-xs text-neutral-500 truncate">{formatDate(transaction.createdAt)}</p>
											<p class="text-xs text-neutral-500 capitalize flex-shrink-0">{transaction.status}</p>
										</div>
									</div>
								</div>
							</div>
							
							<!-- Desktop Layout -->
							<div class="hidden sm:flex items-center justify-between">
								<div class="flex items-center space-x-3 lg:space-x-4 flex-1 min-w-0">
									<div class="w-10 h-10 lg:w-12 lg:h-12 bg-neutral-100 rounded-xl flex items-center justify-center flex-shrink-0">
										<svelte:component this={getTransactionIcon(transaction.type)} class="w-4 h-4 {transaction.type === 'send' || transaction.type === 'withdraw' ? 'text-red-500' : transaction.type === 'receive' ? 'text-green-500' : transaction.type === 'deposit' ? 'text-blue-500' : 'text-orange-500'}" />
									</div>
									<div class="min-w-0 flex-1">
										<p class="font-medium text-neutral-900 text-sm lg:text-base truncate">{transaction.description}</p>
										<p class="text-xs sm:text-sm text-neutral-500">{formatDate(transaction.createdAt)}</p>
									</div>
								</div>
								<div class="text-right flex-shrink-0 ml-2 lg:ml-4">
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
</DashboardLayout>
