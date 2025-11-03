<script lang="ts">
	import { Smartphone, Zap, Lock, Bitcoin, Globe } from '@lucide/svelte';
	import SavingsComparisonTable from '$lib/components/landing/SavingsComparisonTable.svelte';
	import SignInButton from '$lib/components/auth/SignInButton.svelte';
</script>

<!-- Hero Section -->
<section class="bg-white">
	<div class="max-w-6xl mx-auto px-4 sm:px-6 py-8 sm:py-12 md:py-16 lg:py-20">
		<div class="text-center max-w-4xl mx-auto">
			<div class="inline-flex items-center px-2.5 sm:px-3 py-1 rounded-full bg-gray-100 text-gray-700 text-xs sm:text-sm font-medium mb-4 sm:mb-6 md:mb-8">
				<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-green-500 rounded-full mr-1.5 sm:mr-2"></div>
				<span class="text-xs sm:text-sm">Starting in Africa. Coming to the world.</span>
			</div>
			<h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl xl:text-6xl 2xl:text-7xl font-bold text-gray-900 mb-4 sm:mb-6 tracking-tight leading-tight px-2 sm:px-0">
				Send money across Africa
				<br />
				<span class="text-orange-600">in under 1 second</span>
			</h1>
			<p class="text-sm sm:text-base md:text-lg lg:text-xl text-gray-600 mb-6 sm:mb-8 max-w-6xl mx-auto px-4 sm:px-0 leading-relaxed">
				Banking without internet. Transfers without smartphones. Accessible anywhere, on any phone.
			</p>
			<div class="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center px-4 sm:px-0">
				<SignInButton />
				<a
					href="#savings"
					class="w-full sm:w-auto bg-white text-gray-900 border-2 border-gray-300 px-6 sm:px-8 lg:px-10 py-3 sm:py-4 lg:py-5 rounded-xl text-sm sm:text-base lg:text-lg font-semibold hover:border-gray-400 hover:bg-gray-50 transform hover:scale-105 transition-all duration-200 shadow-lg hover:shadow-xl text-center inline-flex items-center justify-center"
				>
					See How Much You Save
				</a>
			</div>
		</div>
	</div>
</section>

<!-- Savings Comparison Table -->
<SavingsComparisonTable />

