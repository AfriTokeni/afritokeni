<script lang="ts">
	import { formatCurrencyAmount, type AfricanCurrency } from '$lib/types/currency';
	import CurrencySelector from './CurrencySelector.svelte';

	interface Props {
		balance: number;
		currency: string;
		onCurrencyChange?: (currency: string) => void;
		showCurrencySelector?: boolean;
	}

	let { balance, currency, onCurrencyChange, showCurrencySelector = true }: Props = $props();
</script>

<div class="bg-white rounded-lg sm:rounded-xl border border-neutral-200 p-4 sm:p-6">
	<div class="flex items-center justify-between mb-3 sm:mb-4">
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-2 mb-1">
				<span class="text-xs font-medium text-neutral-500">{currency}</span>
				<span class="text-xs text-neutral-400">Primary Balance</span>
			</div>
			<p class="text-2xl sm:text-3xl font-bold text-neutral-900 font-mono truncate">
				{currency} {formatCurrencyAmount(balance, currency as AfricanCurrency).replace(currency, '').trim()}
			</p>
		</div>
		{#if showCurrencySelector && onCurrencyChange}
			<CurrencySelector
				currentCurrency={currency}
				{onCurrencyChange}
			/>
		{/if}
	</div>
	<div class="flex items-center gap-2 text-xs text-neutral-500">
		<span class="inline-flex items-center gap-1">
			<span class="w-2 h-2 bg-green-500 rounded-full"></span>
			Active
		</span>
	</div>
</div>
