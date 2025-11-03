<script lang="ts">
	import { Lock, Shield, Clock, CircleCheckBig, AlertTriangle, ArrowRight } from '@lucide/svelte';
</script>

<svelte:head>
	<title>Escrow Protection System - AfriTokeni</title>
	<meta name="description" content="AfriTokeni holds your Bitcoin in secure escrow until both parties confirm the exchange. Zero fraud risk." />
</svelte:head>

<!-- Hero Section -->
<div class="bg-linear-to-br from-blue-600 to-indigo-600 text-white py-16 sm:py-20">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<div class="inline-flex items-center justify-center w-16 h-16 sm:w-20 sm:h-20 bg-white/20 backdrop-blur-sm rounded-2xl mb-6">
			<Lock class="w-8 h-8 sm:w-10 sm:h-10" />
		</div>
		<h1 class="text-3xl sm:text-4xl md:text-5xl font-bold mb-4 sm:mb-6">
			Escrow Protected Transactions
		</h1>
		<p class="text-lg sm:text-xl text-blue-100 max-w-2xl mx-auto">
			AfriTokeni holds your Bitcoin in secure escrow until both parties confirm the exchange. Zero fraud risk.
		</p>
	</div>
</div>

<!-- Problem Section -->
<div class="py-12 sm:py-16 bg-white">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="bg-red-50 border-2 border-red-200 rounded-2xl p-6 sm:p-8 mb-8">
			<div class="flex items-start gap-4">
				<AlertTriangle class="w-6 h-6 sm:w-8 sm:h-8 text-red-600 shrink-0 mt-1" />
				<div>
					<h2 class="text-xl sm:text-2xl font-bold text-gray-900 mb-3">The Problem Without Escrow</h2>
					<ul class="space-y-2 text-gray-700">
						<li class="flex items-start gap-2">
							<span class="text-red-600 font-bold">•</span>
							<span>Agent receives Bitcoin, disappears without giving cash</span>
						</li>
						<li class="flex items-start gap-2">
							<span class="text-red-600 font-bold">•</span>
							<span>User claims they sent Bitcoin but didn't</span>
						</li>
						<li class="flex items-start gap-2">
							<span class="text-red-600 font-bold">•</span>
							<span>No way to reverse fraudulent transactions</span>
						</li>
						<li class="flex items-start gap-2">
							<span class="text-red-600 font-bold">•</span>
							<span>Trust-based system vulnerable to scams</span>
						</li>
					</ul>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- How It Works -->
<div class="py-12 sm:py-16 bg-gray-50">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
		<h2 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 sm:mb-12 text-center">How Escrow Works</h2>
		
		<div class="space-y-6">
			<div class="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
				<div class="flex items-start gap-4">
					<div class="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold shrink-0">
						1
					</div>
					<div>
						<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">User Initiates Exchange</h3>
						<p class="text-gray-600">
							User selects amount and agent. AfriTokeni generates unique escrow address and 6-digit code (e.g., "BTC-847291").
						</p>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
				<div class="flex items-start gap-4">
					<div class="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold shrink-0">
						2
					</div>
					<div>
						<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">Bitcoin Sent to Escrow</h3>
						<p class="text-gray-600">
							User sends Bitcoin to AfriTokeni's escrow address (NOT directly to agent). Funds locked until code verification.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
				<div class="flex items-start gap-4">
					<div class="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold shrink-0">
						3
					</div>
					<div>
						<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">Agent Notified</h3>
						<p class="text-gray-600">
							Once Bitcoin confirmed on blockchain, agent receives SMS notification with transaction details.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
				<div class="flex items-start gap-4">
					<div class="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold shrink-0">
						4
					</div>
					<div>
						<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">In-Person Meeting</h3>
						<p class="text-gray-600">
							User meets agent in person. User shows 6-digit exchange code. Agent verifies ID.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
				<div class="flex items-start gap-4">
					<div class="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold shrink-0">
						5
					</div>
					<div>
						<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">Code Verification</h3>
						<p class="text-gray-600">
							Agent enters 6-digit code via USSD or app. AfriTokeni verifies and releases Bitcoin to agent's wallet.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-xl p-6 sm:p-8 border-2 border-green-200 bg-green-50">
				<div class="flex items-start gap-4">
					<div class="w-10 h-10 sm:w-12 sm:h-12 bg-green-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold shrink-0">
						6
					</div>
					<div>
						<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">Cash Handed Over</h3>
						<p class="text-gray-600">
							Agent gives cash to user. Transaction complete. Both parties protected throughout.
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- Security Features -->
<div class="py-12 sm:py-16 bg-white">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
		<h2 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 sm:mb-12 text-center">Security Features</h2>
		
		<div class="grid sm:grid-cols-2 gap-6">
			<div class="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
				<Shield class="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">6-Digit Codes</h3>
				<p class="text-gray-600 text-sm sm:text-base">
					Unique codes prevent unauthorized Bitcoin release. Only user and agent know the code.
				</p>
			</div>

			<div class="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
				<Clock class="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">24-Hour Refund</h3>
				<p class="text-gray-600 text-sm sm:text-base">
					If code not verified within 24 hours, Bitcoin automatically refunded to user.
				</p>
			</div>

			<div class="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
				<CircleCheckBig class="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">Agent Verification</h3>
				<p class="text-gray-600 text-sm sm:text-base">
					All agents KYC-verified. Rating system tracks performance and trustworthiness.
				</p>
			</div>

			<div class="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
				<Lock class="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
				<h3 class="text-lg sm:text-xl font-bold text-gray-900 mb-2">Blockchain Secured</h3>
				<p class="text-gray-600 text-sm sm:text-base">
					All transactions recorded on Internet Computer blockchain. Immutable and transparent.
				</p>
			</div>
		</div>
	</div>
