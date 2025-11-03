<script lang="ts">
	import { 
		Calculator, 
		MapPin, 
		Clock, 
		Zap, 
		Info, 
		TrendingUp, 
		TrendingDown,
		Users,
		Shield,
		CircleCheckBig,
		Bitcoin
	} from '@lucide/svelte';

	let activeSection = $state<'overview' | 'calculator' | 'examples'>('overview');

	const locationTypes = [
		{
			type: 'Urban Areas',
			description: 'Cities and towns with good infrastructure',
			feeRange: '2.5-4%',
			color: 'green',
			icon: '🏢',
			features: ['Quick access', 'Multiple agents', 'Good connectivity', 'Standard rates']
		},
		{
			type: 'Suburban Areas',
			description: 'Outskirts of cities, moderate accessibility',
			feeRange: '3-5%',
			color: 'blue',
			icon: '🏘️',
			features: ['Moderate travel', 'Good infrastructure', 'Regular service', 'Fair pricing']
		},
		{
			type: 'Rural Areas',
			description: 'Villages with basic infrastructure',
			feeRange: '4-7%',
			color: 'yellow',
			icon: '🌾',
			features: ['Some travel required', 'Fewer agents', 'Basic infrastructure', 'Higher compensation']
		},
		{
			type: 'Remote Villages',
			description: 'Hard-to-reach areas with limited access',
			feeRange: '7-12%',
			color: 'red',
			icon: '🏔️',
			features: ['Significant travel', 'Limited agents', 'Challenging access', 'Premium compensation']
		}
	];

	const serviceTypes = [
		{
			name: 'Standard Service',
			multiplier: '1x',
			description: 'Regular processing within business hours',
			timeframe: '2-4 hours',
			color: 'green'
		},
		{
			name: 'Express Service',
			multiplier: '+30%',
			description: 'Priority processing for urgent needs',
			timeframe: '30-60 minutes',
			color: 'yellow'
		},
		{
			name: 'Emergency Service',
			multiplier: '+80%',
			description: 'Immediate processing for critical situations',
			timeframe: '15-30 minutes',
			color: 'red'
		}
	];

	const timeFactors = [
		{ name: 'Business Hours', time: '8 AM - 6 PM', multiplier: '1x', color: 'green' },
		{ name: 'Evening', time: '6 PM - 10 PM', multiplier: '+20%', color: 'yellow' },
		{ name: 'Night', time: '10 PM - 6 AM', multiplier: '+40%', color: 'red' },
		{ name: 'Weekend', time: 'Saturday & Sunday', multiplier: '+15%', color: 'blue' }
	];

	function getColorClasses(color: string) {
		const colors: Record<string, string> = {
			green: 'text-neutral-700 bg-neutral-50 border-neutral-200',
			blue: 'text-neutral-700 bg-neutral-50 border-neutral-200',
			yellow: 'text-neutral-700 bg-neutral-50 border-neutral-200',
			red: 'text-neutral-700 bg-neutral-50 border-neutral-200'
		};
		return colors[color] || colors.green;
	}
</script>

<svelte:head>
	<title>Pricing - AfriTokeni</title>
	<meta name="description" content="Transparent, location-based pricing across Africa. Fair compensation for agents." />
</svelte:head>

