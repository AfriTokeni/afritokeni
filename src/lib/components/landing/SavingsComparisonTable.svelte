<script lang="ts">
	import { Check, TrendingDown, X, Zap, Bitcoin } from '@lucide/svelte';
	import { getActiveCurrencies, AFRICAN_CURRENCIES, type AfricanCurrency } from '$lib/types/currency';
	
	let amount = $state('100');
	let toCountry = $state<AfricanCurrency>('UGX');
	let location = $state<'urban' | 'suburban' | 'rural' | 'remote'>('urban');

	const activeCurrencies = $derived(getActiveCurrencies().filter(c => c.code !== 'BTC'));
	const toCurrency = $derived(AFRICAN_CURRENCIES[toCountry]);
	const numAmount = $derived(parseFloat(amount) || 0);
	const useCkUSDC = $derived(numAmount < 100);

	// AfriTokeni fees based on location
	const feeMap = {
		urban: 3.25,
		suburban: 4,
		rural: 5.5,
		remote: 9.5
	};
	
	const afriTokeniFeePercent = $derived(feeMap[location]);
	const afriTokeniFee = $derived((numAmount * afriTokeniFeePercent) / 100);

	// Competitor fees
	const westernUnionFee = $derived(
		numAmount < 100 ? 4.99 : numAmount < 500 ? 9.99 : numAmount < 1000 ? 24.99 : 49.99
	);
	const moneyGramFee = $derived(
		numAmount < 100 ? 4.99 : numAmount < 500 ? 9.99 : numAmount < 1000 ? 24.99 : 49.99
	);
	const worldRemitFee = $derived(
		numAmount < 100 ? 3.99 : numAmount < 500 ? 7.99 : numAmount < 1000 ? 19.99 : 39.99
	);
	const mPesaFee = $derived(numAmount * 0.13);
	const mtnFee = $derived(numAmount * 0.11);
	const airtelFee = $derived(numAmount * 0.10);

	const avgCompetitorFee = $derived(
		(westernUnionFee + moneyGramFee + worldRemitFee + mPesaFee + mtnFee + airtelFee) / 6
	);
	const savingsDollar = $derived(Math.max(0, avgCompetitorFee - afriTokeniFee));
	const savings = $derived(
		avgCompetitorFee > 0 ? Math.max(0, Math.round((savingsDollar / avgCompetitorFee) * 100)) : 0
	);

	function handleAmountChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const val = target.value;
		if (val === '' || /^\d+$/.test(val)) {
			amount = val;
		}
	}

	function handleAmountBlur() {
		const num = parseFloat(amount) || 10;
		if (num < 10) amount = '10';
	}
</script>

