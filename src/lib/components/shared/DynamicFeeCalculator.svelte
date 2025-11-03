<script lang="ts">
	import { Calculator, Clock, Info, TrendingUp, TrendingDown } from '@lucide/svelte';
	// TODO: Import when service layer is migrated
	// import { DynamicFeeService, type LocationData, type TransactionRequest } from '$lib/services/dynamicFeeService';
	import { AFRICAN_CURRENCIES, type AfricanCurrency } from '$lib/types/currency';
	
	// Temporary mock types until service is migrated
	type LocationData = { latitude: number; longitude: number; accessibility: string };
	type TransactionRequest = any;

	interface Props {
		onFeeCalculated?: (fee: number, breakdown: any) => void;
	}

	let { onFeeCalculated }: Props = $props();

	let amount = $state('100000');
	let currency = $state<AfricanCurrency>('UGX');
	let customerLocation = $state<'urban' | 'suburban' | 'rural' | 'remote'>('urban');
	let distance = $state('5');
	let urgency = $state<'standard' | 'express' | 'emergency'>('standard');
	let feeCalculation = $state<any>(null);

	$effect(() => {
		if (amount && distance) {
			calculateFee();
		}
	});

	function calculateFee() {
		const amountNum = parseFloat(amount);
		const distanceNum = parseFloat(distance);
		
		if (isNaN(amountNum) || isNaN(distanceNum)) return;

		const now = new Date();
		const hour = now.getHours();
		const timeOfDay = hour < 6 ? 'night' : hour < 12 ? 'morning' : hour < 18 ? 'afternoon' : hour < 22 ? 'evening' : 'night';
		const dayOfWeek = now.getDay() === 0 || now.getDay() === 6 ? 'weekend' : 'weekday';

		const request: TransactionRequest = {
			amount: amountNum,
			currency,
			type: 'bitcoin_buy',
			customerLocation: {
				latitude: 0,
				longitude: 0,
				accessibility: customerLocation
			},
			urgency,
			timeOfDay: timeOfDay as any,
			dayOfWeek: dayOfWeek as any
		};

		const agentLocation: LocationData = {
			latitude: 0,
			longitude: 0,
			accessibility: 'urban'
		};

		// TODO: Replace with real service when migrated
		// const calculation = DynamicFeeService.calculateDynamicFee(request, distanceNum, agentLocation);
		const calculation = {
			totalFeePercentage: 0.035,
			totalFeeAmount: amountNum * 0.035,
			agentCommission: amountNum * 0.025,
			platformRevenue: amountNum * 0.01,
			breakdown: [
				{ description: 'Base fee', percentage: 0.02 },
				{ description: 'Distance fee', percentage: 0.01 },
				{ description: 'Location fee', percentage: 0.005 }
			]
		};
		feeCalculation = calculation;
		
		if (onFeeCalculated) {
			onFeeCalculated(calculation.totalFeeAmount, calculation);
		}
	}

	function getLocationColor(accessibility: string) {
		switch (accessibility) {
			case 'urban': return 'text-green-600 bg-green-50';
			case 'suburban': return 'text-blue-600 bg-blue-50';
			case 'rural': return 'text-yellow-600 bg-yellow-50';
			case 'remote': return 'text-red-600 bg-red-50';
			default: return 'text-neutral-600 bg-neutral-50';
		}
	}

	function getUrgencyColor(urgencyLevel: string) {
		switch (urgencyLevel) {
			case 'standard': return 'text-green-600 bg-green-50';
			case 'express': return 'text-yellow-600 bg-yellow-50';
			case 'emergency': return 'text-red-600 bg-red-50';
			default: return 'text-neutral-600 bg-neutral-50';
		}
	}
</script>

