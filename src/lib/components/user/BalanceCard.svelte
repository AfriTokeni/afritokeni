<script lang="ts">
	import { Eye, EyeOff } from '@lucide/svelte';
	import { formatCurrencyAmount, type AfricanCurrency } from '$lib/types/currency';
	import CurrencySelector from '../dashboard/CurrencySelector.svelte';

	interface Props {
		title: string;
		subtitle?: string;
		balance: number;
		currency: string;
		showBalance: boolean;
		onToggleBalance: () => void;
		onCurrencyChange?: (currency: string) => void;
		showCurrencySelector?: boolean;
		class?: string;
	}

	let {
		title,
		subtitle,
		balance,
		currency,
		showBalance,
		onToggleBalance,
		onCurrencyChange,
		showCurrencySelector = false,
		class: className = ''
	}: Props = $props();
</script>

<div class="bg-white rounded-2xl border border-gray-200 p-4 sm:p-5 md:p-6 {className}">
	<div class="flex flex-col sm:flex-row items-start justify-between gap-3 sm:gap-4">
		<div class="flex-1 w-full sm:w-auto">
			<div class="flex items-center gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
				<p class="text-xs sm:text-sm font-medium text-gray-600">{title}</p>
				{#if subtitle}
					<span class="text-[10px] sm:text-xs text-gray-400">{subtitle}</span>
				{/if}
			</div>
			<div class="flex items-center space-x-2 sm:space-x-3">
				<p class="font-mono text-xl sm:text-2xl md:text-3xl font-bold text-gray-900 break-all">
					{showBalance
						? formatCurrencyAmount(balance, currency as AfricanCurrency)
						: "••••••"}
				</p>
				<button
					onclick={onToggleBalance}
					class="text-gray-400 hover:text-gray-600 shrink-0"
				>
					{#if showBalance}
						<EyeOff class="w-4 h-4 sm:w-5 sm:h-5" />
					{:else}
						<Eye class="w-4 h-4 sm:w-5 sm:h-5" />
					{/if}
				</button>
			</div>
		</div>
		{#if showCurrencySelector && onCurrencyChange}
			<div class="w-full sm:w-auto">
				<CurrencySelector
					currentCurrency={currency}
					{onCurrencyChange}
				/>
			</div>
		{/if}
	</div>
</div>