<!-- Key Differentiators -->
<section class="bg-gradient-to-b from-gray-50 to-white py-12 sm:py-16 md:py-20 lg:py-24">
	<div class="max-w-6xl mx-auto px-4 sm:px-6">
		<div class="text-center mb-8 sm:mb-12 lg:mb-16">
			<h2 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-3 sm:mb-4 px-4 sm:px-0">
				How We Protect Your Money
			</h2>
			<p class="text-base sm:text-lg lg:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
				Security, speed, and fair pricing. Everything traditional services lack.
			</p>
		</div>

		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 lg:gap-8">
			<!-- Escrow Protection -->
			<a href="/info/escrow" class="bg-linear-to-br from-blue-50 to-cyan-50 rounded-2xl p-6 sm:p-8 border-2 border-blue-200 hover:border-blue-400 transition-all hover:shadow-lg flex flex-col">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-blue-600 rounded-xl flex items-center justify-center mb-4 sm:mb-6">
					<Lock class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
				</div>
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-3 sm:mb-4">Escrow Protected</h3>
				<p class="text-sm sm:text-base text-gray-600 leading-relaxed mb-3 sm:mb-4">
					AfriTokeni holds ckBTC/ckUSDC in escrow. Agents can't disappear with your money. 6-digit codes + 24hr refunds.
				</p>
				<div class="text-xs sm:text-sm font-semibold text-blue-600 mt-auto flex items-center gap-2">
					Learn More →
				</div>
			</a>

			<!-- Lightning Speed -->
			<a href="/info/ckbtc" class="bg-linear-to-br from-yellow-50 to-orange-50 rounded-2xl p-6 sm:p-8 border-2 border-yellow-200 hover:border-orange-400 transition-all hover:shadow-lg flex flex-col">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-orange-600 rounded-xl flex items-center justify-center mb-4 sm:mb-6">
					<Zap class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
				</div>
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-3 sm:mb-4">ckBTC Instant</h3>
				<p class="text-sm sm:text-base text-gray-600 leading-relaxed mb-3 sm:mb-4">
					ICP-native Bitcoin transfers in &lt;1 second. ~$0.01 fees. Instant and secure.
				</p>
				<div class="text-xs sm:text-sm font-semibold text-orange-600 mt-auto flex items-center gap-2">
					Learn More →
				</div>
			</a>

			<!-- ckUSD Stable Value -->
			<a href="/info/ckusd" class="bg-linear-to-br from-green-50 to-emerald-50 rounded-2xl p-6 sm:p-8 border-2 border-green-200 hover:border-green-400 transition-all hover:shadow-lg flex flex-col">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-green-600 rounded-xl flex items-center justify-center mb-4 sm:mb-6">
					<Lock class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
				</div>
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-3 sm:mb-4">ckUSD Stable</h3>
				<p class="text-sm sm:text-base text-gray-600 leading-relaxed mb-3 sm:mb-4">
					Protect yourself from Bitcoin volatility. ckUSD stays pegged to $1 USD. Instant transfers, stable value.
				</p>
				<div class="text-xs sm:text-sm font-semibold text-green-600 mt-auto flex items-center gap-2">
					Learn More →
				</div>
			</a>

			<!-- DAO Governance -->
			<a href="/info/dao" class="bg-linear-to-br from-purple-50 to-pink-50 rounded-2xl p-6 sm:p-8 border-2 border-purple-200 hover:border-purple-400 transition-all hover:shadow-lg flex flex-col sm:col-span-2 lg:col-span-1">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-purple-600 rounded-xl flex items-center justify-center mb-4 sm:mb-6">
					<Globe class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
				</div>
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-3 sm:mb-4">DAO Governed</h3>
				<p class="text-sm sm:text-base text-gray-600 leading-relaxed mb-3 sm:mb-4">
					Community-owned. Vote on fees, currencies, and policies via USSD. Agents and users control the platform.
				</p>
				<div class="text-xs sm:text-sm font-semibold text-purple-600 mt-auto flex items-center gap-2">
					Learn More →
				</div>
			</a>
		</div>
	</div>
</section>

