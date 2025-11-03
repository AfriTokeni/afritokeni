<script lang="ts">
	import { goto } from '$app/navigation';
	import { Banknote } from '@lucide/svelte';
	import { formatCurrencyAmount, type AfricanCurrency } from '$lib/types/currency';

	type Agent = {
		id: string;
		businessName: string;
		location: { city: string; latitude: number; longitude: number };
	};

	interface Props {
		localAmount: number;
		btcAmount: string;
		withdrawType: 'cash' | 'bitcoin' | 'ckusdc';
		userCurrency: string;
		fee: number;
		userLocation: [number, number] | null;
		selectedAgent?: Agent;
		withdrawalCode: string;
		onMakeAnotherWithdrawal: () => void;
	}

	let { localAmount, btcAmount, withdrawType, userCurrency, fee, userLocation, selectedAgent, withdrawalCode, onMakeAnotherWithdrawal }: Props = $props();
</script>

<div class="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-6 sm:p-8 text-center">
	<div class="w-16 h-16 sm:w-20 sm:h-20 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4 sm:mb-6">
		<Banknote class="w-8 h-8 sm:w-10 sm:h-10 text-green-600" />
	</div>
	
	<h2 class="text-xl sm:text-2xl font-bold text-gray-900 mb-2">Withdrawal Confirmed!</h2>
	<p class="text-sm sm:text-base text-gray-600 mb-4 sm:mb-6 px-2">
		Your withdrawal of {formatCurrencyAmount(localAmount, userCurrency as AfricanCurrency)} is ready
	</p>

	<div class="bg-gray-50 rounded-lg p-4 sm:p-6 mb-4 sm:mb-6">
		<div class="space-y-2 sm:space-y-3 text-xs sm:text-sm">
			<div class="flex justify-between gap-2">
				<span class="text-gray-600">Withdrawal Code:</span>
				<span class="font-mono font-bold break-all">{withdrawalCode}</span>
			</div>
			{#if selectedAgent}
				<div class="flex justify-between gap-2">
					<span class="text-gray-600">Agent:</span>
					<span class="font-medium truncate">{selectedAgent.businessName}</span>
				</div>
			{/if}
			<div class="flex justify-between gap-2">
				<span class="text-gray-600">Amount:</span>
				<span class="font-medium">{formatCurrencyAmount(localAmount, userCurrency as AfricanCurrency)}</span>
			</div>
			<div class="flex justify-between gap-2">
				<span class="text-gray-600">Fee:</span>
				<span class="font-medium">{formatCurrencyAmount(fee, userCurrency as AfricanCurrency)}</span>
			</div>
		</div>
	</div>

	<div class="flex flex-col sm:flex-row gap-3 sm:gap-4">
		<button
			onclick={() => goto('/users/dashboard')}
			class="flex-1 bg-gray-100 text-gray-700 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-200 transition-colors"
		>
			Back to Dashboard
		</button>
		<button
			onclick={onMakeAnotherWithdrawal}
			class="flex-1 bg-gray-900 text-white py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 transition-colors"
		>
			Make Another Withdrawal
		</button>
	</div>
</div>