<div class="min-h-screen bg-gray-50">
	<!-- Hero Section with Gradient -->
	<div class="bg-linear-to-br from-purple-600 to-indigo-600 text-white py-12 sm:py-16 lg:py-20">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 text-center">
			<div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-sm text-white px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold mb-4 sm:mb-6">
				<Calculator class="w-3 h-3 sm:w-4 sm:h-4" />
				Smart Pricing
			</div>
			<h1 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-4 sm:mb-6">Transparent Pricing System</h1>
			<p class="text-lg sm:text-xl lg:text-2xl opacity-90 max-w-3xl mx-auto">
				Fair compensation for agents with transparent, location-based pricing across Africa
			</p>
		</div>
	</div>

	<div class="max-w-4xl mx-auto px-4 sm:px-6 py-6 sm:py-8 lg:py-12">

		<!-- Navigation -->
		<div class="flex justify-center mb-8 sm:mb-10 lg:mb-12">
			<div class="bg-gray-100 p-1 rounded-xl flex gap-1 w-full max-w-md sm:max-w-lg">
				{#each [
					{ id: 'overview', label: 'Overview', icon: Info },
					{ id: 'calculator', label: 'Calculator', icon: Calculator },
					{ id: 'examples', label: 'Examples', icon: TrendingUp }
				] as { id, label, icon }}
					<button
						onclick={() => activeSection = id as 'overview' | 'calculator' | 'examples'}
						class="flex-1 px-3 sm:px-6 py-2 sm:py-3 rounded-lg font-semibold transition-all duration-200 flex items-center justify-center gap-1 sm:gap-2 text-xs sm:text-base {activeSection === id ? 'bg-white text-gray-900 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
					>
						<svelte:component this={icon} class="w-3 h-3 sm:w-4 sm:h-4" />
						<span class="hidden sm:inline">{label}</span>
						<span class="sm:hidden">{label.slice(0, 3)}</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Overview Section -->
		{#if activeSection === 'overview'}
			<div class="space-y-8 sm:space-y-10 lg:space-y-12">
				<!-- Key Benefits -->
				<div class="bg-white border border-gray-200 p-4 sm:p-6 lg:p-8 rounded-2xl shadow-sm">
					<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold text-gray-900 mb-6 sm:mb-8 text-center">Why Smart Pricing?</h2>
					<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 sm:gap-8">
						<div class="text-center">
							<div class="w-16 h-16 bg-blue-100 rounded-xl flex items-center justify-center mx-auto mb-4">
								<Shield class="w-8 h-8 text-blue-600" />
							</div>
							<h3 class="text-xl font-bold text-gray-900 mb-3">Fair Compensation</h3>
							<p class="text-gray-600">Agents traveling to remote areas receive higher fees for their extra effort and costs.</p>
						</div>
						<div class="text-center">
							<div class="w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center mx-auto mb-4">
								<Users class="w-8 h-8 text-green-600" />
							</div>
							<h3 class="text-xl font-bold text-gray-900 mb-3">Better Coverage</h3>
							<p class="text-gray-600">Incentivizes agents to serve underbanked communities in remote villages</p>
						</div>
						<div class="text-center">
							<div class="w-16 h-16 bg-purple-100 rounded-xl flex items-center justify-center mx-auto mb-4">
								<CircleCheckBig class="w-8 h-8 text-purple-600" />
							</div>
							<h3 class="text-xl font-bold text-gray-900 mb-3">Transparent Pricing</h3>
							<p class="text-gray-600">Clear, predictable fees based on distance, location, and service level</p>
						</div>
					</div>
				</div>

				<!-- Location-Based Pricing -->
				<div>
					<h2 class="text-lg sm:text-xl lg:text-2xl font-bold text-neutral-900 mb-4 sm:mb-6 flex items-center space-x-2">
						<MapPin class="w-5 h-5 sm:w-6 sm:h-6 text-neutral-700" />
						<span>Location-Based Pricing</span>
					</h2>
					<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
						{#each locationTypes as location}
							<div class="border p-6 rounded-xl {getColorClasses(location.color)}">
								<div class="text-center mb-4">
									<div class="text-4xl mb-2">{location.icon}</div>
									<h3 class="text-lg font-bold">{location.type}</h3>
									<p class="text-sm opacity-75 mb-3">{location.description}</p>
									<div class="text-2xl font-bold">{location.feeRange}</div>
									<div class="text-sm opacity-75">commission</div>
								</div>
								<ul class="space-y-1 text-sm">
									{#each location.features as feature}
										<li class="flex items-center space-x-2">
											<CircleCheckBig class="w-3 h-3 shrink-0" />
											<span>{feature}</span>
										</li>
									{/each}
								</ul>
							</div>
						{/each}
					</div>
				</div>

				<!-- Service Level Pricing -->
				<div>
					<h2 class="text-lg sm:text-xl lg:text-2xl font-bold text-neutral-900 mb-4 sm:mb-6 flex items-center space-x-2">
						<Zap class="w-5 h-5 sm:w-6 sm:h-6 text-neutral-700" />
						<span>Service Level Pricing</span>
					</h2>
					<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
						{#each serviceTypes as service}
							<div class="border p-6 rounded-xl {getColorClasses(service.color)}">
								<div class="text-center">
									<h3 class="text-lg font-bold mb-2">{service.name}</h3>
									<div class="text-2xl font-bold mb-2">{service.multiplier}</div>
									<p class="text-sm opacity-75 mb-3">{service.description}</p>
									<div class="text-sm font-semibold">Processing: {service.timeframe}</div>
								</div>
							</div>
						{/each}
					</div>
				</div>

				<!-- Time-Based Adjustments -->
				<div>
					<h2 class="text-lg sm:text-xl lg:text-2xl font-bold text-neutral-900 mb-4 sm:mb-6 flex items-center space-x-2">
						<Clock class="w-5 h-5 sm:w-6 sm:h-6 text-neutral-700" />
						<span>Time-Based Adjustments</span>
					</h2>
					<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
						{#each timeFactors as factor}
							<div class="border p-4 rounded-lg {getColorClasses(factor.color)}">
								<div class="text-center">
									<h4 class="font-semibold mb-1">{factor.name}</h4>
									<p class="text-sm opacity-75 mb-2">{factor.time}</p>
									<div class="text-lg font-bold">{factor.multiplier}</div>
								</div>
							</div>
						{/each}
					</div>
				</div>

				<!-- How It Works -->
				<div class="bg-neutral-50 border border-neutral-200 p-4 sm:p-6 lg:p-8 rounded-xl">
					<h2 class="text-lg sm:text-xl lg:text-2xl font-bold text-neutral-900 mb-4 sm:mb-6 text-center">How It Works</h2>
					<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
						<div class="text-center">
							<div class="w-12 h-12 bg-neutral-900 text-white rounded-full flex items-center justify-center mx-auto mb-4 font-bold text-lg">1</div>
							<h3 class="font-semibold mb-2">Base Fee</h3>
							<p class="text-sm text-neutral-600">Platform starts with 1.5% base fee</p>
						</div>
						<div class="text-center">
							<div class="w-12 h-12 bg-neutral-700 text-white rounded-full flex items-center justify-center mx-auto mb-4 font-bold text-lg">2</div>
							<h3 class="font-semibold mb-2">Distance</h3>
							<p class="text-sm text-neutral-600">Add 0.5-5% based on kilometers</p>
						</div>
						<div class="text-center">
							<div class="w-12 h-12 bg-neutral-500 text-white rounded-full flex items-center justify-center mx-auto mb-4 font-bold text-lg">3</div>
							<h3 class="font-semibold mb-2">Location</h3>
							<p class="text-sm text-neutral-600">Apply accessibility multiplier</p>
						</div>
						<div class="text-center">
							<div class="w-12 h-12 bg-neutral-400 text-white rounded-full flex items-center justify-center mx-auto mb-4 font-bold text-lg">4</div>
							<h3 class="font-semibold mb-2">Adjustments</h3>
							<p class="text-sm text-neutral-600">Add time and urgency factors</p>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Calculator Section -->
		{#if activeSection === 'calculator'}
			<div class="space-y-4 sm:space-y-6">
				<div class="text-center mb-6 sm:mb-8">
					<h2 class="text-lg sm:text-xl lg:text-2xl font-bold text-neutral-900 mb-3 sm:mb-4">Fee Calculator</h2>
					<p class="text-sm sm:text-base text-neutral-600">
						Calculator component will be added here (requires DynamicFeeCalculator component migration)
					</p>
				</div>
			</div>
		{/if}

		<!-- Examples Section -->
		{#if activeSection === 'examples'}
			<div class="space-y-6 sm:space-y-8">
				<div class="text-center mb-6 sm:mb-8">
					<h2 class="text-lg sm:text-xl lg:text-2xl font-bold text-neutral-900 mb-3 sm:mb-4">Real-World Examples</h2>
					<p class="text-sm sm:text-base text-neutral-600">
						See how our smart pricing works in different scenarios across Africa.
					</p>
				</div>

				<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 sm:gap-8">
					<!-- Example 1: Urban Quick -->
					<div class="bg-neutral-50 border border-neutral-200 p-6 rounded-xl">
						<div class="flex items-center space-x-3 mb-4">
							<div class="w-10 h-10 bg-neutral-700 text-white rounded-full flex items-center justify-center">
								<TrendingDown class="w-5 h-5" />
							</div>
							<div>
								<h3 class="text-lg font-bold text-neutral-900">Urban Quick</h3>
								<p class="text-neutral-600 text-sm">Kampala City Center</p>
							</div>
						</div>
						<div class="space-y-3">
							<div class="flex justify-between">
								<span class="text-neutral-600">Transaction:</span>
								<span class="font-semibold">NGN 50,000 → Bitcoin</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Distance:</span>
								<span class="font-semibold">3km</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Service:</span>
								<span class="font-semibold">Quick</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Time:</span>
								<span class="font-semibold">Business hours</span>
							</div>
							<hr class="border-neutral-200" />
							<div class="flex justify-between text-lg font-bold text-neutral-900">
								<span>Total Fee:</span>
								<span>2.8% (NGN 1,400)</span>
							</div>
						</div>
					</div>

					<!-- Example 2: Rural Express -->
					<div class="bg-neutral-50 border border-neutral-200 p-6 rounded-xl">
						<div class="flex items-center space-x-3 mb-4">
							<div class="w-10 h-10 bg-neutral-600 text-white rounded-full flex items-center justify-center">
								<Zap class="w-5 h-5" />
							</div>
							<div>
								<h3 class="text-lg font-bold text-neutral-900">Rural Express</h3>
								<p class="text-neutral-600 text-sm">Gulu District</p>
							</div>
						</div>
						<div class="space-y-3">
							<div class="flex justify-between">
								<span class="text-neutral-600">Transaction:</span>
								<span class="font-semibold">KES 10,000 → Bitcoin</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Distance:</span>
								<span class="font-semibold">25km</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Service:</span>
								<span class="font-semibold">Express</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Time:</span>
								<span class="font-semibold">Weekend</span>
							</div>
							<hr class="border-neutral-200" />
							<div class="flex justify-between text-lg font-bold text-neutral-900">
								<span>Total Fee:</span>
								<span>6.2% (KES 620)</span>
							</div>
						</div>
					</div>

					<!-- Example 3: Remote Emergency -->
					<div class="bg-neutral-50 border border-neutral-200 p-6 rounded-xl">
						<div class="flex items-center space-x-3 mb-4">
							<div class="w-10 h-10 bg-neutral-500 text-white rounded-full flex items-center justify-center">
								<TrendingUp class="w-5 h-5" />
							</div>
							<div>
								<h3 class="text-lg font-bold text-neutral-900">Remote Emergency</h3>
								<p class="text-neutral-600 text-sm">Mountain Village, Karamoja</p>
							</div>
						</div>
						<div class="space-y-3">
							<div class="flex justify-between">
								<span class="text-neutral-600">Transaction:</span>
								<span class="font-semibold">GHS 500 → Bitcoin</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Distance:</span>
								<span class="font-semibold">80km</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Service:</span>
								<span class="font-semibold">Emergency</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Time:</span>
								<span class="font-semibold">Night</span>
							</div>
							<hr class="border-neutral-200" />
							<div class="flex justify-between text-lg font-bold text-neutral-900">
								<span>Total Fee:</span>
								<span>11.8% (GHS 59)</span>
							</div>
						</div>
					</div>

					<!-- Example 4: Suburban Standard -->
					<div class="bg-neutral-50 border border-neutral-200 p-6 rounded-xl">
						<div class="flex items-center space-x-3 mb-4">
							<div class="w-10 h-10 bg-neutral-400 text-white rounded-full flex items-center justify-center">
								<Bitcoin class="w-5 h-5" />
							</div>
							<div>
								<h3 class="text-lg font-bold text-neutral-900">Suburban Standard</h3>
								<p class="text-neutral-600 text-sm">Entebbe Outskirts</p>
							</div>
						</div>
						<div class="space-y-3">
							<div class="flex justify-between">
								<span class="text-neutral-600">Transaction:</span>
								<span class="font-semibold">Bitcoin → ZAR 1,000</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Distance:</span>
								<span class="font-semibold">12km</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Service:</span>
								<span class="font-semibold">Standard</span>
							</div>
							<div class="flex justify-between">
								<span class="text-neutral-600">Time:</span>
								<span class="font-semibold">Evening</span>
							</div>
							<hr class="border-neutral-200" />
							<div class="flex justify-between text-lg font-bold text-neutral-900">
								<span>Total Fee:</span>
								<span>4.1% (ZAR 41)</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Call to Action -->
		<div class="mt-8 sm:mt-10 lg:mt-12 bg-gradient-to-r from-purple-600 to-pink-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
			<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Ready to Get Started?</h2>
			<p class="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
				Join our network of agents or start exchanging Bitcoin with transparent, fair pricing
			</p>
			<div class="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
				<a
					href="/become-agent"
					class="bg-white text-purple-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
				>
					Become an Agent
				</a>
				<a
					href="/bitcoin-exchange"
					class="bg-white text-purple-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
				>
					Start Exchange
				</a>
			</div>
		</div>
	</div>
</div>