<!-- How It Works -->
<section class="bg-gray-50 py-12 sm:py-16 md:py-20 lg:py-24">
	<div class="max-w-6xl mx-auto px-4 sm:px-6">
		<div class="text-center mb-8 sm:mb-12">
			<div class="inline-flex items-center px-3 sm:px-4 py-1.5 sm:py-2 rounded-full bg-yellow-100 border border-yellow-200 text-yellow-800 text-xs sm:text-sm font-medium mb-3 sm:mb-4">
				<Zap class="w-3 h-3 sm:w-4 sm:h-4 mr-1.5 sm:mr-2" />
				ICP-Native Assets
			</div>
			<h2 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-3 sm:mb-4 px-4 sm:px-0">
				ckBTC + ckUSDC Instant Transfers
			</h2>
			<p class="text-base sm:text-lg lg:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
				Send ckBTC (Bitcoin) or ckUSDC (stablecoin) in under 1 second with ~$0.01 fees. Choose speed or stability.
			</p>
		</div>

		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6 lg:gap-8 mb-8 sm:mb-12">
			<!-- Speed -->
			<div class="bg-white rounded-2xl p-6 sm:p-8 shadow-xl border border-gray-100 text-center">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-linear-to-br from-yellow-400 to-orange-500 rounded-2xl flex items-center justify-center mx-auto mb-4 sm:mb-6">
					<Zap class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
				</div>
				<h3 class="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-2 sm:mb-3">
					&lt; 1 Second
				</h3>
				<p class="text-sm sm:text-base text-gray-600 mb-3 sm:mb-4">
					Instant transfers. No waiting for blockchain confirmations.
				</p>
				<div class="text-xs sm:text-sm text-gray-500">
					vs 10-60 min on-chain
				</div>
			</div>

			<!-- Cost -->
			<div class="bg-white rounded-2xl p-6 sm:p-8 shadow-xl border border-gray-100 text-center">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-linear-to-br from-green-400 to-emerald-500 rounded-2xl flex items-center justify-center mx-auto mb-4 sm:mb-6">
					<span class="text-lg sm:text-2xl font-bold text-white">$</span>
				</div>
				<h3 class="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-2 sm:mb-3">
					~$0.01 Fee
				</h3>
				<p class="text-sm sm:text-base text-gray-600 mb-3 sm:mb-4">
					99% cheaper than traditional transfers.
				</p>
				<div class="text-xs sm:text-sm text-gray-500">
					vs $1-50 on-chain
				</div>
			</div>

			<!-- Perfect for Africa -->
			<div class="bg-white rounded-2xl p-6 sm:p-8 shadow-xl border border-gray-100 text-center sm:col-span-2 lg:col-span-1">
				<div class="w-12 h-12 sm:w-16 sm:h-16 bg-linear-to-br from-blue-400 to-indigo-500 rounded-2xl flex items-center justify-center mx-auto mb-4 sm:mb-6">
					<Globe class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
				</div>
				<h3 class="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-2 sm:mb-3">
					Perfect for Daily Use
				</h3>
				<p class="text-sm sm:text-base text-gray-600 mb-3 sm:mb-4">
					Ideal for small payments, remittances, and everyday transactions.
				</p>
				<div class="text-xs sm:text-sm text-gray-500">
					Optimized for &lt;$50
				</div>
			</div>
		</div>

		<!-- SMS Lightning Commands -->
		<div class="bg-white rounded-2xl shadow-2xl border border-gray-200 p-4 sm:p-6 lg:p-8 max-w-4xl mx-auto">
			<div class="text-center mb-4 sm:mb-6">
				<h3 class="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-2">
					ckBTC + ckUSDC via USSD
				</h3>
				<p class="text-sm sm:text-base text-gray-600 px-4 sm:px-0">
					Menu-driven interface - works on any phone, no internet needed
				</p>
			</div>
			
			<div class="bg-gray-900 rounded-xl p-4 sm:p-6 font-mono text-xs sm:text-sm lg:text-base">
				<div class="text-green-400 mb-2">$ Dial USSD menu:</div>
				<div class="text-white mb-4">*229#</div>
				
				<div class="text-yellow-400 mb-2">⚡ Menu appears:</div>
				<div class="text-gray-300 mb-4 space-y-1">
					<div>1. Local Currency (UGX)</div>
					<div>2. Bitcoin (ckBTC)</div>
					<div>3. USDC (ckUSDC)</div>
					<div>4. DAO Governance</div>
					<div>5. Help</div>
				</div>
				
				<div class="text-green-400 mb-2">$ Select option:</div>
				<div class="text-white">2 (Bitcoin)</div>
			</div>

			<div class="mt-4 sm:mt-6 flex flex-wrap items-center justify-center gap-3 sm:gap-4 text-xs sm:text-sm text-gray-600">
				<div class="flex items-center gap-2">
					<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-orange-500 rounded-full"></div>
					<span>ckBTC (Fast)</span>
				</div>
				<div class="flex items-center gap-2">
					<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-green-500 rounded-full"></div>
					<span>ckUSDC (Stable)</span>
				</div>
				<div class="flex items-center gap-2">
					<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-purple-500 rounded-full"></div>
					<span>No Internet</span>
				</div>
			</div>
		</div>
	</div>
</section>

