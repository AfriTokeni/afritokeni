<script lang="ts">
	import Header from '$lib/components/layout/Header.svelte';
	import Footer from '$lib/components/layout/Footer.svelte';
	import { Bitcoin, Shield, Users, CheckCircle, Coins, Smartphone } from '@lucide/svelte';
</script>

<svelte:head>
	<title>How It Works - AfriTokeni</title>
	<meta name="description" content="Learn how AfriTokeni works - Bitcoin and stablecoin banking via ICP blockchain" />
</svelte:head>

<Header />

<div class="min-h-screen bg-gray-50">
	<!-- Hero Section with Gradient -->
	<div class="bg-linear-to-br from-orange-600 to-yellow-600 text-white py-12 sm:py-16 lg:py-20">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 text-center">
			<div class="inline-flex items-center px-3 sm:px-4 py-2 rounded-full bg-white/20 backdrop-blur-sm text-white text-xs sm:text-sm font-medium mb-4 sm:mb-6">
				<Bitcoin class="w-3 h-3 sm:w-4 sm:h-4 mr-2" />
				ICP Blockchain Banking
			</div>
			<h1 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-4 sm:mb-6">
				How AfriTokeni Works
			</h1>
			<p class="text-lg sm:text-xl lg:text-2xl opacity-90 max-w-3xl mx-auto">
				Bitcoin and stablecoin banking on ICP blockchain. Access via web or SMS - works on any phone.
			</p>
		</div>
	</div>

	<div class="max-w-4xl mx-auto space-y-8 sm:space-y-10 lg:space-y-12 py-8 sm:py-10 lg:py-12 px-4 sm:px-6">
		<!-- How It Works -->
		<div class="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 lg:p-8 shadow-sm">
			<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900 mb-6 sm:mb-8 text-center">How It Works</h2>
			
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-orange-100 rounded-xl flex items-center justify-center mx-auto">
						<Smartphone class="w-8 h-8 text-orange-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">1. Sign Up</h3>
						<p class="text-sm text-neutral-600">Create account via web (Internet Identity) or SMS (phone number)</p>
					</div>
				</div>

				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-blue-100 rounded-xl flex items-center justify-center mx-auto">
						<Bitcoin class="w-8 h-8 text-blue-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">2. Hold ckBTC/ckUSD</h3>
						<p class="text-sm text-neutral-600">Your Bitcoin and stablecoins stored on ICP blockchain</p>
					</div>
				</div>

				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center mx-auto">
						<Users class="w-8 h-8 text-green-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">3. Find Agent</h3>
						<p class="text-sm text-neutral-600">Connect with local agents for cash deposits/withdrawals</p>
					</div>
				</div>

				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-purple-100 rounded-xl flex items-center justify-center mx-auto">
						<CheckCircle class="w-8 h-8 text-purple-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">4. Exchange</h3>
						<p class="text-sm text-neutral-600">Convert between ckBTC, ckUSD, and local African currencies</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Key Features -->
		<div class="bg-white border border-gray-200 p-4 sm:p-6 lg:p-8 rounded-2xl shadow-sm">
			<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold text-gray-900 mb-6 sm:mb-8 text-center">Why AfriTokeni?</h2>
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 sm:gap-8">
				<div class="text-center">
					<div class="w-16 h-16 bg-blue-100 rounded-xl flex items-center justify-center mx-auto mb-4">
						<Shield class="w-8 h-8 text-blue-600" />
					</div>
					<h3 class="text-xl font-bold text-gray-900 mb-3">ICP Blockchain</h3>
					<p class="text-gray-600">Secure, decentralized storage of ckBTC and ckUSD on Internet Computer</p>
				</div>
				<div class="text-center">
					<div class="w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center mx-auto mb-4">
						<Coins class="w-8 h-8 text-green-600" />
					</div>
					<h3 class="text-xl font-bold text-gray-900 mb-3">Multi-Currency</h3>
					<p class="text-gray-600">Support for ckBTC, ckUSD, and 39 African currencies</p>
				</div>
				<div class="text-center">
					<div class="w-16 h-16 bg-purple-100 rounded-xl flex items-center justify-center mx-auto mb-4">
						<Smartphone class="w-8 h-8 text-purple-600" />
					</div>
					<h3 class="text-xl font-bold text-gray-900 mb-3">SMS + Web Access</h3>
					<p class="text-gray-600">Full banking via USSD on feature phones or web interface</p>
				</div>
			</div>
		</div>

		<!-- CTA Section -->
		<div class="bg-gradient-to-r from-orange-600 to-yellow-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
			<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Ready to Get Started?</h2>
			<p class="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
				Join the future of African banking with ckBTC and ckUSD on ICP blockchain
			</p>
			<div class="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
				<a
					href="/ussd"
					class="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
				>
					Try USSD Demo
				</a>
				<a
					href="/auth/role-selection"
					class="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
				>
					Sign Up Now
				</a>
			</div>
		</div>
	</div>
</div>

<Footer />
