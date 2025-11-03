<script lang="ts">
	import { Zap } from '@lucide/svelte';
	import { demoMode } from '$lib/stores/demoMode';
</script>

<div class="flex items-center gap-2 sm:gap-3">
	<!-- Icon and label - hide label on mobile -->
	<div class="flex items-center gap-2">
		<Zap class="w-4 h-4 {$demoMode ? 'text-purple-600' : 'text-gray-400'}" />
		<span class="hidden sm:inline text-sm font-medium text-gray-700">Demo Mode</span>
	</div>
	
	<!-- Toggle switch -->
	<button
		onclick={() => demoMode.toggle()}
		class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 {$demoMode ? 'bg-purple-600' : 'bg-gray-200'}"
		role="switch"
		aria-checked={$demoMode}
		aria-label="Toggle demo mode"
	>
		<span
			class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {$demoMode ? 'translate-x-6' : 'translate-x-1'}"
		></span>
	</button>

	<!-- Demo badge - show on all sizes when active -->
	{#if $demoMode}
		<span class="px-2 py-1 bg-purple-100 text-purple-700 text-xs font-medium rounded">
			DEMO
		</span>
	{/if}
</div>
