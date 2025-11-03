<script lang="ts">
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { toast } from '$lib/stores/toast';
	import {
		CheckCircle,
		XCircle,
		Clock,
		User,
		Phone,
		MapPin,
		AlertCircle,
		Search,
		X,
		Info
	} from '@lucide/svelte';

	let showInstructions = $state(false);

	interface WithdrawalRequest {
		id: string;
		userId: string;
		userName: string;
		userPhone: string;
		amount: number;
		currency: string;
		code: string;
		status: 'pending' | 'confirmed' | 'completed' | 'rejected';
		createdAt: string;
		userLocation?: string;
		userPhoto?: string;
	}

	function getUserInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	let withdrawalRequests = $state<WithdrawalRequest[]>([]);
	let selectedRequest = $state<WithdrawalRequest | null>(null);
	let verificationCodes = $state<Record<string, string>>({});
	let isProcessing = $state(false);
	let loading = $state(true);
	let error = $state('');
	let filter = $state<'all' | 'pending' | 'confirmed' | 'completed' | 'rejected'>('pending');
	let searchQuery = $state('');

	// Auto-fetch when stores change
	$effect(() => {
		loadWithdrawalRequests($demoMode, $principalId);
	});

	async function loadWithdrawalRequests(isDemoMode: boolean, agentPrincipal: string | null) {
		// In demo mode, we don't need a principal
		if (!isDemoMode && !agentPrincipal) {
			withdrawalRequests = [];
			loading = false;
			return;
		}

		try {
			loading = true;

			if (isDemoMode) {
				const response = await fetch('/data/demo/demo-withdrawal-requests.json');
				if (!response.ok) throw new Error('Failed to load demo withdrawals');
				withdrawalRequests = await response.json();
			} else {
				// Real canister call
				// const result = await withdrawalCanister.get_agent_withdrawals(agentPrincipal);
				// withdrawalRequests = result;
				withdrawalRequests = [];
			}
		} catch (err: any) {
			error = err.message;
			withdrawalRequests = [];
		} finally {
			loading = false;
		}
	}

	async function handleVerifyCode(request: WithdrawalRequest) {
		const currentCode = (verificationCodes[request.id] || '').trim().toUpperCase();
		const expectedCode = request.code.trim().toUpperCase();

		if (currentCode === expectedCode) {
			try {
				if ($demoMode) {
					// Demo: mark as confirmed
					withdrawalRequests = withdrawalRequests.map((r) =>
						r.id === request.id ? { ...r, status: 'confirmed' as const } : r
					);
					selectedRequest = { ...request, status: 'confirmed' };
					error = '';
				} else {
					// Real canister call
					// await withdrawalCanister.confirm_withdrawal(request.id);
					await loadWithdrawalRequests($demoMode, $principalId);
					selectedRequest = request;
					error = '';
				}
			} catch (err: any) {
				error = 'Failed to confirm withdrawal request. Please try again.';
			}
		} else {
			error = `Invalid withdrawal code. Expected: ${expectedCode}, Got: ${currentCode}`;
		}
	}

	async function handleConfirmWithdrawal(request: WithdrawalRequest) {
		isProcessing = true;
		try {
			if ($demoMode) {
				// Demo: mark as completed (change status, don't remove)
				await new Promise((resolve) => setTimeout(resolve, 1000));
				withdrawalRequests = withdrawalRequests.map((r) =>
					r.id === request.id ? { ...r, status: 'completed' as const } : r
				);
				selectedRequest = null;
				verificationCodes[request.id] = '';
				error = '';
				toast.show('success', `Withdrawal completed! ${formatAmount(request.amount, request.currency)} given to ${request.userName}.`);
			} else {
				// Real canister call
				// await withdrawalCanister.process_withdrawal(request.id, $principalId);
				await loadWithdrawalRequests($demoMode, $principalId);
				selectedRequest = null;
				verificationCodes[request.id] = '';
				error = '';
			}
		} catch (err: any) {
			error = 'Failed to process withdrawal. Please try again.';
		} finally {
			isProcessing = false;
		}
	}

	async function handleRejectWithdrawal(request: WithdrawalRequest) {
		isProcessing = true;
		try {
			if ($demoMode) {
				await new Promise((resolve) => setTimeout(resolve, 500));
				withdrawalRequests = withdrawalRequests.map((r) =>
					r.id === request.id ? { ...r, status: 'rejected' as const } : r
				);
				selectedRequest = null;
				verificationCodes[request.id] = '';
				error = '';
				toast.show('info', `Withdrawal from ${request.userName} has been rejected.`);
			} else {
				// Real canister call
				// await withdrawalCanister.reject_withdrawal(request.id);
				await loadWithdrawalRequests($demoMode, $principalId);
				selectedRequest = null;
				verificationCodes[request.id] = '';
				error = '';
				toast.show('info', `Withdrawal from ${request.userName} has been rejected.`);
			}
		} catch (err: any) {
			error = 'Failed to reject withdrawal. Please try again.';
			toast.show('error', 'Failed to reject withdrawal. Please try again.');
		} finally {
			isProcessing = false;
		}
	}

	const filteredRequests = $derived(
		withdrawalRequests.filter((request) => {
			const statusMatch = filter === 'all' || request.status === filter;
			const searchMatch =
				!searchQuery ||
				request.userName.toLowerCase().includes(searchQuery.toLowerCase()) ||
				request.userPhone.includes(searchQuery);
			return statusMatch && searchMatch;
		})
	);

	function formatAmount(amount: number, currency: string): string {
		return new Intl.NumberFormat('en-UG', {
			style: 'currency',
			currency: currency,
			minimumFractionDigits: 0
		}).format(amount);
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'pending':
				return 'text-yellow-600 bg-yellow-50 border-yellow-200';
			case 'confirmed':
				return 'text-blue-600 bg-blue-50 border-blue-200';
			case 'completed':
				return 'text-green-600 bg-green-50 border-green-200';
			case 'rejected':
				return 'text-red-600 bg-red-50 border-red-200';
			default:
				return 'text-neutral-600 bg-neutral-50 border-neutral-200';
		}
	}
