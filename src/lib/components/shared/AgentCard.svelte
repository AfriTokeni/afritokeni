<script lang="ts">
	import { MapPin, Clock, Phone as PhoneIcon, Mail } from '@lucide/svelte';
	import type { Agent } from '$lib/utils/agents';
	
	interface Props { 
		agent: Agent; 
		distance?: number;
		expanded?: boolean;
	}
	
	let { agent, distance, expanded = false }: Props = $props();
	let isExpanded = $state(expanded);
</script>

<div class="bg-white rounded-lg border-2 border-gray-200 p-6 hover:shadow-lg transition-all">
	<!-- Header -->
	<div class="flex items-start justify-between mb-4">
		<div class="flex-1">
			<h3 class="font-semibold text-gray-900 text-lg mb-1">{agent.businessName}</h3>
			<div class="flex items-center gap-2">
				<MapPin class="w-4 h-4 text-gray-400 shrink-0" />
				<span class="text-sm text-gray-600">{agent.location.address}</span>
			</div>
		</div>
		<span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium shrink-0 {agent.isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}">
			{agent.isActive ? 'Online' : 'Offline'}
		</span>
	</div>

	<!-- Cash & Commission -->
	<div class="grid grid-cols-2 gap-4 mb-4">
		<div>
			<p class="text-sm text-gray-600 mb-1">Cash Available</p>
			<p class="text-lg font-bold text-gray-900">0 UGX</p>
		</div>
		<div>
			<p class="text-sm text-gray-600 mb-1">Commission</p>
			<p class="text-lg font-bold text-gray-900">{agent.commissionRate}%</p>
		</div>
	</div>

	<!-- Reviews -->
	<p class="text-sm text-gray-600 mb-4">No reviews yet</p>

	<!-- Service Badges -->
	<div class="flex flex-wrap gap-2 mb-4">
		<span class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-medium">Cash Deposit</span>
		<span class="px-3 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">Withdrawal</span>
		<span class="px-3 py-1 bg-orange-100 text-orange-800 rounded-full text-xs font-medium">Bitcoin Exchange</span>
	</div>

	<!-- Contact Button -->
	<button 
		onclick={() => isExpanded = !isExpanded}
		class="w-full py-3 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors font-medium mb-4"
	>
		Contact Agent
	</button>

	<!-- Expanded Details -->
	{#if isExpanded}
		<div class="pt-4 border-t border-gray-200 space-y-4">
			<!-- Operating Hours -->
			<div>
				<h4 class="font-semibold text-gray-900 mb-2">Operating Hours</h4>
				<div class="flex items-center gap-2 text-sm text-gray-600">
					<Clock class="w-4 h-4 shrink-0" />
					<span>Mon-Sat: 8:00 AM - 8:00 PM</span>
				</div>
			</div>

			<!-- Services Available -->
			<div>
				<h4 class="font-semibold text-gray-900 mb-2">Services Available</h4>
				<ul class="space-y-1 text-sm text-gray-600">
					<li>• Cash deposits and withdrawals</li>
					<li>• Bitcoin buying and selling</li>
					<li>• Money transfers</li>
					<li>• Account verification</li>
				</ul>
			</div>

			<!-- Contact Information -->
			<div>
				<h4 class="font-semibold text-gray-900 mb-2">Contact Information</h4>
				<div class="space-y-2 text-sm">
					<p class="text-blue-600 font-medium">+256 700 123 456</p>
					<p class="text-gray-600">agent@afritokeni.com</p>
				</div>
			</div>
		</div>
	{/if}
</div>
