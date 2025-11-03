<script lang="ts">
	import { ChevronDown, ChevronUp, User } from '@lucide/svelte';

	interface Props {
		expanded: boolean;
		onToggle: () => void;
		userData: any;
	}

	let { expanded, onToggle, userData }: Props = $props();

	let form = $state({
		firstName: userData.firstName,
		lastName: userData.lastName,
		email: userData.email,
		phone: userData.phone,
		country: userData.location?.country || '',
		city: userData.location?.city || ''
	});
</script>

<div class="bg-gray-50 rounded-lg overflow-hidden mb-2 sm:mb-3">
	<button
		onclick={onToggle}
		class="w-full flex items-center justify-between px-3 py-3 sm:px-4 sm:py-3.5 md:px-5 md:py-4 hover:bg-gray-100 transition-colors duration-200"
	>
		<div class="flex items-center space-x-2 sm:space-x-3">
			<User class="w-4 h-4 sm:w-5 sm:h-5 md:w-6 md:h-6 text-gray-600 shrink-0" />
			<span class="text-sm sm:text-base md:text-lg text-gray-900">Account Settings</span>
		</div>
		{#if expanded}
			<ChevronUp class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />
		{:else}
			<ChevronDown class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />
		{/if}
	</button>

	{#if expanded}
		<div class="px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5 border-t border-gray-200">
			<div class="pt-3 sm:pt-4 space-y-3 sm:space-y-4">
				<div class="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
					<div>
						<label class="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">First Name</label>
						<input
							type="text"
							bind:value={form.firstName}
							class="w-full px-3 py-2 text-xs sm:text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
							placeholder="Enter first name"
						/>
					</div>
					<div>
						<label class="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">Last Name</label>
						<input
							type="text"
							bind:value={form.lastName}
							class="w-full px-3 py-2 text-xs sm:text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
							placeholder="Enter last name"
						/>
					</div>
					<div>
						<label class="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">Email Address</label>
						<input
							type="email"
							bind:value={form.email}
							class="w-full px-3 py-2 text-xs sm:text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 break-all"
							placeholder="Enter email address"
						/>
					</div>
					<div>
						<label class="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">Phone Number</label>
						<input
							type="tel"
							bind:value={form.phone}
							class="w-full px-3 py-2 text-xs sm:text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
							placeholder="Enter phone number"
						/>
					</div>
					<div>
						<label class="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">Country</label>
						<input
							type="text"
							bind:value={form.country}
							class="w-full px-3 py-2 text-xs sm:text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
							placeholder="Enter country"
						/>
					</div>
					<div>
						<label class="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">City</label>
						<input
							type="text"
							bind:value={form.city}
							class="w-full px-3 py-2 text-xs sm:text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
							placeholder="Enter city"
						/>
					</div>
				</div>
				<button class="w-full px-4 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors text-sm sm:text-base font-medium">
					Save Changes
				</button>
			</div>
		</div>
	{/if}
</div>
