<script lang="ts">
	import { Camera, Edit3, Check } from '@lucide/svelte';

	interface Props {
		userData: any;
		onToggleEdit: () => void;
		onProfilePictureUpload: (event: Event) => void;
	}

	let { userData, onToggleEdit, onProfilePictureUpload }: Props = $props();
</script>

<!-- Centered Profile Header -->
<div class="flex flex-col items-center text-center mb-5 sm:mb-6">
	<!-- Avatar -->
	<div class="relative mb-3 sm:mb-4">
		{#if userData.profileImage}
			<img 
				src={userData.profileImage} 
				alt="{userData.firstName} {userData.lastName}"
				class="w-20 h-20 sm:w-24 sm:h-24 md:w-28 md:h-28 lg:w-32 lg:h-32 rounded-full object-cover border-4 border-gray-200"
			/>
		{:else}
			<div class="w-20 h-20 sm:w-24 sm:h-24 md:w-28 md:h-28 lg:w-32 lg:h-32 rounded-full bg-gray-900 flex items-center justify-center text-white text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold">
				{userData.firstName.charAt(0)}
			</div>
		{/if}
		<label class="absolute bottom-0 right-0 w-7 h-7 sm:w-8 sm:h-8 bg-gray-900 rounded-full flex items-center justify-center cursor-pointer hover:bg-gray-800 transition-colors">
			<Camera class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-white shrink-0" />
			<input 
				type="file" 
				accept="image/*" 
				onchange={onProfilePictureUpload}
				class="hidden" 
			/>
		</label>
	</div>

	<!-- Name & Edit Button -->
	<div class="flex-1 min-w-0 w-full">
		<div class="flex items-center justify-center space-x-2 mb-2">
			<h2 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold text-gray-900 wrap-break-word">{userData.firstName} {userData.lastName}</h2>
			<button 
				onclick={onToggleEdit}
				class="p-1 sm:p-1.5 hover:bg-gray-100 rounded-lg transition-colors shrink-0"
				title="Edit Profile"
			>
				<Edit3 class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-500 shrink-0" />
			</button>
		</div>

		<!-- Location -->
		<p class="text-gray-600 text-xs sm:text-sm md:text-base lg:text-lg mb-1.5 sm:mb-2 wrap-break-word">
			{userData.location ? `${userData.location.city}, ${userData.location.country}` : 'Location not set'}
		</p>

		<!-- Verification Status -->
		<div class="flex items-center justify-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
			{#if userData.kycStatus === 'approved'}
				<Check class="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 text-green-500 shrink-0" />
				<span class="text-xs sm:text-sm font-medium text-green-600">Verified Account</span>
			{:else if userData.kycStatus === 'pending'}
				<div class="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 border-2 border-yellow-500 border-t-transparent rounded-full animate-spin shrink-0"></div>
				<span class="text-xs sm:text-sm font-medium text-yellow-600">Pending Verification</span>
			{:else}
				<div class="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 rounded-full bg-gray-400 shrink-0"></div>
				<span class="text-xs sm:text-sm font-medium text-gray-600">Not Verified</span>
			{/if}
		</div>

		<!-- Auth Method Badge -->
		<div class="inline-flex items-center px-2 sm:px-2.5 py-0.5 sm:py-1 rounded-full text-xs sm:text-xs md:text-sm font-medium bg-blue-100 text-blue-800">
			{userData.authMethod === 'sms' ? '📱 SMS User' : '🌐 Web User'}
		</div>
	</div>
</div>
