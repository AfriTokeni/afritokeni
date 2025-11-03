<script lang="ts">
	import { Zap } from '@lucide/svelte';

	interface Props {
		isDemoMode: boolean;
		onToggle: () => void;
	}

	let { isDemoMode, onToggle }: Props = $props();
</script>

<div class="flex items-center gap-3">
	<div class="flex items-center gap-2">
		<Zap class="w-4 h-4 {isDemoMode ? 'text-purple-600' : 'text-gray-400'}" />
		<span class="text-sm font-medium text-gray-700">Demo Mode</span>
	</div>
	
	<button
		onclick={onToggle}
		class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 {isDemoMode ? 'bg-purple-600' : 'bg-gray-200'}"
		role="switch"
		aria-checked={isDemoMode}
	>
		<span
			class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {isDemoMode ? 'translate-x-6' : 'translate-x-1'}"
		/>
	</button>

	{#if isDemoMode}
		<span class="px-2 py-1 bg-purple-100 text-purple-700 text-xs font-medium rounded">
			DEMO
		</span>
	{/if}
</div>