<!-- Bitcoin Banking Section -->
<section class="bg-gray-50 py-10 sm:py-12 md:py-16 lg:py-20">
	<div class="max-w-6xl mx-auto px-4 sm:px-6">
		<div class="text-center mb-8 sm:mb-12 lg:mb-16">
			<div class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-orange-50 border border-orange-100 text-orange-700 text-xs sm:text-sm font-medium mb-2 sm:mb-3 md:mb-4">
				<Bitcoin class="w-3 h-3 sm:w-4 sm:h-4 mr-1.5 sm:mr-2" />
				Dual Asset System
			</div>
			<h2 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold text-gray-900 mb-2 sm:mb-3 md:mb-4 px-4 sm:px-0">
				ckBTC + ckUSDC Banking
			</h2>
			<p class="text-sm sm:text-base md:text-lg lg:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
				ckBTC (ICP Bitcoin - fast) + ckUSDC (stablecoin - no volatility) for 39 African currencies. USSD or web.
			</p>
			
			<!-- Country Flags -->
			<div class="mt-6 sm:mt-8 px-2 sm:px-4 max-w-5xl mx-auto">
				<div class="flex flex-wrap justify-center gap-2 sm:gap-3 mb-2 sm:mb-3">
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇰🇪 Kenya
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇳🇬 Nigeria
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇺🇬 Uganda
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇹🇿 Tanzania
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇷🇼 Rwanda
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇿🇦 South Africa
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇿🇲 Zambia
					</span>
				</div>
				<div class="flex flex-wrap justify-center gap-2 sm:gap-3">
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-white border border-gray-200 text-xs sm:text-sm text-gray-700 shadow-sm">
						🇲🇼 Malawi
					</span>
					<span class="inline-flex items-center px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 rounded-full bg-blue-50 border border-blue-200 text-xs sm:text-sm text-blue-700 font-medium shadow-sm">
						+31 more (USSD + web)
					</span>
				</div>
			</div>
		</div>
		
		<!-- USSD Availability Banner -->
		<div class="max-w-6xl mx-auto mb-4 sm:mb-6">
			<div class="bg-red-50 border-2 border-red-300 rounded-xl p-3 sm:p-4">
				<h3 class="font-bold text-red-900 mb-2 text-xs sm:text-sm md:text-base text-center">🇺🇬 REAL USSD: UGANDA ONLY</h3>
				<p class="text-xs sm:text-sm text-red-800 mb-1 text-center">
					<strong>Uganda:</strong> Dial <code class="bg-red-100 px-1.5 py-0.5 rounded font-mono text-xs">*384*22948#</code> for real USSD!
				</p>
				<p class="text-xs text-red-700 text-center">
					<strong>Other countries:</strong> Coming soon! Use USSD playground for demo.
				</p>
			</div>
		</div>
		
		<!-- USSD/Web Interface Cards -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6 lg:gap-8 max-w-6xl mx-auto items-stretch">
			<!-- USSD Interface Card -->
			<div class="bg-white rounded-2xl shadow-xl border border-gray-200 p-4 sm:p-6 lg:p-8 flex flex-col">
				<div class="space-y-4 sm:space-y-6 flex-1 flex flex-col">
					<div class="text-center">
						<div class="inline-flex items-center px-3 sm:px-4 py-1.5 sm:py-2 rounded-full bg-blue-50 border border-blue-100 text-blue-700 text-xs sm:text-sm font-medium mb-3 sm:mb-4">
							<Smartphone class="w-3 h-3 sm:w-4 sm:h-4 mr-1.5 sm:mr-2" />
							USSD Interface
						</div>
						<h3 class="text-lg sm:text-xl font-semibold text-gray-900 mb-2">Any Phone Works</h3>
						<p class="text-sm sm:text-base text-gray-600">USSD menu - no internet required</p>
					</div>
					
					<div class="space-y-2 sm:space-y-3">
						<div class="flex items-center justify-between">
							<div class="text-xs font-semibold text-gray-500 uppercase tracking-wide">USSD Menu</div>
							<div class="inline-flex items-center px-2 py-1 rounded-full bg-purple-50 border border-purple-200 text-purple-700 text-xs font-medium">
								Dial *384*22948#
							</div>
						</div>
						<div class="grid grid-cols-2 gap-2 sm:gap-3">
							<div class="text-center p-2 sm:p-3 bg-gray-50 rounded-lg border border-gray-200">
								<div class="font-mono text-blue-600 font-medium text-xs sm:text-sm">*384*22948#</div>
								<div class="text-gray-500 text-xs mt-1">Start USSD</div>
							</div>
							<div class="text-center p-2 sm:p-3 bg-gray-50 rounded-lg border border-gray-200">
								<div class="font-mono text-blue-600 font-medium text-xs sm:text-sm">1</div>
								<div class="text-gray-500 text-xs mt-1">Local Currency</div>
							</div>
							<div class="text-center p-2 sm:p-3 bg-orange-50 rounded-lg border border-orange-200">
								<div class="font-mono text-orange-600 font-medium text-xs sm:text-sm">12</div>
								<div class="text-gray-500 text-xs mt-1">Check Balance</div>
							</div>
							<div class="text-center p-2 sm:p-3 bg-green-50 rounded-lg border border-green-200">
								<div class="font-mono text-green-600 font-medium text-xs sm:text-sm">2</div>
								<div class="text-gray-500 text-xs mt-1">Bitcoin (ckBTC)</div>
							</div>
						</div>
					</div>
					
					<div class="text-center pt-4 border-t border-gray-200 mt-auto">
						<a href="/ussd" class="inline-flex items-center text-blue-600 hover:text-blue-700 font-medium text-sm">
							Try USSD Banking →
						</a>
					</div>
				</div>
			</div>

			<!-- Web Interface Card -->
			<div class="bg-white rounded-2xl shadow-xl border border-gray-200 p-4 sm:p-6 lg:p-8 flex flex-col">
				<div class="space-y-4 sm:space-y-6 flex-1 flex flex-col">
					<div class="text-center pt-2">
						<div class="inline-flex items-center px-3 sm:px-4 py-1.5 sm:py-2 rounded-full bg-green-50 border border-green-100 text-green-700 text-xs sm:text-sm font-medium mb-3 sm:mb-4">
							<Globe class="w-3 h-3 sm:w-4 sm:h-4 mr-1.5 sm:mr-2" />
							Web Interface
						</div>
						<h3 class="text-lg sm:text-xl font-semibold text-gray-900 mb-2">Full Dashboard</h3>
						<p class="text-sm sm:text-base text-gray-600">Complete crypto banking: ckBTC + ckUSDC with charts & analytics</p>
					</div>
					
					<div class="grid grid-cols-2 gap-2 sm:gap-3">
						<button onclick={handleICPLogin} class="text-center p-3 sm:p-4 bg-gray-50 hover:bg-gray-100 rounded-xl border border-gray-200 hover:border-green-200 transition-colors group">
							<div class="font-medium text-green-600 text-xs sm:text-sm group-hover:text-green-700">Portfolio</div>
							<div class="text-gray-600 text-xs mt-1">Balance & assets</div>
						</button>
						<button onclick={handleICPLogin} class="text-center p-3 sm:p-4 bg-gray-50 hover:bg-gray-100 rounded-xl border border-gray-200 hover:border-green-200 transition-colors group">
							<div class="font-medium text-green-600 text-xs sm:text-sm group-hover:text-green-700">Exchange</div>
							<div class="text-gray-600 text-xs mt-1">Live trading</div>
						</button>
						<button onclick={handleICPLogin} class="text-center p-3 sm:p-4 bg-gray-50 hover:bg-gray-100 rounded-xl border border-gray-200 hover:border-green-200 transition-colors group">
							<div class="font-medium text-green-600 text-xs sm:text-sm group-hover:text-green-700">Agents</div>
							<div class="text-gray-600 text-xs mt-1">Find nearby</div>
						</button>
						<button onclick={handleICPLogin} class="text-center p-3 sm:p-4 bg-gray-50 hover:bg-gray-100 rounded-xl border border-gray-200 hover:border-green-200 transition-colors group">
							<div class="font-medium text-green-600 text-xs sm:text-sm group-hover:text-green-700">History</div>
							<div class="text-gray-600 text-xs mt-1">All transactions</div>
						</button>
					</div>
					
					<div class="text-center pt-4 border-t border-gray-200 mt-auto">
						<button onclick={handleICPLogin} class="inline-flex items-center text-green-600 hover:text-green-700 font-medium text-sm">
							Start Web Banking →
						</button>
					</div>
				</div>
			</div>
		</div>
		
		<!-- Additional Info -->
		<div class="text-center mt-6 sm:mt-8 lg:mt-12">
			<div class="flex flex-wrap justify-center items-center gap-4 sm:gap-6 text-xs sm:text-sm text-gray-600">
				<div class="flex items-center">
					<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-green-500 rounded-full mr-1.5 sm:mr-2"></div>
					<span>39 African currencies supported</span>
				</div>
				<div class="flex items-center">
					<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-orange-500 rounded-full mr-1.5 sm:mr-2"></div>
					<a href="/tariff" class="text-orange-600 hover:text-orange-700 font-medium">
						Smart pricing system →
					</a>
				</div>
			</div>
		</div>
	</div>
</section>
