<script lang="ts">
	import { Settings, Check, Search } from '@lucide/svelte';
	import { getActiveCurrencies } from '$lib/types/currency';

	interface Props {
		currentCurrency: string;
		onCurrencyChange: (currency: string) => void;
	}

	let { currentCurrency, onCurrencyChange }: Props = $props();

	let isOpen = $state(false);
	let searchQuery = $state('');

	// Get all active currencies from the currency type definitions
	const activeCurrencies = getActiveCurrencies();

	const filteredCurrencies = $derived(
		activeCurrencies.filter(c => 
			c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
			c.code.toLowerCase().includes(searchQuery.toLowerCase()) ||
			c.country.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);

	function handleCurrencySelect(currencyCode: string) {
		onCurrencyChange(currencyCode);
		isOpen = false;
		searchQuery = '';
	}
</script>

<div class="relative">
	<!-- Gear Icon Button -->
	<button
		onclick={() => isOpen = !isOpen}
		class="p-2 text-neutral-400 hover:text-neutral-600 hover:bg-neutral-50 rounded-lg transition-all duration-200"
		title="Change Currency"
	>
		<Settings class="w-4 h-4" />
	</button>

	<!-- Dropdown Menu -->
	{#if isOpen}
		<!-- Backdrop -->
		<button
			class="fixed inset-0 z-10 pointer-events-auto"
			onclick={() => isOpen = false}
			aria-label="Close currency selector"
			tabindex="-1"
		></button>
		
		<!-- Currency Selection Dropdown -->
		<div 
			class="absolute right-0 top-full mt-2 w-80 bg-white border border-neutral-200 rounded-xl shadow-lg z-20 flex flex-col max-h-96"
			role="dialog"
			aria-modal="true"
		>
			<div class="p-4 border-b border-neutral-100">
				<h3 class="font-semibold text-neutral-900 text-sm">Select Currency</h3>
				<p class="text-neutral-500 text-xs mt-1">Choose your preferred African currency</p>
			</div>
			
			<!-- Search Input -->
			<div class="p-3 border-b border-neutral-100">
				<div class="relative">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-400" />
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Search currencies..."
						class="w-full pl-9 pr-3 py-2 text-sm border border-neutral-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-neutral-900 focus:border-transparent"
					/>
				</div>
			</div>
			
			<div class="overflow-y-scroll flex-1 min-h-0">
				{#each filteredCurrencies as currency}
					<button
						onclick={() => handleCurrencySelect(currency.code)}
						class="w-full px-4 py-3 text-left hover:bg-neutral-50 transition-colors duration-150 flex items-center justify-between group"
					>
						<div class="flex items-center space-x-3">
							<div class="w-8 h-8 bg-neutral-100 rounded-lg flex items-center justify-center">
								<span class="text-neutral-700 font-semibold text-xs">
									{currency.code}
								</span>
							</div>
							<div>
								<p class="text-neutral-900 font-medium text-sm">
									{currency.name}
								</p>
								<p class="text-neutral-500 text-xs">
									{currency.symbol} • {currency.country}
								</p>
							</div>
						</div>
						
						{#if currentCurrency === currency.code}
							<Check class="w-4 h-4 text-green-600" />
						{/if}
					</button>
				{/each}
			</div>
			
			<div class="p-3 border-t border-neutral-100 bg-neutral-50">
				<p class="text-neutral-500 text-xs text-center">
					Supporting 33 African currencies
				</p>
			</div>
		</div>
	{/if}
</div>
