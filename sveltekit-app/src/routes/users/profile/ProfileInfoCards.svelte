<script lang="ts">
	import { Shield, Key, Phone, Calendar } from '@lucide/svelte';

	interface Props {
		userData: any;
		onStartKYC?: () => void;
	}

	let { userData, onStartKYC }: Props = $props();
</script>

<!-- Profile Info Cards Grid -->
<div class="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4 mb-5 sm:mb-6">
	<!-- Principal ID Card -->
	<div class="bg-gray-50 rounded-lg p-3 sm:p-4">
		<div class="flex items-center space-x-1.5 sm:space-x-2 mb-2">
			<Key class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-600 shrink-0" />
			<span class="text-xs sm:text-sm font-medium text-gray-700">Principal ID</span>
		</div>
		<p class="text-xs font-mono text-gray-900 break-all">
			{userData.principalId || 'Not available'}
		</p>
	</div>

	<!-- KYC Status Card -->
	<div class="bg-gray-50 rounded-lg p-3 sm:p-4">
		<div class="flex items-center justify-between mb-2">
			<div class="flex items-center space-x-1.5 sm:space-x-2">
				<Shield class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-600 shrink-0" />
				<span class="text-xs sm:text-sm font-medium text-gray-700">KYC Status</span>
			</div>
			{#if userData.kycStatus === 'not_started' || userData.kycStatus === 'rejected'}
				<button 
					onclick={onStartKYC}
					class="text-xs px-2 py-1 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors shrink-0"
				>
					{userData.kycStatus === 'rejected' ? 'Retry KYC' : 'Start KYC'}
				</button>
			{/if}
		</div>
		<div class="flex items-center space-x-1.5 sm:space-x-2">
			<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 rounded-full shrink-0 {
				userData.kycStatus === 'approved' ? 'bg-green-500' :
				userData.kycStatus === 'pending' ? 'bg-yellow-500' :
				userData.kycStatus === 'rejected' ? 'bg-red-500' : 'bg-gray-400'
			}"></div>
			<span class="text-xs sm:text-sm font-medium capitalize text-gray-900">
				{userData.kycStatus.replace('_', ' ')}
			</span>
		</div>
		{#if userData.kycStatus === 'not_started'}
			<p class="text-xs text-gray-500 mt-1">Complete KYC verification to unlock full features</p>
		{:else if userData.kycStatus === 'pending'}
			<p class="text-xs text-yellow-600 mt-1">Your documents are being reviewed (24-48 hours)</p>
		{:else if userData.kycStatus === 'rejected'}
			<p class="text-xs text-red-600 mt-1">KYC was rejected. Please submit new documents.</p>
		{/if}
	</div>

	<!-- Phone -->
	<div class="bg-gray-50 rounded-lg p-3 sm:p-4">
		<div class="flex items-center space-x-1.5 sm:space-x-2 mb-2">
			<Phone class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-600 shrink-0" />
			<span class="text-xs sm:text-sm font-medium text-gray-700">Phone</span>
		</div>
		<p class="text-xs sm:text-sm font-mono text-gray-900 break-all">{userData.phone}</p>
	</div>

	<!-- Member Since -->
	<div class="bg-gray-50 rounded-lg p-3 sm:p-4">
		<div class="flex items-center space-x-1.5 sm:space-x-2 mb-2">
			<Calendar class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-600 shrink-0" />
			<span class="text-xs sm:text-sm font-medium text-gray-700">Member Since</span>
		</div>
		<p class="text-xs sm:text-sm text-gray-900">
			{userData.joinDate.toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			})}
		</p>
	</div>
</div>
