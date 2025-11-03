<script lang="ts">
	import { ChevronDown, ChevronUp, Shield } from '@lucide/svelte';
	interface Props { expanded: boolean; onToggle: () => void; }
	let { expanded, onToggle }: Props = $props();
	let twoFactor = $state(false);
	let smsNotifications = $state(true);
	let searchable = $state(true);
</script>
<div class="bg-gray-50 rounded-lg overflow-hidden mb-2 sm:mb-3">
	<button onclick={onToggle} class="w-full flex items-center justify-between px-3 py-3 sm:px-4 sm:py-3.5 md:px-5 md:py-4 hover:bg-gray-100 transition-colors duration-200">
		<div class="flex items-center space-x-2 sm:space-x-3"><Shield class="w-4 h-4 sm:w-5 sm:h-5 md:w-6 md:h-6 text-gray-600 shrink-0" /><span class="text-sm sm:text-base md:text-lg text-gray-900">Security & Privacy</span></div>
		{#if expanded}<ChevronUp class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />{:else}<ChevronDown class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />{/if}
	</button>
	{#if expanded}
	<div class="px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5 border-t border-gray-200">
		<div class="pt-3 sm:pt-4 space-y-3 sm:space-y-4">
			<div class="flex items-center justify-between p-2.5 sm:p-3 bg-white rounded-lg">
				<div class="flex-1 min-w-0 mr-2"><h4 class="font-medium text-gray-900 text-xs sm:text-sm md:text-base wrap-break-word">Two-Factor Authentication</h4><p class="text-xs sm:text-sm text-gray-600 wrap-break-word">Add extra security to your account</p></div>
				<label class="relative inline-flex items-center cursor-pointer shrink-0"><input type="checkbox" bind:checked={twoFactor} class="sr-only peer" /><div class="w-9 h-5 sm:w-11 sm:h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 sm:after:h-5 sm:after:w-5 after:transition-all peer-checked:bg-gray-900"></div></label>
			</div>
			<div class="flex items-center justify-between p-2.5 sm:p-3 bg-white rounded-lg">
				<div class="flex-1 min-w-0 mr-2"><h4 class="font-medium text-gray-900 text-xs sm:text-sm md:text-base wrap-break-word">SMS Notifications</h4><p class="text-xs sm:text-sm text-gray-600 wrap-break-word">Receive transaction alerts via SMS</p></div>
				<label class="relative inline-flex items-center cursor-pointer shrink-0"><input type="checkbox" bind:checked={smsNotifications} class="sr-only peer" /><div class="w-9 h-5 sm:w-11 sm:h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 sm:after:h-5 sm:after:w-5 after:transition-all peer-checked:bg-gray-900"></div></label>
			</div>
			<div class="flex items-center justify-between p-2.5 sm:p-3 bg-white rounded-lg">
				<div class="flex-1 min-w-0 mr-2"><h4 class="font-medium text-gray-900 text-xs sm:text-sm md:text-base wrap-break-word">Searchable Profile</h4><p class="text-xs sm:text-sm text-gray-600 wrap-break-word">Allow others to find you by name or phone</p></div>
				<label class="relative inline-flex items-center cursor-pointer shrink-0"><input type="checkbox" bind:checked={searchable} class="sr-only peer" /><div class="w-9 h-5 sm:w-11 sm:h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 sm:after:h-5 sm:after:w-5 after:transition-all peer-checked:bg-gray-900"></div></label>
			</div>
			<button class="w-full px-3 sm:px-4 py-2 sm:py-2.5 text-xs sm:text-sm md:text-base bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors">Change Password</button>
		</div>
	</div>
	{/if}
</div>
