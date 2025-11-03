<script lang="ts">
	import { ChevronDown, ChevronUp, CreditCard } from '@lucide/svelte';
	import { formatCurrencyAmount } from '$lib/types/currency';

	interface Props { expanded: boolean; onToggle: () => void; }
	let { expanded, onToggle }: Props = $props();
</script>
<div class="bg-gray-50 rounded-lg overflow-hidden mb-2 sm:mb-3">
<button onclick={onToggle} class="w-full flex items-center justify-between px-3 py-3 sm:px-4 sm:py-3.5 md:px-5 md:py-4 hover:bg-gray-100 transition-colors duration-200">
<div class="flex items-center space-x-2 sm:space-x-3"><CreditCard class="w-4 h-4 sm:w-5 sm:h-5 md:w-6 md:h-6 text-gray-600 shrink-0" /><span class="text-sm sm:text-base md:text-lg text-gray-900">Transaction Limits</span></div>
{#if expanded}<ChevronUp class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />{:else}<ChevronDown class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />{/if}
</button>
	{#if expanded}
		<div class="px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5 border-t border-gray-200">
			<div class="pt-3 sm:pt-4 space-y-3 sm:space-y-4">
				<!-- Limit Cards Grid (from React) -->
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
					<div class="bg-white p-2.5 sm:p-3 rounded-lg">
						<h4 class="font-medium text-gray-900 mb-2 text-xs sm:text-sm md:text-base">Daily Limits</h4>
						<div class="space-y-1.5 sm:space-y-2 text-xs sm:text-sm">
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Send:</span>
								<span class="font-medium wrap-break-word text-right">{formatCurrencyAmount(500000, 'UGX')}</span>
							</div>
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Withdraw:</span>
								<span class="font-medium wrap-break-word text-right">{formatCurrencyAmount(300000, 'UGX')}</span>
							</div>
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Bitcoin:</span>
								<span class="font-medium text-right">0.01 BTC</span>
							</div>
						</div>
					</div>
					<div class="bg-white p-2.5 sm:p-3 rounded-lg">
						<h4 class="font-medium text-gray-900 mb-2 text-xs sm:text-sm md:text-base">Monthly Limits</h4>
						<div class="space-y-1.5 sm:space-y-2 text-xs sm:text-sm">
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Send:</span>
								<span class="font-medium wrap-break-word text-right">{formatCurrencyAmount(10000000, 'UGX')}</span>
							</div>
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Withdraw:</span>
								<span class="font-medium wrap-break-word text-right">{formatCurrencyAmount(5000000, 'UGX')}</span>
							</div>
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Bitcoin:</span>
								<span class="font-medium text-right">0.1 BTC</span>
							</div>
						</div>
					</div>
					<div class="bg-white p-2.5 sm:p-3 rounded-lg sm:col-span-2 lg:col-span-1">
						<h4 class="font-medium text-gray-900 mb-2 text-xs sm:text-sm md:text-base">Per Transaction</h4>
						<div class="space-y-1.5 sm:space-y-2 text-xs sm:text-sm">
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Send:</span>
								<span class="font-medium wrap-break-word text-right">{formatCurrencyAmount(100000, 'UGX')}</span>
							</div>
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Withdraw:</span>
								<span class="font-medium wrap-break-word text-right">{formatCurrencyAmount(50000, 'UGX')}</span>
							</div>
							<div class="flex justify-between gap-2">
								<span class="text-gray-600 shrink-0">Bitcoin:</span>
								<span class="font-medium text-right">0.001 BTC</span>
							</div>
						</div>
					</div>
				</div>

				<!-- Progress Bars (beautiful addition) -->
				<div class="p-3 bg-white rounded-lg">
					<div class="flex justify-between items-center mb-1">
						<span class="text-sm font-medium text-gray-700">Daily Send Limit</span>
						<span class="text-sm font-bold text-gray-900">{formatCurrencyAmount(5000000, 'UGX')}</span>
					</div>
					<div class="w-full bg-gray-200 rounded-full h-2">
						<div class="bg-green-500 h-2 rounded-full" style="width: 30%"></div>
					</div>
					<span class="text-xs text-gray-500 mt-1 block">{formatCurrencyAmount(1500000, 'UGX')} used today</span>
				</div>

				<div class="p-3 bg-white rounded-lg">
					<div class="flex justify-between items-center mb-1">
						<span class="text-sm font-medium text-gray-700">Daily Withdraw Limit</span>
						<span class="text-sm font-bold text-gray-900">{formatCurrencyAmount(3000000, 'UGX')}</span>
					</div>
					<div class="w-full bg-gray-200 rounded-full h-2">
						<div class="bg-blue-500 h-2 rounded-full" style="width: 50%"></div>
					</div>
					<span class="text-xs text-gray-500 mt-1 block">{formatCurrencyAmount(1500000, 'UGX')} used today</span>
				</div>

				<!-- Info Note -->
				<div class="p-3 bg-green-50 rounded-lg border border-green-200">
					<p class="text-xs text-green-800">
						<strong>Verified Account:</strong> Higher limits available. Contact support to increase.
					</p>
				</div>
			</div>
		</div>
	{/if}
</div>