<section id="savings" class="bg-gradient-to-b from-gray-50 to-white py-12 sm:py-16 md:py-20 lg:py-24">
	<div class="max-w-7xl mx-auto px-4 sm:px-6">
		<!-- Header -->
		<div class="text-center mb-8 sm:mb-10 lg:mb-12">
			<div class="inline-flex items-center gap-1.5 sm:gap-2 bg-green-50 text-green-700 px-3 sm:px-4 py-1.5 sm:py-2 rounded-full text-xs sm:text-sm font-semibold mb-3 sm:mb-4">
				<TrendingDown class="w-3 h-3 sm:w-4 sm:h-4" />
				Save Up to {savings}% on Transfer Fees
			</div>
			<h2 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-3 sm:mb-4 px-4 sm:px-0">
				See How Much You Save
			</h2>
			<p class="text-base sm:text-lg lg:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
				Compare AfriTokeni with traditional money transfer services across 39 African countries
			</p>
		</div>

		<!-- Calculator -->
		<div class="bg-linear-to-br from-gray-50 to-white rounded-2xl shadow-[0_8px_30px_rgb(0,0,0,0.04)] p-8 mb-10 border border-gray-200/60 backdrop-blur-sm">
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<!-- Amount -->
				<div class="group">
					<label class="block text-sm font-medium text-gray-700 mb-2.5">
						Transfer Amount
					</label>
					<div class="relative">
						<span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 text-base font-medium">$</span>
						<input
							type="text"
							value={amount}
							oninput={handleAmountChange}
							onblur={handleAmountBlur}
							placeholder="100"
							class="w-full pl-9 pr-4 py-3.5 bg-white border border-gray-200 rounded-xl text-base font-medium text-gray-900 placeholder:text-gray-400 
							focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 
							transition-all duration-200 shadow-sm hover:border-gray-300"
						/>
					</div>
				</div>

				<!-- Destination -->
				<div class="group">
					<label class="block text-sm font-medium text-gray-700 mb-2.5">
						Destination Country
					</label>
					<div class="relative">
						<select
							bind:value={toCountry}
							class="w-full px-4 py-3.5 bg-white border border-gray-200 rounded-xl text-base font-medium text-gray-900 
							focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 
							transition-all duration-200 shadow-sm hover:border-gray-300 appearance-none cursor-pointer"
							style="background-image: url('data:image/svg+xml,%3Csvg xmlns=\'http://www.w3.org/2000/svg\' fill=\'none\' viewBox=\'0 0 20 20\'%3E%3Cpath stroke=\'%239ca3af\' stroke-linecap=\'round\' stroke-linejoin=\'round\' stroke-width=\'2\' d=\'M6 8l4 4 4-4\'/%3E%3C/svg%3E'); background-position: right 1rem center; background-repeat: no-repeat; background-size: 1.25em 1.25em; padding-right: 3rem;"
						>
							{#each activeCurrencies as currency}
								<option value={currency.code}>
									{currency.country} ({currency.code})
								</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Location -->
				<div class="group">
					<label class="block text-sm font-medium text-gray-700 mb-2.5">
						Recipient Location
					</label>
					<div class="relative">
						<select
							bind:value={location}
							class="w-full px-4 py-3.5 bg-white border border-gray-200 rounded-xl text-base font-medium text-gray-900 
							focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 
							transition-all duration-200 shadow-sm hover:border-gray-300 appearance-none cursor-pointer"
							style="background-image: url('data:image/svg+xml,%3Csvg xmlns=\'http://www.w3.org/2000/svg\' fill=\'none\' viewBox=\'0 0 20 20\'%3E%3Cpath stroke=\'%239ca3af\' stroke-linecap=\'round\' stroke-linejoin=\'round\' stroke-width=\'2\' d=\'M6 8l4 4 4-4\'/%3E%3C/svg%3E'); background-position: right 1rem center; background-repeat: no-repeat; background-size: 1.25em 1.25em; padding-right: 3rem;"
						>
							<option value="urban">Urban (City) - 2.5-4%</option>
							<option value="suburban">Suburban - 3-5%</option>
							<option value="rural">Rural (Village) - 4-7%</option>
							<option value="remote">Remote Area - 7-12%</option>
						</select>
					</div>
				</div>
			</div>
		</div>

		<!-- Mobile Cards (Hidden on desktop) -->
		<div class="block sm:hidden space-y-3 sm:space-y-4 mb-6">
			<!-- AfriTokeni Card -->
			<div class="bg-green-50 border border-green-200 rounded-xl p-3 sm:p-4">
				<div class="flex items-center gap-2 sm:gap-3 mb-2 sm:mb-3">
					<div class="w-8 h-8 sm:w-10 sm:h-10 bg-gray-900 rounded-lg flex items-center justify-center font-bold text-white text-sm">A</div>
					<div>
						<div class="font-bold text-gray-900 text-sm sm:text-base">AfriTokeni</div>
						<div class="text-xs text-gray-600 capitalize">{location} Rate</div>
					</div>
					<div class="ml-auto">
						<span class="bg-green-100 text-green-800 text-xs font-semibold px-2 py-1 rounded-full">
							BEST
						</span>
					</div>
				</div>
				<div class="grid grid-cols-2 gap-2 text-xs sm:text-sm">
					<div><span class="font-semibold">Fee:</span> ${afriTokeniFee.toFixed(2)} ({afriTokeniFeePercent}%)</div>
					<div><span class="font-semibold">Speed:</span> Instant</div>
					<div><span class="font-semibold">USSD:</span> Yes</div>
					<div><span class="font-semibold">Countries:</span> 39</div>
				</div>
			</div>

			<!-- Competitor Cards -->
			{#each [
				{ name: 'Western Union', fee: westernUnionFee, speed: 'Minutes-Hours', ussd: false, countries: '200+' },
				{ name: 'MoneyGram', fee: moneyGramFee, speed: 'Minutes-Hours', ussd: false, countries: '200+' },
				{ name: 'WorldRemit', fee: worldRemitFee, speed: 'Minutes-Days', ussd: false, countries: '150+' },
				{ name: 'M-Pesa', fee: mPesaFee, speed: 'Minutes-Hours', ussd: true, countries: 'East Africa' },
				{ name: 'MTN MoMo', fee: mtnFee, speed: 'Minutes-Hours', ussd: true, countries: 'West/Central' },
				{ name: 'Airtel Money', fee: airtelFee, speed: 'Minutes-Hours', ussd: true, countries: '14' }
			] as provider}
				<div class="bg-white border border-gray-200 rounded-xl p-3 sm:p-4">
					<div class="font-semibold text-gray-900 mb-2 text-sm sm:text-base">{provider.name}</div>
					<div class="grid grid-cols-2 gap-2 text-xs sm:text-sm">
						<div><span class="font-semibold">Fee:</span> ${provider.fee.toFixed(2)}</div>
						<div><span class="font-semibold">Speed:</span> {provider.speed}</div>
						<div><span class="font-semibold">USSD:</span> {provider.ussd ? 'Yes' : 'No'}</div>
						<div><span class="font-semibold">Countries:</span> {provider.countries}</div>
					</div>
				</div>
			{/each}
		</div>

		<!-- Desktop Table -->
		<div class="hidden sm:block overflow-hidden rounded-2xl shadow-[0_20px_50px_rgba(0,0,0,0.08)] border border-gray-200 mb-6">
			<table class="w-full border-collapse bg-white min-w-[640px]">
				<thead>
					<tr class="bg-linear-to-br from-slate-900 via-slate-800 to-slate-900 border-b border-slate-700">
						<th class="text-left py-5 px-8 text-white/90 font-semibold text-sm tracking-wide">Provider</th>
						<th class="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">Transfer Fee</th>
						<th class="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">Speed</th>
						<th class="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">USSD</th>
						<th class="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">Bitcoin</th>
						<th class="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">Coverage</th>
					</tr>
				</thead>
				<tbody>
					<!-- AfriTokeni Row -->
					<tr class="relative border-b-2 border-blue-200 bg-gradient-to-r from-blue-50 via-indigo-50/30 to-blue-50 hover:from-blue-100/80 hover:via-indigo-100/40 hover:to-blue-100/80 transition-all duration-300 group">
						<td class="py-7 px-8">
							<div class="flex items-center gap-4">
								<div class="relative">
									<div class="w-12 h-12 bg-linear-to-br from-blue-600 via-blue-600 to-indigo-700 rounded-xl flex items-center justify-center font-bold text-white text-lg shadow-xl shadow-blue-500/30 ring-2 ring-blue-100">
										A
									</div>
									<div class="absolute -top-1 -right-1 w-4 h-4 bg-green-500 rounded-full border-2 border-white"></div>
								</div>
								<div>
									<div class="flex items-center gap-2.5 mb-1">
										<span class="font-bold text-gray-900 text-base">AfriTokeni</span>
										<span class="bg-gradient-to-r from-blue-600 to-indigo-600 text-white text-xs px-2.5 py-1 rounded-full font-semibold shadow-sm">
											Recommended
										</span>
									</div>
									<div class="text-xs text-gray-600 font-medium capitalize">{location} Rate</div>
								</div>
							</div>
						</td>
						<td class="py-7 px-6 text-center">
							<div class="text-3xl font-bold bg-gradient-to-r from-green-600 via-emerald-600 to-green-600 bg-clip-text text-transparent">
								${afriTokeniFee.toFixed(2)}
							</div>
							<div class="text-xs text-emerald-600 font-bold mt-1.5 tracking-wide">
								{afriTokeniFeePercent}% FEE
							</div>
						</td>
						<td class="py-7 px-6 text-center">
							<div class="inline-flex items-center gap-2 bg-gradient-to-r from-green-50 to-emerald-50 text-green-700 px-4 py-2 rounded-lg text-sm font-semibold border border-green-200/50">
								<Zap class="w-4 h-4 fill-current" />
								Instant
							</div>
							<div class="text-xs text-gray-500 mt-2 font-medium">&lt;1 second</div>
						</td>
						<td class="py-7 px-6 text-center">
							<div class="inline-flex items-center justify-center gap-2 bg-gradient-to-r from-green-50 to-emerald-50 text-green-700 px-4 py-2 rounded-lg text-sm font-semibold border border-green-200/50">
								<Check class="w-4 h-4 stroke-[2.5]" />
								Yes
							</div>
							<div class="text-xs text-gray-500 mt-2 font-medium">*229#</div>
						</td>
						<td class="py-7 px-6 text-center">
							<div class="inline-flex items-center justify-center gap-2 bg-gradient-to-r from-orange-50 to-amber-50 text-orange-700 px-4 py-2 rounded-lg text-sm font-semibold border border-orange-200/50">
								<Bitcoin class="w-4 h-4" />
								Yes
							</div>
							<div class="text-xs text-orange-600 mt-2 font-bold">ckBTC</div>
						</td>
						<td class="py-7 px-6 text-center">
							<div class="text-xl font-bold text-gray-900">39</div>
							<div class="text-xs text-gray-500 mt-1 font-medium">All Africa</div>
						</td>
					</tr>

					<!-- Competitor Rows -->
					{#each [
						{ name: 'Western Union', subtitle: 'International Transfer', fee: westernUnionFee, speed: 'Minutes - Hours', ussd: false, bitcoin: false, coverage: '200+' },
						{ name: 'MoneyGram', subtitle: 'International', fee: moneyGramFee, speed: 'Minutes - Hours', ussd: false, bitcoin: false, coverage: '200+' },
						{ name: 'WorldRemit', subtitle: 'International', fee: worldRemitFee, speed: 'Minutes - Days', ussd: false, bitcoin: false, coverage: '150+' },
						{ name: 'M-Pesa', subtitle: 'International Transfer', fee: mPesaFee, speed: 'Minutes - Hours', ussd: true, bitcoin: false, coverage: 'East Africa' },
						{ name: 'MTN MoMo', subtitle: 'Cross-border', fee: mtnFee, speed: 'Minutes - Hours', ussd: true, bitcoin: false, coverage: 'West/Central' },
						{ name: 'Airtel Money', subtitle: 'International', fee: airtelFee, speed: 'Minutes - Hours', ussd: true, bitcoin: false, coverage: '14' }
					] as provider}
						<tr class="border-b border-gray-100 hover:bg-slate-50/50 transition-all duration-200">
							<td class="py-5 px-8">
								<div class="font-semibold text-gray-900 text-sm">{provider.name}</div>
								<div class="text-xs text-gray-500 mt-0.5">{provider.subtitle}</div>
							</td>
							<td class="py-5 px-6 text-center">
								<div class="text-xl font-bold text-gray-900">${provider.fee.toFixed(2)}</div>
							</td>
							<td class="py-5 px-6 text-center">
								<div class="text-sm text-gray-600 font-medium">{provider.speed}</div>
							</td>
							<td class="py-5 px-6 text-center">
								{#if provider.ussd}
									<div class="inline-flex items-center gap-1.5 text-green-600 font-semibold text-sm">
										<Check class="w-4 h-4" />
										Yes
									</div>
								{:else}
									<div class="inline-flex items-center gap-1.5 text-gray-400 text-sm font-medium">
										<X class="w-4 h-4 stroke-2" />
										No
									</div>
								{/if}
							</td>
							<td class="py-5 px-6 text-center">
								<div class="inline-flex items-center gap-1.5 text-gray-400 text-sm font-medium">
									<X class="w-4 h-4 stroke-2" />
									No
								</div>
							</td>
							<td class="py-5 px-6 text-center">
								<div class="text-sm text-gray-700 font-semibold">{provider.coverage}</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Savings Summary -->
		<div class="mt-6 sm:mt-8 bg-gradient-to-r from-green-50 to-emerald-50 rounded-xl p-4 sm:p-6 lg:p-8 border border-green-200">
			<div class="text-center">
				<div class="text-2xl sm:text-3xl lg:text-4xl xl:text-5xl font-bold text-green-600 mb-2">
					Save ${savingsDollar.toFixed(2)}
				</div>
				<div class="text-base sm:text-lg lg:text-xl text-gray-700">
					That's <span class="font-bold text-green-600">{savings}% less</span> than average competitor
				</div>
				<div class="text-xs sm:text-sm text-gray-600 mt-2">
					Sending ${numAmount} to {toCurrency.country} ({location} area)
					{#if useCkUSDC}
						<span class="text-green-600 font-semibold"> • ckUSDC (Stablecoin)</span>
					{:else}
						<span class="text-green-600 font-semibold"> • ckBTC (ICP Bitcoin)</span>
					{/if}
				</div>
			</div>
		</div>

		<!-- Bottom Note -->
		<div class="mt-6 sm:mt-8 text-center px-4">
			<p class="text-xs sm:text-sm text-gray-500 max-w-3xl mx-auto leading-relaxed">
				* AfriTokeni uses ICP-native ckBTC (Bitcoin) and ckUSDC (stablecoin) for instant transfers. 
				Fees: 2.5-4% urban, 4-7% rural, 7-12% remote (fair agent compensation). 
				Mobile money: M-Pesa ~13%, MTN MoMo ~11%, Airtel Money ~10% (includes FX markups). 
				AfriTokeni works via USSD across all 39 African currencies.
			</p>
		</div>

		<!-- CTA -->
		<div class="mt-8 sm:mt-10 lg:mt-12 text-center">
			<a
				href="#get-started"
				class="inline-flex items-center gap-2 bg-gray-900 text-white px-6 sm:px-8 py-3 sm:py-4 rounded-xl text-base sm:text-lg font-semibold hover:bg-gray-800 transform hover:scale-105 transition-all duration-200 shadow-lg hover:shadow-xl"
			>
				<TrendingDown class="w-4 h-4 sm:w-5 sm:h-5" />
				Start Saving Today
			</a>
		</div>
	</div>
</section>
