<script lang="ts">
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { toast } from '$lib/stores/toast';
	import TransactionHistory from '$lib/components/shared/TransactionHistory.svelte';
	import { Search, User, Phone, MapPin, Calendar, Loader2, CheckCircle, Shield, TrendingUp, X, Ban, PhoneCall, History } from '@lucide/svelte';

	interface Customer {
		id: string;
		name: string;
		phone: string;
		location: string;
		joinDate: string;
		totalTransactions: number;
		totalVolume: {
			ugx: number;
			usdc: number;
		};
		lastTransaction: string;
		status: 'active' | 'inactive' | 'blocked';
		kycStatus: 'verified' | 'pending' | 'rejected';
	}

	let customers = $state<Customer[]>([]);
	let searchTerm = $state('');
	let statusFilter = $state<string>('all');
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedCustomer = $state<Customer | null>(null);
	let showModal = $state(false);
	let showBlockConfirm = $state(false);
	let showHistoryModal = $state(false);

	function openCustomerModal(customer: Customer) {
		selectedCustomer = customer;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		selectedCustomer = null;
	}

	function handleCallCustomer() {
		if (selectedCustomer) {
			// In real app, this would initiate a call
			window.location.href = `tel:${selectedCustomer.phone}`;
			toast.show('info', `Calling ${selectedCustomer.name}...`);
		}
	}

	function handleViewHistory() {
		if (selectedCustomer) {
			showHistoryModal = true;
		}
	}

	function handleBlockCustomer() {
		if (selectedCustomer) {
			showBlockConfirm = true;
		}
	}

	function confirmBlock() {
		if (selectedCustomer) {
			const customerName = selectedCustomer.name;
			const customerId = selectedCustomer.id;
			// Update customer status
			customers = customers.map((c) =>
				c.id === customerId ? { ...c, status: 'blocked' as const } : c
			);
			toast.show('success', `${customerName} has been blocked`);
			showBlockConfirm = false;
			closeModal();
		}
	}

	// Auto-fetch when stores change
	$effect(() => {
		loadCustomers($demoMode, $principalId);
	});

	async function loadCustomers(isDemoMode: boolean, agentPrincipal: string | null) {
		try {
			isLoading = true;
			error = null;

			if (isDemoMode) {
				const response = await fetch('/data/demo/agent-customers.json');
				if (!response.ok) throw new Error('Failed to load demo customers');
				customers = await response.json();
			} else {
				// Real canister call
				// const result = await customerService.get_agent_customers(agentPrincipal);
				// customers = result;
				customers = [];
			}
		} catch (err: any) {
			error = err.message;
			customers = [];
		} finally {
			isLoading = false;
		}
	}

	const filteredCustomers = $derived(
		customers.filter((customer) => {
			const matchesSearch =
				customer.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
				customer.phone.includes(searchTerm) ||
				customer.location.toLowerCase().includes(searchTerm.toLowerCase());

			const matchesStatus = statusFilter === 'all' || customer.status === statusFilter;

			return matchesSearch && matchesStatus;
		})
	);

	function getTimeAgo(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60));

		if (diffInMinutes < 60) {
			return `${diffInMinutes}m ago`;
		} else if (diffInMinutes < 1440) {
			return `${Math.floor(diffInMinutes / 60)}h ago`;
		} else {
			return `${Math.floor(diffInMinutes / 1440)}d ago`;
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'active':
				return 'bg-green-100 text-green-800 border-green-200';
			case 'inactive':
				return 'bg-yellow-100 text-yellow-800 border-yellow-200';
			case 'blocked':
				return 'bg-red-100 text-red-800 border-red-200';
			default:
				return 'bg-gray-100 text-gray-800 border-gray-200';
		}
	}

	function getKycStatusColor(status: string): string {
		switch (status) {
			case 'verified':
				return 'bg-green-100 text-green-800 border-green-200';
			case 'pending':
				return 'bg-yellow-100 text-yellow-800 border-yellow-200';
			case 'rejected':
				return 'bg-red-100 text-red-800 border-red-200';
			default:
				return 'bg-gray-100 text-gray-800 border-gray-200';
		}
	}

</script>