</script>

<!-- Error Message -->
{#if error}
	<div class="bg-red-50 border-2 border-red-500 rounded-2xl p-3 sm:p-4 shadow-lg mb-6">
		<div class="flex items-start">
			<AlertCircle class="h-5 w-5 sm:h-6 sm:w-6 text-red-600 mr-2 sm:mr-3 mt-0.5 shrink-0" />
			<div class="flex-1 min-w-0">
				<h3 class="font-semibold text-red-900 mb-1 text-sm sm:text-base">Verification Failed</h3>
				<p class="text-xs sm:text-sm text-red-700">{error}</p>
			</div>
			<button onclick={() => (error = '')} class="text-red-400 hover:text-red-600 shrink-0">
				<X class="h-4 w-4 sm:h-5 sm:w-5" />
			</button>
		</div>
	</div>
{/if}

<!-- Collapsible Instructions -->
{#if showInstructions}
	<div class="bg-blue-50 border border-blue-200 rounded-2xl p-4 sm:p-5 md:p-6 mb-6">
		<div class="flex items-start justify-between mb-2 sm:mb-3">
			<h3 class="font-semibold text-blue-900 text-sm sm:text-base">Withdrawal Process:</h3>
			<button
				onclick={() => (showInstructions = false)}
				class="text-blue-600 hover:text-blue-800 shrink-0"
			>
				<X class="w-4 h-4" />
			</button>
		</div>
		<ol class="text-xs sm:text-sm text-blue-800 space-y-1.5 sm:space-y-2 list-decimal list-inside">
			<li>Customer shows you their withdrawal code</li>
			<li>Verify the code matches the request</li>
			<li>Give the cash amount to customer</li>
			<li>Complete the withdrawal to debit their digital balance</li>
			<li>Customer receives confirmation notification</li>
		</ol>
		<div class="mt-3 sm:mt-4 p-2.5 sm:p-3 bg-blue-100 rounded-lg">
			<p class="text-xs sm:text-sm text-blue-900 font-medium">
				⚠️ Always verify the withdrawal code before giving cash. Rejected withdrawals cannot be reversed.
			</p>
		</div>
		{#if $demoMode}
			<div class="mt-3 sm:mt-4 p-2.5 sm:p-3 bg-purple-100 rounded-lg border border-purple-200">
				<p class="text-xs sm:text-sm text-purple-900 font-medium">
					🎭 Demo: Alice (WTH4MN), Peter (WTH7PQ), Grace (WTH2RS)
				</p>
			</div>
		{/if}
	</div>
{/if}

<!-- Search and Filter -->
<div class="bg-white rounded-2xl border border-gray-200 p-4 sm:p-5 md:p-6">
	<!-- Header with Info Button -->
	<div class="flex items-center justify-between mb-4">
		<h2 class="text-lg sm:text-xl font-semibold text-gray-900">Withdrawal Requests</h2>
		<button
			onclick={() => (showInstructions = !showInstructions)}
			class="flex items-center gap-2 px-3 sm:px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors shadow-sm"
		>
			<Info class="w-4 h-4 sm:w-5 sm:h-5" />
			<span>How it works</span>
		</button>
	</div>

	<!-- Search Bar -->
	<div class="mb-4 sm:mb-5 md:mb-6">
		<div class="relative">
			<Search
				class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0"
			/>
			<input
				type="text"
				placeholder="Search by name or phone number..."
				bind:value={searchQuery}
				class="w-full pl-9 sm:pl-10 pr-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-500 focus:border-transparent text-sm sm:text-base"
			/>
		</div>
	</div>

	<!-- Filter Tabs -->
	<div
		class="flex space-x-1 bg-gray-100 rounded-lg p-1 mb-4 sm:mb-5 md:mb-6 overflow-x-auto scrollbar-hide"
	>
		{#each ['all', 'pending', 'confirmed', 'completed', 'rejected'] as tab}
			<button
				onclick={() => (filter = tab as any)}
				class="shrink-0 py-1.5 sm:py-2 px-2 sm:px-4 rounded-md text-xs sm:text-sm font-medium transition-colors {filter ===
				tab
					? 'bg-white text-neutral-900 shadow-sm'
					: 'text-neutral-600 hover:text-neutral-900'}"
			>
				<span class="block sm:inline whitespace-nowrap"
					>{tab.charAt(0).toUpperCase() + tab.slice(1)}</span
				>
				<span
					class="ml-1 sm:ml-2 text-xs bg-neutral-200 text-neutral-600 px-1.5 sm:px-2 py-0.5 rounded-full"
				>
					{tab === 'all'
						? withdrawalRequests.length
						: withdrawalRequests.filter((r) => r.status === tab).length}
				</span>
			</button>
		{/each}
	</div>

	<!-- Withdrawal Requests List -->
	<div class="space-y-3 sm:space-y-4">
		{#if loading}
			<div class="text-center py-8 sm:py-10 md:py-12">
				<div
					class="animate-spin w-7 h-7 sm:w-8 sm:h-8 border-2 border-blue-600 border-t-transparent rounded-full mx-auto mb-3 sm:mb-4"
				></div>
				<p class="text-neutral-600 text-sm sm:text-base">Loading withdrawal requests...</p>
			</div>
		{:else if filteredRequests.length === 0}
			<div class="text-center py-8 sm:py-10 md:py-12">
				<Clock class="w-10 h-10 sm:w-12 sm:h-12 text-neutral-400 mx-auto mb-3 sm:mb-4 shrink-0" />
				<h3 class="text-base sm:text-lg font-semibold text-neutral-900 mb-2">
					No withdrawal requests
				</h3>
				<p class="text-neutral-600 text-sm sm:text-base">
					{filter === 'pending'
						? 'No pending withdrawals at the moment.'
						: `No ${filter} withdrawals found.`}
				</p>
			</div>
		{:else}
			{#each filteredRequests as request (request.id)}
				<div
					class="border border-gray-200 rounded-2xl p-3 sm:p-4 hover:border-gray-300 transition-colors space-y-3 sm:space-y-4"
				>
					<!-- Header: User Info + Status -->
					<div class="flex items-start justify-between gap-2 sm:gap-3">
						<div class="flex items-start space-x-2 sm:space-x-3 min-w-0 flex-1">
							<!-- User Photo/Avatar -->
							<div class="w-10 h-10 sm:w-12 sm:h-12 rounded-full shrink-0 overflow-hidden border-2 border-gray-200">
								{#if request.userPhoto}
									<img
										src={request.userPhoto}
										alt={request.userName}
										class="w-full h-full object-cover"
									/>
								{:else}
									<div class="w-full h-full bg-black flex items-center justify-center">
										<span class="text-white font-bold text-sm sm:text-base">
											{getUserInitials(request.userName)}
										</span>
									</div>
								{/if}
							</div>
							<div class="min-w-0 flex-1">
								<h3 class="font-semibold text-gray-900 truncate text-sm sm:text-base">
									{request.userName}
								</h3>
								<div class="flex flex-col text-xs sm:text-sm text-gray-600 mt-0.5">
									<div class="flex items-center space-x-1">
										<Phone class="w-3 h-3 sm:w-3.5 sm:h-3.5 shrink-0" />
										<span class="truncate">{request.userPhone}</span>
									</div>
									{#if request.userLocation}
										<div class="flex items-center space-x-1 mt-0.5">
											<MapPin class="w-3 h-3 sm:w-3.5 sm:h-3.5 shrink-0" />
											<span class="truncate">{request.userLocation}</span>
										</div>
									{/if}
								</div>
							</div>
						</div>
						<div
							class="inline-flex items-center space-x-1 px-1.5 sm:px-2 py-0.5 sm:py-1 rounded-md border text-xs font-medium shrink-0 {getStatusColor(
								request.status
							)}"
						>
							{#if request.status === 'pending'}
								<Clock class="w-4 h-4 sm:w-5 sm:h-5" />
							{:else if request.status === 'confirmed'}
								<AlertCircle class="w-4 h-4 sm:w-5 sm:h-5" />
							{:else if request.status === 'completed'}
								<CheckCircle class="w-4 h-4 sm:w-5 sm:h-5" />
							{:else}
								<XCircle class="w-4 h-4 sm:w-5 sm:h-5" />
							{/if}
							<span class="capitalize hidden sm:inline">{request.status}</span>
						</div>
					</div>

					<!-- Amount Section -->
					<div class="bg-gray-50 rounded-lg p-2.5 sm:p-3">
						<div class="text-xs text-gray-500 mb-0.5 sm:mb-1">Withdrawal Amount</div>
						<div class="text-xl sm:text-2xl font-bold text-gray-900 font-mono">
							{formatAmount(request.amount, request.currency)}
						</div>
					</div>

					<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 sm:gap-3">
						<div class="text-xs sm:text-sm text-gray-600">
							{#if !$demoMode}
								<span>Code: </span>
								<span class="font-mono font-semibold text-gray-900">{request.code}</span>
								<span class="ml-2 sm:ml-4"></span>
							{/if}
							<span>{new Date(request.createdAt).toLocaleString()}</span>
						</div>

						<div class="flex flex-col sm:flex-row gap-2">
							{#if request.status === 'pending'}
								<div class="flex items-center space-x-2">
									<input
										type="text"
										placeholder="Enter code"
										bind:value={verificationCodes[request.id]}
										class="flex-1 sm:w-32 px-2 sm:px-3 py-1 border border-neutral-300 rounded text-xs sm:text-sm font-mono uppercase"
									/>
									<button
										onclick={() => handleVerifyCode(request)}
										disabled={!verificationCodes[request.id]}
										class="px-2 sm:px-3 py-1 bg-blue-600 text-white rounded text-xs sm:text-sm hover:bg-blue-700 disabled:bg-neutral-300 whitespace-nowrap"
									>
										Verify
									</button>
								</div>
							{/if}

							{#if request.status === 'confirmed' && selectedRequest?.id === request.id}
								<div class="flex flex-col sm:flex-row gap-2">
									<button
										onclick={() => handleConfirmWithdrawal(request)}
										disabled={isProcessing}
										class="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-green-600 text-white rounded text-xs sm:text-sm hover:bg-green-700 disabled:bg-neutral-300 flex items-center justify-center space-x-1"
									>
										{#if isProcessing}
											<div
												class="animate-spin rounded-full h-3.5 w-3.5 sm:h-4 sm:w-4 border-b-2 border-white"
											></div>
											<span>Processing...</span>
										{:else}
											<CheckCircle class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" />
											<span>Complete Withdrawal</span>
										{/if}
									</button>
									<button
										onclick={() => handleRejectWithdrawal(request)}
										disabled={isProcessing}
										class="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-red-600 text-white rounded text-xs sm:text-sm hover:bg-red-700 disabled:bg-neutral-300 flex items-center justify-center space-x-1"
									>
										<XCircle class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" />
										<span>Reject</span>
									</button>
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
