<script lang="ts">
	import { goto } from '$app/navigation';
	import { principalId } from '$lib/stores/auth';
	import AgentOnboardingModal from '$lib/components/agent/AgentOnboardingModal.svelte';
	import { onMount } from 'svelte';
	import { getDoc } from '@junobuild/core';

	let showOnboarding = $state(true);

	// Check if agent already has profile
	onMount(async () => {
		const currentPrincipalId = $principalId;
		if (currentPrincipalId) {
			const doc = await getDoc({
				collection: 'agents',
				key: currentPrincipalId
			});
			
			// If agent already has profile, redirect to dashboard
			if (doc) {
				goto('/agents/dashboard');
			}
		}
	});

	async function handleOnboardingComplete(data: any) {
		console.log('Agent onboarding complete:', data);
		showOnboarding = false;
		// Redirect to dashboard
		goto('/agents/dashboard');
	}
</script>

<svelte:head>
	<title>Welcome - AfriTokeni Agent</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-purple-50 via-blue-50 to-indigo-50 flex items-center justify-center p-4">
	<div class="max-w-4xl w-full">
		<!-- Welcome Card -->
		<div class="bg-white rounded-3xl shadow-2xl p-8 md:p-12 text-center mb-8">
			<div class="w-24 h-24 bg-gradient-to-br from-purple-600 to-blue-600 rounded-full flex items-center justify-center mx-auto mb-6">
				<svg class="w-12 h-12 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
				</svg>
			</div>
			
			<h1 class="text-4xl md:text-5xl font-bold text-gray-900 mb-4">
				Welcome to AfriTokeni! 🎉
			</h1>
			
			<p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
				You're about to join Africa's leading Bitcoin banking network. Let's get your agent profile set up in just a few minutes.
			</p>

			<!-- Benefits Grid -->
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
				<div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-2xl p-6 border border-green-200">
					<div class="text-4xl mb-3">💰</div>
					<h3 class="font-bold text-gray-900 mb-2">Earn Commissions</h3>
					<p class="text-sm text-gray-600">Get paid for every transaction you process</p>
				</div>
				
				<div class="bg-gradient-to-br from-blue-50 to-cyan-50 rounded-2xl p-6 border border-blue-200">
					<div class="text-4xl mb-3">🌍</div>
					<h3 class="font-bold text-gray-900 mb-2">Serve Your Community</h3>
					<p class="text-sm text-gray-600">Provide Bitcoin banking to the unbanked</p>
				</div>
				
				<div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-2xl p-6 border border-purple-200">
					<div class="text-4xl mb-3">📱</div>
					<h3 class="font-bold text-gray-900 mb-2">Easy to Use</h3>
					<p class="text-sm text-gray-600">Simple dashboard to manage everything</p>
				</div>
			</div>

			<!-- Steps -->
			<div class="bg-gray-50 rounded-2xl p-6 mb-8">
				<h2 class="text-2xl font-bold text-gray-900 mb-6">Quick Setup - 3 Steps</h2>
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					<div class="flex items-start gap-3 text-left">
						<div class="w-8 h-8 bg-purple-600 text-white rounded-full flex items-center justify-center font-bold flex-shrink-0">1</div>
						<div>
							<h4 class="font-semibold text-gray-900">Business Info</h4>
							<p class="text-sm text-gray-600">Name, location, contact details</p>
						</div>
					</div>
					<div class="flex items-start gap-3 text-left">
						<div class="w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold flex-shrink-0">2</div>
						<div>
							<h4 class="font-semibold text-gray-900">Service Details</h4>
							<p class="text-sm text-gray-600">Operating hours, service area</p>
						</div>
					</div>
					<div class="flex items-start gap-3 text-left">
						<div class="w-8 h-8 bg-green-600 text-white rounded-full flex items-center justify-center font-bold flex-shrink-0">3</div>
						<div>
							<h4 class="font-semibold text-gray-900">KYC Verification</h4>
							<p class="text-sm text-gray-600">Upload ID & documents</p>
						</div>
					</div>
				</div>
			</div>

			<button
				onclick={() => showOnboarding = true}
				class="inline-flex items-center gap-3 px-10 py-5 bg-gradient-to-r from-purple-600 to-blue-600 text-white text-xl font-bold rounded-2xl hover:from-purple-700 hover:to-blue-700 transition-all shadow-xl hover:shadow-2xl transform hover:scale-105"
			>
				<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
				</svg>
				Start Setup Now
			</button>
			
			<p class="text-sm text-gray-500 mt-4">
				⏱️ Takes only 5 minutes to complete
			</p>
		</div>

		<!-- Trust Indicators -->
		<div class="text-center text-gray-600">
			<p class="text-sm mb-2">🔒 Secure • 🌍 Trusted by agents across Africa • ⚡ Instant activation</p>
		</div>
	</div>
</div>

<!-- Onboarding Modal -->
<AgentOnboardingModal
	isOpen={showOnboarding}
	onClose={() => goto('/agents/dashboard')}
	onComplete={handleOnboardingComplete}
	currentData={{}}
/>