<!-- Customer Stats -->
{#if !isLoading && !error}
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
		<div class="bg-white border border-gray-200 rounded-2xl p-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center shrink-0">
					<User class="w-5 h-5 text-blue-600" />
				</div>
				<div>
					<p class="text-gray-600 text-sm">Total Customers</p>
					<p class="text-2xl font-bold text-gray-900">{customers.length}</p>
				</div>
			</div>
		</div>

		<div class="bg-white border border-gray-200 rounded-2xl p-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center shrink-0">
					<CheckCircle class="w-5 h-5 text-green-600" />
				</div>
				<div>
					<p class="text-gray-600 text-sm">Active Customers</p>
					<p class="text-2xl font-bold text-gray-900">
						{customers.filter((c) => c.status === 'active').length}
					</p>
				</div>
			</div>
		</div>

		<div class="bg-white border border-gray-200 rounded-2xl p-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center shrink-0">
					<Shield class="w-5 h-5 text-purple-600" />
				</div>
				<div>
					<p class="text-gray-600 text-sm">KYC Verified</p>
					<p class="text-2xl font-bold text-gray-900">
						{customers.filter((c) => c.kycStatus === 'verified').length}
					</p>
				</div>
			</div>
		</div>

		<div class="bg-white border border-gray-200 rounded-2xl p-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-orange-100 rounded-lg flex items-center justify-center shrink-0">
					<TrendingUp class="w-5 h-5 text-orange-600" />
				</div>
				<div>
					<p class="text-gray-600 text-sm">Avg. Transactions</p>
					<p class="text-2xl font-bold text-gray-900">
						{customers.length > 0
							? (
									customers.reduce((sum, c) => sum + c.totalTransactions, 0) / customers.length
								).toFixed(1)
							: '0'}
					</p>
				</div>
			</div>
		</div>
	</div>
{/if}

<!-- Search and Filters -->
<div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-5 md:p-6 mb-6">
	<div class="flex flex-col sm:flex-row gap-3 sm:gap-4">
		<!-- Search -->
		<div class="flex-1 relative">
			<Search
				class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4 sm:w-5 sm:h-5 shrink-0"
			/>
			<input
				type="text"
				placeholder="Search by name, phone, or location..."
				bind:value={searchTerm}
				class="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2 sm:py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent text-sm sm:text-base"
			/>
		</div>
	</div>

	<!-- Filter Buttons -->
	<div class="flex gap-1.5 sm:gap-2 overflow-x-auto scrollbar-hide pb-2 mt-3 sm:mt-4">
		<button
			onclick={() => (statusFilter = 'all')}
			class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {statusFilter ===
			'all'
				? 'bg-black text-white'
				: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
		>
			All
		</button>
		<button
			onclick={() => (statusFilter = 'active')}
			class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {statusFilter ===
			'active'
				? 'bg-black text-white'
				: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
		>
			Active
		</button>
		<button
			onclick={() => (statusFilter = 'inactive')}
			class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {statusFilter ===
			'inactive'
				? 'bg-black text-white'
				: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
		>
			Inactive
		</button>
		<button
			onclick={() => (statusFilter = 'blocked')}
			class="px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors shrink-0 whitespace-nowrap {statusFilter ===
			'blocked'
				? 'bg-black text-white'
				: 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'}"
		>
			Blocked
		</button>
	</div>
</div>

<!-- Customer List -->
<div class="bg-white rounded-2xl shadow-sm border border-gray-200">
	<div class="px-4 sm:px-5 md:px-6 py-3 sm:py-4 border-b border-gray-200">
		<h2 class="text-sm sm:text-base md:text-lg font-semibold text-gray-900">
			Customer List ({filteredCustomers.length})
		</h2>
	</div>

	<div class="p-4 sm:p-5 md:p-6">
		{#if isLoading}
			<div class="text-center py-8 sm:py-10 md:py-12">
				<Loader2 class="w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 text-gray-400 mx-auto mb-3 sm:mb-4 animate-spin" />
				<h3 class="text-base sm:text-lg font-semibold text-gray-900 mb-1.5 sm:mb-2">
					Loading customers...
				</h3>
				<p class="text-sm sm:text-base text-gray-600">
					Please wait while we fetch your customer data.
				</p>
			</div>
		{:else if error}
			<div class="text-center py-8 sm:py-10 md:py-12">
				<User class="w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 text-red-400 mx-auto mb-3 sm:mb-4" />
				<h3 class="text-base sm:text-lg font-semibold text-red-900 mb-1.5 sm:mb-2">
					Error Loading Customers
				</h3>
				<p class="text-sm sm:text-base text-red-600 mb-3 sm:mb-4 px-4">{error}</p>
				<button
					onclick={() => window.location.reload()}
					class="bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition-colors text-xs sm:text-sm"
				>
					Retry
				</button>
			</div>
		{:else if filteredCustomers.length === 0}
			<div class="text-center py-8 sm:py-10 md:py-12">
				<User class="w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 text-gray-400 mx-auto mb-3 sm:mb-4" />
				<h3 class="text-base sm:text-lg font-semibold text-gray-900 mb-1.5 sm:mb-2">
					No customers found
				</h3>
				<p class="text-sm sm:text-base text-gray-600">
					Try adjusting your search or filter criteria.
				</p>
			</div>
		{:else}
			<div class="space-y-3 sm:space-y-4">
				{#each filteredCustomers as customer (customer.id)}
					<div
						class="bg-white border border-gray-200 rounded-2xl p-3 sm:p-4 md:p-5 lg:p-6 hover:shadow-md transition-all duration-200 cursor-pointer hover:border-gray-300"
						onclick={() => openCustomerModal(customer)}
						role="button"
						tabindex="0"
					>
						<div
							class="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-3 sm:space-y-0 gap-3"
						>
							<div class="flex items-center space-x-3 sm:space-x-4 min-w-0 flex-1">
								<!-- Customer Avatar -->
								<div
									class="hidden sm:flex w-10 h-10 md:w-12 md:h-12 bg-black rounded-full items-center justify-center shrink-0"
								>
									<span class="text-white font-bold text-sm">
										{customer.name
											.split(' ')
											.map((n) => n[0])
											.join('')
											.toUpperCase()
											.slice(0, 2)}
									</span>
								</div>

								<!-- Customer Info -->
								<div class="flex-1 min-w-0">
									<h3 class="text-sm sm:text-base md:text-lg font-semibold text-gray-900 truncate">
										{customer.name}
									</h3>
									<div
										class="flex flex-col sm:flex-row sm:items-center space-y-1 sm:space-y-0 sm:space-x-3 md:space-x-4 text-xs sm:text-sm text-gray-600 mt-0.5 sm:mt-1"
									>
										<div class="flex items-center space-x-1">
											<Phone class="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 shrink-0" />
											<span class="truncate">{customer.phone}</span>
										</div>
										<div class="flex items-center space-x-1">
											<MapPin class="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 shrink-0" />
											<span class="truncate">{customer.location}</span>
										</div>
										<div class="flex items-center space-x-1">
											<Calendar class="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 shrink-0" />
											<span class="whitespace-nowrap"
												>Joined {new Date(customer.joinDate).toLocaleDateString()}</span
											>
										</div>
									</div>
								</div>
							</div>

							<!-- Customer Stats and Status -->
							<div class="text-left sm:text-right shrink-0">
								<!-- Transaction Count -->
								<div class="text-[10px] sm:text-xs text-gray-500 mb-1.5 sm:mb-2">
									{customer.totalTransactions} transactions • Last: {getTimeAgo(
										customer.lastTransaction
									)}
								</div>

								<!-- Status Badges -->
								<div class="flex flex-wrap sm:justify-end gap-1.5 sm:gap-2">
									<span
										class="inline-flex items-center px-2 sm:px-2.5 py-0.5 rounded-full text-[10px] sm:text-xs font-semibold border whitespace-nowrap {getStatusColor(
											customer.status
										)}"
									>
										{customer.status.charAt(0).toUpperCase() + customer.status.slice(1)}
									</span>
									<span
										class="inline-flex items-center px-2 sm:px-2.5 py-0.5 rounded-full text-[10px] sm:text-xs font-semibold border whitespace-nowrap {getKycStatusColor(
											customer.kycStatus
										)}"
									>
										KYC: {customer.kycStatus.charAt(0).toUpperCase() + customer.kycStatus.slice(1)}
									</span>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- Customer Details Modal -->
{#if showModal && selectedCustomer}
	<div
		class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
		onclick={closeModal}
		role="button"
		tabindex="-1"
	>
		<div
			class="bg-white rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
		>
			<!-- Modal Header -->
			<div class="sticky top-0 bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between rounded-t-2xl">
				<h2 class="text-xl font-bold text-gray-900">Customer Details</h2>
				<button
					onclick={closeModal}
					class="text-gray-400 hover:text-gray-600 transition-colors"
				>
					<X class="w-6 h-6" />
				</button>
			</div>

			<!-- Modal Content -->
			<div class="p-6 space-y-6">
				<!-- Customer Info -->
				<div class="flex items-center gap-4">
					<div class="w-16 h-16 bg-black rounded-full flex items-center justify-center shrink-0">
						<span class="text-white font-bold text-xl">
							{selectedCustomer.name
								.split(' ')
								.map((n) => n[0])
								.join('')
								.toUpperCase()
								.slice(0, 2)}
						</span>
					</div>
					<div>
						<h3 class="text-2xl font-bold text-gray-900">{selectedCustomer.name}</h3>
						<div class="flex items-center gap-2 mt-1">
							<span
								class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold border {getStatusColor(
									selectedCustomer.status
								)}"
							>
								{selectedCustomer.status.charAt(0).toUpperCase() + selectedCustomer.status.slice(1)}
							</span>
							<span
								class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold border {getKycStatusColor(
									selectedCustomer.kycStatus
								)}"
							>
								KYC: {selectedCustomer.kycStatus.charAt(0).toUpperCase() +
									selectedCustomer.kycStatus.slice(1)}
							</span>
						</div>
					</div>
				</div>

				<!-- Contact Info -->
				<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
					<div class="bg-gray-50 rounded-lg p-4">
						<div class="flex items-center gap-2 text-gray-600 mb-1">
							<Phone class="w-4 h-4" />
							<span class="text-sm font-medium">Phone</span>
						</div>
						<p class="text-gray-900 font-semibold">{selectedCustomer.phone}</p>
					</div>
					<div class="bg-gray-50 rounded-lg p-4">
						<div class="flex items-center gap-2 text-gray-600 mb-1">
							<MapPin class="w-4 h-4" />
							<span class="text-sm font-medium">Location</span>
						</div>
						<p class="text-gray-900 font-semibold">{selectedCustomer.location}</p>
					</div>
				</div>

				<!-- Stats -->
				<div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4 text-center">
						<p class="text-gray-900 text-2xl font-bold">{selectedCustomer.totalTransactions}</p>
						<p class="text-gray-600 text-xs mt-1">Transactions</p>
					</div>
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4 text-center">
						<p class="text-gray-900 text-2xl font-bold">
							{(selectedCustomer.totalVolume.ugx / 1000000).toFixed(1)}M
						</p>
						<p class="text-gray-600 text-xs mt-1">UGX Volume</p>
					</div>
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4 text-center">
						<p class="text-gray-900 text-2xl font-bold">
							${selectedCustomer.totalVolume.usdc.toFixed(0)}
						</p>
						<p class="text-gray-600 text-xs mt-1">USDC Volume</p>
					</div>
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4 text-center">
						<p class="text-gray-900 text-sm font-bold">
							{getTimeAgo(selectedCustomer.lastTransaction)}
						</p>
						<p class="text-gray-600 text-xs mt-1">Last Active</p>
					</div>
				</div>

				<!-- Join Date -->
				<div class="bg-gray-50 rounded-lg p-4">
					<div class="flex items-center gap-2 text-gray-600 mb-1">
						<Calendar class="w-4 h-4" />
						<span class="text-sm font-medium">Member Since</span>
					</div>
					<p class="text-gray-900 font-semibold">
						{new Date(selectedCustomer.joinDate).toLocaleDateString('en-US', {
							year: 'numeric',
							month: 'long',
							day: 'numeric'
						})}
					</p>
				</div>

				<!-- Actions -->
				<div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
					<button
						onclick={handleCallCustomer}
						class="flex items-center justify-center gap-2 px-4 py-3 bg-black text-white rounded-lg hover:bg-gray-800 transition-colors"
					>
						<PhoneCall class="w-4 h-4" />
						<span class="font-medium">Call Customer</span>
					</button>
					<button
						onclick={handleViewHistory}
						class="flex items-center justify-center gap-2 px-4 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
					>
						<History class="w-4 h-4" />
						<span class="font-medium">View History</span>
					</button>
					<button
						onclick={handleBlockCustomer}
						class="flex items-center justify-center gap-2 px-4 py-3 border border-red-300 text-red-600 rounded-lg hover:bg-red-50 transition-colors"
					>
						<Ban class="w-4 h-4" />
						<span class="font-medium">Block Customer</span>
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<!-- Block Confirmation Modal -->
{#if showBlockConfirm && selectedCustomer}
	<div
		class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
		onclick={() => (showBlockConfirm = false)}
		role="button"
		tabindex="-1"
	>
		<div
			class="bg-white rounded-2xl max-w-md w-full p-6"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
		>
			<h3 class="text-xl font-bold text-gray-900 mb-4">Block Customer?</h3>
			<p class="text-gray-600 mb-6">
				Are you sure you want to block <strong>{selectedCustomer.name}</strong>? They will not be able to make transactions until unblocked.
			</p>
			<div class="flex gap-3">
				<button
					onclick={() => (showBlockConfirm = false)}
					class="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={confirmBlock}
					class="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
				>
					Block Customer
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Transaction History Modal -->
{#if showHistoryModal && selectedCustomer}
	<div
		class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
		onclick={() => (showHistoryModal = false)}
		role="button"
		tabindex="-1"
	>
		<div
			class="bg-white rounded-2xl max-w-3xl w-full max-h-[90vh] overflow-y-auto"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
		>
			<!-- Header -->
			<div class="sticky top-0 bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between rounded-t-2xl">
				<div>
					<h2 class="text-xl font-bold text-gray-900">Transaction History</h2>
					<p class="text-sm text-gray-600 mt-1">{selectedCustomer.name}</p>
				</div>
				<button
					onclick={() => (showHistoryModal = false)}
					class="text-gray-400 hover:text-gray-600 transition-colors"
				>
					<X class="w-6 h-6" />
				</button>
			</div>

			<!-- Transaction History Component -->
			<div class="p-6">
				<TransactionHistory maxTransactions={50} showViewAll={false} showFilters={true} />
			</div>
		</div>
	</div>
{/if}