<div class="bg-white border border-neutral-200 p-4 sm:p-5 md:p-6 rounded-xl shadow-sm">
	<h3 class="text-base sm:text-lg font-bold text-neutral-900 mb-1.5 sm:mb-2 flex items-center space-x-2">
		<Calculator class="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 shrink-0" />
		<span>Agent Commission Calculator</span>
	</h3>
	<p class="text-xs sm:text-sm text-neutral-600 mb-3 sm:mb-4 wrap-break-word">
		Calculate agent commission based on location, distance, and service level. Agents earn higher fees for serving remote areas.
	</p>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5 md:gap-6">
		<!-- Input Section -->
		<div class="space-y-3 sm:space-y-4">
			<div>
				<label for="amount-input" class="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
					Transaction Amount
				</label>
				<input
					id="amount-input"
					type="number"
					bind:value={amount}
					placeholder="Enter amount"
					class="w-full px-3 sm:px-4 py-2 sm:py-3 border border-neutral-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
				/>
			</div>

			<div>
				<label for="currency-select" class="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
					Currency
				</label>
				<select
					id="currency-select"
					bind:value={currency}
					class="w-full px-3 sm:px-4 py-2 sm:py-3 border border-neutral-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
				>
					{#each Object.entries(AFRICAN_CURRENCIES)
						.filter(([code]) => code !== 'BTC')
						.sort((a, b) => a[1].name.localeCompare(b[1].name)) as [code, info]}
						<option value={code}>
							{code} - {info.name}
						</option>
					{/each}
				</select>
			</div>

			<div>
				<div class="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
					Customer Location Type
				</div>
				<div class="grid grid-cols-2 gap-1.5 sm:gap-2">
					{#each ['urban', 'suburban', 'rural', 'remote'] as type}
						<button
							onclick={() => customerLocation = type as any}
							class="px-2 sm:px-3 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-semibold transition-colors duration-200 {customerLocation === type
								? getLocationColor(type)
								: 'text-neutral-600 bg-neutral-50 hover:bg-neutral-100'}"
						>
							{type.charAt(0).toUpperCase() + type.slice(1)}
						</button>
					{/each}
				</div>
			</div>

			<div>
				<label for="distance-input" class="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
					Distance to Agent (km)
				</label>
				<input
					id="distance-input"
					type="number"
					bind:value={distance}
					placeholder="Distance in kilometers"
					class="w-full px-3 sm:px-4 py-2 sm:py-3 border border-neutral-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
				/>
			</div>

			<div>
				<div class="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
					Service Urgency
				</div>
				<div class="grid grid-cols-3 gap-1.5 sm:gap-2">
					{#each ['standard', 'express', 'emergency'] as type}
						<button
							onclick={() => urgency = type as any}
							class="px-2 sm:px-3 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-semibold transition-colors duration-200 {urgency === type
								? getUrgencyColor(type)
								: 'text-neutral-600 bg-neutral-50 hover:bg-neutral-100'}"
						>
							{type.charAt(0).toUpperCase() + type.slice(1)}
						</button>
					{/each}
				</div>
			</div>
		</div>

		<!-- Results Section -->
		<div class="space-y-3 sm:space-y-4">
			{#if feeCalculation}
				<div class="bg-blue-50 border border-blue-200 p-3 sm:p-4 rounded-lg">
					<div class="text-center">
						<p class="text-blue-600 text-xs sm:text-sm font-semibold mb-1.5 sm:mb-2">Total Fee</p>
						<p class="text-2xl sm:text-3xl font-bold text-neutral-900 font-mono">
							{(feeCalculation.totalFeePercentage * 100).toFixed(2)}%
						</p>
						<p class="text-lg sm:text-xl font-semibold text-neutral-700 font-mono break-all">
							{currency} {feeCalculation.totalFeeAmount.toLocaleString()}
						</p>
					</div>
				</div>

				<div class="grid grid-cols-2 gap-2 sm:gap-3">
					<div class="bg-green-50 border border-green-200 p-2.5 sm:p-3 rounded-lg">
						<p class="text-green-600 text-[10px] sm:text-xs font-semibold mb-0.5 sm:mb-1">Agent Commission</p>
						<p class="text-base sm:text-lg font-bold text-neutral-900 font-mono break-all">
							{currency} {feeCalculation.agentCommission.toLocaleString()}
						</p>
					</div>
					<div class="bg-neutral-50 border border-neutral-200 p-2.5 sm:p-3 rounded-lg">
						<p class="text-neutral-600 text-[10px] sm:text-xs font-semibold mb-0.5 sm:mb-1">Platform Fee</p>
						<p class="text-base sm:text-lg font-bold text-neutral-900 font-mono break-all">
							{currency} {feeCalculation.platformRevenue.toLocaleString()}
						</p>
					</div>
				</div>

				<div class="space-y-1.5 sm:space-y-2">
					<h4 class="text-xs sm:text-sm font-semibold text-neutral-900">Fee Breakdown:</h4>
					{#each feeCalculation.breakdown as item}
						<div class="flex justify-between items-center text-xs sm:text-sm gap-2">
							<span class="text-neutral-600 wrap-break-word flex-1">{item.description}</span>
							<span class="font-semibold text-neutral-900 whitespace-nowrap">
								{(item.percentage * 100).toFixed(2)}%
							</span>
						</div>
					{/each}
				</div>

				<div class="bg-yellow-50 border border-yellow-200 p-2.5 sm:p-3 rounded-lg">
					<div class="flex items-start space-x-1.5 sm:space-x-2">
						<Info class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-yellow-600 mt-0.5 shrink-0" />
						<div class="min-w-0 flex-1">
							<p class="text-yellow-800 font-semibold text-xs sm:text-sm">Dynamic Pricing</p>
							<p class="text-yellow-700 text-[10px] sm:text-xs mt-0.5 sm:mt-1 wrap-break-word">
								Fees adjust based on distance, location accessibility, time, and demand to ensure fair compensation for agents.
							</p>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Fee Comparison -->
	<div class="mt-4 sm:mt-5 md:mt-6 pt-4 sm:pt-5 md:pt-6 border-t border-neutral-200">
		<h4 class="text-xs sm:text-sm font-semibold text-neutral-900 mb-2 sm:mb-3">Fee Comparison Examples:</h4>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-2 sm:gap-3 text-xs sm:text-sm">
			<div class="bg-green-50 border border-green-200 p-2.5 sm:p-3 rounded-lg">
				<div class="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
					<TrendingDown class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-green-600 shrink-0" />
					<span class="font-semibold text-green-800">Urban - Low Fee</span>
				</div>
				<p class="text-green-700 wrap-break-word">5km, Urban, Standard: ~2.5%</p>
			</div>
			<div class="bg-yellow-50 border border-yellow-200 p-2.5 sm:p-3 rounded-lg">
				<div class="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
					<Clock class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-yellow-600 shrink-0" />
					<span class="font-semibold text-yellow-800">Rural - Medium Fee</span>
				</div>
				<p class="text-yellow-700 wrap-break-word">25km, Rural, Express: ~5.5%</p>
			</div>
			<div class="bg-red-50 border border-red-200 p-2.5 sm:p-3 rounded-lg">
				<div class="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
					<TrendingUp class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-red-600 shrink-0" />
					<span class="font-semibold text-red-800">Remote - High Fee</span>
				</div>
				<p class="text-red-700 wrap-break-word">80km, Remote, Emergency: ~10%</p>
			</div>
		</div>
	</div>
</div>
