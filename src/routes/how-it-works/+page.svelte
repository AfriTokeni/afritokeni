<script lang="ts">
	import { Bitcoin, Shield, Users, CheckCircle } from '@lucide/svelte';
</script>

<svelte:head>
	<title>How It Works - AfriTokeni</title>
	<meta name="description" content="Learn how AfriTokeni works - instant Bitcoin transfers via USSD on any phone" />
</svelte:head>

<div class="min-h-screen bg-gray-50">
	<!-- Hero Section with Gradient -->
	<div class="bg-linear-to-br from-orange-600 to-yellow-600 text-white py-12 sm:py-16 lg:py-20">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 text-center">
			<div class="inline-flex items-center px-3 sm:px-4 py-2 rounded-full bg-white/20 backdrop-blur-sm text-white text-xs sm:text-sm font-medium mb-4 sm:mb-6">
				<Bitcoin class="w-3 h-3 sm:w-4 sm:h-4 mr-2" />
				Bitcoin Exchange
			</div>
			<h1 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-4 sm:mb-6">
				How AfriTokeni Works
			</h1>
			<p class="text-lg sm:text-xl lg:text-2xl opacity-90 max-w-3xl mx-auto">
				Send money instantly across Africa using Bitcoin. No internet needed - works via USSD on any phone.
			</p>
		</div>
	</div>

	<div class="max-w-4xl mx-auto space-y-8 sm:space-y-10 lg:space-y-12 py-8 sm:py-10 lg:py-12 px-4 sm:px-6">
		<!-- How It Works -->
		<div class="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 lg:p-8 shadow-sm">
			<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900 mb-6 sm:mb-8 text-center">Simple 4-Step Process</h2>
			
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-orange-100 rounded-xl flex items-center justify-center mx-auto">
						<Bitcoin class="w-8 h-8 text-orange-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">1. Choose Amount</h3>
						<p class="text-sm text-neutral-600">Enter how much Bitcoin you want to exchange for local currency</p>
					</div>
				</div>

				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-blue-100 rounded-xl flex items-center justify-center mx-auto">
						<Users class="w-8 h-8 text-blue-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">2. Select Agent</h3>
						<p class="text-sm text-neutral-600">Choose a trusted local agent based on rating, location, and fees</p>
					</div>
				</div>

				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center mx-auto">
						<Shield class="w-8 h-8 text-green-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">3. Secure Escrow</h3>
						<p class="text-sm text-neutral-600">Send Bitcoin to our escrow address and receive exchange code</p>
					</div>
				</div>

				<div class="text-center space-y-4">
					<div class="w-16 h-16 bg-purple-100 rounded-xl flex items-center justify-center mx-auto">
						<CheckCircle class="w-8 h-8 text-purple-600" />
					</div>
					<div>
						<h3 class="font-semibold text-neutral-900 mb-2">4. Get Cash</h3>
						<p class="text-sm text-neutral-600">Meet agent, show exchange code, receive cash instantly</p>
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
					<h3 class="text-xl font-bold text-gray-900 mb-3">Secure Escrow</h3>
					<p class="text-gray-600">Your Bitcoin is protected until you receive cash</p>
				</div>
				<div class="text-center">
					<div class="w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center mx-auto mb-4">
						<Users class="w-8 h-8 text-green-600" />
					</div>
					<h3 class="text-xl font-bold text-gray-900 mb-3">Trusted Agents</h3>
					<p class="text-gray-600">Verified agents with ratings and reviews</p>
				</div>
				<div class="text-center">
					<div class="w-16 h-16 bg-purple-100 rounded-xl flex items-center justify-center mx-auto mb-4">
						<CheckCircle class="w-8 h-8 text-purple-600" />
					</div>
					<h3 class="text-xl font-bold text-gray-900 mb-3">USSD Access</h3>
					<p class="text-gray-600">Works on any phone via SMS - no internet needed</p>
				</div>
			</div>
		</div>

		<!-- CTA Section -->
		<div class="bg-gradient-to-r from-orange-600 to-yellow-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
			<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Ready to Get Started?</h2>
			<p class="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
				Join thousands of users safely converting Bitcoin to cash across Africa
			</p>
			<div class="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
				<a
					href="/ussd"
					class="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
				>
					Try USSD Demo
				</a>
				<a
					href="/become-agent"
					class="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
				>
					Become an Agent
				</a>
			</div>
		</div>
	</div>
</div>
