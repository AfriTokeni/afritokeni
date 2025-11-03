<script lang="ts">
	import { TrendingUp, Users, DollarSign, MapPin, AlertCircle } from '@lucide/svelte';

	interface Props {
		agentData: any;
		onStartKYC?: () => void;
	}

	let { agentData, onStartKYC }: Props = $props();

	function formatCurrency(amount: number): string {
		if (amount >= 1000000) {
			return `${(amount / 1000000).toFixed(1)}M`;
		}
		if (amount >= 1000) {
			return `${(amount / 1000).toFixed(0)}K`;
		}
		return amount.toString();
	}
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
	<!-- Total Transactions -->
	<div class="bg-white rounded-xl border border-gray-200 p-4">
		<div class="flex items-center justify-between mb-2">
			<div class="p-2 bg-blue-100 rounded-lg">
				<TrendingUp class="w-5 h-5 text-blue-600" />
			</div>
		</div>
		<p class="text-2xl font-bold text-gray-900">{agentData.totalTransactions || 0}</p>
		<p class="text-sm text-gray-600 mt-1">Total Transactions</p>
	</div>

	<!-- Active Customers -->
	<div class="bg-white rounded-xl border border-gray-200 p-4">
		<div class="flex items-center justify-between mb-2">
			<div class="p-2 bg-green-100 rounded-lg">
				<Users class="w-5 h-5 text-green-600" />
			</div>
		</div>
		<p class="text-2xl font-bold text-gray-900">{agentData.activeCustomers || 0}</p>
		<p class="text-sm text-gray-600 mt-1">Active Customers</p>
	</div>

	<!-- Commission Earned -->
	<div class="bg-white rounded-xl border border-gray-200 p-4">
		<div class="flex items-center justify-between mb-2">
			<div class="p-2 bg-purple-100 rounded-lg">
				<DollarSign class="w-5 h-5 text-purple-600" />
			</div>
		</div>
		<p class="text-2xl font-bold text-gray-900">{formatCurrency(agentData.totalEarnings || 0)} UGX</p>
		<p class="text-sm text-gray-600 mt-1">Total Earnings</p>
	</div>

	<!-- Service Radius -->
	<div class="bg-white rounded-xl border border-gray-200 p-4">
		<div class="flex items-center justify-between mb-2">
			<div class="p-2 bg-orange-100 rounded-lg">
				<MapPin class="w-5 h-5 text-orange-600" />
			</div>
		</div>
		<p class="text-2xl font-bold text-gray-900">{agentData.serviceRadius || 5} km</p>
		<p class="text-sm text-gray-600 mt-1">Service Radius</p>
	</div>
</div>

<!-- KYC Status Card (if not verified) -->
{#if agentData.kycStatus !== 'approved' && onStartKYC}
	<div class="bg-yellow-50 border border-yellow-200 rounded-xl p-4 mt-4">
		<div class="flex items-start gap-3">
			<div class="flex-shrink-0">
				<AlertCircle class="w-5 h-5 text-yellow-600" />
			</div>
			<div class="flex-1">
				<h3 class="text-sm font-semibold text-yellow-900 mb-1">
					{agentData.kycStatus === 'pending' ? 'Verification Pending' : 'Complete Agent Verification'}
				</h3>
				<p class="text-sm text-yellow-700 mb-3">
					{agentData.kycStatus === 'pending' 
						? 'Your verification is being reviewed. This usually takes 1-2 business days.'
						: 'Verify your agent account to unlock higher transaction limits and build customer trust.'}
				</p>
				{#if agentData.kycStatus !== 'pending'}
					<button
						onclick={onStartKYC}
						class="inline-flex items-center gap-2 px-4 py-2 bg-yellow-600 text-white text-sm font-medium rounded-lg hover:bg-yellow-700 transition-colors"
					>
						Start Verification
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}