</div>

<!-- Example Transaction -->
<div class="py-12 sm:py-16 bg-gray-50">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
		<h2 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">Example Transaction</h2>
		
		<div class="bg-white rounded-2xl p-6 sm:p-8 border-2 border-gray-200">
			<div class="space-y-4 text-sm sm:text-base">
				<div class="flex justify-between items-center pb-4 border-b border-gray-200">
					<span class="text-gray-600">User wants to exchange:</span>
					<span class="font-bold text-gray-900">0.01 BTC</span>
				</div>
				<div class="flex justify-between items-center pb-4 border-b border-gray-200">
					<span class="text-gray-600">Current rate:</span>
					<span class="font-bold text-gray-900">1 BTC = 138,500,000 UGX</span>
				</div>
				<div class="flex justify-between items-center pb-4 border-b border-gray-200">
					<span class="text-gray-600">Cash to receive:</span>
					<span class="font-bold text-gray-900">1,385,000 UGX</span>
				</div>
				<div class="flex justify-between items-center pb-4 border-b border-gray-200">
					<span class="text-gray-600">Agent fee (2%):</span>
					<span class="font-bold text-gray-900">27,700 UGX</span>
				</div>
				<div class="flex justify-between items-center pb-4 border-b border-gray-200">
					<span class="text-gray-600">Escrow code:</span>
					<span class="font-mono font-bold text-blue-600">BTC-847291</span>
				</div>
				<div class="flex justify-between items-center pt-2">
					<span class="text-gray-600 font-semibold">User receives:</span>
					<span class="font-bold text-green-600 text-lg sm:text-xl">1,357,300 UGX</span>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- CTA -->
<div class="py-12 sm:py-16 bg-linear-to-br from-blue-600 to-indigo-600 text-white">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<h2 class="text-2xl sm:text-3xl font-bold mb-4 sm:mb-6">
			Start Using Escrow-Protected Exchanges
		</h2>
		<p class="text-lg sm:text-xl text-blue-100 mb-6 sm:mb-8">
			Zero fraud risk. Both parties protected. Instant Bitcoin transfers.
		</p>
		<div class="flex flex-col sm:flex-row gap-4 justify-center">
			<a
				href="/auth/register"
				class="inline-flex items-center justify-center gap-2 bg-white text-blue-600 px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-blue-50 transition-colors"
			>
				Create Account
				<ArrowRight class="w-5 h-5" />
			</a>
			<a
				href="/"
				class="inline-flex items-center justify-center gap-2 bg-white/10 backdrop-blur-sm text-white px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-white/20 transition-colors border-2 border-white/30"
			>
				Learn More
			</a>
		</div>
	</div>
</div>
